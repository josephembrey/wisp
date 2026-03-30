use crate::audio;
use crate::engine;
use crate::history;
use crate::settings::{OverlayState, OverlayStatus, Settings, WispState};
use crate::whisper;
use tauri::{Emitter, Manager};

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
    log::info!(
        "cmd: update_settings (model={} hotkey={} gpu={})",
        settings.model,
        settings.hotkey,
        settings.gpu
    );
    let old = state.settings.lock().clone();
    settings.save(&state.data_dir)?;

    let hotkey_changed =
        old.hotkey != settings.hotkey || old.output_hotkey != settings.output_hotkey;
    let model_changed = old.model != settings.model || old.gpu != settings.gpu;

    if model_changed {
        log::info!(
            "settings: model changed {}(gpu={}) -> {}(gpu={})",
            old.model,
            old.gpu,
            settings.model,
            settings.gpu
        );
        *state.settings.lock() = settings.clone();
        let _ = state.engine_tx.send(crate::engine::AppEvent::ReloadModel);
    } else {
        *state.settings.lock() = settings.clone();
    }

    if hotkey_changed {
        log::info!(
            "settings: hotkeys changed main='{}' output='{}'",
            settings.hotkey,
            settings.output_hotkey
        );
        crate::hotkey::register(&app, &settings.hotkey, &settings.output_hotkey);
    }

    if old.autostart != settings.autostart {
        crate::sync_autostart(&app, settings.autostart);
    }

    let _ = app.emit("settings-changed", &settings);

    engine::set_overlay(
        &app,
        OverlayState {
            status: OverlayStatus::Saved,
            ttl_ms: Some(750),
        },
    );

    Ok(())
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
    log::info!("cmd: download_model {}", name);
    whisper::download_model(app, &state.models_dir, &name).await
}

#[tauri::command]
#[specta::specta]
pub fn delete_model(
    app: tauri::AppHandle,
    state: tauri::State<'_, WispState>,
    name: String,
) -> Result<(), String> {
    log::info!("cmd: delete_model {}", name);
    whisper::delete_model(&state.models_dir, &name)?;
    engine::set_overlay(
        &app,
        OverlayState {
            status: OverlayStatus::Deleted,
            ttl_ms: Some(750),
        },
    );
    Ok(())
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

#[derive(serde::Serialize, specta::Type)]
pub struct MemoryInfo {
    pub total_mb: u64,
    pub available_mb: u64,
}

#[tauri::command]
#[specta::specta]
pub fn get_memory_info(gpu: bool) -> MemoryInfo {
    if gpu {
        if let Some(info) = get_gpu_memory() {
            return info;
        }
    }
    get_system_memory()
}

fn get_system_memory() -> MemoryInfo {
    let mut sys = sysinfo::System::new();
    sys.refresh_memory();
    MemoryInfo {
        total_mb: sys.total_memory() / (1024 * 1024),
        available_mb: sys.available_memory() / (1024 * 1024),
    }
}

#[cfg(target_os = "windows")]
fn get_gpu_memory() -> Option<MemoryInfo> {
    use windows::core::Interface;
    use windows::Win32::Graphics::Dxgi::*;

    unsafe {
        let factory: IDXGIFactory1 = CreateDXGIFactory1().ok()?;
        let adapter = factory.EnumAdapters1(0).ok()?;
        let desc = adapter.GetDesc1().ok()?;

        if desc.DedicatedVideoMemory == 0 {
            return None;
        }

        let total_mb = (desc.DedicatedVideoMemory / (1024 * 1024)) as u64;

        if let Ok(adapter3) = adapter.cast::<IDXGIAdapter3>() {
            let mut info = DXGI_QUERY_VIDEO_MEMORY_INFO::default();
            if adapter3
                .QueryVideoMemoryInfo(0, DXGI_MEMORY_SEGMENT_GROUP_LOCAL, &mut info)
                .is_ok()
            {
                let available_mb = info.Budget.saturating_sub(info.CurrentUsage) / (1024 * 1024);
                return Some(MemoryInfo {
                    total_mb,
                    available_mb,
                });
            }
        }

        Some(MemoryInfo {
            total_mb,
            available_mb: total_mb,
        })
    }
}

#[cfg(not(target_os = "windows"))]
fn get_gpu_memory() -> Option<MemoryInfo> {
    None
}

#[tauri::command]
#[specta::specta]
pub fn get_input_devices() -> Vec<audio::InputDeviceInfo> {
    audio::list_input_devices()
}

#[tauri::command]
#[specta::specta]
pub async fn transcribe_file(
    app: tauri::AppHandle,
    state: tauri::State<'_, WispState>,
    path: String,
) -> Result<String, String> {
    log::info!("transcribe_file: {}", path);
    let file_path = std::path::PathBuf::from(&path);
    if !file_path.exists() {
        return Err(format!("file not found: {}", path));
    }

    let _ = app.emit("transcribe-file-progress", "decoding");
    let audio = audio::decode_file(&file_path)?;

    let settings = state.settings.lock().clone();
    let model_path = whisper::model_path(&state.models_dir, &settings.model);
    if !model_path.exists() {
        return Err(format!("model '{}' not downloaded", settings.model));
    }

    let _ = app.emit("transcribe-file-progress", "loading");
    let engine = whisper::WhisperEngine::new(&model_path, settings.gpu)
        .map_err(|e| format!("failed to load model: {}", e))?;

    let _ = app.emit("transcribe-file-progress", "transcribing");
    let text = engine.transcribe(&audio, &settings.language, None)?;

    let _ = app.emit("transcribe-file-progress", "done");
    log::info!("transcribe_file: {} chars", text.len());

    if settings.history_enabled {
        history::append(&state.data_dir, &text, "file", settings.history_retention);
        let _ = app.emit("history-changed", ());
    }

    Ok(text)
}

#[tauri::command]
#[specta::specta]
pub fn get_history(state: tauri::State<'_, WispState>) -> Vec<history::HistoryEntry> {
    history::load(&state.data_dir)
}

#[tauri::command]
#[specta::specta]
pub fn clear_history(state: tauri::State<'_, WispState>) {
    log::info!("cmd: clear_history");
    history::clear(&state.data_dir);
}

#[tauri::command]
#[specta::specta]
pub fn delete_history_entry(state: tauri::State<'_, WispState>, id: u64) {
    log::info!("cmd: delete_history_entry id={}", id);
    history::delete_entry(&state.data_dir, id);
}

#[tauri::command]
#[specta::specta]
pub fn show_log_dir(app: tauri::AppHandle) -> Result<(), String> {
    let dir = app.path().app_log_dir().map_err(|e| e.to_string())?;
    log::info!("cmd: show_log_dir -> {}", dir.display());
    open::that(&dir).map_err(|e| format!("failed to open {}: {}", dir.display(), e))
}

#[tauri::command]
#[specta::specta]
pub fn quit(app: tauri::AppHandle) {
    log::info!("cmd: quit");
    app.exit(0);
}
