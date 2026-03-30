use super::Settings;

/// Migrate settings from older formats. Returns true if any changes were made.
pub fn migrate(settings: &mut Settings) -> bool {
    let mut changed = false;

    // v0.1 → v0.2: key names changed from browser KeyboardEvent.code format
    // ("Alt+KeyQ", "ControlLeft+ShiftLeft+KeyA") to Tauri accelerator format
    // ("Alt+Q", "Control+Shift+A").
    if migrate_hotkey(&mut settings.hotkey) {
        changed = true;
    }
    if migrate_hotkey(&mut settings.output_hotkey) {
        changed = true;
    }

    changed
}

/// Convert a single hotkey combo from old format to Tauri accelerator format.
fn migrate_hotkey(combo: &mut String) -> bool {
    if combo.is_empty() {
        return false;
    }

    let mut changed = false;
    let parts: Vec<String> = combo
        .split('+')
        .map(|part| {
            let converted = convert_key(part.trim());
            if converted != part.trim() {
                changed = true;
            }
            converted
        })
        .collect();

    if changed {
        *combo = parts.join("+");
        log::info!("settings: migrated hotkey to '{}'", combo);
    }
    changed
}

fn convert_key(key: &str) -> String {
    match key {
        // Modifier Left/Right variants → unified names
        "AltLeft" | "AltRight" | "RightAlt" => "Alt".into(),
        "ControlLeft" | "ControlRight" => "Control".into(),
        "ShiftLeft" | "ShiftRight" => "Shift".into(),
        "MetaLeft" | "MetaRight" => "Super".into(),
        // Key prefix → bare letter
        k if k.starts_with("Key") && k.len() == 4 => k[3..].to_string(),
        // Num prefix → bare digit
        k if k.starts_with("Num") && k.len() == 4 => k[3..].to_string(),
        // Everything else passes through (already correct format)
        k => k.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrates_old_format() {
        let mut combo = "Alt+KeyQ".to_string();
        assert!(migrate_hotkey(&mut combo));
        assert_eq!(combo, "Alt+Q");
    }

    #[test]
    fn migrates_complex_combo() {
        let mut combo = "ControlLeft+ShiftLeft+KeyA".to_string();
        assert!(migrate_hotkey(&mut combo));
        assert_eq!(combo, "Control+Shift+A");
    }

    #[test]
    fn leaves_new_format_alone() {
        let mut combo = "Alt+Q".to_string();
        assert!(!migrate_hotkey(&mut combo));
        assert_eq!(combo, "Alt+Q");
    }

    #[test]
    fn handles_empty() {
        let mut combo = String::new();
        assert!(!migrate_hotkey(&mut combo));
    }

    #[test]
    fn migrates_number_keys() {
        let mut combo = "Alt+Num5".to_string();
        assert!(migrate_hotkey(&mut combo));
        assert_eq!(combo, "Alt+5");
    }

    #[test]
    fn preserves_function_keys() {
        let mut combo = "Alt+F12".to_string();
        assert!(!migrate_hotkey(&mut combo));
        assert_eq!(combo, "Alt+F12");
    }
}
