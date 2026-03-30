# Builds the marketing site (web/) into a static directory.
# Usage in NixOS config:
#   services.caddy.virtualHosts."wisp.josephembrey.com".extraConfig = ''
#     root * ${wisp-web}
#     file_server
#   '';
{
  lib,
  stdenvNoCC,
  bun,
  cacert,
}: let
  src = lib.cleanSourceWith {
    src = ./..;
    filter = path: _type: let
      rel = lib.removePrefix (toString ./..) path;
    in
      builtins.any (p: lib.hasPrefix p rel) [
        "/web"
        "/src/routes/layout.css"
        "/package.json"
        "/bun.lock"
      ];
  };
in
  stdenvNoCC.mkDerivation {
    pname = "wisp-web";
    version = "0.1.0";
    inherit src;

    nativeBuildInputs = [bun cacert];

    buildPhase = ''
      export HOME=$TMPDIR
      bun install --frozen-lockfile
      bunx vite build --config web/vite.config.ts
    '';

    installPhase = ''
      cp -r build/web $out
    '';

    meta = {
      description = "Wisp marketing site";
      platforms = lib.platforms.all;
    };
  }
