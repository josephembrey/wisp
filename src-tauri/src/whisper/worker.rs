use super::WhisperEngine;
use crate::settings::ModelLoading;

use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Arc;

pub enum WorkerMessage {
    Transcribe {
        job_id: u64,
        audio: Vec<f32>,
        language: String,
        model: String,
        gpu: bool,
        model_loading: ModelLoading,
        abort_flag: Arc<std::sync::atomic::AtomicBool>,
        reply: ReplyTo,
    },
    Reload {
        model: String,
        gpu: bool,
    },
}

// WhisperEngine contains a WhisperContext which is Send but not marked as such by whisper-rs.
unsafe impl Send for WorkerMessage {}

pub enum ReplyTo {
    Engine {
        tx: mpsc::Sender<crate::engine::AppEvent>,
        output_mode: crate::settings::OutputMode,
    },
    Caller(mpsc::Sender<Result<String, String>>),
}

pub fn run(
    rx: mpsc::Receiver<WorkerMessage>,
    engine_tx: mpsc::Sender<crate::engine::AppEvent>,
    models_dir: PathBuf,
) {
    let mut engine: Option<WhisperEngine> = None;
    let mut loaded_model = String::new();
    let mut loaded_gpu = false;

    for msg in rx {
        match msg {
            WorkerMessage::Transcribe {
                job_id,
                audio,
                language,
                model,
                gpu,
                model_loading,
                abort_flag,
                reply,
            } => {
                // Load or reload model if needed
                if engine.is_none() || loaded_model != model || loaded_gpu != gpu {
                    log::info!("worker: loading model '{}' (gpu={})", model, gpu);
                    let _ = engine_tx.send(crate::engine::AppEvent::WorkerLoading);
                    match super::load_model(&models_dir, &model, gpu) {
                        Ok(e) => {
                            engine = Some(e);
                            loaded_model = model;
                            loaded_gpu = gpu;
                        }
                        Err(e) => {
                            log::error!("worker: model load failed: {}", e);
                            send_result(&reply, job_id, Err(e));
                            continue;
                        }
                    }
                }

                // Transcribe
                let result =
                    engine
                        .as_ref()
                        .unwrap()
                        .transcribe(&audio, &language, Some(abort_flag));

                send_result(&reply, job_id, result);

                // PerUse: drop engine after each transcription
                if model_loading == ModelLoading::PerUse {
                    engine = None;
                    loaded_model.clear();
                }
            }
            WorkerMessage::Reload { model, gpu } => {
                log::info!("worker: preloading model '{}' (gpu={})", model, gpu);
                let _ = engine_tx.send(crate::engine::AppEvent::WorkerLoading);
                match super::load_model(&models_dir, &model, gpu) {
                    Ok(e) => {
                        engine = Some(e);
                        loaded_model = model;
                        loaded_gpu = gpu;
                        log::info!("worker: model preloaded");
                    }
                    Err(e) => {
                        log::warn!("worker: preload failed: {}", e);
                        let _ = engine_tx.send(crate::engine::AppEvent::WorkerError(e));
                    }
                }
            }
        }
    }

    log::info!("worker: channel closed, exiting");
}

fn send_result(reply: &ReplyTo, job_id: u64, result: Result<String, String>) {
    match reply {
        ReplyTo::Engine { tx, output_mode } => {
            let _ = tx.send(crate::engine::AppEvent::TranscriptionDone {
                job_id,
                result,
                output_mode: *output_mode,
            });
        }
        ReplyTo::Caller(tx) => {
            let _ = tx.send(result);
        }
    }
}
