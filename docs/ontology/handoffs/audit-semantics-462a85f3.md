# Handoff: audit-semantics — 462a85f3

Checkpoint: 2026-09-16, user-requested pause. Parent-curated from runtime records and actual files; not a new completed child report.

- **State:** COMPLETED; reuse output, do not rerun by default
- **Role:** `reviewer`
- **Child run:** `462a85f3-e2a8-4700-99c4-90f5fca5f734`
- **Workflow / key:** `08aa52d7-d2a2-4091-b856-28049dd1b39f` / `audit-semantics`
- **Mission:** `28a33a06-b296-4a78-8ae0-079881816489`
- **Repository:** `/home/theta/repos/stargem.nix`, branch `master`
- **Source baseline:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`
- **Session (machine-local, optional for recovery):** `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/2026-09-16T18-47-01-030Z_01a0ab8b-30a5-761c-90d3-cc64125796e1/4b683275-8083-4390-b00b-edc1060c640c/run-0/session.jsonl`
- **Runtime status (machine-local):** `/tmp/pi-subagents-uid-1000/async-subagent-runs/462a85f3-e2a8-4700-99c4-90f5fca5f734/status.json`

## Authority and boundaries

Read root `AGENTS.md` and `docs/HANDOFF.md` before work. `ontology/` remains canonical and unchanged. This is documentation/research, not authorization to change gameplay. No commit/push/merge/install is authorized. One writer per checkout. Preserve untracked audit files: they are the current work, not disposable clutter. Other agents' full research is intentionally file-backed to protect parent context.

## Resumption (for Agent)

Local audit is complete. Reconcile its findings against issues.md/questions.md; retain distinctions between contradiction, ambiguity and explicitly deferred runtime behavior. Only resume for a focused unresolved interpretation. This audit is not the pending independent review of the new documents.

## Native continuation or fallback

Native resumability was verified with children.list at checkpoint. Recheck it in the next session; retained session availability can change.

```javascript
subagent({ action: "status", id: "462a85f3-e2a8-4700-99c4-90f5fca5f734" })
subagent({ action: "children.list" })
// Only when resumable, and no original writer is still active:
subagent({ action: "resume", id: "462a85f3-e2a8-4700-99c4-90f5fca5f734",
  message: "Read AGENTS.md, docs/HANDOFF.md and docs/ontology/handoffs/audit-semantics-462a85f3.md. Continue ONLY the remaining work recorded there; preserve completed outputs and current files." })
```

If the retained session is missing or not resumable, do not claim a native resume or rerun all research. Launch a fresh `reviewer` as a **same-role fallback continuation**, give it this handoff and the preserved reports, and start at the exact next step above. A resumed run may return a new run ID: record it and continue from the latest identity.

## Verification and acceptance

The original report below states exactly what this child checked and what remains uncertain. Completion of a research child does not mean that document synthesis, independent evidence review, ontology review or user reconciliation has completed. Follow [the main checkpoint](README.md) for dependency order and pending reviews.

## Preserved original output

Original managed artifact: `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/audit/semantics.md`

SHA-256 of original artifact: `312d9463da269014313d9cef028e72d7bb05defb952324c2badb3b998c11ac69`

The complete output is embedded below so recovery does not depend on session/temp-file retention. Original relative code paths refer to the repository root unless the report says otherwise. Historical access failures stay historical.

---

# Local semantic audit

**Repository:** `/home/theta/repos/stargem.nix`  
**Baseline:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`, branch `master`  
**Audit date:** 2026-09-16  
**Scope:** existing local semantics, Rust representation, JSON instances and current consumers.

## Review

- **Correct:** the central composition model, role mappings, equipment families, units and supplied catalog examples are substantially coherent.
- **Finding:** four P1 validation defects and several P2 contract/documentation issues are detailed below. Current supplied instances do not demonstrate malformed data; the validation defects have concrete counterexamples.
- **Fixed:** nothing; this was read-only.
- **Merge verdict:** **OK with notes** for this audit artifact. This is not an endorsement of release-ready validation or implemented multiplayer combat.

No live Star Conflict contradiction is established. No web research, edits, commands, test execution, installations or nested agents were performed.

## Evidence and method

Root `AGENTS.md` and `docs/HANDOFF.md` are absent, consistent with kickoff and the supplied inventory. Read in full:

- Requested ontology skill and supplied inventory.
- `ontology.md:1–65`.
- `ontology/domain.md:1–275`.
- `ontology/model.rs:1–479`.
- All eight JSON instance files: 46 records.
- `client/src/main.rs`, `sim/src/lib.rs`, README and relevant Cargo manifests.

The inventory supplies baseline/history context; historical Git changes were not independently inspected in this pass. Current semantic conclusions were checked against source.

Authority is explicit: `ontology/` is canonical (`README.md:3–4`, `ontology/domain.md:3–4`). The narrative is design input, not independently established external-game truth.

**Classification used below:**

- **Internal logical issue:** implementation or canonical statements disagree.
- **Coverage gap:** representation does not supply an answer or enforce a stated facet.
- **Underspecification:** multiple results remain compatible with the available specification.
- **Deliberate simplification/deferred implementation:** documented boundaries, not defects merely because functionality is absent.
- **Historical change:** local history explains drift; no external history inferred.

P1 means resolve before release reliance on the affected contract. P2 means report/documentation/design follow-up, not a demand to implement deferred gameplay.

---

## 1. Stanford seven-step assessment

| Step | Assessment |
|---|---|
| **1. Domain and scope** | Purpose, users, explicit exclusions and ten competency questions exist (`domain.md:8–38`). Authentication, progression, most economy, AI, maps and generators are intentionally outside the present model. Runtime invariants are in semantic scope but implementation is deferred. |
| **2. Reuse** | Existing local Rust enums, structs, validators and JSON arrays already provide the required architecture. `sim` and `client` reuse the ontology crate. No replacement schema framework is warranted. External ontology reuse was not researched under this local-only assignment. |
| **3. Terms** | Sizes, roles, equipment families, damage kinds, activation flows, modes, ownership and runtime state are enumerated (`domain.md:40–189`). Narrative “Cover Ops” versus canonical `covert-ops` is terminology normalization, not demonstrated semantic disagreement. |
| **4. Classes/hierarchy** | Composition correctly represents ownership, model selection and equipment. Abstract catalog/equipable categories need not become Rust inheritance. Role/size/special mappings are exhaustive. No inheritance cycle or demonstrated erroneous is-a relationship was found. Instance terminology needs correction; see `SEM-INSTANCE-TERMINOLOGY`. |
| **5. Properties** | Catalog properties, value objects and ID relationships are mostly separated correctly. Family-specific equipment references resolve through family-specific lookups. Runtime effects deliberately remain descriptive. Inverse relationships can be derived without redundant stored arrays. |
| **6. Facets** | Rust/Serde enforce shape, enums, integer widths and four hangar slots. Explicit validators enforce additional ranges, references and equipment rules, but have the defects below. Runtime and persistence obligations remain separate. |
| **7. Instances** | All 46 supplied records were inspected. They cover nine roles/specials, three damage kinds, five passive families, both active flows and three mode categories. Three existing ship examples are traced below. Tests were inspected, not executed. |

### Architecture distinctions

- `catalog-item` is an **abstract conceptual superclass**, not a required instantiated Rust type.
- `equipable-item` is a **disjoint conceptual union** of passive, active, weapon and missile families. Separate concrete Rust structs preserve these distinctions; a generic enum is unnecessary without a generic consumer.
- `Stats`, `StatModifiers` and `Loadout` are **embedded value objects**. Nesting these is appropriate; ID-reference guidance applies to independently identified entities.
- A `PlayerShip` **has a** `ShipModel` reference. It is not a subclass of a particular catalog row.
- A role **has one** size and special. Neither relationship is inheritance.
- A missile launcher is deliberately represented by one missile-model reference, not a separately identified launcher entity (`domain.md:178,245`).
- Special modules are outside equipable families, but that exclusion is not wholly enforced by Rust types; see `SEM-CHECK-ATTRIBUTION`.

### References, cardinalities and inverses

`domain.md:193–210` and `model.rs:188–228,343–380` establish:

- One model, weapon and missile reference per player ship.
- Zero through four active entries.
- Passive multiplicity bounded per family, not merely by total slots.
- Four optional hangar references; duplicates and foreign ownership rejected.
- External user references; authentication/user existence is not a catalog-validation obligation.
- Many models may share a role, stock weapon or missile.
- Empty `allowed_roles` means unrestricted, **not** “allowed for no roles.”
- Missing slot-map keys are treated as zero by `model.rs:124`; supplied models enumerate all five keys.

Inverse relations need not be duplicated:

- Size → roles and special → role follow from `ShipRole::ALL` and the mappings.
- User → owned ships is the inverse of `PlayerShip.owner_id`.
- Model → player ships and module → equipped ships are queries over owned instances.
- No transitive class or relationship closure is required by the current model.

Duplicate passive IDs represent repeated equipment occurrences: the existing test deliberately equips three copies (`model.rs:445–450`). Treating these lists as mathematical sets would lose necessary cardinality. Duplicate active IDs are not currently prohibited; no contrary uniqueness rule should be invented.

### Units

No demonstrated unit conversion mismatch was found.

| Quantity | Declared/used units |
|---|---|
| Shield/armor and modifiers | HP |
| Energy | Capacity; active one-shot costs use energy units |
| Regeneration/ongoing costs | Energy per second |
| Ship/missile speed | m/s |
| Agility | deg/s, explicitly converted to rad/s at `sim/src/lib.rs:54–57` |
| Fire rate | Shots/s |
| Heat | Dimensionless normalized quantity; increment per shot, cooling per second |
| Cooldown/reload/overheat penalty | Seconds |
| Damage multipliers | Dimensionless |

The absence of numeric effect strengths, energy-to-damage conversion and durations is a specification gap, not a proven unit mismatch.

---

## 2. C1–C17 coherence and enforcement

| Constraint | Current assessment |
|---|---|
| **C1** | String-ID uniqueness is checked for six families; stock references resolve. Damage/special enum-ID arrays lack uniqueness checks. Player/loadout/hangar references are necessarily checked by their separate validators, not `Catalog::validate` alone. |
| **C2** | `u8` plus validation enforce 0–3 on supplied slot entries (`model.rs:311–313`). Missing entries mean zero. |
| **C3** | Catalog immutability and permanent ownership require future server lifecycle rules. Public mutable Rust fields alone do not disprove that architecture. Wording incorrectly excludes runtime mutation when read literally. |
| **C4** | Family counting is correct for ordinary input but uses an overflowing `u8` accumulator. |
| **C5** | Additive calculation is correct. Positivity is checked by `validate_loadout`, not the function named in the constraint table. Floating-point exceptional values are inadequately checked. |
| **C6** | At most four actives and role eligibility are checked. Empty restrictions correctly mean unrestricted. |
| **C7** | One string field plus reference lookup and size comparison implement the structural rule. |
| **C8** | One missile field and reference validation implement the static launcher abstraction. Runtime ammunition is deferred. |
| **C9** | EM/kinetic inequalities are checked for selected rows. Thermic checks equality to the other multiplier rather than proximity to one. Shield-to-armor spillover remains ambiguous. |
| **C10** | `buy` constructs correct stock equipment and empty module lists (`model.rs:405–419`). Charging price is explicitly server-owned and absent—not a demonstrated unauthorized-purchase implementation. |
| **C11** | Four slots are type-enforced; duplicate, missing and foreign-owner references are checked (`model.rs:208–225`). Persisted player-ID uniqueness is a future ownership-store precondition, not supplied by this slice validator. |
| **C12** | Category/deployment policy is documented; no deployment service, selected-ship object or station relation is implemented. Deferred. |
| **C13** | Destruction threshold is documented, not implemented. Future damage transitions must settle overshoot/clamping. |
| **C14** | Cloak-breaking rule and cloak instance agree. No damage event implementation exists. |
| **C15** | Heat units, bounds and extra penalty agree structurally; “idle” between discrete shots needs an operational definition. |
| **C16** | Energy fields and active cost flows exist. Runtime payment, simultaneous drains, regeneration ordering and command-shield conversion are not implemented. |
| **C17** | Role-derived special identity is explicit and total. Current loadout validation rejects special-only IDs, but arbitrary strings can enter loadout fields; “checked by type system” overstates enforcement. |

**Persistence alignment:** JSON arrays are the existing persisted catalog representation. Serde verifies shape; semantic validation is separate. No database or player-storage implementation exists to audit. SQL constraints are **not applicable**, not automatically missing work.

---

## 3. CQ1–CQ10 answerability

| CQ | Answerability |
|---|---|
| **CQ1: purchasable models/prices** | Catalog answer complete for the supplied examples. No affordability, progression or purchasing transaction answer is promised by the existing helper. |
| **CQ2: role size/special** | Fully answerable through exhaustive Rust mappings and instance lookup, subject to enum-row identity uniqueness. |
| **CQ3: passive compatibility** | Family and capacity answerable; malformed long lists undermine enforcement. Effective-stat viability additionally depends on C5. |
| **CQ4: active eligibility** | Role and four-slot answer complete. Eligibility is not the same as affordable activation. |
| **CQ5: weapon fit** | Fully answerable by size for the current catalog. |
| **CQ6: shield versus armor damage** | Separate multipliers answerable. Damage crossing shield depletion does not have one unambiguous formula. |
| **CQ7: effective stats** | Arithmetic answerable for known references; validity is not guaranteed by `effective_stats` alone. |
| **CQ8: deployable ships** | Hangar candidates answerable for PvP/PvE given owned objects. Open-world selection follows a different relation and is not represented in Rust. |
| **CQ9: respawn/source** | Respawn Boolean and category are answerable. General source policy is prose; concrete station/selection and respawn execution are deferred. |
| **CQ10: destruction/cloak/heat** | Declarative thresholds/events exist. Runtime execution is absent, and heat timing still needs a precise operational convention. |

---

## 4. Three existing instance traces

These use current catalog rows and existing test/client constructions, not newly created persisted examples.

### Anaconda: engineer purchase and equipped test ship

**Sources:** `ship_models.json:2–5`; `model.rs:440–456`.

1. `buy("anaconda", "user-1", "ps-1")` creates the owned instance with no modules.
2. Engineer → frigate → drones (`model.rs:32,40`); drones are a press special, outside the loadout (`special_modules.json:2`).
3. Stock heavy-laser is frigate-sized, thermic, 420 damage/shot (`weapons.json:2–3`); thermic multipliers are 1/1 (`damage_types.json:4`).
4. Stock cruise-missile has kinetic damage 3500, four rounds/life and 15-second reload (`missiles.json:3`).
5. The test equips three shield-extenders. Three shield slots permit this; shield becomes `9000 + 3×1500 = 13500`. Other stats remain `(armor 6000, energy 600, regen 60, speed 180, agility 20)`.
6. Remote-repair is engineer-only and ongoing at 25 energy/s; shield-boost is unrestricted and costs 120 once (`active_modules.json:2–7`). Both fit; engine-suppressor does not.
7. Actual healing and energy evolution cannot be derived beyond these parameters because effects/runtime transitions are deferred.

### Hydra: current client stock ship and hangar example

**Sources:** `ship_models.json:14–17`; `client/src/main.rs:14–20`; `model.rs:466–469`.

1. Default client model is Hydra. Tackler → fighter → cloak.
2. Stock stats are `(4500, 4500, 450, 50, 260, 40)` in the documented stat order.
3. Assault-railgun fits fighter size; kinetic damage is 260/shot at 3 shots/s (`weapons.json:6–7`). Homing-missile is thermic with six rounds (`missiles.json:2`).
4. Flight receives speed 260 m/s and turn rate `40°/s` converted to radians. Cloak is not implemented by the flight consumer.
5. In the hangar test, Hydra is owned by `user-1`; placing `ps-a` in two slots is rejected.
6. As a valid single hangar entry, it would be a declared PvP/PvE deployment candidate. Team-deathmatch permits respawn; operation-scenario does not (`game_modes.json:2,4`).

### Kite: interceptor ownership boundary

**Sources:** `ship_models.json:34–37`; `model.rs:467–470`.

1. ECM → interceptor → em-surge.
2. Stock plasma-gun fits interceptor size and deals electromagnetic damage: 140/shot, 4 shots/s (`weapons.json:10–11`). EM multipliers are 1.5 shield/0.7 armor.
3. Homing-missile resolves; empty purchased module lists satisfy static capacity rules.
4. Base stats are `(3000, 2000, 450, 50, 350, 58)`.
5. The existing test creates Kite for `user-2`, then references it from `user-1`’s hangar. Ownership validation rejects it.
6. EM surge’s “every ship around” is only descriptive (`special_modules.json:10`); radius, duration and inclusion of the emitter are not supplied. No narrower targeting policy can be inferred.

---

## 5. Findings: proven validation and contract issues

### SEM-PASSIVE-COUNT — P1 — Internal logical issue

**Evidence:** C4 at `ontology/domain.md:241`; unbounded passive vector at `ontology/model.rs:189`; `u8` counting at `ontology/model.rs:351–360`.

**Counterexample:** buy stock Anaconda and equip 256 copies of existing `shield-extender`. Counting overflows: checked arithmetic panics; unchecked arithmetic wraps to zero. In the latter case, C4 sees zero shield entries and effective shield remains positive (`393000`), so the other checks do not rescue validation.

**Affected:** C4, CQ3 and valid-loadout interpretation of CQ7.

**Consequence:** a validator can crash or accept grossly overfilled equipment.

**Smallest resolution:** count with `usize` and compare against converted slot counts, or reject as soon as a family exceeds its capacity.

**Acceptance:** 3 copies pass; 4 and 256 copies fail without panic in debug and release configurations.

### SEM-THERMIC-PROFILE — P1 — Verified canonical/code contradiction

**Evidence:** C9 requires both thermic multipliers approximately one (`ontology/domain.md:246`). `ontology/model.rs:293` checks only `abs(vs_shield - vs_armor) < 0.2`.

**Counterexample:** replace the existing thermic pair with `100/100`. It passes positivity and the profile predicate despite not being approximately one.

**Affected:** C9, CQ6.

**Consequence:** semantically invalid balance data pass the canonical validator.

**Smallest resolution options:** validate each multiplier against an explicitly chosen tolerance around one; alternatively, explicitly revise the canonical contract to “balanced with each other” if that is the authorized design. Do not infer that change from the less-specific narrative.

**Acceptance:** current 1/1 passes; 100/100 fails under the present canonical meaning; tolerance boundaries are documented and tested.

### SEM-ENUM-IDENTITY — P1 — Identity/cardinality enforcement gap

**Evidence:** scalar damage properties at `ontology/domain.md:69–71`; role-special bijection at `ontology/domain.md:196`; uniqueness checks cover only six string-ID families (`ontology/model.rs:277–282`). Damage lookup selects the first row (`:268`), and special validation checks presence only (`:299–304`).

**Counterexample:** append a second electromagnetic row with negative multipliers after the existing valid row. Validation reads the first row and reports no error. Likewise, two `cloak` rows with different activation/effect values survive special validation.

**Affected:** C1’s intended identity guarantee, C9, C17, CQ2/CQ6.

**Consequence:** an ID can denote conflicting definitions; damage behavior depends on row order. Special modules are explicitly outside `catalog-item`, so C1’s wording should clarify its scope rather than relying on an unstated superclass membership.

**Smallest resolution:** reject duplicate `DamageKind` and `SpecialModuleKind` IDs and explicitly require one definition per enumerated kind.

**Acceptance:** duplicate rows fail regardless of ordering; all existing unique rows pass.

### SEM-VALIDATION-BOUNDARY — P1 — Current consumer bypasses semantic validation

**Evidence:** `ontology/model.rs:245–261` only deserializes. The production path loads, buys and computes directly (`client/src/main.rs:14–19`); catalog validation appears only in tests. Positive speed is required at `ontology/domain.md:82`.

**Counterexample:** an otherwise valid JSON catalog with Hydra’s speed changed to `-1` deserializes, produces stock stats and reaches `FlightParams::from`. The available range check at `ontology/model.rs:308` is never called on this path.

**Affected:** catalog facets/C1–C2/C9 and CQ1–CQ7 trustworthiness.

**Consequence:** successful loading is not evidence of a usable semantic catalog. This is an authored-local-content boundary, not a demonstrated remote attack surface.

**Smallest resolution options:** validate once during catalog loading, or explicitly validate at every production import boundary. Loadout validation belongs at future equipment-update boundaries; no validated-wrapper architecture is necessary.

**Acceptance:** a malformed scratch catalog is rejected before constructing flight parameters; the supplied catalog loads successfully.

### SEM-FLOAT-DOMAIN — P2 — Numeric facet gap

**Evidence:** positive/nonnegative stats at `ontology/domain.md:78–83`; comparison-only checks at `ontology/model.rs:308,383`; unchecked modifier addition at `:393–400`.

**Counterexample:** set a public model’s speed to `f32::NAN` in Rust. `speed <= 0.0` is false, so catalog and stock-loadout validation do not reject it, although NaN does not satisfy `speed > 0`. Finite large modifiers can also overflow during summation.

**Affected:** C5, numeric facets, CQ7.

**Consequence:** invalid numeric state can pass the validator and enter simulation. Ordinary JSON cannot contain a NaN literal; this is not claimed as such an import path.

**Smallest resolution:** reject nonfinite numeric inputs/results in the existing range-validation seam, preserving legitimate negative modifiers.

**Acceptance:** programmatically supplied NaN is rejected; overflowing modifier sums are rejected; existing finite data and negative light-armor deltas remain valid.

### SEM-ID-GRAMMAR — P2 — Declared identifier facet is unenforced

**Evidence:** stable kebab-case requirement at `ontology/domain.md:6`; `Id = String` at `ontology/model.rs:8`; uniqueness checks at `:273–282` do not check syntax.

**Counterexample:** a uniquely identified game-mode row with `id: ""` passes both deserialization and catalog validation.

**Affected:** identifier policy supporting C1 and CQ1–CQ9 references.

**Consequence:** invalid identifiers can enter persisted catalog data. Temporal stability cannot be proved by one catalog snapshot and remains a lifecycle obligation.

**Smallest resolution:** define the accepted kebab-case grammar and check it for local entity IDs in existing validators. Clarify whether externally supplied user IDs are exempt; do not assume authentication IDs follow local naming rules.

**Acceptance:** empty/whitespace IDs fail, supplied catalog IDs pass, and external-ID policy is explicit.

### SEM-CHECK-ATTRIBUTION — P2 — Enforcement documentation overclaims guarantees

**Evidence:**

- C5 names `effective_stats` as checker (`ontology/domain.md:242`), but it only sums (`ontology/model.rs:390–403`); positivity is checked at `:382–385`.
- C17 says “type system” (`ontology/domain.md:254`), but loadout references are strings (`ontology/model.rs:188–192`).
- Module header assigns C12–C17 to simulation (`ontology/model.rs:1–2`), contradicting C17’s static role derivation.

**Counterexamples:** four current light-armor entries on Kite make `effective_stats` return armor `-400` rather than reject it. An active ID `"cloak"` compiles/deserializes; `validate_loadout`, not the Rust type checker, rejects it as an unknown active.

**Affected:** C5/C17, CQ2/CQ7, validation preconditions.

**Consequence:** callers can mistake a calculation or deserialization for validation.

**Smallest resolution:** accurately document which functions enforce each condition and their preconditions. Stronger typed IDs are optional, not required to repair the documentation.

**Acceptance:** the contract explicitly distinguishes calculation from validated loadouts and states the actual special-exclusion boundary.

---

## 6. Findings: canonical wording and provenance

### SEM-MUTABILITY-SCOPE — P2 — Internal logical issue in scope wording

**Evidence:** C3 says “only `player-ship` and `hangar` mutate” (`ontology/domain.md:240`), while runtime state includes mutable energy/heat/health and C13–C16 describe their changes (`:186–189,250–253`).

**Incompatibility:** taken literally, applying energy regeneration to `ship-state` violates C3.

**Affected:** C3/C13–C16, CQ10.

**Consequence:** the immutability rule mixes persistent ownership/catalog scope with runtime scope.

**Smallest resolution:** qualify C3 as applying to catalog versus persisted player-owned data, explicitly excluding transient runtime state.

**Acceptance:** catalog immutability and permitted per-match state transitions can both be stated without an exception inferred by the reader.

### SEM-INSTANCE-TERMINOLOGY — P2 — Class/instance wording contradiction

**Evidence:** `ontology/domain.md:163` calls player-ship a mutable instance; `:275` says player-ships and hangars “are not instances.”

**Incompatibility:** persistence location does not determine whether an individual ship or hangar is an ontology instance.

**Affected:** Stanford step 7, ownership vocabulary underlying CQ8/C11.

**Consequence:** the final sentence confuses catalog fixture files with conceptual instances.

**Smallest resolution:** say these are not **catalog instance files**, and that owned instances will be persisted separately.

**Acceptance:** distinguish classes, catalog examples and player-owned individuals consistently.

### SEM-NARRATIVE-LINK — P2 — Verified stale reference; historical drift

**Evidence:** `ontology/domain.md:4` points to `../onthology.md`; the present narrative is `ontology.md:1–65`, and the old path is absent. The supplied inventory attributes this to the baseline rename.

**Affected:** source traceability across all CQs.

**Consequence:** following the canonical source reference fails.

**Smallest resolution:** update the path only; no narrative or semantics change is implied.

**Acceptance:** the relative reference resolves to the present narrative.

---

## 7. Design questions and deferred runtime contracts

These are report items, not requests to the user and not verified gameplay contradictions.

### SEM-DAMAGE-SPILLOVER — P2 — Underspecified damage conservation

**Evidence:** `ontology/domain.md:246`; electromagnetic multipliers at `ontology/instances/damage_types.json:2`.

**Concrete ambiguity:** for raw damage 100 against shield HP 75:

- Raw-equivalent spillover consumes `75 / 1.5 = 50` raw damage on shield; remaining armor damage is `50 × 0.7 = 35`.
- Carrying the already-scaled shield remainder produces `(150 − 75) × 0.7 = 52.5`.

The wording does not explicitly select the damage quantity carried across layers.

**Affected:** C9/C13, CQ6/CQ10.

**Consequence:** future server implementations can disagree while using the same multipliers.

**Smallest resolution options:** specify raw-equivalent conservation or another explicit staged formula. The audit does not choose the gameplay rule.

**Acceptance:** worked cases cover intact shield, exact depletion, partial spillover and zero shield, with unique numerical results.

### SEM-HEAT-IDLE — P2 — Underspecified firing/idle boundary

**Evidence:** cooling “while not firing” at `ontology/domain.md:132`; C15 at `:252`; heavy-laser parameters at `ontology/instances/weapons.json:2–3`.

**Concrete ambiguity:** heavy-laser adds 0.05 heat per shot at two shots/s.

- No cooling while a trigger-held firing sequence continues: 20 shots accumulate one heat.
- Cooling during the 0.5-second intervals between instantaneous shots removes 0.125 each interval: heat clears before the next shot.

Both arise from different operational meanings of “firing,” not different data.

**Affected:** C15, CQ10; narrative sustained-overheat intent at `ontology.md:53–54`.

**Consequence:** the same weapon either overheats or does not.

**Smallest resolution:** define whether idle means trigger released, outside a firing sequence, or any interval without a shot; define saturation and when the extra penalty begins.

**Acceptance:** a specified held-trigger/release timeline produces one deterministic shot, heat and unlock sequence.

### SEM-EFFECT-CONTRACT — P2 — Deliberately descriptive effects are not executable rules

**Evidence:** effects are explicitly designer-facing strings (`ontology/domain.md:116,153`). All specials use this representation (`special_modules.json:2–10`), as do actives (`active_modules.json:2–16`). Runtime wire form is deferred (`ontology/domain.md:186–189`).

**Concrete unanswered cases:**

- Shield-boost restores an unspecified “chunk”; 100 and 1000 HP are both compatible.
- Phasic shield cycles three resistance types but has generic `toggle` activation. Its transition sequence is not specified; “toggle” need not be assumed binary.
- Command shield lacks an energy-to-damage conversion and partial-payment rule.
- Sniper mode names no replacement weapon or alternate profile.
- Plasma web lacks damage kind, tick rate and duration.
- Overclock, hyper-propulsion and em-surge lack numeric strength/range/duration.
- Emergency-barrier specifies three seconds, but ordering against cloak-breaking and command-shield energy payment is absent.

**Affected:** C9/C14–C16, detailed CQ6/CQ10 behavior; CQ4 remains answerable for equip eligibility.

**Consequence:** catalog text cannot uniquely generate combat behavior. This is a present coverage limit, not evidence that free-text descriptions violate their declared type.

**Smallest resolution options:** explicitly retain these as non-executable design annotations until combat work; before implementing an effect, define its minimal parameters and transitions. No generic effect DSL is justified.

**Acceptance:** readiness documentation distinguishes descriptive from executable effects; each implemented effect later receives one deterministic worked/checkable transition.

### SEM-DEPLOYMENT-CONTRACT — P2 — Deferred selection and lifecycle decisions

**Evidence:** selected-ship cardinality at `ontology/domain.md:209`; category deployment rule at `:249`; open-world respawn true at `ontology/instances/game_modes.json:5`; owned objects at `ontology/model.rs:195–228`.

**Concrete unanswered case:** a user has valid hangar entries but no selected ship. C12 identifies the open-world deployment source, but does not specify fallback/rejection, selected-ship ownership validation, or whether respawn returns to the same station/selection.

**Affected:** C11/C12, CQ8/CQ9.

**Consequence:** declared mode metadata cannot determine a complete deployment transaction.

**Smallest resolution:** retain deferred implementation status; when deployment is specified, define selected-ship ownership/existence and missing-selection behavior. Do not add auth or station interiors to scope merely to answer this.

**Acceptance:** documented cases for absent selection, foreign selection, valid selection and mode-specific respawn yield unique admission decisions.

---

## 8. Deliberate simplifications and non-findings

The following are **not** established defects:

- Three sizes, nine roles and one special per role are explicit local commitments, not claims of exhaustive live-game fidelity.
- Fixed 0–3 passive-family counts and a separate fifth special slot are stated design choices.
- The narrative’s “4 ActiveCombatModules” is clarified by its own “up to 4” wording (`ontology.md:60`) and canonical 0–4 cardinality.
- Shield capacity zero is permitted canonically; “all ships have a shield” in the narrative does not by itself prove that zero capacity is forbidden.
- Only one interceptor weapon example exists. The narrative mentions multiple weapons per size (`ontology.md:51`), but no canonical claim says the supplied catalog is exhaustive.
- Homing/EMP names alone do not establish missing guidance or electronic-status behavior requirements.
- Missing objectives, wave generators, operation scripts, progression, station interiors and full economy follow explicit deferral/exclusion.
- Fixed acceleration/drag and spherical hulls are marked simplifications (`sim/src/lib.rs:54–57,94–97`), not ontology-derived numerical truths.
- There is no current server, combat loop or player persistence service. Their absence is not an automatic failure of static catalog modeling.

**Product decision to record before external comparison:** whether the catalog is independent inspiration or intended to reproduce a dated Star Conflict version, and whether its rows are examples or exhaustive coverage. `ontology/domain.md:10–23` establishes “Star Conflict-like” scope but no external fidelity/version contract. No external claim should be assigned a contradiction label until that target and evidence are established.

---

## 9. Gruber criteria

| Criterion | Assessment |
|---|---|
| **Clarity** | Strong enumerations, units, ownership and CQ/constraint tables. Weaker identifier grammar, thermic tolerance, instance wording and operational meanings of damage/heat/effects. |
| **Coherence** | Supplied examples and role/size mappings agree. Proven validation counterexamples and enforcement-attribution errors prevent treating the mirror as a complete checker of its prose contract. |
| **Extendibility** | New catalog rows fit without schema changes; new roles/special kinds require deliberate enum/mapping edits. That is acceptable for the expressly closed taxonomy. No speculative hierarchy or generic effect system is necessary. |
| **Minimal encoding bias** | Readable IDs, value objects and composition are appropriate. The `u8` occurrence counter incorrectly lets an encoding choice weaken a domain cardinality. JSON location must not define conceptual instancehood. |
| **Minimal ontological commitment** | Scope exclusions and deferred runtime representation avoid unnecessary claims. Preserve those boundaries; do not import unverified game mechanics or expand the model solely because an external game has more features. |

## 10. Verification status and acceptance follow-up

Existing checks at `ontology/model.rs:431–478` cover:

- Supplied catalog validation.
- Stock purchase, ordinary passive overfill and additive shield stats.
- Active-role and weapon-size restrictions.
- Duplicate/foreign-owner hangar entries.
- Unique role-to-special mappings.

They do **not** exercise the long-list overflow, thermic 100/100, duplicate enum definitions, malformed-load boundary or NaN counterexamples. No passing execution is claimed.

A supervisor may run the existing ontology tests in an already available authorized Rust environment:

`cargo test --offline --locked -p stargem-ontology`

For `SEM-PASSIVE-COUNT`, an eventual regression check must also run with release arithmetic. Build output should use authorized managed scratch; unavailable dependencies should remain a recorded limitation rather than trigger installation.

**Recommended order:** repair the four P1 validation seams; correct documentation attribution and identity/lifecycle wording; retain runtime questions as explicit design decisions until their implementation is authorized.