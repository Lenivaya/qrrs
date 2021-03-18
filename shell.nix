{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell {
  name = "qrrs";
  buildInputs = [ cargo rustc glibc pkg-config rustfmt ];

  RUST_BACKTRACE = 1;
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
