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

    gitignore.url = "github:hercules-ci/gitignore.nix";
    gitignore.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs @ {
    nixpkgs,
    flake-parts,
    naersk,
    treefmt-nix,
    gitignore,
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
        lib,
        ...
      }: let
        naersk' = pkgs.callPackage naersk {};
        inherit (gitignore.lib) gitignoreSource;
        src = gitignoreSource ./.;
      in {
        overlayAttrs = {
          inherit (self'.packages) default;
        };

        packages.default = naersk'.buildPackage {
          inherit src;
          buildInputs = with pkgs;
            lib.optional stdenv.isDarwin libiconv;
        };

        devShells = {
          default = pkgs.mkShell {
            name = "qrrs-dev";
            nativeBuildInputs = with pkgs;
              [
                rustc
                cargo

                cargo-tarpaulin
                cargo-edit

                rustfmt
                clippy

                act
              ]
              ++ lib.optional stdenv.isDarwin libiconv;
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
