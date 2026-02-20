# Wisp

Push-to-talk whisper dictation desktop app.

## What it does

Hold a global hotkey to record from your mic. On release, audio is transcribed locally with
whisper and the text is sent to your clipboard or pasted at your cursor. Also supports
drag-and-drop file transcription for audio files.

Runs as a system tray icon — no main window. A configurable overlay pill shows recording/processing
state. Settings accessible from the tray menu.

## Settings

- **Model:** which whisper model to use (tiny/base/small/medium/large)
- **Output mode:** clipboard or paste-at-cursor
- **Hotkey:** configurable, default Alt+Q
- **Output hotkey:** optional hotkey to toggle output mode
- **Language:** auto-detect or specific language
- **GPU:** enable/disable GPU acceleration
- **Interrupt:** interrupt previous transcription when starting new one
- **Min duration:** minimum recording duration threshold (seconds)
- **Input device:** select microphone device
- **Model loading:** when to load model (eager/lazy/per-use)
- **Autostart:** start with system
- **Overlay:** enable/disable, position, size, monitor, always show
- **History:** enable transcription history and retention count

## Tech stack

- **Frontend:** SvelteKit 2 (Svelte 5 runes syntax), Tailwind CSS v4, TypeScript
- **Backend:** Tauri v2 (Rust)
- **Audio:** cpal (mic capture), symphonia (audio file decoding)
- **Transcription:** whisper-rs (whisper.cpp bindings)
- **Hotkey:** tauri-plugin-global-shortcut (+ Windows polling workaround via GetAsyncKeyState)
- **Output:** arboard (clipboard), enigo (type text at cursor without touching clipboard)
- **IPC types:** tauri-specta (auto-generated TypeScript bindings from Rust types)
- **Plugins:** dialog, autostart, window-state, single-instance, log
- **Package manager:** bun

## Project structure

```
src/                                — SvelteKit frontend
  lib/bindings.ts                   — auto-generated TypeScript types (tauri-specta)
  lib/tauri.ts                      — IPC wrappers and event listeners
  lib/components/settings/          — settings window tab components
  lib/components/overlay/           — overlay pill component
  lib/components/ui/                — bits-ui components (button, switch, select, etc.)
  routes/+page.svelte               — settings UI (main window)
  routes/overlay/+page.svelte       — overlay window

src-tauri/src/                      — Rust backend
  lib.rs                            — Tauri setup, plugin registration, shortcuts
  engine.rs                         — event loop: hotkey → record → transcribe → output
  commands.rs                       — Tauri IPC command handlers
  tray.rs                           — system tray icon, menu, overlay window
  output.rs                         — clipboard + paste simulation
  history.rs                        — transcription history persistence
  audio/                            — mic capture, file decoding, resample to 16kHz
  whisper/                          — model download/management + transcription
  hotkey/                           — key conversion + Windows polling workaround
  settings/                         — settings struct, persistence, app state
```

## Key commands

- `bun tauri dev` — run in development
- `bun tauri build` — production build
- `cargo check --manifest-path src-tauri/Cargo.toml` — check Rust compilation
- `cargo run --manifest-path src-tauri/Cargo.toml --bin generate_bindings --features gen-bindings` — regenerate TypeScript bindings

## Patterns

### Switch components (bits-ui)

The bits-ui `Switch` fires `onCheckedChange` on programmatic prop updates, not just user clicks.
This causes feedback loops when settings are echoed back from the backend via `settings-changed` events.

**Fix:** Make the Switch visual-only (`pointer-events-none`) and handle clicks on a parent `<div>`
(not `<button>`, which retains focus and captures hotkey release events):

```svelte
<div class="cursor-pointer" onclick={() => onsave({ enabled: !settings.enabled })}>
  <Switch checked={settings.enabled} class="pointer-events-none" />
</div>
```
