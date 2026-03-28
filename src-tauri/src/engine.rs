use crate::audio;
use crate::history;
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

pub(crate) fn run(
    app: tauri::AppHandle,
    tx: std::sync::mpsc::Sender<AppEvent>,
    rx: std::sync::mpsc::Receiver<AppEvent>,
) {
    let state = app.state::<WispState>();
    let mut engine: Option<whisper::WhisperEngine> = None;
    let mut loaded_model = String::new();
    let mut loaded_gpu = false;
    let mut recorder: Option<audio::AudioRecorder> = None;
    let mut hotkey_latched = false;

    let abort_flag = Arc::new(AtomicBool::new(false));
    let mut cancelled = false;
    let mut transcription_in_flight = false;
    let mut pending_audio: Option<(Vec<f32>, Settings)> = None;

    {
        let settings = state.settings.lock().clone();
        if settings.model_loading == ModelLoading::Eager {
            set_status(&app, &state, Status::Loading);
            match load_model(&state.models_dir, &settings.model, settings.gpu) {
                Ok(e) => {
                    log::info!("eagerly loaded model: {}", settings.model);
                    engine = Some(e);
                    loaded_model = settings.model.clone();
                    loaded_gpu = settings.gpu;
                }
                Err(e) => log::warn!("eager load skipped: {}", e),
            }
            set_status(&app, &state, Status::Idle);
        }
    }

    for event in rx {
        match event {
            AppEvent::Hotkey(hotkey::HotkeyEvent::Pressed) => {
                if hotkey_latched {
                    log::debug!("ignored duplicate Pressed while latched");
                    continue;
                }
                if recorder.is_some() {
                    continue;
                }

                hotkey_latched = true;
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
                        hotkey_latched = false;
                    }
                }
            }
            AppEvent::Hotkey(hotkey::HotkeyEvent::Released) => {
                hotkey_latched = false;
                let Some(rec) = recorder.take() else {
                    continue;
                };

                let audio = rec.stop();
                let duration_ms = (audio.len() as f64 / 16.0) as u64;
                log::info!(
                    "recording stopped: {} samples ({}ms)",
                    audio.len(),
                    duration_ms
                );
                let settings = state.settings.lock().clone();
                let min_samples = (settings.min_duration * 16_000.0) as usize;
                if audio.len() < min_samples {
                    log::info!(
                        "recording too short ({} samples, min {}), skipping",
                        audio.len(),
                        min_samples
                    );
                    let _ = app.emit("overlay-flash", "Cancelled");
                    if transcription_in_flight {
                        set_status(&app, &state, Status::Processing);
                    } else {
                        set_status(&app, &state, Status::Idle);
                    }
                    continue;
                }

                set_status(&app, &state, Status::Processing);

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
                    let needs_reload = loaded_model != settings.model
                        || loaded_gpu != settings.gpu
                        || engine.is_none();
                    if needs_reload {
                        set_status(&app, &state, Status::Loading);
                        match load_model(&state.models_dir, &settings.model, settings.gpu) {
                            Ok(e) => {
                                engine = Some(e);
                                loaded_model = settings.model.clone();
                                loaded_gpu = settings.gpu;
                            }
                            Err(e) => {
                                log::error!("{}", e);
                                let _ = app.emit("backend-error", &e);
                                set_status(&app, &state, Status::Idle);
                                continue;
                            }
                        }
                        set_status(&app, &state, Status::Processing);
                    }

                    if let Some(ref eng) = engine {
                        match eng.transcribe(&audio, &settings.language, None) {
                            Ok(text) if !text.is_empty() => {
                                emit_output(&app, &text, &settings.output_mode, &state);
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

                    set_status(&app, &state, Status::Idle);
                }
            }
            AppEvent::Hotkey(hotkey::HotkeyEvent::OutputToggle) => {
                let mut settings = state.settings.lock().clone();
                let old_mode = settings.output_mode.clone();
                settings.output_mode = match settings.output_mode {
                    OutputMode::Clipboard => OutputMode::Paste,
                    OutputMode::Paste => OutputMode::Clipboard,
                };
                log::info!(
                    "output mode toggled: {:?} -> {:?}",
                    old_mode,
                    settings.output_mode
                );
                let flash = match settings.output_mode {
                    OutputMode::Clipboard => "Clipboard",
                    OutputMode::Paste => "Paste",
                };
                let _ = settings.save(&state.data_dir);
                *state.settings.lock() = settings.clone();
                let _ = app.emit("settings-changed", &settings);
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
                            emit_output(&app, text, &output_mode, &state);
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
                set_status(&app, &state, Status::Loading);
                match load_model(&state.models_dir, &settings.model, settings.gpu) {
                    Ok(e) => {
                        log::info!("reloaded model: {}", settings.model);
                        engine = Some(e);
                        loaded_model = settings.model.clone();
                        loaded_gpu = settings.gpu;
                    }
                    Err(e) => {
                        log::warn!("{}", e);
                        let _ = app.emit("backend-error", &e);
                    }
                }
                set_status(&app, &state, Status::Idle);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
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
        match load_model(models_dir, &settings.model, settings.gpu) {
            Ok(e) => {
                eng = e;
                *loaded_model = settings.model.clone();
                *loaded_gpu = settings.gpu;
            }
            Err(e) => {
                let _ = tx.send(AppEvent::TranscriptionDone {
                    engine: eng,
                    result: Err(e),
                    output_mode: settings.output_mode.clone(),
                });
                return;
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

fn load_model(
    models_dir: &std::path::Path,
    name: &str,
    use_gpu: bool,
) -> Result<whisper::WhisperEngine, String> {
    let path = whisper::model_path(models_dir, name);
    if !path.exists() {
        return Err(format!("Model '{}' not downloaded", name));
    }
    log::info!("loading model '{}' (gpu={})", name, use_gpu);
    whisper::WhisperEngine::new(&path, use_gpu)
        .map_err(|e| format!("Failed to load model '{}': {}", name, e))
}

fn emit_output(app: &tauri::AppHandle, text: &str, mode: &OutputMode, state: &WispState) {
    log::info!("transcription result: {} chars", text.len());
    if let Err(e) = output::send(text, mode) {
        log::error!("output error: {}", e);
        let _ = app.emit("backend-error", format!("Output error: {}", e));
    }
    let _ = app.emit("transcription", text);
    let flash = match mode {
        OutputMode::Clipboard => "Copied",
        OutputMode::Paste => "Typed",
    };
    let _ = app.emit("overlay-flash", flash);

    let settings = state.settings.lock();
    if settings.history_enabled {
        history::append(&state.data_dir, text, "mic", settings.history_retention);
        let _ = app.emit("history-changed", ());
    }
}

fn set_status(app: &tauri::AppHandle, state: &WispState, status: Status) {
    log::debug!("set_status -> {:?}", status);
    *state.status.lock() = status.clone();
    let _ = app.emit("status-changed", &status);
}
