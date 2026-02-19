use crate::audio;
use crate::settings::{Settings, Status, WispState};
use crate::whisper;
use tauri::Emitter;

#[tauri::command]
#[specta::specta]
pub fn is_first_run(state: tauri::State<'_, WispState>) -> bool {
    state.first_run
}

#[tauri::command]
#[specta::specta]
pub fn get_settings(state: tauri::State<'_, WispState>) -> Settings {
    state.settings.lock().clone()
}

#[tauri::command]
#[specta::specta]
pub fn update_settings(
    app: tauri::AppHandle,
    state: tauri::State<'_, WispState>,
    settings: Settings,
) -> Result<(), String> {
    let old = state.settings.lock().clone();
    settings.save(&state.data_dir)?;

    let hotkey_changed =
        old.hotkey != settings.hotkey || old.output_hotkey != settings.output_hotkey;
    let model_changed = old.model != settings.model || old.gpu != settings.gpu;

    if model_changed {
        log::info!(
            "settings: model changed {}(gpu={}) -> {}(gpu={})",
            old.model, old.gpu, settings.model, settings.gpu
        );
        *state.settings.lock() = settings.clone();
        let _ = app.emit("reload-model", ());
    } else {
        *state.settings.lock() = settings.clone();
    }

    if hotkey_changed {
        log::info!(
            "settings: hotkeys changed main='{}' output='{}'",
            settings.hotkey, settings.output_hotkey
        );
        crate::register_shortcuts(&app, &settings.hotkey, &settings.output_hotkey);
    }

    let _ = app.emit("settings-changed", &settings);

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn get_status(state: tauri::State<'_, WispState>) -> Status {
    state.status.lock().clone()
}

#[tauri::command]
#[specta::specta]
pub fn get_models(state: tauri::State<'_, WispState>) -> Vec<whisper::ModelInfo> {
    whisper::list_models(&state.models_dir)
}

#[tauri::command]
#[specta::specta]
pub async fn download_model(
    app: tauri::AppHandle,
    state: tauri::State<'_, WispState>,
    name: String,
) -> Result<(), String> {
    log::info!("download requested: {}", name);
    whisper::download_model(app, &state.models_dir, &name).await
}

#[tauri::command]
#[specta::specta]
pub fn delete_model(state: tauri::State<'_, WispState>, name: String) -> Result<(), String> {
    log::info!("delete requested: {}", name);
    whisper::delete_model(&state.models_dir, &name)
}

#[tauri::command]
#[specta::specta]
pub fn get_gpu_backend() -> String {
    if cfg!(target_os = "macos") {
        "Metal".to_string()
    } else if cfg!(target_os = "linux") || cfg!(target_os = "windows") {
        "Vulkan".to_string()
    } else {
        "CPU".to_string()
    }
}

#[tauri::command]
#[specta::specta]
pub fn reset_app(app: tauri::AppHandle, state: tauri::State<'_, WispState>) -> Result<(), String> {
    log::warn!("resetting app: deleting settings and models");
    let settings_path = state.data_dir.join("settings.json");
    if settings_path.exists() {
        std::fs::remove_file(&settings_path).map_err(|e| e.to_string())?;
    }
    if state.models_dir.exists() {
        std::fs::remove_dir_all(&state.models_dir).map_err(|e| e.to_string())?;
    }
    app.restart();
}

#[tauri::command]
#[specta::specta]
pub fn resize_window(window: tauri::WebviewWindow, height: f64) {
    let scale = window.scale_factor().unwrap_or(1.0);
    let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize::new(
        (540.0 * scale) as u32,
        (height * scale) as u32,
    )));
}

#[tauri::command]
#[specta::specta]
pub fn get_monitors(app: tauri::AppHandle) -> Vec<MonitorInfo> {
    use tauri::Manager;
    let Some(window) = app.get_webview_window("main") else {
        return vec![];
    };
    let primary = window.primary_monitor().ok().flatten();
    let monitors = window.available_monitors().unwrap_or_default();
    monitors
        .into_iter()
        .enumerate()
        .map(|(i, m)| {
            let is_primary = primary
                .as_ref()
                .map(|p| p.position() == m.position() && p.size() == m.size())
                .unwrap_or(false);
            MonitorInfo {
                index: i,
                name: m.name().cloned().unwrap_or_default(),
                width: m.size().width,
                height: m.size().height,
                primary: is_primary,
            }
        })
        .collect()
}

#[derive(serde::Serialize, specta::Type)]
pub struct MonitorInfo {
    pub index: usize,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub primary: bool,
}

#[tauri::command]
#[specta::specta]
pub fn get_input_devices() -> Vec<audio::InputDeviceInfo> {
    audio::list_input_devices()
}

#[tauri::command]
#[specta::specta]
pub fn quit(app: tauri::AppHandle) {
    app.exit(0);
}
