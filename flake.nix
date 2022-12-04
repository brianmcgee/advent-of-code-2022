{
  description = "advent of code 2022";
  # To update all inputs:
  # $ nix flake update --recreate-lock-file
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-parts.url = "github:hercules-ci/flake-parts";

  inputs.devshell = {
    url = github:numtide/devshell;
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-parts, devshell }@inputs:
    flake-parts.lib.mkFlake { inherit self; } {
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem = { system, pkgs, ... }:
        let
          devshell = import inputs.devshell { inherit system; };
        in
        {
          # This contains a mix of packages, modules, ...
          legacyPackages = pkgs;

          devShells.default = devshell.mkShell {
            name = "advent-of-code-2022";
            packages = with pkgs; [
              # Build tools
              cargo
              rustc
              rustPlatform.rustcSrc
              gcc
              rustPackages.clippy
              rust-analyzer

              # Code formatters
              nixpkgs-fmt
              nodePackages.prettier
              rufo
              rustPackages.rustfmt
              shellcheck
              shfmt
            ];
          };

          # In Nix 2.8 you can run `nix fmt` to format this whole repo.
          formatter = pkgs.treefmt;
        };
    };
}
