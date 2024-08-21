{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  # nativeBuildInputs is usually what you want -- tools you need to run
  buildInputs = [pkgs.mpv];

  nativeBuildInputs = with pkgs; [
    gnumake

    # go development
    go
    gopls
    go-tools
    delve
    golangci-lint
  ];
}
