# Handoff: inventory — afc65a65

Checkpoint: 2026-09-16, user-requested pause. Parent-curated from runtime records and actual files; not a new completed child report.

- **State:** COMPLETED; reuse output, do not rerun by default
- **Role:** `scout`
- **Child run:** `afc65a65-c2d5-471e-a615-9e4f21d7c463`
- **Workflow / key:** `69d652a5-b7d3-47be-83bd-4c3f7677ee17` / `inventory`
- **Mission:** `28a33a06-b296-4a78-8ae0-079881816489`
- **Repository:** `/home/theta/repos/stargem.nix`, branch `master`
- **Source baseline:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`
- **Session (machine-local, optional for recovery):** `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/2026-09-16T18-47-01-030Z_01a0ab8b-30a5-761c-90d3-cc64125796e1/343004fd-e5dc-472c-8e12-d658e5f7d4f0/run-0/session.jsonl`
- **Runtime status (machine-local):** `/tmp/pi-subagents-uid-1000/async-subagent-runs/afc65a65-c2d5-471e-a615-9e4f21d7c463/status.json`

## Authority and boundaries

Read root `AGENTS.md` and `docs/HANDOFF.md` before work. `ontology/` remains canonical and unchanged. This is documentation/research, not authorization to change gameplay. No commit/push/merge/install is authorized. One writer per checkout. Preserve untracked audit files: they are the current work, not disposable clutter. Other agents' full research is intentionally file-backed to protect parent context.

## Resumption (for Agent)

Reuse this inventory. Source/ontology baseline is unchanged. Its claim that AGENTS/HANDOFF were absent was true at kickoff but is now historical. First workflow failed only on undefined optional output metadata; inventory itself succeeded. No re-scout needed unless baseline changes.

## Native continuation or fallback

Recheck live availability before any resume. Completed work normally requires no resume.

Not rechecked in the latest retained-child list; no resume is needed. Use the preserved report below. If genuinely necessary, inspect status/children.list first; otherwise start a same-role focused continuation from this handoff.

## Verification and acceptance

The original report below states exactly what this child checked and what remains uncertain. Completion of a research child does not mean that document synthesis, independent evidence review, ontology review or user reconciliation has completed. Follow [the main checkpoint](README.md) for dependency order and pending reviews.

## Preserved original output

Original managed artifact: `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/69d652a5-b7d3-47be-83bd-4c3f7677ee17/audit/inventory.md`

SHA-256 of original artifact: `10a66171438c4e3786f64edb3910e62dc05eeef3ce830ffb863655997def8a81`

The complete output is embedded below so recovery does not depend on session/temp-file retention. Original relative code paths refer to the repository root unless the report says otherwise. Historical access failures stay historical.

---

# Code Context

Read-only reconnaissance, 2026-09-16. Repository `/home/theta/repos/stargem.nix`; HEAD verified as baseline `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`; working tree clean on inspection. No repository changes, builds, package installations, web research, or submodule traversal performed. Root `AGENTS.md` and `docs/HANDOFF.md` checked first and absent. `graphify-out/graph.json` absent; direct inspection used. Ontology skill read at the requested absolute path; Stanford seven steps and all five Gruber criteria applied below.

## Files Retrieved

All paths below are repository-relative unless absolute. Complete exact inventory closes this report.

1. `ontology/domain.md` (1–275): canonical definitions, ten competency questions, 17 constraints, ownership of future runtime rules.
2. `ontology.md` (1–65): source narrative, not independently verified live-game documentation.
3. `ontology/model.rs` (1–479): complete Rust mirror, serialization, catalog loader, validators, player-owned objects, four tests.
4. All eight `ontology/instances/*.json` files, in full: 46 catalog records total; file ranges at end.
5. `sim/src/lib.rs` (1–153), `client/src/main.rs` (1–95): complete present consumers, no combat implementation hidden elsewhere in tracked Rust.
6. `README.md` (1–25), `Cargo.toml` (1–12), `ontology/Cargo.toml` (1–11), `sim/Cargo.toml` (1–9), `client/Cargo.toml` (1–10), `flake.nix` (1–26), `flake.lock` (1–27), `.gitmodules` (1–6), `.gitignore` (1–2): conventions, dependency wiring, development shell and excluded asset repositories.
7. Requested ontology skill outside repository: `/home/theta/.pi-game-dev/git/github.com/M4jor-Tom/claude-ontology-skill/skills/ontology/SKILL.md`, read in full.

## Key Code

### Vocabulary and exact represented catalog

`ontology/domain.md:59–90`, `ontology/model.rs:8–93`:
- Three sizes: `frigate` (big), `fighter` (medium), `interceptor` (small).
- Nine mutually exclusive roles; role determines size and one special through exhaustive Rust matches (`model.rs:24–51`).
- Five passive slot families: `shield`, `armor`, `capacitor`, `motor`, `computer`.
- Six stats: shield HP, armor HP, energy capacity, energy regeneration/s, speed m/s, agility deg/s. Additive modifiers default omitted fields to zero; negative modifiers allowed.
- Two active flows: tagged `one-shot { energy_cost, cooldown_s }` and `ongoing { energy_per_s, cooldown_s }` (`model.rs:136–150`). Empty allowed-role list means unrestricted.
- Special activation is `press` or `toggle`; effects remain free-text, not executable rules.

| Size | Role | Special | Model | Price | Source lines in ship_models.json |
|---|---|---|---|---:|---|
| frigate | engineer | drones | anaconda | 12000 | 2–5 |
| frigate | long-range | sniper-weapon | styx | 12000 | 6–9 |
| frigate | guard | phasic-shield | mammoth | 12000 | 10–13 |
| fighter | tackler | cloak | hydra | 8000 | 14–17 |
| fighter | gunship | overclock | phoenix | 8000 | 18–21 |
| fighter | command | command-shield | lynx | 8000 | 22–25 |
| interceptor | covert-ops | plasma-web | shrike | 6000 | 26–29 |
| interceptor | recon | hyper-propulsion | wolfhound | 6000 | 30–33 |
| interceptor | ecm | em-surge | kite | 6000 | 34–37 |

These are local mappings, not claims that those named ships have these roles in Star Conflict.

Exact base tuples `(shield, armor, energy, regen, speed, agility)` and slot tuples `(shield, armor, capacitor, motor, computer)`:

| Model | Base | Slots | Stock weapon / missile |
|---|---|---|---|
| anaconda | 9000,6000,600,60,180,20 | 3,1,2,1,1 | heavy-laser / cruise-missile |
| styx | 7000,7000,550,55,190,22 | 2,2,2,1,2 | heavy-railgun / cruise-missile |
| mammoth | 6000,11000,500,50,170,18 | 1,3,2,1,1 | heavy-laser / cruise-missile |
| hydra | 4500,4500,450,50,260,40 | 2,2,1,2,2 | assault-railgun / homing-missile |
| phoenix | 4000,5500,450,48,270,38 | 1,3,2,2,1 | assault-railgun / homing-missile |
| lynx | 5500,4000,500,55,250,36 | 3,1,2,1,2 | pulse-laser / homing-missile |
| shrike | 2500,2500,400,45,360,60 | 1,1,2,3,1 | plasma-gun / homing-missile |
| wolfhound | 2800,2200,420,48,380,62 | 2,1,1,3,2 | plasma-gun / homing-missile |
| kite | 3000,2000,450,50,350,58 | 2,1,2,2,2 | plasma-gun / homing-missile |

Catalog counts and content:
- **9 ship models**, exactly one per role, three per size.
- **9 special modules** (`special_modules.json:2–10`): drones/press heals allies' shields in range; sniper-weapon/toggle switches weapon; phasic-shield/toggle cycles resistance; cloak/press drops on damage; overclock/press boosts weapons and motors; command-shield/toggle drains energy instead of shield; plasma-web/press damage over time; hyper-propulsion/press warp; em-surge/press freezes every nearby ship. No numeric durations/ranges/costs for these effects.
- **3 damage types** (`damage_types.json:2–4`): electromagnetic multipliers shield **1.5**, armor **0.7**; kinetic **0.7**, **1.5**; thermic **1**, **1**.
- **8 passives** (`passive_modules.json:2–9`): shield-extender +1500 shield; reinforced-hull +1800 armor/−10 speed; light-armor −600 armor/+30 speed; capacitor-bank +150 energy; energy-recycler +12 regen; speed-booster +40 speed; gyro-stabilizer +8 agility; targeting-cpu +4 agility/+3 regen. Family/code/modifiers are explicit.
- **5 actives** (`active_modules.json:2–16`): shield-boost unrestricted one-shot 120 energy/25s cooldown; remote-repair engineer ongoing 25/s/10s; afterburner unrestricted ongoing 20/s/5s; emergency-barrier guard or command one-shot 200/60s (text: absorb all damage for 3 seconds); engine-suppressor tackler one-shot 100/30s. Effects otherwise lack numeric magnitude/duration.
- **5 weapons** (`weapons.json:2–11`), tuples `(damage/shot, shots/s, heat/shot, cooling/s, penalty seconds)`: heavy-laser frigate thermic `(420,2,.05,.25,3)`; heavy-railgun frigate kinetic `(900,.8,.12,.25,4)`; assault-railgun fighter kinetic `(260,3,.045,.30,2.5)`; pulse-laser fighter electromagnetic `(180,5,.03,.30,2.5)`; plasma-gun interceptor electromagnetic `(140,4,.04,.35,2)`. Two frigate, two fighter, one interceptor weapon; no catalog completeness claim.
- **3 missiles** (`missiles.json:2–4`), tuples `(damage,speed,ammo,reload seconds)`: homing-missile thermic `(1200,600,6,8)`; cruise-missile kinetic `(3500,400,4,15)`; emp-missile electromagnetic `(900,700,8,6)`. No size/role restriction and no typed guidance, blast radius or status effect.
- **4 modes** (`game_modes.json:2–5`): team-deathmatch/pvp/respawn true; waves-survival/pve/true; operation-scenario/pve/false; open-world/open-world/true. No objectives, team sizes, score or victory rules.

### Loadout, ownership and constraints

`domain.md:162–189,193–211,238–254`; `model.rs:186–228,342–419`:
- PlayerShip has string ID, external owner ID, model ID, loadout. No user/auth class.
- Passives: vector of IDs bounded separately per family by model slot counts (each 0–3); omitted slot-map entry means zero (`model.rs:124`). Duplicate passives are deliberately exercised in tests, so uniqueness is not a safe assumed rule.
- Actives: vector of IDs, 0–4, role restrictions; duplicate active IDs are not forbidden by current contract.
- Exactly one weapon ID, matching derived ship size; exactly one missile ID. Dedicated fifth special slot is derived from role, not an equipable ID.
- Hangar: `[Option<Id>; 4]`, unique held ship IDs, resolve to ships owned by hangar owner. No persisted examples in repository; player objects occur only as transient test/client constructions.
- `Catalog::load` (245–261) deserializes eight JSON arrays **without calling validate**. `validate` (270–340) checks catalog facets; `validate_loadout` (342–388) checks equipped references/ranges; `effective_stats` (390–403) computes only; `buy` (405–419) constructs stock loadout without money or persistence.
- Runtime rules C12–C16 are prose: deployment; destruction at armor zero; cloak breaking on damage; normalized heat/cooling/overheat lockout; energy regeneration and ability costs. No present server implementation.

## Architecture

`ontology/domain.md` → typed serde structs/enums in `ontology/model.rs` ← JSON instance arrays. The model is the `stargem-ontology` crate's nonstandard library root (`ontology/Cargo.toml:6–7`). `sim` and `client` import it under dependency alias `ontology`.

Only live catalog consumer is `client/src/main.rs:12–20`: load catalog → choose CLI model (default hydra) → construct stock player ship → compute stats → derive flight parameters and hull radius. It never calls catalog/loadout validation. `sim/src/lib.rs:54–68` uses speed/agility and size; other combat stats only appear as static HUD numbers (`client/src/main.rs:90–91`). Flight/collision run at client frame time (`client/src/main.rs:54–64`), despite `sim::step` describing a fixed-step integrator. There is no authoritative server crate, netcode, combat state, hangar UI or purchasing service in current tracked tree.

`sim/src/lib.rs:54–57`: degrees converted to radians, fixed accel 6 and drag 1.5. `61–68`: sphere radii frigate 12m/fighter 7m/interceptor 4m. `70–112`: exponential velocity chase plus sphere obstacle collision. `client/src/main.rs:25–40`: one shared Mjolnir glTF placeholder and 24 generated obstacles; failed art load falls back to wireframe at 85–87.

### Conventions and prior intentional deviations

- Source-of-truth priority explicit in `README.md:3–4` and `domain.md:3–6`. IDs stable kebab-case; JSON property names snake_case; serde enum values kebab-case; domain docs use property/type/note and relation/cardinality tables, C1–C17 identifiers, Q1–Q10 traceability. No separate glossary, citations, changelog, provenance schema or audit template exists among tracked documents.
- Narrative explicitly selects arcade rather than Newtonian motion (`ontology.md:52`), fixed per-type passive counts (`34–38`), and a role-derived special outside four actives (`24,44`). Treat these as local design choices, not defects merely because another game differs.
- Scope explicitly excludes auth, matchmaking, history, most economy, progression, station interiors, AI and maps (`domain.md:18–23`); faction/tier and wave/operation generators deferred (`256–260`). Absence is not evidence of live-game contradiction.
- Explicit `ponytail:` limitations: fixed accel/drag pending per-ship balancing (`sim/src/lib.rs:56`); spherical hulls pending meaningful hitboxes (`96`); shared art pending per-ship assets (`client/src/main.rs:25`).
- Historical evidence: commit `26cb6d6599a1f07abf99ff37851b676908fc6f0f` (2026-09-06) introduced ontology from narrative; `00159bd85832187b53f861b71ebe008e5da690d3` (2026-09-06) says complete reboot retaining ontology, replacing prior system with raylib/parry3d and removing protos/stale server/client submodules. Do not report deleted systems as current functionality.
- Baseline commit renamed `onthology.md` → `ontology.md` without content changes. `domain.md:4` still points at `../onthology.md`: verified stale internal reference caused by historical rename, not a gameplay error.

## Findings to carry into the audit

Labels deliberately distinguish local proof from external uncertainty. No verified contradiction with live Star Conflict is established by this stage.

1. **Internal logical issue — thermal validation weaker than definition.** `domain.md:246` says thermic multipliers both approximately 1; `model.rs:292` checks only that their difference is below .2. Equal values 100/100 would pass the profile check. Current instances are 1/1 and do not violate the definition.
2. **Internal logical issue — C5 checking attribution.** `domain.md:242` assigns positivity checking to `effective_stats`; `model.rs:391–403` merely sums and returns possibly nonpositive stats. Actual range checking is in `validate_loadout:382–386`, and client bypasses it (`client/src/main.rs:14–18`). Current stock data are positive; malformed catalog/loadout handling remains a validation-boundary question.
3. **Internal logical issue — passive count overflow.** `model.rs:352–362` counts an unbounded deserialized vector into u8. 256 repeats of one known passive can panic in checked builds or wrap in unchecked builds, undermining C4 validation. This is a static code finding, not a executed reproducer here.
4. **Coverage gap — catalog uniqueness.** `model.rs:277–282` checks six string-ID families, not damage/special enum-ID arrays. `283–307` checks presence, not uniqueness, of these latter families. Duplicate damage/special rows can survive validation; clarify whether C1's scope includes these non-catalog-item classes, while role↔special identity still needs an unambiguous definition.
5. **Coverage gap — validation is opt-in.** Catalog fields are public mutable vectors (`model.rs:232–242`), load does not validate and the client doesn't validate. C3 intentionally assigns immutability to the future server (`domain.md:240`), so Rust mutability is not itself proof of a semantic contradiction. `buy` does not charge price: server-owned C10 behavior is unimplemented, not a hidden economy implementation.
6. **Uncertainty — numeric edge cases and ID grammar.** Bare `String` IDs (`model.rs:8`) do not enforce documented kebab-case/stability. Most numeric checks are comparisons rather than finiteness checks; public Rust structs allow NaN/infinity even though ordinary JSON does not encode those literals. Assess realistic trust boundaries before severity; no demonstrated bad persisted instance.
7. **Coverage gap — runtime competency questions.** Q9/Q10 have declarative answers but no runtime enforcement; modes contain no objective rules, missiles no typed distinct guidance, specials/actives mostly prose effects. This is incomplete executable coverage, not evidence of wrong domain scope.
8. **Uncertainty — damage spillover.** `domain.md:246` describes shield multiplier then remainder to armor without an explicit raw-damage conservation formula. Resolve whether remainder means raw-equivalent or already-scaled damage before future simulation work; no implementation currently settles it.
9. **Uncertainty — activation vocabulary.** Phasic shield is modeled as toggle yet described as cycling three resistance types (`special_modules.json:4`); command shield toggle adds specificity beyond narrative press activation. Distinguish action input from state transition during research; do not silently change enum meanings.
10. **Coverage gap / provenance uncertainty.** Catalog names, roles, module effects and exact balance numbers have no citations or source-version metadata. Narrative's “multiple weapon types per size” (`ontology.md:51`) is represented by only one interceptor example. A partial example catalog is not automatically a contradiction or a completeness claim.
11. **Historical change — broken narrative link.** The rename described above conclusively explains `domain.md:4`; no inference about external game history follows.

## Seven-step audit coverage checklist

- [x] **1 Domain/scope:** captured users, inclusions/exclusions and all ten Qs (`domain.md:8–38`). Remaining product question for report: is fidelity to a dated Star Conflict version intended, or only inspiration for an independent game?
- [x] **2 Reuse:** mapped all current tracked models/consumers; serde plus local Rust model already provides typed/storage representation. No additional ontology package/schema exists in tracked files. External reuse research deferred, not claimed complete.
- [x] **3 Terms:** enumerated sizes/roles/damage/slots/activation/modes and catalog IDs above; narrative terminology differs in spelling (Cover Ops vs covert-ops), not necessarily meaning.
- [x] **4 Classes/hierarchy:** abstract catalog/equipable concepts are documentary; Rust uses separate concrete structs and composition, exhaustive role/size/special enums. Ship has model/loadout, role determines size: no erroneous inheritance needed. Special deliberately outside equipable union. No class cycles observed.
- [x] **5 Properties:** reviewed every model and instance field; intrinsic base stats vs ID relationships distinguished. Runtime state is only prose. External user reference is explicit; user-selected-ship relation lacks a Rust storage object.
- [x] **6 Facets:** traced C1–C17 to types, serde and validators/server obligations. Rust+JSON+runtime-validator alignment substitutes for skill's TypeScript/Zod/SQL example; there is no SQL layer to audit. Found boundary weaknesses above. Execution verification deferred.
- [x] **7 Instances:** all eight files/46 records read, nine role examples and every enum category represented. Four ontology tests inspected; no runtime execution or external realism validation claimed.

### Gruber criteria

| Criterion | Local assessment / audit seam |
|---|---|
| Clarity | Strong explicit units, cardinalities and Q/C traceability; weaker vague effect prose, raw string IDs, “approximately 1,” damage remainder wording and missing provenance. |
| Coherence | Exhaustive mappings and loadout tests help; thermal profile, effective-stat validation boundary and u8 counter are concrete internal seams. Runtime rules remain declarative. |
| Extendibility | Catalog arrays accept new instances without code changes; new sizes/roles/special kinds require enum/match edits. One-special-per-role is deliberate commitment; research variants before proposing relaxation. |
| Minimal encoding bias | Readable string identifiers and composition; f32/u8 and the overflow risk are implementation choices, not domain necessities. Missing slot-map keys map to zero, an encoding convention worth documenting. |
| Minimal ontological commitment | Clearly bounded independent game scope and postponed generators avoid speculative systems. Uniform size/role/special and weapon-size rules make stronger commitments than mere inspiration; compare evidence without treating omitted features as contradictions. |

## Existing checks and safe execution guidance

**Not executed in reconnaissance.** `cargo` is absent from current PATH; `nix` exists at `/run/current-system/sw/bin/nix`. A shallow `/nix/store` inspection found no directly available cargo binary. README recommends `nix develop`, then `cargo test` (`README.md:17–23`). Flake exports default dev shell for x86_64-linux/aarch64-linux with cargo/rustc and raylib build dependencies (`flake.nix:8–22`); lock pins nixpkgs revision `c043004d1c6985732bcc1cbc5a9c9aecbbb4e0f0` (`flake.lock:3–16`).

For a later authorized test stage, use an external managed scratch directory for build output, no source changes:

```sh
cd /home/theta/repos/stargem.nix
# SCRATCH must be an approved managed scratch path outside the repository.
nix develop --offline --no-write-lock-file -c \
  env CARGO_TARGET_DIR="$SCRATCH/target" \
  cargo test --offline --locked -p stargem-ontology -p sim
```

This requires already cached Nix shell closure and Cargo dependencies. If unavailable, record the blocker; do not install/fetch packages under current audit authorization. Shell realization can write Nix caches even with offline mode, so this is guidance, not an action performed here. Full `cargo test --workspace --offline --locked` also builds client/native raylib dependencies; unnecessary for the ontology-specific pass.

Existing tests, seven total:
- `ontology/model.rs:431–437` `instances_satisfy_catalog_constraints`: catalog validator; asserts nine models/four modes.
- `ontology/model.rs:439–461` `bought_ship_is_valid_and_loadout_rules_hold`: stock purchase, passive overfill and addition, role restriction, weapon size.
- `ontology/model.rs:463–471` `hangar_rules`: duplicate plus foreign-owner ship.
- `ontology/model.rs:473–478` `role_tables_cover_every_role`: distinct specials for all nine roles.
- `sim/src/lib.rs:122–132` throttle cap/drag; `134–141` yaw direction; `143–152` collision separation/velocity removal.
- No client unit tests. `client/src/main.rs:44–53` provides `STARGEM_SHOT` frame-120 screenshot-and-exit hook, requiring a graphics context and authorized external output; not an ontology correctness check.

## Recommended research seams

1. Highest leverage: verify each named model's class/role and dated special variants against authoritative ship pages; record source/version, rather than assuming local names imply live-game fidelity.
2. Distinguish actual game class taxonomy from this deliberate three-size/nine-role abstraction. Research omitted classes only as potential coverage expansion, unless product scope expressly promises exhaustive fidelity.
3. Compare damage resistance mechanics, terminology (thermic/thermal, armor/hull), and special activation semantics. Preserve numeric balance values as local unverified design data until provenance is found.
4. Compare equipment slot counts, weapon/missile eligibility, module categories and duplicates, afterburner/emergency-barrier classification. Do not recategorize based on name alone.
5. Confirm mode names, objectives, respawn/deployment semantics and whether modes are generic local scenarios versus named live modes. Economy/progression/factions are explicit deferrals, not automatic defects.
6. Report product decisions without requesting implementation: desired fidelity/date, sample vs exhaustive catalog, whether role-special bijection and fixed slot/weapon rules are intentional long-term commitments, and desired source provenance conventions.

## Start Here

Open `ontology/domain.md:8–38,193–254` first: it establishes what this project actually promises and which rules are delegated to a future server. Then compare `ontology.md:20–55`, `ontology/model.rs:270–419`, and exact instance rows above. Avoid interpreting previous removed systems or asset filenames as current semantic authority.

## Exact tracked file inventory

25 tracked entries: 23 regular files plus 2 gitlinks. No tracked tests outside the two inline Rust test modules. No tracked AGENTS/HANDOFF/graph files. All eight instance files and all three Rust sources have been read in full. `Cargo.lock` was inventoried/line-counted, not read in full; it is dependency pinning, not domain evidence.

| Exact path | Lines / kind | Role and follow-up |
|---|---:|---|
| `.gitignore` | 1–2 | Ignores target/ and .direnv/. |
| `.gitmodules` | 1–6 | Two asset/footage submodule registrations. |
| `Cargo.lock` | 1–706 | Workspace dependency lock; not ontology instances. |
| `Cargo.toml` | 1–12 | Three-member workspace and shared dependencies. |
| `README.md` | 1–25 | Canonical authority, reboot and run conventions. |
| `client/Cargo.toml` | 1–10 | ontology/sim/glam/raylib dependencies. |
| `client/src/main.rs` | 1–95 | Only current catalog runtime consumer; load path 12–20, placeholder art 25–32, frame loop 48–94. |
| `docs/3D/star_conflict_ships.gltf` | gitlink `ba1c8e13824df7f203824946bffc76ff2e7ec9c0` | Art assets, excluded from traversal/content inventory. |
| `docs/footages/star_conflict` | gitlink `4a29f767e579e1be7d6c55b90bb7590a61a95ae1` | Footage assets, excluded from traversal/content inventory. |
| `flake.lock` | 1–27 | Pinned nixpkgs revision. |
| `flake.nix` | 1–26 | Dev shell, native rendering dependencies. |
| `ontology.md` | 1–65 | Narrative: modes 1–4; entities/combat 6–55; build/play environment 57–65. |
| `ontology/Cargo.toml` | 1–11 | stargem-ontology, custom model.rs lib path, serde dependencies. |
| `ontology/domain.md` | 1–275 | Canonical scope 8–23; Qs 25–38; classes/properties 40–189; relations 191–232; constraints 234–254; generators/instance map 256–275. |
| `ontology/instances/active_modules.json` | 1–17 | 5 active modules. |
| `ontology/instances/damage_types.json` | 1–5 | 3 damage profiles. |
| `ontology/instances/game_modes.json` | 1–6 | 4 modes. |
| `ontology/instances/missiles.json` | 1–5 | 3 missiles. |
| `ontology/instances/passive_modules.json` | 1–10 | 8 passives. |
| `ontology/instances/ship_models.json` | 1–38 | 9 ships; exact per-model four-line ranges in table above. |
| `ontology/instances/special_modules.json` | 1–11 | 9 built-in role specials. |
| `ontology/instances/weapons.json` | 1–12 | 5 weapons. |
| `ontology/model.rs` | 1–479 | Enums/maps 8–71; stats/catalog types 73–183; owned objects/hangar 185–228; catalog loader 232–268; validation 270–388; stat/purchase helpers 390–419; tests 422–479. |
| `sim/Cargo.toml` | 1–9 | ontology/glam/parry3d dependencies. |
| `sim/src/lib.rs` | 1–153 | Input/body 12–40; stats conversion/radii 42–68; flight/collision 70–112; tests 114–153. |
