with (import <nixpkgs> {});
mkShell {
  name = "dev-shell";
  buildInputs = [
    cargo python38 python38Packages.pytest
    python38Packages.requests rustfmt
  ];
}
