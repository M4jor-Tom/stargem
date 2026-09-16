# Handoff: research-ships — ba43f0b4

Checkpoint: 2026-09-16, user-requested pause. Parent-curated from runtime records and actual files; not a new completed child report.

- **State:** COMPLETED / SUPERSEDED: network-blocked attempt
- **Role:** `delegate`
- **Child run:** `ba43f0b4-6fda-4956-b4f6-e81729848a74`
- **Workflow / key:** `08aa52d7-d2a2-4091-b856-28049dd1b39f` / `research-ships`
- **Mission:** `28a33a06-b296-4a78-8ae0-079881816489`
- **Repository:** `/home/theta/repos/stargem.nix`, branch `master`
- **Source baseline:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`
- **Session (machine-local, optional for recovery):** `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/2026-09-16T18-47-01-030Z_01a0ab8b-30a5-761c-90d3-cc64125796e1/54042ca3-b9ea-4d74-9771-39a6f88978d1/run-0/session.jsonl`
- **Runtime status (machine-local):** `/tmp/pi-subagents-uid-1000/async-subagent-runs/ba43f0b4-6fda-4956-b4f6-e81729848a74/status.json`

## Authority and boundaries

Read root `AGENTS.md` and `docs/HANDOFF.md` before work. `ontology/` remains canonical and unchanged. This is documentation/research, not authorization to change gameplay. No commit/push/merge/install is authorized. One writer per checkout. Preserve untracked audit files: they are the current work, not disposable clutter. Other agents' full research is intentionally file-backed to protect parent context.

## Resumption (for Agent)

No remaining work for this attempt. Use [research-ships-f0850e7a.md](research-ships-f0850e7a.md) instead. The user fixed the firewall; do not propagate this attempt's access-blocked conclusion as current.

## Native continuation or fallback

Recheck live availability before any resume. Completed work normally requires no resume.

Not rechecked in the latest retained-child list; no resume is needed. Use the preserved report below. If genuinely necessary, inspect status/children.list first; otherwise start a same-role focused continuation from this handoff.

## Verification and acceptance

The original report below states exactly what this child checked and what remains uncertain. Completion of a research child does not mean that document synthesis, independent evidence review, ontology review or user reconciliation has completed. Follow [the main checkpoint](README.md) for dependency order and pending reviews.

## Preserved original output

Original managed artifact: `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/research/ships.md`

SHA-256 of original artifact: `5d09cb8e19397d6176e20762bf9232716b7c55373eae52de39208dbaaf594d0f`

The complete output is embedded below so recovery does not depend on session/temp-file retention. Original relative code paths refer to the repository root unless the report says otherwise. Historical access failures stay historical.

---

# Ship taxonomy and role-special audit — PARTIAL, external verification blocked

- **Audit/attempt date:** 2026-09-16.
- **Repository:** `/home/theta/repos/stargem.nix`, branch `master`, HEAD/baseline `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`.
- **Exclusive scope:** Star Conflict ship taxonomy, role/size mapping, faction/ship identity, destroyers and later/special/alien/Ellydium coverage, and role-special identity/behavior. Economy and unrelated combat mechanics are excluded.
- **Authority:** `ontology/` is canonical; `ontology.md` is a local source narrative, not an external source for Star Conflict. Root `AGENTS.md` and `docs/HANDOFF.md` were checked first and are absent.
- **Outcome:** local audit completed; **no public game page was retrieved**. There are **zero externally verified contradictions, zero externally corroborated agreements, and zero verified real-game examples** in this report. Unretrieved candidate URLs and task-supplied terms are not evidence.
- **Infrastructure decision:** after reporting failures, the supervisor confirmed the same outage across sibling lanes, no approved proxy/cache, and directed a partial report without further retries or network changes.
- **Change boundary:** only this managed report was written as a deliverable; temporary failed-fetch output is under `/tmp/stargem-ships-audit/`. No repository/source/ontology edits, commits, branches, pushes, package installations, nested agents, or staged files.

## 1. Method and evidence boundaries

Read in full: the requested ontology skill; `ontology/domain.md`; `ontology.md`; `ontology/model.rs`; `ontology/instances/ship_models.json`; `ontology/instances/special_modules.json`; `ontology/instances/active_modules.json`; `README.md`; `sim/src/lib.rs`; `client/src/main.rs`; and the provided inventory at `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/69d652a5-b7d3-47be-83bd-4c3f7677ee17/audit/inventory.md`. The inventory was orientation, not a substitute for reading the relevant files.

Applied the Stanford seven steps and all five Gruber criteria below, without modifying ontology semantics. Followed callers of `ShipRole::special`, `ShipRole::size`, `ShipModel::size`, and `ShipSize` through validation, tests, client display, and collision sizing.

**Classification rules:**

- **Verified contradiction:** exact local assertion conflicts with retrieved, attributable external evidence. None can be established here.
- **Deliberate simplification:** an explicit local modeling commitment. This label describes local intent/shape; it does not prove that Star Conflict differs.
- **Coverage gap:** a question or possible feature cannot be represented or answered. Absence is not a contradiction.
- **Uncertainty:** ambiguity, unknown provenance, unavailable external verification, or unknown currentness.
- **Historical change:** requires dated evidence for change over time. No external historical change was established.
- **Internal logical issue:** a locally demonstrable definition/validation mismatch, independent of external game fidelity. One narrowly scoped issue is documented below; the current records do not contain its counterexample.

### Competency questions for this lane

| Question | Local answer | External answer/status |
|---|---|---|
| Q-S1. Which ship sizes/classes and roles exist, and how do they relate? | Exactly three size enum values and nine roles; role determines size. | Blocked. Actual class vocabulary, completeness, and time scope unverified. |
| Q-S2. Does role determine an exact special, a family, or a default with variants? | Exact one-to-one role → `SpecialModuleKind`; no model override. | Blocked. The task's requested variants are research questions, not established facts here. |
| Q-S3. What do each named model's class, role, and special actually denote? | Nine catalog records and derived specials listed below. | All nine external identities unverified. Matching a name is not enough. |
| Q-S4. Do factions alter ship characteristics or special identity? | No faction field/relation; faction/tier explicitly deferred. | Blocked; no faction traits inferred or assigned. |
| Q-S5. Can destroyer, later, special, alien, or Ellydium ships fit the local taxonomy? | No such typed class/faction vocabulary. Additional instances can fit only the existing role/size/special commitments. | Their actual game classification and counterexamples were not verified. |
| Q-S6. What are the activation, target, duration, cancellation, and effect semantics of the nine specials? | Two activation labels plus brief prose; several details unspecified. | Blocked; requested terminology crosswalk remains unconfirmed. |
| Q-S7. Does the data enforce the claimed special cardinality? | Role→kind mapping is bijective; validator checks presence of each kind, not uniqueness of special-definition rows. | Internal question, answered without web evidence. |
| Q-S8. Are facts current, historical, or intentionally independent design choices? | Canonical scope says “Star Conflict-like”; no external source/version metadata on records. | No live-game date or historical target can be inferred. |

## 2. Retrieval record and freshness

### Starting navigation angles

1. Official-hosted wiki main index → intended taxonomy, ship, and special-module pages.
2. Official Star Conflict home/news → intended dated announcements for later ships, destroyers, and Ellydium.
3. Public search → intended nonredundant discovery of role-special variants and category pages.
4. Steam product page → intended publisher/product identity and broad overview.

All failed at connection setup, not a returned anti-bot page or HTTP refusal. Public MediaWiki APIs on the failed wiki host were not retried: the problem was host connectivity, not an unavailable wiki route. The requested targeted gap-closing pass and roughly 10–20-page substantive coverage could not begin. No search snippets were obtained or used.

### Attempt log

Common first-attempt command shape:

```sh
curl -L --connect-timeout 10 --max-time 35 -sS -D "$SCRATCH/name.headers" \
  'EXACT_URL' -o "$SCRATCH/name.html"
markitdown "$SCRATCH/name.html" > "$SCRATCH/name.md"
```

`SCRATCH=/tmp/stargem-ships-audit`. The initial shell sequence did not guard conversion after failed downloads; the three `markitdown` invocations therefore failed with `FileNotFoundError`. Their redirected Markdown files are empty, not converted evidence. Later invocations used `&&` to avoid this misleading failure cascade.

| ID | Navigation label, not a retrieved article title | Exact URL | Intended publisher/type | Result on 2026-09-16 |
|---|---|---|---|---|
| F1 | Wiki Main Page | https://wiki.star-conflict.com/index.php?title=Main_Page | Community-maintained official-hosted wiki | `curl: (28) Connection timed out after 10002 milliseconds`; no HTML; conversion failed because input did not exist. One subsequent IPv4 attempt also failed after 8002 ms, HTTP `000`, empty remote IP. |
| F2 | Star Conflict official home | https://star-conflict.com/en/ | Official game site, navigation toward announcements | `curl: (28) Connection timed out after 10002 milliseconds`; no HTML; conversion input absent. |
| F3 | Search discovery | https://www.google.com/search?q=site%3Awiki.star-conflict.com+special+modules+destroyer+Ellydium | Search engine; discovery only | `curl: (28) Connection timed out after 10002 milliseconds`; no HTML/snippet; conversion input absent. |
| F4 | Steam product seed | https://store.steampowered.com/app/212070/Star_Conflict/ | Steam publisher/product storefront | IPv4 `curl` timed out after 8001 ms, HTTP `000`, empty remote IP; guarded conversion did not run. |

IPv4 probe command shape:

```sh
curl -4 -L --connect-timeout 8 --max-time 20 -sS \
  -w 'http_code=%{http_code} remote=%{remote_ip}\n' \
  'EXACT_URL' -o "$SCRATCH/name.html" && \
  markitdown "$SCRATCH/name.html" > "$SCRATCH/name.md"
```

Diagnostic evidence: standard-library `socket.getaddrinfo` resolved `wiki.star-conflict.com` to `23.109.154.234`, `star-conflict.com` to `185.253.20.214`, and `store.steampowered.com` to `2.20.33.213`. `http_proxy`, `https_proxy`, `HTTP_PROXY`, `HTTPS_PROXY`, `ALL_PROXY`, and `NO_PROXY` were unset. This establishes failed outbound attempts despite DNS resolution, not a conclusion that the public sites were globally unavailable. No environment/network settings were changed.

### Source register limitation

**Useful retrieved external sources: none.** For F1–F4, actual page title, retrieval date, article/update/patch date, relevant heading, faithful excerpt, and supported gameplay claim are all **unavailable**. `2026-09-16` is the attempt/audit date, not a successful retrieval date. No wiki statement is labeled current. No original page, current patch, or conflicting external source pair was available to evaluate.

For resumed research, every useful page must supply: actual title; exact resolved URL; official announcement vs official-hosted community wiki vs community classification; successful retrieval date; visible article/update/patch date or `unknown`; relevant heading; short excerpt/paraphrase; and the exact supported claim. A wiki edit date alone does not prove that every table reflects a current patch.

## 3. Verified local taxonomy and relationships

### Vocabulary and cardinalities

| Term/relation | Exact local meaning and range | Anchor |
|---|---|---|
| `ship-size` | `frigate` (big), `fighter` (medium), `interceptor` (small). This is a local size taxonomy, not a verified external terminology crosswalk. | `ontology/domain.md:59–64`; `ontology/model.rs:10–16` |
| `ship-role` | Nine mutually exclusive enum values, one per model. | `ontology/domain.md:62–64,95`; `ontology/model.rs:14–16,110–119` |
| `model-has-role` | Many models → exactly one role. | `ontology/domain.md:197` |
| `role-has-size` | Each role → exactly one size; multiple roles can share a size. | `ontology/domain.md:195,212–218`; `ontology/model.rs:29–35` |
| `role-has-special` | One role ↔ one special-module kind. | `ontology/domain.md:196,220–232`; `ontology/model.rs:37–50` |
| `special-module` | Built into role, dedicated fifth slot, not an equipable/catalog item, never a loadout field. | `ontology/domain.md:51,145–153,254`; `ontology/model.rs:102–107,186–192` |
| `activation` | `press` or `toggle`, with no typed special cost/cooldown/duration/target/effect parameters. | `ontology/domain.md:149–153`; `ontology/model.rs:65–67,102–107` |
| `player-ship` → model | Owned instance references one model; loadout has no special selector. | `ontology/domain.md:162–180,201`; `ontology/model.rs:186–200` |
| faction/tier | Explicitly deferred, not a current field on `ShipModel`. | `ontology/domain.md:22–23`; `ontology/model.rs:110–119` |
| runtime state | Includes shield/armor/energy/heat/cloak and active-module cooldowns; no explicit special state machine. No wire form yet. | `ontology/domain.md:186–189` |

Terms requested for research—destroyer, alien, Ellydium, disintegrator, guided torpedo, phase shield, overdrive, diffusion shield, microwarp, metastable field—do **not** gain verified definitions merely by appearing in this task. The local IDs below are established; equivalence to those terms remains an external question.

### Complete local role table and named catalog

Every row is independently read from the repository. **These are not verified Star Conflict assignments.**

| Local model and instance anchor | Local role | Derived local size | Derived local special | Mapping anchor |
|---|---|---|---|---|
| Anaconda, `ontology/instances/ship_models.json:2–5` | engineer | frigate | drones | `ontology/model.rs:32,40` |
| Styx, `ontology/instances/ship_models.json:6–9` | long-range | frigate | sniper-weapon | `ontology/model.rs:32,41` |
| Mammoth, `ontology/instances/ship_models.json:10–13` | guard | frigate | phasic-shield | `ontology/model.rs:32,42` |
| Hydra, `ontology/instances/ship_models.json:14–17` | tackler | fighter | cloak | `ontology/model.rs:33,43` |
| Phoenix, `ontology/instances/ship_models.json:18–21` | gunship | fighter | overclock | `ontology/model.rs:33,44` |
| Lynx, `ontology/instances/ship_models.json:22–25` | command | fighter | command-shield | `ontology/model.rs:33,45` |
| Shrike, `ontology/instances/ship_models.json:26–29` | covert-ops | interceptor | plasma-web | `ontology/model.rs:34,46` |
| Wolfhound, `ontology/instances/ship_models.json:30–33` | recon | interceptor | hyper-propulsion | `ontology/model.rs:34,47` |
| Kite, `ontology/instances/ship_models.json:34–37` | ecm | interceptor | em-surge | `ontology/model.rs:34,48` |

The catalog contains exactly one example per role and three examples per size. It supplies no external ship ID, source URL, source version, rank, faction, or per-model special override. There is no explicit claim that its nine rows exhaust Star Conflict. Catalog names and apparent familiarity cannot establish external identity.

### Internally agreed facts

1. `ontology.md:20–23`, `ontology/domain.md:212–218`, and `ontology/model.rs:29–35` agree on the same three groups of three roles.
2. `ontology.md:24–33,44`, `ontology/domain.md:145–153,196,220–232,254`, and `ontology/model.rs:37–50` agree that the special is inherited from role rather than selected in the loadout.
3. All nine current special kinds have exactly one current JSON row, and all nine local models have recognized roles. An executed Python check confirmed these data facts; it did not compile Rust or verify game fidelity.
4. Narrative, canonical constraint C14, and the cloak row all specify cancellation on damage: `ontology.md:28`; `ontology/domain.md:251`; `ontology/instances/special_modules.json:5`.
5. Narrative, C16, and the command-shield row agree about consuming energy instead of shield HP: `ontology.md:30`; `ontology/domain.md:253`; `ontology/instances/special_modules.json:7`. They do not fully specify activation lifecycle.

## 4. Special identity and behavior: exact claims and verification seams

The middle column contains exact local descriptions. The final column contains **unanswered research questions**, not sourced competing mechanics or proposed changes. All task-requested alternative names remain unconfirmed aliases.

| Role / local ID / anchor | Exact local activation and effect | Targeted external check / local ambiguity |
|---|---|---|
| Engineer / `drones` / `ontology/instances/special_modules.json:2` | `press`; “Heals shield of allies in range on press.” | Verify the requested engineer-drone identity: persistent entities or a pulse, autonomous effects, self versus ally targets, hull versus shield repair, offensive behavior, drone count, sacrifice/replenishment, and ship/faction variants. The local text establishes only ally-shield healing on press. `ontology.md:25` says “drones (heals Shield on key pressed)” without naming the target; JSON adds ally/range specificity. |
| Long-range / `sniper-weapon` / `.../special_modules.json:3` | `toggle`; “Swaps to a long-range weapon; fires like a regular weapon while on.” | Retrieve original disintegrator and guided-torpedo pages, if those task-supplied labels resolve. Determine whether they share a family or have distinct activation/aim/control behavior, and which named ships use which. No guided-projectile special identity or model-specific branch exists locally. This is not proof that any local behavior is externally false. |
| Guard / `phasic-shield` / `.../special_modules.json:4` | `toggle`; “Cycles improved resistance against thermic / electromagnetic / kinetic.” | Verify name correspondence to requested “phase shield.” Clarify whether `toggle` means binary enable/disable or cycling three resistance modes; the local enum supplies no definition that settles this. Verify baseline/changed resistance, mode order, activation timing, and ship variants before discussing numerical fidelity. |
| Tackler / `cloak` / `.../special_modules.json:5` | `press`; “Ship becomes invisible; drops when the ship takes damage.” | Verify cloak identity and all cancellation conditions: damage, firing, module use, time, distance, detection, or others. No additional condition is established by this audit. A condition missing locally is not proof of disagreement until sourced. C14 makes damage cancellation explicit, not an assertion that it is the only possible rule. |
| Gunship / `overclock` / `.../special_modules.json:6` | `press`; “Boosts weapons and motors for a short time.” | Verify equivalence, if any, to requested “overdrive”; determine exactly which weapon/flight properties, duration, resource costs, and variants. “Weapons and motors” is broad descriptive text, not a numerical effect model. |
| Command / `command-shield` / `.../special_modules.json:7` | `toggle`; “Additional shield that drains energy instead of shield HP.” | Verify requested “diffusion shield” identity, absorption/resource mechanism, duration/deactivation, target scope, and variants. `ontology.md:30` describes key activation but does not establish a sustained toggle. Canonical `toggle` may be intentional added specificity, not automatically an inconsistency. |
| Covert ops / `plasma-web` / `.../special_modules.json:8` | `press`; “Inflicts damage over time on the target.” | Verify exact damage type, duration, target/range and loss-of-target rules, modifiers, and special variants. Shared spelling with the requested term is not evidence of exact behavior parity. |
| Recon / `hyper-propulsion` / `.../special_modules.json:9` | `press`; “Warps the ship far away.” | Verify relation to requested “microwarp”: travel direction, charge-up, interruption, collision/obstacle behavior, range, and variants. “Far away” has no measurable facet locally. |
| ECM / `em-surge` / `.../special_modules.json:10` | `press`; “Freezes every ship around for a short while.” | Verify relation to requested “metastable field”: caster state, invulnerability, effect timing, disabled systems versus movement, target allegiance, radius, duration, and variants. “Every ship” currently does not distinguish self/friendly/enemy; do not silently rewrite it to “enemies.” |

Special descriptions are not executable combat rules in this repository. `client/src/main.rs:54–64` builds flight input only; `sim/src/lib.rs` implements movement and collision. Special identity affects validation and conceptual data, not currently implemented special activation. Active-module flow parameters at `ontology/model.rs:136–150` belong to another class; they must not be assumed to supply costs/cooldowns for specials.

## 5. Three representative local examples; external examples not fulfilled

These are **real repository instances**, not certified real-game examples. Three externally sourced Star Conflict examples remain an unmet research requirement.

1. **Anaconda — local ordinary role example.** `ontology/instances/ship_models.json:2–5` selects `engineer`, yielding `frigate` and `drones`; the local special heals allies' shields on press. This proves the local derivation path. To validate a real counterpart, obtain the exact named-ship page, class/role/faction field, and special listing with version context. No such source was retrieved.
2. **Styx — local long-range variant stress test.** `ontology/instances/ship_models.json:6–9` selects `long-range`, so the model necessarily inherits `sniper-weapon`, regardless of any future faction or ship lore. The shape has no room for a different *exact* special per long-range model without changing the commitment or treating `sniper-weapon` as a family. Whether Styx's actual game role/special agrees is unknown here.
3. **Kite — local target/behavior stress test.** `ontology/instances/ship_models.json:34–37` selects `ecm`, hence `interceptor` and `em-surge`. The effect says “every ship” and does not type timing or caster state. Verify an external named-ship page and its actual special separately; resemblance to the task's metastable-field term does not certify this mapping.

The nine-model table remains the full local catalog for this lane; no potentially inconvenient names were omitted. A later researcher should use at least one verified ordinary ship, one verified long-range alternative, and one verified later/Ellydium/special ship for external instance-fit tests, only after retrieving sources.

## 6. Candidate reconciliation ledger

**Confidence convention:** “high local” means repository evidence is direct; “external unknown” means no source comparison was possible. **Impact is conditional on the desired fidelity scope**, not an implementation demand. “Sourced claim: unavailable” explicitly distinguishes these from verified external drift items.

| ID | Exact local claim and anchors | Sourced claim / classification / confidence | Impact and reconciliation question/options |
|---|---|---|---|
| SH-01 | `ship-size` enumerates only frigate/fighter/interceptor; nine-role enum determines size: `ontology/domain.md:59–64,212–218`; `ontology/model.rs:12,16,30–35`. | Sourced claim unavailable. **Deliberate simplification + external coverage uncertainty**; high local, external unknown. No verified destroyer omission relative to a completeness promise. | Potentially high if aiming for exhaustive game taxonomy; none if the selected subset is intentional. Is the scope “this three-size game” or a dated subset of Star Conflict? Retain explicitly bounded taxonomy, or later assess source-backed extension; do not add a class merely from this task's wording. |
| SH-02 | Special is “Built into the role; one per role”; `role-has-special` is `1 ↔ 1`; C17 forbids it in loadout: `ontology/domain.md:145–153,196,254`; `ontology/model.rs:20–22,37–50,186–192`. | Sourced claim unavailable. **Deliberate simplification / candidate variant coverage gap**; high local, external unknown. | High if exact ship-specific alternatives matter: every existing and new ship with a role gets the same kind, with no selection field. Does the ID denote an exact module, a family, or a default? Options after evidence: retain exact bijection; document family-level identity; or explicitly model permitted ship-level variants. No option authorized here. |
| SH-03 | All nine named model-role assignments in §3 are local records without external IDs or provenance. | Sourced claim unavailable. **Identity/provenance uncertainty**, not nine asserted mismatches; high local, external unknown. | Potentially high: names can suggest fidelity the data does not establish. Are these intentionally borrowed labels or intended copies of actual ships? Options: record intentional independent identity; or source and review each named mapping. Do not rename/reclassify based on memory. |
| SH-04 | No faction field, faction constraints, or faction-specific special selection; faction/tier is deferred: `ontology/domain.md:22–23`; `ontology/model.rs:110–119`. | Sourced claim unavailable. **Explicitly deferred coverage**, not a defect; high local, external unknown. | Deferred under current scope. Do faction differences belong solely in per-model base stats, or must provenance/restrictions/variant selection eventually be queryable? No claimed Empire/Federation/Jericho/Ellydium characteristic is verified in this report. |
| SH-05 | Engineer `drones` only describes ally-shield healing on press; long-range `sniper-weapon` describes toggled weapon replacement: `ontology/instances/special_modules.json:2–3`; narrative `ontology.md:25–26`. | Sourced claim unavailable. **Behavioral coverage uncertainty**; high local, external unknown. | Medium/high for future simulation if drone entities or guided special controls are required. Does the desired scope intentionally collapse these identities to effects? Retrieve both model and module evidence before distinguishing omitted details from contradiction. |
| SH-06 | Guard `activation: toggle` has a three-mode cycling effect: `ontology/instances/special_modules.json:4`; `Activation` has only Press/Toggle: `ontology/model.rs:65–67`; command special is also `toggle`: JSON line 7. | External claim unavailable. **Internal semantic uncertainty**, not a proven logical contradiction: `toggle` has no formal special-state definition. High confidence in ambiguity. | Medium: UI/input and state-machine implementations could disagree. Does `toggle` mean any repeated-key state change or strictly on/off? Options: document its intended meaning, or later distinguish cycling from toggling if needed. Do not assume active-module `ongoing` semantics apply. |
| SH-07 | ECM effect says “Freezes every ship around for a short while”: `ontology/instances/special_modules.json:10`; `ontology.md:33`. | Sourced claim unavailable. **Target/timing uncertainty**, not confirmed wrong allegiance or effect; high local, external unknown. | High if implemented literally: self/friendly inclusion and what “freeze” disables affect gameplay. Is “every ship” intentional? Obtain exact external targets/timing before reconciliation; define local scope explicitly regardless of fidelity decision. |
| SH-08 | Cloak cancels on damage (C14); other specials use unmeasured “short time,” “far away,” and free-text effects: `ontology/domain.md:149–153,251`; `ontology/instances/special_modules.json:5–9`. | Sourced claim unavailable. **Deliberately minimal descriptive model + executable coverage gap**; high local, external unknown. | Low for catalog documentation, high before simulation. Are effects only UI prose at present? Options: keep prose and record the ceiling; later add only parameters/state required by accepted mechanics. Absence of additional cloak cancellation text is not proof that the ontology asserts they cannot exist. |
| SH-09 | Exactly one special definition is implied by “one per role” and `1 ↔ 1`, but `Catalog::validate` checks only `.any(...)` for each kind: `ontology/domain.md:147,196`; `ontology/model.rs:277–282,299–304`. Existing test checks uniqueness of mapped enum kinds, not JSON definitions: `ontology/model.rs:473–478`. | **Internal logical issue: validator permits duplicate special-definition IDs**; high from direct code plus an in-memory predicate counterexample. Current nine rows are unique. No external source needed. | Medium content-integrity risk: two rows can disagree on activation/effect for the same derived kind. Is one definition per kind a required facet? Options: clarify that facet and later reject duplicates, or explicitly define multiple versioned definitions and a selection rule. Do not silently treat current rows as corrupt. C1 is scoped to catalog IDs, and specials are explicitly non-catalog; rely on special identity/cardinality, not an overbroad reading of C1. |
| SH-10 | Size is derived rather than independently stored; it controls weapon compatibility and collision radius: `ontology/model.rs:123,314–317,374–378`; `sim/src/lib.rs:61–68`; `client/src/main.rs:20,90`. | **Internal agreement / impact trace**, not a drift allegation; high local. | A later accepted role correction can change size, stock-weapon validity, collision sizing, and displayed type. Review the whole derivation path; do not “fix” an instance role without checking derived consequences. Existing radii are local simplifications, not measurements of Star Conflict hulls. |

### Historical-change status

No retrieved announcement or dated patch supports when any destroyer, special, alien, or Ellydium class/variant appeared or changed. Do not classify a candidate difference as “outdated” rather than “independent design” without a historical source. The repository's local reboot context in `README.md:6–12` does not date Star Conflict mechanics. No conflicting external sources were available; no apparent conflict has been silently resolved.

## 7. Resume-only source discovery plan — UNRETRIEVED

These are **candidate routes, not citations or discovered pages**. Actual page existence, titles, article dates, headings, and content are unknown. No network request was made to these candidates after the supervisor stopped retries. Use official indexes/API navigation if a guessed title does not exist; do not repeatedly probe inaccessible hosts.

### Highest-value exact named-model candidates

| Candidate title/search identity | Exact candidate URL | Intended check |
|---|---|---|
| Anaconda | https://wiki.star-conflict.com/index.php?title=Anaconda | Exact model identity, class/role/faction, special. |
| Styx | https://wiki.star-conflict.com/index.php?title=Styx | Same; particularly local long-range assignment. |
| Mammoth | https://wiki.star-conflict.com/index.php?title=Mammoth | Same; local guard assignment. |
| Hydra | https://wiki.star-conflict.com/index.php?title=Hydra | Same; local fighter/tackler assignment. |
| Phoenix | https://wiki.star-conflict.com/index.php?title=Phoenix | Same; local fighter/gunship assignment. |
| Lynx | https://wiki.star-conflict.com/index.php?title=Lynx | Same; local command assignment. |
| Shrike | https://wiki.star-conflict.com/index.php?title=Shrike | Same; local covert-ops assignment. |
| Wolfhound | https://wiki.star-conflict.com/index.php?title=Wolfhound | Same; local interceptor/recon assignment. |
| Kite | https://wiki.star-conflict.com/index.php?title=Kite | Same; local ECM assignment. |

### Nonredundant taxonomy and variant routes

- Wiki navigation seed: https://wiki.star-conflict.com/index.php?title=Main_Page
- Candidate wiki search for taxonomy: https://wiki.star-conflict.com/index.php?title=Special%3ASearch&search=ships
- Candidate wiki search for special identities: https://wiki.star-conflict.com/index.php?title=Special%3ASearch&search=special+modules
- Candidate wiki search for destroyers: https://wiki.star-conflict.com/index.php?title=Special%3ASearch&search=destroyer
- Candidate wiki search for later/faction-specific ships: https://wiki.star-conflict.com/index.php?title=Special%3ASearch&search=Ellydium
- Official dated-original discovery: https://star-conflict.com/en/

From successfully retrieved indexes, choose original pages that resolve the role/class table, faction tendencies versus per-model exceptions, and two different special identities within the same role if such evidence exists. The long-range disintegrator/guided-torpedo pair requested by the task is a high-value *hypothesis to test*, not a verified example. Then use one gap-closing pass for remaining special behaviors and dated changes. A later/alien ship should be a representative instance only after its actual model, role/class and special have been sourced. Do not infer a fifth faction or new class merely from a marketing label.

## 8. Stanford seven-step audit and Gruber evaluation

### Stanford seven steps

1. **Domain and scope:** eight competency questions in §1. Current authority is an intentionally Star Conflict-like game, not a declared complete replica. Economy and deferred progression/faction systems were not audited as mandatory features. Product intent remains a report question, not a request to the user.
2. **Reuse:** reuse the existing canonical definitions, Rust enums/structs, validators, and JSON instances for comparison. No ontology package or new abstraction is needed for this report. Wiki/site material would be external evidence, not canonical authority; none was available.
3. **Terms:** established local nouns (`ship-model`, `player-ship`, role, size, special, activation), attributes (name, base stats, slots, role), and relations (has-role, derived size, derived special, has-model). Keep task-supplied external terms provisional; do not collapse “class,” “size,” “role,” “faction,” and “special family” into one concept without definitions.
4. **Classes/hierarchy:** ship models are catalog items; player ships reference models; roles and sizes are enums, not inheritance subclasses; special modules are separate, non-equipable definitions. The current role→size and role→special tables are internally exhaustive. A later expansion must first decide whether a term is a size, role, faction, model family, or independent attribute; no speculative class was added.
5. **Slots/properties:** role is a model property; size and special are derived; ownership and model references belong to player ships; activation/effect belong to special definitions. No faction, rank, per-model variant, or source/version property exists. Describing this absence does not authorize adding them.
6. **Facets:** local many→one role/size relationships and one↔one special identity are explicit. Rust enums constrain values and serde loads JSON; there is no SQL layer. Current rows fit these enum mappings. Duplicate-special-row uniqueness is not checked by the validator. The effects remain prose, so no numerical duration/radius/target constraint can be verified. No Rust execution was claimed.
7. **Instances:** all nine models and nine special definitions were checked locally; three representative derivations appear in §5. Real-game instance fit and the requested external examples remain blocked, not falsely marked complete.

### Gruber criteria

| Criterion | Finding within this lane |
|---|---|
| **Clarity** | Local role/size tables, enum IDs, and cardinalities are clear. “Toggle” versus three-mode cycling, “freeze,” “every ship,” “weapons and motors,” and “far away” are underspecified. External naming equivalence and time/version scope are unknown. |
| **Coherence** | Current model-role-size-special data agree with local definitions. The special collection permits duplicate definition rows even though role→kind is bijective. There is no externally proved mismatch to call incoherent. |
| **Extendibility** | More models with existing roles fit without code edits; additional size/role/special kinds require enum/match changes. Exact per-model special variants cannot be expressed by a new JSON row alone. This may be an acceptable scoped commitment. |
| **Minimal encoding bias** | Human-readable IDs and relations aid portability. Input labels `press`/`toggle` currently risk conflating UI actions with effect state transitions. No alternate encoding is prescribed. |
| **Minimal ontological commitment** | Deferred faction/tier and descriptive effects keep scope bounded. Exact role→special bijection is stronger than “role suggests a special family”; whether that strength is warranted depends on product intent and unavailable evidence. Do not expand ontology simply to match every possible game feature. |

## 9. Local validation evidence and reproducibility

Executed a read-only Python JSON check: nine models, nine unique special IDs, one model per each of the nine roles, and every expected role-derived special present. The mapping used was transcribed from the fully read Rust match; this check is data cross-checking, not a Rust parser/compiler or complete `Catalog::validate` execution.

It also constructed a duplicate `drones` row **in memory only** with conflicting effect text. The equivalent of the validator's presence predicate still passed while the `drones` row count became two. Static code inspection confirms no special-definition uniqueness call in `ontology/model.rs:277–282`. This is sufficient for the narrow SH-09 validation-gap finding, not a claim of a failing current catalog or an executed Rust reproducer.

Runnable minimal audit check (no writes):

```sh
python3 - <<'PY'
import json
from collections import Counter
from pathlib import Path
p = Path('ontology/instances')
ships = json.loads((p / 'ship_models.json').read_text())
specials = json.loads((p / 'special_modules.json').read_text())
role_special = {
    'engineer': 'drones', 'long-range': 'sniper-weapon',
    'guard': 'phasic-shield', 'tackler': 'cloak',
    'gunship': 'overclock', 'command': 'command-shield',
    'covert-ops': 'plasma-web', 'recon': 'hyper-propulsion',
    'ecm': 'em-surge',
}
assert len(ships) == len(specials) == 9
assert Counter(s['role'] for s in ships) == Counter(role_special.keys())
assert len({s['id'] for s in specials}) == 9
assert all(any(s['id'] == k for s in specials) for k in role_special.values())
duplicated = specials + [dict(specials[0], effect='AUDIT-ONLY CONFLICT')]
assert all(any(s['id'] == k for s in duplicated) for k in role_special.values())
assert sum(s['id'] == 'drones' for s in duplicated) == 2
print('PASS: current local coverage; duplicate-row presence-only counterexample')
PY
```

Actual executed check additionally printed the nine model→role→size→special chains and checked that ship rows have only the known fields. Output ended:

```text
PASS: current 9 models/9 specials are unique and cover all local roles.
PASS: in-memory duplicate-ID counterexample still satisfies the Rust-loop equivalent presence predicate; no files mutated.
```

The embedded minimal check above was then extracted from this report and executed successfully; the fenced acceptance JSON also parsed successfully. No tests were added or updated. `cargo` was unavailable on PATH; no build/development environment was installed or realized. Existing `role_tables_cover_every_role` at `ontology/model.rs:473–478` was inspected, not run. Git status and staged-file queries were empty, branch remained `master`, and HEAD remained the baseline.

## 10. Unanswered reconciliation questions

1. Is the intended fidelity a dated Star Conflict snapshot, a selected familiar subset, or an independently balanced game with borrowed labels?
2. Are the nine catalog names intended to identify their exact game counterparts? If not, how should later documentation prevent readers from mistaking them for sourced assignments?
3. Does each special ID denote an exact module, a family, or a default? This determines whether a source-backed variant would be a contradiction or merely more detailed coverage.
4. Are destroyer and later/special/alien/Ellydium examples desired now, deferred, or explicitly excluded? Their possible omission alone establishes no defect.
5. If faction information is later included, is it descriptive provenance, a balance tendency, an equipment constraint, or a special-variant selector? Avoid substituting a tendency for a universal rule.
6. What exactly do local `toggle`, `freeze`, “every ship,” and “far away” mean before any simulation work?
7. Should special-definition IDs be unique independently of the non-applicable catalog-item C1 label, and should that be explicitly validated?
8. Which source date/patch and conflicting-source policy should govern later reconciliation? No dated external evidence currently supports a live-game claim.

**Review gate:** required. The report is independently reviewable for local facts and scope discipline; public-game research acceptance must remain open until connectivity permits retrieval of original evidence. Do not turn the unverified candidate ledger into an implementation ticket asserting game drift.

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "satisfied",
      "evidence": "Produced only the authorized, supervisor-directed PARTIAL research report; read canonical files and applied the ontology method without changing source, ontology semantics, branches, or scope."
    },
    {
      "id": "criterion-2",
      "status": "not-satisfied",
      "evidence": "Local anchors, executed JSON checks, and network failures are documented, but no external page was retrieved; actual Star Conflict claims, freshness, and three real-game examples remain unverified."
    }
  ],
  "changedFiles": [
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/research/ships.md"
  ],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {
      "command": "Check AGENTS.md/docs/HANDOFF.md; git status --short; git rev-parse HEAD; git diff --cached --name-only",
      "result": "passed",
      "summary": "Both instruction files absent; clean working tree and index; expected baseline."
    },
    {
      "command": "curl -L --connect-timeout 10 --max-time 35 to wiki Main_Page, official home, and Google discovery URL; markitdown conversion attempts",
      "result": "failed",
      "summary": "All curl connections timed out (28); no HTML existed; conversions failed with FileNotFoundError. Exact URLs and failure details are in section 2."
    },
    {
      "command": "Python socket.getaddrinfo and proxy-variable presence checks; curl -4 --connect-timeout 8 --max-time 20 to Steam and wiki",
      "result": "failed",
      "summary": "DNS worked and no proxy variables were set; both IPv4 fetches timed out with HTTP 000 and no remote connection."
    },
    {
      "command": "Read-only Python JSON assertions for nine role/model/special records and in-memory duplicate-special presence predicate",
      "result": "passed",
      "summary": "Current local coverage and uniqueness confirmed; duplicate definition row satisfies presence-only predicate. No repository data mutated."
    },
    {
      "command": "Python parse of report acceptance JSON and execution of the report's embedded read-only shell check",
      "result": "passed",
      "summary": "Acceptance JSON valid; embedded current-coverage and duplicate-presence assertions pass."
    },
    {
      "command": "cargo test",
      "result": "not-run",
      "summary": "Cargo unavailable on PATH; no package installation or environment realization authorized/performed."
    },
    {
      "command": "git status --short; git diff --cached --name-only; git branch --show-current; git rev-parse HEAD",
      "result": "passed",
      "summary": "No repository changes or staged paths; master at baseline."
    }
  ],
  "validationOutput": [
    "Nine local models cover all nine local roles exactly once; nine current special IDs are unique.",
    "Zero externally verified gameplay claims, contradictions, agreements, or real-game examples.",
    "Supervisor confirmed infrastructure-wide network failure and directed a partial report without further retries."
  ],
  "residualRisks": [
    "External game taxonomy, all named-ship roles, special variants/behavior, faction traits, historical changes, and currentness remain unverified.",
    "Candidate URLs may not resolve to existing pages and are discovery suggestions only.",
    "Duplicate-special validation finding is based on static Rust inspection plus equivalent Python predicate, not executed Rust."
  ],
  "noStagedFiles": true,
  "diffSummary": "Added only this managed research artifact outside the repository; no source/ontology diff.",
  "reviewFindings": [
    "blocker: outbound connectivity prevented required external source evidence; public-research completion is not accepted.",
    "internal finding: ontology/model.rs:299-304 checks special presence but not duplicate definition IDs; current catalog remains unique."
  ],
  "manualNotes": "Required reviewer gate remains open. Treat all external comparison cells as unknown, not remembered or inferred Star Conflict facts."
}
```
