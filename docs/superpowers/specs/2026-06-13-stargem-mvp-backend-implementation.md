# Stargem MVP Backend Implementation

## Overview

Implement remaining backend tasks from `tasks/backlog/tasks-stargem-mvp.md`:
4.0 (Database), 5.0 (Auth), 6.0 (gRPC Services), 7.0 (Team Deathmatch).

Frontend (8.0) is deferred — it lives in the `client/` UE5 submodule.

## Phase 1: Database Layer (4.0)

### Schema (`server/sql/schema.sql`)

```sql
CREATE TYPE ship_size AS ENUM ('Frigate', 'Fighter', 'Interceptor');
CREATE TYPE ship_role AS ENUM ('Engineer', 'LongRange', 'Guard', 'Tackler', 'Gunship', 'Command', 'CoverOps', 'Recon', 'Ecm');
CREATE TYPE match_type AS ENUM ('team_deathmatch');
CREATE TYPE match_result AS ENUM ('victory', 'defeat', 'draw');
CREATE TYPE passive_module_type AS ENUM ('shield', 'armor', 'capacitor', 'motor', 'computer');
CREATE TYPE damage_type AS ENUM ('electromagnetic', 'kinetic', 'thermic');
CREATE TYPE weapon_model AS ENUM ('cannon', 'beam', 'lance', 'autocannon', 'pulse', 'railgun', 'machine_gun', 'laser', 'ion_cannon');
CREATE TYPE activation_flow AS ENUM ('ongoing', 'one_shot');

CREATE TABLE users (
    id UUID PRIMARY KEY,
    steam_id TEXT UNIQUE NOT NULL,
    display_name TEXT NOT NULL,
    credit_balance INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE ship_models (
    id UUID PRIMARY KEY,
    size ship_size NOT NULL,
    role ship_role NOT NULL,
    price INTEGER NOT NULL,
    base_shield REAL NOT NULL,
    base_armor REAL NOT NULL,
    base_energy REAL NOT NULL,
    base_speed REAL NOT NULL,
    base_agility REAL NOT NULL,
    shields_count   INTEGER NOT NULL DEFAULT 1 CHECK (shields_count   BETWEEN 0 AND 3),
    armors_count    INTEGER NOT NULL DEFAULT 1 CHECK (armors_count    BETWEEN 0 AND 3),
    capacitors_count INTEGER NOT NULL DEFAULT 1 CHECK (capacitors_count BETWEEN 0 AND 3),
    motors_count    INTEGER NOT NULL DEFAULT 0 CHECK (motors_count    BETWEEN 0 AND 3),
    computers_count INTEGER NOT NULL DEFAULT 1 CHECK (computers_count BETWEEN 0 AND 3)
);

CREATE TABLE passive_modules (
    id UUID PRIMARY KEY,
    model INTEGER NOT NULL,
    module_type passive_module_type NOT NULL,
    shield_hp_modifier  REAL NOT NULL DEFAULT 0.0,
    armor_hp_modifier   REAL NOT NULL DEFAULT 0.0,
    energy_modifier     REAL NOT NULL DEFAULT 0.0,
    speed_modifier      REAL NOT NULL DEFAULT 0.0,
    agility_modifier    REAL NOT NULL DEFAULT 0.0
);

CREATE TABLE active_modules (
    id UUID PRIMARY KEY,
    model INTEGER NOT NULL,
    activation_flow activation_flow NOT NULL,
    energy_cost REAL NOT NULL DEFAULT 0.0,
    cooldown_seconds REAL NOT NULL DEFAULT 0.0
);

CREATE TABLE weapons (
    id UUID PRIMARY KEY,
    name weapon_model NOT NULL,
    size ship_size NOT NULL,
    damage_type damage_type NOT NULL,
    damage_per_shot REAL NOT NULL,
    fire_rate REAL NOT NULL,
    heat_per_shot REAL NOT NULL
);

CREATE TABLE missile_models (
    id UUID PRIMARY KEY,
    damage_type damage_type NOT NULL,
    damage REAL NOT NULL,
    speed REAL NOT NULL,
    turn_rate REAL NOT NULL,
    lifetime_seconds REAL NOT NULL,
    blast_radius REAL NOT NULL DEFAULT 0.0
);

CREATE TABLE player_ships (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    ship_model_id UUID NOT NULL REFERENCES ship_models(id)
);

CREATE TABLE loadout_configs (
    id UUID PRIMARY KEY,
    player_ship_id UUID NOT NULL REFERENCES player_ships(id) ON DELETE CASCADE,
    weapon_id UUID REFERENCES weapons(id),
    missile_id UUID REFERENCES missile_models(id)
);

CREATE TABLE loadout_passive_modules (
    loadout_id UUID NOT NULL REFERENCES loadout_configs(id) ON DELETE CASCADE,
    slot_index INTEGER NOT NULL,
    module_id UUID NOT NULL REFERENCES passive_modules(id),
    PRIMARY KEY (loadout_id, slot_index)
);

CREATE TABLE loadout_active_modules (
    loadout_id UUID NOT NULL REFERENCES loadout_configs(id) ON DELETE CASCADE,
    slot_index INTEGER NOT NULL CHECK (slot_index BETWEEN 0 AND 3),
    module_id UUID NOT NULL REFERENCES active_modules(id),
    PRIMARY KEY (loadout_id, slot_index)
);

CREATE TABLE hangar_assignments (
    user_id UUID NOT NULL REFERENCES users(id),
    slot_index INTEGER NOT NULL CHECK (slot_index BETWEEN 0 AND 3),
    player_ship_id UUID NOT NULL REFERENCES player_ships(id),
    PRIMARY KEY (user_id, slot_index)
);

CREATE TABLE match_records (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    match_type match_type NOT NULL DEFAULT 'team_deathmatch',
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    kills INTEGER NOT NULL DEFAULT 0,
    deaths INTEGER NOT NULL DEFAULT 0,
    damage_dealt REAL NOT NULL DEFAULT 0.0,
    damage_taken REAL NOT NULL DEFAULT 0.0,
    result match_result NOT NULL
);
```

### Seed (`server/sql/seed.sql`)

Seeds 9 ship models with UUIDs matching `config/ships/*.toml`. No display names — the app derives text from model IDs.

### Ship Config TOMLs

`server/config/ships/*.toml` — per-model config with slot counts instead of a slot list:

```toml
id = "550e8400-e29b-41d4-a716-446655440001"
size = "Frigate"
role = "Guard"
price = 50000
base_shield = 200.0
base_armor = 300.0
base_energy = 150.0
base_speed = 80.0
base_agility = 0.3
shields_count = 1
armors_count = 2
capacitors_count = 1
motors_count = 0
computers_count = 1
```

### Domain Model Updates (`server/src/ship/model.rs`)

- `ShipModel.id` changes from `String` to `Uuid`
- `ShipModel.name` removed (app derives display text from model ID)
- `ShipModel.passive_slots_layout: Vec<String>` replaced with individual count fields: `shields_count`, `armors_count`, `capacitors_count`, `motors_count`, `computers_count` (all `u8`)
- `ShipSize` and `ShipRole` enums map to PG ENUMs; serialized with `#[serde(rename_all = "snake_case")]`

### Catalog Structs (`server/src/ship/`)

| Struct | File | Key Fields |
|---|---|---|
| `PassiveModuleDef` | `modules.rs` | `id: Uuid`, `model: i32`, `module_type: PassiveModuleType`, stat modifiers |
| `ActiveModuleDef` | `active_modules.rs` | `id: Uuid`, `model: i32`, `activation_flow: ActivationFlow`, `energy_cost`, `cooldown_seconds` |
| `WeaponDef` | `weapons.rs` | `id: Uuid`, `name: WeaponModel` (enum), `size: ShipSize`, `damage_type: DamageType` |
| `MissileModelDef` | `missile_models.rs` | `id: Uuid`, `damage_type: DamageType`, `speed`, `turn_rate`, `lifetime_secs`, `blast_radius` |

### Startup Wiring

- Add `sqlx` (postgres, runtime-tokio, uuid) + `uuid` v4 to Cargo.toml
- `main.rs`: if `--database-url` provided, create `PgPool`, run `schema.sql` then `seed.sql`
- Seed data also loaded from `server/config/ships/*.toml`

## Phase 2: Auth Module (5.0)

### Trait (`server/src/auth.rs`)

```rust
#[async_trait]
pub trait AuthProvider: Send + Sync {
    async fn authenticate(&self, token: &str) -> Result<Uuid, AuthError>;
    async fn validate_session(&self, session_id: &str) -> Result<Uuid, AuthError>;
}
```

### Implementations

- **`MockAuthProvider`**: accepts any token, returns deterministic UUID. Used in dev/CI.
- **`SteamAuthProvider`**: reads `STEAM_WEB_API_KEY` from env, calls Steam Web API. Returns `AuthError::InvalidToken` on failure.

### Wire

AuthProvider injected into gRPC handler state. `AuthService.Login` calls `provider.authenticate()`, upserts user in `users` table, returns session token.

## Phase 3: gRPC Services (6.0)

### Architecture

- Tonic server in `server/src/grpc.rs`, started alongside QUIC in `main.rs`
- All 6 proto services implemented as handler structs
- Shared `AppState` (PgPool, AuthProvider, Matchmaking queue) passed via tonic `Router`
- `main.rs` uses `tokio::try_join!` for gRPC + QUIC servers

### Handler Behaviors

| Service | Key Behavior |
|---|---|
| Auth | Calls AuthProvider, upserts user, returns session (UUID) |
| Shop | Lists ship_models from DB, validates purchase, inserts player_ship |
| Hangar | CRUD hangar_assignments (max 4) |
| Loadout | CRUD loadout_configs + join tables per player_ship |
| Matchmaking | In-memory queue with position tracking |
| Match History | Query match_records with pagination |

## Phase 4: Team Deathmatch (7.0)

### Match Lifecycle

1. Queue fills → match created with 2 balanced teams
2. Each player picks a ship from their hangar
3. Combat tick loop runs (existing `CombatTickLoop` in `tick.rs`)
4. On death: increment opposing team score, schedule respawn (configurable delay)
5. Match ends when `score_limit` reached or `time_limit` expires
6. Match stats written to `match_records`

### GameMode Trait (`server/src/game_mode.rs`)

```rust
pub trait GameMode: Send + Sync {
    fn on_player_death(&mut self, player_id: &str, killer_id: Option<&str>);
    fn on_tick(&mut self, tick: &TickSnapshot);
    fn is_finished(&self) -> bool;
    fn results(&self) -> Option<MatchResults>;
    fn teams(&self) -> &[Vec<String>];
}
```

`TeamDeathmatch` implements this trait, wired into `CombatTickLoop`.

## Schema Changes Summary

- Display text (`name`, `description`) removed from all catalog tables — app derives text from model codes
- `passive_modules.module_type` → `passive_module_type` ENUM
- `active_modules.activation_flow` → `activation_flow` ENUM
- `weapons.name` → `weapon_model` ENUM, `weapons.damage_type` → `damage_type` ENUM
- `missiles` table renamed to `missile_models`, `damage_type` → `damage_type` ENUM
- `weapons.size` → `ship_size` ENUM (reuses the ship size enum)
- `passive_modules.model`, `active_modules.model` are `INTEGER` (enum-like codes, not display text)
- `active_modules.effect_definition` removed (no longer stored in schema)

## Success Criteria

- `cargo build` compiles cleanly
- `cargo test` passes all tests
- `cargo clippy -- -D warnings` passes
- Database schema loads and seeds on startup
- gRPC services respond to requests
- Auth mock provider returns expected responses
