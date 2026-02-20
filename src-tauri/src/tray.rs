use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    webview::WebviewWindowBuilder,
    Manager, WebviewUrl,
};

pub fn setup(app: &tauri::App, first_run: bool) -> tauri::Result<()> {
    log::info!("tray: setting up system tray and overlay");
    let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&settings_item, &quit_item])?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "settings" => {
                log::info!("tray: settings menu clicked");
                if let Some(window) = app.get_webview_window("main") {
                    log::info!("tray: showing main window");
                    let _ = window.show();
                    let _ = window.set_focus();
                } else {
                    log::warn!("tray: main window not found");
                }
            }
            "quit" => {
                log::info!("tray: quit");
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                log::info!("tray: left-clicked, showing main window");
                if let Some(window) = tray.app_handle().get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                } else {
                    log::warn!("tray: main window not found on click");
                }
            }
        })
        .build(app)?;
    log::info!("tray: tray icon built");

    log::info!("overlay: creating overlay window");
    let overlay =
        WebviewWindowBuilder::new(app, "overlay", WebviewUrl::App("overlay".into()))
            .title("Wisp Status")
            .inner_size(1.0, 1.0)
            .decorations(false)
            .transparent(true)
            .shadow(false)
            .always_on_top(true)
            .focused(false)
            .skip_taskbar(true)
            .visible(false)
            .build()?;
    log::info!("overlay: window created");

    match overlay.set_ignore_cursor_events(true) {
        Ok(()) => log::info!("overlay: set_ignore_cursor_events(true) succeeded"),
        Err(e) => log::error!("overlay: set_ignore_cursor_events(true) FAILED: {}", e),
    }

    if first_run {
        log::info!("first run: showing main window");
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }

    Ok(())
}
