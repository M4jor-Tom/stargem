# stargem

**Monorepo shell** that assembles git submodules.

## Submodules

Not checked out in a default clone — you **must** init them explicitly:

| Path | Source |
|---|---|
| `server/` | `github:M4jor-Tom/stargem_backend.rs.git` |
| `client/` | `github:M4jor-Tom/stargem_frontend.cpp.git` |
| `.opencode/rules/` | `github:M4jor-Tom/rules.md.git` |
| `ai-dev-tasks/` | `github:snarktank/ai-dev-tasks.git` |

```
git submodule update --init --recursive
```

Commit changes **inside** the submodule repo, not in the root.

## Commands

Everything runs via `nix develop ./server -c <cmd>` — don't call `cargo` directly.

| Command | What it does |
|---|---|
| `just build` | Release build (`nix build ./server`) |
| `just build-dev` | Debug build (`cargo build`) |
| `just test` | `cargo test` |
| `just lint` | `cargo clippy -- -D warnings` |
| `just fmt` | `cargo fmt` |
| `just fmt-check` | `cargo fmt --check` |
| `just proto` | Regenerate proto stubs from `protos/` |
| `just proto-check` | Regenerate + fail if `server/` has unstaged changes |
| `just docker-image` | Build Docker image via Nix + `skopeo copy` |
| `just up` / `just down` | `podman-compose up` / down |
| `just up-docker` | Build image + start services |
| `just up-dev` / `just down-dev` | with dev profile (adminer) |
| `just logs` | `podman-compose logs -f` |

## CI order (`.github/workflows/ci.yml`)

1. `cargo fmt --check`
2. `cargo clippy -- -D warnings`
3. `cargo test`
4. Proto drift check (rebuild from `protos/`, `git -C server diff --exit-code`)
5. Build Docker image

## Proto stubs

`server/build.rs` generates stubs **only when `PROTO_SRC` is set**. Without it, `cargo build` uses the committed stubs in `server/src/proto_gen/`. See `.opencode/rules/proto-sharing.md`.

## Architecture

- **Two transports**: gRPC (tonic, TCP/50051) and QUIC (quinn, UDP/50052).
- **Proto stubs**: tonic-build generates gRPC server stubs + QUIC message structs; committed in `server/src/proto_gen/`.
- **Docker**: `pkgs.dockerTools.buildImage` in `server/image.nix`, no Dockerfile. Use `skopeo copy --policy containers/policy.json` to load. See `.opencode/rules/docker-images.md`.
- **Database**: PostgreSQL via `docker-compose.yml`. Schema in `server/sql/`.
- **Config**: TOML under `server/config/` (e.g., `damage_multipliers.toml`).
- **Game design**: `onthology.md` — damage types, ship roles, special modules.
- **AI dev tasks**: `ai-dev-tasks/` has structured prompt templates (`create-prd.md`, `generate-tasks.md`).
- **Rust builds**: Use `github:ipetkov/crane` per `.opencode/rules/rust-builds.md`.

## Loaded instructions

`.opencode/rules/*.md` (a submodule) provides repo-specific guidance on Rust builds, Docker images, and proto sharing — they are loaded automatically by opencode.json.
