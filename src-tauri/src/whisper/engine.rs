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
        let ctx =
            WhisperContext::new_with_params(path.to_str().ok_or("invalid model path")?, params)
                .map_err(|e| format!("{:?}", e))?;
        Ok(Self { ctx })
    }

    pub fn transcribe(
        &self,
        audio: &[f32],
        language: &str,
        abort: Option<Arc<AtomicBool>>,
    ) -> Result<String, String> {
        let mut state = self.ctx.create_state().map_err(|e| format!("{:?}", e))?;

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
        if let Some(flag) = abort {
            let cb = move || flag.load(Ordering::Relaxed);
            params.set_abort_callback_safe(cb);
        }

        state.full(params, audio).map_err(|e| format!("{:?}", e))?;

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
        Ok(filtered.trim().to_string())
    }
}
