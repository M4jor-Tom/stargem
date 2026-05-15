# stargem

**Monorepo shell** that assembles two submodules plus AI workflow prompts.

## Repository structure

| Path | Description | Source |
|---|---|---|
| `server/` | Rust backend | `git@github.com:M4jor-Tom/stargem_backend.rs.git` |
| `client/` | C++ (Unreal) frontend | `git@github.com:M4jor-Tom/stargem_frontend.cpp.git` |
| `ai-dev-tasks/` | AI-assisted dev workflow prompts | `git@github.com:snarktank/ai-dev-tasks.git` |
| `protos/` | Proto definitions (submodule) | `git@github.com:M4jor-Tom/stargem_protos.git` |
| `onthology.md` | Game design doc (entities, damage types, ship roles) | — |

## Submodule workflow

The three code submodules (`client/`, `server/`, `protos/`) are **not checked out** in the default clone. You must explicitly init them:

```bash
git submodule update --init --recursive
```

Individual submodule:
```bash
git submodule update --init server          # Rust backend only
git submodule update --init client          # C++/Unreal frontend only
git submodule update --init protos  # Proto definitions only
```

Changes to submodule source should be committed **inside** the submodule repo, not in this root repo. The root only tracks submodule pointer commits.

## Proto sharing

| Principle | Description |
|---|---|
| **Source of truth** | The `protos/` submodule pins the canonical proto repository. |
| **Generated code** | Each subproject commits its generated proto stubs (`server/src/proto_gen/`). |
| **CI drift check** | Regenerating from `protos/` must produce no diff — ensures stubs are fresh. |
| **Local regeneration** | Run `just proto` to regenerate stubs after editing `protos/`. |
| **Update workflow** | Edit in `protos/`, run `just proto`, commit+push each submodule. |

## Game design reference

`onthology.md` is the single source of truth for game mechanics: damage type interactions (Electromagnetic/Kinetic/Thermic), ship sizes (Frigate/Fighter/Interceptor) and roles, special modules, and combat module system.

## AI dev tasks

`ai-dev-tasks/` contains markdown prompts (`create-prd.md`, `generate-tasks.md`) for structured AI-assisted feature development. It is a git submodule of `github.com:snarktank/ai-dev-tasks.git`.

## Commands

| Command | Description |
|---|---|
| `just docker-image` | Build Docker image via Nix (`nix build ./server#dockerImage && docker load < result`) |
| `just build` | Build backend release via crane (`nix build ./server`) |
| `just up` | Start services via podman-compose |
| `just up-docker` | Build Docker image then start services |
| `just test` | Run backend tests |
| `just lint` | Run clippy |
| `just fmt` | Format Rust code |
| `just proto` | Regenerate proto stubs from `protos/` |
| `just proto-check` | Check committed stubs match `protos/` |
