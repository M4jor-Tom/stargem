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
          nativeBuildInputs = with pkgs; [
            rustToolchain pkg-config protobuf
          ];
          buildInputs = with pkgs; [
            alsa-lib udev vulkan-loader
            libxkbcommon wayland
            libx11 libxcursor libxi libxrandr
          ];
          LD_LIBRARY_PATH = "${pkgs.vulkan-loader}/lib:${pkgs.libxkbcommon}/lib:${pkgs.wayland}/lib";
        };

        apps.default = flake-utils.lib.mkApp {
          drv = pkgs.writeShellApplication {
            name = "spectator-client";
            runtimeInputs = with pkgs; [
              rustToolchain pkg-config protobuf
              alsa-lib udev vulkan-loader
              libxkbcommon wayland
              libx11 libxcursor libxi libxrandr
            ];
            text = ''
              export PROTO_SRC="${self}/protos"
              export PROTOC="${pkgs.protobuf}/bin/protoc"
              export PROTOC_INCLUDE="${pkgs.protobuf}/include"
              export LD_LIBRARY_PATH="${pkgs.vulkan-loader}/lib:${pkgs.libxkbcommon}/lib:${pkgs.wayland}/lib"
              cargo run --manifest-path "${self}/spectator-client/Cargo.toml" --release -- "$@"
            '';
          };
        };
      });
}