// Asset Converter CLI - converts original game assets to modern formats.
//!
//! See cnc_asset::convert for the conversion implementations and tests.

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use cnc_asset::convert::{
    batch_convert, load_palette_or_grayscale, palette_to_css, palette_to_json, palette_to_png,
    validate_game, write_audio, write_shp_frames, write_shp_sheet, AudioFormat, AudioLoop,
    BatchOptions, DEFAULT_OGG_QUALITY, DEFAULT_TRANSPARENT_INDEX,
};
use cnc_asset::{AtlasStyle, MixArchive, PalFile, SpriteAtlas};
use std::fs;
#[allow(unused_imports)]
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "cnc-asset-cli")]
#[command(about = "Command & Conquer Asset Converter", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List or extract a MIX archive.
    Mix {
        #[arg(short, long)]
        input: PathBuf,
        /// Destination directory for extraction (ignored with --list).
        #[arg(short, long)]
        output: PathBuf,
        /// Only list entries; do not extract.
        #[arg(short, long)]
        list: bool,
    },
    /// Convert SHP frames to PNGs or one packed sprite sheet.
    Shp {
        #[arg(short, long)]
        input: PathBuf,
        /// Output directory (frames) or .png file (--sprite-sheet).
        #[arg(short, long)]
        output: PathBuf,
        /// Optional .pal palette; grayscale is used when omitted.
        #[arg(short, long)]
        palette: Option<PathBuf>,
        /// Pack all frames into a single horizontal sprite sheet.
        #[arg(long)]
        sprite_sheet: bool,
        /// Keep palette index 0 opaque (it is transparent by default).
        #[arg(long)]
        opaque: bool,
    },
    /// Convert a .voc or .aud file to WAV or Ogg Vorbis.
    Voc {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        /// Output container: wav or ogg (inferred from the output extension when omitted).
        #[arg(short, long)]
        format: Option<String>,
        /// First loop sample (inclusive); requires --loop-end.
        #[arg(long, requires = "loop_end")]
        loop_start: Option<u64>,
        /// One past the last loop sample; requires --loop-start.
        #[arg(long, requires = "loop_start")]
        loop_end: Option<u64>,
        /// Vorbis quality in [-0.2, 1.0] (only used for Ogg).
        #[arg(short, long, default_value_t = DEFAULT_OGG_QUALITY)]
        quality: f32,
    },
    /// Export a .pal palette as json, png or css.
    Pal {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long, default_value = "json")]
        format: String,
    },
    /// Recursively convert every supported asset in a directory tree.
    Batch {
        #[arg(short, long)]
        input: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
        /// Game selector: td / tiberian-dawn / ra / red-alert.
        #[arg(short, long)]
        game: String,
        /// Optional .pal palette used for SHP frames.
        #[arg(short, long)]
        palette: Option<PathBuf>,
        /// Pack each SHP into a sprite sheet instead of individual frames.
        #[arg(long)]
        sprite_sheet: bool,
        /// Keep palette index 0 opaque (it is transparent by default).
        #[arg(long)]
        opaque: bool,
        /// Audio container for .voc/.aud: wav or ogg.
        #[arg(long, default_value = "wav")]
        audio_format: String,
        /// Vorbis quality in [-0.2, 1.0] when --audio-format ogg is used.
        audio_quality: f32,
    },
    GenerateAtlas {
        #[arg(short, long)]
        out: PathBuf,
        #[arg(short, long, default_value = "neon")]
        style: String,
    },
    /// Fetch original C&C Remastered assets from Steam install to local store.
    OriginalFetch {
        /// Destination directory for the asset store (default: RUST/assets/original).
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Game to fetch: td / tiberian-dawn / ra / red-alert (default: both).
        #[arg(short, long)]
        game: Option<String>,
        /// Override Steam install path (default: auto-detect).
        #[arg(long)]
        steam_root: Option<PathBuf>,
    },
    /// Catalog original assets: list templates, icons, and generate previews.
    OriginalCatalog {
        /// Asset store root (default: CNC_ORIGINAL_ASSETS_DIR or RUST/assets/original).
        #[arg(short, long)]
        store: Option<PathBuf>,
        /// Output directory for previews and catalog (default: store/previews).
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Game to catalog: td / tiberian-dawn / ra / red-alert (default: both).
        #[arg(short, long)]
        game: Option<String>,
        /// Theater to catalog (default: all for game).
        #[arg(long)]
        theater: Option<String>,
        /// Only list templates; do not render previews.
        #[arg(long)]
        list_only: bool,
    },
}
fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Mix {
            input,
            output,
            list,
        } => cmd_mix(input, output, list),
        Commands::Shp {
            input,
            output,
            palette,
            sprite_sheet,
            opaque,
        } => cmd_shp(input, output, palette, sprite_sheet, opaque),
        Commands::Voc {
            input,
            output,
            format,
            loop_start,
            loop_end,
            quality,
        } => cmd_voc(input, output, format, loop_start, loop_end, quality),
        Commands::Pal {
            input,
            output,
            format,
        } => cmd_pal(input, output, format),
        Commands::Batch {
            input,
            output,
            game,
            palette,
            sprite_sheet,
            opaque,
            audio_format,
            audio_quality,
        } => cmd_batch(
            input,
            output,
            game,
            palette,
            sprite_sheet,
            opaque,
            audio_format,
            audio_quality,
        ),
        Commands::GenerateAtlas { out, style } => cmd_generate_atlas(out, style),
        Commands::OriginalFetch {
            output,
            game,
            steam_root,
        } => cmd_original_fetch(output, game, steam_root),
        Commands::OriginalCatalog {
            store,
            output,
            game,
            theater,
            list_only,
        } => cmd_original_catalog(store, output, game, theater, list_only),
    }
}

fn transparent_index(opaque: bool) -> Option<u8> {
    if opaque {
        None
    } else {
        Some(DEFAULT_TRANSPARENT_INDEX)
    }
}

fn cmd_generate_atlas(out: PathBuf, style: String) -> Result<()> {
    let style = AtlasStyle::parse(&style).ok_or_else(|| {
        anyhow::anyhow!("unknown atlas style '{}' (expected classic or neon)", style)
    })?;
    let atlas = SpriteAtlas::generate(style);
    atlas.save(&out)?;
    println!(
        "generated {} frames into {}",
        atlas.frames.len(),
        out.display()
    );
    Ok(())
}
fn cmd_original_fetch(
    output: Option<PathBuf>,
    game: Option<String>,
    steam_root: Option<PathBuf>,
) -> Result<()> {
    use cnc_asset::{steam_install_root, Game};
    use std::fs;

    // Determine output directory
    let out_dir = output
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/original"));

    // Determine which games to fetch
    let games: Vec<Game> = match game.as_deref() {
        Some(g) => match g.to_lowercase().as_str() {
            "td" | "tiberian-dawn" | "tiberian_dawn" => vec![Game::TiberianDawn],
            "ra" | "red-alert" | "red_alert" => vec![Game::RedAlert],
            _ => bail!(
                "unknown game '{}' (expected td, tiberian-dawn, ra, red-alert)",
                g
            ),
        },
        None => vec![Game::TiberianDawn, Game::RedAlert],
    };

    // Determine Steam root
    let steam_path = steam_root.or_else(steam_install_root).ok_or_else(|| {
        anyhow::anyhow!("could not locate Steam install; use --steam-root to specify")
    })?;

    println!(
        "Fetching original assets from {} to {}",
        steam_path.display(),
        out_dir.display()
    );

    for g in &games {
        let meg_name = g.texture_meg();
        let src = steam_path.join("Data").join(meg_name);
        if !src.is_file() {
            eprintln!("Warning: {} not found at {}", meg_name, src.display());
            continue;
        }

        let dst = out_dir.join("Data").join(meg_name);
        fs::create_dir_all(dst.parent().unwrap())?;
        fs::copy(&src, &dst)
            .with_context(|| format!("copying {} to {}", src.display(), dst.display()))?;
        println!("  copied {} -> {}", meg_name, dst.display());
    }

    // Copy TD theater MIX files for MAP occupancy data
    if games.contains(&Game::TiberianDawn) {
        let td_cncddata = steam_path.join("Data/CNCDATA/TIBERIAN_DAWN");
        for dir in ["CD1", "CD2", "CD3", "CONSOLE_1", "CONSOLE_2", "CUSTOMMAPS"] {
            let src_dir = td_cncddata.join(dir);
            if src_dir.is_dir() {
                for entry in fs::read_dir(&src_dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    let name = path.file_name().unwrap().to_string_lossy().to_string();
                    if name.to_uppercase().ends_with(".MIX") {
                        let dst = out_dir
                            .join("Data/CNCDATA/TIBERIAN_DAWN")
                            .join(dir)
                            .join(&*entry.file_name().to_string_lossy());
                        fs::create_dir_all(dst.parent().unwrap())?;
                        fs::copy(&path, &dst).with_context(|| {
                            format!("copying {} to {}", path.display(), dst.display())
                        })?;
                        println!("  copied {} -> {}", name, dst.display());
                    }
                }
            }
        }
    }

    // Write manifest.json
    let manifest = serde_json::json!({
        "source": "steam",
        "steam_root": steam_path.to_string_lossy(),
        "games": games.iter().map(|g| match g { Game::TiberianDawn => "tiberian_dawn", Game::RedAlert => "red_alert" }).collect::<Vec<_>>(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    let manifest_path = out_dir.join("manifest.json");
    fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;
    println!("wrote manifest to {}", manifest_path.display());

    Ok(())
}

fn cmd_original_catalog(
    store: Option<PathBuf>,
    output: Option<PathBuf>,
    game: Option<String>,
    theater: Option<String>,
    list_only: bool,
) -> Result<()> {
    use cnc_asset::{render_previews, Game, OriginalAssets, Theater};

    // Determine store root
    let store_root = store
        .or_else(OriginalAssets::original_store_root)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "asset store not found; set CNC_ORIGINAL_ASSETS_DIR or run original-fetch first"
            )
        })?;

    if !store_root.join("manifest.json").is_file() {
        bail!(
            "store root {} does not contain manifest.json",
            store_root.display()
        );
    }

    // Determine which games to catalog
    let games: Vec<Game> = match game.as_deref() {
        Some(g) => match g.to_lowercase().as_str() {
            "td" | "tiberian-dawn" | "tiberian_dawn" => vec![Game::TiberianDawn],
            "ra" | "red-alert" | "red_alert" => vec![Game::RedAlert],
            _ => bail!(
                "unknown game '{}' (expected td, tiberian-dawn, ra, red-alert)",
                g
            ),
        },
        None => vec![Game::TiberianDawn, Game::RedAlert],
    };

    // Determine theater filter
    let _theater_filter: Option<Theater> = theater.as_deref().and_then(Theater::parse);

    // If list_only, just print template info
    if list_only {
        for g in &games {
            println!(
                "=== {} ===",
                match g {
                    Game::TiberianDawn => "Tiberian Dawn",
                    Game::RedAlert => "Red Alert",
                }
            );
            let assets = OriginalAssets::open(&store_root, *g)?;
            println!("  member count: {}", assets.member_count());
            for name in assets.member_names() {
                println!("    {}", name);
            }
        }
        return Ok(());
    }

    // Render previews
    let default_out_dir = store_root.join("previews");
    let out_dir = output.as_deref().unwrap_or(&default_out_dir);
    println!(
        "Rendering previews from {} to {}",
        store_root.display(),
        out_dir.display()
    );
    render_previews(out_dir, &store_root)?;
    println!("Done.");

    Ok(())
}

fn cmd_mix(input: PathBuf, output: PathBuf, list: bool) -> Result<()> {
    let archive = MixArchive::load(&input)
        .with_context(|| format!("failed to open MIX archive {}", input.display()))?;
    if list {
        let mut names = archive.list();
        names.sort_unstable();
        println!("{} entries in {}:", names.len(), input.display());
        for name in names {
            println!("{name}");
        }
        return Ok(());
    }
    let count = archive.len();
    archive.extract_all(&output)?;
    println!(
        "extracted {count} entries from {} into {}",
        input.display(),
        output.display()
    );
    Ok(())
}

fn cmd_shp(
    input: PathBuf,
    output: PathBuf,
    palette: Option<PathBuf>,
    sprite_sheet: bool,
    opaque: bool,
) -> Result<()> {
    let palette = load_palette_or_grayscale(palette.as_deref())?;
    if palette.colors == PalFile::grayscale().colors {
        eprintln!("warning: no palette supplied, using grayscale");
    }
    let transparent = transparent_index(opaque);
    if sprite_sheet {
        let sheet = write_shp_sheet(&input, &output, &palette, transparent)?;
        println!(
            "wrote sprite sheet {} and manifest {}",
            output.display(),
            sheet.display()
        );
    } else {
        let written = write_shp_frames(&input, &output, &palette, transparent)?;
        println!(
            "wrote {} frame(s) from {} into {}",
            written.len(),
            input.display(),
            output.display()
        );
    }
    Ok(())
}

fn cmd_voc(
    input: PathBuf,
    output: PathBuf,
    format: Option<String>,
    loop_start: Option<u64>,
    loop_end: Option<u64>,
    quality: f32,
) -> Result<()> {
    let format = match format {
        Some(format) => AudioFormat::parse(&format)?,
        None => output
            .extension()
            .and_then(|extension| extension.to_str())
            .map(AudioFormat::parse)
            .transpose()?
            .unwrap_or(AudioFormat::Wav),
    };
    let loop_override = match (loop_start, loop_end) {
        (Some(start), Some(end)) => Some(AudioLoop::new(start, end)?),
        (None, None) => None,
        _ => bail!("--loop-start and --loop-end must be provided together"),
    };
    write_audio(&input, &output, format, loop_override, quality)?;
    println!(
        "wrote {} -> {} ({})",
        input.display(),
        output.display(),
        format.extension()
    );
    Ok(())
}

fn cmd_pal(input: PathBuf, output: PathBuf, format: String) -> Result<()> {
    let data = fs::read(&input)?;
    let pal = PalFile::load(&data)?;
    fs::create_dir_all(&output)?;
    match format.as_str() {
        "json" => fs::write(output.join("palette.json"), palette_to_json(&pal)?)?,
        "png" => palette_to_png(&pal).save(output.join("palette.png"))?,
        "css" => fs::write(output.join("palette.css"), palette_to_css(&pal))?,
        other => bail!(
            "unknown palette format '{}' (expected json, png or css)",
            other
        ),
    }
    println!("wrote {} palette to {}", format, output.display());
    Ok(())
}

// CLI subcommand with one parameter per clap flag; grouping into a config
// struct would just move the same eight fields around one call site.
#[allow(clippy::too_many_arguments)]
fn cmd_batch(
    input: PathBuf,
    output: PathBuf,
    game: String,
    palette: Option<PathBuf>,
    sprite_sheet: bool,
    opaque: bool,
    audio_format: String,
    audio_quality: f32,
) -> Result<()> {
    validate_game(&game)?;
    if !(-0.2..=1.0).contains(&audio_quality) {
        bail!("audio quality must be between -0.2 and 1.0");
    }
    let palette = match palette {
        Some(path) => Some(PalFile::load_file(&path)?),
        None => None,
    };
    let options = BatchOptions {
        palette,
        transparent: transparent_index(opaque),
        sprite_sheet,
        audio_format: AudioFormat::parse(&audio_format)?,
        audio_quality,
    };
    println!(
        "batch converting {} ({}) into {}",
        input.display(),
        game,
        output.display()
    );
    let report = batch_convert(&input, &output, &game, &options)?;
    println!(
        "converted {} file(s), skipped {}, failed {}",
        report.converted,
        report.skipped,
        report.failed()
    );
    for kind in ["mix", "shp", "voc", "aud", "pal"] {
        if let Some(count) = report.counts.get(kind) {
            println!("  {kind}: {count}");
        }
    }
    if !report.failures.is_empty() {
        for (path, error) in &report.failures {
            eprintln!("failed {}: {error}", path.display());
        }
        bail!("{} asset(s) failed to convert", report.failures.len());
    }
    Ok(())
}
