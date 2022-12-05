{
  description = "advent of code 2022";
  # To update all inputs:
  # $ nix flake update --recreate-lock-file
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";

  inputs.devshell = {
    url = github:numtide/devshell;
    inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, flake-utils, devshell, rust-overlay }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = import nixpkgs { inherit system overlays; };
        devshell = import inputs.devshell { inherit system; };

        rustVersion = pkgs.rust-bin.stable.latest.default;
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        inherit (pkgs) lib stdenv;
      in
      {
        # This contains a mix of packages, modules, ...
        legacyPackages = pkgs;

        devShells.default = devshell.mkShell {
          name = "advent-of-code-2022";
          env = [
            {
              # On darwin for example enables finding of libiconv
              name = "LIBRARY_PATH";
              # append in case it needs to be modified
              eval = "$DEVSHELL_DIR/lib";
            }
            {
              # some *-sys crates require additional includes
              name = "CFLAGS";
              # append in case it needs to be modified
              eval = "\"-I $DEVSHELL_DIR/include ${lib.optionalString stdenv.isDarwin "-iframework $DEVSHELL_DIR/Library/Frameworks"}\"";
            }
          ] ++ lib.optionals stdenv.isDarwin [
            {
              # On darwin for example required for some *-sys crate compilation
              name = "RUSTFLAGS";
              # append in case it needs to be modified
              eval = "\"-L framework=$DEVSHELL_DIR/Library/Frameworks\"";
            }
            {
              # rustdoc uses a different set of flags
              name = "RUSTDOCFLAGS";
              # append in case it needs to be modified
              eval = "\"-L framework=$DEVSHELL_DIR/Library/Frameworks\"";
            }
          ];

          packages = with pkgs; [
            # Build tools
            (rustVersion.override {
              extensions = [ "rust-src" "rustfmt"];
            })
            rust-analyzer
            gcc

            # Code formatters
            nixpkgs-fmt
            nodePackages.prettier
            shellcheck
            shfmt
          ] ++ lib.optionals stdenv.isDarwin [ libiconv ];
        };

        # In Nix 2.8 you can run `nix fmt` to format this whole repo.
        formatter = pkgs.treefmt;
      });

}
