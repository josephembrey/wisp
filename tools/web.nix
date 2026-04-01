# Builds the marketing site (web/) into a static directory.
# Usage in NixOS config (where wisp is a flake input):
#   root * ${wisp.packages.${system}.web}
#   file_server
{
  lib,
  stdenvNoCC,
  bash,
  bun,
  cacert,
  coreutils,
}: let
  # FOD using builtins.derivation to avoid mkDerivation putting store paths
  # in env (which Nix 2.31+ rejects for fixed-output derivations).
  bunDeps = derivation {
    name = "wisp-web-deps";
    system = stdenvNoCC.system;
    builder = "${bash}/bin/bash";
    args = [
      "-c"
      ''
        export HOME="$TMPDIR"
        export PATH="${lib.makeBinPath [bun coreutils]}:$PATH"
        export SSL_CERT_FILE="${cacert}/etc/ssl/certs/ca-bundle.crt"
        cp "${../package.json}" package.json
        cp "${../bun.lock}" bun.lock
        bun install --frozen-lockfile
        cp -r node_modules "$out"
      ''
    ];
    outputHashMode = "recursive";
    outputHashAlgo = "sha256";
    outputHash = "sha256-Whr9s6+DUDbuRBqqjucgmNGB3+IyLAlW236XKfNQuBM=";
  };

  src = lib.cleanSourceWith {
    src = ./..;
    filter = path: type: let
      rel = lib.removePrefix (toString ./..) path;
    in
      (type == "directory" && builtins.any (p: lib.hasPrefix rel p) ["/web" "/src/routes/layout.css"])
      || builtins.any (p: lib.hasPrefix p rel) ["/web" "/src/routes/layout.css"];
  };
in
  stdenvNoCC.mkDerivation {
    pname = "wisp-web";
    version = "0.1.0";
    inherit src;

    nativeBuildInputs = [bun];

    buildPhase = ''
      cp -r ${bunDeps} node_modules
      chmod -R u+w node_modules
      export HOME=$TMPDIR
      bun node_modules/vite/bin/vite.js build --config web/vite.config.ts
    '';

    installPhase = ''
      cp -r build/web $out
    '';

    meta = {
      description = "Wisp marketing site";
      platforms = lib.platforms.all;
    };
  }
