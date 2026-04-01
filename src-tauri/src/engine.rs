use crate::audio;
use crate::history;
use crate::hotkey;
use crate::output;
use crate::settings::{OutputMode, OverlayState, OverlayStatus, WispState};
use crate::whisper;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{Emitter, Manager};

// --- Events ---

pub(crate) enum AppEvent {
    Hotkey(hotkey::HotkeyEvent),
    TranscriptionDone {
        job_id: u64,
        result: Result<String, String>,
        output_mode: OutputMode,
    },
    WorkerLoading,
    WorkerError(String),
}

// --- State machine ---

enum EngineState {
    Idle,
    Recording {
        recorder: audio::AudioRecorder,
        overlay: OverlayScope,
    },
    Transcribing {
        job_id: u64,
        overlay: OverlayScope,
    },
}

// --- Main loop ---

pub(crate) fn run(
    app: tauri::AppHandle,
    tx: std::sync::mpsc::Sender<AppEvent>,
    rx: std::sync::mpsc::Receiver<AppEvent>,
    worker_tx: std::sync::mpsc::Sender<whisper::worker::WorkerMessage>,
) {
    let state = app.state::<WispState>();
    let mut engine_state = EngineState::Idle;
    let mut current_job: u64 = 0;
    let abort_flag = Arc::new(AtomicBool::new(false));

    // Eager preload
    {
        let settings = state.settings.lock().clone();
        if settings.model_loading == crate::settings::ModelLoading::Eager {
            let _ = worker_tx.send(whisper::worker::WorkerMessage::Reload {
                model: settings.model.clone(),
                gpu: settings.gpu,
            });
        }
    }

    for event in rx {
        match (&mut engine_state, event) {
            // --- Idle ---
            (EngineState::Idle, AppEvent::Hotkey(hotkey::HotkeyEvent::Pressed)) => {
                let settings = state.settings.lock().clone();
                match audio::AudioRecorder::start(&settings.input_device) {
                    Ok(recorder) => {
                        let overlay = OverlayScope::new(&app, OverlayStatus::Recording);
                        engine_state = EngineState::Recording { recorder, overlay };
                    }
                    Err(e) => {
                        log::error!("recording: failed to start: {}", e);
                        let _ = app.emit("backend-error", format!("Mic error: {}", e));
                    }
                }
            }

            // --- Recording ---
            (EngineState::Recording { .. }, AppEvent::Hotkey(hotkey::HotkeyEvent::Pressed)) => {
                // Duplicate press, ignore
            }

            (EngineState::Recording { .. }, AppEvent::Hotkey(hotkey::HotkeyEvent::Released)) => {
                // Take ownership of recorder and overlay by swapping to Idle temporarily
                let prev = std::mem::replace(&mut engine_state, EngineState::Idle);
                let EngineState::Recording { recorder, overlay } = prev else {
                    unreachable!()
                };

                let audio = recorder.stop();
                let duration_ms = (audio.len() as f64 / 16.0) as u64;
                log::info!(
                    "recording: stopped {} samples ({}ms)",
                    audio.len(),
                    duration_ms
                );

                let settings = state.settings.lock().clone();
                let min_samples = (settings.min_duration * 16_000.0) as usize;
                if audio.len() < min_samples {
                    log::info!(
                        "recording: too short ({} < {}), skipping",
                        audio.len(),
                        min_samples
                    );
                    overlay.finish(OverlayStatus::Cancelled, 1000);
                    // engine_state is already Idle from the swap
                    continue;
                }

                overlay.set(OverlayStatus::Processing);
                current_job += 1;
                abort_flag.store(false, Ordering::Relaxed);

                let _ = worker_tx.send(whisper::worker::WorkerMessage::Transcribe {
                    job_id: current_job,
                    audio,
                    language: settings.language.clone(),
                    model: settings.model.clone(),
                    gpu: settings.gpu,
                    model_loading: settings.model_loading,
                    abort_flag: abort_flag.clone(),
                    reply: whisper::worker::ReplyTo::Engine {
                        tx: tx.clone(),
                        output_mode: settings.output_mode,
                    },
                });

                engine_state = EngineState::Transcribing {
                    job_id: current_job,
                    overlay,
                };
            }

            // --- Transcribing ---
            (EngineState::Transcribing { .. }, AppEvent::Hotkey(hotkey::HotkeyEvent::Pressed)) => {
                let guard = state.settings.lock();
                if !guard.interrupt {
                    continue;
                }
                let input_device = guard.input_device.clone();
                drop(guard);

                // Abort the in-flight transcription
                abort_flag.store(true, Ordering::Relaxed);

                // Take ownership of overlay, start new recording
                let prev = std::mem::replace(&mut engine_state, EngineState::Idle);
                let EngineState::Transcribing { overlay, .. } = prev else {
                    unreachable!()
                };

                match audio::AudioRecorder::start(&input_device) {
                    Ok(recorder) => {
                        // Suppress old overlay's idle emission, create new Recording overlay
                        let new_overlay =
                            OverlayScope::replace_from(overlay, &app, OverlayStatus::Recording);
                        engine_state = EngineState::Recording {
                            recorder,
                            overlay: new_overlay,
                        };
                    }
                    Err(e) => {
                        log::error!("recording: failed to start: {}", e);
                        let _ = app.emit("backend-error", format!("Mic error: {}", e));
                        overlay.finish(OverlayStatus::Cancelled, 1000);
                        // engine_state is already Idle
                    }
                }
            }

            (
                EngineState::Transcribing { job_id, .. },
                AppEvent::TranscriptionDone {
                    job_id: result_job_id,
                    result,
                    output_mode,
                },
            ) => {
                if result_job_id != *job_id {
                    log::debug!(
                        "engine: stale result (got job {}, expected {}), discarding",
                        result_job_id,
                        job_id
                    );
                    continue;
                }

                let prev = std::mem::replace(&mut engine_state, EngineState::Idle);
                let EngineState::Transcribing { overlay, .. } = prev else {
                    unreachable!()
                };

                match result {
                    Ok(ref text) if !text.is_empty() => {
                        handle_transcription(&app, text, &output_mode, &state);
                        overlay.finish(OverlayStatus::from(&output_mode), 1000);
                    }
                    Ok(_) => {
                        // Empty transcription, just go idle
                        drop(overlay);
                    }
                    Err(ref e) => {
                        log::error!("transcription error: {}", e);
                        let _ = app.emit("backend-error", format!("Transcription error: {}", e));
                        overlay.finish(OverlayStatus::Cancelled, 1000);
                    }
                }
            }

            // Stale result arriving in non-Transcribing state — discard
            (_, AppEvent::TranscriptionDone { job_id, .. }) => {
                log::debug!(
                    "engine: TranscriptionDone in non-Transcribing state (job {}), discarding",
                    job_id
                );
            }

            // --- Any state ---
            (_, AppEvent::Hotkey(hotkey::HotkeyEvent::OutputToggle)) => {
                let (old_mode, new_settings) = {
                    let mut guard = state.settings.lock();
                    let old_mode = guard.output_mode;
                    guard.output_mode = match old_mode {
                        OutputMode::Clipboard => OutputMode::Paste,
                        OutputMode::Paste => OutputMode::Clipboard,
                    };
                    (old_mode, guard.clone())
                };
                log::info!(
                    "output mode toggled: {:?} -> {:?}",
                    old_mode,
                    new_settings.output_mode
                );
                new_settings.save(&state.data_dir).ok();
                let _ = app.emit("settings-changed", &new_settings);
                set_overlay(
                    &app,
                    OverlayState {
                        status: OverlayStatus::OutputMode,
                        ttl_ms: Some(1000),
                    },
                );
            }

            (_, AppEvent::Hotkey(hotkey::HotkeyEvent::Released)) => {
                // Released in non-Recording state, ignore
            }

            (_, AppEvent::WorkerLoading) => {
                set_overlay(
                    &app,
                    OverlayState {
                        status: OverlayStatus::Loading,
                        ttl_ms: None,
                    },
                );
            }

            (_, AppEvent::WorkerError(e)) => {
                let _ = app.emit("backend-error", &e);
                set_overlay(&app, OverlayState::default());
            }
        }
    }
}

// --- Helpers ---

fn handle_transcription(app: &tauri::AppHandle, text: &str, mode: &OutputMode, state: &WispState) {
    log::info!("transcription: {} chars", text.len());

    if let Err(e) = output::send(text, mode) {
        log::error!("output error: {}", e);
        let _ = app.emit("backend-error", format!("Output error: {}", e));
    }

    let _ = app.emit("transcription", text);

    let settings = state.settings.lock();
    if settings.history_enabled {
        history::append(&state.data_dir, text, "mic", settings.history_retention);
        let _ = app.emit("history-changed", ());
    }
}

pub(crate) fn set_overlay(app: &tauri::AppHandle, overlay: OverlayState) {
    let _ = app.emit("overlay-state", &overlay);
}

// --- Overlay RAII guard ---

struct OverlayScope {
    app: tauri::AppHandle,
    active: bool,
}

impl OverlayScope {
    fn new(app: &tauri::AppHandle, status: OverlayStatus) -> Self {
        set_overlay(
            app,
            OverlayState {
                status,
                ttl_ms: None,
            },
        );
        Self {
            app: app.clone(),
            active: true,
        }
    }

    /// Suppress the old scope's idle emission and create a new one.
    fn replace_from(mut old: OverlayScope, app: &tauri::AppHandle, status: OverlayStatus) -> Self {
        old.active = false;
        Self::new(app, status)
    }

    fn set(&self, status: OverlayStatus) {
        set_overlay(
            &self.app,
            OverlayState {
                status,
                ttl_ms: None,
            },
        );
    }

    fn finish(mut self, status: OverlayStatus, ttl_ms: u32) {
        self.active = false;
        set_overlay(
            &self.app,
            OverlayState {
                status,
                ttl_ms: Some(ttl_ms),
            },
        );
    }
}

impl Drop for OverlayScope {
    fn drop(&mut self) {
        if self.active {
            set_overlay(&self.app, OverlayState::default());
        }
    }
}
