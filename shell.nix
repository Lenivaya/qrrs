{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell {
  name = "qrrs-dev";
  buildInputs = [
    # Rust
    rustup
    rustfmt
    rls

  ];
}
