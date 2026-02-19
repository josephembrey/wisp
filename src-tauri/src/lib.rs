// Hotkey architecture:
//
// On macOS/Linux the `tauri-plugin-global-shortcut` handles both main PTT and
// output-toggle hotkeys.  On Windows the plugin's underlying `WM_HOTKEY`
// mechanism has a timing bug: its release-detection spin-loop only checks the
// *main* key, ignoring modifiers, which produces ghost Pressed events when a
// modifier is released first.  To work around this the main PTT hotkey on
// Windows is driven by a dedicated polling thread (`hotkey::start_ptt_polling`)
// that reads physical key state via `GetAsyncKeyState`.  The output-toggle
// hotkey still uses the plugin on all platforms since it only needs press
// detection (no release), so the bug does not apply.

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

                    // DISABLED FOR TESTING: only process main hotkey via plugin on non-Windows
                    // #[cfg(not(target_os = "windows"))]
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

            // DISABLED FOR TESTING: Windows polling workaround
            // #[cfg(target_os = "windows")]
            // hotkey::start_ptt_polling(app.handle().clone(), event_tx.clone());

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

pub fn register_shortcuts(app: &tauri::AppHandle, main_combo: &str, output_combo: &str) {
    use tauri_plugin_global_shortcut::GlobalShortcutExt;

    let gs = app.global_shortcut();
    let _ = gs.unregister_all();

    // DISABLED FOR TESTING: skip main hotkey on Windows (polling workaround)
    // #[cfg(target_os = "windows")]
    // let _ = main_combo;

    // #[cfg(not(target_os = "windows"))]
    if let Some(accel) = hotkey::to_accelerator(main_combo) {
        match accel.parse::<tauri_plugin_global_shortcut::Shortcut>() {
            Ok(shortcut) => {
                if let Err(e) = gs.register(shortcut) {
                    log::warn!("failed to register main hotkey '{}': {}", accel, e);
                } else {
                    log::info!("registered main hotkey: {}", accel);
                }
            }
            Err(e) => log::warn!("invalid main hotkey '{}': {}", accel, e),
        }
    }

    if let Some(accel) = hotkey::to_accelerator(output_combo) {
        match accel.parse::<tauri_plugin_global_shortcut::Shortcut>() {
            Ok(shortcut) => {
                if let Err(e) = gs.register(shortcut) {
                    log::warn!("failed to register output hotkey '{}': {}", accel, e);
                } else {
                    log::info!("registered output hotkey: {}", accel);
                }
            }
            Err(e) => log::warn!("invalid output hotkey '{}': {}", accel, e),
        }
    }
}
