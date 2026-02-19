use parking_lot::Mutex;
use rdev::{Event, EventType, Key};
use std::collections::HashSet;
use std::sync::mpsc;
use std::sync::Arc;

use super::HotkeyEvent;

pub fn start(
    hotkey: Arc<Mutex<Vec<Key>>>,
    output_hotkey: Arc<Mutex<Vec<Key>>>,
    tx: mpsc::Sender<HotkeyEvent>,
) {
    std::thread::spawn(move || {
        log::info!("hotkey listener thread started");
        let mut pressed_keys: HashSet<Key> = HashSet::new();
        let mut main_active = false;
        let mut output_active = false;

        if let Err(e) = rdev::listen(move |event: Event| {
            let main_keys = hotkey.lock().clone();
            let out_keys = output_hotkey.lock().clone();
            match event.event_type {
                EventType::KeyPress(key) => {
                    pressed_keys.insert(key);

                    if !output_active
                        && !out_keys.is_empty()
                        && out_keys.iter().all(|k| pressed_keys.contains(k))
                    {
                        output_active = true;
                        let _ = tx.send(HotkeyEvent::OutputToggle);
                    } else if !main_active
                        && !main_keys.is_empty()
                        && main_keys.iter().all(|k| pressed_keys.contains(k))
                    {
                        main_active = true;
                        let _ = tx.send(HotkeyEvent::Pressed);
                    }
                }
                EventType::KeyRelease(key) => {
                    if main_active && main_keys.contains(&key) {
                        main_active = false;
                        let _ = tx.send(HotkeyEvent::Released);
                    }
                    if output_active && out_keys.contains(&key) {
                        output_active = false;
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
