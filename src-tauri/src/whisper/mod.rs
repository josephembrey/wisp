mod engine;
mod models;
pub mod worker;

pub use engine::WhisperEngine;
pub use models::{
    delete_model, download_model, list_models, model_path, DownloadProgress, ModelInfo,
};

pub fn load_model(
    models_dir: &std::path::Path,
    name: &str,
    use_gpu: bool,
) -> Result<WhisperEngine, String> {
    let path = model_path(models_dir, name);
    if !path.exists() {
        return Err(format!("Model '{}' not downloaded", name));
    }
    WhisperEngine::new(&path, use_gpu)
        .map_err(|e| format!("Failed to load model '{}': {}", name, e))
}
