use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::Emitter;

const MODELS: &[(&str, u64)] = &[
    ("tiny", 75),
    ("base", 142),
    ("small", 466),
    ("medium", 1533),
    ("large", 2952),
];

fn model_url(name: &str) -> String {
    format!(
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-{}.bin",
        name
    )
}

pub fn model_path(models_dir: &Path, name: &str) -> PathBuf {
    models_dir.join(format!("ggml-{}.bin", name))
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct ModelInfo {
    pub name: String,
    pub size_mb: u64,
    pub downloaded: bool,
}

#[derive(Debug, Clone, Serialize, specta::Type)]
pub struct DownloadProgress {
    pub model: String,
    pub downloaded: u64,
    pub total: u64,
}

pub fn list_models(models_dir: &Path) -> Vec<ModelInfo> {
    MODELS
        .iter()
        .map(|(name, size_mb)| ModelInfo {
            name: name.to_string(),
            size_mb: *size_mb,
            downloaded: model_path(models_dir, name).exists(),
        })
        .collect()
}

pub async fn download_model(
    app: tauri::AppHandle,
    models_dir: &Path,
    name: &str,
) -> Result<(), String> {
    fs::create_dir_all(models_dir).map_err(|e| e.to_string())?;

    let url = model_url(name);
    let path = model_path(models_dir, name);

    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
    let total = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;

    let mut file = fs::File::create(&path).map_err(|e| e.to_string())?;
    let mut stream = response.bytes_stream();

    use futures_util::StreamExt;
    use std::io::Write;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        file.write_all(&chunk).map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;

        let _ = app.emit(
            "download-progress",
            DownloadProgress {
                model: name.to_string(),
                downloaded,
                total,
            },
        );
    }

    Ok(())
}

pub fn delete_model(models_dir: &Path, name: &str) -> Result<(), String> {
    let path = model_path(models_dir, name);
    if path.exists() {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}
