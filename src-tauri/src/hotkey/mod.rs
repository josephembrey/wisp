mod convert;

pub use convert::to_accelerator;

pub enum HotkeyEvent {
    Pressed,
    Released,
    OutputToggle,
}
