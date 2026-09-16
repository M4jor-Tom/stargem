# Handoff: research-equipment — e54d8f00

Checkpoint: 2026-09-16, user-requested pause. Parent-curated from runtime records and actual files; not a new completed child report.

- **State:** COMPLETED / SUPERSEDED: network-blocked attempt
- **Role:** `delegate`
- **Child run:** `e54d8f00-03b1-4c80-bf7c-bdbacb4ddee9`
- **Workflow / key:** `08aa52d7-d2a2-4091-b856-28049dd1b39f` / `research-equipment`
- **Mission:** `28a33a06-b296-4a78-8ae0-079881816489`
- **Repository:** `/home/theta/repos/stargem.nix`, branch `master`
- **Source baseline:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`
- **Session (machine-local, optional for recovery):** `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/2026-09-16T18-47-01-030Z_01a0ab8b-30a5-761c-90d3-cc64125796e1/50b97a34-dd8d-44e5-9e28-e322e1bf9c02/run-0/session.jsonl`
- **Runtime status (machine-local):** `/tmp/pi-subagents-uid-1000/async-subagent-runs/e54d8f00-03b1-4c80-bf7c-bdbacb4ddee9/status.json`

## Authority and boundaries

Read root `AGENTS.md` and `docs/HANDOFF.md` before work. `ontology/` remains canonical and unchanged. This is documentation/research, not authorization to change gameplay. No commit/push/merge/install is authorized. One writer per checkout. Preserve untracked audit files: they are the current work, not disposable clutter. Other agents' full research is intentionally file-backed to protect parent context.

## Resumption (for Agent)

No remaining work for this attempt. Use [research-equipment-442c87bb.md](research-equipment-442c87bb.md) instead. The user fixed the firewall; do not propagate this attempt's access-blocked conclusion as current.

## Native continuation or fallback

Recheck live availability before any resume. Completed work normally requires no resume.

Not rechecked in the latest retained-child list; no resume is needed. Use the preserved report below. If genuinely necessary, inspect status/children.list first; otherwise start a same-role focused continuation from this handoff.

## Verification and acceptance

The original report below states exactly what this child checked and what remains uncertain. Completion of a research child does not mean that document synthesis, independent evidence review, ontology review or user reconciliation has completed. Follow [the main checkpoint](README.md) for dependency order and pending reviews.

## Preserved original output

Original managed artifact: `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/research/equipment.md`

SHA-256 of original artifact: `d5ae46a020d1937ef978fbbdc07f4d4bd4fa51c26ea948cbb8098a314037e6d4`

The complete output is embedded below so recovery does not depend on session/temp-file retention. Original relative code paths refer to the repository root unless the report says otherwise. Historical access failures stay historical.

---

# Equipment audit — PARTIAL: external verification blocked

- **Audit date:** 2026-09-16.
- **Repository/baseline:** `/home/theta/repos/stargem.nix`, `master`, `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`; HEAD verified unchanged. Working tree and index clean on inspection.
- **Exclusive topic:** active/passive equipment, capacities and eligibility, repeated equipment/stacking, modifiers, quality/upgrades, activation/energy/cooldown, weapons, missile-slot payloads, stock loadouts, and special-module equipability.
- **Authority:** `ontology/domain.md` is canonical; `ontology/model.rs` and instances implement/populate that local design. `ontology.md` is its source narrative, **not verified Star Conflict documentation**.
- **Result:** a complete local equipment trace and a bounded, unsuccessful external retrieval attempt. **Zero retrieved external pages, zero externally verified agreements, zero verified contradictions with Star Conflict, and zero verified historical equipment changes.** This report must not be relabeled a completed live-game audit.
- **Blocker:** all four initial source requests timed out; a neutral network control also timed out. Supervisor confirmed the same failure across research lanes, no known approved proxy, and directed this lane to stop external retries. No browsing/tooling workaround was attempted.
- **Safety:** root `AGENTS.md` and `docs/HANDOFF.md` checked first and absent. Requested ontology skill and supplied inventory read. No repository/source/ontology edits, installs, commits, branches, pushes, nested agents, or gameplay implementation. Only this report and managed scratch evidence were written.

## 1. Method and evidence boundary

Applied Stanford's seven steps and all five Gruber criteria as an **audit**, not permission to redesign the ontology. Local conclusions are based on direct reads, not merely the inventory. Network failures prevented the external halves of the comparison and real-game example validation.

### Navigation angles attempted

1. **Equipment taxonomy and eligibility:** official-hosted wiki Main Page and candidate Equipment page.
2. **Dated changes/announcements:** official Star Conflict home page as the announcement-index entry point.
3. **Product identity and publisher description:** Steam app **212070 / Star Conflict**, not Star Citizen.

Four initial URLs were fetched concurrently with bounded `curl` calls (`--connect-timeout 12 --max-time 35`). The intended pipeline used installed `markitdown` for successfully fetched HTML. **No HTML arrived, so no conversion or source-body reading occurred.** No search snippets were used as evidence; no search engine was queried after the infrastructure blocker became clear. A targeted content gap-closing pass was not possible and was explicitly stopped, rather than replaced with remembered mechanics.

### External source/access registry

All entries below are **UNRETRIEVED**. Titles are requested/discovery labels, not extracted page titles. For **each entry**, attempted retrieval date is **2026-09-16**; article/update/patch date is **unknown**; relevant heading is **unknown/unread**; excerpt is **none**; supported gameplay claim is **none**. Host/type labels identify the intended source class, not proof of any page content or editorial freshness.

| ID | Requested title/label | Exact URL | Intended publisher/type | Exact result | Intended use if access is restored |
|---|---|---|---|---|---|
| U1 | Main Page — Star Conflict Wiki | https://wiki.star-conflict.com/index.php?title=Main_Page | Star Conflict official-hosted, community-maintained wiki | curl exit 28; HTTP `000`; `Connection timed out after 12003 milliseconds` | Resolve actual equipment/category titles and follow primary wiki links; do not assume a guessed title exists. |
| U2 | Equipment — candidate wiki title | https://wiki.star-conflict.com/index.php?title=Equipment | Star Conflict official-hosted, community-maintained wiki | curl exit 28; HTTP `000`; `Connection timed out after 12003 milliseconds` | Equipment families, slots, restrictions, upgrades; page existence and contents unconfirmed. |
| U3 | Star Conflict — official home | https://star-conflict.com/en/ | Official game site / potential announcement index | curl exit 28; HTTP `000`; `Resolving timed out after 12002 milliseconds` | Find dated patch notes or original equipment announcements corroborating wiki descriptions. |
| U4 | Star Conflict — Steam store app 212070 | https://store.steampowered.com/app/212070/Star_Conflict/ | Publisher storefront description hosted by Valve; not itself patch documentation | curl exit 28; HTTP `000`; `Resolving timed out after 12002 milliseconds` | Product identity and broad description only; not sufficient for fine equipment rules. |

**Infrastructure diagnostic, not game evidence:** no proxy-related environment variables were present. `timeout 8 getent hosts wiki.star-conflict.com` subsequently resolved `23.109.154.234`; DNS resolution alone did not demonstrate HTTP reachability. `curl -I -sS --connect-timeout 5 --max-time 8 https://example.com` failed with exit 28, `Connection timed out after 5001 milliseconds`. No direct-IP, IPv4, alternative-host, search-engine, or repeated-host retry followed the supervisor's stop instruction. Another research lane owned any further shared diagnostic.

**Freshness/conflicting-source status:** neither wiki revision dates nor official patch dates were retrieved. No wiki statement can be called current, and no external conflict can be adjudicated. Do not reinterpret the local four-active-module rule, additive passives, or built-in specials as historically accurate or outdated without a dated original source.

**Managed raw failure evidence:** sibling directory `../scratch/equipment/` contains `wiki-main.url`, `equipment.url`, `official-main.url`, `steam.url` with requested URLs and curl outcomes, and corresponding `.headers` files. The header files are empty and no `.html` or converted `.md` bodies exist. A successful tool invocation around the batch does not mean the individual curl requests succeeded; all four reported exit 28.

## 2. Local material read and authority

Directly read in full:

- Requested skill: `/home/theta/.pi-game-dev/git/github.com/M4jor-Tom/claude-ontology-skill/skills/ontology/SKILL.md`.
- Supplied reconnaissance: `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/69d652a5-b7d3-47be-83bd-4c3f7677ee17/audit/inventory.md`.
- `ontology/domain.md`, `ontology.md`, `ontology/model.rs`.
- `ontology/instances/{ship_models,passive_modules,active_modules,weapons,missiles,special_modules}.json`.
- `client/src/main.rs`, to trace the stock-loadout/effective-stat consumer.

Also searched every Rust caller of `Catalog::load`, `buy`, `effective_stats`, and `validate_loadout`, and all `.validate()` calls. No source edits were made. Remaining damage/game-mode material is outside this equipment lane except where existing equipment fields refer to it.

Local provenance for all following claims is **the baseline repository snapshot**, accessed 2026-09-16, not a game patch. Exact names and numbers are local sample data. A locally named ship/module is not established as a faithful counterpart of a real-game object.

## 3. Stanford step 1 — scope and competency questions

**Domain sentence:** determine what equipment this repository lets a player ship carry and how it computes/represents equipment effects, then compare only against independently retrieved Star Conflict evidence when available.

Primary users are content authors, the future authoritative server, and client/loadout UI (`domain.md:10–17`). Progression/ship-tree ranking is expressly out of scope; module purchasing and faction/tier are possible later additions (`domain.md:19–23`). Their absence is not an internal defect or external contradiction.

| Competency question | Local answer and exact seam | External answer |
|---|---|---|
| 1. Which passive families exist and how many can be fitted? | Five families; per-model 0–3 each; fit count at most capacity (`domain.md:88–100,239–241`). | Unverified. |
| 2. Is capacity mandatory occupancy? | No for passives/actives: 0..Σslots and 0..4. Exactly one weapon and one missile ID remain required (`domain.md:171–178,202–205`). | Unverified. |
| 3. Which active modules fit by rank/class/role/model? | Role whitelist only; empty whitelist means any of nine roles. All models share maximum four (`domain.md:110–121,243`). No rank or model eligibility facet. | Unverified. |
| 4. May an equipment identity repeat, and how do repeated effects combine? | Passive IDs may repeat; modifiers add linearly; existing test expressly fits three identical shield extenders. Active duplicates are not rejected (`model.rs:351–370,390–403,445–450`). | Unverified; no general stacking rule may be imported. |
| 5. Are effects flat, percentage, conditional, or temporary? | Passives: flat deltas on six stats only. Actives/specials: effect text; active flow has cost and cooldown but no typed duration/magnitude (`domain.md:73–86,102–121,145–153`). | Unverified. |
| 6. Do quality/upgrades change an equipment instance? | No equipment-quality, upgrade-level, rank, or owned-module instance fields (`model.rs:126–178,186–199`). | Unverified; progression is excluded locally. |
| 7. Does weapon eligibility use more than size? | No: one size per weapon and exact size match (`domain.md:123–133,244`; `model.rs:372–377`). | Unverified. |
| 8. Can a missile slot hold a mine, drone, or nondamaging consumable? | No such variant; `MissileModel` requires positive damage/speed/ammo/reload. This is a structural observation, not evidence these payloads exist in Star Conflict (`domain.md:135–143`; `model.rs:166–175,329–330`). | Unverified. |
| 9. What equipment is stock, and can specials be changed? | Stock weapon+missile only; no ordinary modules at purchase. One special derived from role, never equipped in a loadout (`domain.md:145–147,247,254`; `model.rs:405–419`). | Unverified. |
| 10. Are activation, sustained consumption, duration, cooldown, and ammunition independent? | Two active flow variants distinguish once/per-second costs. Duration is prose only; weapon heat and missile ammo/reload are separate structures (`domain.md:118–143`). No modeled cooldown-start event. | Unverified. |

## 4. Stanford steps 2–5 — reuse, terms, classes, properties

### Existing representation to reuse

The existing `Stats`, `StatModifiers`, `PassiveSlotType`, `ActivationFlow`, `ShipRole`, catalog records, and loadout vectors already express the selected local design. There is no need to add a new ontology library or abstraction for this documentation audit. Any future reconciliation should begin with these types, not create a parallel unofficial model. No external vocabulary/schema could be reviewed over the failed connection.

### Terms and relationships actually represented

| Term | Local meaning / relationship | Cardinality or property distinction |
|---|---|---|
| Ship model | Immutable catalog template; has role, base stats, passive capacities, stock equipment references | One role, one stock weapon, one stock missile; capacities intrinsic to the model. |
| Player ship | Owned object referencing model plus loadout | Many player ships may use one model; equipped IDs are relationships, not intrinsic model stats. |
| Passive module | Catalog item occupying exactly one of shield/armor/capacitor/motor/computer families | One family per module; same ID may occur multiple times; fit capacity is per family. |
| Active module | Catalog item with role eligibility, activation flow, designer-facing effect | Allowed-role list 0..many; empty means unrestricted, not fits-none. Equipped list 0..4. |
| Weapon | Single equipped weapon definition for a size | One required reference; this does not separately model physical turrets, barrels, or hardpoints. |
| Missile model | Single equipped launcher-payload definition | One required reference. `ammo` is per launcher per life, not number of slots. No distinct launcher catalog class. |
| Special module | Built-in role-derived action | Role↔special bijection; separate dedicated fifth slot, not a selectable equipable item. |
| Stats/modifiers | Six-field value objects | Base stats intrinsic; computed effective stats depend on passive relationships. Deltas may be negative. |
| Energy cost / drain | Once on activation versus per-second while on | Strictly positive in both active variants; cooldown is nonnegative. |
| Effect | Text description for actives and specials | Not a typed buff/debuff, duration, condition, or stacking policy. |

**Important vocabulary not represented:** rank eligibility, quality/upgrade state, model-specific equipment eligibility, percentage operator, effect trigger, repeat limit, diminishing-return curve, ammo-clip versus reserve distinction, missile-slot payload variants, and special-module selection. These are audit questions, **not assertions that the actual game necessarily has each feature**.

**Hierarchy check:** local passive/active/weapon/missile classes are members of the equipable-item union; player-ship **has** a model/loadout, not **is** a module. Special is expressly outside the union. No inappropriate inheritance is needed to explain the present data. Whether external evidence requires expanding these boundaries remains unknown.

### Exact slot capacities in the current instances

Tuple order is **shield, armor, capacitor, motor, computer**. These numbers are capacities, not required occupancy. All nine local models separately have at most four ordinary active modules, exactly one weapon reference, one missile reference, and the role-derived special. There is no rank/class/model-varying ordinary active capacity.

| Local model | Local role | Passive capacity tuple | Total passive capacity | Local stock weapon / missile | Anchor |
|---|---|---|---:|---|---|
| anaconda | engineer | 3,1,2,1,1 | 8 | heavy-laser / cruise-missile | `instances/ship_models.json:2–5` |
| styx | long-range | 2,2,2,1,2 | 9 | heavy-railgun / cruise-missile | `instances/ship_models.json:6–9` |
| mammoth | guard | 1,3,2,1,1 | 8 | heavy-laser / cruise-missile | `instances/ship_models.json:10–13` |
| hydra | tackler | 2,2,1,2,2 | 9 | assault-railgun / homing-missile | `instances/ship_models.json:14–17` |
| phoenix | gunship | 1,3,2,2,1 | 9 | assault-railgun / homing-missile | `instances/ship_models.json:18–21` |
| lynx | command | 3,1,2,1,2 | 9 | pulse-laser / homing-missile | `instances/ship_models.json:22–25` |
| shrike | covert-ops | 1,1,2,3,1 | 8 | plasma-gun / homing-missile | `instances/ship_models.json:26–29` |
| wolfhound | recon | 2,1,1,3,2 | 9 | plasma-gun / homing-missile | `instances/ship_models.json:30–33` |
| kite | ecm | 2,1,2,2,2 | 9 | plasma-gun / homing-missile | `instances/ship_models.json:34–37` |

Paths in this table are relative to `ontology/`. Model slot maps missing a family implicitly mean zero (`model.rs:124`). Local type/facets permit up to **15** total passive slots (five families × three), but shipped examples use eight or nine; there is no additional total-capacity rule. The four-slot **hangar** is an unrelated ship-deployment relationship, not an equipment slot count.

### Exact passive modifiers and repeated-equipment semantics

`ontology/instances/passive_modules.json:2–9`:

| ID | Family | Exact local flat delta |
|---|---|---|
| shield-extender | shield | shield HP +1500 |
| reinforced-hull | armor | armor HP +1800; speed −10 m/s |
| light-armor | armor | armor HP −600; speed +30 m/s |
| capacitor-bank | capacitor | energy capacity +150 |
| energy-recycler | capacitor | energy regeneration +12/s |
| speed-booster | motor | speed +40 m/s |
| gyro-stabilizer | motor | agility +8 deg/s |
| targeting-cpu | computer | agility +4 deg/s; energy regeneration +3/s |

All six modifiers default to zero; no percentage field, conditional trigger, duration, or direct weapon-stat modifier exists (`model.rs:83–93`). Passive family controls slot occupancy, not which stat can change: armor affecting speed and computer affecting regeneration are expressly representable. A `model_code` string is a code, **not a documented quality or upgrade level**; do not infer a progression system from the sample `-1` suffixes.

The repeated-passive rule is not merely accidental: `model.rs:445–450` tests three identical shield extenders and expects `9000 + 3 × 1500`. There is no duplicate-specific attenuation or effect-priority step in `effective_stats`. Active duplication is different evidence: its validator simply lacks a uniqueness rule; there is no corresponding duplicate-active test or runtime effect implementation.

### Exact active flows, distinct from effect durations

`ontology/instances/active_modules.json:2–16`:

| ID | Allowed roles | Flow/cost | Cooldown | Effect description and missing structure |
|---|---|---|---|---|
| shield-boost | empty = any | one-shot, 120 energy | 25 s | Instant shield restoration; amount absent. |
| remote-repair | engineer | ongoing, 25 energy/s | 10 s | Targeted ally shield healing while on; healing rate/range absent. |
| afterburner | empty = any | ongoing, 20 energy/s | 5 s | Raises speed cap; increase absent. |
| emergency-barrier | guard, command | one-shot, 200 energy | 60 s | Text says absorbs all damage for **3 seconds**; that duration is distinct from cooldown and not a typed field. |
| engine-suppressor | tackler | one-shot, 100 energy | 30 s | Slows target “for a short while”; strength/duration absent. |

`domain.md:120–121` defines one-shot as press/pay once/wait cooldown; ongoing as toggle/drain while on/cooldown before re-enabling. It does not say whether ongoing cooldown starts on activation or on deactivation, whether it runs while enabled, or how involuntary loss of energy changes it. C16 constrains payment but does not resolve those timing questions (`domain.md:253`). Cooldown **can be zero**; the existence of a cooldown field must not be reported as “every module always has a positive delay.” Active cost/drain **cannot be zero** under the present positive-value facet (`model.rs:333–336`).

These are local selected mechanics. In particular, this report does **not** verify that the real game categorizes “Afterburner” or “Emergency Barrier” the same way or gives them these costs/effects.

### Weapons, missile-slot representation, specials, and stock

- Five local weapons: two frigate, two fighter, one interceptor (`weapons.json:2–11`). Eligibility is size-only; no role/model/rank/quality field. A single `weapon_id` does not by itself claim that a ship has only one physical gun mount. Damage, fire rate, heat, cooling, and overheat penalty are fixed per definition (`model.rs:153–164`); no quality-dependent variation is represented.
- Three local missiles, all available to all local models because validation only resolves the ID (`model.rs:379–380`): homing-missile `(1200 damage, 600 m/s, 6 ammo, 8 s reload)`, cruise-missile `(3500, 400, 4, 15)`, emp-missile `(900, 700, 8, 6)` (`missiles.json:2–4`). Names do not supply executable homing/EMP behavior. There is no guidance/status-effect field, mine/drone/consumable union, or payload role/rank/model restriction.
- A single required missile-model reference combines the documented idea of “one launcher loaded with one model” into one ID (`domain.md:178,245`). Positive speed/damage constraints make a stationary or purely supporting payload impossible **if such a payload were later required**; this does not establish its presence in Star Conflict.
- Stock equipment is model-defined weapon+missile, no passives or actives (`domain.md:99–100,247`; `model.rs:406–418`). This is a purchase-default rule, not a maximum-loadout or mandatory-module-fill rule. Stock reference validity and stock weapon-size compatibility are checked by `Catalog::validate` (`model.rs:314–322`).
- Special modules are explicitly not equipable: role selects one special, absent from `Loadout` (`domain.md:51,145–147,196,254`; `model.rs:37–49,186–192`). Their activation field is press/toggle, **not** the active module's energy/cooldown flow (`model.rs:102–108`). The local distinction must not be erased by calling every special a fifth ordinary active module. No choice, replacement, upgrade, or stock-special override is modeled.

## 5. Stanford step 6 — facets and reconciliation candidates

### Classification policy

- **Verified contradiction:** opposing local and retrieved external claims. **None established.**
- **Deliberate simplification:** a clear local design commitment; not automatically a mistake.
- **Coverage gap:** a question/shape the local model cannot answer. Missing scope does not establish an invalid model.
- **Uncertainty:** ambiguous intent, representation, timing, or unavailable source evidence.
- **Historical change:** requires dated before/after evidence. **None established for this topic.**
- **Internal logical issue:** conflict inside documentation/code independent of Star Conflict evidence.

The following is a **candidate reconciliation ledger**, not confirmed game drift. Confidence distinguishes high-confidence local inspection from unknown live-game correspondence. External source cells deliberately say unavailable rather than filling in remembered mechanics.

### E01 — capacity versus occupancy and variable slot layouts

- **Exact local claim:** `domain.md:98` says slots are “fixed per model,” 0..=3 per passive family; `domain.md:176,243` and `model.rs:204,365` set active maximum four for every model. `domain.md:202–205` gives 0..Σslots passives, 0..4 actives, exactly one weapon and missile.
- **External sourced claim:** none; U1/U2 failed. No rank-, role-, or class-based live capacity table retrieved.
- **Classification/confidence:** deliberate simplification; high local confidence, external unknown. Rank progression is expressly outside scope (`domain.md:19–23`).
- **Impact:** if rank/model-dependent fitting is desired later, the fixed active maximum cannot answer it. Empty passive/active lists are already valid; “has four slots” must not become “must equip four modules.”
- **Reconciliation question/options:** retain a fixed four-slot independent-game design, or add capacity/eligibility only after a dated class/rank/model table is verified. Resolve whether equipment-slot unlocks are in scope before adding rank.

### E02 — repeated passives and repeated actives are different commitments

- **Exact local claim:** C5 is `base + Σ modifiers` (`domain.md:242`); `model.rs:445–450` intentionally accepts three identical shield extenders. Active validator (`model.rs:364–370`) checks count and role only, allowing four copies of an unrestricted known active.
- **External sourced claim:** none; no original stacking/duplicate-module rules retrieved.
- **Classification/confidence:** deliberate passive simplification; active-repeat policy uncertainty. High confidence about what local validation permits, unknown live correspondence.
- **Impact:** unique modules, copy limits, nonlinear stacking, or mutually exclusive effects cannot be represented as policies. Duplicate active IDs also do not specify independent cooldown instances versus shared cooldown; runtime has not settled this.
- **Reconciliation question/options:** keep unlimited repeats within slot capacity and linear passive sums, or explicitly select per-item copy/stacking policies after evidence. Do not infer an all-equipment uniqueness rule from an unrelated real-world example.

### E03 — flat deltas are not percentage bonuses or effect rules

- **Exact local claim:** modifiers are “additive deltas” (`domain.md:85–86`; `model.rs:83–93`), applied directly to six stats (`model.rs:390–403`). Example: `light-armor` is −600 armor and +30 speed (`passive_modules.json:4`).
- **External sourced claim:** none; no bonus formula or module tooltip retrieved.
- **Classification/confidence:** deliberate simplification; potential coverage gap if broader effect semantics are required. High local confidence.
- **Impact:** a percent bonus cannot be honestly encoded as a constant delta for every ship; ordering, caps, resistance changes, triggered defensive effects, and weapon-stat changes have no semantics here. Current local data do not claim to be percent bonuses.
- **Reconciliation question/options:** retain flat balancing, or first define exactly which operator/base/order/trigger/duration distinctions are needed. Do not silently convert numbers to percentages or claim live values.

### E04 — module quality/upgrades are absent, not contradicted

- **Exact local claim:** passive/active/weapon records have no quality/upgrade fields (`model.rs:126–164`); loadouts store definition IDs, not owned module instances (`model.rs:186–192`). Progression and additional economy are excluded/deferred (`domain.md:19–23`).
- **External sourced claim:** none; quality names, quality count, upgrade paths, rank ranges, and historical changes unverified.
- **Classification/confidence:** explicit out-of-scope/deferred area plus potential coverage gap; high local confidence. Not an internal logical issue.
- **Impact:** no representation of the same module at different improvements or mutable owned-equipment state. A new catalog ID could represent a separately authored variant, but no quality/upgrade relationship currently exists.
- **Reconciliation question/options:** leave progression out; distinguish definition variant from player-owned upgrade state only if product scope expands and source evidence warrants it.

### E05 — energy cost, sustained drain, duration, cooldown, and trigger differ

- **Exact local claim:** two active variants at `domain.md:118–121`; positive cost/drain but nonnegative cooldown at `model.rs:333–336`. `emergency-barrier` has one-shot 200 energy, 60 s cooldown, and text-only 3 s absorption (`active_modules.json:11–13`).
- **External sourced claim:** none; no source confirms actual-game activation type, timing, or categorization of these names.
- **Classification/confidence:** deliberate two-flow abstraction with internal timing/coverage uncertainty; high local confidence.
- **Impact:** one-shot payment must not be read as instantaneous duration. Ongoing cooldown start, zero-cost effects, triggers, channeling, charges, and independent durations are either unspecified or unrepresentable. It would be unsafe to implement timing directly from the current short effect strings.
- **Reconciliation question/options:** document a local timing interpretation or introduce only the needed missing dimensions later. Specifically verify the actual classification of Afterburner and Emergency Barrier before making any recategorization proposal; the names alone are not evidence.

### E06 — weapon fit is size-only; weapon identity is not hardpoint count

- **Exact local claim:** `weapon.size` says “only ships of this size may equip it” (`domain.md:127`); C7 enforces size equality and exactly one weapon (`domain.md:244`; `model.rs:372–377`). All other restriction axes are absent.
- **External sourced claim:** none; no live role-, rank-, model-, faction-, or special-weapon eligibility retrieved.
- **Classification/confidence:** deliberate simplification; possible coverage gap. High local confidence, external unknown.
- **Impact:** cannot restrict two same-size ships differently, share one weapon definition across several sizes, or model a per-model exclusive. One equipped weapon type is not evidence of one physical barrel/turret.
- **Reconciliation question/options:** retain size-only policy; if fidelity is required, verify a general rule and at least one explicit exception before broadening eligibility. Keep equipped definition count distinct from physical mounts.

### E07 — missile reference is narrower than a general payload slot

- **Exact local claim:** exactly one missile-model (`domain.md:178,245`); damage/speed/ammo/reload all positive (`domain.md:139–143`; `model.rs:329–330`); ID existence is the only fit check (`model.rs:379–380`).
- **External sourced claim:** none. Mines, drones, and non-missile consumables are requested research targets, **not verified game mechanics in this report**.
- **Classification/confidence:** local missile-only simplification; potential payload coverage gap. High confidence in present schema limits.
- **Impact:** any confirmed stationary, nondamaging, healing, or deployable payload would not fit faithfully by merely changing a name. Inventory/resupply/clip behavior and optional empty payload slots are also not represented.
- **Reconciliation question/options:** preserve a missile-only game; otherwise establish actual slot/payload terminology and restrictions first, then decide whether a general payload category is in scope. Do not “fix” a positive-speed field to zero without revisiting what the class means.

### E08 — stock loadout and special equipability are separate rules

- **Exact local claim:** C10 creates stock weapon/missile and “no modules” (`domain.md:247`); C17 derives an unchangeable special from role (`domain.md:254`). `buy` constructs empty ordinary-module vectors (`model.rs:413–417`); role→special mapping is hardcoded (`model.rs:37–49`).
- **External sourced claim:** none; no purchase-default screenshot/table or selectable-special original page retrieved.
- **Classification/confidence:** deliberate local commitment; high confidence. “No modules” should be read in context as no ordinary equipped passive/active modules, not absence of the built-in special.
- **Impact:** changing the special for a specific model/player cannot be expressed; neither can purchase defaults containing ordinary modules without changing `buy`/schema. These are not current defects simply because other designs are imaginable.
- **Reconciliation question/options:** keep role identity fixed and stock ordinary-module lists empty; or verify concrete stock and special-selection examples before reconsidering the commitment.

### E09 — narrative phrasing can be misread as mandatory module fill

- **Exact local claims:** `ontology.md:40–41` says “filling the ship model's per-type slots” and “4 ActiveCombatModules”; the same narrative says “up to 4” at line 60. Canonical `domain.md:175–176,202–203` permits zero, and `buy` uses empty lists.
- **External sourced claim:** not needed; this is an internal wording comparison.
- **Classification/confidence:** internal clarity uncertainty, **not an unresolved canonical logical contradiction**. High confidence; canonical priority resolves the operational rule.
- **Impact:** a future writer/implementer using only the narrative could mistakenly require full occupancy.
- **Reconciliation question/options:** preserve the canonical upper-bound interpretation; a future authorized documentation edit could distinguish capacity from occupancy explicitly. No edit made here.

### E10 — passive fit counting can overflow before enforcing the slot bound

- **Exact local claims:** C4 promises count ≤ capacity (`domain.md:241`). `model.rs:351–357` accumulates an unbounded deserialized `Vec<Id>` into a `u8` count, then compares with capacity at lines 360–362.
- **External sourced claim:** not applicable; this is an internal implementation/constraint issue.
- **Classification/confidence:** **internal logical issue**, high static confidence. Rust execution not performed.
- **Impact and concrete counterexample:** 256 copies of known `shield-extender` in an anaconda loadout exceed its three shield slots. Checked overflow can panic; unchecked `u8` wrapping can yield count zero. Effective shield HP remains positive at `9000 + 256 × 1500 = 393000`, so the later positive-stat check need not reject that input. The exact runtime outcome depends on overflow-check settings; it is not claimed as an executed exploit. Current normal sample loadouts are unaffected.
- **Reconciliation question/options:** future authorized repair could count with the vector's natural size type or reject excessive vectors before narrow accumulation, with a boundary test. This is a validator correctness question, not permission in this audit to alter code or external game semantics.

### E11 — effective-stat validation is attributed to the wrong boundary

- **Exact local claims:** C5's “checked by” column names `Catalog::effective_stats` (`domain.md:242`). Actual `effective_stats` only resolves IDs and sums (`model.rs:391–403`); positive-range checking is in `validate_loadout` (`model.rs:382–386`). `client/src/main.rs:14–18` loads/buys/computes without invoking either catalog or loadout validation.
- **External sourced claim:** not applicable.
- **Classification/confidence:** **internal logical issue / validation-attribution gap**, high confidence from complete call-site trace.
- **Impact:** a public call to `effective_stats` can return `Some` with invalid stats or an overfilled loadout. Example arithmetic: hydra with eight `light-armor` references gives armor `4500 − 8 × 600 = −300`, yet IDs resolve and summation succeeds. Such a loadout is not C4-legal and would be rejected by a separately called `validate_loadout`. The current client creates an empty stock loadout, so no current stock failure is alleged.
- **Reconciliation question/options:** decide whether this helper is deliberately pure calculation with a validation precondition (then correct documentation/call boundaries later), or promises to reject invalid results itself. Do not confuse an absent future server with a currently implemented authority layer.

## 6. Stanford step 7 — examples and local validation

### Three representative worked local examples

These are **real records in this repository, not externally verified real-game examples**. The requested three representative Star Conflict examples could not be verified. Do not promote the following local role mappings, stock equipment, or numbers into live-game documentation.

1. **Anaconda / repeated shield modules** (`ship_models.json:2–5`, `passive_modules.json:2`, `model.rs:445–450`). The local engineer has three shield slots. Three repeated `shield-extender` IDs produce **13,500** shield HP from base 9,000; four exceed local capacity. It can legally have zero ordinary modules at purchase, with heavy-laser and cruise-missile. This validates linear repetition and upper bounds, not mandatory filling.
2. **Hydra / cross-family stat effects and active eligibility** (`ship_models.json:14–17`, `passive_modules.json:4`, `active_modules.json:5–7,14–16`). The local tackler has two armor slots. Two `light-armor` copies change base armor 4,500→**3,300** and speed 260→**320 m/s**. `engine-suppressor` includes tackler; `remote-repair` allows engineer only. These are local rules even if a real ship named Hydra has different attributes.
3. **Kite / stock references and separate special** (`ship_models.json:34–37`, `weapons.json:10–11`, `missiles.json:2`, `model.rs:48,413–417`). The local ECM interceptor starts with plasma-gun and homing-missile, no passive/active IDs; its role supplies em-surge independently. The homing-missile catalog lists six rounds and eight-second between-launch reload (`domain.md:142–143`), not six missile slots. No factual claim is made about real Kite stock equipment.

### Read-only runnable check performed

Managed helper: `../scratch/equipment/check_local.py`; output: `../scratch/equipment/local-validation.txt`.

Executed:

```sh
python3 /home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/scratch/equipment/check_local.py
```

The standard-library check reads existing JSON and does not mutate repository data. It checked:

- Unique IDs within the five equipment/model families read by the helper; stock references exist and stock weapons match the locally derived size.
- Slot counts 0–3 and modifier fields confined to the six local stats.
- **All 7,686 capacity-legal passive multisets** for all nine local ships, including repeated IDs and empty choices, have positive effective stats under the local arithmetic/facets.
- The concrete repeated-shield and light-armor arithmetic above, the negative-stat overfilled example, and the overflow example's numerical consequences.

Observed summary:

```text
PASS: 9 ships, 8 passives, 5 actives, 5 weapons, 3 missiles; 7686 capacity-legal passive multisets have valid effective stats.
PASS: worked arithmetic examples. No Rust code, live gameplay, or external facts were verified.
```

Per-model multiset counts: anaconda 432; styx 972; mammoth 720; hydra 972; phoenix 1,440; lynx 648; shrike 720; wolfhound 810; kite 972. This check supports that the **shipped passive catalog is internally usable under the chosen local rules**; it does not execute Rust overflow behavior, reproduce active timing, prove external accuracy, or replace the Rust validator tests.

`cargo` and `rustc` were not available in PATH. No environment realization, installation, dependency download, build, or Rust test run was attempted. Existing relevant tests were read at `model.rs:431–461`; no repository test was added/updated.

## 7. Agreed facts, boundaries, and Gruber assessment

### Internally agreed facts — not external agreements

- Canonical document, Rust model, and example data share the five passive families, role-whitelisted actives, size-limited weapons, and required weapon/missile references.
- Cross-family effects (armor changing speed) agree with the source narrative (`ontology.md:37–38`) and passive instances.
- Empty passive/active vectors from purchase agree with canonical maximum-cardinality rules; the source narrative's “up to” wording agrees too.
- The existing repeated-shield unit test and `effective_stats` agree on **flat addition**.
- Local initial stock references/weapon sizes and all capacity-legal sample passive combinations passed the separate arithmetic/reference check.
- Special modules remain separate from ordinary active modules consistently across role mapping, loadout shape, and C17.

**There are no verified agreements with actual Star Conflict in this report**, because no external body was obtained.

### Gruber criteria

| Criterion | Local assessment | Audit consequence |
|---|---|---|
| Clarity | Units and main cardinalities are explicit. “Four modules” versus “up to four,” slot versus launcher/ammo, role-derived special versus equipable active, and cooldown versus duration require careful wording. Effects and provenance are less precise. | Keep local vocabulary and capacity/occupancy separate in research notes; do not infer external facts from names. |
| Coherence | Current instance arithmetic is coherent for 7,686 legal passive multisets. C4's `u8` accumulation and C5's attributed validation boundary remain internal issues. | These two issues can be reviewed independently of the internet blocker; they do not establish game drift. |
| Extendibility | New catalog items fit existing vectors readily; percent/triggered effects, per-model eligibility, qualities, payload variants, or selectable specials are not expressible without changing the model. | Treat expansion as a product decision grounded in actual evidence, not automatic ontology repair. |
| Minimal encoding bias | Readable IDs and role/family enums are useful. `u8` counter overflow is an implementation limit, not a natural equipment rule; the single missile ID conflates the conceptual launcher/payload relationship. | Distinguish storage choices from game facts, and avoid interpreting ID/name suffixes as quality levels. |
| Minimal ontological commitment | Scope exclusions appropriately avoid speculative progression/economy. Conversely, uniform active maximum, size-only weapons, positive projectile speed/damage, and role-special bijection are strong explicit commitments. | Preserve them as local design unless a fidelity goal and reliable evidence require reconciliation. A broader actual game would not itself make this bounded ontology incoherent. |

## 8. Unanswered product questions and bounded next research pass

Questions are recorded here, not sent to the user; none authorize implementation:

1. Is this an independent Star Conflict-like design, a sample ontology inspired by the game, or an intended faithful snapshot? If faithful, which patch/version/date?
2. Are local names intended as exact real-game identity references or renamed/rebalanced examples? This controls whether name-level comparisons are meaningful.
3. Should rank-dependent fitting be researched only as context, or should the explicit progression exclusion change later? Do not widen scope by accident.
4. Should capacity remain model-fixed with all ships allowing four actives? Are empty weapon/missile slots meaningful in the chosen product, or deliberately forbidden?
5. Are repeated actives intended to be legal? If so, do repeats have independent state? Passive repetition is already explicitly tested.
6. Is flat additive balancing sufficient, or are any percentage/conditional effects specifically required? Which stat base and stacking order would they use?
7. Is the missile model intentionally missile-only? Which **verified** non-missile payloads, if any, should belong to future scope?
8. Are built-in role specials intentionally immutable? Is any future choice per model, per player ship, or part of owned equipment?
9. Must buy/load stock loadouts match a source version, or are empty ordinary-module defaults a deliberate independent design?
10. Is `effective_stats` intended to be a pure helper requiring prior validation, and what boundary will validate untrusted persisted loadouts?

### Candidate navigation queue — not sources

No entries below were fetched; guessed wiki titles may not exist. They are restart points only:

- The U1 wiki index should resolve actual equipment, ship, and category page titles before following links.
- Public MediaWiki search/API entry point, **UNRETRIEVED**: https://wiki.star-conflict.com/api.php?action=query&list=search&srsearch=equipment&format=json . Endpoint availability/schema was not checked.
- Candidate title URLs, **UNRETRIEVED and unconfirmed**:
  - https://wiki.star-conflict.com/index.php?title=Modules
  - https://wiki.star-conflict.com/index.php?title=Weapons
  - https://wiki.star-conflict.com/index.php?title=Missiles
  - https://wiki.star-conflict.com/index.php?title=Emergency_Barrier
  - https://wiki.star-conflict.com/index.php?title=Afterburner
  - https://wiki.star-conflict.com/index.php?title=Anaconda
  - https://wiki.star-conflict.com/index.php?title=Hydra
  - https://wiki.star-conflict.com/index.php?title=Kite
- U3 official site should supply the actual announcement/patch-index links. Do not invent an official article slug or publication date.

Once infrastructure is independently restored, a bounded pass should obtain a general equipment page; slot/eligibility evidence for three contrasting documented ships; repeated-module/stacking rules; examples of flat/percent/effect behavior; quality/upgrade documentation; active timing examples; weapon restrictions beyond size **if documented**; and original missile-slot/stock/special-selection evidence **if present**. Resolve the discovered pages through official indexes rather than endlessly retrying guesses. Then use one targeted pass for unresolved conflicts; roughly 10–20 strong nonredundant pages is guidance, not a quota.

For each actually useful future source retain: extracted title, exact/final URL, publisher/type, access date, visible publication/update/patch/revision date or unknown, heading, short faithful excerpt or precise paraphrase, exact supported claim, and version limitations. Search snippets are discovery only. Pair wiki claims with dated official material where possible and preserve conflicts rather than silently selecting one. **The present report does not supply those missing source bodies and cannot eliminate the need for renewed external research.**

## 9. Commands, artifacts, and acceptance

### Commands/results

- Initial `pwd`, `test -f AGENTS.md`, `test -f docs/HANDOFF.md`, `git status --short`, `git rev-parse HEAD`, `command -v curl`, `command -v markitdown`: passed; correct clean baseline, both instruction files absent, fetch/conversion commands installed.
- Four concurrent bounded `curl -L --max-time 35 --connect-timeout 12 -sS -D <scratch headers> -o <scratch html> -w '%{http_code} %{url_effective}' <U1–U4>`: all failed, exit 28 / HTTP 000; exact failures above. Conditional markitdown stage not entered.
- Proxy variable-name inspection and `timeout 8 getent hosts wiki.star-conflict.com`: passed; no proxy variables, DNS later resolved.
- Neutral `curl -I -sS --connect-timeout 5 --max-time 8 https://example.com`: failed, exit 28. Supervisor then directed no further retries.
- Direct file reads/grep: completed; relevant local equipment flow and all relevant call sites inspected.
- `python3 <managed scratch>/check_local.py`: passed; output retained, 7,686 passive multisets checked.
- `command -v cargo; command -v rustc`: neither available; Rust tests not run and no packages installed.
- Final `git status --short; git diff --cached --name-only; git rev-parse HEAD`: no working-tree changes, no staged files, original HEAD.

### Reviewer gate

**Review required, not yet performed.** Accept only the local inspection and access-failure evidence as checked findings. Treat all live-equipment comparison/real-example requirements as incomplete. The two internal issues are supported by source inspection and numerical examples, not an executed Rust reproducer. No semantic or gameplay changes were made.

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "not-satisfied",
      "evidence": "Read-only equipment audit artifact produced without widening scope, but requested external Star Conflict verification and three sourced real-game examples remain blocked by outbound timeouts."
    },
    {
      "id": "criterion-2",
      "status": "satisfied",
      "evidence": "Exact local anchors, failed URL/error registry, classification-aware candidate ledger, and repeatable managed scratch check with 7686 legal passive multisets permit independent review of the partial result."
    }
  ],
  "changedFiles": [
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/research/equipment.md",
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/scratch/equipment/check_local.py",
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/scratch/equipment/local-validation.txt",
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/scratch/equipment/wiki-main.url",
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/scratch/equipment/wiki-main.headers",
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/scratch/equipment/equipment.url",
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/scratch/equipment/equipment.headers",
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/scratch/equipment/official-main.url",
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/scratch/equipment/official-main.headers",
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/scratch/equipment/steam.url",
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/scratch/equipment/steam.headers"
  ],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    { "command": "bounded curl requests to U1-U4", "result": "failed", "summary": "All exit 28 / HTTP 000; no external source body obtained." },
    { "command": "bounded curl HEAD https://example.com", "result": "failed", "summary": "Neutral control connect timeout; supervisor confirmed shared blocker and stopped retries." },
    { "command": "python3 ../scratch/equipment/check_local.py", "result": "passed", "summary": "Read-only JSON/reference/arithmetic checks; 7686 legal passive multisets valid." },
    { "command": "Rust tests", "result": "not-run", "summary": "cargo/rustc unavailable; no installations or environment realization authorized/performed." },
    { "command": "git status --short; git diff --cached --name-only; git rev-parse HEAD", "result": "passed", "summary": "Clean worktree/index and original baseline HEAD." }
  ],
  "validationOutput": [
    "PASS: 9 ships, 8 passives, 5 actives, 5 weapons, 3 missiles; 7686 capacity-legal passive multisets have valid effective stats.",
    "Zero external agreements, contradictions, historical changes, or real-game examples verified."
  ],
  "residualRisks": [
    "External research and source freshness remain unverified because outbound access failed.",
    "Two internal validator issues are static findings, not executed Rust reproductions.",
    "Local worked examples must not be mistaken for actual Star Conflict equipment data.",
    "Reviewer acceptance gate remains outstanding."
  ],
  "noStagedFiles": true,
  "diffSummary": "No repository diff; managed partial equipment research report, read-only check, and failed-fetch evidence only.",
  "reviewFindings": [
    "blocker: outbound networking prevented requested original-source research.",
    "ontology/model.rs:351-357 - u8 passive counter can overflow before capacity validation.",
    "ontology/domain.md:242 and ontology/model.rs:391-403 - effective_stats does not perform the positive-range validation attributed to it."
  ],
  "manualNotes": "Supervisor approved reporting PARTIAL and stopping retries. No product questions were sent to the user and no external mechanics were inferred from memory."
}
```
