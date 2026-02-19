use crate::audio;
use crate::hotkey;
use crate::output;
use crate::settings::{ModelLoading, OutputMode, Settings, Status, WispState};
use crate::whisper;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{Emitter, Manager};

pub(crate) enum AppEvent {
    Hotkey(hotkey::HotkeyEvent),
    TranscriptionDone {
        engine: whisper::WhisperEngine,
        result: Result<String, String>,
        output_mode: OutputMode,
    },
    ReloadModel,
}

// WhisperEngine contains a WhisperContext which is Send but not marked as such by whisper-rs.
unsafe impl Send for AppEvent {}

pub fn run(
    app: tauri::AppHandle,
    tx: std::sync::mpsc::Sender<AppEvent>,
    rx: std::sync::mpsc::Receiver<AppEvent>,
) {
    let state = app.state::<WispState>();
    let mut engine: Option<whisper::WhisperEngine> = None;
    let mut loaded_model = String::new();
    let mut loaded_gpu = false;
    let mut recorder: Option<audio::AudioRecorder> = None;

    let abort_flag = Arc::new(AtomicBool::new(false));
    let mut cancelled = false;
    let mut transcription_in_flight = false;
    let mut pending_audio: Option<(Vec<f32>, Settings)> = None;

    // Eagerly load the configured model at startup
    {
        let settings = state.settings.lock().clone();
        if settings.model_loading == ModelLoading::Eager {
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
    }

    for event in rx {
        match event {
            AppEvent::Hotkey(hotkey::HotkeyEvent::Pressed) => {
                if recorder.is_some() {
                    continue;
                }

                let settings = state.settings.lock().clone();

                if settings.interrupt && transcription_in_flight {
                    abort_flag.store(true, Ordering::Relaxed);
                    cancelled = true;
                }

                match audio::AudioRecorder::start(&settings.input_device) {
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

                let settings = state.settings.lock().clone();
                let min_samples = (settings.min_duration * 16_000.0) as usize;
                if audio.len() < min_samples {
                    log::info!(
                        "recording too short ({} samples, min {}), skipping",
                        audio.len(),
                        min_samples
                    );
                    let _ = app.emit("overlay-flash", "Cancelled");
                    if !transcription_in_flight {
                        set_status(&app, &state, Status::Idle);
                    }
                    continue;
                }

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
                    if let Some(eng) = engine.take() {
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
                        pending_audio = Some((audio, settings));
                    }
                } else {
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

                    let mut did_output = false;
                    if let Some(ref eng) = engine {
                        match eng.transcribe(&audio, &settings.language, None) {
                            Ok(text) if !text.is_empty() => {
                                if let Err(e) = output::send(&text, &settings.output_mode) {
                                    log::error!("output error: {}", e);
                                    let _ =
                                        app.emit("backend-error", format!("Output error: {}", e));
                                }
                                let _ = app.emit("transcription", &text);
                                did_output = true;
                            }
                            Ok(_) => {}
                            Err(e) => {
                                log::error!("transcription error: {}", e);
                                let _ = app
                                    .emit("backend-error", format!("Transcription error: {}", e));
                            }
                        }
                    }

                    if settings.model_loading == ModelLoading::PerUse {
                        engine = None;
                        loaded_model.clear();
                    }

                    if did_output {
                        let flash = match settings.output_mode {
                            OutputMode::Clipboard => "Copied",
                            OutputMode::Paste => "Typed",
                        };
                        let _ = app.emit("overlay-flash", flash);
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
                let flash = match settings.output_mode {
                    OutputMode::Clipboard => "Clipboard",
                    OutputMode::Paste => "Paste",
                };
                let _ = settings.save(&state.data_dir);
                *state.settings.lock() = settings;
                let _ = app.emit("settings-changed", ());
                let _ = app.emit("overlay-flash", flash);
            }
            AppEvent::TranscriptionDone {
                engine: returned_engine,
                result,
                output_mode,
            } => {
                transcription_in_flight = false;
                abort_flag.store(false, Ordering::Relaxed);

                if !cancelled {
                    match result {
                        Ok(ref text) if !text.is_empty() => {
                            if let Err(e) = output::send(text, &output_mode) {
                                log::error!("output error: {}", e);
                                let _ = app.emit("backend-error", format!("Output error: {}", e));
                            }
                            let _ = app.emit("transcription", text);
                            let flash = match output_mode {
                                OutputMode::Clipboard => "Copied",
                                OutputMode::Paste => "Typed",
                            };
                            let _ = app.emit("overlay-flash", flash);
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

                let per_use = state.settings.lock().model_loading == ModelLoading::PerUse;

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
                } else if per_use {
                    drop(returned_engine);
                    loaded_model.clear();
                } else {
                    engine = Some(returned_engine);
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
