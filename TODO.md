why is the binary so large?
signing certificate for windows
investigate hotkey "invalid" sound on Windows — if it recurs, switch from rdev::listen to rdev::grab to suppress hotkey propagation (return None for matched keys). Needs rdev feature "unstable_grab", which pulls in evdev-sys on Linux (add libevdev to devenv.nix). Could also make it Windows-only via cfg.
add transparent overlay for visual status feedback (recording/processing). Use a second Tauri window with transparent, decorations:false, shadow:false, always_on_top, focusable:false, skip_taskbar. Keep it always open but offscreen when idle to avoid show/hide animation lag. Detect fullscreen/borderless-fullscreen apps and hide the overlay — check if the foreground window covers the full monitor rect on focus change.
make the default hotkey LeftAlt + Q
add input device selection (list all devices, allow user to select one)
add an overlay option for "always show vs only when recording"
format the about page a little more nicely
fix not being able to use the hotkey while the overlay is focused
