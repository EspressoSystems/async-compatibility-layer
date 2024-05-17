{
  description = "A devShell example";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.rust-overlay.url = "github:oxalica/rust-overlay";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        CARGO_TARGET_DIR = "target_dirs/nix_rustc";
        devShells.default = mkShell {
          buildInputs = [
            just
            rust-bin.stable.latest.default
          ];
        };
      }
    );
}
