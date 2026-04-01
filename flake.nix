{
  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = {nixpkgs, ...}: let
    systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    eachSystem = f:
      nixpkgs.lib.genAttrs systems (system: f (import nixpkgs {inherit system;}));
  in {
    devShells = eachSystem (pkgs: {default = import ./tools/shell.nix {inherit pkgs;};});
    packages = eachSystem (pkgs: {
      default = pkgs.callPackage ./tools/package.nix {};
      web = pkgs.callPackage ./tools/web.nix {};
    });
  };
}
