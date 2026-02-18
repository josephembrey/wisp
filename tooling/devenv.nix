{pkgs, ...}: {
  packages = with pkgs; [
    cmake
    pkg-config
    alsa-lib
    libclang.lib
    openssl
    gtk3
    webkitgtk_4_1
    librsvg
    libayatana-appindicator
    xorg.libXtst
    xdotool

    # Windows cross-compilation
    cargo-xwin
    ninja
    nsis
    clang
    llvmPackages.lld
    llvmPackages.llvm
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
      targets = ["x86_64-pc-windows-msvc"];
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
      clippy = {
        enable = true;
        packageOverrides = let
          toolchain = pkgs.rust-bin.stable.latest.default;
        in {
          cargo = toolchain;
          clippy = toolchain;
        };
      };
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

  scripts.pre.exec = "cd \"$(git rev-parse --show-toplevel)\" && prek run --all-files";
  scripts.clean.exec = "cd \"$(git rev-parse --show-toplevel)\" && cargo clean";
  scripts.build-windows.exec = ''
    cd "$(git rev-parse --show-toplevel)"
    XWIN_CACHE="''${CARGO_XWIN_CACHE_DIR:-$HOME/.cache/cargo-xwin}/xwin"
    export BINDGEN_EXTRA_CLANG_ARGS_x86_64_pc_windows_msvc="--target=x86_64-pc-windows-msvc -I$XWIN_CACHE/sdk/include/ucrt -I$XWIN_CACHE/crt/include -I$XWIN_CACHE/sdk/include/shared"
    bun tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc --no-bundle
  '';

  env.LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
}
