use super::HotkeyEvent;
use crate::settings::WispState;
use std::sync::mpsc::Sender;
use tauri::Manager;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;

/// Spawns a thread that polls physical key state at ~125 Hz to detect
/// press/release edges for the main PTT hotkey combo.
///
/// On Windows the `global-hotkey` crate (used by `tauri-plugin-global-shortcut`)
/// relies on `WM_HOTKEY` + a spin-loop that only checks the *main* key for
/// release, ignoring modifiers.  When a modifier is released first the plugin
/// can emit ghost `Pressed` events or miss the `Released` entirely.  This
/// polling thread replaces that mechanism with deterministic edge detection
/// over the full combo via `GetAsyncKeyState`.
pub fn start_ptt_polling(app: tauri::AppHandle, tx: Sender<HotkeyEvent>) {
    std::thread::spawn(move || {
        let mut was_down = false;
        loop {
            let combo = app.state::<WispState>().settings.lock().hotkey.clone();
            let is_down = combo_is_down(&combo);
            if is_down && !was_down {
                let _ = tx.send(HotkeyEvent::Pressed);
            } else if !is_down && was_down {
                let _ = tx.send(HotkeyEvent::Released);
            }
            was_down = is_down;
            std::thread::sleep(std::time::Duration::from_millis(8));
        }
    });
}

fn is_key_down(vk: i32) -> bool {
    unsafe { (GetAsyncKeyState(vk) as u16 & 0x8000) != 0 }
}

fn combo_is_down(combo: &str) -> bool {
    let mut saw_any = false;
    for part in combo.split('+').map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let Some(vk) = key_to_vk(part) else {
            return false;
        };
        saw_any = true;
        if !is_key_down(vk) {
            return false;
        }
    }
    saw_any
}

fn key_to_vk(key: &str) -> Option<i32> {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
    let vk = match key {
        "Alt" | "RightAlt" | "AltRight" => VK_MENU,
        "AltLeft" => VK_LMENU,
        "ControlLeft" => VK_LCONTROL,
        "ControlRight" => VK_RCONTROL,
        "ShiftLeft" => VK_LSHIFT,
        "ShiftRight" => VK_RSHIFT,
        "MetaLeft" => VK_LWIN,
        "MetaRight" => VK_RWIN,
        "Space" => VK_SPACE,
        "Enter" => VK_RETURN,
        "CapsLock" => VK_CAPITAL,
        k if k.starts_with("Key") && k.len() == 4 => {
            let c = k.as_bytes()[3] as char;
            if c.is_ascii_uppercase() {
                c as u16
            } else {
                return None;
            }
        }
        k if k.len() == 1 && k.as_bytes()[0].is_ascii_uppercase() => k.as_bytes()[0] as u16,
        k if k.starts_with("Num") && k.len() == 4 => {
            let c = k.as_bytes()[3] as char;
            if c.is_ascii_digit() {
                c as u16
            } else {
                return None;
            }
        }
        k if k.starts_with('F') => {
            let n = k[1..].parse::<u32>().ok()?;
            match n {
                1 => VK_F1,
                2 => VK_F2,
                3 => VK_F3,
                4 => VK_F4,
                5 => VK_F5,
                6 => VK_F6,
                7 => VK_F7,
                8 => VK_F8,
                9 => VK_F9,
                10 => VK_F10,
                11 => VK_F11,
                12 => VK_F12,
                13 => VK_F13,
                14 => VK_F14,
                15 => VK_F15,
                16 => VK_F16,
                17 => VK_F17,
                18 => VK_F18,
                19 => VK_F19,
                20 => VK_F20,
                21 => VK_F21,
                22 => VK_F22,
                23 => VK_F23,
                24 => VK_F24,
                _ => return None,
            }
        }
        _ => return None,
    };
    Some(vk as i32)
}
