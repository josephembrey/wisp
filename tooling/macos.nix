# macOS-specific devenv configuration.
#
# On macOS, Nix-managed Rust and Apple framework packages produce binaries
# linked against /nix/store/ paths instead of system frameworks, which breaks
# native app builds (Tauri, CoreAudio, WebKit, etc.).
#
# This module disables Nix-managed Rust and uses Homebrew + system tooling:
#   - Rust toolchain:    rustup (system)
#   - Apple SDK/frameworks: Xcode Command Line Tools (system)
#   - Build deps:        Homebrew (cmake, pkg-config)
{lib, ...}: let
  brewDeps = ["cmake" "pkg-config"];
in {
  # Disable Nix-managed Rust — use rustup instead.
  # Nix Rust links against Nix store libs instead of system frameworks.
  # See: https://github.com/tauri-apps/tauri/issues/11246
  languages.rust.enable = lib.mkForce false;

  enterShell = lib.mkAfter ''
    # --- macOS environment checks ---

    if ! command -v brew &> /dev/null; then
      echo ""
      echo "WARNING: Homebrew not found."
      echo "  Install: /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
      echo ""
    else
      missing=()
      for dep in ${lib.concatStringsSep " " brewDeps}; do
        if ! brew list "$dep" &> /dev/null 2>&1; then
          missing+=("$dep")
        fi
      done
      if [ ''${#missing[@]} -gt 0 ]; then
        echo "Installing missing Homebrew dependencies: ''${missing[*]}"
        brew install "''${missing[@]}"
      fi
    fi

    if ! command -v rustc &> /dev/null; then
      echo ""
      echo "WARNING: rustc not found."
      echo "  Install via rustup: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
      echo ""
    fi

    if ! xcode-select -p &> /dev/null 2>&1; then
      echo ""
      echo "WARNING: Xcode Command Line Tools not found."
      echo "  Install: xcode-select --install"
      echo ""
    fi
  '';
}
