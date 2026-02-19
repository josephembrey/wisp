/// Convert our internal hotkey format ("Alt+KeyQ") to the accelerator
/// format expected by tauri-plugin-global-shortcut ("Alt+Q").
pub fn to_accelerator(combo: &str) -> Option<String> {
    if combo.is_empty() {
        return None;
    }

    let parts: Vec<&str> = combo.split('+').map(|s| s.trim()).collect();
    let converted: Vec<String> = parts.iter().filter_map(|p| convert_key(p)).collect();

    if converted.is_empty() {
        return None;
    }

    Some(converted.join("+"))
}

fn convert_key(key: &str) -> Option<String> {
    let mapped = match key {
        "Alt" | "RightAlt" | "AltRight" => "Alt",
        "ControlLeft" | "ControlRight" => "Control",
        "ShiftLeft" | "ShiftRight" => "Shift",
        "MetaLeft" | "MetaRight" => "Super",
        "Space" => "Space",
        "CapsLock" => "CapsLock",
        k if k.starts_with("Key") => return Some(k[3..].to_string()),
        k if k.starts_with("Num") => return Some(k[3..].to_string()),
        k if k.starts_with('F') && k[1..].parse::<u32>().is_ok() => k,
        _ => return None,
    };
    Some(mapped.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_combo() {
        assert_eq!(to_accelerator("Alt+KeyQ"), Some("Alt+Q".to_string()));
    }

    #[test]
    fn modifier_only() {
        assert_eq!(to_accelerator("Alt"), Some("Alt".to_string()));
    }

    #[test]
    fn complex_combo() {
        assert_eq!(
            to_accelerator("ControlLeft+ShiftLeft+KeyA"),
            Some("Control+Shift+A".to_string())
        );
    }

    #[test]
    fn empty() {
        assert_eq!(to_accelerator(""), None);
    }

    #[test]
    fn function_key() {
        assert_eq!(to_accelerator("F12"), Some("F12".to_string()));
    }

    #[test]
    fn number_key() {
        assert_eq!(to_accelerator("Alt+Num5"), Some("Alt+5".to_string()));
    }
}
