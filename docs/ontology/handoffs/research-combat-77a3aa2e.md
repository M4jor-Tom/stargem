# Handoff: research-combat — 77a3aa2e

Checkpoint: 2026-09-16, user-requested pause. Parent-curated from runtime records and actual files; not a new completed child report.

- **State:** COMPLETED / SUPERSEDED: network-blocked attempt
- **Role:** `delegate`
- **Child run:** `77a3aa2e-8e69-4ebf-929b-1d5744ab3ea1`
- **Workflow / key:** `08aa52d7-d2a2-4091-b856-28049dd1b39f` / `research-combat`
- **Mission:** `28a33a06-b296-4a78-8ae0-079881816489`
- **Repository:** `/home/theta/repos/stargem.nix`, branch `master`
- **Source baseline:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`
- **Session (machine-local, optional for recovery):** `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/2026-09-16T18-47-01-030Z_01a0ab8b-30a5-761c-90d3-cc64125796e1/197ae10a-93e7-407f-b538-80fb1ec1756e/run-0/session.jsonl`
- **Runtime status (machine-local):** `/tmp/pi-subagents-uid-1000/async-subagent-runs/77a3aa2e-8e69-4ebf-929b-1d5744ab3ea1/status.json`

## Authority and boundaries

Read root `AGENTS.md` and `docs/HANDOFF.md` before work. `ontology/` remains canonical and unchanged. This is documentation/research, not authorization to change gameplay. No commit/push/merge/install is authorized. One writer per checkout. Preserve untracked audit files: they are the current work, not disposable clutter. Other agents' full research is intentionally file-backed to protect parent context.

## Resumption (for Agent)

No remaining work for this attempt. Use [research-combat-2fcf909b.md](research-combat-2fcf909b.md) instead. The user fixed the firewall; do not propagate this attempt's access-blocked conclusion as current.

## Native continuation or fallback

Recheck live availability before any resume. Completed work normally requires no resume.

Not rechecked in the latest retained-child list; no resume is needed. Use the preserved report below. If genuinely necessary, inspect status/children.list first; otherwise start a same-role focused continuation from this handoff.

## Verification and acceptance

The original report below states exactly what this child checked and what remains uncertain. Completion of a research child does not mean that document synthesis, independent evidence review, ontology review or user reconciliation has completed. Follow [the main checkpoint](README.md) for dependency order and pending reviews.

## Preserved original output

Original managed artifact: `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/research/combat.md`

SHA-256 of original artifact: `9ce8d2c4ae2d7b720ef33b3d9d42b881d129ff46e8da31d156491e7fe7cfc51d`

The complete output is embedded below so recovery does not depend on session/temp-file retention. Original relative code paths refer to the repository root unless the report says otherwise. Historical access failures stay historical.

---

# Combat mechanics research audit — PARTIAL / internet verification blocked

- **Audit / attempted retrieval date:** 2026-09-16.
- **Repository:** `/home/theta/repos/stargem.nix`; branch `master`; HEAD verified as `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`.
- **Exclusive scope:** damage, hull/armor, resistances, shield overflow/regeneration, energy, flight, weapons, heat, projectiles, missiles and launchers. Not a module-catalog taxonomy audit.
- **Authority:** `ontology/` is canonical. `ontology.md` is the local source narrative, not independently established Star Conflict documentation. All file anchors below are repository-relative.
- **Result:** local contracts and combat-related seams inspected directly; **zero external source pages retrieved successfully; zero verified contradictions with actual Star Conflict; zero independently verified live-game agreements**. Do not promote the candidates below to live-game drift findings.
- **Changes:** this external report only. No source/ontology changes, repository artifacts, installations, commits, branches, staging, pushes, or nested agents. Root `AGENTS.md` and `docs/HANDOFF.md` checked first and absent.

## 1. Executive findings

1. Local damage is a **global, constant two-layer multiplier model**: EM `(1.5, 0.7)`, kinetic `(0.7, 1.5)`, thermic `(1, 1)`. Whether Star Conflict uses target-dependent resistance values, and any documented conversion, could not be researched. The narrative's “usually weak” becomes a universal canonical inequality; that is a verified local strengthening, not proof of an external error.
2. A concrete **internal coherence issue** exists independently of internet access: C9 says thermic multipliers are both approximately one, but `ontology/model.rs:293` checks only their difference. Positive `100/100` satisfies that predicate. The shipped instance is correct at `1/1`.
3. Shield overflow is under-specified locally. Shield regeneration, damage-specific resistance state, weapon hit-delivery/range/ammunition, and missile lock/guidance/remaining-ammo state are not given typed mechanics. These are **coverage gaps or uncertainties**, not demonstrated disagreements with another game.
4. The arcade-flight choice is deliberate. Current flight has a finite velocity-response rate, one shared angular rate, and no strafe or afterburner input. Calling its acceleration “instant” needs qualification; this does not justify replacing the design with Newtonian flight.
5. Weapon heat and missile budgets have meaningful local commitments that later research must not silently overwrite: idle-only cooling, full cooling plus extra lockout after overheat, one launcher, one missile model, ammunition per life, and one interval between launches.
6. A second internal seam affects combat stats: C5 attributes positivity checking to `effective_stats`, which only sums. Checking is in `validate_loadout`, and the present client does not call it. No invalid stock ship or current combat crash was demonstrated.

**Highest-value next action when authorized internet access is restored:** collect original, dated Star Conflict evidence for resistance/overflow, firing and cooling, and missile replenishment versus launch cadence. No formula or mechanic from memory has been substituted for that evidence.

## 2. Retrieval record and evidence limitations

### Initial navigation angles

Three angles were started in parallel: (1) official-hosted wiki main/index for mechanics navigation, (2) official game site for primary explanations/patches, (3) public search for damage/resistance/shield/missile/overheat discovery. Steam was attempted as an independent seed/access diagnostic. A single subsequent IPv4 wiki probe also failed. This was bounded discovery, not a crawl.

| Attempted exact URL | Intended publisher/type, not an established fetched article | Result on 2026-09-16 |
|---|---|---|
| `https://wiki.star-conflict.com/index.php?title=Main_Page` | Official-hosted, community-maintained wiki navigation | `curl` exit 28; HTTP `000`; TCP connection timeout after approximately 10 seconds. |
| `https://star-conflict.com/en/` | Official game website / primary-source navigation | Exit 28; HTTP `000`; TCP connection timeout after approximately 10 seconds. |
| `https://www.google.com/search?q=site%3Awiki.star-conflict.com+damage+resistance+shield+missiles+overheating` | Search discovery only; never substantive evidence | Exit 28; HTTP `000`; TCP connection timeout after approximately 10 seconds. No snippets obtained. |
| `https://wiki.star-conflict.com/index.php?title=Main_Page` | Same resource; one IPv4 HEAD diagnostic, not a second source | `curl -4 -I -v`; exit 28 after approximately 8 seconds, before an HTTP response. |
| `https://store.steampowered.com/app/212070/Star_Conflict/` | Valve-hosted game store listing; intended identity/access cross-check | Exit 28; TCP connection timeout after approximately 8 seconds. No page received. |

Initial fetches used `curl -L --connect-timeout 10 --max-time 35 -A Mozilla/5.0 -sS`, with `-w '%{http_code}\t%{url_effective}'`. The diagnostic used connect timeout 8 / total timeout 15; Steam used connect timeout 8 / total timeout 20. DNS resolved the official site, wiki and Google. No proxy environment variables were present. Thus this was not evidence of nonexistent articles, empty pages, or an HTTP denial by the game publisher.

`markitdown` was installed and found on PATH; the retrieval procedure was configured to convert successful HTML before reading it. **No HTML arrived, so no conversion or table parsing was possible or performed.** No standard-library HTML-parser fallback, AI CLI, or alternate execution mode was used.

The infrastructure blocker was reported through the authorized supervisor channel. The supervisor confirmed broad outbound failures in other lanes, no known approved proxy, and instructed this lane to stop retries and deliver a partial report. Consequently neither the intended broad 10–20-page pass nor a targeted external gap-closing pass could be completed.

### External source register

**Useful fetched external sources: none.** For every attempted resource above, recovered article title, article/update/patch date, relevant heading, excerpt, and supported gameplay claim are **unavailable**. URLs identify attempted endpoints only. There is no basis to label any wiki mechanic “current,” reconcile conflicting external sources, or distinguish historical game changes from present rules.

The initial machine-readable failure log is managed scratch at `/tmp/stargem-combat-audit-08aa52d7/fetch-log.tsv`; the table above additionally preserves the later IPv4 and Steam diagnostics. Scratch is not a durable evidence dependency for this report: the relevant failures are recorded here.

### Targeted resumption queue — all UNRETRIEVED, not evidence

Prefer the known main index / official home URLs above, then follow actual links. These exact search endpoints are candidate navigation requests, **not verified article locations**:

1. `https://wiki.star-conflict.com/index.php?title=Special%3ASearch&search=damage+resistance` — locate definitions and any explicitly published formula, positive/negative resistance handling, per-layer properties, and worked examples.
2. `https://wiki.star-conflict.com/index.php?title=Special%3ASearch&search=shield+regeneration` — locate shield recovery and overflow rules, delays, interruption, and energy interaction.
3. `https://wiki.star-conflict.com/index.php?title=Special%3ASearch&search=afterburner` — movement controls, strafe, axis-specific rotation and acceleration; distinguish controls from equipable modifications.
4. `https://wiki.star-conflict.com/index.php?title=Special%3ASearch&search=weapon+overheating` — firing/cooling, ammunition, projectile/beam behavior, range and energy use.
5. `https://wiki.star-conflict.com/index.php?title=Special%3ASearch&search=missile+reload` — lock prerequisites, guidance, interruption, launch intervals, magazines/reserves/replenishment, and launcher limits.

A later researcher should retain, per useful original page: exact title/URL; official announcement versus official-hosted community wiki versus independent community; retrieval date; visible article/update/patch date or unknown; relevant heading; short faithful quote/paraphrase; supported claim; scope/version caveat. Search results alone are discovery, not proof. Only compare a live formula if a retrieved source actually publishes it. No resistance, overflow, projectile, or missile formula is asserted in this report.

## 3. Local evidence examined directly

The supplied inventory was read as orientation, then relevant files were independently read rather than treating inventory conclusions as authority:

- `ontology/domain.md` in full; scope, Q6/Q7/Q10, combat properties, relations, C5/C8/C9/C13/C15/C16.
- `ontology.md` in full; particularly lines 13–19 and 24–55.
- `ontology/model.rs` in full; enums, combat structs, loading, validators, effective stats and existing tests.
- `ontology/instances/{damage_types,weapons,missiles,ship_models,passive_modules,active_modules,special_modules}.json` in full.
- `sim/src/lib.rs`, `client/src/main.rs`, `README.md` in full.
- The required ontology skill and `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/69d652a5-b7d3-47be-83bd-4c3f7677ee17/audit/inventory.md` in full.

All local claims refer to the specified Git snapshot, inspected 2026-09-16; they are not evidence of when a live Star Conflict mechanic existed. A search of the tracked Rust files found only `ontology/model.rs`, `sim/src/lib.rs`, and `client/src/main.rs`. Present runtime behavior is flight/collision; damage, shield, weapon and missile rules remain declarative. The client displays base/effective shield and armor values rather than simulating their depletion (`client/src/main.rs:90–91`).

## 4. Stanford seven-step audit application

### Step 1 — domain/scope and competency questions

Purpose: establish what this Star Conflict-inspired ontology actually commits to in combat, then compare to retrieved game evidence if available. Consumers are the prospective rules server, client and content authors (`ontology/domain.md:10–17`). No gameplay implementation or fidelity policy decision is authorized here.

| Competency question | Local answer | External status |
|---|---|---|
| What are the damage kinds, and what determines loss on each layer? | Three enum kinds; two global multipliers per kind; C9. | Unverified. |
| Is `armor_hp` a second durability pool, resistance value, or hull synonym? | Positive durability pool; destruction at zero. No alias/resistance definition. | Terminology unverified. |
| What happens to one hit that exhausts the shield? | C9 says a remainder hits armor; conversion/order unclear. | No documented Star Conflict overflow rule obtained. |
| What recovers shield and energy, and under what conditions? | Energy continuously recovers; shield-healing effects are prose; no shield-regeneration stat/rule. | Unverified. |
| Which movement axes and speed/acceleration/rotation properties exist? | Speed cap and one agility; sim forward/reverse plus yaw/pitch/roll; no strafe input. | Unverified; arcade choice explicit locally. |
| What is one weapon shot, how does it hit, and does firing consume energy/ammo? | Damage, fire rate, heat defined. Delivery, range, energy and ammunition unspecified. | Unverified. |
| When does firing overheat, and when can it resume? | Normalized heat; idle cooling; cool fully plus penalty. Boundary/event semantics incomplete. | Unverified. |
| What are missile launch prerequisites, guidance, stock and replenishment? | One launcher/model; speed/damage; ammunition per life; interval between launches. No lock/guidance semantics. | Unverified. |

### Step 2 — reuse

Existing Rust value objects, tagged activation flow, string ID relations and JSON instances already express the local design. Reused these for the audit; no schema/framework/package introduced. External vocabulary reuse cannot be assessed without sources. The skill's TypeScript/Zod/SQL examples are methodological analogies, not a requirement to introduce those technologies into this Rust/JSON project.

### Step 3 — terms

- **Represented entities/value objects:** damage type, ship model, stats, stat modifier, active module, weapon, missile model, player-ship/loadout; prose-only runtime ship-state.
- **Represented properties:** shield HP, armor HP, energy capacity, energy regeneration per second, speed cap in m/s, agility in deg/s, per-shot damage, shots/s, normalized heat/shot, idle cooling/s, extra lockout seconds, missile damage/speed/ammo/launch interval.
- **Represented relationships/actions:** damage multiplies each defensive layer; ship equips weapon/missile model; passives add stats; activation spends energy; fire adds heat; idle cools; armor zero destroys ship.
- **Research vocabulary only, not asserted game mechanics:** hull/armor synonymy; resistance points versus percent; layer overflow; passive regeneration versus repair; translational acceleration, strafing, angular rates; beam/projectile/hitscan; spread/range/falloff; ammunition magazine/reserve; missile target acquisition, lock, homing, guidance loss, launch cadence and replenishment.

### Steps 4–5 — classes, properties and relationships

The inspected hierarchy has no combat-specific is-a error established here. A ship **has** stats and equipment; a damage kind is not a ship subclass. The key commitments are property placement and missing distinctions, not a need for more inheritance.

| Local relation/property | Cardinality / attachment | Audit consequence |
|---|---|---|
| `DamageType` → layer multiplier | Exactly one shield and one armor multiplier per record (`model.rs:95–100`). | Target-independent baseline; no typed per-target resistance or modifier. |
| `Weapon` → damage kind | Exactly one (`model.rs:154–164`). | Mixed/switching damage would need an explicit design decision if later required; none is established by evidence here. |
| `MissileModel` → damage kind | Exactly one (`model.rs:167–175`). | Same single-kind commitment; no typed guidance discriminator. |
| Player ship → weapon / missile | Exactly one ID of each (`domain.md:177–178,202–203`); C8 calls missile entry one launcher. | One chosen model is not automatically one physical barrel or tube; those distinctions are not represented. |
| Weapon → permitted size | Exactly one size (`domain.md:127,205`; C7). | A universal local compatibility rule; actual-game compatibility unverified. |
| Ship model → base stats; equipped passives → deltas | One stats object; bounded list of additive modifiers (`model.rs:110–133,390–403`). | Capacity/base values and future changing runtime values should not be conflated. |
| Active module → activation flow | One of one-shot/ongoing (`domain.md:118–121`). | Cost/cooldown represented; shield/healing/speed effect amounts often prose-only. |
| Loadout missile selection → launcher/ammo behavior | Launcher is not a separate Rust class; ammo/reload properties live on `MissileModel`. | This is composition/encoding simplification, not proof a separate launcher entity is needed. |

### Step 6 — facets

Local facets include positive damage/rates/capacity, nonnegative shield HP and energy regeneration, heat per shot in `(0,1]`, positive u8 missile ammo, and zero-or-positive extra overheat penalty. Runtime constraints are delegated to a future server; the schema's numeric fields are not an implemented combat engine. The thermal-predicate and C5 attribution issues below concern alignment between documented facets and actual validation. No SQL/storage validator exists to inspect.

### Step 7 — representative instances

Three existing local loadouts are worked through in §7. These are **real repository instances**, not verified real-game examples. The requested three externally grounded Star Conflict examples remain blocked; no identity, role, weapon characteristics, or balance values are inferred merely because a local item has a familiar game-like name.

## 5. Local agreements and deliberate commitments

These are agreements **between local layers only**, not agreements with independently fetched Star Conflict sources:

- Three damage names and qualitative layer preference agree across narrative `ontology.md:16–19`, canonical C9, `DamageKind` (`model.rs:55`) and `damage_types.json:2–4`; canonical strictness exceeds the narrative's “usually.”
- Armor is the destruction-bearing pool in both narrative `ontology.md:13–15` and C13 (`domain.md:250`). It is not defined as a resistance number.
- Continuous energy recovery appears in `ontology.md:14`, `domain.md:81,253`, and `Stats.energy_regen` (`model.rs:78`). This is declarative agreement, not executable regeneration.
- Overheat/cooling and an extra overheat penalty agree qualitatively across `ontology.md:53–54`, `domain.md:131–133,252` and `Weapon` (`model.rs:161–163`). The exact normalized model is the canonical elaboration.
- One missile launcher is consistent in narrative `ontology.md:43,55`, canonical C8 and one `Loadout.missile_id`. No retrieved evidence establishes whether this matches Star Conflict.
- Arcade movement is a deliberate design choice in `ontology.md:52`. The sim explicitly marks fixed acceleration/drag as a simplification pending per-ship balancing (`sim/src/lib.rs:54–58`). No need to widen scope merely to reproduce another game's movement.

## 6. Reconciliation ledger

**Legend:** “verified contradiction” means an exact conflict supported on both sides; none with Star Conflict can be assigned. “Internal logical issue” concerns local contract/implementation alignment. “Coverage gap” means absent/incomplete representation, not automatically a defect. “Deliberate simplification” has local intent evidence. “Uncertainty” means intent, mechanics or evidence is unresolved. “Historical change” requires dated comparative evidence; no external historical-change conclusion is available.

Confidence below is confidence in the **local observation**, not confidence in an unobserved live-game claim. Every item's external comparison is explicitly pending.

### COM-01 — constant multipliers and narrative tightening

- **Exact local claims:** `ontology.md:17–19`: EM/kinetic are powerful against one layer and “usually weak” against the other; thermic is balanced. `domain.md:70–71,246` requires type-owned multipliers, EM `vs_shield > 1 > vs_armor`, kinetic inverse, thermic both approximately one. `damage_types.json:2–4`: `(1.5,0.7)`, `(0.7,1.5)`, `(1,1)`.
- **Sourced external claim:** none retrieved. Target-dependent resistances, resistance baselines, and any conversion formula are questions, not facts established here.
- **Classification / confidence / impact:** uncertainty of intended fidelity, with verified local strengthening; high local confidence; high impact on every future damage calculation and defensive loadout.
- **Reconciliation question/options:** Are these explicitly original Stargem balance constants, default profiles standing in for later resistance, or intended Star Conflict rules? Retain and label original balance; or, only after evidence/product approval, specify how target/layer resistance relates to damage type. Do not reverse-engineer resistance points from the existing multipliers.

### COM-02 — thermic validation does not enforce “approximately one”

- **Exact local claims:** C9 `domain.md:246`: “thermic: both ≈ 1.” `model.rs:289,293`: positive values plus `(d.vs_shield - d.vs_armor).abs() < 0.2`.
- **Counterexample:** positive `100/100` passes those scalar predicates; neither value is approximately one. The shipped `damage_types.json:4` is `1/1`, so present catalog data do not exhibit the bad case.
- **Sourced external claim:** not applicable; this is independent of game rules.
- **Classification / confidence / impact:** internal logical issue; high confidence by source inspection and independent scalar check; medium immediate impact, potentially high content-authoring impact.
- **Reconciliation question/options:** Does “balanced” mean equal layer multipliers or near-unit magnitude? Clarify the canonical intent and then align the validator in a separately authorized change. Do not assume changing the source definition is preferable to changing validation.

### COM-03 — shield-to-armor overflow has no precise conservation rule

- **Exact local claim:** C9 `domain.md:246`: “shield takes `raw × vs_shield` until 0, remainder hits armor at `raw × vs_armor`.” No code resolves the remainder; `DamageType` only stores two multipliers (`model.rs:95–100`).
- **Sourced external claim:** none; no Star Conflict overflow rule documented in accessible evidence.
- **Classification / confidence / impact:** uncertainty; high confidence that the wording lacks a precise remainder variable; high future implementation impact.
- **Illustration of ambiguity, NOT a live formula:** for a hypothetical 100-raw EM hit against 75 shield, current local multipliers yield a shield-scaled amount of 150. Converting the consumed shield back to raw units leaves 50 raw and would produce 35 armor damage; carrying 75 shield-scaled overflow directly into the armor multiplier would produce 52.5. These are alternative readings, not recommendations or claims about Star Conflict.
- **Reconciliation question/options:** Specify whether leftover damage is raw-equivalent or already scaled; define hit ordering, zero-shield and shield-exhaustion cases. Preserve uncertainty until an authoritative reference or explicit local design decision resolves it.

### COM-04 — hull/armor naming and resistance semantics

- **Exact local claims:** `domain.md:79,250` uses `armor_hp` and destroys at zero. `passive_modules.json:3` names a `reinforced-hull` module in the `armor` family and adds armor HP. `sim/src/lib.rs:61–68` uses “hull” for geometric radius. `special_modules.json:4` says phasic shield “Cycles improved resistance against thermic / electromagnetic / kinetic.” Neither `Stats` (`model.rs:74–81`) nor `DamageType` has per-ship/per-layer resistance state.
- **Sourced external claim:** none; neither official terminology nor resistance mechanics verified.
- **Classification / confidence / impact:** terminology uncertainty plus coverage gap for an already mentioned resistance-changing effect; high local confidence; medium/high impact on vocabulary and special-effect implementation.
- **Reconciliation question/options:** Is armor HP intentionally a hull-durability synonym? If so document the alias without renaming IDs automatically. Does “resistance” mean temporarily changing a multiplier, adding a separate property, or merely designer-facing prose? Geometry, durability and mitigation should not silently become the same concept.

### COM-05 — shield recovery is not defined as passive regeneration

- **Exact local claims:** stats list shield HP but only **energy** regeneration (`domain.md:78–81`; `model.rs:74–81`). `active_modules.json:2–7` describes instant shield boost and ongoing ally shield healing; `special_modules.json:2` describes drone shield healing. No shield-regeneration rate, delay, interruption or energy dependency is specified in C13–C16 or the numeric stats.
- **Sourced external claim:** none; actual Star Conflict shield recovery unverified.
- **Classification / confidence / impact:** coverage gap / scope uncertainty, not contradiction; high local confidence; high if passive recovery is intended in future combat.
- **Reconciliation question/options:** Is shield recovery deliberately repair-only, deferred, or omitted unintentionally? Keep it unspecified or explicitly repair-only unless a desired rule is established. Do not infer “no shields ever regenerate” merely from the absent field.

### COM-06 — energy consumption covers active flow, not every combat action

- **Exact local claims:** C16 `domain.md:253` bounds energy and permits active activation/continuation only while costs can be paid. `model.rs:139–140` represents one-shot cost or ongoing energy/s. `Weapon`/`MissileModel` lack energy costs (`154–175`). Command shield “drains energy instead of shield HP” (`special_modules.json:7`), but no numeric damage-to-energy ratio or exhaustion routing is given.
- **Sourced external claim:** none; weapon/afterburner/special energy behavior and regeneration conditions in Star Conflict not verified.
- **Classification / confidence / impact:** coverage gap / uncertainty; high local confidence; high shared-resource impact.
- **Reconciliation question/options:** Which actions consume energy, does regeneration continue during them, and what happens when a command-shield hit exceeds available energy? Do not equate absent cost fields with a confirmed zero-cost live mechanic. Retain the simple active-only contract if intentional.
- **Local balance observation, not a bug:** afterburner is 20 energy/s (`active_modules.json:8–10`), while the nine base regeneration values range 45–60/s. Under the literal continuous additive C16 interpretation and no competing load, afterburner alone would not deplete their energy. Whether that is desired is a product question, not evidence of drift.

### COM-07 — movement simplification and “instant” acceleration wording

- **Exact local claims:** `ontology.md:52` and `domain.md:82` say arcade speed cap/drag/instant acceleration. `sim/src/lib.rs:43–58` has `accel` and `drag` response rates in `1/s`, fixed at `6.0` and `1.5`; `71–85` exponentially approaches target velocity. At a 1/60-second step from rest, the existing expression approaches about **9.516%** of the target velocity, not 100%. `Input` has throttle, yaw, pitch and roll only (`13–21`); all rotation axes use the same turn rate (`71–76`).
- **Sourced external claim:** none; no Star Conflict movement formula, speed cap, strafe speed or axis-rate values obtained.
- **Classification / confidence / impact:** deliberate simplification, with terminology uncertainty and strafe coverage gap; high confidence; medium design/interface impact.
- **Reconciliation question/options:** Describe “instant” as a fast arcade velocity response or decide whether literal instantaneous changes were intended. Is strafe an intended future control? Are independent axis rates required? Do not call `accel=6` a physical acceleration of 6 m/s²; the code documents a response constant instead. Preserve the explicit simplified motion model unless scope changes.

### COM-08 — afterburner exists as catalog effect, not current flight control

- **Exact local claims:** `active_modules.json:8–10` models unrestricted, ongoing 20 energy/s afterburner with 5-second re-enable cooldown and “Raises the speed cap while on.” No boost magnitude is defined. `client/src/main.rs:55–62` sends only throttle/rotation; `sim::Input` has no boost or strafe; `FlightParams` is derived once before the loop (`client/src/main.rs:19`).
- **Sourced external claim:** none; whether Star Conflict afterburner is a baseline control, equipment-dependent effect, or has particular cost/cooldown semantics is not established here.
- **Classification / confidence / impact:** coverage gap between declarative equipment and current flight runtime; high confidence; medium impact. Classification of actual game modules is deliberately not attempted.
- **Reconciliation question/options:** Is this an intentional optional local ability occupying one active slot, and what changes the cap? Keep it as an original design if so; collect authoritative movement/control evidence before claiming equivalence. No runtime implementation requested.

### COM-09 — weapon firing has no hit-delivery, range or ammo semantics

- **Exact local claims:** `domain.md:123–133` / `model.rs:154–164` describe one damage type, per-shot damage, shots/s and heat/cooling. No beam/projectile/hitscan, velocity, lifetime, range, falloff, spread, targeting/lead, ammunition or reload fields exist. `weapons.json:2–11` gives only that common shape even for names “Heavy Laser,” “Heavy Railgun,” “Pulse Laser,” and “Plasma Gun.”
- **Sourced external claim:** none. Names are not sufficient evidence of beam behavior, damage kind, speed, firing cadence, or ammunition in Star Conflict.
- **Classification / confidence / impact:** coverage gap; high confidence; high if future combat must answer hit-resolution or sustained-fire questions. Absence is not proof all local weapons have infinite range/ammo or identical projectiles.
- **Reconciliation question/options:** What minimal hit-delivery distinctions must Stargem answer? Is one weapon ID a weapon family/install or one physical mount? Record an explicit scope decision; do not introduce a projectile hierarchy merely because another game may have one.

### COM-10 — precise heat model is a local commitment with edge ambiguities

- **Exact local claims:** `domain.md:131–133,252`: heat in 0..1, firing adds per-shot heat, **idle** removes cooling/s, at 1 lock until fully cooled plus penalty. `weapons.json:2–11` sets all five profiles. Narrative `ontology.md:53–54` only says prolonged use overheats and overheating takes longer to cool.
- **Sourced external claim:** none; idle-only cooling, threshold behavior and extra penalty have not been verified against Star Conflict.
- **Classification / confidence / impact:** uncertainty of fidelity and runtime boundaries; high local confidence; high sustained-fire/balance impact.
- **Reconciliation question/options:** Does “idle” mean trigger released or time between shots? If an addition crosses rather than exactly equals 1, is heat clamped and lockout triggered? Does the extra penalty begin after zero heat or overlap cooling? The prose suggests a sequential extra delay, but no executable timer resolves it. This is not a proven overshoot bug because there is no heat implementation.
- **Local worked check:** Heavy Laser `cooling_per_s=0.25` and `overheat_penalty_s=3`; a literal uninterrupted cool-from-one then extra-delay reading gives 4+3=7 seconds. This is arithmetic on the local definition, not a claimed Star Conflict cooldown or full firing-cycle simulation.

### COM-11 — missile ammo/launch interval conflate possible lifecycle distinctions

- **Exact local claims:** `domain.md:142–143`: `ammo` is “per launcher per life”; `reload_s` is “between launches.” C8 (`245`) has exactly one launcher loaded with one model. `model.rs:167–175,188–194` stores a missile model and selection ID, not a launcher instance or changing stock. The listed ship-state (`domain.md:186–189`) includes heat and active cooldowns but does not name remaining missile ammo or a missile launch timer.
- **Sourced external claim:** none; no documentation on Star Conflict launchers, per-life limits, magazines, rearm/replenishment, or reserves was obtained.
- **Classification / confidence / impact:** deliberate one-launcher simplification where explicitly specified; coverage gap / uncertainty for lifecycle state; high confidence; high if missile persistence or sustained combat is implemented.
- **Reconciliation question/options:** Does `ammo` mean an entire life budget exactly as written, or was a replenishable magazine intended? Is `reload_s` solely launch cadence? Retain those precise local meanings unless revised intentionally; if a later source distinguishes several timers/pools, do not collapse them under the word “reload.” A separate launcher catalog is not automatically necessary.

### COM-12 — missile names do not define lock/guidance or special effects

- **Exact local claims:** `missiles.json:2–4` contains `homing-missile`, `cruise-missile`, and `emp-missile`, but all share only damage type, damage, scalar speed, ammo and interval. `model.rs:167–175` has no lock time, target requirement, guidance discriminator, turn rate, lifetime/range, blast radius or EMP status effect. The narrative says missile models have “distinct flight and damage behavior from weapons” (`ontology.md:55`).
- **Sourced external claim:** none. Actual lock-on requirements, guidance and ammunition behavior remain unverified.
- **Classification / confidence / impact:** coverage gap; high confidence; high implementation impact if the names are intended to promise behavior beyond damage/speed.
- **Reconciliation question/options:** Are homing/EMP merely labels in an illustrative catalog or behavioral commitments? If behavioral, state the required targeting/effect rules before implementation. Do not infer an EMP disable from electromagnetic damage, or guided flight from the homing label alone.

### COM-13 — effective-stat positivity check is attributed to the wrong function

- **Exact local claims:** C5 (`domain.md:242`) assigns effective-stat positivity enforcement to `Catalog::effective_stats`. That function (`model.rs:390–403`) only looks up and sums base plus deltas. `validate_loadout` performs the range checks (`382–386`). `client/src/main.rs:14–19` loads, buys and computes stats without calling either catalog validation or loadout validation.
- **Sourced external claim:** not applicable; this is an internal contract/caller-boundary issue.
- **Classification / confidence / impact:** internal logical/documentation issue; high confidence; medium impact on future input/content validation. Current supplied stock stats are positive; no invalid live session demonstrated.
- **Reconciliation question/options:** Correct the documented checker attribution, or intentionally enforce the invariant at a defined boundary in a separate task. Do not imply the current `Option<Stats>` promises positive output. Existing tests (`model.rs:439–460`) explicitly validate before asserting modified stats; the client does not follow that pattern.

## 7. Three representative local examples and what they cannot prove

All values below are from existing files. Arithmetic applies only the explicitly documented **local** rules and does not establish real-game identity or balance.

### Example A — Anaconda / Heavy Laser / Cruise Missile

- `ship_models.json:2–5`: local engineer/frigate; shield 9000, armor 6000, energy 600, regeneration 60/s, speed 180 m/s, agility 20 deg/s.
- `weapons.json:2–3`: Heavy Laser is thermic, 420 damage/shot, 2 shots/s, 0.05 heat/shot, 0.25 cooling/s, 3-second extra penalty. With `damage_types.json:4`, a hit wholly applied to either one layer receives multiplier 1. No beam/range/ammo behavior follows from the name.
- `missiles.json:3`: Cruise Missile is kinetic, 3500 damage, 400 m/s, ammo 4 per local life budget, interval 15 seconds. No cruise guidance, target lock, launch acceleration or splash is represented.
- **Use:** concrete full-cooling interpretation check in COM-10; shows where projectile and launcher questions begin. Not a verified actual-game Anaconda loadout.

### Example B — Hydra / Light Armor / Gyro Stabilizer

- `ship_models.json:14–17`: base armor 4500, speed 260 m/s, agility 40 deg/s; local stock Assault Railgun and Homing Missile.
- Equip one `light-armor` and one `gyro-stabilizer` (`passive_modules.json:4,8`). Their armor/motor slots fit Hydra's two of each. By C5 the resulting values are armor **3900**, speed **290**, agility **48**; shield 4500 and energy 450/50 per second are unchanged. Checked using the JSON data.
- `weapons.json:6–7`: local Assault Railgun is kinetic, 260 damage/shot, 3 shots/s. On a hit applied wholly to shield, the local multiplier gives 182; wholly to armor gives 390. This deliberately avoids the unresolved overflow case.
- `missiles.json:2`: local Homing Missile is thermic, 1200 damage, 600 m/s, ammo 6, interval 8 seconds. No homing algorithm or locking requirement is defined.
- **Use:** validates additive stat interpretation and separates a named missile from unmodeled guidance. No external class/weapon correspondence asserted.

### Example C — Kite / Plasma Gun / Afterburner

- `ship_models.json:34–37`: local ECM/interceptor; shield 3000, armor 2000, energy 450, regeneration 50/s, speed 350 m/s, agility 58 deg/s.
- `weapons.json:10–11`: Plasma Gun is electromagnetic, 140 damage/shot, 4 shots/s, 0.04 heat/shot, 0.35 cooling/s, 2-second extra penalty. Local single-layer outcomes are 210 shield or 98 armor; no actual-game plasma rule asserted.
- Unrestricted afterburner (`active_modules.json:8–10`) would occupy one active entry and demand 20 energy/s with a 5-second re-enable cooldown. Under additive continuous regeneration, with no other drain, 50−20=30 energy/s before the capacity cap; the speed increase is not numerically specified and no boost input is implemented.
- **Use:** distinguishes stat capacity, energy flow, effect magnitude and runtime implementation. Not proof actual Star Conflict afterburners occupy an equipment slot or have these costs.

**Unmet external example requirement:** three real Star Conflict examples, with original-source support, could not be produced. The local examples must not be relabeled as that evidence.

## 8. Gruber assessment

| Criterion | Assessment and reconciliation seam |
|---|---|
| **Clarity** | Good local units and explicit layer multipliers. Clarify hull/armor aliasing, what resistance changes, raw overflow, weapon “shot,” idle cooling, launch interval versus reload, and “instant” acceleration. Source provenance is absent; do not add implied live-game authority. |
| **Coherence** | Current JSON damage profiles agree with canonical defaults. Thermic approximate-unit constraint is weaker in validation; C5 checker attribution differs from implementation. Missing future runtime mechanisms are not themselves contradictions. |
| **Extendibility** | New weapon/missile records fit current arrays. Per-target resistance, distinct hit delivery or multi-stage ammo lifecycle would require conceptual additions if actually required. No new classes or abstractions proposed without a competency question and evidence. |
| **Minimal encoding bias** | Readable IDs/enums are helpful. `agility` merges rotation axes; `accel` is a response coefficient rather than physical acceleration; launcher properties reside on missile models; u8 ammo is an encoding ceiling, not a sourced gameplay limit. None alone requires a change. |
| **Minimal ontological commitment** | Preserve the explicit arcade/one-weapon/one-launcher model unless product scope changes. Conversely, universal type multipliers and a specific overheat cycle are substantive commitments, not generic consequences of “Star Conflict-like.” Document whether they are original design or sourced fidelity. |

## 9. Unanswered product/research questions (report only)

1. Is combat meant to reproduce a dated Star Conflict ruleset, or be an independently balanced inspired game? What evidence date/version should govern disagreements?
2. Are the current weapon/missile names exact identity claims or illustrative local catalog labels? Do numeric constants have any provenance?
3. Should armor be a documented hull synonym, and is damage-specific resistance part of the intended local model?
4. What exact raw-damage accounting applies at shield exhaustion? Is an exception/bypass mechanic in scope, if sourced later?
5. Is baseline shield regeneration intended, and how is it distinct from shield repairs and energy regeneration?
6. Which actions share the energy budget; what is command-shield's damage-to-energy rule and depletion behavior?
7. Are strafe, axis-specific rotation and per-ship acceleration requirements or explicitly deferred refinements? Is afterburner optional equipment by design?
8. What does a weapon “shot” represent for different delivery modes, and which range/ammo/energy distinctions matter to the planned combat questions?
9. Does overheat penalty run sequentially after full cooling, and is cooling possible between shots while the trigger remains held?
10. Are missile ammo values life budgets, magazines or something else; what replenishment and launch timers must exist? What behavioral promise follows from “homing” and “EMP” names?

No question was sent to the user; no semantics were changed to answer one.

## 10. Validation and review evidence

### Commands/actions performed

- Verified requested root instruction-file absence before substantive reads; inspected the skill, inventory and relevant local files using read tools.
- `git rev-parse HEAD`, `git branch --show-current`, `git status --porcelain=v1`, `git diff --cached --name-only`, `git ls-files '*.rs'`: baseline/master confirmed; no working-tree or staged changes; only three tracked Rust files.
- `command -v curl`, `command -v markitdown`, `command -v python3`: available. `command -v cargo`: no path. No build environment was installed or realized.
- Bounded `curl` fetch batch, DNS/proxy checks, one IPv4 diagnostic and independent Steam seed as detailed in §2: network retrieval failed. No external substantive claims passed verification.
- Read-only Python standard-library assertions over the JSON and source text: counts **3 damage types, 5 weapons, 3 missiles, 9 ships**; stock weapon/missile references resolve; all weapon and missile scalar facets tested meet the documented checks; Hydra's selected modifiers yield `3900/290/48`; thermic `100/100` satisfies the existing scalar difference predicate; literal Heavy Laser full-cooling-plus-penalty arithmetic gives 7 seconds; base energy regeneration minimum is 45/s; baseline and clean index/worktree rechecked.

Exact validation summary returned:

```text
PASS: JSON counts, stock references, weapon/missile scalar facets, Hydra modified stats, thermal predicate counterexample, literal Heavy Laser cooling arithmetic, clean baseline/index.
COUNTEREXAMPLE: thermic 100/100 passes scalar predicate abs(a-b)<0.2; Rust validator itself not executed.
LOCAL FLIGHT CHECK: one 1/60s step from rest at accel=6 approaches 9.516258% of target velocity.
No repository files changed; no Rust tests run; no external facts verified.
```

These checks are not presented as an execution of the Rust test suite or a combat simulation. No tests were added/updated because this is a read-only research assignment. Existing ontology/sim tests were read, not run. The independent acceptance reviewer should check the cited source lines and the clear distinction between verified local findings and externally unverified candidates.

### Short reproducible read-only check

This needs Python's standard library only and writes nothing:

```sh
cd /home/theta/repos/stargem.nix
python3 - <<'PY'
import json
from pathlib import Path
p = Path('ontology/instances')
load = lambda n: {x['id']: x for x in json.loads((p / (n + '.json')).read_text())}
d, w, m, s, mods = [load(n) for n in ('damage_types', 'weapons', 'missiles', 'ship_models', 'passive_modules')]
assert (len(d), len(w), len(m), len(s)) == (3, 5, 3, 9)
assert all(x['stock_weapon_id'] in w and x['stock_missile_id'] in m for x in s.values())
h = s['hydra']['base'].copy()
for name in ('light-armor', 'gyro-stabilizer'):
    for k, v in mods[name]['modifiers'].items():
        h[k] += v
assert (h['armor_hp'], h['speed'], h['agility']) == (3900, 290, 48)
assert 'DamageKind::Thermic => (d.vs_shield - d.vs_armor).abs() < 0.2' in Path('ontology/model.rs').read_text()
assert abs(100.0 - 100.0) < 0.2  # scalar counterexample, not executing Rust
print('PASS: local example and thermal-predicate evidence')
PY
git status --porcelain=v1
git diff --cached --name-only
```

### Residual risks / acceptance boundary

- **Blocking research limitation:** actual Star Conflict information, freshness, conflicting sources and historical changes remain unverified. A later writer can use this local ledger but must still retrieve external evidence before publishing a real-game drift claim.
- Combat-specific ambiguities are not resolved by currently executable flight/collision behavior; numeric local examples are not live formulas.
- No Rust build/tests were executed; source inspection plus read-only scalar/data checks is the stated validation level.
- Required reviewer gate remains outstanding. No source or ontology change is recommended as already authorized.

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "not-satisfied",
      "evidence": "Scoped read-only combat report delivered without repository or semantic changes, but the requested actual-game research could not be completed: all external retrievals timed out and the supervisor instructed a partial report."
    },
    {
      "id": "criterion-2",
      "status": "satisfied",
      "evidence": "Report retains exact attempted URLs and failures, baseline/file:line anchors, precise local claims and counterexamples, explicit unverified external status, read-only check outputs, resumption targets and clean-index evidence for independent review."
    }
  ],
  "changedFiles": [
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/research/combat.md"
  ],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {"command": "git rev-parse HEAD; git branch --show-current; git status --porcelain=v1; git diff --cached --name-only", "result": "passed", "summary": "Expected baseline on master; no repository or staged changes."},
    {"command": "Bounded curl requests to official site, official-hosted wiki, Google search and Steam; one IPv4 wiki diagnostic", "result": "failed", "summary": "All TCP connections timed out with curl exit 28; zero pages retrieved."},
    {"command": "getent ahostsv4 wiki.star-conflict.com star-conflict.com www.google.com; inspect proxy environment", "result": "passed", "summary": "DNS resolved; no proxy variables present; timeout occurred before HTTP response."},
    {"command": "Read-only Python standard-library JSON/source/scalar assertions", "result": "passed", "summary": "Counts, references, facets, local loadout arithmetic, thermal predicate counterexample and clean baseline/index verified; not an execution of Rust."},
    {"command": "cargo test", "result": "not-run", "summary": "Research-only assignment; cargo absent from PATH; no packages or build environments installed."},
    {"command": "markitdown fetched HTML", "result": "not-run", "summary": "Tool available but no HTML was retrieved to convert."}
  ],
  "validationOutput": [
    "3 damage types, 5 weapons, 3 missiles and 9 ships checked; stock references resolve.",
    "Hydra + Light Armor + Gyro Stabilizer: armor 3900, speed 290, agility 48.",
    "Thermic 100/100 passes the existing scalar difference predicate; shipped thermic instance is 1/1.",
    "No repository files changed; no external facts verified."
  ],
  "residualRisks": [
    "Internet verification blocked; no actual-game contradiction or agreement established.",
    "Article dates, freshness and three source-grounded real-game examples unavailable.",
    "Rust tests not executed; required independent reviewer gate remains outstanding."
  ],
  "noStagedFiles": true,
  "diffSummary": "External research artifact only; repository/source/ontology untouched.",
  "reviewFindings": [
    "blocker: external research incomplete due to outbound connection timeouts.",
    "internal issue: ontology/model.rs:293 checks thermic multiplier equality, not proximity to one required by ontology/domain.md:246.",
    "internal issue: ontology/domain.md:242 attributes positivity checking to effective_stats, while model.rs:382-386 performs it in validate_loadout."
  ],
  "manualNotes": "Supervisor authorized stopping retries and returning a PARTIAL report. No remembered game mechanics were used as evidence; all external comparison candidates remain unverified."
}
```
