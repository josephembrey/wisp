investigate hotkey "invalid" sound on Windows — if it recurs, switch from rdev::listen to rdev::grab to suppress hotkey propagation (return None for matched keys). Needs rdev feature "unstable_grab", which pulls in evdev-sys on Linux (add libevdev to devenv.nix). Could also make it Windows-only via cfg.
start on login
the input devices need more descriptive names
better hotkey detection while the overlay is focused
setting for preemptive model loading
add debug build mode that spits out logs
understand rust-overlay and why we need it
