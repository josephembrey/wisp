mod migrate;
mod state;
mod types;

pub use state::WispState;
pub use types::{ModelLoading, OutputMode, OverlayState, OverlayStatus};

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct Settings {
    pub model: String,
    pub output_mode: OutputMode,
    pub hotkey: String,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_gpu")]
    pub gpu: bool,
    #[serde(default)]
    pub interrupt: bool,
    #[serde(default)]
    pub output_hotkey: String,
    #[serde(default = "default_min_duration")]
    pub min_duration: f64,
    #[serde(default = "default_overlay_enabled")]
    pub overlay_enabled: bool,
    #[serde(default = "default_overlay_position")]
    pub overlay_position: String,
    #[serde(default = "default_overlay_size")]
    pub overlay_size: String,
    #[serde(default)]
    pub overlay_monitor: usize,
    #[serde(default)]
    pub overlay_always_show: bool,
    #[serde(default)]
    pub input_device: String,
    #[serde(default)]
    pub model_loading: ModelLoading,
    #[serde(default)]
    pub autostart: bool,
    #[serde(default = "default_history_enabled")]
    pub history_enabled: bool,
    #[serde(default = "default_history_retention")]
    pub history_retention: usize,
}

fn default_language() -> String {
    "en".to_string()
}

fn default_gpu() -> bool {
    true
}

fn default_min_duration() -> f64 {
    0.5
}

fn default_overlay_enabled() -> bool {
    true
}

fn default_overlay_position() -> String {
    "top-right".to_string()
}

fn default_overlay_size() -> String {
    "medium".to_string()
}

fn default_history_enabled() -> bool {
    true
}

fn default_history_retention() -> usize {
    100
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            model: "base".to_string(),
            output_mode: OutputMode::Paste,
            hotkey: "Alt+Q".to_string(),
            language: "en".to_string(),
            gpu: true,
            interrupt: false,
            output_hotkey: String::new(),
            min_duration: 0.5,
            overlay_enabled: true,
            overlay_position: "top-right".to_string(),
            overlay_size: "medium".to_string(),
            overlay_monitor: 0,
            overlay_always_show: false,
            input_device: String::new(),
            model_loading: ModelLoading::Eager,
            autostart: false,
            history_enabled: true,
            history_retention: 100,
        }
    }
}

impl Settings {
    pub fn exists(data_dir: &Path) -> bool {
        data_dir.join("settings.json").exists()
    }

    pub fn load(data_dir: &Path) -> Self {
        let path = data_dir.join("settings.json");
        let mut settings = if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Settings::default()
        };
        if migrate::migrate(&mut settings) {
            let _ = settings.save(data_dir);
        }
        settings
    }

    pub fn save(&self, data_dir: &Path) -> Result<(), String> {
        fs::create_dir_all(data_dir).map_err(|e| e.to_string())?;
        let path = data_dir.join("settings.json");
        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(&path, content).map_err(|e| e.to_string())
    }
}
