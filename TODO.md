why is the binary so large?
signing certificate for windows
investigate hotkey "invalid" sound on Windows — if it recurs, switch from rdev::listen to rdev::grab to suppress hotkey propagation (return None for matched keys). Needs rdev feature "unstable_grab", which pulls in evdev-sys on Linux (add libevdev to devenv.nix). Could also make it Windows-only via cfg.
add overlay for visual feedback on recording
