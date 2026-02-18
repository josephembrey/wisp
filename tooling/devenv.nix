{pkgs, ...}: let
  vulkanSdk = pkgs.symlinkJoin {
    name = "vulkan-sdk";
    paths = with pkgs; [vulkan-loader vulkan-headers shaderc.bin];
  };
in {
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

    # GPU acceleration (Vulkan)
    vulkanSdk

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

    # Set up Windows Vulkan SDK for cross-compilation
    WIN_VK="$(pwd)/target/win-vulkan-sdk"
    mkdir -p "$WIN_VK/Lib" "$WIN_VK/Bin"
    ln -sfn "${vulkanSdk}/include" "$WIN_VK/Include"
    ln -sfn "${vulkanSdk}/bin/glslc" "$WIN_VK/Bin/glslc"
    if [ ! -f "$WIN_VK/Lib/vulkan-1.lib" ]; then
      curl -sL https://raw.githubusercontent.com/KhronosGroup/Vulkan-Loader/main/loader/vulkan-1.def -o "$WIN_VK/vulkan-1.def"
      llvm-dlltool -m i386:x86-64 -d "$WIN_VK/vulkan-1.def" -l "$WIN_VK/Lib/vulkan-1.lib"
    fi
    # whisper-rs-sys build.rs uses cfg!(target_os) which evaluates to HOST,
    # so on Linux it emits cargo:rustc-link-lib=vulkan instead of vulkan-1
    cp -f "$WIN_VK/Lib/vulkan-1.lib" "$WIN_VK/Lib/vulkan.lib"
    export VULKAN_SDK="$WIN_VK"
    export CMAKE_LIBRARY_PATH="''${CMAKE_LIBRARY_PATH:+$CMAKE_LIBRARY_PATH:}$WIN_VK/Lib"
    # Add to Rust linker search path (CMAKE_LIBRARY_PATH only affects cmake, not lld-link)
    export CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_RUSTFLAGS="''${CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_RUSTFLAGS:+$CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_RUSTFLAGS }-Lnative=$WIN_VK/Lib"

    # Shadow gcc with clang to avoid collect2 posix_spawn failure (nix PATH too long)
    GCC_SHIM="$(pwd)/target/gcc-shim"
    mkdir -p "$GCC_SHIM"
    ln -sfn "$(which clang)" "$GCC_SHIM/gcc"
    ln -sfn "$(which clang++)" "$GCC_SHIM/g++"
    export PATH="$GCC_SHIM:$PATH"

    # Clean cached cmake host compiler detection to ensure gcc shim is used
    rm -rf target/x86_64-pc-windows-msvc/release/build/whisper-rs-sys-*/out/build/ggml/src/ggml-vulkan/vulkan-shaders-gen-prefix

    bun tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc --no-bundle
  '';

  env.LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  env.VULKAN_SDK = "${vulkanSdk}";
}
