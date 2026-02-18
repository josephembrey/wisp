mod audio;
mod commands;
mod hotkey;
mod output;
mod state;
mod whisper;

use state::{Settings, Status, WispState};
use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Initialize app data directories
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            let models_dir = data_dir.join("models");

            // Load settings and create app state
            let settings = Settings::load(&data_dir);
            let initial_key = hotkey::parse_key(&settings.hotkey).unwrap_or(rdev::Key::AltGr);
            let hotkey_key = Arc::new(parking_lot::Mutex::new(initial_key));

            app.manage(WispState {
                settings: parking_lot::Mutex::new(settings),
                status: parking_lot::Mutex::new(Status::Idle),
                data_dir,
                models_dir,
                hotkey: hotkey_key.clone(),
            });

            // Start hotkey listener
            let (tx, rx) = std::sync::mpsc::channel();
            hotkey::start(hotkey_key, tx);

            // Start event loop
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                run_event_loop(app_handle, rx);
            });

            // Build tray menu
            let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&settings_item, &quit_item])?;

            // Build tray icon
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "settings" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_settings,
            commands::update_settings,
            commands::get_status,
            commands::get_models,
            commands::download_model,
            commands::delete_model,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn run_event_loop(app: tauri::AppHandle, rx: std::sync::mpsc::Receiver<hotkey::HotkeyEvent>) {
    let state = app.state::<WispState>();
    let mut engine: Option<whisper::WhisperEngine> = None;
    let mut loaded_model = String::new();
    let mut recorder: Option<audio::AudioRecorder> = None;

    for event in rx {
        match event {
            hotkey::HotkeyEvent::Pressed => match audio::AudioRecorder::start() {
                Ok(rec) => {
                    recorder = Some(rec);
                    set_status(&app, &state, Status::Recording);
                }
                Err(e) => log::error!("failed to start recording: {}", e),
            },
            hotkey::HotkeyEvent::Released => {
                let Some(rec) = recorder.take() else {
                    continue;
                };

                set_status(&app, &state, Status::Processing);
                let audio = rec.stop();
                let settings = state.settings.lock().clone();

                // Check model is downloaded
                let model_file = state
                    .models_dir
                    .join(format!("ggml-{}.bin", settings.model));
                if !model_file.exists() {
                    log::error!("model not downloaded: {}", settings.model);
                    set_status(&app, &state, Status::Idle);
                    continue;
                }

                // Load or reload model if needed
                if loaded_model != settings.model || engine.is_none() {
                    match whisper::WhisperEngine::new(&model_file) {
                        Ok(e) => {
                            engine = Some(e);
                            loaded_model = settings.model.clone();
                        }
                        Err(e) => {
                            log::error!("failed to load model: {}", e);
                            set_status(&app, &state, Status::Idle);
                            continue;
                        }
                    }
                }

                // Transcribe and output
                if let Some(ref eng) = engine {
                    match eng.transcribe(&audio) {
                        Ok(text) if !text.is_empty() => {
                            if let Err(e) = output::send(&text, &settings.output_mode) {
                                log::error!("output error: {}", e);
                            }
                            let _ = app.emit("transcription", &text);
                        }
                        Ok(_) => {}
                        Err(e) => log::error!("transcription error: {}", e),
                    }
                }

                set_status(&app, &state, Status::Idle);
            }
        }
    }
}

fn set_status(app: &tauri::AppHandle, state: &WispState, status: Status) {
    *state.status.lock() = status.clone();
    let _ = app.emit("status-changed", &status);
}
