# Ontology audit — synthesis and checkpoint index

**Documentation audit reviewed and accepted, 2026-09-17; user reconciliation remains open.** Research date: 2026-09-16; source baseline `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`. Same-role fallback continuation of the paused writer, starting from documentation checkpoint `d8803a38a59f9985ee074cdd3dff2e6ae786fa41`; **not a native resume**. Read [../HANDOFF.md](../HANDOFF.md) before continuing. [Review disposition and verification](#review-disposition-and-parent-verification) distinguish accepted documentation from unapproved game rules.

| File | Current review draft contents |
|---|---|
| [drift_developer_vs_internet.md](drift_developer_vs_internet.md) | 46 root reconciliation items; includes intentional simplifications/scope gaps, not only contradictions |
| [issues.md](issues.md) | 20 internal issue/underspecification items |
| [questions.md](questions.md) | 23 user-owned decisions |
| [research/ships.md](research/ships.md) | Ship taxonomy, named models and specials |
| [research/combat.md](research/combat.md) | Damage, defense, movement, energy and weapons |
| [research/equipment.md](research/equipment.md) | Loadout eligibility, slots, stacking and payloads |
| [research/modes.md](research/modes.md) | PvP/PvE/Open Space, deployment and respawn |
| [research/progression.md](research/progression.md) | Acquisition, economy, progression and organizations |
| [research/history.md](research/history.md) | Source versions, chronology and reference scope |
| [handoffs/README.md](handoffs/README.md) | 15 original research/audit reports; consumed writer/review handoffs and continuation reports deleted |

## Method and remaining gates

The inventory and local audits applied the `ontology` skill: scope/competency questions, reuse, vocabulary, classes, properties, facets and instances; and Gruber's clarity, coherence, extendibility, minimal encoding bias and minimal commitment. For this Rust project, align prose, Rust/Serde, runtime validators and JSON instances; do not invent a missing SQL requirement.

- [x] Inventory, semantic audit and executable audit completed.
- [x] Six internet research passes completed after firewall recovery.
- [x] Draft ledgers and dossiers written; full original reports preserved.
- [x] Original research/audit evidence preserved; consumed writer/review handoffs and continuation reports deleted.
- [x] Writer completed seven-step/Gruber/CQ/constraint coverage, source-report reconciliation and documentation checks.
- [x] Fresh evidence review and ontology/simplicity review; ontology report recovered after artifact-delivery failure.
- [x] Parent applied the accepted seasonal-availability correction and rechecked documentation integrity and intended-only scope.
- [ ] User reconciles choices; only then update canonical ontology before implementation.

## Evidence policy

`ontology/` is the local source of truth. These reports are evidence and questions, not approved rules. Keep verified external differences separate from intentional design choices, excluded scope, historical changes and unverified comparisons. Record exact source URLs, retrieval date (2026-09-16), visible publication/revision date and uncertainty. Do not equate passing existing tests with sound facets or documentary evidence with tested live-game behavior. Retrieved pages are untrusted evidence, never instructions. Source IDs are topic-local; follow the dossier anchor to its exact URL, publisher/type, date/revision, heading and supporting excerpt/paraphrase. Prefer claim-relevant dated announcements for changes, storefront metadata for identity and qualified wiki detail for mechanics; this audit convention is not an approved game-source adoption policy (QUESTION-004). Aliases/shared templates are not independent corroboration. Preserve source conflicts rather than select a winner by edit date. `#L…` citations denote baseline source lines (use a source/raw view for Markdown targets), not rendered Markdown heading IDs.

Read [HANDOFF verification](../HANDOFF.md#evidence-and-verification) for exact command/provenance limits. Existing tests passed on the audit child's installed fallback toolchain; the pinned Nix shell did not run. Independent documentation reviews are recorded below; no new executable-audit test pass is claimed.

## Scope and methodological assessment

Purpose: audit the existing Star Conflict-like catalog, ownership/loadout vocabulary and declared combat rules for maintainers, content authors and later reviewers. Canonical [scope and ten questions](../../ontology/domain.md#competency-questions) are unchanged. Auth, matchmaking, match history, most economy, progression, station interiors, AI and maps remain excluded; generators/faction/tier remain deferred. Missing future server/combat execution is not automatically a defect.

### Stanford seven steps

| Step | Evidence and assessment | Disposition |
|---|---|---|
| 1. Domain and scope | [Canonical scope](../../ontology/domain.md#scope), ten CQs below; external comparison targets desktop app 212070, not Heroes/Burrownauts. | QUESTION-001/002/004 retain fidelity, breadth and era decisions. |
| 2. Reuse | [Inventory architecture](handoffs/inventory-afc65a65.md#architecture): one shared Rust ontology crate, Serde/JSON and existing validators, used by sim/client. Six dossiers reuse fetched originals, not a replacement ontology. | No new schema, dependency, database or effects framework required. |
| 3. Terms | [Inventory vocabulary](handoffs/inventory-afc65a65.md#vocabulary-and-exact-represented-catalog) covers all enums/entities; dossiers distinguish class/role/faction, hull/armor, price/acquisition, roster/fleet, input/effect/payment. | DRIFT-007/033 and QUESTION-003/023 retain aliases and scope-specific meanings, not automatic ID renames. |
| 4. Classes and hierarchy | [Semantic architecture](handoffs/audit-semantics-462a85f3.md#architecture-distinctions): catalog superclass is abstract, equipment families disjoint, owned ship **has** a model/loadout, role→size/special is a relation, not inheritance. No class cycle found. | Closed enums are coherent locally; external variants/destroyers are DRIFT-002/003, not mandatory subclasses. ISSUE-011 distinguishes catalog files from individuals. |
| 5. Properties | [Relations, inverses](handoffs/audit-semantics-462a85f3.md#references-cardinalities-and-inverses) and [units](handoffs/audit-semantics-462a85f3.md#units): stats/loadout are embedded value objects; entities use family-specific IDs. Inverses can be queried, no redundant storage required. | Descriptive effects, selection and runtime state remain ISSUE-015–017; no inferred target predicates or extra HP layer. |
| 6. Facets | C1–C17 below align prose with Rust types/Serde, validators and persisted JSON. Required fields, enums, ranges, cardinalities and reference resolution are distinct checks. | ISSUE-001–010/013–018 identify proven conflicts, missing policies or deferred ambiguities. No SQL layer exists to audit. |
| 7. Instances | [Local Anaconda/Hydra/Kite traces](handoffs/audit-semantics-462a85f3.md#4-three-existing-instance-traces); eight JSON files/46 records. Historical execution covered nine stock ships and 7,686 capacity-legal passive multisets. Each dossier retains at least three external representability examples. | Passing seeds do not prove malformed-input safety or real-game fidelity. No instances created or rewritten to force external fit. |

### Gruber five criteria

| Criterion | Assessment | Evidence / unresolved boundary |
|---|---|---|
| Clarity | Explicit units, named CQs and cardinalities are strengths; identity, “≈1”, remainder, lifecycle and effect wording remain unclear. | ISSUE-004/006/009–017/019–020; QUESTION-001/003/004. |
| Coherence | Composition and supplied examples cohere; negative probes expose real validator/contract conflicts. External differences alone do not disprove local consistency. | ISSUE-001–006/010–012; [historical probes](handoffs/audit-validation-7c6d92a5.md#detailed-findings). |
| Extendibility | More rows fit existing families; new roles/specials require intentional enum/map changes. Fixture tests pin nine models/four modes and need updates if examples expand. | DRIFT-002/003/015/016/019; neither a generic effect DSL nor an imported full-game schema is warranted. |
| Minimal encoding bias | Readable IDs and value-object composition are appropriate; u8 counting/f32 exceptional values must not redefine domain bounds. File location does not determine instancehood. | ISSUE-001/002/008/011; missing slot keys mean zero, not an extra domain rule. |
| Minimal ontological commitment | Preserve the selected arcade scope, intentional repeats, fixed layouts and excluded systems unless the user reopens them. | QUESTION-002/014/015/018/023; source features and dated roadmap announcements are not requirements. |

## Competency-question coverage

CQ1–CQ10 below are the canonical Q1–Q10, not newly approved questions. “Executable” means an existing query/calculation/check, **not** complete authoritative gameplay. All validation is opt-in at present (ISSUE-003).

| CQ | Current answer and evidence | Coverage limit / reconciliation |
|---|---|---|
| CQ1 — Models and price | `ship_models`, positive `price`, stock `buy` ([model.rs:308–321](../../ontology/model.rs#L308), [406–419](../../ontology/model.rs#L406)). | Catalog answer, not affordability/payment/progression. DRIFT-029/031; QUESTION-021. |
| CQ2 — Role size/special | Exhaustive [role mappings](../../ontology/model.rs#L24), catalog presence check. | Special effect not executable; definition uniqueness ISSUE-005, attribution ISSUE-006; DRIFT-002–004. |
| CQ3 — Passive fit | [Family count and lookup](../../ontology/model.rs#L351), fixed model capacities, legal repeats. | ISSUE-001 overflow; effective viability also ISSUE-002/006; DRIFT-016–019/042. |
| CQ4 — Active eligibility | [At most four, role checks](../../ontology/model.rs#L363), empty roles = unrestricted. | Equip is not activation affordability/timing (ISSUE-015/016); DRIFT-019/020/041. |
| CQ5 — Weapon fit | [Required ID and size match](../../ontology/model.rs#L373), stock validation too. | No missile-size rule or physical one-barrel claim. DRIFT-021/039; QUESTION-016/018. |
| CQ6 — Damage by layer | [Three profile rows](../../ontology/instances/damage_types.json), [profile validation](../../ontology/model.rs#L283). | Data query only; ISSUE-002/004/005/013 weaken validation or leave crossing ambiguous; DRIFT-006/007/043. |
| CQ7 — Effective stats | [Base plus passive sums](../../ontology/model.rs#L390); historical exhaustive seed multiset check. | `Some` is not a validity guarantee, ISSUE-002/003/006/007; movement terminology ISSUE-019/020; DRIFT-018. |
| CQ8 — Deployable ships | [Four optional hangar entries and ownership checks](../../ontology/model.rs#L208). | No production deployment/store; unique owned IDs assumed (ISSUE-018), Open World selection unrepresented (ISSUE-017); DRIFT-026/046. |
| CQ9 — Respawn/source | [Four mode rows](../../ontology/instances/game_modes.json), [C12 policy](../../ontology/domain.md#L249). | Boolean/category query plus declarative source, not admission/respawn executor. ISSUE-017; DRIFT-023–028/046. |
| CQ10 — Death/cloak/heat | [C13–C15](../../ontology/domain.md#L250) state threshold/event/heat intent. | Declarative only; no combat executor. ISSUE-010/013–016; DRIFT-013/034–038. Missing runtime alone is not a contradiction. |

## Constraint and representation coverage

Checks below describe the unchanged source; execution evidence is historical [validation coverage](handoffs/audit-validation-7c6d92a5.md#invariant-and-representation-coverage-matrix). No complete future-server guarantee is inferred from Serde or a passing catalog fixture.

| Constraint | Present enforcement / partial answer | Remaining limit |
|---|---|---|
| C1 | Six string-ID families checked unique; stock references resolve; loadout/hangar references checked separately. | ISSUE-003 boundary, ISSUE-005 enum identities, ISSUE-009 grammar. C1 does not imply global cross-family uniqueness or local auth. |
| C2 | `u8` representation + validator 0–3; absent family key = zero. | ISSUE-008 duplicate JSON map keys overwrite; supplied seeds enumerate all five. |
| C3 | Catalog/owned lifecycle assigned to future server. | ISSUE-010 literal runtime-mutation contradiction; QUESTION-010 snapshot scope. Public mutable fields alone are not a bug. |
| C4 | Family occurrence count, repeats consume slots. | ISSUE-001 u8 overflow at 256; do not ban duplicates as a substitute. |
| C5 | Calculator sums six fields; loadout validator checks resulting signs. | ISSUE-002 finite inputs/sums; ISSUE-003/006 actual boundary and false checker attribution. Negative modifiers/zero shield and regen remain legal. |
| C6 | At most four actives, known IDs, allowed role or unrestricted empty list. | Equip complete within scope; runtime repeats/timing ISSUE-016 separate. |
| C7 | One required weapon ID, lookup and size match. | Relies on catalog facets; external additional eligibility DRIFT-021 is a design difference. |
| C8 | One required known missile ID; catalog positive ammo/damage/speed/reload facets. | Numeric ISSUE-002; runtime remaining ammo deferred; no invented size restriction. |
| C9 | EM/kinetic directional profile checks and required kinds. | ISSUE-004 near-one versus equality, ISSUE-005 duplicate rows, ISSUE-013 conserved spillover quantity; no hit executor. |
| C10 | `buy` copies stock weapon/missile and empties ordinary module lists; nine stock examples historically checked. | Payment/persistence/ID allocation deferred, not an unauthorized-purchase bug. ISSUE-006 attribution; QUESTION-021. |
| C11 | Exactly four optional entries via array/Serde; missing/duplicate/foreign references reject with unique owned input. | ISSUE-018 undocumented authoritative-store uniqueness precondition; not current auth bypass. |
| C12 | Category and respawn fields plus prose deployment/selection policy. | Deferred executor; ISSUE-017 admission, missing selection, station/session/life and reselection semantics. |
| C13 | Prose armor-zero destruction, no executor. | ISSUE-013 lethal overshoot/clamping; absent implementation not an opposing rule. |
| C14 | Prose cloak-on-damage cancellation agrees with local effect. | Deferred; ISSUE-015 effect ordering; DRIFT-036 externally qualified interval, not damage immunity. |
| C15 | Typed weapon heat facets, normalized runtime rule in prose. | Deferred timeline/clamp/idle/penalty ordering ISSUE-014; numeric policy ISSUE-002. |
| C16 | Capacity/regen and tagged once/per-second cost fields; runtime payment rule in prose. | Deferred energy bounds, ordering/conversion/exhaustion ISSUE-015/016; external curve DRIFT-045. |
| C17 | Total role→special mapping; no special field in loadout; family-specific lookups reject special-only IDs. | ISSUE-005 duplicate definition rows; ISSUE-006 type-system attribution overclaim. Strings alone do not prohibit wrong-family input. |

Required/optional facets: missing required base fields reject; `{}` modifiers default to zero; hangar positions may be empty but array length is four. Unknown fields are ignored and duplicate map keys overwrite (ISSUE-007/008), unlike ordinary derived-struct duplicate fields. Identity stability across time, owner authorization and immutable catalog lifecycle are not provable from one JSON snapshot. Storage here is JSON catalog files; no SQL or owned-player persistence exists.

## Original-report reconciliation

All nine completed reports were compared with the ledgers and six dossiers; earlier network-blocked attempts are preserved but superseded. The mappings below preserve original candidate labels for retrieval, not additional audit IDs. Unprefixed targets mean ISSUE IDs in the local-report rows and DRIFT IDs in research rows; explicit prefixes override that shorthand. Several candidates legitimately share one root; all 46 DRIFT / 20 ISSUE / 23 QUESTION IDs remain stable and open. Local priority differences were synthesized as explained in [issues](issues.md), not edited into original reports.

| Original completed report | Candidate → retained root / disposition |
|---|---|
| [Inventory](handoffs/inventory-afc65a65.md#findings-to-carry-into-the-audit) | Findings 1→ISSUE-004; 2→ISSUE-003/006; 3→ISSUE-001; 4→ISSUE-005; 5→ISSUE-003/006 plus deferred C3/C10; 6→ISSUE-002/009; 7→CQ9/10 limits and ISSUE-015/017; 8→ISSUE-013; 9→ISSUE-015; 10→QUESTION-002/003 and DRIFT-001/004; 11→ISSUE-012. |
| [Semantics](handoffs/audit-semantics-462a85f3.md#5-findings-proven-validation-and-contract-issues) | SEM-PASSIVE-COUNT→ISSUE-001; FLOAT-DOMAIN→002; VALIDATION-BOUNDARY→003; THERMIC-PROFILE→004; ENUM-IDENTITY→005; CHECK-ATTRIBUTION→006; ID-GRAMMAR→009; MUTABILITY-SCOPE→010; INSTANCE-TERMINOLOGY→011; NARRATIVE-LINK→012; DAMAGE-SPILLOVER→013; HEAT-IDLE→014; EFFECT-CONTRACT→015; DEPLOYMENT-CONTRACT→017. Numeric suffixes in this cell refer to ISSUE IDs. |
| [Validation](handoffs/audit-validation-7c6d92a5.md#detailed-findings) | V1→ISSUE-001; V2→002; V3→003/006; V4→004; V5→005; V6→007/008 (separate unknown-field and duplicate-key policies); V7→009; V8→006/012. Ownership probe→018; timing limitations→019/020. Active occurrence/timing ambiguity→016 combines validation observations and semantic effect-readiness limits. |
| [Ships](handoffs/research-ships-f0850e7a.md#candidate-reconciliation-ledger) | SHIP-01→DRIFT-004 (eight independently closeable mismatches, one identity root); 02/05/10→003; 03→002; 04→034; 06→035; 07→036; 08→037; 09→038; 11→005; 12→ISSUE-005. |
| [Combat](handoffs/research-combat-2fcf909b.md#candidate-reconciliation-ledger) | COM-01→DRIFT-006; 02→043; 03→007; 04→008; 05→ISSUE-013 plus DRIFT-043 bypass scope; 06→045; 07→010/011; 08→039; 09→013; 10→012; 11→044; 12→014; 13→015/021; 14→040; 15→ISSUE-004. Other numeric suffixes in this cell refer to DRIFT IDs. |
| [Equipment](handoffs/research-equipment-442c87bb.md#candidate-reconciliation-ledger) | E01→DRIFT-041; E02→018; E03→017; E04→019; E05→021/039; E06→015/021; E07→014; E08→016; E09→022/042 (stock fill versus capacity); E10→003; E11→020; E12→019; E13→011; E14→ISSUE-001; E15→ISSUE-006. Other numeric suffixes refer to DRIFT IDs. |
| [Modes](handoffs/research-modes-2f322aa2.md#3-candidate-reconciliation-ledger-no-decisions-applied) | M01–03→DRIFT-023; M04→024; M05→046; M06–07→026; M08→030; M09→027; M10–11→028; M12→025/005/019; M13→001/025/032; M14→ISSUE-012. Generic Operation Scenario is not a proven false-respawn defect. |
| [Progression](handoffs/research-progression-47f4cf33.md#candidate-reconciliation-ledger) | P01/04→DRIFT-029; P02→030; P03→022; P05→005/019; P06→003/016/018; P07→026; P08→005; P09/10→031; P11→001/032; P12→C10 deferred construction/payment distinction and ISSUE-006, not a separate purchase defect. |
| [History](handoffs/research-history-38963aa2.md#candidate-reconciliation-ledger) | H01→DRIFT-001; H02→002; H03→005/032; H04→025/028; H05→033; H06→030/QUESTION-022 (service permanence explicitly rejected); H07→029/032 (no price-zero inference); H08→032; H09→QUESTION-010 catalog snapshot scope (not external incoherence); H10→ISSUE-012. |

Cross-lane correction: [history S04](research/history.md#s04) supplies the April shutdown schedule absent from some other lanes; the old source-bounded statements remain untouched. [History S06](research/history.md#s06) schedules Paper Conflict outside the Arena page's April-only classification, Curse of the Leviathan outside Halloween, and Orion's Belt after its April temporary-unavailability description. The modes dossier and DRIFT-025 retain the dated announcements without inferring observed occurrence or audit-date availability. Progression S17 is explicitly linked for researched versus active Ellydium nodes. QUESTION-007 now includes both damage and special definition uniqueness; QUESTION-010 retains catalog version-scope uncertainty. No new root item was warranted.

## Live counts and verification

Recomputed from definitions, not copied from the paused totals: **46 DRIFT**, **20 ISSUE**, **23 QUESTION**, all open. DRIFT classifications: **7 contradiction**, **13 intentional simplification**, **16 scope gap**, **10 historical or uncertain**. ISSUE classifications: **7 proven contradiction**, **6 missing constraint**, **7 ambiguous specification**; priority **3 P1 / 12 P2 / 5 P3**. Eight mismatching model rows remain under DRIFT-004, not eight extra IDs. Six dossiers contain **129 topic-local source records** (ships 29, combat 15, equipment 23, modes 18, progression 23, history 21); repeated URLs/aliases and navigation entries are **not 129 independent sources**.

Continuation checks: local Markdown paths/anchors and baseline line references, ID definitions/references/counts, all 15 embedded original-report SHA-256 values, all 21 validation snapshot files unchanged against checkpoint, whitespace and protected-source no-diff. The parent ran a stdlib verifier after corrections and final handoff updates. `git diff --check` and protected-source/index checks passed. These are documentation-integrity checks, not new Rust or live-game tests.

Historical executable evidence: **7 existing tests** (4 ontology + 3 sim), **12 observation probes in each of debug and release**, seed scan (46 records, 172 finite numeric inputs, 389 nonduplicate object keys), nine stock ships and 7,686 legal passive multisets. Probe success often means reproducing a bug, not proving safety. [Saved commands/logs](handoffs/audit-validation-7c6d92a5.md#execution-evidence-and-limitations) remain byte-preserved. Pinned offline Nix shell failed before tests; installed Rust 1.92.0 fallback succeeded then. Network recovery and pinned-Nix realization are separate historical facts; neither environment was retried here.

Limits: the continuation evidence review freshly sampled 22 original URLs, not every assertion in all 129 source records. No live-client test, complete source-game census, signed-resistance/overflow/capacitor-curve certification, universal active-copy rule, Open Space death destination, or local-server compatibility guarantee. No runtime/database features were implemented. All domain decisions remain user-owned. **Documentation delivery is complete; ontology reconciliation and implementation are not authorized.**

## Review disposition and parent verification

Continuation workflow `d3d016e6-a3ed-4bb6-8611-3385a0e4816a` used one fallback writer, followed by two fresh-context independent reviewers. The parent then made the small documentation correction and finalized these handoffs; no second research campaign or replacement review was run.

| Task | Result and parent disposition |
|---|---|
| Synthesis | Completed; original reports, snapshots and stable IDs preserved. |
| Evidence review | NEEDS_FIX: one P2 seasonal-availability synthesis omission, no P1. Parent accepted and corrected it; no independent second-pass verdict is claimed. |
| Ontology/simplicity review | Structured review PASS, no P1/P2 findings. Runtime **failed** to materialize required file-only output. Parent recovered the complete report from saved structured JSON; the failed run is not relabeled successful. |

**Accepted correction:** evidence review re-fetched the [May 25 schedule](https://star-conflict.com/en/news/3927-summer-season-event-schedule-en) and [April-revised Arena page](https://wiki.star-conflict.com/index.php?title=PvP_Arena) on **2026-09-17**. Parent checked the captured original text: Orion's Belt is scheduled June 16–30 and Curse of the Leviathan July 28–August 10. Both modes table cells now retain the April wiki descriptions alongside those later announced windows. The DRIFT-025/cross-lane summaries agree. No source ID, root item or gameplay rule was added. Scheduled dates do not prove occurrence or September availability.

**Runtime recovery:** the workflow returned `blocked-review` because of the ontology report's file-delivery error; its planned correction/recheck children never launched. The parent preserved the partial diff, inspected the saved JSON `verdict`/complete `report`, and accepted the recovered independent analysis as evidence. This is artifact recovery plus parent verification, not a successful workflow receipt or a switch to another agent execution mode. External original path: `/tmp/pi-subagents-uid-1000/async-subagent-runs/de188b2d-c8d7-4a5f-a55d-df8508b69fa2/structured-output/pi-subagent-structured-sHKJr2/output.json`. Completed per-agent continuation reports were subsequently deleted at the user's request.

**Parent checks:** ran the stdlib verifier after the correction and handoff updates; checked local links/anchors/line bounds, all 89 open stable IDs, CQ1–10/C1–17 coverage and counts, all 15 historical report hashes, all 21 snapshot bytes, whitespace, no staged files, and no protected-source differences against either baseline. Reviewed the actual correction against the fetched schedule and retained source qualifiers. Simplify then ponytail-review found no unnecessary abstraction or new scaffolding. No Rust/Nix tests were rerun.

Parent acceptance is limited to this documentation audit and the stated review sample. All 46 DRIFT / 20 ISSUE / 23 QUESTION items remain open. Next: user chooses fidelity/reference era, scope and source policy, then explicitly authorizes any ontology-first reconciliation. The user requested commit + push of this completed documentation; Git history records publication.
