# Wisp

Push-to-talk whisper dictation desktop app.

## What it does

Hold a global hotkey to record from your mic. On release, audio is transcribed locally with
whisper and the text is sent to your clipboard or pasted at your cursor.

Runs as a system tray icon — no main window. Settings accessible from the tray menu.

## Settings

- **Model:** which whisper model to use (tiny/base/small/medium/large)
- **Output mode:** clipboard or paste-at-cursor
- **Hotkey:** configurable, default RightAlt

## Tech stack

- **Frontend:** SvelteKit 5 (runes syntax), Tailwind CSS v4, TypeScript
- **Backend:** Tauri v2 (Rust)
- **Audio:** cpal (cross-platform mic capture)
- **Transcription:** whisper-rs (whisper.cpp bindings)
- **Hotkey:** rdev (global key press/release listener)
- **Output:** arboard (clipboard), enigo (type text at cursor without touching clipboard)
- **Package manager:** bun

## Project structure

```
src/                    — SvelteKit frontend
  lib/tauri.ts          — TypeScript wrappers for Tauri IPC
  routes/+page.svelte   — settings UI

src-tauri/src/          — Rust backend
  lib.rs                — Tauri setup, tray, state, commands, orchestration
  audio.rs              — mic capture, mono conversion, resample to 16kHz
  whisper.rs            — model download/management + transcription
  hotkey.rs             — global key listener (press + release)
  output.rs             — clipboard + paste simulation
  settings.rs           — settings struct, persistence
```

## Key commands

- `bun tauri dev` — run in development
- `bun tauri build` — production build
- `cargo check` — check Rust compilation (from src-tauri/)

## Implementation plan

See PLAN.md for the step-by-step implementation plan.
