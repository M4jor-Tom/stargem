# Graph Report - .  (2026-06-13)

## Corpus Check
- Corpus is ~17,366 words - fits in a single context window. You may not need a graph.

## Summary
- 467 nodes · 655 edges · 40 communities (37 shown, 3 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 6 edges (avg confidence: 0.93)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- [[_COMMUNITY_Shop gRPC Service|Shop gRPC Service]]
- [[_COMMUNITY_Core Domain Concepts|Core Domain Concepts]]
- [[_COMMUNITY_Hangar gRPC Service|Hangar gRPC Service]]
- [[_COMMUNITY_Loadout gRPC Service|Loadout gRPC Service]]
- [[_COMMUNITY_Matchmaking gRPC Service|Matchmaking gRPC Service]]
- [[_COMMUNITY_Auth gRPC Service|Auth gRPC Service]]
- [[_COMMUNITY_Match History gRPC Service|Match History gRPC Service]]
- [[_COMMUNITY_Combat Tick Loop|Combat Tick Loop]]
- [[_COMMUNITY_Loadout Proto Messages|Loadout Proto Messages]]
- [[_COMMUNITY_Matchmaking Proto Messages|Matchmaking Proto Messages]]
- [[_COMMUNITY_Combat Damage System|Combat Damage System]]
- [[_COMMUNITY_Hangar Proto Messages|Hangar Proto Messages]]
- [[_COMMUNITY_Auth Proto Messages|Auth Proto Messages]]
- [[_COMMUNITY_Match History Proto|Match History Proto]]
- [[_COMMUNITY_QUIC Combat Protocol|QUIC Combat Protocol]]
- [[_COMMUNITY_Ship Model Types|Ship Model Types]]
- [[_COMMUNITY_Combat Physics|Combat Physics]]
- [[_COMMUNITY_Shop Proto Messages|Shop Proto Messages]]
- [[_COMMUNITY_Weapons Module|Weapons Module]]
- [[_COMMUNITY_Main Entry Types|Main Entry Types]]
- [[_COMMUNITY_Ship Stats|Ship Stats]]
- [[_COMMUNITY_QUIC Common Types|QUIC Common Types]]
- [[_COMMUNITY_Transport QUIC Server|Transport QUIC Server]]
- [[_COMMUNITY_Build Script|Build Script]]
- [[_COMMUNITY_Active Modules|Active Modules]]
- [[_COMMUNITY_Missiles Module|Missiles Module]]
- [[_COMMUNITY_Passive Modules|Passive Modules]]
- [[_COMMUNITY_OpenCode Config|OpenCode Config]]
- [[_COMMUNITY_OpenCode Package|OpenCode Package]]
- [[_COMMUNITY_Proto Verify Script|Proto Verify Script]]

## God Nodes (most connected - your core abstractions)
1. `AuthServiceServer<T>` - 15 edges
2. `HangarServiceServer<T>` - 15 edges
3. `LoadoutServiceServer<T>` - 15 edges
4. `MatchHistoryServiceServer<T>` - 15 edges
5. `MatchmakingServiceServer<T>` - 15 edges
6. `ShopServiceServer<T>` - 15 edges
7. `CombatTickLoop` - 14 edges
8. `Ship Domain Model` - 11 edges
9. `Stargem Project` - 10 edges
10. `Backend Server` - 10 edges

## Surprising Connections (you probably didn't know these)
- `Frontend Client` --communicates_with--> `Backend Server`  [EXTRACTED]
  client/README.md → server/src/main.rs
- `Rust Builds Rule` --conceptually_related_to--> `Backend Server`  [EXTRACTED]
  .opencode/rules/rust-builds.md → server/src/main.rs
- `Combat System` --reads_config--> `Damage Multipliers Config`  [INFERRED]
  server/src/combat/mod.rs → server/config/damage_multipliers.toml
- `Game Design Ontology` --conceptually_related_to--> `Combat System`  [EXTRACTED]
  onthology.md → server/src/combat/mod.rs
- `Game Design Ontology` --conceptually_related_to--> `Ship Domain Model`  [EXTRACTED]
  onthology.md → server/src/ship/model.rs

## Import Cycles
- 1-file cycle: `server/src/combat/physics.rs -> server/src/combat/physics.rs`
- 1-file cycle: `server/src/combat/tick.rs -> server/src/combat/tick.rs`
- 1-file cycle: `server/src/ship/active_modules.rs -> server/src/ship/active_modules.rs`
- 1-file cycle: `server/src/ship/missiles.rs -> server/src/ship/missiles.rs`
- 1-file cycle: `server/src/ship/model.rs -> server/src/ship/model.rs`
- 1-file cycle: `server/src/ship/modules.rs -> server/src/ship/modules.rs`
- 1-file cycle: `server/src/ship/stats.rs -> server/src/ship/stats.rs`
- 1-file cycle: `server/src/ship/weapons.rs -> server/src/ship/weapons.rs`

## Communities (40 total, 3 thin omitted)

### Community 0 - "Shop gRPC Service"
Cohesion: 0.08
Nodes (25): _Inner, _Inner<T>, ShopServiceServer, ShopServiceServer<T>, Arc, B, Clone, CompressionEncoding (+17 more)

### Community 1 - "Core Domain Concepts"
Cohesion: 0.08
Nodes (33): ActiveModuleDef Struct, Pluggable Auth Module, Backend Server, CI Pipeline, Combat System, Commands Rule, Damage Multipliers Config, DamageType Enum (+25 more)

### Community 2 - "Hangar gRPC Service"
Cohesion: 0.10
Nodes (21): HangarServiceServer<T>, _Inner, _Inner<T>, Arc, B, Clone, CompressionEncoding, Context (+13 more)

### Community 3 - "Loadout gRPC Service"
Cohesion: 0.10
Nodes (21): _Inner, _Inner<T>, LoadoutServiceServer<T>, Arc, B, Clone, CompressionEncoding, Context (+13 more)

### Community 4 - "Matchmaking gRPC Service"
Cohesion: 0.10
Nodes (21): _Inner, _Inner<T>, MatchmakingServiceServer<T>, Arc, B, Clone, CompressionEncoding, Context (+13 more)

### Community 5 - "Auth gRPC Service"
Cohesion: 0.10
Nodes (20): AuthServiceServer<T>, _Inner<T>, Arc, B, Clone, CompressionEncoding, Context, Debug (+12 more)

### Community 6 - "Match History gRPC Service"
Cohesion: 0.10
Nodes (20): _Inner<T>, MatchHistoryServiceServer<T>, Arc, B, Clone, CompressionEncoding, Context, Debug (+12 more)

### Community 7 - "Combat Tick Loop"
Cohesion: 0.20
Nodes (15): CombatTickLoop, DamageEventRecord, PlayerState, TickSnapshot, DamageEventRecord, DamageMultipliers, HashMap, PhysicsState (+7 more)

### Community 8 - "Loadout Proto Messages"
Cohesion: 0.18
Nodes (15): EquipActiveModuleRequest, EquipLoadoutResponse, EquipMissileRequest, EquipPassiveModuleRequest, EquipWeaponRequest, Loadout, LoadoutService, LoadoutServiceServer (+7 more)

### Community 9 - "Matchmaking Proto Messages"
Cohesion: 0.17
Nodes (15): LeaveQueueRequest, LeaveQueueResponse, MatchmakingService, MatchmakingServiceServer, QueueForMatchRequest, QueueForMatchResponse, QueueState, QueueStatusRequest (+7 more)

### Community 10 - "Combat Damage System"
Cohesion: 0.23
Nodes (12): apply_damage(), DamageMultipliers, DamageResult, DamageType, load_damage_multipliers(), test_electromagnetic_vs_shield(), test_ship_destroyed(), TypeMultipliers (+4 more)

### Community 11 - "Hangar Proto Messages"
Cohesion: 0.16
Nodes (14): AssignShipToSlotRequest, AssignShipToSlotResponse, HangarService, HangarServiceServer, HangarSlot, ListHangarRequest, ListHangarResponse, EnabledCompressionEncodings (+6 more)

### Community 12 - "Auth Proto Messages"
Cohesion: 0.18
Nodes (13): AuthService, AuthServiceServer, _Inner, LoginRequest, LoginResponse, ValidateSessionRequest, ValidateSessionResponse, EnabledCompressionEncodings (+5 more)

### Community 13 - "Match History Proto"
Cohesion: 0.15
Nodes (13): GetHistoryRequest, GetHistoryResponse, _Inner, MatchHistoryService, MatchHistoryServiceServer, MatchRecord, EnabledCompressionEncodings, _Inner (+5 more)

### Community 14 - "QUIC Combat Protocol"
Cohesion: 0.25
Nodes (13): PlayerId, Quaternion, DamageEvent, GameStateSnapshot, MissileLaunch, ModuleActivation, PlayerInput, ShipState (+5 more)

### Community 15 - "Ship Model Types"
Cohesion: 0.26
Nodes (9): Option, Self, String, Uuid, HullStats, PlayerShip, ShipModel, ShipRole (+1 more)

### Community 16 - "Combat Physics"
Cohesion: 0.29
Nodes (6): PhysicsState, quaternion_from_axis_angle(), quaternion_multiply(), ShipInput, PlayerShipStats, Self

### Community 17 - "Shop Proto Messages"
Cohesion: 0.24
Nodes (10): BuyShipRequest, BuyShipResponse, ListShipsRequest, ListShipsResponse, ShipModel, ShopService, Send, String (+2 more)

### Community 18 - "Weapons Module"
Cohesion: 0.33
Nodes (7): DamageType, Self, String, Uuid, WeaponDef, WeaponSize, ShipSize

### Community 19 - "Main Entry Types"
Cohesion: 0.25
Nodes (7): Box, Error, Option, Result, String, Args, main()

### Community 20 - "Ship Stats"
Cohesion: 0.33
Nodes (5): PassiveModuleDef, PlayerShip, Self, PlayerShipStats, ShipModel

### Community 21 - "QUIC Common Types"
Cohesion: 0.38
Nodes (6): MatchId, PlayerId, Quaternion, ShipId, Vector3, String

### Community 22 - "Transport QUIC Server"
Cohesion: 0.48
Nodes (6): Box, Error, Result, ServerConfig, make_server_config(), serve()

### Community 23 - "Build Script"
Cohesion: 0.40
Nodes (4): main(), Box, Error, Result

### Community 24 - "Active Modules"
Cohesion: 0.60
Nodes (4): String, Uuid, ActivationFlow, ActiveModuleDef

### Community 25 - "Missiles Module"
Cohesion: 0.60
Nodes (4): DamageType, String, Uuid, MissileDef

### Community 26 - "Passive Modules"
Cohesion: 0.60
Nodes (4): String, Uuid, PassiveModuleDef, PassiveModuleType

## Knowledge Gaps
- **160 isolated node(s):** `@opencode-ai/plugin`, `$schema`, `instructions`, `verify.sh script`, `Result` (+155 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **3 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `CombatTickLoop` connect `Combat Tick Loop` to `Core Domain Concepts`?**
  _High betweenness centrality (0.011) - this node is a cross-community bridge._
- **Why does `load_damage_multipliers()` connect `Combat Damage System` to `Combat Tick Loop`?**
  _High betweenness centrality (0.007) - this node is a cross-community bridge._
- **What connects `@opencode-ai/plugin`, `$schema`, `instructions` to the rest of the system?**
  _160 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Shop gRPC Service` be split into smaller, more focused modules?**
  _Cohesion score 0.08258258258258258 - nodes in this community are weakly interconnected._
- **Should `Core Domain Concepts` be split into smaller, more focused modules?**
  _Cohesion score 0.07765151515151515 - nodes in this community are weakly interconnected._
- **Should `Hangar gRPC Service` be split into smaller, more focused modules?**
  _Cohesion score 0.09659090909090909 - nodes in this community are weakly interconnected._
- **Should `Loadout gRPC Service` be split into smaller, more focused modules?**
  _Cohesion score 0.09659090909090909 - nodes in this community are weakly interconnected._