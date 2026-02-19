use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const FILE_NAME: &str = "history.json";

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct HistoryEntry {
    pub id: u64,
    pub timestamp: u64,
    pub text: String,
    pub source: String,
}

pub fn load(data_dir: &Path) -> Vec<HistoryEntry> {
    let path = data_dir.join(FILE_NAME);
    if !path.exists() {
        return Vec::new();
    }
    let content = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

pub fn append(data_dir: &Path, text: &str, source: &str, max_entries: usize) {
    let mut entries = load(data_dir);

    let next_id = entries.first().map(|e| e.id + 1).unwrap_or(1);
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    entries.insert(
        0,
        HistoryEntry {
            id: next_id,
            timestamp,
            text: text.to_string(),
            source: source.to_string(),
        },
    );

    if max_entries > 0 {
        entries.truncate(max_entries);
    }

    save(data_dir, &entries);
}

pub fn clear(data_dir: &Path) {
    let path = data_dir.join(FILE_NAME);
    let _ = fs::remove_file(path);
}

pub fn delete_entry(data_dir: &Path, id: u64) {
    let mut entries = load(data_dir);
    entries.retain(|e| e.id != id);
    save(data_dir, &entries);
}

fn save(data_dir: &Path, entries: &[HistoryEntry]) {
    let _ = fs::create_dir_all(data_dir);
    let path = data_dir.join(FILE_NAME);
    if let Ok(content) = serde_json::to_string_pretty(entries) {
        let _ = fs::write(path, content);
    }
}
