mod migrate;
mod state;
mod types;

pub use state::WispState;
pub use types::{
    ModelLoading, OutputMode, OverlayPosition, OverlaySize, OverlayState, OverlayStatus,
};

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(default)]
pub struct Settings {
    pub model: String,
    pub output_mode: OutputMode,
    pub hotkey: String,
    pub language: String,
    pub gpu: bool,
    pub interrupt: bool,
    pub output_hotkey: String,
    pub min_duration: f64,
    pub overlay_enabled: bool,
    pub overlay_position: OverlayPosition,
    pub overlay_size: OverlaySize,
    pub overlay_monitor: usize,
    pub overlay_always_show: bool,
    pub input_device: String,
    pub model_loading: ModelLoading,
    pub autostart: bool,
    pub history_enabled: bool,
    pub history_retention: usize,
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
            overlay_position: OverlayPosition::default(),
            overlay_size: OverlaySize::default(),
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

/// Apply side-effects when settings change: reload model, re-register hotkeys, sync autostart.
pub fn apply_settings_diff(
    old: &Settings,
    new: &Settings,
    worker_tx: &std::sync::mpsc::Sender<crate::whisper::worker::WorkerMessage>,
    app: &tauri::AppHandle,
) {
    if old.model != new.model || old.gpu != new.gpu {
        log::info!(
            "settings: model changed {}(gpu={}) -> {}(gpu={})",
            old.model,
            old.gpu,
            new.model,
            new.gpu
        );
        let _ = worker_tx.send(crate::whisper::worker::WorkerMessage::Reload {
            model: new.model.clone(),
            gpu: new.gpu,
        });
    }

    if old.hotkey != new.hotkey || old.output_hotkey != new.output_hotkey {
        log::info!(
            "settings: hotkeys changed main='{}' output='{}'",
            new.hotkey,
            new.output_hotkey
        );
        crate::hotkey::register(app, &new.hotkey, &new.output_hotkey);
    }

    if old.autostart != new.autostart {
        crate::sync_autostart(app, new.autostart);
    }
}
