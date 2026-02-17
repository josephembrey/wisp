use crate::state::{Settings, Status, WispState};

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
