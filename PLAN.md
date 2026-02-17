# Wisp — Implementation Plan

Push-to-talk whisper dictation: hold a hotkey to record, release to transcribe, output text.

---

## How it works (big picture)

```
[Hold hotkey] → record mic audio
[Release]     → send audio to whisper → get text → clipboard or paste at cursor
```

The app runs as a **system tray icon** — no visible window. Right-click the tray to open a
small settings panel where you pick your whisper model, hotkey, and output mode.

---

## Step 1: Cargo dependencies

**What:** Add all the Rust crates we need to `src-tauri/Cargo.toml`.

**Why each one:**

| Crate | Purpose |
|-------|---------|
| `whisper-rs` | Rust bindings to whisper.cpp — does the actual speech-to-text |
| `cpal` | Cross-platform microphone capture. Gives us raw audio samples |
| `rdev` | Listens for global keyboard events (key press AND release). Tauri's built-in global-shortcut plugin only fires on press, not release — we need release to know when to stop recording |
| `arboard` | Cross-platform clipboard access (read/write) |
| `enigo` | Simulates keyboard input — used to send Ctrl+V for "paste at cursor" mode |
| `reqwest` | HTTP client to download whisper model files from HuggingFace |
| `futures-util` | Needed to stream the download response body (for progress reporting) |
| `parking_lot` | Faster mutex implementation — used for shared state between threads |

We also enable the `tray-icon` feature on the `tauri` crate, which unlocks the system tray API.

**Note:** `whisper-rs` compiles whisper.cpp from C++ source during build. This requires a
C/C++ compiler (MSVC on Windows, gcc on Linux). You should already have this since Tauri
requires it too.

---

## Step 2: System tray + hidden window

**What:** Make the app launch to the system tray with no visible window.

**Changes to `tauri.conf.json`:**
- Set `visible: false` on the window — it starts hidden
- Shrink the window to 480x600 (it's just a settings panel)
- Add `withGlobalTauri: true` so the frontend can listen to events from Rust

**Changes to `lib.rs`:**
- Build a tray icon with a right-click menu: "Settings" and "Quit"
- "Settings" shows the hidden window, "Quit" exits the app
- When you close the settings window, it hides instead of quitting (override close event)

**Why:** This is the app shell. Everything else plugs into it. A tray app is the right UX for
a background utility — you don't want a window sitting on your taskbar.

---

## Step 3: Settings + app state

**What:** Define the settings the user can configure, persist them to disk, and expose them
to the frontend via Tauri commands.

**New file `settings.rs`:**
- `Settings` struct: `model` (string), `output_mode` (clipboard/paste), `hotkey` (string)
- Load/save to a JSON file in the app data directory
- Defaults: base model, clipboard mode, RightAlt hotkey

**New file concept — `AppState`:**
- Holds the current settings (behind a Mutex so multiple threads can access)
- Holds the current status (idle/recording/processing)
- Holds paths to the data and models directories

**Tauri commands** (functions the frontend can call):
- `get_settings` → returns current settings
- `update_settings` → saves new settings to disk
- `get_status` → returns current app status

**Why:** The frontend needs to read and write settings. Tauri commands are the bridge between
the Svelte UI and the Rust backend — the frontend calls `invoke('get_settings')` and gets
back the JSON. The Mutex is needed because multiple threads (hotkey listener, audio thread,
main thread) may need to read settings simultaneously.

---

## Step 4: Model management

**What:** Download whisper model files from HuggingFace, list available models, delete models.

**Added to `whisper.rs`:**
- Model files are GGML format, hosted at `huggingface.co/ggerganov/whisper.cpp`
- Available: tiny (75MB), base (142MB), small (466MB), medium (1.5GB), large (3GB)
- Stored in `{app_data_dir}/models/ggml-{name}.bin`

**Tauri commands:**
- `get_models` → lists all models with download status
- `download_model` → streams the download, emits progress events to the frontend
- `delete_model` → removes a model file

**Why:** Whisper needs a model file to run. Users should be able to pick their size/quality
tradeoff. Tiny is fast but less accurate. Large is very accurate but slow and 3GB. We stream
the download so the UI can show a progress bar.

---

## Step 5: Audio capture

**What:** Record audio from the default microphone and prepare it for whisper.

**New file `audio.rs`:**
- `AudioRecorder` struct with `start()` and `stop()` methods
- `start()` opens the default mic with `cpal` and buffers raw audio samples
- `stop()` returns the buffered audio, converted to the format whisper expects

**Audio processing (inside `stop()`):**
1. Convert stereo to mono (average the channels)
2. Resample to 16,000 Hz (whisper's expected sample rate)
   - Most mics record at 44,100 or 48,000 Hz
   - We use simple linear interpolation — good enough for speech

**Why `cpal`:** It's the standard Rust crate for audio. It abstracts away platform differences
(WASAPI on Windows, ALSA on Linux, CoreAudio on macOS). The audio callback runs on cpal's
own thread — we just provide a closure that appends samples to a buffer.

**Why resample:** Whisper was trained on 16kHz audio. Feeding it 48kHz audio would sound like
chipmunks to the model. Linear interpolation is fine for speech (a music app would need a
fancier resampler, but we don't).

---

## Step 6: Whisper integration

**What:** Load a model file and transcribe audio.

**Added to `whisper.rs`:**
- `WhisperEngine` struct with `load_model()` and `transcribe()` methods
- `load_model()` loads the GGML file into memory (this takes a few seconds for larger models)
- `transcribe()` runs inference on f32 audio samples and returns text
- Uses greedy decoding, English language, single segment (one continuous output)

**Why a separate engine struct:** Loading a model is expensive (seconds). We load it once and
reuse it across multiple transcriptions. If the user changes their model selection, we reload.

---

## Step 7: Global hotkey

**What:** Detect when the user presses and releases their configured hotkey, anywhere in the
system.

**New file `hotkey.rs`:**
- Uses `rdev` to listen for ALL keyboard events globally
- Filters for the configured hotkey key
- Sends `Pressed` / `Released` events over a channel

**Why `rdev` instead of Tauri's global shortcut plugin:** Tauri's plugin registers hotkeys
and fires on press, but has no concept of release. We need release to know when to stop
recording. `rdev` gives us raw KeyPress and KeyRelease events.

**Key detail — key repeat filtering:** When you hold a key on Windows, the OS sends repeated
press events. We track `is_pressed` state and ignore repeats, so we only get one "start
recording" event per hold.

**Key detail — runtime hotkey changes:** The `rdev` listener runs on its own thread and
blocks forever. You can't restart it to change the hotkey. Instead, the current hotkey is
stored in an `Arc<Mutex<Key>>` that the listener checks on every event. When the user changes
their hotkey in settings, we update the mutex value and the listener picks it up immediately.

---

## Step 8: Output

**What:** Put the transcribed text where the user wants it.

**New file `output.rs`:**
- **Clipboard mode:** Write text to system clipboard using `arboard`
- **Cursor mode:** Type text directly at the cursor using `enigo`'s `text()` method

**Why `enigo::text()` for cursor mode:** This simulates typing the text character by
character at wherever the cursor is. It doesn't touch the clipboard, so whatever the user
had copied stays intact. Since wisp runs in the background (tray app), the user's active
window keeps focus, so the keystrokes go to the right place.

---

## Step 9: Wire it all together

**What:** Connect the hotkey → audio → whisper → output pipeline in `lib.rs`.

**The event loop (runs on a dedicated thread):**
```
receive hotkey event from channel
  → Pressed:  start audio recording, emit "recording" status
  → Released: stop recording, emit "processing" status,
              run whisper on the audio buffer,
              output the text,
              emit "idle" status
```

**Why a dedicated thread:** The whisper inference is CPU-bound and blocks for potentially
seconds. Running it on this thread keeps the Tauri main thread and UI responsive. We can't
start a new recording during processing anyway (you need to release the key first), so
blocking here is fine.

---

## Step 10: Frontend settings page

**What:** Build the Svelte UI for the settings panel.

**New file `src/lib/tauri.ts`:**
- TypeScript wrappers around `invoke()` calls and event listeners
- Typed interfaces for Settings, ModelInfo, DownloadProgress

**Rewrite `src/routes/+page.svelte`:**
- Status indicator (idle / recording / processing) with color
- Model picker: dropdown of available models, download/delete buttons, progress bar
- Output mode: toggle between clipboard and paste-at-cursor
- Hotkey: shows current hotkey, "Change" button captures next keypress
- Last transcription: shows the most recent result

Uses Svelte 5 runes (`$state`, `$effect`) and Tailwind CSS for styling.

**Why `src/lib/tauri.ts`:** Keeps the Tauri IPC calls in one place with proper TypeScript
types, rather than scattering `invoke()` calls throughout components.

---

## File overview

```
src-tauri/src/
  main.rs        — entry point (no changes needed)
  lib.rs         — tray setup, state init, commands, event loop orchestration
  audio.rs       — mic recording with cpal, mono + resample to 16kHz
  whisper.rs     — model download/management + transcription
  hotkey.rs      — rdev global key listener (press + release)
  output.rs      — clipboard write + optional Ctrl+V simulation
  settings.rs    — Settings/AppState structs, JSON persistence

src/
  lib/tauri.ts   — TypeScript wrappers for Tauri invoke/listen
  routes/
    +page.svelte — settings UI
```

---

## Testing each step

After each step, we verify before moving on:

1. **Dependencies** → `cargo check` in src-tauri (compiles without errors)
2. **Tray + window** → `bun tauri dev` → tray icon appears, Settings/Quit menu works
3. **Settings** → settings save/load from frontend, persist across restart
4. **Models** → download tiny model, see progress, verify file on disk
5. **Audio** → record a few seconds, verify we get non-empty audio buffer
6. **Whisper** → transcribe a test recording, verify text output
7. **Hotkey** → hold key → recording starts, release → processing starts
8. **Output** → text appears in clipboard / at cursor
9. **Full pipeline** → hold hotkey, speak, release, text appears
10. **UI** → all settings controls work, status updates in real time
