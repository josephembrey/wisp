# Short target dir on Windows to avoid MAX_PATH failures
export CARGO_TARGET_DIR := if os() == "windows" { "C:/wisp" } else { justfile_directory() / "src-tauri" / "target" }

set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]

# Show all available recipes
default:
    @just --list

# Generate TypeScript bindings
bindings:
    cargo run --manifest-path src-tauri/Cargo.toml --bin generate_bindings --features gen-bindings

# Production build
build: bindings
    bun tauri build

# Production build with verbose logging
build-debug: bindings
    bun tauri build -- --features verbose-log

# Type-check rust and svelte
check:
    cargo check --manifest-path src-tauri/Cargo.toml
    bun run check

# Remove all build artifacts
[unix]
clean:
    cargo clean --manifest-path src-tauri/Cargo.toml
    rm -rf .svelte-kit build src-tauri/gen/schemas node_modules

# Remove all build artifacts
[windows]
clean:
    cargo clean --manifest-path src-tauri/Cargo.toml
    @('.svelte-kit', 'build', 'src-tauri\gen\schemas', 'node_modules') | Where-Object { Test-Path $_ } | Remove-Item -Recurse -Force

# Run in development mode
dev:
    bun tauri dev

# Install dependencies
[linux]
install:
    bun install

# Install dependencies (+ Xcode CLT if missing)
[macos]
install:
    @if ! xcode-select -p &> /dev/null 2>&1; then echo "Installing Xcode CLT..."; xcode-select --install; fi
    bun install

# Install dependencies (system + node)
[windows]
install:
    powershell -NoProfile -File tools/win/install.ps1
    bun install

# Run pre-commit hooks on all files
[unix]
pre:
    prek run --config tools/prek.toml --all-files

# Run pre-commit hooks on all files (skip nix hooks)
[windows]
pre:
    prek run --config tools/prek.toml --all-files --skip alejandra

# Reloads the direnv environment
reload:
    direnv reload

# Sign the built executable
[windows]
sign:
    powershell -NoProfile -File tools/win/sign.ps1

# Sign the built executable (TODO)
[macos]
sign:
    @echo "macOS signing not yet implemented"

# Sign the built executable (not applicable)
[linux]
sign:
    @echo "Linux signing not required"
