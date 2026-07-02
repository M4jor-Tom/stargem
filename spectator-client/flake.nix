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
        nativeLibs = with pkgs; [
          alsa-lib udev vulkan-loader
          libxkbcommon wayland
          libx11 libxcursor libxi libxrandr
        ];
        pkgConfigPath = pkgs.lib.makeSearchPath "lib/pkgconfig"
          (map (p: p.dev or p) nativeLibs);
      in
      {
        devShells.default = pkgs.mkShell {
          name = "stargem-spectator";
          nativeBuildInputs = with pkgs; [
            rustToolchain pkg-config protobuf
          ];
          buildInputs = nativeLibs;
          LD_LIBRARY_PATH = "${pkgs.vulkan-loader}/lib:${pkgs.libxkbcommon}/lib:${pkgs.wayland}/lib";
        };

        apps.default = flake-utils.lib.mkApp {
          drv = pkgs.writeShellApplication {
            name = "spectator-client";
            runtimeInputs = [ rustToolchain pkgs.stdenv.cc pkgs.pkg-config pkgs.protobuf ] ++ nativeLibs;
            text = ''
              export PROTO_SRC="${self}/../protos"
              export PROTOC="${pkgs.protobuf}/bin/protoc"
              export PROTOC_INCLUDE="${pkgs.protobuf}/include"
              export PKG_CONFIG_PATH="${pkgConfigPath}"
              export LD_LIBRARY_PATH="${pkgs.vulkan-loader}/lib:${pkgs.libxkbcommon}/lib:${pkgs.wayland}/lib"
              export CARGO_TARGET_DIR="''${CARGO_TARGET_DIR:-''${XDG_CACHE_HOME:-$HOME/.cache}/stargem-spectator/target}"
              cargo run --manifest-path "${self}/Cargo.toml" --release -- "$@"
            '';
          };
        };
      });
}