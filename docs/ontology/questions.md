# User-owned reconciliation questions

Audit **2026-09-16**. [Navigation](README.md) · [Internal issues](issues.md) · [External comparison queue](drift_developer_vs_internet.md). Every question is open, no answer or rule change implied. Resolve fidelity/era/scope first. Settled repository choices are explicitly retained unless the user intentionally revisits them.

<a id="question-001"></a>
## QUESTION-001 — Fidelity, product and era first

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/domain.md:10 says Star Conflict-like, not exact replica. Fetched desktop-game evidence spans 2019–2026; official patch 1.14.15 is dated September 11, monetization ended June 1, shutdown is scheduled October 10, and the announced local-server version is not established as released ([history](research/history.md)).
- **Decision/options and tradeoffs:** Choose independent inspired rules (least provenance burden), a bounded dated adaptation (explicit subset/patch), or rolling fidelity (ongoing evidence upkeep). If fidelity is chosen, identify exact product and era; do not merge Star Conflict: Heroes by name. Local authority is already settled and is not up for accidental replacement.
- **Affected / remains blocked:** All DRIFT items; documentary comparisons are possible now, but choosing which differences require a local change remains blocked.

<a id="question-002"></a>
## QUESTION-002 — Preserve or intentionally revisit scope and catalog breadth

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/domain.md:18–23 excludes auth, progression, most economy, station interiors, AI/maps and defers faction/tier/generators. Nine ship examples are not declared exhaustive.
- **Decision/options and tradeoffs:** Keep the settled scope and label catalog illustrative or launch-complete within that scope; or explicitly reopen selected exclusions with new CQs. State which existing CQs need executable rather than declarative answers next.
- **Affected / remains blocked:** [DRIFT-002](drift_developer_vs_internet.md#drift-002), [DRIFT-005](drift_developer_vs_internet.md#drift-005), [DRIFT-019](drift_developer_vs_internet.md#drift-019), [DRIFT-025](drift_developer_vs_internet.md#drift-025), [DRIFT-028](drift_developer_vs_internet.md#drift-028), [DRIFT-031](drift_developer_vs_internet.md#drift-031); expansion and full-game completeness claims blocked.

<a id="question-003"></a>
## QUESTION-003 — Names, numerical provenance and terminology

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/instances/ship_models.json:2–37 has familiar names but no external IDs; ontology.md:23 says Cover Ops while canonical role is covert-ops.
- **Decision/options and tradeoffs:** Decide homages/placeholders versus exact counterpart identity; keep stable local IDs and record aliases/display corrections separately. Determine whether prices/stats are original balance or sourced values. Do not rename IDs just to match a translation. Eight ship-role identities, Pulse Laser, Cruise Missile and Emergency Barrier differ from fetched named counterparts; independent custom identities remain a legitimate option.
- **Affected / remains blocked:** [DRIFT-039](drift_developer_vs_internet.md#drift-039), [DRIFT-040](drift_developer_vs_internet.md#drift-040), [DRIFT-041](drift_developer_vs_internet.md#drift-041), [DRIFT-042](drift_developer_vs_internet.md#drift-042); [DRIFT-004](drift_developer_vs_internet.md#drift-004), [DRIFT-006](drift_developer_vs_internet.md#drift-006), [DRIFT-023](drift_developer_vs_internet.md#drift-023), [DRIFT-029](drift_developer_vs_internet.md#drift-029), [DRIFT-033](drift_developer_vs_internet.md#drift-033); name-level mismatch and numeric corrections blocked.

<a id="question-004"></a>
## QUESTION-004 — Source/version and conflict policy

- [ ] **Open · decision owner: user**
- **Context/evidence:** Original articles and wiki revisions were retrieved; their applicability differs. Guided-torpedo duration, range, Singularity classification and signed-resistance notation conflict across sources ([sources](research/sources.md)). Repository dates date local commits only.
- **Decision/options and tradeoffs:** Prefer claim-relevant original announcements for changes, storefront for identity, revision-aware wiki for detail; preserve unresolved conflicts. Choose a frozen source snapshot or per-claim validity windows, and durable notes versus gameplay-schema provenance (notes alone suffice unless runtime needs it).
- **Affected / remains blocked:** [DRIFT-001](drift_developer_vs_internet.md#drift-001), [DRIFT-032](drift_developer_vs_internet.md#drift-032); source-conflict resolution and any adopted version-specific rules remain blocked; official dated historical changes are already evidenced.

<a id="question-005"></a>
## QUESTION-005 — Which API promises semantically valid data?

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/model.rs:245 parses only; effective_stats at 391 sums only; client/src/main.rs:14–19 calls neither validator.
- **Decision/options and tradeoffs:** Retain raw parse/calculation helpers with explicit validated import/use boundaries, or make successful checked loading/calculation imply the facets. Preserve negative passive deltas and allowed zero fields while rejecting nonfinite state. No new wrapper architecture is required.
- **Affected / remains blocked:** [ISSUE-001](issues.md#issue-001), [ISSUE-002](issues.md#issue-002), [ISSUE-003](issues.md#issue-003), [ISSUE-006](issues.md#issue-006); safe boundary reliance blocked; fixes still require authorization.

<a id="question-006"></a>
## QUESTION-006 — Thermic near-one tolerance

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/domain.md:246 already says both ≈1; model.rs:293 only compares the two values.
- **Decision/options and tradeoffs:** Choose and document around-one tolerance and boundaries under existing intent, or intentionally revisit the canonical rule to mean equal at any magnitude. The latter changes semantics; do not silently select it because code currently accepts 100/100.
- **Affected / remains blocked:** [ISSUE-004](issues.md#issue-004); exact acceptance thresholds blocked, current defect established independently.

<a id="question-007"></a>
## QUESTION-007 — Identity of specials and their definition rows

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/domain.md:196 currently fixes role↔special kind; model.rs:299 checks presence only.
- **Decision/options and tradeoffs:** Preserve exact role-derived specials and one definition per kind; if intentionally reopening, decide whether kind means family/default with approved model-level alternatives. Versioned multiple rows need an explicit selector, not first-match ambiguity.
- **Affected / remains blocked:** [ISSUE-005](issues.md#issue-005), [DRIFT-003](drift_developer_vs_internet.md#drift-003); definition uniqueness and any source-backed variant design require explicit reconciliation.

<a id="question-008"></a>
## QUESTION-008 — Strict authoring versus forward-compatible reading

- [ ] **Open · decision owner: user**
- **Context/evidence:** Unknown fields disappear, default modifiers become zero; duplicate map keys overwrite (ontology/model.rs:83,117,245).
- **Decision/options and tradeoffs:** Strict catalog deserialization is simplest for authors; strict CI plus tolerant runtime preserves forward reading. Independently choose duplicate-key rejection. Keep {} modifiers and missing-slot-as-zero behavior unless deliberately revised.
- **Affected / remains blocked:** [ISSUE-007](issues.md#issue-007), [ISSUE-008](issues.md#issue-008); authoring-format acceptance policy blocked.

<a id="question-009"></a>
## QUESTION-009 — Identifier lifecycle and authoritative owned-store preconditions

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/domain.md:6 promises stable kebab-case; model.rs:221 assumes first matching owned ship is authoritative.
- **Decision/options and tradeoffs:** Define local-ID grammar and whether external user IDs are exempt; choose where unique permanent PlayerShip IDs are allocated/enforced. Retain family-local catalog namespaces. Defensive duplicate-list rejection can complement, not replace, authoritative uniqueness.
- **Affected / remains blocked:** [ISSUE-009](issues.md#issue-009), [ISSUE-018](issues.md#issue-018); durable identity/import contract blocked; no auth implementation implied.

<a id="question-010"></a>
## QUESTION-010 — Authorize canonical documentation-only reconciliation

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/domain.md:4 stale path, :240 mutability scope, :275 instance wording; authority of ontology/ is settled.
- **Decision/options and tradeoffs:** Approve a later narrow correction of path, instance terminology and persistent-versus-runtime mutability wording, separately from behavioral decisions; or defer with known issues. No request to restate canonical authority.
- **Affected / remains blocked:** [ISSUE-010](issues.md#issue-010), [ISSUE-011](issues.md#issue-011), [ISSUE-012](issues.md#issue-012) and checker attribution [ISSUE-006](issues.md#issue-006); audit cannot edit canonical files.

<a id="question-011"></a>
## QUESTION-011 — Damage layers, hull vocabulary and recovery

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/domain.md:246 does not name spillover quantity; armor_hp is durability, reinforced-hull adds it; phasic shield mentions resistance but Stats has no mitigation/recovery fields.
- **Decision/options and tradeoffs:** Choose explicit spillover conservation and destruction overshoot semantics. Decide whether hull is an armor-HP alias, whether target/layer resistance and passive shield regeneration are desired or deliberately deferred. Keep constant multipliers if original balance is intended; resolve the documented source-formula conflicts before claiming external equivalence; choose whether raw/collision damage, shield bypass and explosion/target-size properties are intentionally excluded.
- **Affected / remains blocked:** [DRIFT-043](drift_developer_vs_internet.md#drift-043); [ISSUE-013](issues.md#issue-013), [DRIFT-006](drift_developer_vs_internet.md#drift-006), [DRIFT-007](drift_developer_vs_internet.md#drift-007), [DRIFT-008](drift_developer_vs_internet.md#drift-008); deterministic damage/recovery blocked.

<a id="question-012"></a>
## QUESTION-012 — Heat event/timer semantics

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/domain.md:132,252 says idle cooling and full cooling plus penalty; no executor exists.
- **Decision/options and tradeoffs:** Define trigger-held versus inter-shot idle, threshold crossing/clamping, shot cadence and sequential versus overlapping penalty. Preserve prose intent unless deliberately revised.
- **Affected / remains blocked:** [ISSUE-014](issues.md#issue-014), [DRIFT-013](drift_developer_vs_internet.md#drift-013); sustained-fire outcome blocked.

<a id="question-013"></a>
## QUESTION-013 — Executable effect and shared-energy scope

- [ ] **Open · decision owner: user**
- **Context/evidence:** Effects are explicitly prose at ontology/domain.md:116,153; C16 at :253 only supplies active payment and command-shield direction.
- **Decision/options and tradeoffs:** Keep descriptions until selected for implementation, then define only needed targets/magnitudes/ranges/durations/transitions. Resolve press/toggle/cycle meanings, ECM self/friendly/disabled-system scope, cloak cancellation interactions, command conversion/partial payment, damage immunity ordering and simultaneous energy drains/regeneration. Decide which weapons/specials/movement actions share energy; absent costs are not proven zero. Retain constant local regeneration or intentionally revisit the sourced nominal-rate/fullness distinction. For ECM, explicitly choose all-ship freeze versus the documented self-lock then enemy-only pulse; for cloak, clarify the sourced initial protected interval rather than infer damage immunity.
- **Affected / remains blocked:** [DRIFT-034](drift_developer_vs_internet.md#drift-034), [DRIFT-035](drift_developer_vs_internet.md#drift-035), [DRIFT-036](drift_developer_vs_internet.md#drift-036), [DRIFT-037](drift_developer_vs_internet.md#drift-037), [DRIFT-038](drift_developer_vs_internet.md#drift-038), [DRIFT-045](drift_developer_vs_internet.md#drift-045); [ISSUE-015](issues.md#issue-015), [DRIFT-003](drift_developer_vs_internet.md#drift-003), [DRIFT-007](drift_developer_vs_internet.md#drift-007), [DRIFT-009](drift_developer_vs_internet.md#drift-009), [DRIFT-011](drift_developer_vs_internet.md#drift-011); effect execution blocked, equip eligibility not blocked.

<a id="question-014"></a>
## QUESTION-014 — Active cooldowns, repeated equipment and stacking

- [ ] **Open · decision owner: user**
- **Context/evidence:** Three repeated passives are intentionally linear (ontology/model.rs:447–450); four repeated actives currently fit (:364–370).
- **Decision/options and tradeoffs:** Keep passive rule unless intentionally revisiting it. For actives choose legal repeats with per-occurrence/shared cooldowns or explicit repeat restrictions; define cooldown start on activation/deactivation/forced exhaustion. If percentage/conditional passives are later wanted, specify operator/base/order/caps rather than reinterpreting flat deltas.
- **Affected / remains blocked:** [DRIFT-041](drift_developer_vs_internet.md#drift-041); [ISSUE-016](issues.md#issue-016), [DRIFT-017](drift_developer_vs_internet.md#drift-017), [DRIFT-018](drift_developer_vs_internet.md#drift-018), [DRIFT-020](drift_developer_vs_internet.md#drift-020); active runtime and any broader stacking model blocked.

<a id="question-015"></a>
## QUESTION-015 — Retain arcade motion with precise timing vocabulary?

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/domain.md:82 says instant accel; sim/src/lib.rs:48–57 defines finite response; client/src/main.rs:54 uses frame delta.
- **Decision/options and tradeoffs:** Keep fast arcade response/shared tuning/spheres and describe it accurately; or intentionally reopen literal instantaneous/per-ship response. Decide whether strafe, separate axis rates, afterburner control and fixed-tick integration are needed for an authorized next phase. No Newtonian rewrite follows from research.
- **Affected / remains blocked:** [ISSUE-019](issues.md#issue-019), [ISSUE-020](issues.md#issue-020), [DRIFT-010](drift_developer_vs_internet.md#drift-010), [DRIFT-011](drift_developer_vs_internet.md#drift-011); stronger deterministic timing/control promises blocked.

<a id="question-016"></a>
## QUESTION-016 — Minimum weapon and payload behaviors

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/model.rs:154–175 lacks weapon delivery/range/ammo and missile guidance/status fields. Positive missile damage/speed excludes stationary/nondamaging payloads.
- **Decision/options and tradeoffs:** Keep illustrative weapon/missile labels and simple selected definitions; or specify needed hit delivery, range/falloff/spread/ammo/energy and targeting/guidance/EMP effects. Mines, drones and weapon munitions are now documented; broaden to selected ones only if scope deliberately changes. Specify whether a shot means one turret or an abstract battery, and keep munition configuration distinct from finite bullet counts. Distinguish one equipped weapon definition from physical hardpoints.
- **Affected / remains blocked:** [DRIFT-039](drift_developer_vs_internet.md#drift-039), [DRIFT-040](drift_developer_vs_internet.md#drift-040), [DRIFT-044](drift_developer_vs_internet.md#drift-044); [DRIFT-012](drift_developer_vs_internet.md#drift-012), [DRIFT-015](drift_developer_vs_internet.md#drift-015), [DRIFT-021](drift_developer_vs_internet.md#drift-021); hit resolution and broader payload semantics blocked.

<a id="question-017"></a>
## QUESTION-017 — Missile ammunition lifecycle

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/domain.md:142–143 explicitly says ammo per launcher per life and reload_s between launches.
- **Decision/options and tradeoffs:** Retain that settled simple budget/cadence, or intentionally revisit for magazines/reserves/replenishment after evidence. Specify remaining stock/timer reset at death/redeployment when runtime work is authorized.
- **Affected / remains blocked:** [DRIFT-014](drift_developer_vs_internet.md#drift-014); lifecycle execution blocked, no permission to silently reinterpret reload.

<a id="question-018"></a>
## QUESTION-018 — Equipment capacities, eligibility and purchase defaults

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/domain.md:98,175–178,243–247 settles model-fixed passive capacities, ≤4 actives, one size-fit weapon/one missile, empty ordinary modules on purchase.
- **Decision/options and tradeoffs:** Keep these rules; only intentionally revisit if the fetched stock/layout and eligibility examples are adopted and require rank/model eligibility, unlocked slots, optional empty weapons/payloads or different stock. Quality and owned-upgrade state remain excluded unless scope changes.
- **Affected / remains blocked:** [DRIFT-041](drift_developer_vs_internet.md#drift-041), [DRIFT-042](drift_developer_vs_internet.md#drift-042); [DRIFT-016](drift_developer_vs_internet.md#drift-016), [DRIFT-019](drift_developer_vs_internet.md#drift-019), [DRIFT-021](drift_developer_vs_internet.md#drift-021), [DRIFT-022](drift_developer_vs_internet.md#drift-022); no correction implied solely by external features.

<a id="question-019"></a>
## QUESTION-019 — Respawn, ship reselection and Open World launch

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/domain.md:209,249 has optional selection and conditional next life; no selected-ship/station structs.
- **Decision/options and tradeoffs:** Clarify boolean as local permission versus richer finite/conditional lives; separate reselection if needed. Choose selected-ship ownership/hangar membership, missing selection behavior and station/death destination policies. Four Hangar positions are a combat subset, not total fleet cap. Choose whether provided-ship event modes stay excluded; if included, separate control from permanent ownership. Do not infer Open Space death destination from station-undock evidence.
- **Affected / remains blocked:** [DRIFT-046](drift_developer_vs_internet.md#drift-046); [ISSUE-017](issues.md#issue-017), [DRIFT-024](drift_developer_vs_internet.md#drift-024), [DRIFT-026](drift_developer_vs_internet.md#drift-026), [DRIFT-027](drift_developer_vs_internet.md#drift-027); admission/respawn execution blocked.

<a id="question-020"></a>
## QUESTION-020 — Mode definition versus running session

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/model.rs:178–183 only stores category/respawn; ontology.md:63 mentions game instance; domain.md:186 says per-match state.
- **Decision/options and tradeoffs:** Retain generic scenarios and deferred objectives/generators; before authorized multiplayer work define participant/deployment/life identity and persistence through docking/sector transfer. Add group/team cardinalities only for accepted CQs and retrieved evidence, not genre assumptions. Decide whether category means entry context: Open Space includes PvP/PvE, and CO-OP reuses Arena objectives. Keep quests, time-limited events and availability separate from static mode definitions.
- **Affected / remains blocked:** [DRIFT-023](drift_developer_vs_internet.md#drift-023), [DRIFT-025](drift_developer_vs_internet.md#drift-025), [DRIFT-028](drift_developer_vs_internet.md#drift-028); runnable mode/session semantics blocked.

<a id="question-021"></a>
## QUESTION-021 — What does the positive price mean?

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/domain.md:96,247 requires one positive u32; currency and acquisition alternatives are absent; buy is construction only.
- **Decision/options and tradeoffs:** Keep an abstract local balancing currency and immediate catalog availability; or intentionally reopen acquisition eligibility/offers, zero-cost grants or construction when supported and in scope. Never treat missing payment in the demo as a game rule.
- **Affected / remains blocked:** [DRIFT-029](drift_developer_vs_internet.md#drift-029), [DRIFT-031](drift_developer_vs_internet.md#drift-031); economic transaction design and price-fidelity claims blocked.

<a id="question-022"></a>
## QUESTION-022 — What does permanent single-user ownership exclude?

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/domain.md:163,200 says owned forever; model.rs:406 permits multiple instances of a model with caller-provided IDs.
- **Decision/options and tradeoffs:** Keep strong no-transfer/no-expiry/no-disposal ownership, or clarify whether permanence only excludes combat loss. Decide whether multiple instances of the same model per owner are intentional. Anaconda sale is documented, while rentals, whole-ship trading and duplicate owned-model policy remain unverified. Permanent ownership does not promise endless official service or account transfer to the announced local-server version.
- **Affected / remains blocked:** [DRIFT-030](drift_developer_vs_internet.md#drift-030); ownership lifecycle and store uniqueness policy beyond [ISSUE-018](issues.md#issue-018) blocked.

<a id="question-023"></a>
## QUESTION-023 — If deferred context returns, where do its concepts attach?

- [ ] **Open · decision owner: user**
- **Context/evidence:** ontology/domain.md:19–23 excludes progression/economy and defers faction/tier; class tree has no corporate assets.
- **Decision/options and tradeoffs:** Only if [QUESTION-002](questions.md#question-002) reopens scope, distinguish faction/manufacturer/allegiance, account clearance/ship rank/item quality, crew/implant stat attachment, resources/offers, and collective ownership/control from PlayerShip.owner_id. Otherwise keep these research-only.
- **Affected / remains blocked:** [DRIFT-005](drift_developer_vs_internet.md#drift-005), [DRIFT-019](drift_developer_vs_internet.md#drift-019), [DRIFT-028](drift_developer_vs_internet.md#drift-028), [DRIFT-031](drift_developer_vs_internet.md#drift-031); speculative hierarchy expansion blocked.
