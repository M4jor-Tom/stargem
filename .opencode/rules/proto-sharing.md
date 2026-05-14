# Proto sharing across monorepo + submodules

- Source of truth: dedicated proto repository
- Root monorepo pins it as a submodule at `protos/`
- Each subproject references it as a Nix flake input
- Root flake overrides the input to `path:./protos` for local dev
- Rust build scripts read `$PROTO_SRC` env var (never hardcoded paths)
- To update protos: edit in protos/, commit+push submodule, update flake lock
