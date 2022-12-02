{
  description = "advent of code 2022";
  # To update all inputs:
  # $ nix flake update --recreate-lock-file
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-parts.url = "github:hercules-ci/flake-parts";
  inputs.flake-parts.inputs.nixpkgs.follows = "nixpkgs";

  outputs = { self, nixpkgs, flake-parts }@inputs:
    flake-parts.lib.mkFlake { inherit self; } {
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem = { system, pkgs, ... }:
        let
          packages = import ./. {
            inherit system;
            inputs = null;
            nixpkgs = pkgs;
          };
        in
        {
          # This contains a mix of packages, modules, ...
          legacyPackages = packages;

          devShells.default = packages.devShell;

          # In Nix 2.8 you can run `nix fmt` to format this whole repo.
          formatter = pkgs.treefmt;
        };
    };
}
