{
  description = "stargem — ontology-driven space combat";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAll = f: nixpkgs.lib.genAttrs systems (s: f nixpkgs.legacyPackages.${s});
    in {
      devShells = forAll (pkgs: {
        default = pkgs.mkShell {
          packages = with pkgs; [
            cargo rustc rust-analyzer clippy rustfmt
            gcc cmake clang pkg-config curl
            # raylib (built from source by the raylib crate)
            glfw libGL wayland libxkbcommon wayland-protocols
            libx11 libxrandr libxinerama libxcursor libxi
            alsa-lib
          ];
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [ libGL wayland libxkbcommon libx11 ]);
        };
      });
    };
}
