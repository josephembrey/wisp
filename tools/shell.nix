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

    env.LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
  }
