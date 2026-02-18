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
            hotkey: "RightAlt".to_string(),
        }
    }
}

impl Settings {
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
    Recording,
    Processing,
}

pub struct WispState {
    pub settings: Mutex<Settings>,
    pub status: Mutex<Status>,
    pub data_dir: PathBuf,
    pub models_dir: PathBuf,
    pub hotkey: Arc<Mutex<rdev::Key>>,
}
