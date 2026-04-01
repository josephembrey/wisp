use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Type)]
#[serde(rename_all = "lowercase")]
pub enum OutputMode {
    Clipboard,
    Paste,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Type)]
#[serde(rename_all = "snake_case")]
pub enum ModelLoading {
    #[default]
    Eager,
    Lazy,
    PerUse,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Type)]
#[serde(rename_all = "kebab-case")]
pub enum OverlayPosition {
    TopLeft,
    TopCenter,
    #[default]
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Type)]
#[serde(rename_all = "lowercase")]
pub enum OverlaySize {
    Small,
    #[default]
    Medium,
    Large,
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

impl From<&OutputMode> for OverlayStatus {
    fn from(mode: &OutputMode) -> Self {
        match mode {
            OutputMode::Clipboard => OverlayStatus::Copied,
            OutputMode::Paste => OverlayStatus::Typed,
        }
    }
}

impl Default for OverlayState {
    fn default() -> Self {
        Self {
            status: OverlayStatus::Idle,
            ttl_ms: None,
        }
    }
}
