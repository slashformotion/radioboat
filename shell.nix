{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    mpv
    pkg-config
  ];

  nativeBuildInputs = with pkgs; [
    gnumake
    just

    # go development
    go
    gopls
    go-tools
    delve
    golangci-lint

    # rust development (using stable from overlay)
    (rust-bin.stable.latest.default.override {
      extensions = ["rust-src" "rust-analyzer"];
    })
    clippy
    rustfmt
  ];
}
