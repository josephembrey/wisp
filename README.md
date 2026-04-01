# Wisp

Local, private speech-to-text. Push a key, speak, get text.

Wisp runs [Whisper](https://github.com/ggerganov/whisper.cpp) entirely on your machine — no cloud, no API keys, no data leaves your device.

## Features

- **Push-to-talk** — Hold a hotkey to record, release to transcribe
- **GPU accelerated** — Vulkan (Windows/Linux) and Metal (macOS)
- **Two output modes** — Paste at cursor or copy to clipboard, toggle with a hotkey
- **Multiple models** — Tiny (75 MB) to Large (3 GB), download and swap on the fly
- **Status overlay** — Floating pill shows recording/processing/done state
- **Transcription history** — Searchable log of past transcriptions
- **File transcription** — Transcribe audio files (MP3, FLAC, WAV, OGG, etc.)
- **System tray** — Runs in the background, always ready
- **Cross-platform** — Windows, macOS, Linux

## Download

Grab the latest binary from [Releases](https://github.com/josephembrey/wisp/releases). Windows, macOS (ARM + Intel), and Linux.

## Development

**Linux/macOS** — install [Nix](https://nixos.org/) + [direnv](https://direnv.net/), then:

```bash
direnv allow # Run once to allow direnv to load the devshell automatically
just install # Install remaining non-nix dependencies
just dev     # Run in development mode
```

**Windows** — install [Just](https://github.com/casey/just), then:

```powershell
just install # Install dependencies
just dev     # Run in development mode
```

| Command             | Description                                                      |
| ------------------- | ---------------------------------------------------------------- |
| `just bindings`     | Regenerate TypeScript bindings from Rust                         |
| `just build`        | Production build                                                 |
| `just build-debug`  | Production build with verbose logging                            |
| `just check`        | Type-check Rust and Svelte                                       |
| `just clean`        | Remove all build artifacts                                       |
| `just dev`          | Development mode with hot-reload                                 |
| `just icons`        | Regenerate app icons from `src-tauri/icons/icon.png`             |
| `just install [ci]` | Install dependencies (`ci` skips interactive prompts on Windows) |
| `just pre`          | Run linters and formatters                                       |
| `just reload`       | Reload direnv environment                                        |
| `just sign`         | Sign built executable (Windows: Azure, macOS: TODO)              |
| `just web`          | Build marketing site to `build/web`                              |
| `just web-dev`      | Dev server for marketing site                                    |

## Tech Stack

- [Tauri 2](https://v2.tauri.app/) — Desktop framework
- [SvelteKit](https://svelte.dev/) + [Svelte 5](https://svelte.dev/docs/svelte/overview) — Frontend
- [whisper-rs](https://github.com/tazz4843/whisper-rs) — Speech-to-text
- [Tailwind CSS 4](https://tailwindcss.com/) — Styling
- [shadcn-svelte](https://www.shadcn-svelte.com/) — UI components

## License

[GPL-3.0-or-later](LICENSE)
