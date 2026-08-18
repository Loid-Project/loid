{
  description = "Loid - a programming language from the FUTURE !! CALLED LOID !!";

  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };

    flake-utils = {
      url = "github:numtide/flake-utils";
    };

    crane = {
      url = "github:ipetkov/crane";
      # inputs.nixpkgs.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      crane,
      rust-overlay,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        commonArgs = {
          src = ./.;
          buildInputs = with pkgs; [
            openssl
            pkg-config
          ];
          nativeBuildInputs = with pkgs; [ pkg-config ];
        };

        loid = craneLib.buildPackage (
          commonArgs
          // {
            cargoLock = ./Cargo.lock;
          }
        );
      in
      {
        packages.default = loid;
        apps.default = flake-utils.lib.mkApp { drv = loid; };
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # rust stuff
            rustToolchain
            openssl
            pkg-config
            cargo
            rustc
            rustfmt
            clippy
            rust-analyzer

            # node
            nodejs_26
            javascript-typescript-langserver
            prettier

            # nix
            nixfmt
            nixd

            # just
            just
            just-formatter
            just-lsp
          ];
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
