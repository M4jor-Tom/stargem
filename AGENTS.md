# stargem

**Monorepo shell** that assembles two submodules plus AI workflow prompts.

## Repository structure

| Path | Description | Source |
|---|---|---|
| `server/` | Rust backend | `git@github.com:M4jor-Tom/stargem_backend.rs.git` |
| `client/` | C++ (Unreal) frontend | `git@github.com:M4jor-Tom/stargem_frontend.cpp.git` |
| `ai-dev-tasks/` | AI-assisted dev workflow prompts | `git@github.com:snarktank/ai-dev-tasks.git` |
| `onthology.md` | Game design doc (entities, damage types, ship roles) | — |

## Submodule workflow

The two code submodules (`client/`, `server/`) are **not checked out** in the default clone. You must explicitly init them:

```bash
git submodule update --init --recursive
```

Individual submodule:
```bash
git submodule update --init server   # Rust backend only
git submodule update --init client   # C++/Unreal frontend only
```

Changes to submodule source should be committed **inside** the submodule repo, not in this root repo. The root only tracks submodule pointer commits.

## Game design reference

`onthology.md` is the single source of truth for game mechanics: damage type interactions (Electromagnetic/Kinetic/Thermic), ship sizes (Frigate/Fighter/Interceptor) and roles, special modules, and combat module system.

## AI dev tasks

`ai-dev-tasks/` contains markdown prompts (`create-prd.md`, `generate-tasks.md`) for structured AI-assisted feature development. It is a git submodule of `github.com:snarktank/ai-dev-tasks.git`.

## Commands

No root-level build/test/lint commands exist — run those inside the submodule directories.
