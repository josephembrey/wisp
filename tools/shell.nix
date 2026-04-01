{pkgs}: let
  inherit (pkgs) lib stdenv;
  isLinux = stdenv.isLinux;
  isDarwin = stdenv.isDarwin;

  # Tauri's macOS bundler calls `base64 --decode` (BSD flag), but Nix's GNU
  # coreutils base64 uses `-d`. This shim forwards to the system binary.
  macosBase64 = pkgs.writeShellScriptBin "base64" ''exec /usr/bin/base64 "$@"'';
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
      ++ lib.optionals isDarwin [
        macosBase64
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
      ];

    buildInputs = lib.optionals isDarwin [
      (pkgs.darwinMinVersionHook "10.15")
    ];

    shellHook = ''
      prek install -q --config tools/prek.toml --hook-type pre-commit --hook-type commit-msg
    '';

    env.LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    env.MACOSX_DEPLOYMENT_TARGET = lib.optionalString isDarwin "10.15";
    env.CMAKE_OSX_DEPLOYMENT_TARGET = lib.optionalString isDarwin "10.15";
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
