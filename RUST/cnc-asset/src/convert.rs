//! Asset conversion helpers shared by the cnc-asset-cli binary (Phase 4).
//!
//! The original collection ships Westwood-era formats (.mix/.shp/.voc/.aud/.pal).
//! These helpers turn them into runtime friendly files (PNG frames, sprite sheets,
//! JSON manifests, WAV/OGG audio) without touching the parsers themselves.

use anyhow::{bail, Context, Result};
use cnc_core::RGB888;
use image::{ImageBuffer, Rgba, RgbaImage};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::num::{NonZeroU32, NonZeroU8};
use std::path::{Path, PathBuf};
use vorbis_rs::{VorbisBitrateManagementStrategy, VorbisEncoderBuilder};

use crate::archive::MixArchive;
use crate::atlas::SpriteFrame;
use crate::audio::{AudFile, VocFile};
use crate::palette::PalFile;
use crate::shape::{ShpFile, ShpFrame};

/// Palette index treated as transparent by default (Westwood shapes use index 0).
pub const DEFAULT_TRANSPARENT_INDEX: u8 = 0;

/// One rectangle in a generated sprite sheet.
#[derive(Debug, Clone, Serialize)]
pub struct SheetFrame {
    pub index: usize,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl From<(usize, &SpriteFrame)> for SheetFrame {
    fn from((index, frame): (usize, &SpriteFrame)) -> Self {
        Self {
            index,
            x: frame.x,
            y: frame.y,
            w: frame.w,
            h: frame.h,
        }
    }
}

/// A packed strip of SHP frames plus their rectangles.
#[derive(Debug, Clone)]
pub struct SpriteSheet {
    pub image: RgbaImage,
    pub frames: Vec<SpriteFrame>,
}

/// Convert one SHP frame to RGBA, honouring the transparent palette index.
pub fn frame_to_rgba(
    frame: &ShpFrame,
    palette: &[RGB888; 256],
    transparent: Option<u8>,
) -> Result<RgbaImage> {
    let width = frame.width as u32;
    let height = frame.height as u32;
    let expected = (width as usize) * (height as usize);
    if frame.data.len() < expected {
        bail!(
            "frame data is too short: expected {} palette indices, got {}",
            expected,
            frame.data.len()
        );
    }
    let mut pixels = Vec::with_capacity(expected * 4);
    for &index in &frame.data[..expected] {
        let color = palette[index as usize];
        let alpha = if Some(index) == transparent { 0 } else { 255 };
        pixels.push(color.r);
        pixels.push(color.g);
        pixels.push(color.b);
        pixels.push(alpha);
    }
    RgbaImage::from_raw(width, height, pixels).context("failed to build RGBA image from SHP frame")
}

/// Convert every frame in a SHP file to RGBA.
pub fn shp_frames(
    shp: &ShpFile,
    palette: &[RGB888; 256],
    transparent: Option<u8>,
) -> Result<Vec<RgbaImage>> {
    let mut images = Vec::with_capacity(shp.frame_count());
    for (index, frame) in shp.frames.iter().enumerate() {
        images.push(
            frame_to_rgba(frame, palette, transparent)
                .with_context(|| format!("failed to convert SHP frame {}", index))?,
        );
    }
    Ok(images)
}

/// Pack every SHP frame into one horizontal strip.
pub fn shp_to_sprite_sheet(
    shp: &ShpFile,
    palette: &[RGB888; 256],
    transparent: Option<u8>,
) -> Result<SpriteSheet> {
    if shp.frames.is_empty() {
        bail!("SHP has no frames to pack");
    }
    let images = shp_frames(shp, palette, transparent)?;
    let width: u32 = images.iter().map(|image| image.width()).sum();
    let height: u32 = images.iter().map(|image| image.height()).max().unwrap_or(0);
    if width == 0 || height == 0 {
        bail!("SHP frames are empty (zero width or height)");
    }
    let mut canvas: RgbaImage = ImageBuffer::from_pixel(width, height, Rgba([0, 0, 0, 0]));
    let mut frames = Vec::with_capacity(images.len());
    let mut x = 0u32;
    for image in &images {
        image::imageops::overlay(&mut canvas, image, i64::from(x), 0);
        frames.push(SpriteFrame {
            x,
            y: 0,
            w: image.width(),
            h: image.height(),
        });
        x += image.width();
    }
    Ok(SpriteSheet {
        image: canvas,
        frames,
    })
}

/// Manifest written next to a generated sprite sheet.
#[derive(Debug, Clone, Serialize)]
pub struct SheetManifest {
    pub width: u32,
    pub height: u32,
    pub frames: Vec<SheetFrame>,
}

/// Write a sprite sheet image and its JSON manifest.
pub fn save_sprite_sheet(sheet: &SpriteSheet, png_path: &Path, json_path: &Path) -> Result<()> {
    if let Some(parent) = png_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create output directory {:?}", parent))?;
        }
    }
    sheet
        .image
        .save(png_path)
        .with_context(|| format!("failed to write sprite sheet {:?}", png_path))?;
    let manifest = SheetManifest {
        width: sheet.image.width(),
        height: sheet.image.height(),
        frames: sheet
            .frames
            .iter()
            .enumerate()
            .map(SheetFrame::from)
            .collect(),
    };
    fs::write(json_path, serde_json::to_vec_pretty(&manifest)?)
        .with_context(|| format!("failed to write sheet manifest {:?}", json_path))?;
    Ok(())
}

/// Load a SHP file from disk.
pub fn load_shp(path: &Path) -> Result<ShpFile> {
    let data = fs::read(path).with_context(|| format!("failed to read SHP {:?}", path))?;
    ShpFile::load(&data).with_context(|| format!("failed to parse SHP {:?}", path))
}

/// Load an optional PAL file, falling back to a deterministic grayscale palette.
pub fn load_palette_or_grayscale(path: Option<&Path>) -> Result<PalFile> {
    match path {
        Some(path) => PalFile::load_file(path),
        None => Ok(PalFile::grayscale()),
    }
}

/// Write each SHP frame as {stem}_frame{index:03}.png.
pub fn write_shp_frames(
    input: &Path,
    output_dir: &Path,
    palette: &PalFile,
    transparent: Option<u8>,
) -> Result<Vec<PathBuf>> {
    let shp = load_shp(input)?;
    if shp.frames.is_empty() {
        bail!("{:?} contains no frames", input);
    }
    fs::create_dir_all(output_dir)
        .with_context(|| format!("failed to create output directory {:?}", output_dir))?;
    let stem = input
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("shape");
    let images = shp_frames(&shp, &palette.colors, transparent)?;
    let mut written = Vec::with_capacity(images.len());
    for (index, image) in images.iter().enumerate() {
        let path = output_dir.join(format!("{stem}_frame{index:03}.png"));
        image
            .save(&path)
            .with_context(|| format!("failed to write frame {:?}", path))?;
        written.push(path);
    }
    Ok(written)
}

/// Write a single packed sprite sheet plus manifest for a SHP file.
pub fn write_shp_sheet(
    input: &Path,
    output_png: &Path,
    palette: &PalFile,
    transparent: Option<u8>,
) -> Result<PathBuf> {
    let shp = load_shp(input)?;
    let sheet = shp_to_sprite_sheet(&shp, &palette.colors, transparent)?;
    let mut json_path = output_png.to_path_buf();
    json_path.set_extension("json");
    save_sprite_sheet(&sheet, output_png, &json_path)?;
    Ok(json_path)
}

/// Default Vorbis quality used by the converter (0.4 is transparent enough for
/// the low-bitrate Westwood source material).
pub const DEFAULT_OGG_QUALITY: f32 = 0.4;

/// Sample-range loop points, preserved in Ogg comments and the WAV smpl chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct AudioLoop {
    /// First sample of the loop (inclusive).
    pub start: u64,
    /// One past the last sample of the loop (exclusive).
    pub end: u64,
}

impl AudioLoop {
    pub fn new(start: u64, end: u64) -> Result<Self> {
        if end <= start {
            bail!("invalid loop points: end ({end}) must be greater than start ({start})");
        }
        Ok(Self { start, end })
    }

    pub fn length(&self) -> u64 {
        self.end - self.start
    }
}

/// Audio container produced by the converter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioFormat {
    Wav,
    Ogg,
}

impl AudioFormat {
    pub fn parse(value: &str) -> Result<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "wav" | "wave" => Ok(Self::Wav),
            "ogg" | "vorbis" | "ogg-vorbis" => Ok(Self::Ogg),
            other => bail!("unknown audio format '{}' (expected wav or ogg)", other),
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Wav => "wav",
            Self::Ogg => "ogg",
        }
    }
}

/// Decoded PCM audio plus any loop points declared by the source file.
#[derive(Debug, Clone)]
pub struct PcmAudio {
    pub sample_rate: u32,
    pub channels: u16,
    pub samples: Vec<i16>,
    pub loop_points: Option<AudioLoop>,
}

impl PcmAudio {
    /// Number of interleaved frames (samples per channel).
    pub fn frames(&self) -> usize {
        if self.channels == 0 {
            0
        } else {
            self.samples.len() / usize::from(self.channels)
        }
    }

    /// Interleave-free f32 channels in [-1.0, 1.0] for the encoder.
    fn channel_samples(&self) -> Vec<Vec<f32>> {
        let channels = usize::from(self.channels.max(1));
        let frames = self.frames();
        let mut planar = vec![Vec::with_capacity(frames); channels];
        for frame in 0..frames {
            for (channel, samples) in planar.iter_mut().enumerate() {
                let sample = self.samples[frame * channels + channel];
                samples.push(f32::from(sample) / 32_768.0);
            }
        }
        planar
    }
}

/// Decode a .voc or .aud file into PCM.
pub fn load_audio(input: &Path) -> Result<PcmAudio> {
    let data = fs::read(input).with_context(|| format!("failed to read audio {:?}", input))?;
    let extension = input
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    match extension.as_str() {
        "voc" => {
            let voc =
                VocFile::load(&data).with_context(|| format!("failed to parse VOC {:?}", input))?;
            let declared = match (voc.loop_start, voc.loop_end) {
                (Some(start), Some(end)) => Some(AudioLoop::new(u64::from(start), u64::from(end))?),
                _ => None,
            };
            Ok(PcmAudio {
                sample_rate: voc.sample_rate,
                channels: voc.channels,
                samples: voc.data,
                loop_points: declared,
            })
        }
        "aud" => {
            let aud =
                AudFile::load(&data).with_context(|| format!("failed to parse AUD {:?}", input))?;
            Ok(PcmAudio {
                sample_rate: aud.sample_rate,
                channels: aud.channels,
                samples: aud.data,
                loop_points: None,
            })
        }
        other => bail!("unsupported audio extension '{}' for {:?}", other, input),
    }
}

/// PCM to WAV bytes, optionally with a RIFF smpl loop chunk.
pub fn audio_to_wav(audio: &PcmAudio, loop_points: Option<AudioLoop>) -> Result<Vec<u8>> {
    if audio.channels == 0 {
        bail!("audio has zero channels");
    }
    let spec = hound::WavSpec {
        channels: audio.channels,
        sample_rate: audio.sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut buffer = Vec::new();
    {
        let mut writer = hound::WavWriter::new(std::io::Cursor::new(&mut buffer), spec)?;
        for &sample in &audio.samples {
            writer.write_sample(sample)?;
        }
        writer.finalize()?;
    }
    match loop_points {
        Some(loop_points) => append_smpl_chunk(buffer, audio.sample_rate, loop_points),
        None => Ok(buffer),
    }
}

/// Append a RIFF smpl chunk describing one forward loop.
fn append_smpl_chunk(
    mut wav: Vec<u8>,
    sample_rate: u32,
    loop_points: AudioLoop,
) -> Result<Vec<u8>> {
    if wav.len() < 12 || &wav[0..4] != b"RIFF" || &wav[8..12] != b"WAVE" {
        bail!("cannot add loop points: encoder did not produce a RIFF/WAVE file");
    }
    let last_sample = loop_points.end.saturating_sub(1);
    let mut chunk = Vec::with_capacity(68);
    chunk.extend_from_slice(b"smpl");
    chunk.extend_from_slice(&60u32.to_le_bytes()); // chunk size: 36 + 24 * loops
    chunk.extend_from_slice(&0u32.to_le_bytes()); // manufacturer
    chunk.extend_from_slice(&0u32.to_le_bytes()); // product
    let sample_period = if sample_rate == 0 {
        0
    } else {
        (1_000_000_000u64 / u64::from(sample_rate)) as u32
    };
    chunk.extend_from_slice(&sample_period.to_le_bytes());
    chunk.extend_from_slice(&60u32.to_le_bytes()); // MIDI unity note
    chunk.extend_from_slice(&0u32.to_le_bytes()); // MIDI pitch fraction
    chunk.extend_from_slice(&0u32.to_le_bytes()); // SMPTE format
    chunk.extend_from_slice(&0u32.to_le_bytes()); // SMPTE offset
    chunk.extend_from_slice(&1u32.to_le_bytes()); // number of loops
    chunk.extend_from_slice(&0u32.to_le_bytes()); // sampler data
    chunk.extend_from_slice(&0u32.to_le_bytes()); // cue point id
    chunk.extend_from_slice(&0u32.to_le_bytes()); // type: forward loop
    chunk.extend_from_slice(&(loop_points.start.min(u64::from(u32::MAX)) as u32).to_le_bytes());
    chunk.extend_from_slice(&(last_sample.min(u64::from(u32::MAX)) as u32).to_le_bytes());
    chunk.extend_from_slice(&0u32.to_le_bytes()); // fraction
    chunk.extend_from_slice(&0u32.to_le_bytes()); // play count: infinite

    wav.extend_from_slice(&chunk);
    let riff_size = (wav.len() - 8) as u32;
    wav[4..8].copy_from_slice(&riff_size.to_le_bytes());
    Ok(wav)
}

/// Encode PCM to an Ogg Vorbis stream, writing loop points as Vorbis comments.
pub fn audio_to_ogg(
    audio: &PcmAudio,
    quality: f32,
    loop_points: Option<AudioLoop>,
) -> Result<Vec<u8>> {
    if audio.channels == 0 || audio.channels > 8 {
        bail!(
            "unsupported channel count {} for Ogg Vorbis",
            audio.channels
        );
    }
    if audio.frames() == 0 {
        bail!("audio contains no samples");
    }
    let sample_rate = NonZeroU32::new(audio.sample_rate).context("sample rate must be non-zero")?;
    let channels =
        NonZeroU8::new(audio.channels as u8).context("channel count must be non-zero")?;

    let mut builder = VorbisEncoderBuilder::new(sample_rate, channels, Vec::<u8>::new())
        .context("failed to create the Vorbis encoder")?;
    builder.bitrate_management_strategy(VorbisBitrateManagementStrategy::QualityVbr {
        target_quality: quality.clamp(-0.2, 1.0),
    });
    if let Some(loop_points) = loop_points {
        builder.comment_tag("LOOPSTART", loop_points.start.to_string())?;
        builder.comment_tag("LOOPEND", loop_points.end.to_string())?;
        builder.comment_tag("LOOPLENGTH", loop_points.length().to_string())?;
    }
    let mut encoder = builder.build().context("failed to start Vorbis encoding")?;

    let planar = audio.channel_samples();
    let frames = audio.frames();
    const BLOCK: usize = 1024;
    let mut start = 0;
    while start < frames {
        let end = (start + BLOCK).min(frames);
        let block: Vec<&[f32]> = planar.iter().map(|channel| &channel[start..end]).collect();
        encoder
            .encode_audio_block(&block)
            .context("failed to encode an Ogg Vorbis block")?;
        start = end;
    }
    encoder
        .finish()
        .context("failed to finish the Ogg Vorbis stream")
}

/// Convert a .voc or .aud file into WAV or Ogg, preserving or overriding loops.
pub fn write_audio(
    input: &Path,
    output: &Path,
    format: AudioFormat,
    loop_override: Option<AudioLoop>,
    quality: f32,
) -> Result<()> {
    let audio = load_audio(input)?;
    let loop_points = loop_override.or(audio.loop_points);
    let bytes = match format {
        AudioFormat::Wav => audio_to_wav(&audio, loop_points)?,
        AudioFormat::Ogg => audio_to_ogg(&audio, quality, loop_points)?,
    };
    if let Some(parent) = output.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(output, bytes).with_context(|| format!("failed to write audio {:?}", output))?;
    Ok(())
}

/// Convert a .voc or .aud file to a WAV file at output (loop points preserved).
pub fn write_audio_wav(input: &Path, output: &Path) -> Result<()> {
    write_audio(input, output, AudioFormat::Wav, None, DEFAULT_OGG_QUALITY)
}

/// Export a palette as JSON colours.
pub fn palette_to_json(palette: &PalFile) -> Result<String> {
    let colors: Vec<_> = palette
        .colors
        .iter()
        .map(|color| serde_json::json!({"r": color.r, "g": color.g, "b": color.b}))
        .collect();
    Ok(serde_json::to_string_pretty(
        &serde_json::json!({ "colors": colors }),
    )?)
}

/// Export a palette as a 16x16 PNG swatch.
pub fn palette_to_png(palette: &PalFile) -> RgbaImage {
    let mut image: RgbaImage = ImageBuffer::new(16, 16);
    for (index, color) in palette.colors.iter().enumerate() {
        let x = (index % 16) as u32;
        let y = (index / 16) as u32;
        image.put_pixel(x, y, Rgba([color.r, color.g, color.b, 255]));
    }
    image
}

/// Export a palette as CSS custom properties.
pub fn palette_to_css(palette: &PalFile) -> String {
    let mut css = String::from(":root {\n");
    for (index, color) in palette.colors.iter().enumerate() {
        css.push_str(&format!(
            "  --palette-{}: rgb({}, {}, {});\n",
            index, color.r, color.g, color.b
        ));
    }
    css.push_str("}\n");
    css
}

/// Extract every entry of a MIX archive into output_dir.
pub fn extract_mix(input: &Path, output_dir: &Path) -> Result<Vec<String>> {
    let archive =
        MixArchive::load(input).with_context(|| format!("failed to parse MIX {:?}", input))?;
    archive.extract_all(output_dir)?;
    Ok(archive
        .list()
        .iter()
        .map(|name| (*name).to_string())
        .collect())
}

/// Supported batch conversion kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AssetKind {
    Mix,
    Shp,
    Voc,
    Aud,
    Pal,
}

impl AssetKind {
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension.to_ascii_lowercase().as_str() {
            "mix" => Some(Self::Mix),
            "shp" => Some(Self::Shp),
            "voc" => Some(Self::Voc),
            "aud" => Some(Self::Aud),
            "pal" => Some(Self::Pal),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Mix => "mix",
            Self::Shp => "shp",
            Self::Voc => "voc",
            Self::Aud => "aud",
            Self::Pal => "pal",
        }
    }
}

/// Options controlling a batch conversion run.
#[derive(Debug, Clone)]
pub struct BatchOptions {
    pub palette: Option<PalFile>,
    pub transparent: Option<u8>,
    pub sprite_sheet: bool,
    /// Container written for .voc/.aud sources (WAV by default).
    pub audio_format: AudioFormat,
    /// Vorbis quality used when audio_format is Ogg.
    pub audio_quality: f32,
}

impl Default for BatchOptions {
    fn default() -> Self {
        Self {
            palette: None,
            transparent: Some(DEFAULT_TRANSPARENT_INDEX),
            sprite_sheet: false,
            audio_format: AudioFormat::Wav,
            audio_quality: DEFAULT_OGG_QUALITY,
        }
    }
}

/// Summary of a batch conversion run.
#[derive(Debug, Clone, Default)]
pub struct BatchReport {
    /// Number of source files converted successfully.
    pub converted: usize,
    /// Number of source files skipped because they have no converter.
    pub skipped: usize,
    /// Per-kind conversion counts.
    pub counts: BTreeMap<&'static str, usize>,
    /// Files that failed to convert, with the error message.
    pub failures: Vec<(PathBuf, String)>,
}

impl BatchReport {
    pub fn count(&self, kind: AssetKind) -> usize {
        self.counts.get(kind.label()).copied().unwrap_or(0)
    }

    pub fn failed(&self) -> usize {
        self.failures.len()
    }
}

/// Validate the game selector accepted by the CLI.
pub fn validate_game(game: &str) -> Result<()> {
    match game.to_ascii_lowercase().as_str() {
        "td" | "tiberian-dawn" | "tiberiandawn" | "ra" | "red-alert" | "redalert" => Ok(()),
        other => bail!(
            "unknown game '{}': expected td, tiberian-dawn, ra or red-alert",
            other
        ),
    }
}

/// Recursively convert every supported asset under input into output,
/// mirroring the source tree layout.
pub fn batch_convert(
    input: &Path,
    output: &Path,
    game: &str,
    options: &BatchOptions,
) -> Result<BatchReport> {
    validate_game(game)?;
    if !input.is_dir() {
        bail!("batch input {:?} is not a directory", input);
    }
    fs::create_dir_all(output)
        .with_context(|| format!("failed to create output directory {:?}", output))?;
    let mut report = BatchReport::default();
    walk_and_convert(input, Path::new(""), output, options, &mut report)?;
    Ok(report)
}

fn walk_and_convert(
    directory: &Path,
    relative_dir: &Path,
    output_root: &Path,
    options: &BatchOptions,
    report: &mut BatchReport,
) -> Result<()> {
    let mut entries: Vec<_> = fs::read_dir(directory)
        .with_context(|| format!("failed to read directory {:?}", directory))?
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        let name = entry.file_name();
        if path.is_dir() {
            walk_and_convert(
                &path,
                &relative_dir.join(&name),
                output_root,
                options,
                report,
            )?;
        } else if path.is_file() {
            let output_dir = output_root.join(relative_dir);
            convert_single(&path, &output_dir, options, report);
        }
    }
    Ok(())
}

fn convert_single(
    path: &Path,
    output_dir: &Path,
    options: &BatchOptions,
    report: &mut BatchReport,
) {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default();
    let Some(kind) = AssetKind::from_extension(extension) else {
        report.skipped += 1;
        return;
    };
    let palette = options.palette.clone().unwrap_or_else(PalFile::grayscale);
    let result = (|| -> Result<()> {
        match kind {
            AssetKind::Mix => {
                let stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("archive");
                extract_mix(path, &output_dir.join(stem))?;
            }
            AssetKind::Shp => {
                if options.sprite_sheet {
                    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("shape");
                    let png = output_dir.join(format!("{stem}_sheet.png"));
                    write_shp_sheet(path, &png, &palette, options.transparent)?;
                } else {
                    write_shp_frames(path, output_dir, &palette, options.transparent)?;
                }
            }
            AssetKind::Voc | AssetKind::Aud => {
                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("audio");
                let output =
                    output_dir.join(format!("{stem}.{}", options.audio_format.extension()));
                write_audio(
                    path,
                    &output,
                    options.audio_format,
                    None,
                    options.audio_quality,
                )?;
            }
            AssetKind::Pal => {
                let stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("palette");
                let pal = PalFile::load_file(path)?;
                fs::create_dir_all(output_dir)?;
                fs::write(
                    output_dir.join(format!("{stem}.json")),
                    palette_to_json(&pal)?,
                )?;
                pal.save(&output_dir.join(format!("{stem}.pal")))?;
            }
        }
        Ok(())
    })();
    match result {
        Ok(()) => {
            report.converted += 1;
            *report.counts.entry(kind.label()).or_insert(0) += 1;
        }
        Err(error) => report
            .failures
            .push((path.to_path_buf(), format!("{error:#}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "cnc_asset_convert_{}_{}_{}",
            name,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Build a minimal uncompressed SHP containing solid-colour frames.
    fn synthetic_shp(frames: &[(u16, u16, u8)]) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&0u16.to_le_bytes()); // version
        out.extend_from_slice(&0u16.to_le_bytes()); // flags
        out.extend_from_slice(&(frames.len() as u16).to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        for (width, height, _) in frames {
            out.extend_from_slice(&width.to_le_bytes());
            out.extend_from_slice(&height.to_le_bytes());
            out.extend_from_slice(&0i16.to_le_bytes()); // offset x
            out.extend_from_slice(&0i16.to_le_bytes()); // offset y
        }
        let header_len = 8 + frames.len() * 8 + frames.len() * 4;
        let mut cursor = header_len as u32;
        for (width, height, _) in frames {
            out.extend_from_slice(&cursor.to_le_bytes());
            cursor += u32::from(*width) * u32::from(*height);
        }
        for (width, height, color) in frames {
            out.extend(std::iter::repeat_n(
                *color,
                usize::from(*width) * usize::from(*height),
            ));
        }
        out
    }

    fn synthetic_voc() -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(b"Creative Voice File\x1a");
        data.extend_from_slice(&26u16.to_le_bytes());
        data.extend_from_slice(&0x0100u16.to_le_bytes());
        data.extend_from_slice(&0u16.to_le_bytes());
        data.push(0x00);
        data.extend_from_slice(&[0, 0, 0, 0]);
        data
    }

    fn synthetic_aud() -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&22050u32.to_le_bytes());
        data.extend_from_slice(&1u16.to_le_bytes());
        data.extend_from_slice(&0u16.to_le_bytes());
        data.extend_from_slice(&(-100i16).to_le_bytes());
        data.extend_from_slice(&100i16.to_le_bytes());
        data
    }

    fn synthetic_mix(entries: &[(&str, &[u8])]) -> Vec<u8> {
        // MIX v0 (uncompressed): signature, count, name offsets, sizes, names, data.
        let mut out = Vec::new();
        out.extend_from_slice(b"MIX\x00");
        out.extend_from_slice(&(entries.len() as u32).to_le_bytes());
        let mut name_table = Vec::new();
        let mut name_offsets = Vec::new();
        for (name, _) in entries {
            name_offsets.push(name_table.len() as u32);
            name_table.extend_from_slice(name.as_bytes());
            name_table.push(0);
        }
        for offset in &name_offsets {
            out.extend_from_slice(&offset.to_le_bytes());
        }
        for (_, data) in entries {
            out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        }
        out.extend_from_slice(&name_table);
        for (_, data) in entries {
            out.extend_from_slice(data);
        }
        out
    }

    #[test]
    fn frames_are_transparent_at_index_zero() {
        let shp = ShpFile::load(&synthetic_shp(&[(2, 2, 0)])).unwrap();
        let palette = PalFile::grayscale();
        let image = frame_to_rgba(&shp.frames[0], &palette.colors, Some(0)).unwrap();
        assert_eq!(image.dimensions(), (2, 2));
        assert_eq!(image.get_pixel(0, 0)[3], 0);
        assert_eq!(image.get_pixel(1, 1)[3], 0);

        let opaque = frame_to_rgba(&shp.frames[0], &palette.colors, None).unwrap();
        assert_eq!(opaque.get_pixel(0, 0)[3], 255);
    }

    #[test]
    fn frames_preserve_palette_colours() {
        let shp = ShpFile::load(&synthetic_shp(&[(1, 1, 7)])).unwrap();
        let palette = PalFile::grayscale();
        let image = frame_to_rgba(&shp.frames[0], &palette.colors, Some(0)).unwrap();
        let pixel = image.get_pixel(0, 0);
        assert_eq!([pixel[0], pixel[1], pixel[2], pixel[3]], [7, 7, 7, 255]);
    }

    #[test]
    fn sprite_sheet_packs_frames_horizontally() {
        let shp = ShpFile::load(&synthetic_shp(&[(2, 3, 10), (4, 2, 20)])).unwrap();
        let palette = PalFile::grayscale();
        let sheet = shp_to_sprite_sheet(&shp, &palette.colors, Some(0)).unwrap();
        assert_eq!(sheet.image.dimensions(), (6, 3));
        assert_eq!(sheet.frames.len(), 2);
        assert_eq!((sheet.frames[0].x, sheet.frames[1].x), (0, 2));
        assert_eq!(sheet.frames[1].w, 4);
        // Second frame starts where the first ends; unpainted rows stay transparent.
        assert_eq!(sheet.image.get_pixel(2, 0)[3], 255);
        assert_eq!(sheet.image.get_pixel(5, 2)[3], 0);
    }

    #[test]
    fn writes_frame_pngs_and_sheet_manifest() {
        let dir = test_dir("write");
        let shp_path = dir.join("E1.SHP");
        fs::write(&shp_path, synthetic_shp(&[(2, 2, 3), (2, 2, 4)])).unwrap();
        let palette = PalFile::grayscale();

        let frames_dir = dir.join("frames");
        let written = write_shp_frames(&shp_path, &frames_dir, &palette, Some(0)).unwrap();
        assert_eq!(written.len(), 2);
        assert!(written[0].ends_with("E1_frame000.png"));
        let loaded = image::open(&written[1]).unwrap().to_rgba8();
        assert_eq!(loaded.dimensions(), (2, 2));

        let sheet_png = dir.join("out").join("E1_sheet.png");
        let json = write_shp_sheet(&shp_path, &sheet_png, &palette, Some(0)).unwrap();
        assert!(sheet_png.exists());
        assert!(json.ends_with("E1_sheet.json"));
        let manifest: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&json).unwrap()).unwrap();
        assert_eq!(manifest["width"], 4);
        assert_eq!(manifest["frames"].as_array().unwrap().len(), 2);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn audio_conversion_handles_voc_and_aud() {
        let dir = test_dir("audio");
        let voc = dir.join("beep.voc");
        fs::write(&voc, synthetic_voc()).unwrap();
        let voc_wav = dir.join("beep.wav");
        write_audio_wav(&voc, &voc_wav).unwrap();
        assert_eq!(&fs::read(&voc_wav).unwrap()[..4], b"RIFF");

        let aud = dir.join("shot.aud");
        fs::write(&aud, synthetic_aud()).unwrap();
        let aud_wav = dir.join("shot.wav");
        write_audio_wav(&aud, &aud_wav).unwrap();
        assert_eq!(&fs::read(&aud_wav).unwrap()[..4], b"RIFF");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn mix_extraction_writes_entries() {
        let dir = test_dir("mix");
        let mix_path = dir.join("test.mix");
        fs::write(
            &mix_path,
            synthetic_mix(&[("A.INI", b"hello"), ("B.BIN", b"xyz")]),
        )
        .unwrap();
        let out = dir.join("out");
        let names = extract_mix(&mix_path, &out).unwrap();
        assert_eq!(names.len(), 2);
        assert_eq!(fs::read(out.join("A.INI")).unwrap(), b"hello");
        assert_eq!(fs::read(out.join("B.BIN")).unwrap(), b"xyz");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn palette_exports_are_consistent() {
        let palette = PalFile::grayscale();
        let json = palette_to_json(&palette).unwrap();
        assert!(json.contains("\"r\": 255"));
        assert_eq!(palette_to_css(&palette).lines().count(), 258);
        assert_eq!(palette_to_png(&palette).dimensions(), (16, 16));
    }

    #[test]
    fn batch_conversion_mirrors_tree_and_counts() {
        let dir = test_dir("batch");
        let input = dir.join("in");
        let nested = input.join("nested");
        fs::create_dir_all(&nested).unwrap();
        fs::write(input.join("A.SHP"), synthetic_shp(&[(1, 1, 5)])).unwrap();
        fs::write(nested.join("B.VOC"), synthetic_voc()).unwrap();
        fs::write(nested.join("C.AUD"), synthetic_aud()).unwrap();
        fs::write(nested.join("D.PAL"), PalFile::grayscale().to_bytes()).unwrap();
        fs::write(nested.join("E.TXT"), b"ignored").unwrap();

        let options = BatchOptions::default();
        let report = batch_convert(&input, &dir.join("out"), "td", &options).unwrap();
        assert_eq!(report.converted, 4);
        assert_eq!(report.skipped, 1);
        assert_eq!(report.failed(), 0);
        assert_eq!(report.count(AssetKind::Shp), 1);
        assert_eq!(report.count(AssetKind::Voc), 1);
        assert_eq!(report.count(AssetKind::Aud), 1);
        assert_eq!(report.count(AssetKind::Pal), 1);
        assert!(dir.join("out").join("A_frame000.png").exists());
        assert!(dir.join("out").join("nested").join("B.wav").exists());
        assert!(dir.join("out").join("nested").join("D.json").exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn batch_with_sprite_sheet_mode_and_bad_game() {
        let dir = test_dir("batch_sheet");
        let input = dir.join("in");
        fs::create_dir_all(&input).unwrap();
        fs::write(input.join("A.SHP"), synthetic_shp(&[(2, 2, 9)])).unwrap();
        let options = BatchOptions {
            sprite_sheet: true,
            ..BatchOptions::default()
        };
        let report = batch_convert(&input, &dir.join("out"), "ra", &options).unwrap();
        assert_eq!(report.converted, 1);
        assert!(dir.join("out").join("A_sheet.png").exists());
        assert!(batch_convert(&input, &dir.join("out2"), "quake", &options).is_err());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn batch_records_failures_without_aborting() {
        let dir = test_dir("batch_fail");
        let input = dir.join("in");
        fs::create_dir_all(&input).unwrap();
        fs::write(input.join("BAD.SHP"), b"not a shape").unwrap();
        fs::write(input.join("GOOD.SHP"), synthetic_shp(&[(1, 1, 1)])).unwrap();
        let report =
            batch_convert(&input, &dir.join("out"), "td", &BatchOptions::default()).unwrap();
        assert_eq!(report.converted, 1);
        assert_eq!(report.failed(), 1);
        assert_eq!(report.failures[0].0.file_name().unwrap(), "BAD.SHP");
        fs::remove_dir_all(&dir).unwrap();
    }
}

#[cfg(test)]
mod audio_tests {
    use super::*;
    use std::io::Cursor;

    fn sine_audio(rate: u32, frames: usize) -> PcmAudio {
        let samples = (0..frames)
            .map(|index| {
                let phase = (index as f32) * 440.0 * std::f32::consts::TAU / rate as f32;
                (phase.sin() * 12_000.0) as i16
            })
            .collect();
        PcmAudio {
            sample_rate: rate,
            channels: 1,
            samples,
            loop_points: None,
        }
    }

    fn read_u32(bytes: &[u8], offset: usize) -> u32 {
        u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
    }

    #[test]
    fn audio_loop_rejects_inverted_ranges() {
        assert!(AudioLoop::new(10, 10).is_err());
        assert!(AudioLoop::new(20, 10).is_err());
        let good = AudioLoop::new(10, 20).unwrap();
        assert_eq!(good.length(), 10);
    }

    #[test]
    fn audio_format_parses_names_and_extensions() {
        assert_eq!(AudioFormat::parse("WAV").unwrap(), AudioFormat::Wav);
        assert_eq!(AudioFormat::parse("ogg").unwrap(), AudioFormat::Ogg);
        assert_eq!(AudioFormat::parse("vorbis").unwrap(), AudioFormat::Ogg);
        assert!(AudioFormat::parse("mp3").is_err());
        assert_eq!(AudioFormat::Ogg.extension(), "ogg");
    }

    #[test]
    fn ogg_stream_is_valid_and_carries_loop_comments() {
        let audio = sine_audio(22_050, 4_410);
        let loop_points = AudioLoop::new(100, 4_000).unwrap();
        let ogg = audio_to_ogg(&audio, 0.3, Some(loop_points)).unwrap();

        assert_eq!(&ogg[..4], b"OggS");
        assert!(
            ogg.windows(13).any(|window| window == b"LOOPSTART=100"),
            "LOOPSTART comment missing"
        );
        assert!(
            ogg.windows(12).any(|window| window == b"LOOPEND=4000"),
            "LOOPEND comment missing"
        );
        assert!(
            ogg.windows(15).any(|window| window == b"LOOPLENGTH=3900"),
            "LOOPLENGTH comment missing"
        );

        let mut decoder = vorbis_rs::VorbisDecoder::new(Cursor::new(ogg)).unwrap();
        assert_eq!(decoder.channels().get(), 1);
        assert_eq!(decoder.sampling_frequency().get(), 22_050);
        let mut decoded_frames = 0usize;
        while let Some(block) = decoder.decode_audio_block().unwrap() {
            decoded_frames += block.samples()[0].len();
        }
        assert!(
            decoded_frames > 3_000,
            "decoded too few frames: {decoded_frames}"
        );
    }

    #[test]
    fn ogg_without_loop_points_has_no_loop_comments() {
        let audio = sine_audio(22_050, 1_024);
        let ogg = audio_to_ogg(&audio, 0.2, None).unwrap();
        assert!(!ogg.windows(9).any(|window| window == b"LOOPSTART"));
    }

    #[test]
    fn wav_smpl_chunk_encodes_loop_and_updates_riff_size() {
        let audio = sine_audio(22_050, 2_000);
        let loop_points = AudioLoop::new(50, 1_500).unwrap();
        let wav = audio_to_wav(&audio, Some(loop_points)).unwrap();

        assert_eq!(&wav[..4], b"RIFF");
        assert_eq!(&wav[8..12], b"WAVE");
        assert_eq!(read_u32(&wav, 4) as usize, wav.len() - 8);

        let smpl = wav
            .windows(4)
            .position(|window| window == b"smpl")
            .expect("smpl chunk missing");
        assert_eq!(read_u32(&wav, smpl + 4), 60);
        assert_eq!(read_u32(&wav, smpl + 8 + 8), 1_000_000_000 / 22_050);
        assert_eq!(read_u32(&wav, smpl + 8 + 28), 1, "number of loops");
        assert_eq!(read_u32(&wav, smpl + 8 + 44), 50, "loop start");
        assert_eq!(
            read_u32(&wav, smpl + 8 + 48),
            1_499,
            "loop end is inclusive"
        );
    }

    #[test]
    fn wav_without_loop_has_no_smpl_chunk() {
        let audio = sine_audio(22_050, 256);
        let wav = audio_to_wav(&audio, None).unwrap();
        assert!(!wav.windows(4).any(|window| window == b"smpl"));
    }

    #[test]
    fn write_audio_converts_aud_to_ogg_with_override() {
        let dir = test_dir("ogg_e2e");
        let aud = dir.join("shot.aud");
        let mut data = Vec::new();
        data.extend_from_slice(&22_050u32.to_le_bytes());
        data.extend_from_slice(&1u16.to_le_bytes());
        data.extend_from_slice(&0u16.to_le_bytes());
        for index in 0..2_048i16 {
            data.extend_from_slice(&(index % 1_000).to_le_bytes());
        }
        fs::write(&aud, data).unwrap();

        let ogg = dir.join("shot.ogg");
        let loop_points = AudioLoop::new(10, 1_000).unwrap();
        write_audio(&aud, &ogg, AudioFormat::Ogg, Some(loop_points), 0.3).unwrap();
        let bytes = fs::read(&ogg).unwrap();
        assert_eq!(&bytes[..4], b"OggS");
        assert!(bytes.windows(12).any(|window| window == b"LOOPEND=1000"));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn voc_loop_points_are_preserved_by_default() {
        // Block 0x09/0x0A carry loop start/end sample offsets.
        let dir = test_dir("voc_loop");
        let voc = dir.join("music.voc");
        let mut data = Vec::new();
        data.extend_from_slice(b"Creative Voice File\x1a");
        data.extend_from_slice(&26u16.to_le_bytes());
        data.extend_from_slice(&0x0100u16.to_le_bytes());
        data.extend_from_slice(&0u16.to_le_bytes());
        data.push(0x09);
        data.extend_from_slice(&4u32.to_le_bytes());
        data.extend_from_slice(&120u32.to_le_bytes());
        data.push(0x0A);
        data.extend_from_slice(&4u32.to_le_bytes());
        data.extend_from_slice(&900u32.to_le_bytes());
        data.push(0x01);
        data.extend_from_slice(&(2u32 + 64).to_le_bytes());
        data.push(0xD3); // time constant for ~22050 Hz
        data.push(0);
        for index in 0..64u8 {
            data.push(index);
        }
        data.push(0x00);
        data.extend_from_slice(&0u32.to_le_bytes());
        fs::write(&voc, data).unwrap();

        let audio = load_audio(&voc).unwrap();
        assert_eq!(audio.sample_rate, 22_222); // 1_000_000 / (256 - 0xD3)
        assert_eq!(audio.loop_points, Some(AudioLoop::new(120, 900).unwrap()));
        assert_eq!(audio.samples.len(), 64);

        let wav = dir.join("music.wav");
        write_audio(&voc, &wav, AudioFormat::Wav, None, 0.4).unwrap();
        let bytes = fs::read(&wav).unwrap();
        let smpl = bytes
            .windows(4)
            .position(|window| window == b"smpl")
            .unwrap();
        assert_eq!(read_u32(&bytes, smpl + 8 + 44), 120);
        fs::remove_dir_all(&dir).unwrap();
    }

    fn test_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "cnc_asset_audio_{}_{}_{}",
            name,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&dir).unwrap();
        dir
    }
}
