# Design System

## Product Context

- **What this is:** Push-to-talk whisper dictation desktop app
- **Who it's for:** Developers and power users who want local, private speech-to-text
- **Project type:** Tauri tray app with a small settings/control panel (540px fixed width)
- **Default mode:** Dark

## Aesthetic Direction

- **Direction:** Industrial/Utilitarian
- **Decoration level:** Minimal (typography does all the work)
- **Mood:** Quiet, competent tool. Feels like a terminal utility with a GUI face.
  Not flashy, not playful, not corporate. A tool that respects your attention.

## Typography

- **All text:** JetBrains Mono Variable (monospace identity for a dev tool)
- **Loading:** `@fontsource-variable/jetbrains-mono` (self-hosted, no CDN)
- **Scale:** text-xs (12px) for all content, text-sm (14px) for the app name only
- **Weight:** 400 normal, 500 medium for labels, 600 semibold for section headers

## Color

- **Approach:** Restrained (achromatic neutrals, color is rare and semantic only)
- **System:** oklch, fully achromatic (zero chroma) for all theme tokens
- **Dark mode palette:**
  - Background: `oklch(0.145 0 0)` (#252525)
  - Card/Surface: `oklch(0.205 0 0)` (#343434)
  - Border/Input: `oklch(0.269 0 0)` (#434343)
  - Muted text: `oklch(0.708 0 0)` (#b3b3b3)
  - Foreground: `oklch(0.985 0 0)` (#fbfbfb)
  - Primary: `oklch(0.922 0 0)` (#ebebeb)
- **Semantic colors (used sparingly):**
  - Destructive: `oklch(0.704 0.191 22.216)` (red, for reset/delete/errors)
  - Overlay recording: `#ef4444` (red pulse)
  - Overlay processing: `#fbbf24` (amber spinner)
  - Overlay success: `#4ade80` (green check)
- **Rule:** No accent color. No brand color. Color only appears for semantic meaning.

## Spacing

- **Base unit:** 4px
- **Density:** Compact (this is a small tray app, not a dashboard)
- **Common gaps:** gap-1.5 (6px) within fields, gap-3 (12px) between fields, p-2 (8px) window padding, p-3 (12px) content padding

## Layout

- **Approach:** Utility-first composition with shadcn-svelte components
- **Window:** 540px fixed width, variable height (auto-resizes to content)
- **Structure:** Titlebar (32px) + tab bar + tab content
- **No grid system, no layout wrappers.** Controls self-size. Arrange with flexbox and Tailwind spacing.
- **Border radius:** `--radius: 0.625rem` (10px), scaled down for sm/md variants

## Motion

- **Approach:** Minimal-functional
- **Tab height:** CSS transition, 200ms ease-out
- **Badge text:** transition-all 300ms (overlay state changes)
- **Everything else:** no animation. Instant state changes.

## Components

- **Library:** shadcn-svelte (customize in place, don't wrap)
- **Icons:** @lucide/svelte (no inline SVGs)
- **Toggles:** Custom SettingSwitch component wrapping shadcn Switch with role/keyboard handling
- **Toasts:** svelte-sonner for errors only
- **Overlay state:** Unified notification model through the store, rendered by titlebar badge and overlay pill

## Overlay (separate transparent window)

- **Has its own color tokens** (scoped CSS vars, not app theme refs)
- **Visible over any desktop background** (dark bg with blur, white text)
- **Semantic by icon:** dot=idle, pulse=recording, spinner=processing, check=success, x=warning

## Interaction States

- **Loading:** Settings window shows content skeleton or {#if} guard until data arrives.
  Do not assume loading is instant. The app may run on slow hardware or cold-start
  with a large model. Show the titlebar and tab bar immediately, content area waits.
- **Empty:** History tab: "No transcription history yet." Model list: always populated
  (available models are hardcoded, download state varies). Input devices: "Default" fallback.
- **Errors:** Toast notifications via svelte-sonner. Never silent failures.
- **Success:** Overlay state (check icon, "Saved"/"Deleted", 750ms TTL)

## What to avoid

- Color for decoration (no accent backgrounds, no colored borders)
- Uniform row layouts (controls should vary in visual weight)
- Wrapper components that homogenize control sizing
- Inline SVGs (use Lucide)
- Frontend timer hacks for transient state (use the overlay state model)
- Any font other than JetBrains Mono
