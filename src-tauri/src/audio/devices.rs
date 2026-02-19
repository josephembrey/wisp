use cpal::traits::{DeviceTrait, HostTrait};

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
