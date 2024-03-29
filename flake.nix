{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };

    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";

    treefmt-nix.url = "github:numtide/treefmt-nix";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs @ {
    nixpkgs,
    flake-parts,
    naersk,
    treefmt-nix,
    ...
  }:
    flake-parts.lib.mkFlake {inherit inputs;}
    {
      imports = [
        treefmt-nix.flakeModule
        flake-parts.flakeModules.easyOverlay
      ];

      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem = {
        pkgs,
        self',
        ...
      }: let
        naersk' = pkgs.callPackage naersk {};
      in {
        overlayAttrs = {
          inherit (self'.packages) default;
        };

        packages.default = naersk'.buildPackage {src = ./.;};

        devShells = {
          default = pkgs.mkShell {
            name = "qrrs-dev";
            nativeBuildInputs = with pkgs; [
              rustc
              cargo

              cargo-tarpaulin
              cargo-edit

              rustfmt
              clippy

              act
            ];
            RUST_BACKTRACE = 1;
          };

          testing = pkgs.mkShell {
            name = "qrrs-test";
            nativeBuildInputs = [self'.packages.default];
          };
        };

        treefmt = {
          projectRootFile = "flake.nix";

          programs = {
            alejandra.enable = true;
            rustfmt.enable = true;
            yamlfmt.enable = true;
            prettier.enable = true;
          };
        };
      };
    };
}
