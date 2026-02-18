use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    false
}

fn default_overlay_position() -> String {
    "top-center".to_string()
}

fn default_overlay_size() -> String {
    "medium".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputMode {
    Clipboard,
    Paste,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            model: "base".to_string(),
            output_mode: OutputMode::Clipboard,
            hotkey: "Alt+KeyQ".to_string(),
            language: "en".to_string(),
            gpu: true,
            interrupt: false,
            output_hotkey: String::new(),
            min_duration: 0.5,
            overlay_enabled: false,
            overlay_position: "top-center".to_string(),
            overlay_size: "medium".to_string(),
            overlay_monitor: 0,
            overlay_always_show: false,
            input_device: String::new(),
        }
    }
}

impl Settings {
    pub fn exists(data_dir: &PathBuf) -> bool {
        data_dir.join("settings.json").exists()
    }

    pub fn load(data_dir: &PathBuf) -> Self {
        let path = data_dir.join("settings.json");
        if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Settings::default()
        }
    }

    pub fn save(&self, data_dir: &PathBuf) -> Result<(), String> {
        fs::create_dir_all(data_dir).map_err(|e| e.to_string())?;
        let path = data_dir.join("settings.json");
        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(&path, content).map_err(|e| e.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Idle,
    Loading,
    Recording,
    Processing,
}

pub struct WispState {
    pub settings: Mutex<Settings>,
    pub status: Mutex<Status>,
    pub data_dir: PathBuf,
    pub models_dir: PathBuf,
    pub hotkey: Arc<Mutex<Vec<rdev::Key>>>,
    pub output_hotkey: Arc<Mutex<Vec<rdev::Key>>>,
    pub hotkey_tx: std::sync::mpsc::Sender<crate::hotkey::HotkeyEvent>,
    pub first_run: bool,
}
