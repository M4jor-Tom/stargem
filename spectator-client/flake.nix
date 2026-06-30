{
  description = "Stargem spectator client — Bevy 3D viewer";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "clippy" "rustfmt" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "stargem-spectator";
          buildInputs = with pkgs; [
            rustToolchain pkg-config
            alsa-lib udev vulkan-loader
            libxkbcommon wayland
            xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
            protobuf
          ];
          LD_LIBRARY_PATH = "${pkgs.vulkan-loader}/lib:${pkgs.libxkbcommon}/lib:${pkgs.wayland}/lib";
        };
      });
}
