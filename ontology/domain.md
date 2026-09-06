# Stargem — domain ontology

Canonical semantic layer. `model.rs` mirrors it; `instances/` populates it. Source
narrative: `../onthology.md`. Runtime state has no wire form yet; the server defines it when netcode lands.

IDs are stable kebab-case and never change once referenced.

## Scope

**Domain:** multiplayer space-ship combat (Star Conflict-like): ship catalog, player
ownership and loadouts, combat rules, and the game modes ships are deployed into.

**Primary users:** the authoritative server (rules, validation, persistence), the client
(display, hangar UI), content authors (instances).

**In scope:** ship models, sizes, roles, special modules, passive/active modules, weapons,
missiles, damage types, player ships, hangar, game modes, runtime combat invariants.

**Out of scope:** authentication, matchmaking, match history, economy beyond ship price,
progression / ship tree ranking, station interiors, AI behaviour, map/level content.

**Might add later (v2):** ship tree (prerequisites between ship models), module prices and
purchase, faction/tier, wave generator for `waves-survival`, operation scripts.

## Competency questions

| # | Question | Answered by |
|---|----------|-------------|
| 1 | Which ship models can a user buy, and for what price? | `ship-model.price`, `instances/ship_models.json` |
| 2 | What size and special module does a ship of role R have? | `role-has-size`, `role-has-special` |
| 3 | Can passive module M go into slot slot-type T of player ship S? | `passive-module.module_type`, `ship-model.slots`, C4 |
| 4 | Can active module A be equipped on a ship of role R? | `active-module.allowed_roles`, C6 |
| 5 | Which weapons fit a ship of size Z? | `weapon.size`, C7 |
| 6 | How much does damage type D hurt shield vs armor? | `damage-type.vs_shield / vs_armor`, C9 |
| 7 | What are the effective stats of a player ship after passives? | `stats + Σ passive.modifiers`, C5 |
| 8 | Which ships can the user deploy into a match? | `hangar.slots`, C11 |
| 9 | Does game mode G allow respawn, and where does the ship come from? | `game-mode.respawn_allowed`, `game-mode.category`, C12 |
| 10 | When is a ship destroyed, when does cloak drop, when does a weapon overheat? | C13, C14, C15 |

## Classes

Hierarchy (is-a):

```
catalog-item            (abstract: id, name; immutable, ship-independent)
├── ship-model
├── passive-module      ┐
├── active-module       │ equipable-item (disjoint union)
├── weapon              │
└── missile-model       ┘
special-module          (built into a role, NOT a catalog/equipable item)
damage-type
game-mode
player-ship             (mutable, owned)
hangar                  (mutable, owned)
ship-state              (runtime only; no wire form yet)
```

### ship-size
Enumeration: `frigate` (big), `fighter` (medium), `interceptor` (small).

### ship-role
Enumeration: `engineer`, `long-range`, `guard`, `tackler`, `gunship`, `command`,
`covert-ops`, `recon`, `ecm`. A role belongs to exactly one size (see `role-has-size`).

### damage-type
| property | type | note |
|----------|------|------|
| id | `electromagnetic` \| `kinetic` \| `thermic` | |
| vs_shield | f32 > 0 | multiplier applied to shield damage |
| vs_armor | f32 > 0 | multiplier applied to armor damage |

### stats
Value object, used as base stats of a ship model and as the result of applying passives.

| property | type | note |
|----------|------|------|
| shield_hp | f32 ≥ 0 | |
| armor_hp | f32 > 0 | |
| energy | f32 > 0 | capacity |
| energy_regen | f32 ≥ 0 | per second, continuous |
| speed | f32 > 0 | m/s cap (arcade physics: cap, drag, instant accel) |
| agility | f32 > 0 | deg/s |

### stat-modifiers
Value object: same fields as `stats`, additive deltas, each defaults to 0. May be negative.

### passive-slot-type
Enumeration: `shield`, `armor`, `capacitor`, `motor`, `computer`.

### ship-model (catalog-item)
| property | type | note |
|----------|------|------|
| id, name | id, string | |
| role | ship-role | size is derived from role |
| price | u32 > 0 | |
| base | stats | |
| slots | map passive-slot-type → u8 in 0..=3 | fixed per model |
| stock_weapon_id | → weapon | equipped at purchase |
| stock_missile_id | → missile-model | equipped at purchase |

### passive-module (catalog-item, equipable-item)
| property | type | note |
|----------|------|------|
| id, name | id, string | |
| module_type | passive-slot-type | slot family it occupies |
| model_code | string | machine-readable code |
| modifiers | stat-modifiers | may act "against type" (light armor: +speed, −armor) |

### active-module (catalog-item, equipable-item)
| property | type | note |
|----------|------|------|
| id, name | id, string | |
| allowed_roles | ship-role[] | empty = any role |
| flow | activation-flow | |
| effect | string | designer-facing description |

### activation-flow
Discriminated union:
- `one-shot { energy_cost: f32 > 0, cooldown_s: f32 ≥ 0 }` — press → effect, pay once, wait cooldown.
- `ongoing { energy_per_s: f32 > 0, cooldown_s: f32 ≥ 0 }` — toggle; drains while on; cooldown before re-enabling.

### weapon (catalog-item, equipable-item)
| property | type | note |
|----------|------|------|
| id, name | id, string | |
| size | ship-size | only ships of this size may equip it |
| damage_type | damage-type | |
| damage_per_shot | f32 > 0 | |
| fire_rate | f32 > 0 | shots/s |
| heat_per_shot | f32 in (0,1] | heat is normalised 0..=1 |
| cooling_per_s | f32 > 0 | heat lost per second while not firing |
| overheat_penalty_s | f32 ≥ 0 | extra lock-out added when heat reaches 1 |

### missile-model (catalog-item, equipable-item)
| property | type | note |
|----------|------|------|
| id, name | id, string | |
| damage_type | damage-type | |
| damage | f32 > 0 | |
| speed | f32 > 0 | m/s |
| ammo | u8 > 0 | per launcher per life |
| reload_s | f32 > 0 | between launches |

### special-module
Built into the role; one per role; occupies the dedicated 5th slot; never in a loadout.

| property | type | note |
|----------|------|------|
| id | `drones` \| `sniper-weapon` \| `phasic-shield` \| `cloak` \| `overclock` \| `command-shield` \| `plasma-web` \| `hyper-propulsion` \| `em-surge` | |
| name | string | |
| activation | `press` \| `toggle` | |
| effect | string | |

### game-mode
| property | type | note |
|----------|------|------|
| id, name | id, string | |
| category | `pvp` \| `pve` \| `open-world` | |
| respawn_allowed | bool | |

### player-ship
Mutable instance of a ship model owned forever by one user.

| property | type | note |
|----------|------|------|
| id | id | |
| owner_id | → user | user is external (auth); only its id is referenced |
| ship_model_id | → ship-model | |
| loadout | loadout | |

### loadout
| property | type | note |
|----------|------|------|
| passive_module_ids | → passive-module[] | bounded by `ship-model.slots` |
| active_module_ids | → active-module[] | 0..=4 |
| weapon_id | → weapon | exactly 1 |
| missile_id | → missile-model | exactly 1 launcher |

### hangar
| property | type | note |
|----------|------|------|
| owner_id | → user | |
| slots | (→ player-ship)?[4] | fixed 4 slots, each may be empty |

### ship-state (runtime)
Extrinsic, per match, not persisted: position, velocity, rotation, `shield_hp`, `armor_hp`,
`energy`, `heat_level`, `cloaked`, active-module cooldowns. Wire form:
none yet (defined with the netcode). Initial values come from the effective stats (Q7).

## Relations

| id | subject | relation | object | cardinality |
|----|---------|----------|--------|-------------|
| role-has-size | ship-role | has-size | ship-size | many → 1 (functional) |
| role-has-special | ship-role | has-special | special-module | 1 ↔ 1 |
| model-has-role | ship-model | has-role | ship-role | many → 1 |
| model-stock-weapon | ship-model | stock-weapon | weapon | many → 1 |
| model-stock-missile | ship-model | stock-missile | missile-model | many → 1 |
| user-owns-ship | user | owns | player-ship | 1 → many, permanent |
| ship-has-model | player-ship | has-model | ship-model | many → 1 |
| ship-equips-passive | player-ship | equips | passive-module | many → 0..Σslots |
| ship-equips-active | player-ship | equips | active-module | many → 0..4 |
| ship-equips-weapon | player-ship | equips | weapon | many → 1 |
| ship-equips-missile | player-ship | equips | missile-model | many → 1 |
| active-restricted-to | active-module | restricted-to | ship-role | many → 0..many |
| weapon-fits-size | weapon | fits | ship-size | many → 1 |
| hangar-holds | hangar | holds | player-ship | 1 → 0..4 |
| user-selects-ship | user | selected-ship | player-ship | 1 → 0..1 (open world) |
| damage-affects | damage-type | multiplies | shield, armor | per `vs_shield`, `vs_armor` |

`role-has-size`:

| size | roles |
|------|-------|
| frigate | engineer, long-range, guard |
| fighter | tackler, gunship, command |
| interceptor | covert-ops, recon, ecm |

`role-has-special`:

| role | special-module |
|------|----------------|
| engineer | drones |
| long-range | sniper-weapon |
| guard | phasic-shield |
| tackler | cloak |
| gunship | overclock |
| command | command-shield |
| covert-ops | plasma-web |
| recon | hyper-propulsion |
| ecm | em-surge |

## Constraints

| id | constraint | checked by |
|----|-----------|------------|
| C1 | Every catalog id is unique within its family and every reference resolves. | `Catalog::validate` |
| C2 | `ship-model.slots[t]` ∈ 0..=3 for each passive-slot-type. | type `u8` + validate |
| C3 | Catalog items are immutable; only `player-ship` and `hangar` mutate. | server |
| C4 | For each slot type t: count of equipped passives with `module_type = t` ≤ `slots[t]`. | `Catalog::validate_loadout` |
| C5 | Effective stats = `base + Σ modifiers`; every effective stat must stay > 0 (`shield_hp`, `energy_regen` ≥ 0). | `Catalog::effective_stats` |
| C6 | Each equipped active module has empty `allowed_roles` or contains the ship's role; at most 4. | `Catalog::validate_loadout` |
| C7 | `weapon.size == ship-model.size` for the equipped weapon; exactly 1. | `Catalog::validate_loadout` |
| C8 | Exactly 1 missile launcher, loaded with 1 missile-model. | `Catalog::validate_loadout` |
| C9 | Damage dealt: shield takes `raw × vs_shield` until 0, remainder hits armor at `raw × vs_armor`. EM: `vs_shield > 1 > vs_armor`; kinetic: `vs_armor > 1 > vs_shield`; thermic: both ≈ 1. | validate + server |
| C10 | Buying a ship-model costs `price`; the resulting player-ship is created with stock weapon/missile and no modules. | server |
| C11 | Hangar has exactly 4 slots; a player-ship appears in at most one slot; all held ships belong to `owner_id`. | `Hangar::validate` |
| C12 | `pvp`/`pve`: deploy from hangar; if `respawn_allowed` the next life may pick another hangar ship. `open-world`: deploy the selected ship from the current space station. | server |
| C13 | A ship is destroyed when `armor_hp` reaches 0. | server |
| C14 | `cloak` turns off when the cloaked ship takes damage. | server |
| C15 | Weapon heat is 0..=1; firing adds `heat_per_shot`, idle removes `cooling_per_s`. At 1 the weapon is locked until heat cools fully plus `overheat_penalty_s`. | server |
| C16 | Energy is 0..=capacity and regenerates `energy_regen`/s; an active module can only activate (or stay on) while its cost can be paid. `command-shield` drains energy instead of shield. | server |
| C17 | `special-module` is never in a loadout; it is derived from the role (`role-has-special`). | type system |

## Generators

None yet. Candidates (v2): wave spawner for `waves-survival`, operation script for
`operation-scenario`. Model each as a class with parameters, seed and invariants before
implementing.

## Instances

| file | class |
|------|-------|
| `instances/damage_types.json` | damage-type |
| `instances/special_modules.json` | special-module |
| `instances/ship_models.json` | ship-model |
| `instances/passive_modules.json` | passive-module |
| `instances/active_modules.json` | active-module |
| `instances/weapons.json` | weapon |
| `instances/missiles.json` | missile-model |
| `instances/game_modes.json` | game-mode |

Player-ships and hangars are not instances; they are persisted per user by the server.
