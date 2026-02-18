use parking_lot::Mutex;
use rdev::{Event, EventType, Key};
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
        _ => None,
    }
}

pub fn start(hotkey: Arc<Mutex<Key>>, tx: mpsc::Sender<HotkeyEvent>) {
    std::thread::spawn(move || {
        let mut is_pressed = false;

        rdev::listen(move |event: Event| {
            let target = *hotkey.lock();
            match event.event_type {
                EventType::KeyPress(key) if key == target => {
                    if !is_pressed {
                        is_pressed = true;
                        let _ = tx.send(HotkeyEvent::Pressed);
                    }
                }
                EventType::KeyRelease(key) if key == target => {
                    if is_pressed {
                        is_pressed = false;
                        let _ = tx.send(HotkeyEvent::Released);
                    }
                }
                _ => {}
            }
        })
        .expect("failed to listen for hotkey events");
    });
}
