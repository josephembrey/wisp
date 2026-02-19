use parking_lot::Mutex;
use std::path::PathBuf;

use super::{Settings, Status};

pub struct WispState {
    pub settings: Mutex<Settings>,
    pub status: Mutex<Status>,
    pub data_dir: PathBuf,
    pub models_dir: PathBuf,
    pub hotkey_tx: std::sync::mpsc::Sender<crate::hotkey::HotkeyEvent>,
    pub first_run: bool,
}
