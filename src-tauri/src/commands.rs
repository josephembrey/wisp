use crate::state::{Settings, Status, WispState};
use crate::whisper;

#[tauri::command]
pub fn get_settings(state: tauri::State<'_, WispState>) -> Settings {
    state.settings.lock().clone()
}

#[tauri::command]
pub fn update_settings(
    state: tauri::State<'_, WispState>,
    settings: Settings,
) -> Result<(), String> {
    settings.save(&state.data_dir)?;
    *state.settings.lock() = settings;
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
