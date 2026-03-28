# Wisp — push-to-talk whisper dictation

# On Windows, use a short cargo target dir to avoid MAX_PATH failures
# (whisper.cpp Vulkan shader builds create deeply nested paths)
export CARGO_TARGET_DIR := env_var_or_default("CARGO_TARGET_DIR", if os() == "windows" { "C:/wt" } else { "src-tauri/target" })

manifest := "src-tauri/Cargo.toml"

# List available commands
default:
    @just --list

# Run in development mode
dev: _ensure-deps
    bun tauri dev

# Build for production
build: _ensure-deps bindings
    bun tauri build

# Build with verbose logging enabled
build-debug: _ensure-deps bindings
    bun tauri build -- --features verbose-log

# Build and sign for production (Windows)
[windows]
build-sign: build sign

# Build with verbose logging and sign (Windows)
[windows]
build-sign-debug: build-debug sign

# Generate TypeScript bindings from Rust types
bindings:
    cargo run --manifest-path {{manifest}} --bin generate_bindings --features gen-bindings

# Run all checks (cargo, svelte, lint)
check: check-rust check-svelte check-lint

# Check Rust compilation
check-rust:
    cargo check --manifest-path {{manifest}}

# Check SvelteKit types
check-svelte:
    bun run check

# Run prettier and eslint
check-lint:
    bun run prettier --check .
    bun run eslint .

# Auto-format all code
format:
    bun run format
    cargo fmt --manifest-path {{manifest}}

# Clean build artifacts
clean:
    cargo clean --manifest-path {{manifest}}
    rm -rf .svelte-kit build src-tauri/gen/schemas

# Clean everything including node_modules
clean-all: clean
    rm -rf node_modules

# Sign the built executable (Windows)
[windows]
sign:
    powershell -NoProfile -File signing/sign.ps1

# Install development dependencies (Windows)
[windows]
install *scope:
    powershell -NoProfile -File tooling/windows/install.ps1 {{scope}}

# Run pre-commit hooks on all files
pre:
    prek run --all-files

[private]
_ensure-deps:
    @[ -d node_modules ] || bun install
