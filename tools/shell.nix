{pkgs}: let
  inherit (pkgs) lib stdenv;
  isLinux = stdenv.isLinux;
  isDarwin = stdenv.isDarwin;
in
  pkgs.mkShell {
    packages = with pkgs;
      [
        alejandra
        bun
        cargo
        clang
        clippy
        rustfmt
        cmake
        just
        libclang.lib
        ninja
        pkg-config
        prek
        rustc
      ]
      ++ lib.optionals isLinux [
        alsa-lib
        gtk3
        libayatana-appindicator
        librsvg
        mold
        openssl
        shaderc.bin
        vulkan-headers
        vulkan-loader
        webkitgtk_4_1
        xdotool
        xorg.libXtst
      ]
      ++ lib.optionals isDarwin (with darwin.apple_sdk.frameworks; [
        AppKit
        AudioToolbox
        CoreAudio
        WebKit
      ]);

    shellHook = ''
      prek install -q --config tools/prek.toml --hook-type pre-commit --hook-type commit-msg
    '';

    env.LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    env.LD_LIBRARY_PATH = lib.optionalString isLinux (lib.makeLibraryPath [
      pkgs.alsa-lib
      pkgs.gtk3
      pkgs.libayatana-appindicator
      pkgs.librsvg
      pkgs.openssl
      pkgs.vulkan-loader
      pkgs.webkitgtk_4_1
    ]);
  }
