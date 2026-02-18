investigate hotkey "invalid" sound on Windows — if it recurs, switch from rdev::listen to rdev::grab to suppress hotkey propagation (return None for matched keys). Needs rdev feature "unstable_grab", which pulls in evdev-sys on Linux (add libevdev to devenv.nix). Could also make it Windows-only via cfg.
start on login
the input devices need more descriptive names
better hotkey detection while the overlay is focused
fix "fullscreen borderless" detection

signing certificate for windows
disabling/enabling the always on overlay doesnt change immediately (has to change state to processing/recording to take effect and show/hide the idle state)
better hotkey detection while the overlay is focused
fix "fullscreen borderless" detection
signing certificate for windows
sizes dont work
copied/typed indicators dont work
