use parking_lot::Mutex;
use std::path::PathBuf;

use super::Settings;

pub struct WispState {
    pub settings: Mutex<Settings>,
    pub data_dir: PathBuf,
    pub models_dir: PathBuf,
    pub engine_tx: std::sync::mpsc::Sender<crate::engine::AppEvent>,
    pub worker_tx: std::sync::mpsc::Sender<crate::whisper::worker::WorkerMessage>,
    pub first_run: bool,
}
