use parking_lot::Mutex;
use rdev::{Event, EventType, Key};
use std::collections::HashSet;
use std::sync::mpsc;
use std::sync::Arc;

pub enum HotkeyEvent {
    Pressed,
    Released,
}

pub fn parse_key(name: &str) -> Option<Key> {
    match name {
        "Alt" => Some(Key::Alt),
        "RightAlt" => Some(Key::AltGr),
        "ControlLeft" => Some(Key::ControlLeft),
        "ControlRight" => Some(Key::ControlRight),
        "ShiftLeft" => Some(Key::ShiftLeft),
        "ShiftRight" => Some(Key::ShiftRight),
        "MetaLeft" => Some(Key::MetaLeft),
        "MetaRight" => Some(Key::MetaRight),
        "Space" => Some(Key::Space),
        "CapsLock" => Some(Key::CapsLock),
        "F1" => Some(Key::F1),
        "F2" => Some(Key::F2),
        "F3" => Some(Key::F3),
        "F4" => Some(Key::F4),
        "F5" => Some(Key::F5),
        "F6" => Some(Key::F6),
        "F7" => Some(Key::F7),
        "F8" => Some(Key::F8),
        "F9" => Some(Key::F9),
        "F10" => Some(Key::F10),
        "F11" => Some(Key::F11),
        "F12" => Some(Key::F12),
        "KeyA" => Some(Key::KeyA),
        "KeyB" => Some(Key::KeyB),
        "KeyC" => Some(Key::KeyC),
        "KeyD" => Some(Key::KeyD),
        "KeyE" => Some(Key::KeyE),
        "KeyF" => Some(Key::KeyF),
        "KeyG" => Some(Key::KeyG),
        "KeyH" => Some(Key::KeyH),
        "KeyI" => Some(Key::KeyI),
        "KeyJ" => Some(Key::KeyJ),
        "KeyK" => Some(Key::KeyK),
        "KeyL" => Some(Key::KeyL),
        "KeyM" => Some(Key::KeyM),
        "KeyN" => Some(Key::KeyN),
        "KeyO" => Some(Key::KeyO),
        "KeyP" => Some(Key::KeyP),
        "KeyQ" => Some(Key::KeyQ),
        "KeyR" => Some(Key::KeyR),
        "KeyS" => Some(Key::KeyS),
        "KeyT" => Some(Key::KeyT),
        "KeyU" => Some(Key::KeyU),
        "KeyV" => Some(Key::KeyV),
        "KeyW" => Some(Key::KeyW),
        "KeyX" => Some(Key::KeyX),
        "KeyY" => Some(Key::KeyY),
        "KeyZ" => Some(Key::KeyZ),
        "Num0" => Some(Key::Num0),
        "Num1" => Some(Key::Num1),
        "Num2" => Some(Key::Num2),
        "Num3" => Some(Key::Num3),
        "Num4" => Some(Key::Num4),
        "Num5" => Some(Key::Num5),
        "Num6" => Some(Key::Num6),
        "Num7" => Some(Key::Num7),
        "Num8" => Some(Key::Num8),
        "Num9" => Some(Key::Num9),
        _ => None,
    }
}

pub fn parse_combo(combo: &str) -> Option<Vec<Key>> {
    let keys: Vec<Key> = combo
        .split('+')
        .map(|s| s.trim())
        .filter_map(parse_key)
        .collect();
    if keys.is_empty() {
        None
    } else {
        Some(keys)
    }
}

pub fn start(hotkey: Arc<Mutex<Vec<Key>>>, tx: mpsc::Sender<HotkeyEvent>) {
    std::thread::spawn(move || {
        log::info!("hotkey listener thread started");
        let mut pressed_keys: HashSet<Key> = HashSet::new();
        let mut combo_active = false;

        if let Err(e) = rdev::listen(move |event: Event| {
            let target_keys = hotkey.lock().clone();
            match event.event_type {
                EventType::KeyPress(key) => {
                    pressed_keys.insert(key);
                    if !combo_active && target_keys.iter().all(|k| pressed_keys.contains(k)) {
                        combo_active = true;
                        let _ = tx.send(HotkeyEvent::Pressed);
                    }
                }
                EventType::KeyRelease(key) => {
                    if combo_active && target_keys.contains(&key) {
                        combo_active = false;
                        let _ = tx.send(HotkeyEvent::Released);
                    }
                    pressed_keys.remove(&key);
                }
                _ => {}
            }
        }) {
            log::error!("failed to listen for hotkey events: {:?}", e);
        }
    });
}
