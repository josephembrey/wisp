{pkgs}: let
  inherit (pkgs) lib stdenv;
  isLinux = stdenv.isLinux;
  isDarwin = stdenv.isDarwin;
in
  pkgs.mkShell {
    packages = with pkgs;
      [
        just
        bun
        alejandra
        prek
        cmake
        ninja
        pkg-config
        libclang.lib
      ]
      ++ lib.optionals isLinux [
        mold
        # Tauri
        openssl
        alsa-lib
        gtk3
        webkitgtk_4_1
        librsvg
        libayatana-appindicator
        # Input simulation (enigo)
        xorg.libXtst
        xdotool
        # Vulkan (whisper-rs GPU)
        vulkan-loader
        vulkan-headers
        shaderc.bin
      ]
      ++ lib.optionals isDarwin (with darwin.apple_sdk.frameworks; [
        CoreAudio
        AudioToolbox
        WebKit
        AppKit
      ]);

    env.LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";

    shellHook =
      ''
        if ! command -v rustc &> /dev/null; then
          echo "Rust not found. Install via rustup: https://rustup.rs"
        fi
      ''
      + lib.optionalString isDarwin ''
        if ! xcode-select -p &> /dev/null 2>&1; then
          echo "Xcode CLT not found: xcode-select --install"
        fi
      '';
  }
