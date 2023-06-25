{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";

    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
    treefmt-nix,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
        };

        naersk' = pkgs.callPackage naersk {};
      in rec {
        # For `nix build` & `nix run`:
        defaultPackage = naersk'.buildPackage {
          src = ./.;
        };

        # For `nix develop` (optional, can be skipped):
        devShell = pkgs.mkShell {
          name = "qrrs";

          nativeBuildInputs = with pkgs; [
            rustc
            rustfmt

            cargo
            cargo-tarpaulin
            cargo-edit
          ];

          RUST_BACKTRACE = 1;
        };

        formatter =
          treefmt-nix.lib.mkWrapper
          pkgs
          {
            projectRootFile = "flake.nix";

            programs = {
              alejandra.enable = true;
              rustfmt.enable = true;
            };
          };
      }
    );
}
