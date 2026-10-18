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
use std::path::PathBuf;

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
        #[arg(long, default_value_t = DEFAULT_OGG_QUALITY)]
        audio_quality: f32,
    },
    /// Generate the built-in placeholder sprite atlas.
    GenerateAtlas {
        #[arg(short, long)]
        out: PathBuf,
        #[arg(short, long, default_value = "neon")]
        style: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Mix { input, output, list } => cmd_mix(input, output, list),
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
    let style = AtlasStyle::parse(&style)
        .ok_or_else(|| anyhow::anyhow!("unknown atlas style '{}' (expected classic or neon)", style))?;
    let atlas = SpriteAtlas::generate(style);
    atlas.save(&out)?;
    println!(
        "generated {} frames into {}",
        atlas.frames.len(),
        out.display()
    );
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
        other => bail!("unknown palette format '{}' (expected json, png or css)", other),
    }
    println!("wrote {} palette to {}", format, output.display());
    Ok(())
}

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
