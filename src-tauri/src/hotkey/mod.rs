mod convert;
#[cfg(target_os = "windows")]
mod windows;

pub use convert::to_accelerator;
#[cfg(target_os = "windows")]
pub use windows::start_ptt_polling;

pub enum HotkeyEvent {
    Pressed,
    Released,
    OutputToggle,
}
