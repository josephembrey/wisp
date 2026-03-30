use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Type)]
#[serde(rename_all = "lowercase")]
pub enum OutputMode {
    Clipboard,
    Paste,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Type)]
#[serde(rename_all = "snake_case")]
pub enum ModelLoading {
    #[default]
    Eager,
    Lazy,
    PerUse,
}

#[derive(Debug, Clone, Serialize, PartialEq, Type)]
#[serde(rename_all = "snake_case")]
pub enum OverlayStatus {
    Idle,
    Recording,
    Processing,
    Loading,
    Saved,
    Copied,
    Typed,
    Deleted,
    Cancelled,
    OutputMode,
}

#[derive(Debug, Clone, Serialize, Type)]
pub struct OverlayState {
    pub status: OverlayStatus,
    pub ttl_ms: Option<u32>,
}

impl Default for OverlayState {
    fn default() -> Self {
        Self {
            status: OverlayStatus::Idle,
            ttl_ms: None,
        }
    }
}
