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
      default = pkgs.callPackage ./package.nix {};
    });
    devShells = forAllSystems (pkgs: {
      default = import ./shell.nix {inherit pkgs;};
    });
  };
}
