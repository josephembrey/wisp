# Wisp

Push-to-talk whisper dictation desktop app.

Hold a global hotkey to record from your mic. On release, audio is transcribed
locally with [whisper.cpp](https://github.com/ggerganov/whisper.cpp) and the
text is sent to your clipboard or pasted at your cursor.

Runs as a system tray icon with no main window. Settings accessible from the
tray menu.

## Features

- **Local transcription** — all processing happens on-device, nothing leaves your machine
- **Push-to-talk** — hold a configurable hotkey to record, release to transcribe
- **File transcription** — drag-and-drop audio/video files to transcribe
- **Multiple models** — tiny through large, downloaded on demand
- **GPU acceleration** — Vulkan on Windows/Linux, Metal on macOS
- **Output modes** — copy to clipboard or type at cursor
- **Status overlay** — unobtrusive pill indicator shows recording/processing state
- **Transcription history** — searchable log of past transcriptions
- **Autostart** — optional launch on system startup
- **Configurable** — hotkeys, model, language, overlay position/size, input device

## Development

**Prerequisites:** Rust, Bun, LLVM/Clang, Vulkan SDK (Windows/Linux) or Xcode (macOS)

```sh
# Windows — install all dependencies
.\win.ps1 install

# Linux/macOS — enter the Nix dev environment
devenv shell

# Run in development
bun tauri dev

# Production build
bun tauri build
```

See `.\win.ps1` for the full list of Windows commands (build, check, format, sign, etc.).

## Tech stack

- **Frontend:** SvelteKit 2 (Svelte 5), Tailwind CSS v4, TypeScript
- **Backend:** Tauri v2 (Rust)
- **Audio:** cpal (mic capture), symphonia (audio/video file decoding)
- **Transcription:** whisper-rs (whisper.cpp bindings)
- **Hotkeys:** tauri-plugin-global-shortcut
- **Output:** arboard (clipboard), enigo (paste-at-cursor)
- **IPC:** tauri-specta (auto-generated TypeScript bindings)
