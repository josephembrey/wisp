mod audio;
mod commands;
mod engine;
mod hotkey;
mod output;
mod settings;
mod tray;
mod whisper;

use settings::{Settings, Status, WispState};
use tauri::{Listener, Manager};
use tauri_plugin_global_shortcut::{Shortcut, ShortcutState};

pub fn specta_builder() -> tauri_specta::Builder<tauri::Wry> {
    tauri_specta::Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![
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
        ])
        .typ::<whisper::DownloadProgress>()
}

pub fn ts_export_config() -> specta_typescript::Typescript {
    specta_typescript::Typescript::default()
        .bigint(specta_typescript::BigIntExportBehavior::Number)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta_builder();

    #[cfg(debug_assertions)]
    builder
        .export(ts_export_config(), "../src/lib/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    let state = app.state::<WispState>();
                    let settings = state.settings.lock().clone();

                    let main_shortcut = hotkey::to_accelerator(&settings.hotkey)
                        .and_then(|s| s.parse::<Shortcut>().ok());
                    let output_shortcut = hotkey::to_accelerator(&settings.output_hotkey)
                        .and_then(|s| s.parse::<Shortcut>().ok());

                    if main_shortcut.as_ref() == Some(shortcut) {
                        match event.state() {
                            ShortcutState::Pressed => {
                                let _ = state.hotkey_tx.send(hotkey::HotkeyEvent::Pressed);
                            }
                            ShortcutState::Released => {
                                let _ = state.hotkey_tx.send(hotkey::HotkeyEvent::Released);
                            }
                        }
                    } else if output_shortcut.as_ref() == Some(shortcut) {
                        if event.state() == ShortcutState::Pressed {
                            let _ = state.hotkey_tx.send(hotkey::HotkeyEvent::OutputToggle);
                        }
                    }
                })
                .build(),
        )
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

            let (tx, rx) = std::sync::mpsc::channel::<engine::AppEvent>();

            let (event_tx, event_rx) = std::sync::mpsc::channel();
            let tx_fwd = tx.clone();
            std::thread::spawn(move || {
                for e in event_rx {
                    let _ = tx_fwd.send(engine::AppEvent::Hotkey(e));
                }
            });

            app.manage(WispState {
                settings: parking_lot::Mutex::new(settings.clone()),
                status: parking_lot::Mutex::new(Status::Idle),
                data_dir,
                models_dir,
                hotkey_tx: event_tx.clone(),
                first_run,
            });

            register_shortcuts(app.handle(), &settings.hotkey, &settings.output_hotkey);

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
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub(crate) fn register_shortcuts(
    app: &tauri::AppHandle,
    main_combo: &str,
    output_combo: &str,
) {
    use tauri_plugin_global_shortcut::GlobalShortcutExt;

    let gs = app.global_shortcut();
    let _ = gs.unregister_all();

    for (label, combo) in [("main", main_combo), ("output", output_combo)] {
        let Some(accel) = hotkey::to_accelerator(combo) else {
            continue;
        };
        match accel.parse::<tauri_plugin_global_shortcut::Shortcut>() {
            Ok(shortcut) => {
                if let Err(e) = gs.register(shortcut) {
                    log::warn!("failed to register {} hotkey '{}': {}", label, accel, e);
                } else {
                    log::info!("registered {} hotkey: {}", label, accel);
                }
            }
            Err(e) => log::warn!("invalid {} hotkey '{}': {}", label, accel, e),
        }
    }
}
