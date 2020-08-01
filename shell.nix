{ pkgs ? import <nixpkgs> {
  overlays = [
    (import (builtins.fetchTarball
      "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz"))
  ];
} }:

with pkgs;

let
  toolchain = with pkgs.rustChannels.stable;
    (rust.override { extensions = [ "rust-src" ]; });
in mkShell {
  name = "qrrs";
  buildInputs = [ toolchain rustfmt rls pkg-config ];

  RUST_BACKTRACE = 1;

}
