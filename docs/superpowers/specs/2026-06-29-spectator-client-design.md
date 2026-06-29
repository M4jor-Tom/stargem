# Spectator Client — Design

**Status:** Approved (brainstorming)
**Date:** 2026-06-29
**Scope:** New `spectator-client/` submodule + the minimum server-side wiring needed to broadcast live game events to it.

## Goal

A lightweight, read-only Bevy app at `spectator-client/` that subscribes to a live game over gRPC streaming, renders ships in 3D, journals events, and lets the user switch which player the camera follows. The first observable "games" are the existing `server/tests/` scenarios, upgraded to spawn ships at defined positions and run inside a real tick loop that broadcasts to spectators.

## Non-goals

- Writing anything to the server from the spectator. The spectator subscribes and renders; nothing else.
- Final ship art. Placeholder GLTF first; user-supplied models swap in via config later.
- Hooking the production combat loop into spectator broadcast. The broadcast channel is generic and the wiring is one line, but only the scenario-runner uses it for now.
- HUD parity with the real client. Journal + camera follow + name labels are enough for v1.

## Architecture

Three change surfaces:

```
[ server/ submodule ]                    [ root protos/ ]            [ spectator-client/ (new submodule) ]

  CombatTickLoop ──► broadcast::Sender   protos/grpc/                   tonic gRPC client
       │            <GameStateSnapshot>  └─ spectator.proto             │
       │                  │                  SpectatorService           ▼
       │                  ▼                  .SubscribeMatch          bevy::App
  scenarios/ lib    SpectatorService          (stream                   ├─ EventStream system
  (shared by tests  gRPC impl ──────────       GameStateSnapshot)       ├─ ShipFollower system
   and scenario-    fan-out via broadcast ──────────────────────────►   ├─ Journal UI (egui)
   runner binary)   channel                                             └─ GLTF placeholder loader
                          ▲
                          │ subscribes to ticks
  bin/scenario-runner ── drives a real tick loop with scripted
                         spawn positions + inputs + damage events
```

### Data flow per tick

1. `CombatTickLoop` produces `TickSnapshot` (already does this today).
2. New step: encode → `GameStateSnapshot` proto, push to a `tokio::sync::broadcast::Sender`.
3. `SpectatorService.SubscribeMatch` reads from a cloned receiver, streams to each connected spectator over gRPC.
4. Spectator-client decodes the proto, updates a `LiveWorld` resource in Bevy; ECS systems read it to move ship entities and append journal entries.

### Key principle

The spectator is purely a consumer. The server is the only thing that orchestrates "live". Test scenarios become two consumers of one shared scenario definition:
- `#[test]` functions assert on damage/match outcomes.
- A new `scenario-runner` binary spawns a real tick loop, drives the scenario, broadcasts to spectators.

## Server-side changes (`server/` submodule)

### Proto: `protos/grpc/spectator.proto`

```proto
syntax = "proto3";
package stargem.grpc.spectator;
import "quic/combat.proto";
import "quic/common.proto";

service SpectatorService {
  rpc SubscribeMatch(SubscribeRequest) returns (stream stargem.quic.combat.GameStateSnapshot);
  rpc ListMatches(ListMatchesRequest) returns (ListMatchesResponse);
}

message SubscribeRequest { stargem.quic.common.MatchId match_id = 1; }
message ListMatchesRequest {}
message ListMatchesResponse { repeated MatchInfo matches = 1; }
message MatchInfo {
  stargem.quic.common.MatchId match_id = 1;
  string name = 2;
  repeated string player_ids = 3;
}
```

`server/build.rs` already regenerates stubs from `protos/` when `PROTO_SRC` is set; extend it to include `spectator.proto`. Spectator-client's own `build.rs` regenerates the same protos via tonic-build using `protos/` as include root — same pattern as today, no shared Rust crate.

### Broadcast plumbing in `combat/tick.rs`

- Add `broadcast_tx: Option<tokio::sync::broadcast::Sender<GameStateSnapshot>>` on `CombatTickLoop`.
- After producing each `TickSnapshot`, convert it to a `GameStateSnapshot` proto (small mapping fn — `tick_number`, `players` → `ShipState`, `damage_events` → `DamageEvent`) and `broadcast_tx.send(...)`. Lagged subscribers drop frames; that is acceptable.
- New method `pub fn record_damage(&mut self, src, tgt, dtype, raw, mitigated)` that pushes a `DamageEventRecord` into a per-tick buffer; the next snapshot drains the buffer into `damage_events`. This replaces the current hardcoded `Vec::new()`.
- New method `pub fn spawn_at(&mut self, id, stats, position: [f32; 3])` so scenarios can place ships at known coordinates instead of all at origin.

Comment marker:
```rust
// ponytail: scenario-runner is currently the only broadcast publisher; production
// combat loop will pass Some(tx) here when the spectator endpoint goes live.
```

### Spectator gRPC service — `server/src/grpc/spectator.rs`

- `MatchRegistry { matches: HashMap<MatchId, MatchHandle> }` shared via `Arc<Mutex<...>>`.
- `MatchHandle { name, player_ids, broadcast_tx }`.
- `SubscribeMatch` looks up the match, calls `broadcast_tx.subscribe()`, returns a `Stream` that forwards received snapshots.
- `ListMatches` returns the current registry contents.
- Wired into the existing tonic server in `grpc.rs`.

### Scenario refactor — `server/tests/common/scenarios.rs`

Extract each existing test's logic into a reusable `Scenario`:

```rust
pub struct Scenario {
    pub name: &'static str,
    pub spawns: Vec<Spawn>,
    pub script: Vec<ScriptStep>,
}
pub struct Spawn {
    pub player_id: Uuid,
    pub position: [f32; 3],
    pub stats: PlayerShipStats,
}
pub struct ScriptStep { pub at_tick: u64, pub action: ScenarioAction }
pub enum ScenarioAction {
    Input { player: Uuid, input: ShipInput },
    Damage { src: Uuid, tgt: Uuid, dtype: DamageType, raw: f32 },
    EndMatch,
}
```

A synchronous `run_scenario(&Scenario) -> ScenarioOutcome` drives a `CombatTickLoop` without networking, for use in `#[test]` functions. The three existing tests in `ship_destruction_scenario.rs` get rewritten to construct a `Scenario` and assert on `ScenarioOutcome` — same numbers, same assertions, just the construction moves.

Each scenario must have at least two spawn positions defined (per the requirement that scenarios spawn ships at known coordinates). For the existing destruction scenarios, attacker and defender get arbitrary but visually-spaced positions (e.g. `[0,0,0]` and `[50,0,0]`).

### New binary — `server/src/bin/scenario-runner.rs`

CLI:
```
scenario-runner --scenario <name> [--grpc-addr 0.0.0.0:50051] [--tick-rate 30] [--loop]
```

Behavior:
1. Start a gRPC server hosting `SpectatorService`.
2. Construct the named `Scenario`, register it in the `MatchRegistry` with a `broadcast::Sender`.
3. Spawn a `CombatTickLoop` configured with `broadcast_tx = Some(sender)`, populate ships via `spawn_at`, and apply each `ScriptStep` at its `at_tick`:
   - `Input` → set the named player's `ShipInput` for subsequent ticks.
   - `Damage` → call `apply_damage(dtype, raw, target.shield, target.armor, &mult)`; write the resulting shield/armor back onto the target ship state; call `record_damage(src, tgt, dtype, raw, result.mitigated)` so the next snapshot carries the event.
   - `EndMatch` → exit loop, or restart from tick 0 if `--loop`.
4. Hold the server open until the scenario ends; with `--loop`, restart from tick 0 indefinitely so a spectator can connect any time.

## Spectator-client (`spectator-client/` new submodule)

### Layout

```
spectator-client/
├── flake.nix              # rust-overlay + crane, dev shell, package output
├── Cargo.toml
├── build.rs               # tonic-build from ../protos
├── README.md
├── assets/
│   └── ships/
│       └── placeholder.gltf   # primitive cube; swap in real models later
├── config/
│   └── ship_models.toml       # ShipModel → asset path mapping (v2)
└── src/
    ├── main.rs            # bevy::App, plugins, CLI parsing
    ├── grpc.rs            # tonic client task → crossbeam channel
    ├── world.rs           # LiveWorld resource + Journal
    ├── render.rs          # spawn/update ship entities, GLTF loading
    ├── camera.rs          # follow camera + target switching
    └── ui.rs              # bevy_egui: journal panel, player list
```

### Tech choices

- **Bevy** for rendering, ECS, GLTF loading.
- **tonic** as the gRPC client (reuses the proto stubs generated from `protos/`).
- **bevy_egui** for the journal panel and player-list sidebar.
- **crossbeam-channel** to bridge tokio (gRPC stream) to Bevy systems (frame loop). The tonic stream runs in a `tokio::spawn`, sends decoded `GameStateSnapshot`s onto an unbounded `crossbeam_channel::Sender`; a Bevy system drains it every frame into `LiveWorld`.

### CLI

```
spectator-client --grpc-addr 127.0.0.1:50051 [--match <uuid>] [--match-name <ship_destruction>]
```

If neither `--match` nor `--match-name` is given, the client calls `ListMatches` on startup: if exactly one match is available, auto-subscribe; otherwise show an egui chooser. `--match-name` is a convenience that resolves to a `MatchId` via the same `ListMatches` call.

### Player switching

- `Tab` cycles through known players (in `LiveWorld.players` insertion order).
- Number keys `1..9` jump directly to the Nth player.
- egui sidebar lists players (id + last-known shield/armor) with click-to-follow.

### GLTF placeholder strategy

- v1: a single `assets/ships/placeholder.gltf` is loaded for every ship. If the file is missing at startup, fall back to a `Mesh::from(shape::Cube)` mesh built in code so a fresh checkout always renders something. Marked `// ponytail: cube fallback, replace when models land`.
- v2 (user adds models): `config/ship_models.toml` maps `ShipModel` codes to asset paths. Out of scope for first impl.

### Journal

`LiveWorld.journal: VecDeque<JournalEntry>` capped at 200 entries (FIFO). Each incoming `DamageEvent` produces an entry like `"<src> hit <tgt> for 25 kinetic (mitigated 12.5)"`. Future `ModuleActivation` / `MissileLaunch` entries plug in the same way. Rendered as a scrollable right-side egui panel, newest at the top.

## Root repo changes

- `.gitmodules` += entry for `spectator-client/` (sibling of `server/`, `client/`).
- `justfile` additions:
  - `just spectator-run` → runs `cargo run --release -p spectator-client -- --match-name $SCENARIO`.
  - `just scenario-run SCENARIO=ship_destruction` → runs `cargo run --release --bin scenario-runner -- --scenario $SCENARIO --loop` from `server/`.
  - `just spectate SCENARIO=ship_destruction` → runs both concurrently.
- `AGENTS.md`: add a `spectator-client/` row to the submodule table and a one-line description.

## Out of scope (v1)

- Hooking the production game loop into spectator broadcast (deferred; the channel is generic).
- AI bots, autopilot, replay-from-log, recording.
- Real ship models, real HUD, real combat UI.
- Multi-match navigation UI beyond a single chooser.
- Auth on the spectator stream.

## Testing strategy

- **Server side**:
  - Existing `#[test]` functions in `ship_destruction_scenario.rs` keep running and passing, after being rewritten on top of the new `Scenario` + `run_scenario`.
  - One new unit test asserts the `TickSnapshot → GameStateSnapshot` mapping is correct.
  - One new integration test starts the gRPC server in-process, subscribes via tonic client, runs a tiny scenario, asserts that ≥1 `GameStateSnapshot` was received and contains the expected ship ids.
- **Spectator-client**:
  - One unit test for the `GameStateSnapshot → LiveWorld` reducer (proto in, world state out).
  - Smoke-run: `just spectate` opens a window showing two cubes, with the journal populating over time. Confirmed manually for v1.

## Open questions (for the plan, not blockers)

- Bevy version pin? Latest stable.
- Should `scenario-runner` listen on both gRPC and QUIC (in case real spectators come over QUIC later)? No — keep it gRPC only; the production server already runs both.
- Camera mode: orbit (mouse drag) vs strict chase? Start with strict chase + a small fixed offset behind the target; orbit is one ECS system away later.
