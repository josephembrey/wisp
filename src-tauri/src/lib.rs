mod audio;
mod commands;
mod hotkey;
mod output;
mod state;
mod whisper;

use state::{OutputMode, Settings, Status, WispState};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Listener, Manager,
};

enum AppEvent {
    Hotkey(hotkey::HotkeyEvent),
    TranscriptionDone {
        engine: whisper::WhisperEngine,
        result: Result<String, String>,
        output_mode: OutputMode,
    },
    ReloadModel,
}

// WhisperEngine contains a WhisperContext which is Send but not marked as such by whisper-rs.
// We need to send it between threads for the async transcription path.
unsafe impl Send for AppEvent {}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // Another instance tried to launch — focus the existing window
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

            // Initialize app data directories
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            let models_dir = data_dir.join("models");

            // First run: show the window so the user can configure settings
            let first_run = !Settings::exists(&data_dir);

            // Load settings and create app state
            let settings = Settings::load(&data_dir);
            if first_run {
                let _ = settings.save(&data_dir);
            }
            let initial_keys =
                hotkey::parse_combo(&settings.hotkey).unwrap_or_else(|| vec![rdev::Key::AltGr]);
            let hotkey_key = Arc::new(parking_lot::Mutex::new(initial_keys));
            let initial_output_keys =
                hotkey::parse_combo(&settings.output_hotkey).unwrap_or_default();
            let output_hotkey_key = Arc::new(parking_lot::Mutex::new(initial_output_keys));

            app.manage(WispState {
                settings: parking_lot::Mutex::new(settings),
                status: parking_lot::Mutex::new(Status::Idle),
                data_dir,
                models_dir,
                hotkey: hotkey_key.clone(),
                output_hotkey: output_hotkey_key.clone(),
                first_run,
            });

            // Unified event channel
            let (tx, rx) = std::sync::mpsc::channel::<AppEvent>();

            // Start hotkey listener → forward into AppEvent channel
            let (hotkey_tx, hotkey_rx) = std::sync::mpsc::channel();
            hotkey::start(hotkey_key, output_hotkey_key, hotkey_tx);
            let tx_fwd = tx.clone();
            std::thread::spawn(move || {
                for e in hotkey_rx {
                    let _ = tx_fwd.send(AppEvent::Hotkey(e));
                }
            });

            // Forward reload-model Tauri events
            let tx_reload = tx.clone();
            app.listen("reload-model", move |_| {
                let _ = tx_reload.send(AppEvent::ReloadModel);
            });

            // Start event loop
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                run_event_loop(app_handle, tx, rx);
            });

            // Build tray menu
            let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&settings_item, &quit_item])?;

            // Build tray icon
            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
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
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        if let Some(window) = tray.app_handle().get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            if first_run {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                window.app_handle().exit(0);
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
            commands::quit,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn run_event_loop(
    app: tauri::AppHandle,
    tx: std::sync::mpsc::Sender<AppEvent>,
    rx: std::sync::mpsc::Receiver<AppEvent>,
) {
    let state = app.state::<WispState>();
    let mut engine: Option<whisper::WhisperEngine> = None;
    let mut loaded_model = String::new();
    let mut loaded_gpu = false;
    let mut recorder: Option<audio::AudioRecorder> = None;

    // Interrupt mode state
    let abort_flag = Arc::new(AtomicBool::new(false));
    let mut cancelled = false;
    let mut transcription_in_flight = false;
    let mut pending_audio: Option<(Vec<f32>, Settings)> = None;

    // Eagerly load the configured model at startup
    {
        let settings = state.settings.lock().clone();
        let model_file = state
            .models_dir
            .join(format!("ggml-{}.bin", settings.model));
        if model_file.exists() {
            set_status(&app, &state, Status::Loading);
            match whisper::WhisperEngine::new(&model_file, settings.gpu) {
                Ok(e) => {
                    log::info!("eagerly loaded model: {}", settings.model);
                    engine = Some(e);
                    loaded_model = settings.model.clone();
                    loaded_gpu = settings.gpu;
                }
                Err(e) => log::warn!("failed to eagerly load model: {}", e),
            }
            set_status(&app, &state, Status::Idle);
        }
    }

    for event in rx {
        match event {
            AppEvent::Hotkey(hotkey::HotkeyEvent::Pressed) => {
                let settings = state.settings.lock().clone();

                if settings.interrupt && transcription_in_flight {
                    // Interrupt: abort the in-flight transcription and start recording
                    abort_flag.store(true, Ordering::Relaxed);
                    cancelled = true;
                }

                match audio::AudioRecorder::start() {
                    Ok(rec) => {
                        recorder = Some(rec);
                        set_status(&app, &state, Status::Recording);
                    }
                    Err(e) => {
                        log::error!("failed to start recording: {}", e);
                        let _ = app.emit("backend-error", format!("Mic error: {}", e));
                    }
                }
            }
            AppEvent::Hotkey(hotkey::HotkeyEvent::Released) => {
                let Some(rec) = recorder.take() else {
                    continue;
                };

                set_status(&app, &state, Status::Processing);
                let audio = rec.stop();

                // Skip recordings shorter than min_duration
                let settings = state.settings.lock().clone();
                let min_samples = (settings.min_duration * 16_000.0) as usize;
                if audio.len() < min_samples {
                    log::info!(
                        "recording too short ({} samples, min {}), skipping",
                        audio.len(),
                        min_samples
                    );
                    if !transcription_in_flight {
                        set_status(&app, &state, Status::Idle);
                    }
                    continue;
                }

                // Check model is downloaded
                let model_file = state
                    .models_dir
                    .join(format!("ggml-{}.bin", settings.model));
                if !model_file.exists() {
                    log::error!("model not downloaded: {}", settings.model);
                    let _ = app.emit(
                        "backend-error",
                        format!("Model '{}' not downloaded", settings.model),
                    );
                    if !transcription_in_flight {
                        set_status(&app, &state, Status::Idle);
                    }
                    continue;
                }

                if settings.interrupt {
                    // Async transcription path
                    if let Some(eng) = engine.take() {
                        // Engine available — start transcription in thread
                        start_transcription(
                            eng,
                            audio,
                            &settings,
                            &abort_flag,
                            &tx,
                            &mut loaded_model,
                            &mut loaded_gpu,
                            &state.models_dir,
                        );
                        transcription_in_flight = true;
                    } else {
                        // Engine in use by transcription thread — queue audio
                        pending_audio = Some((audio, settings));
                    }
                } else {
                    // Synchronous path (no interrupt)
                    // Load or reload model if needed
                    if loaded_model != settings.model
                        || loaded_gpu != settings.gpu
                        || engine.is_none()
                    {
                        set_status(&app, &state, Status::Loading);
                        match whisper::WhisperEngine::new(&model_file, settings.gpu) {
                            Ok(e) => {
                                engine = Some(e);
                                loaded_model = settings.model.clone();
                                loaded_gpu = settings.gpu;
                            }
                            Err(e) => {
                                log::error!("failed to load model: {}", e);
                                let _ = app
                                    .emit("backend-error", format!("Failed to load model: {}", e));
                                set_status(&app, &state, Status::Idle);
                                continue;
                            }
                        }
                        set_status(&app, &state, Status::Processing);
                    }

                    if let Some(ref eng) = engine {
                        match eng.transcribe(&audio, &settings.language, None) {
                            Ok(text) if !text.is_empty() => {
                                if let Err(e) = output::send(&text, &settings.output_mode) {
                                    log::error!("output error: {}", e);
                                    let _ =
                                        app.emit("backend-error", format!("Output error: {}", e));
                                }
                                let _ = app.emit("transcription", &text);
                            }
                            Ok(_) => {}
                            Err(e) => {
                                log::error!("transcription error: {}", e);
                                let _ = app
                                    .emit("backend-error", format!("Transcription error: {}", e));
                            }
                        }
                    }

                    set_status(&app, &state, Status::Idle);
                }
            }
            AppEvent::Hotkey(hotkey::HotkeyEvent::OutputToggle) => {
                let mut settings = state.settings.lock().clone();
                settings.output_mode = match settings.output_mode {
                    OutputMode::Clipboard => OutputMode::Paste,
                    OutputMode::Paste => OutputMode::Clipboard,
                };
                let _ = settings.save(&state.data_dir);
                *state.settings.lock() = settings;
                let _ = app.emit("settings-changed", ());
            }
            AppEvent::TranscriptionDone {
                engine: returned_engine,
                result,
                output_mode,
            } => {
                transcription_in_flight = false;
                abort_flag.store(false, Ordering::Relaxed);

                if !cancelled {
                    // Output the result
                    match result {
                        Ok(ref text) if !text.is_empty() => {
                            if let Err(e) = output::send(text, &output_mode) {
                                log::error!("output error: {}", e);
                                let _ = app.emit("backend-error", format!("Output error: {}", e));
                            }
                            let _ = app.emit("transcription", text);
                        }
                        Ok(_) => {}
                        Err(ref e) => {
                            log::error!("transcription error: {}", e);
                            let _ =
                                app.emit("backend-error", format!("Transcription error: {}", e));
                        }
                    }
                }
                cancelled = false;

                // Check for pending audio
                if let Some((audio, settings)) = pending_audio.take() {
                    engine = Some(returned_engine);
                    start_transcription(
                        engine.take().unwrap(),
                        audio,
                        &settings,
                        &abort_flag,
                        &tx,
                        &mut loaded_model,
                        &mut loaded_gpu,
                        &state.models_dir,
                    );
                    transcription_in_flight = true;
                } else {
                    engine = Some(returned_engine);
                    // Only go idle if we're not recording
                    if recorder.is_none() {
                        set_status(&app, &state, Status::Idle);
                    }
                }
            }
            AppEvent::ReloadModel => {
                let settings = state.settings.lock().clone();
                let model_file = state
                    .models_dir
                    .join(format!("ggml-{}.bin", settings.model));
                if !model_file.exists() {
                    log::warn!("model not downloaded for reload: {}", settings.model);
                    continue;
                }

                set_status(&app, &state, Status::Loading);
                match whisper::WhisperEngine::new(&model_file, settings.gpu) {
                    Ok(e) => {
                        log::info!("reloaded model: {}", settings.model);
                        engine = Some(e);
                        loaded_model = settings.model.clone();
                        loaded_gpu = settings.gpu;
                    }
                    Err(e) => {
                        log::error!("failed to reload model: {}", e);
                        let _ = app.emit("backend-error", format!("Failed to load model: {}", e));
                    }
                }
                set_status(&app, &state, Status::Idle);
            }
        }
    }
}

fn start_transcription(
    mut eng: whisper::WhisperEngine,
    audio: Vec<f32>,
    settings: &Settings,
    abort_flag: &Arc<AtomicBool>,
    tx: &std::sync::mpsc::Sender<AppEvent>,
    loaded_model: &mut String,
    loaded_gpu: &mut bool,
    models_dir: &std::path::Path,
) {
    // Load or reload model if needed
    if *loaded_model != settings.model || *loaded_gpu != settings.gpu {
        let model_file = models_dir.join(format!("ggml-{}.bin", settings.model));
        if model_file.exists() {
            match whisper::WhisperEngine::new(&model_file, settings.gpu) {
                Ok(e) => {
                    eng = e;
                    *loaded_model = settings.model.clone();
                    *loaded_gpu = settings.gpu;
                }
                Err(e) => {
                    log::error!("failed to load model for transcription: {}", e);
                    let _ = tx.send(AppEvent::TranscriptionDone {
                        engine: eng,
                        result: Err(format!("Failed to load model: {}", e)),
                        output_mode: settings.output_mode.clone(),
                    });
                    return;
                }
            }
        }
    }

    let language = settings.language.clone();
    let output_mode = settings.output_mode.clone();
    let flag = abort_flag.clone();
    let tx = tx.clone();
    flag.store(false, Ordering::Relaxed);

    std::thread::spawn(move || {
        let result = eng.transcribe(&audio, &language, Some(flag));
        let _ = tx.send(AppEvent::TranscriptionDone {
            engine: eng,
            result,
            output_mode,
        });
    });
}

fn set_status(app: &tauri::AppHandle, state: &WispState, status: Status) {
    *state.status.lock() = status.clone();
    let _ = app.emit("status-changed", &status);
}
