use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    webview::WebviewWindowBuilder,
    WebviewUrl,
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
                log::debug!("tray: settings menu clicked");
                crate::show_main_window(app);
            }
            "quit" => {
                log::debug!("tray: quit");
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray: &tauri::tray::TrayIcon, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                log::debug!("tray: left-clicked, showing main window");
                crate::show_main_window(tray.app_handle());
            }
        })
        .build(app)?;
    log::info!("tray: tray icon built");

    log::info!("overlay: creating overlay window");
    let overlay = WebviewWindowBuilder::new(app, "overlay", WebviewUrl::App("overlay".into()))
        .title("Wisp Status")
        .decorations(false)
        .transparent(true)
        .background_color(tauri::window::Color(0, 0, 0, 0))
        .shadow(false)
        .always_on_top(true)
        .focused(false)
        .skip_taskbar(true)
        .visible(false)
        .resizable(false)
        .build()?;
    log::info!("overlay: window created (hidden, waiting for frontend ready)");

    match overlay.set_ignore_cursor_events(true) {
        Ok(()) => log::info!("overlay: set_ignore_cursor_events(true) succeeded"),
        Err(e) => log::error!("overlay: set_ignore_cursor_events(true) FAILED: {}", e),
    }

    if first_run {
        log::info!("first run: showing main window");
        crate::show_main_window(app);
    }

    Ok(())
}
