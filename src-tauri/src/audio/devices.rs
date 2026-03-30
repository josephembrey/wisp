use cpal::traits::{DeviceTrait, HostTrait};

#[derive(serde::Serialize, specta::Type)]
pub struct InputDeviceInfo {
    pub name: String,
    pub label: String,
}

/// Clean up device names from OS/driver strings.
fn clean_name(name: &str) -> String {
    name.replace("(R)", "")
        .replace("(TM)", "")
        .replace("®", "")
        .replace("™", "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
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
                    let mut label = clean_name(&name);
                    if let Some(detail) = detail {
                        label = format!("{label} · {detail}");
                    }
                    if is_default {
                        label = format!("{label} (Default)");
                    }
                    Some(InputDeviceInfo { name, label })
                })
                .collect()
        })
        .unwrap_or_default()
}
