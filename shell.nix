{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  name = "dev-shell";
  buildInputs = with pkgs; [
    cargo python3 rustfmt
  ];
}
