/// Check if the foreground window is fullscreen (covers the entire monitor).

#[cfg(target_os = "windows")]
pub fn is_foreground_fullscreen() -> bool {
    use windows_sys::Win32::Foundation::RECT;
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowRect};

    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() {
            return false;
        }

        let mut win_rect: RECT = std::mem::zeroed();
        if GetWindowRect(hwnd, &mut win_rect) == 0 {
            return false;
        }

        let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        if monitor.is_null() {
            return false;
        }

        let mut info: MONITORINFO = std::mem::zeroed();
        info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
        if GetMonitorInfoW(monitor, &mut info) == 0 {
            return false;
        }

        win_rect.left <= info.rcMonitor.left
            && win_rect.top <= info.rcMonitor.top
            && win_rect.right >= info.rcMonitor.right
            && win_rect.bottom >= info.rcMonitor.bottom
    }
}

#[cfg(not(target_os = "windows"))]
pub fn is_foreground_fullscreen() -> bool {
    false
}
