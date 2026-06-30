{
  description = "Stargem spectator client — Bevy 3D viewer";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "stargem-spectator";
          nativeBuildInputs = with pkgs; [
            rustPackages.rustc rustPackages.cargo
            rustPackages.clippy rustPackages.rustfmt
            pkg-config
          ];
          buildInputs = with pkgs; [
            alsa-lib udev vulkan-loader
            libxkbcommon wayland
            libx11 libxcursor libxi libxrandr
            protobuf
          ];
          LD_LIBRARY_PATH = "${pkgs.vulkan-loader}/lib:${pkgs.libxkbcommon}/lib:${pkgs.wayland}/lib";
          PROTOC = "${pkgs.protobuf}/bin/protoc";
          PROTOC_INCLUDE = "${pkgs.protobuf}/include";
        };
      });
}