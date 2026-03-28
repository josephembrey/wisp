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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Type)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Idle,
    Loading,
    Recording,
    Processing,
}
