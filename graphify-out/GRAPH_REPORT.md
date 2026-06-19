# Graph Report - .  (2026-06-19)

## Corpus Check
- 59 files · ~25,575 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 722 nodes · 1258 edges · 47 communities (38 shown, 9 thin omitted)
- Extraction: 94% EXTRACTED · 6% INFERRED · 0% AMBIGUOUS · INFERRED: 73 edges (avg confidence: 0.91)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- [[_COMMUNITY_gRPC Ship Slot Service|gRPC Ship Slot Service]]
- [[_COMMUNITY_gRPC Loadout Service|gRPC Loadout Service]]
- [[_COMMUNITY_gRPC Matchmaking Service|gRPC Matchmaking Service]]
- [[_COMMUNITY_gRPC Hangar Service|gRPC Hangar Service]]
- [[_COMMUNITY_gRPC Shop Service|gRPC Shop Service]]
- [[_COMMUNITY_Rust Core Types|Rust Core Types]]
- [[_COMMUNITY_gRPC Auth Service|gRPC Auth Service]]
- [[_COMMUNITY_gRPC Match History Service|gRPC Match History Service]]
- [[_COMMUNITY_Damage System|Damage System]]
- [[_COMMUNITY_Rust Concurrency Types|Rust Concurrency Types]]
- [[_COMMUNITY_Physics System|Physics System]]
- [[_COMMUNITY_Combat Tick Loop|Combat Tick Loop]]
- [[_COMMUNITY_Ship Model|Ship Model]]
- [[_COMMUNITY_Weapons & Damage Types|Weapons & Damage Types]]
- [[_COMMUNITY_Active Module System|Active Module System]]
- [[_COMMUNITY_CI & DevOps Pipeline|CI & DevOps Pipeline]]
- [[_COMMUNITY_Stargem MVP Design Docs|Stargem MVP Design Docs]]
- [[_COMMUNITY_QUIC Combat Protocol|QUIC Combat Protocol]]
- [[_COMMUNITY_Ship Stats & Passives|Ship Stats & Passives]]
- [[_COMMUNITY_Active Module State|Active Module State]]
- [[_COMMUNITY_Database Connection|Database Connection]]
- [[_COMMUNITY_Weapon Heat System|Weapon Heat System]]
- [[_COMMUNITY_Missile System|Missile System]]
- [[_COMMUNITY_QUIC Common Types|QUIC Common Types]]
- [[_COMMUNITY_Database Schema|Database Schema]]
- [[_COMMUNITY_QUIC Transport Config|QUIC Transport Config]]
- [[_COMMUNITY_Ship Modules|Ship Modules]]
- [[_COMMUNITY_Equipment Database Tables|Equipment Database Tables]]
- [[_COMMUNITY_Build Script|Build Script]]
- [[_COMMUNITY_Missile Definitions|Missile Definitions]]
- [[_COMMUNITY_OpenCode Config|OpenCode Config]]
- [[_COMMUNITY_Plugin Package|Plugin Package]]
- [[_COMMUNITY_Proto Verify Script|Proto Verify Script]]
- [[_COMMUNITY_Damage Config|Damage Config]]
- [[_COMMUNITY_Missile Struct|Missile Struct]]
- [[_COMMUNITY_Special Modules Per Role|Special Modules Per Role]]
- [[_COMMUNITY_Rust Builds Rule|Rust Builds Rule]]
- [[_COMMUNITY_Max Active Modules|Max Active Modules]]
- [[_COMMUNITY_Ship Config TOMLs|Ship Config TOMLs]]

## God Nodes (most connected - your core abstractions)
1. `TeamDeathmatch` - 20 edges
2. `PRD: Stargem MVP` - 19 edges
3. `apply_damage()` - 16 edges
4. `AppState` - 16 edges
5. `AuthServiceServer<T>` - 15 edges
6. `HangarServiceServer<T>` - 15 edges
7. `LoadoutServiceServer<T>` - 15 edges
8. `MatchHistoryServiceServer<T>` - 15 edges
9. `MatchmakingServiceServer<T>` - 15 edges
10. `ShopServiceServer<T>` - 15 edges

## Surprising Connections (you probably didn't know these)
- `3-Damage-Type Rock-Paper-Scissors System` --conceptually_related_to--> `DamageType`  [INFERRED]
  onthology.md → server/src/combat/damage.rs
- `3-Damage-Type Rock-Paper-Scissors System` --conceptually_related_to--> `DamageMultipliers`  [INFERRED]
  onthology.md → server/src/combat/damage.rs
- `Arcade-Style Ship Physics` --conceptually_related_to--> `PhysicsState`  [INFERRED]
  onthology.md → server/src/combat/physics.rs
- `Active Module Activation System` --rationale_for--> `ActiveModuleState`  [INFERRED]
  tasks/implemented/prd-stargem-mvp.md → server/src/ship/active_module_state.rs
- `PRD: Stargem MVP` --conceptually_related_to--> `ActiveModuleState`  [INFERRED]
  tasks/implemented/prd-stargem-mvp.md → server/src/ship/active_module_state.rs

## Import Cycles
- 1-file cycle: `server/src/ship/missiles.rs -> server/src/ship/missiles.rs`
- 1-file cycle: `server/src/auth.rs -> server/src/auth.rs`
- 1-file cycle: `server/src/combat/physics.rs -> server/src/combat/physics.rs`
- 1-file cycle: `server/src/combat/tick.rs -> server/src/combat/tick.rs`
- 1-file cycle: `server/src/database.rs -> server/src/database.rs`
- 1-file cycle: `server/src/game_mode.rs -> server/src/game_mode.rs`
- 1-file cycle: `server/src/grpc.rs -> server/src/grpc.rs`
- 1-file cycle: `server/src/ship/active_modules.rs -> server/src/ship/active_modules.rs`
- 1-file cycle: `server/src/ship/missile_models.rs -> server/src/ship/missile_models.rs`
- 1-file cycle: `server/src/ship/model.rs -> server/src/ship/model.rs`
- 1-file cycle: `server/src/ship/modules.rs -> server/src/ship/modules.rs`
- 1-file cycle: `server/src/ship/stats.rs -> server/src/ship/stats.rs`
- 1-file cycle: `server/src/ship/weapons.rs -> server/src/ship/weapons.rs`

## Hyperedges (group relationships)
- **gRPC API Handlers** — src_grpc_authhandler, src_grpc_shophandler, src_grpc_hangarhandler, src_grpc_loadouthandler, src_grpc_matchmakinghandler, src_grpc_matchhistoryhandler [EXTRACTED 1.00]
- **Combat Tick Loop System** — combat_damage_applydamage, combat_physics_physicsstate, combat_tick_combattickloop, combat_damage_damagemultipliers, combat_tick_playerstate [EXTRACTED 1.00]
- **Game Mode Match Lifecycle** — src_game_mode_gamemode, src_game_mode_teamdeathmatch, src_game_mode_matchmanager [EXTRACTED 1.00]
- **Test Coverage Initiative** — missing_test_scenarios_design, missing_test_scenarios_plan, prd_stargem_mvp, tasks_stargem_mvp [INFERRED 0.85]
- **Ship Domain Model** — ship_model_shipsize, ship_model_shiprole, ship_model_hullstats, ship_model_shipmodel, ship_model_playership [INFERRED 0.95]
- **Module System** — ship_modules_passivemoduletype, ship_modules_passivemoduledef, ship_active_modules_activationflow, ship_active_modules_activemoduledef, ship_active_module_state_activationstatus, ship_active_module_state_activationflow, ship_active_module_state_activemodulestate [INFERRED 0.85]

## Communities (47 total, 9 thin omitted)

### Community 0 - "gRPC Ship Slot Service"
Cohesion: 0.06
Nodes (60): AssignShipToSlotRequest, AssignShipToSlotResponse, AuthProvider, AuthService, BuyShipRequest, BuyShipResponse, EquipActiveModuleRequest, EquipLoadoutResponse (+52 more)

### Community 1 - "gRPC Loadout Service"
Cohesion: 0.06
Nodes (36): EquipActiveModuleRequest, EquipLoadoutResponse, EquipMissileRequest, EquipPassiveModuleRequest, EquipWeaponRequest, _Inner, _Inner<T>, Loadout (+28 more)

### Community 2 - "gRPC Matchmaking Service"
Cohesion: 0.06
Nodes (36): _Inner, _Inner<T>, LeaveQueueRequest, LeaveQueueResponse, MatchmakingService, MatchmakingServiceServer, MatchmakingServiceServer<T>, QueueForMatchRequest (+28 more)

### Community 3 - "gRPC Hangar Service"
Cohesion: 0.06
Nodes (35): AssignShipToSlotRequest, AssignShipToSlotResponse, HangarService, HangarServiceServer, HangarServiceServer<T>, HangarSlot, _Inner, _Inner<T> (+27 more)

### Community 4 - "gRPC Shop Service"
Cohesion: 0.06
Nodes (35): BuyShipRequest, BuyShipResponse, _Inner, _Inner<T>, ListShipsRequest, ListShipsResponse, ShipModel, ShopService (+27 more)

### Community 5 - "Rust Core Types"
Cohesion: 0.11
Nodes (34): Arc, HashMap, Mutex, Option, Self, Send, String, Sync (+26 more)

### Community 6 - "gRPC Auth Service"
Cohesion: 0.07
Nodes (33): AuthService, AuthServiceServer, AuthServiceServer<T>, _Inner, _Inner<T>, LoginRequest, LoginResponse, ValidateSessionRequest (+25 more)

### Community 7 - "gRPC Match History Service"
Cohesion: 0.06
Nodes (33): GetHistoryRequest, GetHistoryResponse, _Inner, _Inner<T>, MatchHistoryService, MatchHistoryServiceServer, MatchHistoryServiceServer<T>, MatchRecord (+25 more)

### Community 8 - "Damage System"
Cohesion: 0.14
Nodes (26): apply_damage(), apply_damage Function, DamageMultipliers, DamageResult, DamageType, load_damage_multipliers(), test_all_damage_types_with_zero_shield(), test_damage_bleeds_through_shield_into_armor() (+18 more)

### Community 9 - "Rust Concurrency Types"
Cohesion: 0.16
Nodes (19): HashMap, Mutex, Result, Self, Send, String, Sync, Uuid (+11 more)

### Community 10 - "Physics System"
Cohesion: 0.20
Nodes (21): dummy_stats(), PhysicsState, quaternion_from_axis_angle(), quaternion_multiply(), ShipInput, test_dt_zero_produces_no_change(), test_forward_vector_identity(), test_forward_vector_unit_length_after_rotation() (+13 more)

### Community 11 - "Combat Tick Loop"
Cohesion: 0.17
Nodes (20): load_damage_multipliers Function, CombatTickLoop, DamageEventRecord, PlayerState, test_add_player_initializes_hp_from_stats(), test_add_player_inserts_player_state(), test_combat_tick_loop_loads_default_damage_multipliers(), test_combat_tick_loop_new_defaults() (+12 more)

### Community 12 - "Ship Model"
Cohesion: 0.14
Nodes (4): Option, Self, test_ship_role_serde_roundtrip_all_variants(), test_ship_size_serde_roundtrip()

### Community 13 - "Weapons & Damage Types"
Cohesion: 0.16
Nodes (6): DamageType, Uuid, WeaponDef, WeaponModel, ShipSize, Weapon Overheat Mechanic

### Community 14 - "Active Module System"
Cohesion: 0.16
Nodes (4): Active Module Activation System, Uuid, ActivationFlow, ActiveModuleDef

### Community 15 - "CI & DevOps Pipeline"
Cohesion: 0.14
Nodes (14): CI Pipeline, Combat System, Commands Rule, Docker Compose, Docker Image Rule, Frontend Client, Just Orchestration, Monorepo Shell (+6 more)

### Community 16 - "Stargem MVP Design Docs"
Cohesion: 0.38
Nodes (13): Missing Test Scenarios Design, Missing Test Scenarios Plan, PRD: Stargem MVP, Uuid, Ship Domain Model, HullStats, PlayerShip, ShipModel (+5 more)

### Community 17 - "QUIC Combat Protocol"
Cohesion: 0.25
Nodes (13): PlayerId, Quaternion, DamageEvent, GameStateSnapshot, MissileLaunch, ModuleActivation, PlayerInput, ShipState (+5 more)

### Community 18 - "Ship Stats & Passives"
Cohesion: 0.37
Nodes (11): PassiveModuleDef, PlayerShip, Self, dummy_model(), dummy_ship(), test_additive_module_stacking(), test_compute_base_stats_no_modules(), test_compute_with_modules_applies_multipliers() (+3 more)

### Community 19 - "Active Module State"
Cohesion: 0.21
Nodes (5): Result, Self, ActivationFlow, ActivationStatus, ActiveModuleState

### Community 20 - "Database Connection"
Cohesion: 0.18
Nodes (10): Box, Error, Option, Result, String, init_pool Function, run_schema Function, run_seed Function (+2 more)

### Community 21 - "Weapon Heat System"
Cohesion: 0.22
Nodes (3): Default, Self, WeaponHeatState

### Community 22 - "Missile System"
Cohesion: 0.38
Nodes (4): Damage Type System, DamageType, Uuid, MissileModelDef

### Community 23 - "QUIC Common Types"
Cohesion: 0.38
Nodes (6): MatchId, PlayerId, Quaternion, ShipId, Vector3, String

### Community 24 - "Database Schema"
Cohesion: 0.62
Nodes (6): Error, PgPool, Result, init_pool(), run_schema(), run_seed()

### Community 25 - "QUIC Transport Config"
Cohesion: 0.48
Nodes (6): Box, Error, Result, ServerConfig, make_server_config(), serve()

### Community 26 - "Ship Modules"
Cohesion: 0.47
Nodes (3): Uuid, PassiveModuleDef, PassiveModuleType

### Community 27 - "Equipment Database Tables"
Cohesion: 0.40
Nodes (5): Equipable Items System, active_modules Table, missile_models Table, passive_modules Table, weapons Table

### Community 28 - "Build Script"
Cohesion: 0.40
Nodes (4): main(), Box, Error, Result

### Community 29 - "Missile Definitions"
Cohesion: 0.60
Nodes (4): DamageType, String, Uuid, MissileDef

## Knowledge Gaps
- **197 isolated node(s):** `@opencode-ai/plugin`, `$schema`, `instructions`, `verify.sh script`, `Result` (+192 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **9 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `DamageType` connect `Damage System` to `Weapons & Damage Types`, `Missile System`?**
  _High betweenness centrality (0.053) - this node is a cross-community bridge._
- **Why does `WeaponDef` connect `Weapons & Damage Types` to `Stargem MVP Design Docs`, `Damage System`, `Weapon Heat System`, `Missile System`?**
  _High betweenness centrality (0.037) - this node is a cross-community bridge._
- **Are the 17 inferred relationships involving `PRD: Stargem MVP` (e.g. with `Active Module Activation System` and `Damage Type System`) actually correct?**
  _`PRD: Stargem MVP` has 17 INFERRED edges - model-reasoned connections that need verification._
- **Are the 3 inferred relationships involving `apply_damage()` (e.g. with `electromag_pierces_shield_quickly()` and `gunship_cannon_destroys_recon_in_team_deathmatch()`) actually correct?**
  _`apply_damage()` has 3 INFERRED edges - model-reasoned connections that need verification._
- **What connects `@opencode-ai/plugin`, `$schema`, `instructions` to the rest of the system?**
  _198 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `gRPC Ship Slot Service` be split into smaller, more focused modules?**
  _Cohesion score 0.06493506493506493 - nodes in this community are weakly interconnected._
- **Should `gRPC Loadout Service` be split into smaller, more focused modules?**
  _Cohesion score 0.06377551020408163 - nodes in this community are weakly interconnected._