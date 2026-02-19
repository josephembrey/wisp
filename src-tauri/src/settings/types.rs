use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputMode {
    Clipboard,
    Paste,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ModelLoading {
    Eager,
    Lazy,
    PerUse,
}

impl Default for ModelLoading {
    fn default() -> Self {
        Self::Eager
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Idle,
    Loading,
    Recording,
    Processing,
}
