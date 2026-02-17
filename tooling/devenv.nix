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
    rust.enable = true;
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

  scripts.fmt.exec = "cd \"$(git rev-parse --show-toplevel)\" && prek run --all-files";

  env.LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
}
