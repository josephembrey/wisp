use crate::audio;
use crate::hotkey;
use crate::state::{Settings, Status, WispState};
use crate::whisper;
use tauri::Emitter;

#[tauri::command]
pub fn is_first_run(state: tauri::State<'_, WispState>) -> bool {
    state.first_run
}

#[tauri::command]
pub fn get_settings(state: tauri::State<'_, WispState>) -> Settings {
    state.settings.lock().clone()
}

#[tauri::command]
pub fn update_settings(
    app: tauri::AppHandle,
    state: tauri::State<'_, WispState>,
    settings: Settings,
) -> Result<(), String> {
    let old = state.settings.lock().clone();
    settings.save(&state.data_dir)?;

    // Update main hotkey
    if let Some(keys) = hotkey::parse_combo(&settings.hotkey) {
        *state.hotkey.lock() = keys;
    } else {
        *state.hotkey.lock() = Vec::new();
    }

    // Update output hotkey
    *state.output_hotkey.lock() = hotkey::parse_combo(&settings.output_hotkey).unwrap_or_default();

    // Emit reload-model if model or GPU changed
    if old.model != settings.model || old.gpu != settings.gpu {
        *state.settings.lock() = settings;
        let _ = app.emit("reload-model", ());
    } else {
        *state.settings.lock() = settings;
    }

    let _ = app.emit("settings-changed", ());

    Ok(())
}

#[tauri::command]
pub fn get_status(state: tauri::State<'_, WispState>) -> Status {
    state.status.lock().clone()
}

#[tauri::command]
pub fn get_models(state: tauri::State<'_, WispState>) -> Vec<whisper::ModelInfo> {
    whisper::list_models(&state.models_dir)
}

#[tauri::command]
pub async fn download_model(
    app: tauri::AppHandle,
    state: tauri::State<'_, WispState>,
    name: String,
) -> Result<(), String> {
    whisper::download_model(app, &state.models_dir, &name).await
}

#[tauri::command]
pub fn delete_model(state: tauri::State<'_, WispState>, name: String) -> Result<(), String> {
    whisper::delete_model(&state.models_dir, &name)
}

#[tauri::command]
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
pub fn reset_app(app: tauri::AppHandle, state: tauri::State<'_, WispState>) -> Result<(), String> {
    // Delete settings file
    let settings_path = state.data_dir.join("settings.json");
    if settings_path.exists() {
        std::fs::remove_file(&settings_path).map_err(|e| e.to_string())?;
    }
    // Delete models directory
    if state.models_dir.exists() {
        std::fs::remove_dir_all(&state.models_dir).map_err(|e| e.to_string())?;
    }
    // Restart the app
    app.restart();
}

#[tauri::command]
pub fn resize_window(window: tauri::WebviewWindow, height: f64) {
    let scale = window.scale_factor().unwrap_or(1.0);
    let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize::new(
        (540.0 * scale) as u32,
        (height * scale) as u32,
    )));
}

#[tauri::command]
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

#[derive(serde::Serialize)]
pub struct MonitorInfo {
    pub index: usize,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub primary: bool,
}

#[tauri::command]
pub fn get_input_devices() -> Vec<audio::InputDeviceInfo> {
    audio::list_input_devices()
}

#[tauri::command]
pub fn quit(app: tauri::AppHandle) {
    app.exit(0);
}

#[tauri::command]
pub fn hotkey_press(state: tauri::State<'_, WispState>) {
    let _ = state.hotkey_tx.send(crate::hotkey::HotkeyEvent::Pressed);
}

#[tauri::command]
pub fn hotkey_release(state: tauri::State<'_, WispState>) {
    let _ = state.hotkey_tx.send(crate::hotkey::HotkeyEvent::Released);
}

#[tauri::command]
pub fn output_toggle(state: tauri::State<'_, WispState>) {
    let _ = state.hotkey_tx.send(crate::hotkey::HotkeyEvent::OutputToggle);
}
