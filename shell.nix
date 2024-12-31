let
  sources = { nixpkgs = <nixpkgs>; };
in
{ pkgs ? import sources.nixpkgs { config = {}; overlays = []; } }:
pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    clippy
    rustfmt
    rust-analyzer
    cargo-flamegraph
    cargo-aoc
  ];
}
