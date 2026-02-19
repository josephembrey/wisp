# Wisp

Push-to-talk whisper dictation desktop app.

## What it does

Hold a global hotkey to record from your mic. On release, audio is transcribed locally with
whisper and the text is sent to your clipboard or pasted at your cursor.

Runs as a system tray icon — no main window. Settings accessible from the tray menu.

## Settings

- **Model:** which whisper model to use (tiny/base/small/medium/large)
- **Output mode:** clipboard or paste-at-cursor
- **Hotkey:** configurable, default Alt+Q

## Tech stack

- **Frontend:** SvelteKit 5 (runes syntax), Tailwind CSS v4, TypeScript
- **Backend:** Tauri v2 (Rust)
- **Audio:** cpal (cross-platform mic capture)
- **Transcription:** whisper-rs (whisper.cpp bindings)
- **Hotkey:** tauri-plugin-global-shortcut (+ Windows polling workaround via GetAsyncKeyState)
- **Output:** arboard (clipboard), enigo (type text at cursor without touching clipboard)
- **IPC types:** tauri-specta (auto-generated TypeScript bindings from Rust types)
- **Package manager:** bun

## Project structure

```
src/                                — SvelteKit frontend
  lib/bindings.ts                   — auto-generated TypeScript types (tauri-specta)
  lib/tauri.ts                      — IPC wrappers and event listeners
  lib/components/settings/          — settings window tab components
  lib/components/overlay/           — overlay pill component
  routes/+page.svelte               — settings UI (main window)
  routes/overlay/+page.svelte       — overlay window

src-tauri/src/                      — Rust backend
  lib.rs                            — Tauri setup, plugin registration, shortcuts
  engine.rs                         — event loop: hotkey → record → transcribe → output
  commands.rs                       — Tauri IPC command handlers
  tray.rs                           — system tray icon and menu
  output.rs                         — clipboard + paste simulation
  audio/                            — mic capture, mono conversion, resample to 16kHz
  whisper/                          — model download/management + transcription
  hotkey/                           — key conversion + Windows polling workaround
  settings/                         — settings struct, persistence, app state
```

## Key commands

- `bun tauri dev` — run in development
- `bun tauri build` — production build
- `cargo check --manifest-path src-tauri/Cargo.toml` — check Rust compilation
- `cargo run --manifest-path src-tauri/Cargo.toml --bin generate_bindings` — regenerate TypeScript bindings
