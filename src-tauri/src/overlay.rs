/// Overlay helpers: work area detection, fullscreen check.

/// Work area rect in physical pixels (excludes taskbar/docks).
pub struct WorkArea {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

/// Get the work area for a monitor at the given physical position.
/// On Windows, uses `MONITORINFO.rcWork` to exclude the taskbar.
/// On other platforms, falls back to the full monitor area.
#[cfg(target_os = "windows")]
pub fn get_work_area(mon_x: i32, mon_y: i32, mon_w: i32, mon_h: i32) -> WorkArea {
    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromPoint, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    };

    unsafe {
        // Find the HMONITOR for this monitor's top-left corner
        let pt = POINT {
            x: mon_x + 1,
            y: mon_y + 1,
        };
        let hmon = MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST);
        if !hmon.is_null() {
            let mut info: MONITORINFO = std::mem::zeroed();
            info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
            if GetMonitorInfoW(hmon, &mut info) != 0 {
                return WorkArea {
                    x: info.rcWork.left,
                    y: info.rcWork.top,
                    width: info.rcWork.right - info.rcWork.left,
                    height: info.rcWork.bottom - info.rcWork.top,
                };
            }
        }
    }

    // Fallback: full monitor area
    WorkArea {
        x: mon_x,
        y: mon_y,
        width: mon_w,
        height: mon_h,
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_work_area(mon_x: i32, mon_y: i32, mon_w: i32, mon_h: i32) -> WorkArea {
    WorkArea {
        x: mon_x,
        y: mon_y,
        width: mon_w,
        height: mon_h,
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[cfg(not(target_os = "windows"))]
pub fn is_foreground_fullscreen() -> bool {
    false
}
