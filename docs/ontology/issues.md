# Internal ontology and validation issues

Audit: **2026-09-16**, baseline `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`. All items open; no repairs authorized or made. [Navigation](README.md) · [Questions](questions.md) · [Execution evidence](validation.md).

P1: resolve before relying on newly authored data/loadout boundaries. P2: correctness or specification gate for affected work. P3: clarity/traceability. These priorities synthesize the two local audits: numerical JSON propagation raises nonfinite handling to P1; the present client bypass is P1; thermic/enum authoring issues are P2. Passing seed tests do **not** establish ontology coherence.

Classification separates a proven local conflict, missing validation/policy constraint, and ambiguous specification. Deferred runtime entries are design-readiness limits, **not requests to implement gameplay**.

<a id="issue-001"></a>
## ISSUE-001 — Passive-family counter overflow

- [ ] **Open · P1 · proven contradiction**
- **Anchors:** [`ontology/model.rs:351`](../../ontology/model.rs#L351), [`ontology/domain.md:241`](../../ontology/domain.md#L241)
- **Constraint/CQ:** C4 / CQ3, CQ7.
- **Proof/counterexample:** 256 existing shield-extenders on Anaconda overflow the u8 counter: debug panics; default release wraps to zero and accepts. Shield calculation still sums all occurrences: 9000 + 256×1500 = 393000. Executed in the validation audit, not a current seed failure.
- **Impact:** A later loadout boundary can crash or accept extreme overfill; no current network submission route exists.
- **Minimal repair options (not approval):** Use usize occurrence counts or reject immediately on exceeding family capacity. Do not ban repeated IDs: three copies are intentionally legal.
- **Acceptance check:** Three copies pass; four and 256 fail without panic in debug and release.
- **Decision:** [QUESTION-005](questions.md#question-005).

<a id="issue-002"></a>
## ISSUE-002 — Nonfinite inputs and effective sums

- [ ] **Open · P1 · proven contradiction**
- **Anchors:** [`ontology/model.rs:308`](../../ontology/model.rs#L308), [`ontology/model.rs:383`](../../ontology/model.rs#L383), [`ontology/model.rs:393`](../../ontology/model.rs#L393), [`sim/src/lib.rs:54`](../../sim/src/lib.rs#L54)
- **Constraint/CQ:** Stats facets / C5 / CQ7.
- **Proof/counterexample:** Validation audit executed valid JSON speed 1e39 → f32 infinity, accepted by catalog/loadout validation, then actual sim::step yielded Vec3(NaN, NaN, -inf). Public Rust NaN values bypass comparison-only facets; two finite f32::MAX shield deltas within slot capacity overflow the sum. JSON NaN tokens themselves are rejected. Heat bounds and damage-profile comparisons do not share every NaN weakness.
- **Impact:** Invalid authored data can poison present flight. Positive-infinity exclusion is a missing finite-value policy; NaN acceptance already violates the positive facets. Current seeds and enumerated legal combinations are finite.
- **Minimal repair options (not approval):** Check finiteness and existing ranges at numeric inputs and computed outputs; preserve finite negative modifiers and allowed zero shield/regen. No arbitrary balance ceilings needed.
- **Acceptance check:** Reject 1e39 conversion, NaN base stats and overflowing finite sums before flight; retain current legal values.
- **Decision:** [QUESTION-005](questions.md#question-005).

<a id="issue-003"></a>
## ISSUE-003 — Semantic validation is absent from production import

- [ ] **Open · P1 · missing constraint**
- **Anchors:** [`ontology/model.rs:245`](../../ontology/model.rs#L245), [`client/src/main.rs:14`](../../client/src/main.rs#L14)
- **Constraint/CQ:** Catalog facets / C1–C2, C5–C9 / CQ1–CQ7.
- **Proof/counterexample:** Loader only deserializes. Client loads → buys → effective_stats → FlightParams without either validator. A scratch Anaconda with price 0, speed −1 and missing stock weapon loads and constructs successfully; explicit catalog validation returns three errors, loadout validation two.
- **Impact:** Present authored-local-content boundary, not a demonstrated remote exploit. Adding calls alone does not repair ISSUE-002.
- **Minimal repair options (not approval):** Validate once at import and before consuming changed loadouts, or introduce a checked loader/calculator contract. Keep a raw parser only with explicit preconditions.
- **Acceptance check:** Malformed stock data is rejected before FlightParams; valid seed data loads; all consumers follow the selected contract.
- **Decision:** [QUESTION-005](questions.md#question-005).

<a id="issue-004"></a>
## ISSUE-004 — Thermic equality is not proximity to one

- [ ] **Open · P2 · proven contradiction**
- **Anchors:** [`ontology/domain.md:246`](../../ontology/domain.md#L246), [`ontology/model.rs:293`](../../ontology/model.rs#L293)
- **Constraint/CQ:** C9 / CQ6.
- **Proof/counterexample:** Thermic 100/100 passes positivity and abs(shield−armor)<0.2 but violates canonical “both ≈ 1”. Current seed is 1/1. Executed by validation audit.
- **Impact:** False semantic acceptance, not an observed combat result. Narrative balanced-power wording cannot override canonical C9.
- **Minimal repair options (not approval):** Check each multiplier against an approved around-one tolerance; alternatively intentionally revisit C9 to mean only equal layer strength. Do not infer tolerance from the cross-difference constant.
- **Acceptance check:** 1/1 passes, 100/100 fails under current intent; approved tolerance edge cases are documented and checked.
- **Decision:** [QUESTION-006](questions.md#question-006).

<a id="issue-005"></a>
## ISSUE-005 — Duplicate enum identities escape validation

- [ ] **Open · P2 · missing constraint**
- **Anchors:** [`ontology/model.rs:268`](../../ontology/model.rs#L268), [`ontology/model.rs:277`](../../ontology/model.rs#L277), [`ontology/model.rs:299`](../../ontology/model.rs#L299), [`ontology/domain.md:196`](../../ontology/domain.md#L196)
- **Constraint/CQ:** Damage identity / C9, role-special bijection / C17 / CQ2, CQ6.
- **Proof/counterexample:** Append an electromagnetic row with negative multiplier after the valid row and a conflicting drones row: catalog passes. Move invalid damage row first: it fails. Presence and enum typing do not enforce one row per kind. Executed.
- **Impact:** First-match order controls which definition is visible. Current rows are unique. C1 expressly names catalog IDs, while specials are outside catalog-item: clarify the identity facet rather than silently broadening that superclass.
- **Minimal repair options (not approval):** Reject duplicate DamageKind and SpecialModuleKind definitions alongside required-kind checks; a map is optional and still needs duplicate-key policy.
- **Acceptance check:** Duplicate rows fail even if identical and regardless of order; all required unique kinds pass.
- **Decision:** [QUESTION-007](questions.md#question-007).

<a id="issue-006"></a>
## ISSUE-006 — Constraint-checker attribution overstates guarantees

- [ ] **Open · P2 · proven contradiction**
- **Anchors:** [`ontology/domain.md:242`](../../ontology/domain.md#L242), [`ontology/domain.md:254`](../../ontology/domain.md#L254), [`ontology/model.rs:1`](../../ontology/model.rs#L1), [`ontology/model.rs:390`](../../ontology/model.rs#L390)
- **Constraint/CQ:** C3, C5, C10, C17 / CQ2, CQ7.
- **Proof/counterexample:** C5 names effective_stats, but it only sums; validate_loadout checks positivity. With a −7000 light-armor delta, one capacity-legal Anaconda passive returns Some(armor −1000), then loadout validation rejects. A string active ID cloak compiles; family lookup rejects it, not the type checker. Header assigns all C1–C11 checks to crate/all C12–C17 to server, contrary to C3/payment C10 deferral and static C17 mapping.
- **Impact:** Callers may mistake calculation/deserialization for validation. Root production bypass is separately ISSUE-003.
- **Minimal repair options (not approval):** Document exact checked APIs and preconditions, including partial C10 stock construction and C17 role derivation/family lookup. Stronger ID types are optional.
- **Acceptance check:** Responsibility table matches actual functions; no claim that Some(stats), parsing, or buy guarantees full validity/payment.
- **Decision:** [QUESTION-005](questions.md#question-005).

<a id="issue-007"></a>
## ISSUE-007 — Unknown fields silently erase intended modifiers

- [ ] **Open · P2 · missing constraint**
- **Anchors:** [`ontology/model.rs:83`](../../ontology/model.rs#L83), [`ontology/model.rs:245`](../../ontology/model.rs#L245)
- **Constraint/CQ:** Authoring policy unspecified / CQ7.
- **Proof/counterexample:** serde parses {"sheild_hp":1500} as all-zero StatModifiers because unknown fields are ignored and absent correct fields default. Required base fields still reject omission. Executed.
- **Impact:** A typo becomes successful no-op data; not a violation of an existing strict-unknown-fields promise.
- **Minimal repair options (not approval):** Choose strict catalog/value deserialization or a strict authoring/CI boundary while retaining forward-compatible readers. Preserve intentional omitted-field defaults.
- **Acceptance check:** Chosen authoring check rejects sheild_hp but accepts {}; document compatibility policy.
- **Decision:** [QUESTION-008](questions.md#question-008).

<a id="issue-008"></a>
## ISSUE-008 — Duplicate JSON map keys overwrite slot declarations

- [ ] **Open · P2 · missing constraint**
- **Anchors:** [`ontology/model.rs:117`](../../ontology/model.rs#L117), [`ontology/model.rs:124`](../../ontology/model.rs#L124)
- **Constraint/CQ:** Authoring policy unspecified / C2, CQ3.
- **Proof/counterexample:** Slot map with shield:3 followed by shield:1 deserializes to 1. Typed-map duplicate handling differs from derived-struct duplicate fields. Current JSON has no duplicate object keys. Executed.
- **Impact:** Conflicting author declarations disappear without diagnosis; absence of a reject policy is not a proven contradiction. Missing keys meaning zero is a separate accepted code convention.
- **Minimal repair options (not approval):** Reject duplicates at the authoring boundary, or explicitly retain last-value-wins compatibility. Do not assume a map automatically enforces input uniqueness.
- **Acceptance check:** A repeated-key sample has the documented outcome; if strict, it fails before semantic use; omitted keys remain explicitly defined.
- **Decision:** [QUESTION-008](questions.md#question-008).

<a id="issue-009"></a>
## ISSUE-009 — Stable kebab-case identifier grammar is unenforced

- [ ] **Open · P3 · missing constraint**
- **Anchors:** [`ontology/domain.md:6`](../../ontology/domain.md#L6), [`ontology/model.rs:8`](../../ontology/model.rs#L8), [`ontology/model.rs:273`](../../ontology/model.rs#L273)
- **Constraint/CQ:** Identifier facet / C1 reference vocabulary / CQ1–CQ9.
- **Proof/counterexample:** Unique game-mode IDs BAD ID and empty string both pass Catalog::validate. All current IDs conform. Stability across time cannot be tested from one snapshot.
- **Impact:** Authoring can violate the declared naming convention; no authentication vulnerability proved.
- **Minimal repair options (not approval):** Define/check local-ID grammar in existing validation or an authoring check; explicitly exempt or define external user IDs. A newtype is optional.
- **Acceptance check:** Empty/uppercase/space local IDs reject; seed IDs and family-local same-spelling IDs remain valid; owner-ID scope is explicit.
- **Decision:** [QUESTION-009](questions.md#question-009).

<a id="issue-010"></a>
## ISSUE-010 — Mutability rule includes contradictory runtime scope

- [ ] **Open · P2 · proven contradiction**
- **Anchors:** [`ontology/domain.md:240`](../../ontology/domain.md#L240), [`ontology/domain.md:186`](../../ontology/domain.md#L186), [`ontology/domain.md:253`](../../ontology/domain.md#L253)
- **Constraint/CQ:** C3 versus C13–C16 / CQ10.
- **Proof/counterexample:** Literal “only player-ship and hangar mutate” forbids energy regeneration and damage/heat changes in ship-state that the same document requires.
- **Impact:** Persistent catalog immutability and transient combat lifecycle are conflated. Public mutable Rust fields alone are not a separate lifecycle defect.
- **Minimal repair options (not approval):** Qualify C3 as catalog versus persisted owned data, excluding transient runtime state; or explicitly enumerate each lifecycle scope.
- **Acceptance check:** State regeneration can comply with C3 and C16 without an unstated exception.
- **Decision:** [QUESTION-010](questions.md#question-010).

<a id="issue-011"></a>
## ISSUE-011 — Owned individuals incorrectly called non-instances

- [ ] **Open · P3 · proven contradiction**
- **Anchors:** [`ontology/domain.md:163`](../../ontology/domain.md#L163), [`ontology/domain.md:275`](../../ontology/domain.md#L275)
- **Constraint/CQ:** Stanford step 7 / ownership concepts / CQ8.
- **Proof/counterexample:** PlayerShip is called a mutable instance, then player-ships/hangars “are not instances”. Persistence location cannot change conceptual instancehood.
- **Impact:** Confuses catalog fixture files with domain individuals.
- **Minimal repair options (not approval):** Say they are not catalog instance files and are persisted per user separately.
- **Acceptance check:** Class, catalog example and player-owned instance wording is consistent; no owned fixture/store implementation required.
- **Decision:** [QUESTION-010](questions.md#question-010).

<a id="issue-012"></a>
## ISSUE-012 — Narrative reference stale after local rename

- [ ] **Open · P3 · proven contradiction**
- **Anchors:** [`ontology/domain.md:4`](../../ontology/domain.md#L4), [`ontology.md:1`](../../ontology.md#L1)
- **Constraint/CQ:** Traceability / all CQs.
- **Proof/counterexample:** Canonical narrative path ../onthology.md is absent. Baseline commit renamed onthology.md to ontology.md R100 with no content change. Verified local history, not external game evolution.
- **Impact:** Readers cannot follow source lineage. Audit permissions prohibit fixing canonical files in this pass.
- **Minimal repair options (not approval):** In a later authorized documentation change correct only the path.
- **Acceptance check:** Relative narrative path resolves to ontology.md, with no semantic change.
- **Decision:** [QUESTION-010](questions.md#question-010).

<a id="issue-013"></a>
## ISSUE-013 — Shield spillover does not name conserved quantity

- [ ] **Open · P2 · ambiguous specification**
- **Anchors:** [`ontology/domain.md:246`](../../ontology/domain.md#L246), [`ontology/instances/damage_types.json:2`](../../ontology/instances/damage_types.json#L2)
- **Constraint/CQ:** C9, C13 / CQ6, CQ10; deferred runtime.
- **Proof/counterexample:** Raw EM 100 against shield 75: consuming 75/1.5 raw units leaves 50×0.7=35 armor damage; carrying the scaled remainder gives (150−75)×0.7=52.5. No executor selects one reading.
- **Impact:** Future combat implementations can diverge; not an implemented damage bug.
- **Minimal repair options (not approval):** Define raw-equivalent conservation or another explicit staged formula; define zero shield, exact depletion and overshoot/clamping.
- **Acceptance check:** Worked intact/exhausted/partial/zero-shield hits and lethal overshoot have unique outputs.
- **Decision:** [QUESTION-011](questions.md#question-011).

<a id="issue-014"></a>
## ISSUE-014 — Firing versus idle and overheat timeline

- [ ] **Open · P2 · ambiguous specification**
- **Anchors:** [`ontology/domain.md:132`](../../ontology/domain.md#L132), [`ontology/domain.md:252`](../../ontology/domain.md#L252), [`ontology/instances/weapons.json:2`](../../ontology/instances/weapons.json#L2)
- **Constraint/CQ:** C15 / CQ10; deferred runtime.
- **Proof/counterexample:** Heavy Laser adds 0.05 at 2 shots/s. No cooling while trigger held gives threshold after 20 shots; cooling during each 0.5-second inter-shot gap removes 0.125 and prevents accumulation. Full cooling from 1 at 0.25/s plus sequential penalty 3 s gives 7 s, but event ordering is not executable.
- **Impact:** Same data can imply incompatible sustained-fire behavior. Crossing 1 has no explicit saturation rule.
- **Minimal repair options (not approval):** Define idle, shot scheduling, clamping and whether penalty overlaps or follows cooling, preserving or intentionally revisiting current prose.
- **Acceptance check:** One held/released-trigger timeline specifies shots, threshold crossing, heat and earliest unlock uniquely.
- **Decision:** [QUESTION-012](questions.md#question-012).

<a id="issue-015"></a>
## ISSUE-015 — Descriptive effects are not executable contracts

- [ ] **Open · P2 · ambiguous specification**
- **Anchors:** [`ontology/domain.md:116`](../../ontology/domain.md#L116), [`ontology/domain.md:153`](../../ontology/domain.md#L153), [`ontology/instances/special_modules.json:2`](../../ontology/instances/special_modules.json#L2), [`ontology/instances/active_modules.json:2`](../../ontology/instances/active_modules.json#L2)
- **Constraint/CQ:** CQ6, CQ10 / C14–C16; deliberate descriptive scope, deferred runtime.
- **Proof/counterexample:** Shield boost has no amount; drones no radius/rate/entity lifecycle; sniper no replacement profile; phasic toggle cycles three resistances without state transitions; cloak has damage cancellation but no duration; overclock/web/warp/surge omit strengths/ticks/range. “Every ship” does not resolve self/friendly handling. Barrier gives 3 s only in text; command shield omits conversion/payment exhaustion.
- **Impact:** Free text satisfies its declared type but cannot yield unique combat behavior. Do not call this a schema violation or implement from guessed real-game analogies.
- **Minimal repair options (not approval):** Keep descriptions explicitly non-executable; before each authorized effect, choose minimal parameters, target predicates and transitions. No speculative effect DSL.
- **Acceptance check:** Each effect actually selected for implementation has one deterministic worked transition including ordering/interactions; others stay deferred.
- **Decision:** [QUESTION-013](questions.md#question-013).

<a id="issue-016"></a>
## ISSUE-016 — Active timing and repeated-active state identity

- [ ] **Open · P2 · ambiguous specification**
- **Anchors:** [`ontology/domain.md:120`](../../ontology/domain.md#L120), [`ontology/model.rs:364`](../../ontology/model.rs#L364), [`ontology/domain.md:186`](../../ontology/domain.md#L186)
- **Constraint/CQ:** C6, C16 / CQ4 equip answer versus future activation; deferred runtime.
- **Proof/counterexample:** Ongoing cooldown could start on activation or deactivation; forced stop on energy exhaustion is unspecified. Four shield-boost IDs fit but there is no rule for separate versus shared cooldown state. One-shot cost is not effect duration (barrier has 3 s effect, 60 s cooldown).
- **Impact:** Equip eligibility is answerable; activation sequence/state storage is not. Passive repetition is already intentionally linear and is not reopened by this issue.
- **Minimal repair options (not approval):** Specify timing and per-occurrence or shared active state; alternatively intentionally prohibit repeated actives. Keep zero cooldown legal unless revisited.
- **Acceptance check:** Two repeated actives and voluntary/forced ongoing shutdown have unambiguous cost, cooldown start and next activation outcomes.
- **Decision:** [QUESTION-014](questions.md#question-014).

<a id="issue-017"></a>
## ISSUE-017 — Deployment selection and lifecycle remain declarative

- [ ] **Open · P2 · ambiguous specification**
- **Anchors:** [`ontology/domain.md:209`](../../ontology/domain.md#L209), [`ontology/domain.md:249`](../../ontology/domain.md#L249), [`ontology/instances/game_modes.json:5`](../../ontology/instances/game_modes.json#L5)
- **Constraint/CQ:** C11–C13 / CQ8, CQ9; deferred runtime.
- **Proof/counterexample:** An owner may have a valid hangar but no selected ship. C12 gives no missing/foreign selection outcome, station binding, hangar-membership requirement, respawn destination, or open-world session boundary. Boolean respawn does not formally separate next-life permission from ship reselection.
- **Impact:** Mode metadata is not a complete admission/respawn transaction. Current CLI flight demo bypass is intentional prototype scope, not a contradictory deployment executor.
- **Minimal repair options (not approval):** Before deployment work define selection ownership/existence, optional membership, missing selection, station/session/life identity and respawn/reselection policy. Keep generators/maps/auth deferred.
- **Acceptance check:** Absent, foreign and valid selection plus each mode’s death/reselection cases yield unique outcomes; no unapproved runtime work starts.
- **Decision:** [QUESTION-019](questions.md#question-019).

<a id="issue-018"></a>
## ISSUE-018 — Hangar validation assumes unique owned-ship IDs

- [ ] **Open · P2 · missing constraint**
- **Anchors:** [`ontology/model.rs:215`](../../ontology/model.rs#L215), [`ontology/model.rs:221`](../../ontology/model.rs#L221), [`ontology/domain.md:248`](../../ontology/domain.md#L248)
- **Constraint/CQ:** C11 / CQ8; deferred authoritative-store precondition.
- **Proof/counterexample:** Two supplied PlayerShip rows share an ID with different owners. Owned-first passes; foreign-first fails because find selects first. A single foreign ship is correctly rejected. Executed in validation audit.
- **Impact:** Undocumented input precondition; no current storage service or production hangar caller, therefore not a present auth bypass.
- **Minimal repair options (not approval):** Document/enforce unique player-ship IDs at authoritative persistence; optionally reject duplicates defensively in this validator. Do not impose global cross-family catalog uniqueness.
- **Acceptance check:** Future store rejects conflicting rows in either order, or validator rejects ambiguous input; ordinary ownership checks still pass/fail correctly.
- **Decision:** [QUESTION-009](questions.md#question-009).

<a id="issue-019"></a>
## ISSUE-019 — Instant acceleration wording versus finite response

- [ ] **Open · P3 · ambiguous specification**
- **Anchors:** [`ontology/domain.md:82`](../../ontology/domain.md#L82), [`ontology.md:52`](../../ontology.md#L52), [`sim/src/lib.rs:48`](../../sim/src/lib.rs#L48), [`sim/src/lib.rs:81`](../../sim/src/lib.rs#L81)
- **Constraint/CQ:** Speed semantics / CQ7 consumer.
- **Proof/counterexample:** At accel 6/s and dt 1/60, exponential response reaches 1−exp(−0.1)=9.516% of target velocity in the first step, not 100%. Code explicitly calls big response “instant” and marks shared tuning knobs a simplification.
- **Impact:** Loose arcade terminology, not a reason to replace the motion model or claim failed finite-input tests.
- **Minimal repair options (not approval):** Clarify “instant” as fast velocity response; only change dynamics if literal instantaneous motion is intentionally chosen.
- **Acceptance check:** Documentation states response units and chosen meaning; existing flight behavior remains unchanged unless separately authorized.
- **Decision:** [QUESTION-015](questions.md#question-015).

<a id="issue-020"></a>
## ISSUE-020 — Fixed-step wording versus frame-delta caller

- [ ] **Open · P3 · ambiguous specification**
- **Anchors:** [`sim/src/lib.rs:70`](../../sim/src/lib.rs#L70), [`client/src/main.rs:54`](../../client/src/main.rs#L54), [`client/src/main.rs:62`](../../client/src/main.rs#L62)
- **Constraint/CQ:** Simulation timing contract; deferred deterministic server integration.
- **Proof/counterexample:** step is called a fixed-step integration but client supplies get_frame_time directly rather than an accumulator. A variable frame sequence therefore is not a fixed tick schedule.
- **Impact:** Documentation/integration readiness gap, not proof of faulty present flight tests or complete frame-rate independence of all collision/position results.
- **Minimal repair options (not approval):** Describe the current variable-delta demo honestly, or separately authorize a fixed-tick adapter when deterministic simulation is required.
- **Acceptance check:** Document actual timestep preconditions; any future fixed-tick claim is checked against its caller schedule.
- **Decision:** [QUESTION-015](questions.md#question-015).

## Rejected or narrowed findings

- Scope exclusions, missing combat/server/economy, one interceptor weapon example and only nine models are not defects by omission. No requirement says this catalog exhausts Star Conflict.
- Do not ban passive duplicates: the existing test explicitly accepts three. Active uniqueness is unresolved, not a current violated constraint.
- Narrative “4 modules” is resolved by its own “up to 4”, canonical 0..4 and empty purchase vectors; no separate contradiction. Zero shield is explicitly permitted.
- No missile size restriction, physical one-barrel rule, homing/EMP algorithm, shield passive regeneration, or external-user ID grammar may be inferred from names/absence.
- Public mutable fields do not prove lifecycle ownership/immutability violations. buy is stock construction, not a promised payment service.
- Special IDs are non-catalog kinds: duplicate-definition finding rests on identity/bijection, not an invented catalog superclass.
- “toggle” need not mean binary; three-state cycling is an ambiguity, not a proven type conflict.
- External documentary mismatches are now established in the [drift ledger](drift_developer_vs_internet.md); they do not themselves prove local incoherence. No complete current signed-resistance or shield-spillover formula, arbitrary removed-mode claim, or automatic ship-role correction is approved.
