use std::path::Path;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use super::resample::{resample, to_mono};

const WHISPER_SAMPLE_RATE: u32 = 16_000;

/// Decode an audio file to mono f32 samples at 16 kHz.
pub fn decode_file(path: &Path) -> Result<Vec<f32>, String> {
    let file = std::fs::File::open(path).map_err(|e| format!("failed to open file: {}", e))?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| format!("unsupported audio format: {}", e))?;

    let mut format = probed.format;

    let track = format.default_track().ok_or("no audio track found")?;

    let channels = track
        .codec_params
        .channels
        .map(|c| c.count() as u16)
        .unwrap_or(1);
    let sample_rate = track
        .codec_params
        .sample_rate
        .unwrap_or(WHISPER_SAMPLE_RATE);
    let track_id = track.id;

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|e| format!("unsupported codec: {}", e))?;

    let mut all_samples: Vec<f32> = Vec::new();

    loop {
        let packet = match format.next_packet() {
            Ok(p) => p,
            Err(symphonia::core::errors::Error::IoError(ref e))
                if e.kind() == std::io::ErrorKind::UnexpectedEof =>
            {
                break;
            }
            Err(e) => return Err(format!("decode error: {}", e)),
        };

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = match decoder.decode(&packet) {
            Ok(d) => d,
            Err(symphonia::core::errors::Error::DecodeError(msg)) => {
                log::warn!("decode: skipping corrupt frame: {}", msg);
                continue;
            }
            Err(e) => return Err(format!("decode error: {}", e)),
        };

        let spec = *decoded.spec();
        let duration = decoded.capacity();
        let mut buf = SampleBuffer::<f32>::new(duration as u64, spec);
        buf.copy_interleaved_ref(decoded);
        all_samples.extend_from_slice(buf.samples());
    }

    if all_samples.is_empty() {
        return Err("audio file contains no samples".to_string());
    }

    let mono = to_mono(&all_samples, channels);
    let resampled = resample(&mono, sample_rate, WHISPER_SAMPLE_RATE);

    log::info!(
        "decoded file: {}ch {}Hz -> mono 16kHz, {} samples ({:.1}s)",
        channels,
        sample_rate,
        resampled.len(),
        resampled.len() as f64 / WHISPER_SAMPLE_RATE as f64
    );

    Ok(resampled)
}
