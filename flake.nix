{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = {nixpkgs, rust-overlay, ...}: let
    forAllSystems = function:
      nixpkgs.lib.genAttrs [
        "x86_64-darwin"
        "x86_64-linux"
        "aarch64-darwin"
        "aarch64-linux"
      ] (system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };
      in function pkgs);
  in {
    packages = forAllSystems (pkgs: {
      default = pkgs.rustPlatform.buildRustPackage {
        pname = "radioboat";
        version = "0.4.0";

        src = pkgs.nix-gitignore.gitignoreSource [] ./.;

        cargoLock.lockFile = ./Cargo.lock;

        buildInputs = [pkgs.mpv];

        nativeBuildInputs = [pkgs.makeWrapper pkgs.installShellFiles];

        preFixup = ''
          wrapProgram $out/bin/radioboat --prefix PATH ":" "${pkgs.lib.makeBinPath [pkgs.mpv]}";
        '';

        postInstall = ''
          installShellCompletion --cmd radioboat \
            --bash <($out/bin/radioboat completion bash) \
            --fish <($out/bin/radioboat completion fish) \
            --zsh <($out/bin/radioboat completion zsh)
        '';

        passthru = {
          updateScript = pkgs.nix-update-script {};
          tests.version = pkgs.testers.testVersion {
            package = pkgs.radioboat;
            command = "radioboat --version";
          };
        };

        meta = with pkgs.lib; {
          description = "Radioboat is a terminal web radio client, built with simplicity in mind.";
          mainProgram = "radioboat";
          homepage = "https://github.com/slashformotion/radioboat";
          license = licenses.asl20;
          platforms = platforms.linux ++ platforms.darwin;
        };
      };
    });
    devShells = forAllSystems (pkgs: {
      default = pkgs.mkShell {
        buildInputs = with pkgs; [
          mpv
          pkg-config
        ];

        nativeBuildInputs = with pkgs; [
          gnumake
          just
          cargo-zigbuild
          (rust-bin.stable.latest.default.override {
            extensions = ["rust-src" "rust-analyzer"];
            targets = ["aarch64-unknown-linux-gnu" "x86_64-apple-darwin" "aarch64-apple-darwin"];
          })
          clippy
          rustfmt
        ];
      };
    });
  };
}
