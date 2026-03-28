# TODO: Tauri app derivation for Linux and macOS builds.
#
# Needs:
#   1. Fixed-output derivation for bun install (network access)
#   2. Frontend bundle: bun run build
#   3. rustPlatform.buildRustPackage for src-tauri/
#   4. Tauri CLI bundle step
#   5. whisper-rs native build (cmake, vulkan shaders)
{
  lib,
  stdenv,
  rustPlatform,
  bun,
  cmake,
  ninja,
  pkg-config,
  libclang,
  openssl ? null,
  alsa-lib ? null,
  gtk3 ? null,
  webkitgtk_4_1 ? null,
  librsvg ? null,
  libayatana-appindicator ? null,
  xorg ? null,
  xdotool ? null,
  vulkan-loader ? null,
  vulkan-headers ? null,
  shaderc ? null,
  darwin ? null,
}: let
  isLinux = stdenv.isLinux;
  isDarwin = stdenv.isDarwin;
in
  rustPlatform.buildRustPackage {
    pname = "wisp";
    version = "0.1.0";
    src = ./..;

    cargoLock.lockFile = ../Cargo.lock;

    LIBCLANG_PATH = "${libclang.lib}/lib";

    nativeBuildInputs = [cmake ninja pkg-config libclang.lib];

    buildInputs =
      lib.optionals isLinux [
        openssl
        alsa-lib
        gtk3
        webkitgtk_4_1
        librsvg
        libayatana-appindicator
        xorg.libXtst
        xdotool
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

    meta = {
      description = "Push-to-talk whisper dictation";
      platforms = lib.platforms.linux ++ lib.platforms.darwin;
    };
  }
