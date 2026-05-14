# PRD: Stargem MVP — Full-Stack Networking & Business Definition

## Introduction/Overview

Stargem is a multiplayer space combat game. This MVP establishes the foundational full-stack architecture — a Rust backend with PostgreSQL communicating with an Unreal Engine 5 frontend — focused on **PvP Team Deathmatch**. The primary goal is to wire all layers together (infrastructure, networking protocols, auth, combat systems) so modules can connect end-to-end, even in minimal form.

The MVP delivers one playable game mode, a ship hangar with loadout configuration, and the protocol contracts (gRPC + QUIC, documented via .proto files) that every future feature will build on.

## Goals

1. Establish a **reproducible development environment** via Docker Compose and Nix flakes.
2. Deliver a **playable PvP Team Deathmatch** match with 2+ players, each controlling a ship from their hangar.
3. Implement the **damage type system** (Electromagnetic / Kinetic / Thermic) with correct shield-armor interactions.
4. Provide **ship loadout configuration** (passive modules, active modules, weapons, missiles, ship selection) stored in PostgreSQL.
5. Define and document **all domain-level network messages** as .proto files at the project root.
6. Implement **two separate transport protocols**: gRPC for cold-time operations, QUIC for combat-time operations.
7. Build auth as a **pluggable module** — start with Steam, design for Keycloak migration.
8. Set up **GitHub CI** for automated testing of backend and frontend builds.

## User Stories

1. **As a player**, I want to launch the game, authenticate via Steam, and reach the main menu.
2. **As a player**, I want to browse the ship arborescence and buy a ship model.
3. **As a player**, I want to equip passive modules (within my ship model's fixed slot layout), active modules (up to 4), a weapon, and missiles on my ships.
4. **As a player**, I want to assign up to 4 ships to my hangar.
5. **As a player**, I want to queue for a Team Deathmatch, pick a ship from my hangar, and fight.
6. **As a player**, I want to shoot weapons, fire missiles, and activate modules during combat.
7. **As a player**, I want my shield and armor to properly resist incoming damage based on damage type.
8. **As a player**, I want to overheat my weapon if I fire too long, then wait for cooldown.
9. **As a player**, I want to activate my role's special module with a dedicated key (separate from my 4 active combat modules).
10. **As a developer**, I want to run `docker compose up` and have the full stack running locally.
11. **As a developer**, I want to read .proto files at the project root to understand every domain message.
12. **As a developer**, I want to run CI checks on every PR to verify backend and frontend compile and tests pass.

## Functional Requirements

### FR-1: Development Environment & Infrastructure
- FR-1.1: The project root must provide a `docker-compose.yml` that orchestrates the Rust backend, PostgreSQL, and any required services.
- FR-1.2: The project root must contain a `flake.nix` that provides a reproducible dev shell with Podman (for running docker-compose) and any other root-level tooling. This flake must NOT include Rust or Unreal Engine dependencies — those are handled by submodule-level flakes.
- FR-1.3: The backend directory must contain a `flake.nix` that provides a reproducible dev shell with all Rust toolchain dependencies.
- FR-1.4: The frontend directory must contain a `flake.nix` that provides a reproducible dev shell with Unreal Engine 5 build dependencies.
- FR-1.5: A `docker-compose.ci.yml` (or equivalent) must be provided for overriding `docker-compose.yml` in CI environments.
- FR-1.6: A GitHub Actions workflow must compile the Rust backend, run its tests, and verify the frontend builds on each PR.

### FR-2: Authentication (Pluggable Module)
- FR-2.1: The auth system must be an abstract trait/interface with a pluggable implementation.
- FR-2.2: The MVP must implement a `SteamAuthProvider` that authenticates users via the Steam API (using the Steamworks SDK on the client, Steam Web API on the backend).
- FR-2.3: The auth interface must be designed so that a `KeycloakAuthProvider` (OIDC-based) can be added later without changing core game logic.
- FR-2.4: Auth messages (login tokens, session validation) must be defined in `.proto` files alongside other gRPC services, under `protos/grpc/auth.proto`.

### FR-3: Database & Persistence (PostgreSQL)
- FR-3.1: PostgreSQL must store: users (linked to Steam ID), credit balance, owned ship models, loadout configurations, hangar assignments.
- FR-3.2: The backend must be able to integrate migrations (e.g., `sqlx migrate`) to manage schema changes, but not implement it yet.
- FR-3.3: The database schema must support the full ship/loadout model from `onthology.md` (ship sizes, roles, modules, weapons, missiles).
- FR-3.4: Each ship model must define its fixed passive module slots (e.g., 1 Shield, 2 Armor, 1 Capacitor, 0 Motor, 1 Computer) in a seed/config file, not in application code.

### FR-4: Cold-Time Networking (gRPC)
- FR-4.1: All non-combat operations must use gRPC transport.
- FR-4.2: gRPC services must cover: auth session management, shop/browse ships, buy ship, equip modules/weapons/missiles, hangar management, matchmaking queue, and match result history.
- FR-4.3: Each gRPC service and message must be defined in a `.proto` file under `protos/grpc/`.

### FR-5: Combat-Time Networking (QUIC)
- FR-5.1: All real-time combat operations must use QUIC as the transport protocol.
- FR-5.2: Combat messages must include: player input (throttle, yaw/pitch/roll, shoot, activate module), ship state updates (position, velocity, shield/armor/energy), damage events, module activations, weapon overheat state, missile launches.
- FR-5.3: The server must be authoritative — it validates all inputs, simulates combat, and broadcasts state.
- FR-5.4: Each QUIC message must be defined in a `.proto` file under `protos/quic/`.
- FR-5.5: Messages must be versioned to allow future protocol evolution.

### FR-6: Ship & Combat Systems (Backend)
- FR-6.1: The backend must implement the ship domain model from `onthology.md`, distinguishing between:
      - `ShipModel` (immutable store entities, each with a `price`, base stats (shield/armor/energy), a `ShipSize`, and a `ShipRole`)
      - `PlayerShip` (mutable entities owned by users, each referencing a `ShipModel`, with an equipable loadout)
      - `ShipSize` enum (Frigate, Fighter, Interceptor) and `ShipRole` enum with their associated `SpecialCombatModule` (per `onthology.md`)
- FR-6.2: The damage type system (Electromagnetic > Shield, Kinetic > Armor, Thermic balanced) must be implemented with configurable multipliers.
- FR-6.3: `PassiveCombatModules` must modify ship stats (e.g., "+X% shield HP", "-Y% armor HP + speed bonus").
- FR-6.4: `ActiveCombatModules` must support two activation flows: "One Shot" (instant effect, energy cost, cooldown) and "Ongoing" (toggle, continuous energy drain, cooldown between activations).
- FR-6.5: Weapons must differentiate across multiple types per size class (e.g., different fire rates, damage per shot, heat generation). Weapons are not limited to one generic weapon per size.
- FR-6.6: Weapons must have an overheat mechanic: sustained fire increases heat, overheating forces a longer cooldown period.
- FR-6.7: Ships must be able to fire missiles with distinct flight and damage behavior.
- FR-6.8: A ship is destroyed when armor reaches zero.
- FR-6.9: The combat simulation must run at a fixed tick rate — 60 Hz by default, configurable at server startup via a CLI flag or config file.
- FR-6.10: Ship movement must use arcade-style physics (speed cap, drag, instant acceleration) rather than Newtonian simulation.

### FR-7: Team Deathmatch Game Mode
- FR-7.1: Two teams, each with 4 to 8 players. The exact match size is determined by the matchmaking algorithm at queue time based on available players, ranging from 4v4 to 8v8.
- FR-7.2: Each player spawns in their chosen ship from their hangar.
- FR-7.3: The player respawns after a configurable delay and can pick any ship from their hangar.
- FR-7.4: The match ends when one team reaches the score limit (configurable kills) or the time limit expires.
- FR-7.5: The match must appear in the player's match history with basic stats (kills, deaths, damage dealt, damage taken).

### FR-8: Frontend (Unreal Engine 5)
- FR-8.1: The UE5 client must connect to the backend via gRPC for all cold-time screens (main menu, shop, hangar, loadout, matchmaking queue).
- FR-8.2: The UE5 client must connect to the backend via QUIC for combat (ship movement, shooting, module activation).
- FR-8.3: The frontend must display ship stats (shield, armor, energy bars), weapon heat indicator, and module cooldowns during combat.
- FR-8.4: The frontend must integrate the Steamworks SDK for Steam authentication on startup.
- FR-8.5: During combat, the client must send input at the tick rate and interpolate received state updates for smooth rendering.

## Non-Goals (Out of Scope for MVP)

- PvE game modes (Waves Survival, Operation Scenario) and Open World mode.
- Ship models with visual 3D assets (placeholder geometry is acceptable).
- Keycloak auth provider implementation (only the interface + Steam provider).
- In-game economy, premium currency, monetization.
- Spectator mode, replays, or streaming features.
- Dedicated server tooling / server browser (matchmaking is queue-based).
- Anti-cheat beyond server-authoritative simulation.
- Cross-platform (Linux only for MVP)

## Design Considerations

### .proto Files as Single Source of Truth
- All domain-level messages for both gRPC and QUIC must be defined in `.proto` files under `protos/grpc/` and `protos/quic/`.
- `.proto` filenames must match the protobuf package name (e.g., `combat.proto`, `hangar.proto`).
- The Rust backend compiles these via `tonic-build`; the UE5 frontend compiles them via the standard protobuf toolchain.

### gRPC vs QUIC Boundary
| Concern | Transport | Rationale |
|---|---|---|
| Auth, Shop, Hangar, Loadout, Matchmaking, History | gRPC | Request-response, latency-tolerant |
| Ship input, State sync, Damage, Module activation, Missiles | QUIC | Low-latency, loss-tolerant, real-time |

### Auth Pluggability
Define a Rust trait:
```rust
#[async_trait]
trait AuthProvider {
    async fn authenticate(&self, token: &str) -> Result<UserId, AuthError>;
    async fn validate_session(&self, session_id: &str) -> Result<UserId, AuthError>;
}
```
The Steam provider reads from `SteamAuthConfig` (environment/flake). A future Keycloak provider implements the same trait.

### Nix Flakes
- Root flake: provides `podman`, `podman-compose`, and any root-level tooling (e.g., `just`, `direnv`). Must NOT include Rust or Unreal Engine dependencies.
- Backend flake: provides `rustc`, `cargo`, `sqlx-cli`, `protoc`, and system deps (openssl, etc.).
- Frontend flake: provides the Unreal Engine 5 build environment and required SDKs (Steamworks headers).
- All three flakes must expose `devShells.default` and `packages.default`.

## Technical Considerations

### Backend Technology Choices
- **Language:** Rust (stable toolchain).
- **Web framework for gRPC:** Tonic (based on Tokio).
- **QUIC implementation:** Quinn (Rust QUIC stack).
- **Database:** PostgreSQL with sqlx for async queries and compile-time checked SQL.
- **Authentication:** SteamWorks SDK integration on UE5 side; Steam Web API on backend side.
- **Protobuf compilation:** Use `tonic-build` in `build.rs`; protoc must be available via flake.

### Frontend Technology Choices
- **Engine:** Unreal Engine 5.7.
- **Networking:** Custom QUIC plugin (or integration via UE5's platform socket layer); gRPC via HTTP/2 plugin (or C++ gRPC library).
- **Auth:** Steamworks SDK C++ integration.

### Infrastructure
- `docker-compose.yml` must include: `backend` (Rust binary), `postgres` (official image), and optionally `adminer` or similar for dev DB inspection.
- Backend Dockerfile must use multi-stage build: cargo-chef for dependency caching, then release build.
- CI must run `cargo test`, `cargo clippy`, and `cargo fmt --check` on the backend.

### Protobuf Tooling
- `protoc` must be available in the backend Nix flake.
- Both `tonic-build` (Rust) and the UE5 protobuf plugin must compile from the same `.proto` files under `protos/`.
- CI must enforce that Rust struct definitions match their corresponding `.proto` files — any drift between backend/frontend implementations and the `.proto` contracts causes a CI failure.

## Success Metrics

1. **Green CI**: Every PR passes backend compilation, clippy, and tests.
2. **Bootable stack**: `docker compose up` starts backend + DB, and `docker compose down` cleans up, without manual steps.
3. **Login flow**: A Steam-authenticated player reaches the main menu.
4. **Loadout persistence**: A player can equip a ship, save, reload, and see the same configuration.
5. **Playable match**: Two players can join the same match, move their ships, deal damage with correct type interactions, and finish a Team Deathmatch.
6. **Proto completeness**: Every gRPC message and QUIC message has a corresponding `.proto` definition.
7. **Auth swap**: The codebase has both `SteamAuthProvider` and a mock provider, demonstrating the pluggable interface works.

## Open Questions

None. All prior open questions have been resolved (see decisions above).
