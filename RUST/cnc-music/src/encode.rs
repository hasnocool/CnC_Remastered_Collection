//! OGG Vorbis encoding of rendered sample buffers.
//!
//! Determinism scope: the *samples* are a pure function of the MIDI input
//! (see [`crate::synth`]); the OGG container may vary in the stream serial
//! number, which `vorbis_rs` randomizes by default. Everything else is
//! reproducible. Tests therefore assert on structure, not exact bytes.

use std::path::Path;

/// Feed block size (frames). libvorbis buffers internally, so this is a
/// throughput knob, not a format parameter.
const BLOCK: usize = 1024;

/// Encode mono f32 samples to an OGG Vorbis byte stream.
pub fn encode_ogg(samples: &[f32], sample_rate: u32) -> anyhow::Result<Vec<u8>> {
    let mut sink: Vec<u8> = Vec::new();
    {
        let rate = std::num::NonZeroU32::new(sample_rate)
            .ok_or_else(|| anyhow::anyhow!("sample rate must be non-zero"))?;
        let channels = std::num::NonZeroU8::new(1).expect("mono channel");
        let mut encoder =
            vorbis_rs::VorbisEncoderBuilder::new(rate, channels, &mut sink)?.build()?;
        for block in samples.chunks(BLOCK) {
            // Planar layout: one Vec<f32> per channel (mono here).
            encoder.encode_audio_block(vec![block.to_vec()])?;
        }
        encoder.finish()?;
    }
    Ok(sink)
}

/// Write OGG Vorbis to `path` atomically (temp file + rename).
pub fn write_ogg(path: &Path, samples: &[f32], sample_rate: u32) -> anyhow::Result<()> {
    let bytes = encode_ogg(samples, sample_rate)?;
    let tmp = path.with_extension("ogg.tmp");
    std::fs::write(&tmp, &bytes)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_ogg_with_header() {
        let samples: Vec<f32> = (0..8_000).map(|i| (i as f32 * 0.01).sin() * 0.5).collect();
        let bytes = encode_ogg(&samples, 22_050).expect("encode");
        assert_eq!(&bytes[..4], b"OggS", "missing ogg capture pattern");
        assert!(
            bytes.len() > 1024,
            "encoded stream suspiciously small: {}",
            bytes.len()
        );
    }

    #[test]
    fn encoding_is_deterministic_in_size() {
        let samples: Vec<f32> = (0..4_000).map(|i| (i as f32 * 0.02).cos() * 0.3).collect();
        let a = encode_ogg(&samples, 22_050).expect("first");
        let b = encode_ogg(&samples, 22_050).expect("second");
        assert_eq!(a.len(), b.len(), "size drifted between identical encodes");
    }

    #[test]
    fn empty_input_still_produces_a_stream() {
        let bytes = encode_ogg(&[], 22_050).expect("empty encode");
        assert_eq!(&bytes[..4], b"OggS");
    }

    #[test]
    fn write_ogg_round_trips_to_disk() {
        let samples: Vec<f32> = (0..4_000).map(|i| (i as f32 * 0.013).sin() * 0.4).collect();
        let dir = std::env::temp_dir().join("cnc_music_test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("roundtrip_test.ogg");
        write_ogg(&path, &samples, 22_050).expect("write");
        let written = std::fs::read(&path).expect("read back");
        assert_eq!(&written[..4], b"OggS");
        assert_eq!(written.len(), encode_ogg(&samples, 22_050).unwrap().len());
    }
}
