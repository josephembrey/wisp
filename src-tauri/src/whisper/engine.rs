use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct WhisperEngine {
    ctx: WhisperContext,
}

impl WhisperEngine {
    pub fn new(path: &Path, use_gpu: bool) -> Result<Self, String> {
        let mut params = WhisperContextParameters::new();
        params.use_gpu(use_gpu);
        log::info!("whisper context: use_gpu={}", use_gpu);
        // Read model into memory first to avoid file descriptor invalidation
        // during Vulkan backend initialization inside whisper_init.
        let buffer = std::fs::read(path).map_err(|e| format!("read model file: {}", e))?;
        log::info!(
            "whisper model: read {} bytes from {}",
            buffer.len(),
            path.display()
        );
        let ctx = WhisperContext::new_from_buffer_with_params(&buffer, params)
            .map_err(|e| format!("{:?}", e))?;

        // Force Vulkan shader compilation now so the first real transcription
        // doesn't fail when it runs on a different (worker) thread.
        let engine = Self { ctx };
        engine.warmup();
        Ok(engine)
    }

    fn warmup(&self) {
        log::debug!("whisper: running warmup encode");
        let silence = vec![0.0f32; 16000]; // 1s of silence
        let mut state = match self.ctx.create_state() {
            Ok(s) => s,
            Err(e) => {
                log::warn!("whisper warmup: create_state failed: {:?}", e);
                return;
            }
        };
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some("en"));
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_special(false);
        params.set_print_timestamps(false);
        params.set_no_timestamps(true);
        match state.full(params, &silence) {
            Ok(_) => log::debug!("whisper: warmup complete"),
            Err(e) => log::warn!("whisper warmup: encode failed: {:?}", e),
        }
    }

    pub fn transcribe(
        &self,
        audio: &[f32],
        language: &str,
        abort: Option<Arc<AtomicBool>>,
    ) -> Result<String, String> {
        let make_params = |abort_ref: &Option<Arc<AtomicBool>>| {
            let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
            let lang = if language == "auto" {
                None
            } else {
                Some(language)
            };
            params.set_language(lang);
            params.set_print_progress(false);
            params.set_print_realtime(false);
            params.set_print_special(false);
            params.set_print_timestamps(false);
            params.set_no_timestamps(true);
            if let Some(flag) = abort_ref.clone() {
                let cb = move || flag.load(Ordering::Relaxed);
                params.set_abort_callback_safe(cb);
            }
            params
        };

        // Try up to 2 times with a fresh state each attempt — Vulkan encode
        // can fail transiently on some GPU/driver combinations.
        let mut last_err = String::new();
        for attempt in 0..2 {
            let mut state = self.ctx.create_state().map_err(|e| format!("{:?}", e))?;
            let params = make_params(&abort);
            match state.full(params, audio) {
                Ok(_) => {
                    if attempt > 0 {
                        log::info!("whisper: succeeded on retry");
                    }
                    let n = state.full_n_segments();
                    let mut text = String::new();
                    for i in 0..n {
                        if let Some(seg) = state.get_segment(i) {
                            text.push_str(&seg.to_string());
                        }
                    }
                    let filtered = regex_lite::Regex::new(r"\[.*?\]")
                        .unwrap()
                        .replace_all(text.trim(), "");
                    return Ok(filtered.trim().to_string());
                }
                Err(e) => {
                    last_err = format!("{:?}", e);
                    log::warn!(
                        "whisper: encode attempt {} failed: {}",
                        attempt + 1,
                        last_err
                    );
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
        Err(last_err)
    }
}
