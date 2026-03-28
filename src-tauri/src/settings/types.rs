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
pub enum OverlayIcon {
    Dot,
    Pulse,
    Spinner,
    Check,
    X,
}

#[derive(Debug, Clone, Serialize, Type)]
pub struct OverlayState {
    pub icon: OverlayIcon,
    pub label: String,
    pub ttl_ms: Option<u32>,
}

impl Default for OverlayState {
    fn default() -> Self {
        Self {
            icon: OverlayIcon::Dot,
            label: "Idle".into(),
            ttl_ms: None,
        }
    }
}
