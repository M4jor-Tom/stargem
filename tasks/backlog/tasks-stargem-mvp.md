# Task List: Stargem MVP

Generated from `tasks/backlog/prd-stargem-mvp.md` using `ai-dev-tasks/backlog/generate-tasks.md`.

## Task Summary

| Task | Status | Depth |
|------|--------|-------|
| `1.0` Development Environment & Infrastructure | **Complete** | 8/8 sub-tasks |
| `2.0` Protobuf Protocol Definitions | **Complete** | 11/11 sub-tasks |
| `3.0` Backend Combat Systems & QUIC Transport | **Complete** | 13/13 sub-tasks |
| `4.0` Database Schema & Seed Data | **Stub (placeholder files created)** | 2/3 TODO |
| `5.0` Pluggable Authentication Module | **Stub (placeholder created)** | TODO only |
| `6.0` Backend gRPC Services | **Stub (placeholder created)** | TODO only |
| `7.0` Team Deathmatch Game Mode | **Stub (placeholder created)** | TODO only |
| `8.0` Frontend UE5 Integration | **Stub (placeholder created)** | TODO only |

---

## Relevant Files

### Infrastructure
- `flake.nix` — Root Nix flake (Podman, Podman Compose, root-level tooling)
- `docker-compose.yml` — Orchestrates backend + PostgreSQL
- `docker-compose.ci.yml` — CI-specific overrides
- `.github/workflows/ci.yml` — CI pipeline
- `justfile` — Common dev commands
- `server/flake.nix` — Backend Nix flake (Rust toolchain, protoc, sqlx-cli)
- `server/Dockerfile` — Multi-stage backend Docker build
- `client/flake.nix` — Frontend Nix flake (UE5 build dependencies)

### Protocol Definitions
- `protos/grpc/auth.proto` — Auth session messages
- `protos/grpc/shop.proto` — Ship shop/catalog messages
- `protos/grpc/hangar.proto` — Hangar management messages
- `protos/grpc/loadout.proto` — Loadout configuration messages
- `protos/grpc/matchmaking.proto` — Matchmaking queue messages
- `protos/grpc/match_history.proto` — Match result messages
- `protos/quic/common.proto` — Shared QUIC types (Vector3, Quaternion, PlayerId)
- `protos/quic/combat.proto` — Real-time combat messages

### Backend Combat Systems
- `server/Cargo.toml` — Rust dependencies (tonic, quinn, tokio, serde, etc.)
- `server/build.rs` — tonic-build proto compilation
- `server/src/main.rs` — Entry point, server bootstrap
- `server/src/ship/model.rs` — ShipModel, PlayerShip, ShipSize, ShipRole
- `server/src/ship/stats.rs` — Hull stats (shield, armor, energy)
- `server/src/ship/modules.rs` — Passive module definitions + slot layout
- `server/src/ship/active_modules.rs` — Active module definitions + activation flows
- `server/src/ship/weapons.rs` — Weapon types, fire rate, damage, heat
- `server/src/ship/missiles.rs` — Missile types, flight behavior, damage
- `server/src/combat/tick.rs` — 60Hz fixed tick loop
- `server/src/combat/physics.rs` — Arcade physics (speed cap, drag, instant acceleration)
- `server/src/combat/damage.rs` — Damage type system + multipliers
- `server/src/transport/quic.rs` — QUIC server using Quinn

---

## Instructions for Completing Tasks

**IMPORTANT:** As you complete each task, check it off by changing `- [ ]` to `- [x]`. Update the file after completing each sub-task.

---

## Tasks

### [x] 1.0 Development Environment & Infrastructure

- [x] 1.1 Create `flake.nix` (root) with `devShells.default` providing `podman`, `podman-compose`, `just`. Must NOT include Rust or UE5 dependencies.
- [x] 1.2 Create `server/flake.nix` with `devShells.default` and `packages.default` providing `rustc`, `cargo`, `protoc`, `openssl`, and system build deps.
- [x] 1.3 Create `client/flake.nix` with `devShells.default` providing Unreal Engine 5 build dependencies (placeholder structure — exact UE5 deps depend on the environment).
- [x] 1.4 Create `docker-compose.yml` orchestrating:
      - `backend` service (build from `server/Dockerfile`, expose gRPC and QUIC ports)
      - `postgres` service (official PostgreSQL image, persistent volume)
      - `adminer` (optional, for dev DB inspection)
- [x] 1.5 Create `server/Dockerfile` with multi-stage build:
      - Stage 1: `cargo-chef` for dependency caching
      - Stage 2: `cargo build --release`
      - Stage 3: runtime image with the compiled binary
- [x] 1.6 Create `docker-compose.ci.yml` with CI overrides (e.g., no Adminer, different resource limits)
- [x] 1.7 Create `.github/workflows/ci.yml` with jobs:
      - `backend`: `cargo test`, `cargo clippy`, `cargo fmt --check`
      - `frontend`: Verify UE5 project builds (placeholder check)
      - Enforce proto drift detection: verify Rust structs match `.proto` files
- [x] 1.8 Create `justfile` with common dev commands (`just up`, `just down`, `just test`, `just lint`, `just build`)

### [x] 2.0 Protobuf Protocol Definitions

- [x] 2.1 Create directory structure: `protos/grpc/`, `protos/quic/`
- [x] 2.2 Create `protos/grpc/auth.proto` — Auth service + messages:
      - `LoginRequest` (steam auth ticket), `LoginResponse` (session token, user ID)
      - `ValidateSession` RPC
- [x] 2.3 Create `protos/grpc/shop.proto` — Shop service + messages:
      - `ShipModel` message (id, name, size, role, price)
      - `ListShips` RPC, `BuyShip` RPC
- [x] 2.4 Create `protos/grpc/hangar.proto` — Hangar service + messages:
      - `HangarSlot` message (slot index, ship model ID, loadout reference)
      - `ListHangar` RPC, `AssignShipToSlot` RPC
- [x] 2.5 Create `protos/grpc/loadout.proto` — Loadout service + messages:
      - `EquipPassiveModule`, `EquipActiveModule`, `EquipWeapon`, `EquipMissile` RPCs
      - `Loadout` message (passive slots, active slots, weapon, missile)
- [x] 2.6 Create `protos/grpc/matchmaking.proto` — Matchmaking service + messages:
      - `QueueForMatch` RPC, `QueueStatus` RPC, `LeaveQueue` RPC
      - `QueueState` message (position, estimated wait, status)
- [x] 2.7 Create `protos/grpc/match_history.proto` — Match history service + messages:
      - `MatchRecord` message (match ID, timestamp, kills, deaths, damage dealt, damage taken, result)
      - `GetHistory` RPC
- [x] 2.8 Create `protos/quic/common.proto` — Shared types:
      - `Vector3` (x, y, z), `Quaternion` (x, y, z, w)
      - `PlayerId`, `ShipId`, `MatchId`
- [x] 2.9 Create `protos/quic/combat.proto` — Combat messages with version field:
      - `PlayerInput` (throttle, yaw, pitch, roll, shoot, activate_module, timestamp)
      - `ShipState` (position, velocity, rotation, shield_hp, armor_hp, energy, heat_level)
      - `DamageEvent` (source, target, damage_type, raw_amount, mitigated_amount)
      - `ModuleActivation` (module_id, activation_type, target_id)
      - `WeaponOverheat` (weapon_id, heat_percentage, overheated_bool)
      - `MissileLaunch` (missile_id, launcher_id, target, position, velocity)
      - `GameStateSnapshot` (tick_number, players states, damage_events, missile_states)
- [x] 2.10 Version all QUIC messages via a `uint32 version` field (per FR-5.5)
- [x] 2.11 Verify all `.proto` files compile with `protoc` (add a CI script under `protos/verify.sh`)

### [x] 3.0 Backend Combat Systems & QUIC Transport

- [x] 3.1 Create `server/Cargo.toml` with dependencies: `tokio`, `tonic`, `tonic-build`, `quinn`, `prost`, `serde`, `clap` (for CLI config), `tracing`
- [x] 3.2 Create `server/build.rs` — compile all `.proto` files via `tonic-build`
- [x] 3.3 Create `server/src/main.rs` — CLI arg parsing (tick rate, ports, DB URL), server bootstrap
- [x] 3.4 Create `server/src/ship/model.rs` — Ship domain structs:
      - `ShipModel` (immutable store entity: id, name, size, role, price, base_stats, passive_slots layout)
      - `PlayerShip` (mutable user-owned entity: id, user_id, ship_model_id, loadout reference)
      - `ShipSize` enum (Frigate, Fighter, Interceptor)
      - `ShipRole` enum with special module per role (from `onthology.md`)
      - `HullStats` (base shield, armor, energy, speed, agility)
- [x] 3.5 Create `server/src/ship/stats.rs` — `PlayerShipStats` with computed stats:
      - Base stats from linked `ShipModel` + modifiers from equipped `PassiveCombatModules`
      - Shield/Armor/Energy current + max values
- [x] 3.6 Create `server/src/ship/modules.rs` — Passive module system:
      - `PassiveModuleType` enum (Shield, Armor, Capacitor, Motor, Computer)
      - `PassiveModule` with stat modifiers (e.g., `+X% shield_hp`, `-Y% armor_hp + speed bonus`)
      - Fixed slot layout per ship model (from seed config, not hardcoded)
- [x] 3.7 Create `server/src/ship/active_modules.rs` — Active module system:
      - `ActivationFlow` enum (OneShot, Ongoing)
      - `ActiveModule` (energy_cost, cooldown, effect definition)
      - Slot limit: exactly 4 active modules per ship + 1 special role module
- [x] 3.8 Create `server/src/ship/weapons.rs` — Weapon system:
      - `WeaponSize` enum keyed to ship size
      - Multiple weapon types per size class (different fire rate, damage per shot, heat_per_shot)
      - Overheat mechanic: heat accumulates on fire, overheated forces longer cooldown
- [x] 3.9 Create `server/src/ship/missiles.rs` — Missile system:
      - Missile flight behavior (speed, turn rate, lifetime, lock-on)
      - Missile damage on impact
- [x] 3.10 Create `server/src/combat/damage.rs` — Damage system:
      - `DamageType` enum (Electromagnetic, Kinetic, Thermic)
      - Configurable multiplier matrix: `[type vs shield]`, `[type vs armor]`
      - Shield absorbs first; overflow bleeds to armor; ship destroyed at 0 armor
- [x] 3.11 Create `server/src/combat/physics.rs` — Arcade physics:
      - Speed cap, drag coefficient, instant acceleration
      - Yaw/pitch/roll input maps to rotation and thrust
- [x] 3.12 Create `server/src/combat/tick.rs` — Combat tick loop:
      - Fixed rate (default 60 Hz, configurable via `--tick-rate` CLI flag)
      - Each tick: gather inputs, simulate physics, process weapons/missiles/modules, apply damage, broadcast state
- [x] 3.13 Create `server/src/transport/quic.rs` — QUIC server:
      - Initialize Quinn endpoint with server certificate
      - Accept client connections, maintain per-client streams
      - Deserialize `PlayerInput` from QUIC streams, serialize `GameStateSnapshot` to broadcasts
      - Wire tick loop output into QUIC broadcast channel

### [x] 4.0 Database Schema & Seed Data

> **TODO:** This task is stubbed. Full implementation deferred. Placeholder files created.

- [x] 4.1 TODO: Create raw SQL files (`server/sql/schema.sql`, `server/sql/seed.sql`) for: users, credit_balance, ship_models (seed catalog), player_ships, loadout_configs, hangar_assignments — *placeholder files created with TODO comments*
- [x] 4.2 TODO: Create seed config file for ship models with fixed passive module slots per model (YAML/TOML under `server/config/ships/`) — *placeholder README created*
- [ ] 4.3 TODO: Load `schema.sql` and `seed.sql` on backend startup via `sqlx::raw_sql` — *not yet implemented*

### [x] 5.0 Pluggable Authentication Module

> **TODO:** This task is stubbed. Full implementation deferred. Placeholder file at `server/src/auth.rs`.

- [ ] 5.1 TODO: Implement `AuthProvider` trait as defined in PRD (`authenticate`, `validate_session`)
- [ ] 5.2 TODO: Implement `SteamAuthProvider` (reads SteamAuthConfig from env)
- [ ] 5.3 TODO: Implement `MockAuthProvider` for testing
- [ ] 5.4 TODO: Wire auth into gRPC auth service handler

### [x] 6.0 Backend gRPC Services

> **TODO:** This task is stubbed. Full implementation deferred. Placeholder file at `server/src/grpc.rs`.

- [ ] 6.1 TODO: Implement gRPC server with Tonic, serving all services from `protos/grpc/`
- [ ] 6.2 TODO: Implement Shop service (list ships, buy ship)
- [ ] 6.3 TODO: Implement Hangar service (list, assign)
- [ ] 6.4 TODO: Implement Loadout service (equip modules/weapons/missiles)
- [ ] 6.5 TODO: Implement Matchmaking service (queue, status, leave)
- [ ] 6.6 TODO: Implement Match History service (get history)

### [x] 7.0 Team Deathmatch Game Mode

> **TODO:** This task is stubbed. Full implementation deferred. Placeholder file at `server/src/game_mode.rs`.

- [ ] 7.1 TODO: Implement team assignment (2 teams, 4v4 to 8v8)
- [ ] 7.2 TODO: Implement spawn logic (pick ship from hangar, place at spawn point)
- [ ] 7.3 TODO: Implement respawn with configurable delay, allow ship change
- [ ] 7.4 TODO: Implement scoring (kill = 1 point, score limit wins)
- [ ] 7.5 TODO: Implement time limit (match ends when time expires, higher score wins)
- [ ] 7.6 TODO: Record basic match stats (kills, deaths, damage dealt, damage taken)

### [x] 8.0 Frontend UE5 Integration

> **TODO:** This task is stubbed. Full implementation deferred. Placeholder file at `server/src/frontend.rs`.

- [ ] 8.1 TODO: Integrate Steamworks SDK for Steam authentication on startup
- [ ] 8.2 TODO: Implement gRPC client plugin for cold-time operations
- [ ] 8.3 TODO: Implement QUIC client plugin for combat-time operations
- [ ] 8.4 TODO: Implement HUD (shield, armor, energy bars, weapon heat, module cooldowns)
- [ ] 8.5 TODO: Implement input sending at tick rate + state interpolation for smooth rendering

---

### Notes

- **Submodules are NOT checked out by default.** Run `git submodule update --init server` and/or `git submodule update --init client` before working on those directories.
- All changes inside `server/` and `client/` must be committed inside those submodule repos, not from the root.
- `.proto` files live at the project root under `protos/`, NOT inside either submodule, so both backend and frontend compile from the same source of truth.
- The tick rate CLI flag should use `clap` with a default of 60.
- Damage type multipliers should be loaded from a config file (e.g., `server/config/damage_multipliers.toml`) so they are tunable without recompilation.
- `protoc` must be available in the build environment (provided by `server/flake.nix`).
