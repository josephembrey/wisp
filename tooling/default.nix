{inputs, ...}: {
  perSystem = {
    system,
    pkgs,
    ...
  }: {
    devShells.default =
      if builtins.getEnv "PWD" != ""
      then
        inputs.devenv.lib.mkShell {
          inherit inputs;
          pkgs = import inputs.nixpkgs-devenv {
            inherit system;
          };
          modules = [./devenv.nix];
        }
      else pkgs.mkShell {name = "devenv-requires-impure";};
  };
}
