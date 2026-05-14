{
  description = "Stargem monorepo — root-level developer tooling";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "clippy" "rustfmt" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        protoSrc = builtins.path {
          path = ./stargem_protos;
          name = "stargem-protos";
        };

        serverSrc = builtins.path {
          path = ./server;
          name = "stargem-server";
        };

        commonArgs = {
          PROTO_SRC = "${protoSrc}";
          nativeBuildInputs = with pkgs; [ protobuf pkg-config ];
          buildInputs = with pkgs; [ openssl clang ];
        };

        cargoArtifacts = craneLib.buildDepsOnly (commonArgs // {
          src = serverSrc;
        });

        backend = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          src = serverSrc;
        });
      in
      {
        devShells.default = pkgs.mkShell {
          name = "stargem-root";
          buildInputs = with pkgs; [ podman podman-compose just ];
        };

        packages.default = backend;
        packages.dockerImage = pkgs.dockerTools.buildImage {
          name = "stargem-backend";
          tag = "latest";
          copyToRoot = pkgs.buildEnv {
            name = "image-root";
            paths = [
              backend
              pkgs.cacert
              (pkgs.runCommand "config" {} ''
                mkdir -p $out/app/config
                cp ${./server/config/damage_multipliers.toml} $out/app/config/damage_multipliers.toml
              '')
            ];
          };
          config = {
            Env = [ "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt" ];
            ExposedPorts = { "50051/tcp" = {}; "50052/udp" = {}; };
            Entrypoint = [ "${backend}/bin/stargem-backend" ];
            WorkingDir = "/app";
          };
        };
      });
}
