{
  description = "A basic flake with a shell";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    naersk,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };
      naersk' = pkgs.callPackage naersk {};
      generic_c_drv = naersk'.buildPackage {
        src = ./.;
      };
    in {
      packages = rec {
        default = generic_c;
        generic_c = generic_c_drv;
      };

      app = flake-utils.lib.mkApp {drv = generic_c_drv;};
      devShell = with pkgs; let
        rust_dev =
          rust-bin.stable.latest.default.override
          {
            extensions = ["rust-src"];

            targets = ["wasm32-unknown-unknown" "x86_64-unknown-linux-gnu"];
          };
      in
        mkShell {
          nativeBuildInputs = [
            pkgs.bashInteractive
          ];
          buildInputs = [
            # Rust
            cmake
            rust_dev
          ];
        };
    });
}
