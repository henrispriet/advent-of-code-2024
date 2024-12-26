let
  sources = { nixpkgs = <nixpkgs>; };
in
{ pkgs ? import sources.nixpkgs { config = {}; overlays = []; } }:
pkgs.mkShell {
	packages = with pkgs; [
		cargo
		clippy
		rustfmt
		rust-analyzer
	];
}
