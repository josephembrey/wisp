use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::Arc;

const WHISPER_SAMPLE_RATE: u32 = 16_000;

#[derive(serde::Serialize)]
pub struct InputDeviceInfo {
    pub name: String,
    pub label: String,
}

#[allow(deprecated)]
pub fn list_input_devices() -> Vec<InputDeviceInfo> {
    let host = cpal::default_host();
    let default_name = host
        .default_input_device()
        .and_then(|d| d.name().ok())
        .unwrap_or_default();
    host.input_devices()
        .map(|devices| {
            devices
                .filter_map(|d| {
                    let name = d.name().ok()?;
                    let is_default = name == default_name;
                    let detail = d.default_input_config().ok().map(|c| {
                        let ch = if c.channels() == 1 { "mono" } else { "stereo" };
                        let rate = c.sample_rate() / 1000;
                        format!("{rate}kHz {ch}")
                    });
                    let mut label = name.clone();
                    if is_default {
                        label.push_str(" (Default)");
                    }
                    if let Some(detail) = detail {
                        label.push_str(" - ");
                        label.push_str(&detail);
                    }
                    Some(InputDeviceInfo { name, label })
                })
                .collect()
        })
        .unwrap_or_default()
}

pub struct AudioRecorder {
    stream: cpal::Stream,
    buffer: Arc<parking_lot::Mutex<Vec<f32>>>,
    channels: u16,
    sample_rate: u32,
}

impl AudioRecorder {
    #[allow(deprecated)]
    pub fn start(device_name: &str) -> Result<Self, String> {
        let host = cpal::default_host();
        let device = if device_name.is_empty() {
            host.default_input_device().ok_or("no input device found")?
        } else {
            host.input_devices()
                .map_err(|e| e.to_string())?
                .find(|d| d.name().map(|n| n == device_name).unwrap_or(false))
                .ok_or_else(|| format!("input device '{}' not found, using default", device_name))
                .or_else(|e| {
                    log::warn!("{}", e);
                    host.default_input_device()
                        .ok_or("no input device found".to_string())
                })?
        };

        let config = device.default_input_config().map_err(|e| e.to_string())?;

        let channels = config.channels();
        let sample_rate = config.sample_rate();
        let buffer: Arc<parking_lot::Mutex<Vec<f32>>> =
            Arc::new(parking_lot::Mutex::new(Vec::new()));
        let buf = buffer.clone();

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    buf.lock().extend_from_slice(data);
                },
                |err| log::error!("audio stream error: {}", err),
                None,
            ),
            cpal::SampleFormat::I16 => device.build_input_stream(
                &config.into(),
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    let floats: Vec<f32> =
                        data.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
                    buf.lock().extend_from_slice(&floats);
                },
                |err| log::error!("audio stream error: {}", err),
                None,
            ),
            format => return Err(format!("unsupported sample format: {:?}", format)),
        }
        .map_err(|e| e.to_string())?;

        stream.play().map_err(|e| e.to_string())?;

        Ok(Self {
            stream,
            buffer,
            channels,
            sample_rate,
        })
    }

    pub fn stop(self) -> Vec<f32> {
        drop(self.stream);
        let raw = std::mem::take(&mut *self.buffer.lock());
        let mono = to_mono(&raw, self.channels);
        resample(&mono, self.sample_rate, WHISPER_SAMPLE_RATE)
    }
}

fn to_mono(samples: &[f32], channels: u16) -> Vec<f32> {
    if channels == 1 {
        return samples.to_vec();
    }
    let ch = channels as usize;
    samples
        .chunks_exact(ch)
        .map(|frame| frame.iter().sum::<f32>() / ch as f32)
        .collect()
}

fn resample(samples: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
    if from_rate == to_rate {
        return samples.to_vec();
    }
    let ratio = from_rate as f64 / to_rate as f64;
    let output_len = (samples.len() as f64 / ratio) as usize;
    (0..output_len)
        .map(|i| {
            let src = i as f64 * ratio;
            let idx = src as usize;
            let frac = src - idx as f64;
            let a = samples[idx];
            let b = samples.get(idx + 1).copied().unwrap_or(a);
            a + (b - a) * frac as f32
        })
        .collect()
}
