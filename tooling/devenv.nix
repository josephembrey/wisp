{pkgs, lib, ...}: let
  isLinux = pkgs.stdenv.isLinux;
  isDarwin = pkgs.stdenv.isDarwin;

  vulkanSdk = pkgs.symlinkJoin {
    name = "vulkan-sdk";
    paths = with pkgs; [vulkan-loader vulkan-headers shaderc.bin];
  };
in {
  packages = with pkgs;
    [
      cmake
      pkg-config
      libclang.lib
      openssl
    ]
    ++ lib.optionals isLinux [
      alsa-lib
      gtk3
      webkitgtk_4_1
      librsvg
      libayatana-appindicator
      xorg.libXtst
      xdotool
      vulkanSdk
    ]
    ++ lib.optionals isDarwin [
      darwin.apple_sdk.frameworks.AppKit
      darwin.apple_sdk.frameworks.CoreAudio
      darwin.apple_sdk.frameworks.AudioToolbox
      darwin.apple_sdk.frameworks.Metal
    ];

  languages = {
    javascript = {
      enable = true;
      bun = {
        enable = true;
        install.enable = true;
      };
    };
    nix.enable = true;
    rust = {
      enable = true;
      channel = "stable";
    };
    typescript.enable = true;
  };

  claude.code = {
    enable = true;
    mcpServers = {
      svelte = {
        type = "http";
        url = "https://mcp.svelte.dev/mcp";
      };
    };
  };

  git-hooks = {
    enable = true;
    hooks = {
      alejandra.enable = true;
      check-json = {
        enable = true;
        excludes = ["tsconfig\\.json$"];
      };
      check-toml.enable = true;
      clippy.enable = true;
      commitizen.enable = true;
      end-of-file-fixer.enable = true;
      eslint = {
        enable = true;
        settings.binPath = "./node_modules/.bin/eslint";
      };
      prettier = {
        enable = true;
        settings.binPath = "./node_modules/.bin/prettier";
      };
      rustfmt.enable = true;
      trim-trailing-whitespace.enable = true;
    };
  };

  enterShell = "";

  scripts.pre.exec = "cd \"$(git rev-parse --show-toplevel)\" && prek run --all-files";
  scripts.clean.exec = "cd \"$(git rev-parse --show-toplevel)\" && cargo clean";

  scripts.dev.exec = ''
    cd "$(git rev-parse --show-toplevel)"
    bun tauri dev
  '';
  scripts.build.exec = ''
    cd "$(git rev-parse --show-toplevel)"
    echo "Generating TypeScript bindings..."
    cargo run --manifest-path src-tauri/Cargo.toml --bin generate-bindings
    echo "Bindings generated."
    bun tauri build
  '';
  scripts.check.exec = ''
    cd "$(git rev-parse --show-toplevel)"
    echo "[cargo check] Checking Rust compilation..."
    cargo check --manifest-path src-tauri/Cargo.toml
    echo "[svelte-check] Checking SvelteKit types..."
    bun run check
    echo "[lint] Checking formatting and linting..."
    bun run lint
    echo "All checks passed."
  '';

  env.LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  env.VULKAN_SDK = if isLinux then "${vulkanSdk}" else "";
}
