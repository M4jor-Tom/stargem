# stargem

Ontology-driven multiplayer space combat. `ontology/` is the source of truth; everything else
derives from it (see `ontology/domain.md`).

## Stack (reboot, 2026-09-06)

| crate | role | why |
|-------|------|-----|
| `ontology` | typed model + catalog instances + validator | source of truth |
| `sim` | arcade flight + collision (parry3d), headless | shared by client and the future authoritative server |
| `client` | raylib (via `raylib` crate) renderer, input, camera | lightest 3D engine with glTF and collision, zero deps, 14 platforms |

Rejected for the client: Bevy (80+ MiB binaries, slow builds), macroquad (3D is minimal and
under-documented), Godot/Unity/Unreal (editor-sized), three.js (needs a browser).

## Run

```
nix develop
cargo test
cargo run -p client
```

Ship art: `docs/3D/star_conflict_ships.gltf` (git submodule, `git submodule update --init`).
