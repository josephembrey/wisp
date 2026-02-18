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
pub fn quit(app: tauri::AppHandle) {
    app.exit(0);
}
