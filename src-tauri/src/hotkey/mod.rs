pub enum HotkeyEvent {
    Pressed,
    Released,
    OutputToggle,
}

/// Register global shortcuts for the main hotkey and output toggle.
/// Combos are in Tauri accelerator format (e.g. "Alt+Q", "Control+Shift+A").
/// On Windows, the main PTT hotkey is handled by polling (see windows.rs),
/// so only the output toggle is registered with the plugin.
pub fn register(app: &tauri::AppHandle, main_combo: &str, output_combo: &str) {
    use tauri_plugin_global_shortcut::GlobalShortcutExt;

    let gs = app.global_shortcut();
    let _ = gs.unregister_all();

    #[cfg(target_os = "windows")]
    let combos = [("output", output_combo)];
    #[cfg(not(target_os = "windows"))]
    let combos = [("main", main_combo), ("output", output_combo)];

    let _ = main_combo; // used on non-Windows only

    for (label, combo) in combos {
        if combo.is_empty() {
            continue;
        }
        match combo.parse::<tauri_plugin_global_shortcut::Shortcut>() {
            Ok(shortcut) => {
                if let Err(e) = gs.register(shortcut) {
                    log::warn!("failed to register {} hotkey '{}': {}", label, combo, e);
                } else {
                    log::info!("registered {} hotkey: {}", label, combo);
                }
            }
            Err(e) => log::warn!("invalid {} hotkey '{}': {}", label, combo, e),
        }
    }
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::start_ptt_polling;
