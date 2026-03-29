pub enum HotkeyEvent {
    Pressed,
    Released,
    OutputToggle,
}

/// Register global shortcuts for the main hotkey and output toggle.
/// Combos are in Tauri accelerator format (e.g. "Alt+Q", "Control+Shift+A").
pub fn register(app: &tauri::AppHandle, main_combo: &str, output_combo: &str) {
    use tauri_plugin_global_shortcut::GlobalShortcutExt;

    let gs = app.global_shortcut();
    let _ = gs.unregister_all();

    for (label, combo) in [("main", main_combo), ("output", output_combo)] {
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

// DORMANT — Windows-specific PTT polling workaround.
// See windows.rs for details and re-enablement instructions.
// Uncomment the line below + add "Win32_UI_Input" feature to windows-sys in Cargo.toml.
// mod windows;
