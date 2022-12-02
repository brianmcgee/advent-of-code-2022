{ system ? builtins.currentSystem
, inputs ? import ./flake.lock.nix { }
, nixpkgs ? import inputs.nixpkgs {
    inherit system;
    # Makes the config pure as well. See <nixpkgs>/top-level/impure.nix:
    config = { };
    overlays = [ ];
  }
, rustPackages ? nixpkgs.rustPackages
}:
let
  lib = nixpkgs.lib;

  cargoToml = with builtins; (fromTOML (readFile ./Cargo.toml));

  # Use the Nix module system to validate the aoc config file format.
  evalModule = config:
    lib.evalModules {
      modules = [
        {
          _module.args = { inherit nixpkgs lib aoc; };
        }
        ./module-options.nix
        config
      ];
    };

  # What is used when invoking `nix run github:brianmcgee/advent-of-code-2022`
  aoc = rustPackages.rustPlatform.buildRustPackage {
    inherit (cargoToml.package) name version;

    src = builtins.path {
      path = ./.;
      filter = name: type:
        name == toString ./Cargo.toml
        || name == toString ./Cargo.lock
        || lib.hasPrefix (toString ./src) name
        || lib.hasPrefix (toString ./benches) name
      ;
    };

    buildInputs = with nixpkgs; lib.optionals stdenv.isDarwin [ darwin.apple_sdk.frameworks.Security libiconv ];

    doCheck = true;

    cargoLock.lockFile = ./Cargo.lock;

    meta.description = "one CLI to format the code tree";

    passthru.withConfig = config:
      let
        mod = evalModule config;
      in
      mod.config.build.wrapper;
  };

  # Add all the dependencies of aoc, plus more build tools
  devShell = aoc.overrideAttrs (prev: {
    shellHook = ''
      # Put the aoc binary on the PATH when it's built
      export PATH=$PWD/target/debug:$PATH
    '';

    nativeBuildInputs = prev.nativeBuildInputs ++ (with nixpkgs; [
      # Build tools
      rustPackages.clippy
      rust-analyzer

      # Code formatters
      nixpkgs-fmt
      nodePackages.prettier
      rufo
      rustPackages.rustfmt
      shellcheck
      shfmt

      mdbook
    ]);
  });
in
{
  inherit aoc devShell evalModule;

  # reduce a bit of repetition
  inherit (aoc.passthru) withConfig;

  # A collection of packages for the project
  docs = nixpkgs.callPackage ./docs { };

  # Flake attributes
  default = aoc;

}
