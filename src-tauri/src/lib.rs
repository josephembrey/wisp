mod audio;
mod commands;
mod engine;
mod hotkey;
mod output;
mod settings;
mod tray;
mod whisper;

use settings::{Settings, Status, WispState};
use std::sync::Arc;
use tauri::{Listener, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            app.handle().plugin(
                tauri_plugin_log::Builder::default()
                    .level(if cfg!(debug_assertions) {
                        log::LevelFilter::Info
                    } else {
                        log::LevelFilter::Warn
                    })
                    .build(),
            )?;

            let data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            let models_dir = data_dir.join("models");
            let first_run = !Settings::exists(&data_dir);

            let settings = Settings::load(&data_dir);
            if first_run {
                let _ = settings.save(&data_dir);
            }

            let initial_keys = hotkey::parse_combo(&settings.hotkey)
                .unwrap_or_else(|| vec![rdev::Key::Alt, rdev::Key::KeyQ]);
            let hotkey_key = Arc::new(parking_lot::Mutex::new(initial_keys));
            let initial_output_keys =
                hotkey::parse_combo(&settings.output_hotkey).unwrap_or_default();
            let output_hotkey_key = Arc::new(parking_lot::Mutex::new(initial_output_keys));

            let (tx, rx) = std::sync::mpsc::channel::<engine::AppEvent>();

            let (hotkey_tx, hotkey_rx) = std::sync::mpsc::channel();
            hotkey::start(
                hotkey_key.clone(),
                output_hotkey_key.clone(),
                hotkey_tx.clone(),
            );
            let tx_fwd = tx.clone();
            std::thread::spawn(move || {
                for e in hotkey_rx {
                    let _ = tx_fwd.send(engine::AppEvent::Hotkey(e));
                }
            });

            app.manage(WispState {
                settings: parking_lot::Mutex::new(settings),
                status: parking_lot::Mutex::new(Status::Idle),
                data_dir,
                models_dir,
                hotkey: hotkey_key,
                output_hotkey: output_hotkey_key,
                hotkey_tx,
                first_run,
            });

            let tx_reload = tx.clone();
            app.listen("reload-model", move |_| {
                let _ = tx_reload.send(engine::AppEvent::ReloadModel);
            });

            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                engine::run(app_handle, tx, rx);
            });

            tray::setup(app, first_run)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                if window.label() != "overlay" {
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::is_first_run,
            commands::get_settings,
            commands::update_settings,
            commands::get_status,
            commands::get_models,
            commands::download_model,
            commands::delete_model,
            commands::get_gpu_backend,
            commands::resize_window,
            commands::reset_app,
            commands::get_monitors,
            commands::get_input_devices,
            commands::quit,
            commands::hotkey_press,
            commands::hotkey_release,
            commands::output_toggle,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
