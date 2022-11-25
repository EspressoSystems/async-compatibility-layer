{
  description = "Nll crate";

  nixConfig = {
    extra-substituters = [ "https://espresso-systems-private.cachix.org" ];
    extra-trusted-public-keys = [
      "espresso-systems-private.cachix.org-1:LHYk03zKQCeZ4dvg3NctyCq88e44oBZVug5LpYKjPRI="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self, nixpkgs, flake-compat, utils, fenix }:
    utils.lib.eachDefaultSystem (system:
      let
        fenixStable = fenix.packages.${system}.stable.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
          "llvm-tools-preview"
        ];

        CARGO_TARGET_DIR = "target_dirs/nix_rustc";

        rustOverlay = final: prev: {
          rustc = fenixStable;
          cargo = fenixStable;
          rust-src = fenixStable;
        };

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rustOverlay ];
        };

        buildDeps = with pkgs;
          [
            nixpkgs-fmt
            fenix.packages.${system}.rust-analyzer
          ] ++ lib.optionals stdenv.isDarwin [
            darwin.apple_sdk.frameworks.Security
            pkgs.libiconv
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];
      in {
        devShell = pkgs.mkShell {
          inherit CARGO_TARGET_DIR;
          buildInputs = [ fenixStable ] ++ buildDeps;
        };

      });
}
