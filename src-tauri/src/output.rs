use crate::state::OutputMode;
use enigo::{Enigo, Keyboard, Settings};

pub fn send(text: &str, mode: &OutputMode) -> Result<(), String> {
    match mode {
        OutputMode::Clipboard => {
            let mut clipboard =
                arboard::Clipboard::new().map_err(|e| format!("clipboard: {}", e))?;
            clipboard
                .set_text(text)
                .map_err(|e| format!("clipboard: {}", e))
        }
        OutputMode::Paste => {
            let mut enigo =
                Enigo::new(&Settings::default()).map_err(|e| format!("enigo: {:?}", e))?;
            enigo.text(text).map_err(|e| format!("enigo: {:?}", e))
        }
    }
}
