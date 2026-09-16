# Handoff: research-combat — 2fcf909b

Checkpoint: 2026-09-16, user-requested pause. Parent-curated from runtime records and actual files; not a new completed child report.

- **State:** COMPLETED; reuse output, do not rerun by default
- **Role:** `delegate`
- **Child run:** `2fcf909b-17b7-4dbb-a2ba-f545345836cb`
- **Workflow / key:** `507d64bc-df21-4587-a4fb-a4425950ac5f` / `research-combat`
- **Mission:** `28a33a06-b296-4a78-8ae0-079881816489`
- **Repository:** `/home/theta/repos/stargem.nix`, branch `master`
- **Source baseline:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`
- **Session (machine-local, optional for recovery):** `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/2026-09-16T18-47-01-030Z_01a0ab8b-30a5-761c-90d3-cc64125796e1/0e2b02f1-9f27-4bfe-87f1-9b959223d008/run-0/session.jsonl`
- **Runtime status (machine-local):** `/tmp/pi-subagents-uid-1000/async-subagent-runs/2fcf909b-17b7-4dbb-a2ba-f545345836cb/status.json`

## Authority and boundaries

Read root `AGENTS.md` and `docs/HANDOFF.md` before work. `ontology/` remains canonical and unchanged. This is documentation/research, not authorization to change gameplay. No commit/push/merge/install is authorized. One writer per checkout. Preserve untracked audit files: they are the current work, not disposable clutter. Other agents' full research is intentionally file-backed to protect parent context.

## Resumption (for Agent)

Research is complete for this pass. Reuse the original report below and `../research/combat.md`; do not refetch everything. Synthesis/review are still pending. Only resume this child for a specific missing citation, disputed claim or fresh evidence request. Keep publication/patch date distinct from retrieval date and deliberate local scope distinct from defects.

## Native continuation or fallback

Native resumability was verified with children.list at checkpoint. Recheck it in the next session; retained session availability can change.

```javascript
subagent({ action: "status", id: "2fcf909b-17b7-4dbb-a2ba-f545345836cb" })
subagent({ action: "children.list" })
// Only when resumable, and no original writer is still active:
subagent({ action: "resume", id: "2fcf909b-17b7-4dbb-a2ba-f545345836cb",
  message: "Read AGENTS.md, docs/HANDOFF.md and docs/ontology/handoffs/research-combat-2fcf909b.md. Continue ONLY the remaining work recorded there; preserve completed outputs and current files." })
```

If the retained session is missing or not resumable, do not claim a native resume or rerun all research. Launch a fresh `delegate` as a **same-role fallback continuation**, give it this handoff and the preserved reports, and start at the exact next step above. A resumed run may return a new run ID: record it and continue from the latest identity.

## Verification and acceptance

The original report below states exactly what this child checked and what remains uncertain. Completion of a research child does not mean that document synthesis, independent evidence review, ontology review or user reconciliation has completed. Follow [the main checkpoint](README.md) for dependency order and pending reviews.

## Preserved original output

Original managed artifact: `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/507d64bc-df21-4587-a4fb-a4425950ac5f/research/combat.md`

SHA-256 of original artifact: `5e342e08919c260f28cb3c8a6a7ee9b8d55172b72821d42d822e90d74495501d`

The complete output is embedded below so recovery does not depend on session/temp-file retention. Original relative code paths refer to the repository root unless the report says otherwise. Historical access failures stay historical.

---

# Star Conflict combat-mechanics research and reconciliation candidates

Audit date and retrieval date: **2026-09-16**. Repository: `/home/theta/repos/stargem.nix`; branch supplied as `master`; independently verified HEAD: `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`.

## Scope, authority, and outcome

This is evidence for reconciliation, **not approved Stargem rules**. `ontology/` remains canonical; `ontology.md` is its source narrative, not an authoritative description of Star Conflict. No repository, ontology, source, or existing audit-draft files were edited. No installations, builds, commits, staging, branches, pushes, or nested agents were used.

Read root `AGENTS.md` first after checking its presence; `docs/HANDOFF.md` was absent. Unlike the older inventory's kickoff state, root `AGENTS.md` now exists and was respected. The existing untracked `AGENTS.md` and `docs/ontology/` are authorized work and were preserved. Read the requested ontology skill and inventory, then independently read `ontology/domain.md`, `ontology.md`, `ontology/model.rs`, relevant instances (damage, weapons, missiles, ships, passive/active/special modules), and the complete simulation/client consumers.

**The network retry succeeded.** Between approximately 19:38 UTC and this report's preparation, all **24 HTML requests returned HTTP 200** and produced nonempty `markitdown` output. They represent 21 distinct content pages after deduplicating three aliases of Ship construction. Public MediaWiki discovery APIs also responded. Previous access-blocked conclusions do not describe this research run; retain them only as earlier chronology.

### Most consequential results

1. **Documented resistance is target/layer/type dependent, not a universal multiplier attached to a damage kind.** The qualitative EM/shield and kinetic/hull tendencies agree with the narrative, but C9's constant `1.5/0.7` implementation model does not represent the documented mechanism.
2. **Two exact reused names disagree with their sourced counterparts:** local Pulse Laser is electromagnetic/fighter rather than thermal/interceptor; local Cruise Missile is kinetic rather than thermal. Whether these are errors or intentionally renamed/custom equipment is a product decision.
3. **Missile cartridge capacity, launch interval, and cartridge reload are distinct.** The local finite `ammo per life` plus one `reload_s between launches` cannot express the documented Cruise missile's 4-charge cartridge, 12-second recharge, and 120-second cartridge reload.
4. **Shield regeneration, nonlinear capacitor regeneration, strafing, separate movement rates, lock-on, projectile/range behavior, and weapon munitions are either missing or simplified.** Missing representation is not automatically a logical defect or a mandate to add it.
5. **No reliable shield-crossing hit formula was found.** The sources describe shield-first damage and exceptions but do not settle C9's ambiguous remainder accounting. Several wiki formulas also conflict internally; do not import them blindly.

Here, **verified contradiction** means an exact disagreement with directly fetched, dated documentation, not experimental verification of the running September 2026 client. Confidence in the documented disagreement and confidence that an old numerical value remains live are different. No wiki revision is called current merely because it was retrieved today.

## Research method and Stanford seven-step application

Initial navigation used three independent angles: (1) official homepage/game guide for primary-source framing and freshness, (2) official-hosted wiki Main Page → Damage/Resistance/Ship construction, and (3) weapon and missile indexes → named item pages. Public MediaWiki search/allpages located regeneration and ship-system terminology; snippets were discovery only. One bounded gap-closing pass inspected Ship construction, a concrete ship, weapon-temperature behavior, and the conflicting Singularity classification. No search-engine dependency or unbounded crawl was needed.

HTML was retrieved using `curl -L --connect-timeout 10 --max-time 35 -sS`; API queries used a 25-second maximum. Each HTML document was converted with installed `markitdown` before reading. A standard-library `html.parser.HTMLParser` was used **only for Lynx's nested tables**, whose row/column grouping was degraded in Markdown; it verified the original rowspans/colspans and the level-5 data below. No external page was treated as an instruction.

### Step 1 — combat competency questions

| Question | Evidence-based answer / local consequence |
|---|---|
| What damage kinds and damage properties must combat distinguish? | Three ordinary weapon kinds are documented; Damage also describes raw/white damage. Explosive behavior is an additional property, not a fourth ordinary elemental weapon kind. Scope decision needed for collision/raw damage. |
| What makes EM better against shields and kinetic better against hull? | Default per-layer resistances, which equipment can change; not unconditional type multipliers. [S1–S3, S12] |
| When does a ship die, and what does the shield protect? | Hull strength zero destroys it; ordinary damage reaches the shield first, with documented bypass exceptions. Exact same-hit overflow accounting remains unresolved. [S2–S3] |
| What regenerates and what spends energy? | Native shield recovery; normally no native hull healing; stored capacitor energy powers afterburners/modules and some weapons. Capacitor regeneration is described as nonlinear. [S3–S4, S6, S13] |
| Which independent motion capabilities exist? | Normal/reverse speed, acceleration, afterburner speed/draw, strafe, pitch/yaw and roll. Local arcade simplification is explicit. [S3, S12–S13] |
| What determines a weapon hit and sustained firing? | Damage, rate, turret firing behavior, criticals, lead/projectile characteristics, range, spread, heat/cooling, sometimes energy/repair effects. Only a subset is local. [S6–S8, S14–S15] |
| How do missiles find targets and replenish? | Guided/unguided behavior, lock state and countermeasures; cartridge capacity and cartridge reload separate from successive-launch recharge. [S9–S10] |
| Do three realistic examples fit the local model? | Pulse Laser, Cruise missile and Lynx expose specific mismatches/omitted facets; see examples. This does not require expanding a deliberately independent game. |

### Steps 2–6 — reuse, terms, classes, slots, facets

- **Reuse:** Existing `DamageKind`, `Stats`, `Weapon`, `MissileModel`, `Loadout`, `ActivationFlow` and JSON catalogs already define local concepts. No new package, parallel ontology, schema or code abstraction was created. External pages are reference vocabulary/evidence, not drop-in approved rules.
- **Terms:** hull strength/HP, shield charge/HP, resistance points, damage reduction, raw damage, kinetic, EM, thermal, explosive property, capacitor capacity/regeneration, afterburner draw, acceleration, reverse/strafe speed, pitch/yaw/roll rate, main weapon/turret, volley, projectile/beam, spread, critical chance/bonus, optimal/maximum range, heat/cooling, munition, missile equipment, guided/unguided missile, target lock, launch recharge, cartridge capacity/reload.
- **Classes:** Ship **has** hull/shield/capacitor/engine properties; a weapon is not a ship subclass. One equipped weapon definition versus multiple physical turrets is an abstraction distinction, not an automatic cardinality contradiction. Thermal/kinetic/EM and explosive/non-explosive describe different axes. Missile-slot equipment includes things that are not missiles; extending local scope to those would need a broader concept, not forcing every device into `MissileModel`'s positive speed/damage facets.
- **Properties:** Damage kind belongs to an attack/weapon; defensive resistance belongs to a target's layer for that kind. A missile's cartridge size and two timing properties belong to its firing configuration; remaining rounds, reload progress, lock and projectile state are runtime state. Rank/upgrade/ship bonuses condition published item numbers but progression itself is explicitly out of local scope.
- **Facets:** Local `f32` and normalized heat are encodings, not live-game ontology facts. Local `ammo: u8 > 0` has documented per-life semantics, which differs from cartridge capacity. Published resistances can be negative (Lynx shield EM −10), so positivity of the *damage multiplier* cannot be confused with positivity of *resistance*. Validate the already-approved local contract separately from any later fidelity change.

### Step 7 — real instance checks

Three primary examples, plus an agreeing weapon, are documented below. They were compared without creating or modifying catalog instances. Published numbers retain their rank/experience/Mk context; unspecified local numbers were not treated as a particular rank's live balance data.

## Source register

**Shared metadata:** every source below was retrieved **2026-09-16**, HTTP 200, through ordinary public HTTPS. `W` means **Star Conflict Wiki, community-maintained official-hosted wiki**, not an official patch announcement. `O` means **Star Conflict official website, StarGem/Gaijin publisher material**. Wiki dates below are page footer revision dates, not guaranteed dates of a mechanical change. Revision IDs allow later readers to retrieve the inspected version using `&oldid=…`.

### S1 — Damage

- URL: https://wiki.star-conflict.com/index.php?title=Damage
- Publisher/type: **W**. Updated **2023-09-26 12:50**, revision **14245**.
- Headings: **Damage Types**, **Damage Properties**, **Weapon Turrets**, **Base Damage**, **Critical hit**.
- Short excerpt: “if one to have 100 kinetic resistance and 100 of EM resistance on a hull, both kinetic and EM damage would behave in same way, and will be reduced by 50%.”
- Supports: kinetic/EM/thermal vulnerability shorthand comes from resistance; raw/white damage bypasses resistance and occurs with collisions/destroyed destroyer modules; explosion damage has additional target-size factors. Base damage and turret behavior are separate from critical modifiers.
- Caveats: older page; its range formula and Singularity classification conflict with S6. Its assertion that all missiles/rockets are explosive should not be expanded to every non-missile device in the missile slot.

### S2 — Resistance

- URL: https://wiki.star-conflict.com/index.php?title=Resistance
- Publisher/type: **W**, player-authored explanatory guide. Updated **2023-09-23 14:15**, revision **14214**; body says **“As of Patch 1.0.1”**, with another comparison labelled patch **1.0.4**.
- Headings: **Health (HP)**, **Resistance (R)**, **Damage Reductions (DR)**, **Linearity**, **Incoming Healing (and Recovery Rates)**.
- Short excerpt: “Resistance is the in-game attribute that is used to calculate Damage Reduction.”
- Supports: separate hull/shield HP and resistance; positive-resistance formulas; an illustrative native shield regeneration entry. Does not supply a same-hit shield-overflow algorithm.
- Explicit positive formula: `DR = (1 - 1/(1 + R/100)) × 100`; `DF = (1/(1 + R/100)) × 100` (percent). At R=100, DF=50%, agreeing with S1. These are documented formulas, not invented live formulas.
- **Do not import its negative branch or EHP notation without reconciliation:** it prints `DR=(-R), DF=100+R` for negative R, and `EHP=HP/(100-DR)=HP/DF` despite defining DF in percent. The negative branch runs against the stated resistance direction; the EHP expression omits percent normalization relative to the worked examples. Patch-age and source inconsistencies prevent treating this as a current complete damage specification.

### S3 — Ship construction

- Canonical URL fetched: https://wiki.star-conflict.com/index.php?title=Ship_construction
- Also fetched aliases, all the same content: https://wiki.star-conflict.com/index.php?title=Shield ; https://wiki.star-conflict.com/index.php?title=Capacitor ; https://wiki.star-conflict.com/index.php?title=Engine . They redirect internally to the corresponding section and are **not three corroborating sources**.
- Publisher/type: **W**. Updated **2021-08-08 15:24**, revision **11874**.
- Headings: **Hull**, **Shield**, **Engine**, **Capacitor**, **CPU**, **Main weapons**, **Missile weapon**, **Ammo refilling**.
- Short excerpt: “Regeneration is constant, but can slow down or accelerate under different influences.” This sentence refers to **shield**, not capacitor, regeneration.
- Supports: hull-zero destruction; hull normally needs repair rather than native regeneration; shields recover and usually intercept damage, with weapons/modules that ignore shields. Engine distinguishes normal/reverse speed and acceleration, energy-spending afterburner, strafing and rotation. CPU has sensor range and target lock time. Missile slot contains guided/unguided missiles and other devices; missiles and munitions can be refilled between battles.
- Caveats: old general guide; “any further damage is applied to the Hull” does not establish the arithmetic for a hit that breaks the shield. “Guided missiles require a target lock” needs the unlocked straight-flight qualification in S9. No acceleration integrator, drag coefficient or universal lock time is established here.

### S4 — Energy regeneration

- URL: https://wiki.star-conflict.com/index.php?title=Energy_regeneration
- Publisher/type: **W**. Updated **2023-09-26 12:39**, revision **14238**.
- Heading: lead text (no substantive subheading).
- Short excerpt: “The rate of regeneration of energy in the ship is nonlinear.”
- Supports: capacitor fullness determines a correction multiplied by the ship's listed energy regeneration rate.
- Caveat: includes a graph image, not a textual exact equation. No new formula was inferred, fitted or asserted from it. Present-day curve remains unverified.

### S5 — Munitions

- URL: https://wiki.star-conflict.com/index.php?title=Munitions
- Publisher/type: **W**. Updated **2025-07-21 17:39**, revision **15898**.
- Heading: introductory definition, followed by **For thermal weapons / For kinetic weapons / For EM weapons**.
- Short excerpt: “One ammunition is enough for one fight.”
- Supports: munitions are prebattle consumable shells or additional weapon systems modifying damage/projectile speed and potentially trading off other properties. They are not automatically a finite count of bullets spent per shot.
- Caveat: does not prove every gun has or lacks an in-combat magazine; do not conflate weapon munitions with missile cartridges.

### S6 — Main weapon

- Exact fetched URL: https://wiki.star-conflict.com/index.php?title=Weapons (MediaWiki redirect to **Main weapon**).
- Canonical URL: https://wiki.star-conflict.com/index.php?title=Main_weapon
- Publisher/type: **W**. Updated **2026-05-09 09:46**, revision **16428**.
- Headings: lead, **Parameters of weapons**, **Aiming**, **Shooting**, **List of weapons**.
- Short excerpts: “most weapons have zero energy consumption”; “if some weapons are completely overheated, they will take noticeably longer to cool down.”
- Supports: physical turret counts and sequential/simultaneous firing distinctions; damage at zero resistance; DPS = damage/shot × shots/second; projectile/beam size, projectile speed, spread, criticals, heat/cooling, range, some weapon energy costs and ally repair. Many thermal lasers need no lead. Shooting without target lock is described as doing 10% less damage. Weapon eligibility is not universally determined only by size; the page distinguishes common and unique/ship-specific weapons.
- Documented range relation: optimal range is **75% of maximum**; maximum is one-third more than optimal. Do not generalize to all attacks or resolve S1's conflicting text silently.
- Lead says **Singularity Cannon has size-dependent but non-explosive damage**. Damage S1 calls it explosive. This is an unresolved source conflict, not an approved mechanic.
- Turret context: interceptors 2, fighters 4, ordinary frigates 4 / long-range 6, Object NY18 exception 8; destroyers 12 mounts with up to 8 firing. These are physical mounts, not evidence of multiple independently selectable local `weapon_id`s.

### S7 — Pulse Laser

- URL: https://wiki.star-conflict.com/index.php?title=Pulse_Laser
- Publisher/type: **W**. Updated **2024-08-30 12:38**, revision **15371**.
- Headings: **Type / For ship type**, **Description**, **Characteristics**.
- Short excerpt: “Type: Close range thermal weapon”; “For ship type: Interceptor.”
- Supports: direct same-name discrepancy with local EM fighter weapon. Pulse Laser 17 Mk.1 table: **180 thermal damage**, **312 rounds/min**, **1850 m maximum range**, **4/1 s overheating/cooling**, **5% critical chance**, **50% critical damage bonus**, **0.9° spread**. Table is for **one weapon**, without ship bonuses/modifiers.
- Caveat: rank/Mk and turret context matter; 180 matching local damage does not establish shared provenance. Weapon-history flavor text mentions a “light fighter or interceptor,” but the explicit equipment field is interceptor; do not use lore to override that field.

### S8 — Assault Railgun

- URL: https://wiki.star-conflict.com/index.php?title=Assault_Railgun
- Publisher/type: **W**. Updated **2024-09-13 12:45**, revision **15387**.
- Headings: **Type / For ship type**, **Description**, **Characteristics**.
- Short excerpt: “A balanced kinetic weapon for medium distances.” Explicit ship type: fighter.
- Supports: local fighter/kinetic identity agrees. Assault Railgun 17 Mk.1: **159 damage**, **210 rounds/min**, **7101 m/s projectile speed**, **2950 m maximum range**, **6/0.8 s overheating/cooling**, **20% critical chance**, **50% critical bonus**, **0.14–2° spread**, per one weapon without bonuses.
- Caveat: local 260 damage and 3 shots/s have no specified rank, Mk or turret aggregation. They are unverified custom/sample balance, not a proved erroneous copy of this rank's values.

### S9 — Missiles

- URL: https://wiki.star-conflict.com/index.php?title=Missiles
- Publisher/type: **W**. Updated **2026-04-26 15:42**, revision **16376**.
- Headings: introduction; **Types of missiles and similar devices**.
- Short excerpts: “Missiles are loaded in cartridges”; when spent, the cartridge “will begin reloading, which is a long process.”
- Supports: guided and unguided missiles; guided missiles lock onto a target but can also be fired unlocked under unguided rules; IR flares/obstacles can defeat guided missiles. Size and sometimes role limit fitting. Missile-slot equipment also includes bombs, mines and other devices; most missile equipment deals explosive damage with target-size factors.
- Caveat: its opening lock-required sentence is qualified by its own parenthesis permitting unlocked launch. Preserve that distinction rather than reducing the mechanic to “cannot fire without a lock.” Cartridge description does not establish unlimited resupply in every mode or the exact ordering of the last-shot cooldown versus cartridge reload.

### S10 — Cruise missile

- URL: https://wiki.star-conflict.com/index.php?title=Cruise_missile
- Publisher/type: **W**. Updated **2024-01-28 22:11**, revision **14671**.
- Heading: item infobox/table (no separate section title).
- Short excerpt: “Type: Homing missile”; “Guided long-range missile. Heavy thermal damage.”
- Supports: frigate-role icons (engineering, guard, long-range); **4 charges/cartridge**, **12 s recharge**, **120 s cartridge reload**, **36°/s maneuvering**, **200 m explosion radius**, **60 m trigger radius**, **11550 m flight range**. For ranks **13–17**, **745 m/s** speed and **8717 damage against fighters**, with separate size-adjusted values for other target classes.
- Caveat: the damage row is explicitly target-size-dependent; do not call every number “base damage.” Local kinetic identity differs; its `ammo=4` matches a cartridge count only superficially because local semantics say per life.

### S11 — Star Conflict 1.14.15. “Quarter for thirty” marathon update

- URL: https://star-conflict.com/en/news/3934-star-conflict-1-14-15-quarter-for-thirty-marathon-update-en
- Publisher/type: **O, official update announcement**. Published **2026-09-11**; no separate update date shown.
- Heading: **Changes to the “Ways of kindness” marathon**.
- Short paraphrase: the announcement changes tasks from restoring 70,000 shield to 40,000 shield, and from restoring 100,000 hull to 75,000 hull.
- Supports: fresh official use of **shield** and **hull**, and repair/restoration objectives; identifies the latest dated update inspected. It does **not** prove unchanged 2021–2025 regeneration, resistance, heat or missile values, and contains no such formula.

### S12 — Lynx

- URL: https://wiki.star-conflict.com/index.php?title=Lynx
- Publisher/type: **W**. Updated **2025-06-07 10:18**, revision **15773**.
- Headings: **Tables of ship's technical characteristics** → **Ship's resistance to incoming damage**, **Ship's characteristics on different experience levels**, **Ship's speed on different experience levels**.
- Precise paraphrase: separate rows record layer/type resistance, shield and energy regeneration, afterburner draw, acceleration, strafe, pitch and roll. Main data are conditioned on experience level and no equipment.
- Supports: concrete existence of those independent facets. At experience level 5: hull **4031**, shield **4147**, energy capacity **492**, afterburner draw **86/s**, listed energy regen **124/s**, shield regen **96/s**, sensor range **3000 m**, target lock **1 s**; normal speed **356 m/s**, afterburner **427 m/s**, acceleration **92 m/s²**, strafe **88 m/s**, pitch **72°/s**, roll **66°/s**. Shield resistance thermal/kinetic/EM **20/50/−10**; hull **35/5/65**.
- Caveat: not an instruction to replace local Lynx's custom values. The page also calls Lynx a tackler, but ship-role reconciliation is outside this delegate's exclusive mechanics focus. Original HTML row/column spans were checked after Markdown degraded table layout.

### S13 — Spaсeship modules

- URL: https://star-conflict.com/en/game/gameguide/1/current/
- Publisher/type: **O, official explanatory game guide**, not a patch note. Publication/update date **unknown**; path word `current` does not prove currency.
- Headings/labels: **Hull**, **Energy Plant**, **Shunting Engines**, **Boosters**, **Afterburning Engines**, **Rocket Compartment**.
- Short excerpt: afterburning engines “provide significant acceleration” and are “extremely energy-intensive.”
- Supports: official hull terminology; energy-intensive movement boost distinguished from ordinary propulsion; ship systems constrain missile carrying. Broad energy-plant prose is not evidence that every shot consumes capacitor energy—S6 explicitly says most weapons consume zero.

### S14 — Vulcan

- URL: https://wiki.star-conflict.com/index.php?title=Vulcan
- Publisher/type: **W**. Updated **2023-08-18 11:19**, revision **13828**.
- Headings: **Description**, **Characteristics**.
- Short excerpt: “Critical hit chance increases with weapon temperature.”
- Supports: heat can affect attack properties, not merely lockout; table lists **10–50% critical chance** and **12/2 s overheating/cooling**.
- Caveat: no heat→critical-chance curve is documented in inspected text. This is an optional coverage example, not proof current local weapons must acquire this behavior.

### S15 — Singularity Cannon

- URL: https://wiki.star-conflict.com/index.php?title=Singularity_Cannon
- Publisher/type: **W**. Updated **2025-01-24 18:58**, revision **15566**.
- Headings: **Description**, **Characteristics**.
- Short excerpt: “Damage depends on target size.”
- Supports: medium-range EM fighter weapon with slow projectile spheres; named variants change firing/heat characteristics. Standard listed overheating/cooling is **8/2 s**, pirate **2.5/2 s**.
- Caveat: this item's mechanical description does not explicitly settle the explosive/non-explosive disagreement between S1 and S6. Lore mentioning an explosion is not a damage-property definition.

### Navigation and secondary pages retained, not promoted to combat-rule authority

All retrieved 2026-09-16 with HTTP 200. These have useful discovery/context roles but were not counted as independent corroboration for mechanics already supported above.

| Title / exact URL | Publisher/type; visible date/revision | Heading and faithful content / use |
|---|---|---|
| Main Page — https://wiki.star-conflict.com/index.php?title=Main_Page | W; 2024-01-19 14:24, oldid 14636 | General information / Ship equipment link directly to Damage, Resistance, weapon, missile and construction pages. Navigation evidence only. |
| Star Conflict home — https://star-conflict.com/en/ | O, news index; aggregate update unknown; visible entries include 2026-09-14 and 2026-09-11 | Game menu links official guide and wiki; news list discovered S11. Homepage snippets were not substituted for original announcement content. |
| Game guide — https://star-conflict.com/en/game/gameguide | O, official guide index; date unknown | Entry “Spaсeship modules” discovered S13; video tutorials were not watched/transcribed and support no detailed mechanics here. |
| Features — https://wiki.star-conflict.com/index.php?title=Features | W; 2023-09-26 12:36, oldid 14234 | Key Features describes combat ships/weapon customization. No additional formula; retained as navigation/background, not an independent rule source. |
| Interceptor — fetched https://wiki.star-conflict.com/index.php?title=Interceptors ; canonical https://wiki.star-conflict.com/index.php?title=Interceptor | W; 2026-03-10 18:25, oldid 16346 | Unique characteristics: two turrets, reduced explosion damage and incoming healing; used for discovery only. Its 33.3% explosion reduction versus the 66%-received approximation on other pages is not resolved as an exact balance value. |
| Ship — https://wiki.star-conflict.com/index.php?title=Ship | W; 2025-12-28 15:17, oldid 16206 | Lists four classes and explicitly labels technological tiers “old.” No class expansion is requested by this combat report. |

## Domain relationships and cardinalities worth preserving

| Concept / relation | Local representation | Sourced distinction and audit consequence |
|---|---|---|
| Damage kind → layer response | One global pair `vs_shield`, `vs_armor` per kind | Target layer has resistance **per kind**; ordinary weapon kind still singular. Raw bypass and explosion properties are orthogonal. [S1–S3] |
| Ship → defensive pools | One shield pool, one armor pool; armor-zero death | “Hull” is the documented HP pool; no evidence here requires a third separate armor-HP layer. [S3, S11] |
| Ship → energy | Capacity/current energy and scalar regeneration | Capacity and nominal regen agree; regeneration correction depends on fullness. Afterburner/modules/some weapons can draw from it. [S3–S4, S6] |
| Ship → motion | One speed and one angular agility | Multiple translation/rotation capabilities have separately listed quantities. No exact external flight equation was established. [S3, S12] |
| Loadout → weapon | Exactly one weapon definition | One selected weapon may operate multiple physical guns; sequential and simultaneous firing aggregate differently. A one-ID abstraction can remain valid. [S6–S8] |
| Weapon → munition | No relation | Consumable configuration distinct from weapon, with an effect lasting a battle per guide. Exact required/optional loadout cardinality was not established. [S5] |
| Loadout → missile equipment | Exactly one missile-model/launcher | A missile slot is documented, but can hold non-missile devices. No evidence inspected requires multiple independently selected launcher slots. [S3, S9] |
| Missile configuration → firing resources | `ammo` per life; `reload_s` between launches | Cartridge size, per-launch recharge and full cartridge reload are different quantities. Total reserves/refill rules by mode remain unresolved. [S9–S10] |
| Guided attack → target lock | Absent | Lock acquisition/state, guided flight, unlocked straight flight and countermeasures are behaviorally distinct. No universal lock-time constant: S3 says normally ~3 s, Lynx table says 1 s. [S3, S9, S12] |

## Candidate reconciliation ledger

Each candidate is bounded to combat mechanics. **No row authorizes an ontology change.** “High confidence” below refers to text/representation comparison; source-age qualifications remain as above.

### COM-01 — Constant multipliers are not the documented resistance model

- **Local claim:** `ontology/domain.md:66–71,210,246` defines response as `raw × vs_shield` / `raw × vs_armor`, with unconditional EM and kinetic inequalities. `ontology/instances/damage_types.json:2–4` fixes EM `(1.5,0.7)`, kinetic `(0.7,1.5)`, thermic `(1,1)`. `ontology/model.rs:95–100,285–296` encodes/validates it.
- **Sourced claim:** S1 explains that default vulnerabilities arise from resistance and equal resistances produce equal reduction regardless of kinetic/EM identity; S2 explicitly defines resistance-derived damage factors; S12 has different shield/hull resistances for each kind.
- **Classification:** **Verified contradiction with documented mechanics**, potentially an intentional independent-game simplification, not an internal defect simply because it differs.
- **Confidence / impact:** High on mismatch; medium on a complete current formula. **High impact** on damage, defensive fitting and effects that change resistance. Sourced EM vulnerability is not intrinsically a 1.5× modifier.
- **Reconciliation question/options:** Retain these constants and label the rule intentionally simplified, or adopt target/layer resistance after choosing a dated ruleset and verifying negative resistance/stacking/overflow. Avoid merely renaming the constants “resistance.”

### COM-02 — Raw damage and explosion properties are outside the three-kind model

- **Local claim:** `ontology.md:16–19`; `ontology/domain.md:66–71`; `ontology/model.rs:55` contain only EM/kinetic/thermic. Weapon/missile records have one `damage_type` and no attack-property dimension (`model.rs:154–175`). Collisions only separate bodies and cancel velocity (`sim/src/lib.rs:97–111`); the client only flashes collision feedback (`client/src/main.rs:63–64`).
- **Sourced claim:** S1 lists raw/white damage and resistance bypass; S6/S9 describe size-sensitive explosive attacks, while S6 distinguishes some non-explosive size-dependent attacks.
- **Classification:** **Coverage gap**, not “all three ordinary damage kinds are wrong.” Collision damage may be deliberately unimplemented; destroyer-specific behavior need not enter local scope.
- **Confidence / impact:** High existence evidence; exact special-case rules medium/uncertain. Medium impact if collision/explosion gameplay is intended.
- **Question/options:** Are the three enums intended to cover ordinary weapon damage only or every damage event? Explicitly defer raw/explosion behavior or separately specify it; do not add “explosive” as if it were the same axis as thermal.

### COM-03 — Armor versus hull is mainly a terminology reconciliation

- **Local claim:** `ontology.md:13–15`, `ontology/domain.md:79,187,250` call the destruction pool `armor_hp`; the HUD says armor (`client/src/main.rs:91`). `passive_modules.json:3` already calls one armor modifier “Reinforced Hull.”
- **Sourced claim:** S3 Hull defines strength-zero destruction; S11 uses official “restore hull” language. S2 likewise calls this HP Hull. Some wiki prose also uses armour informally.
- **Classification:** **Terminological uncertainty / agreed underlying fact**, not proof of a missing third defense layer.
- **Confidence / impact:** High; low mechanical impact, medium clarity impact.
- **Question/options:** Document `armor_hp` as a local synonym for hull strength, or later rename public terminology with migration planning. Do not invent separate hull and armor HP without a new requirement.

### COM-04 — Passive shield recovery is not represented

- **Local claim:** Stats have `shield_hp` but no shield regeneration (`ontology/domain.md:73–83`; `ontology/model.rs:74–92`); C16 only specifies energy regeneration (`domain.md:253`). Shield-boost/remote-repair effects exist as active effects (`active_modules.json:2–7`), not native shield recovery.
- **Sourced claim:** S3 says shields automatically recover with a regeneration stat; S12 records Lynx shield regeneration 96/s. Hull normally does not recover naturally without a repair mechanism.
- **Classification:** **Coverage gap**, not a contradicted explicit “shields never regenerate” statement.
- **Confidence / impact:** High representation gap; medium freshness of precise rates. High if sustained-combat fidelity is desired.
- **Question/options:** Keep native recovery explicitly omitted, or define shield regen and its modifiers/timing. Do not infer an out-of-combat delay, interruption timer, or energy cost absent sourced support.

### COM-05 — Shield-breaking hit remainder remains ambiguous

- **Local claim:** C9 at `ontology/domain.md:246`: “shield takes `raw × vs_shield` until 0, remainder hits armor at `raw × vs_armor`.” No combat damage function exists in inspected runtime files.
- **Sourced claim:** S2/S3 support shield-first ordinary damage and then hull, but do not specify how a single attack exhausting a shield conserves raw-equivalent damage versus post-reduction damage. S3 notes bypass exceptions.
- **Classification:** **Uncertainty in local rule**, plus coverage gap for bypass. **Not a verified live-game formula contradiction.**
- **Confidence / impact:** High that wording is under-specified; actual external algorithm unknown. High for a future damage implementation.
- **Question/options:** Clarify what unit “remainder” has and whether crossing uses one attack's residual raw damage, residual reduced damage, or another rule. Record a small worked example once selected. Do not silently infer an overflow equation from “any further damage.”

### COM-06 — Constant energy recovery differs from the nonlinear description

- **Local claim:** `ontology/domain.md:81,253` says continuous `energy_regen`/s; `Stats` has capacity and one regen scalar (`ontology/model.rs:77–78`), with additive modifiers only. Narrative merely says energy regenerates continuously (`ontology.md:14`), which alone does not imply linearity.
- **Sourced claim:** S4 expressly says rate is nonlinear and depends on capacitor fullness; S12 lists a nominal energy regen separately from afterburner consumption.
- **Classification:** **Verified documentation mismatch** for C16's constant-rate interpretation; potential deliberate simplification. Not a contradiction of continuous regeneration itself.
- **Confidence / impact:** High textual mismatch; medium present-day curve confidence. Medium/high for energy starvation and sustainable afterburning.
- **Question/options:** Approve flat regen as the local rule, or define a nominal-rate/fullness distinction after verifying the curve. No exact new live formula is established by this audit.

### COM-07 — Motion is intentionally compressed; strafing/afterburning remain uncovered

- **Local claim:** `ontology.md:52`, `ontology/domain.md:82–83` explicitly choose arcade cap/drag/“instant accel” and one agility. `sim/src/lib.rs:14–21,54–57,71–85` has throttle/yaw/pitch/roll only, one turn rate and fixed accel=6/drag=1.5; reverse throttle uses the same speed cap. The comment deliberately defers per-ship acceleration. `active_modules.json:8–10` represents afterburner as ongoing 20 energy/s, 5-second re-enable cooldown, raising the speed cap; current flight input never activates it.
- **Sourced claim:** S3 distinguishes reverse speed, acceleration, strafe and rotary axes; S12 has finite acceleration and separate normal/afterburner/strafe/pitch/roll values; S13 corroborates energy-intensive afterburning.
- **Classification:** **Deliberate simplification** for arcade movement; **coverage gaps** for strafe/afterburner execution and independent rates. The 20/s and 5 s are **unverified local design values**, not sourced game constants. Active-module classification is left to the equipment audit.
- **Confidence / impact:** High; medium impact unless flight fidelity is a goal.
- **Question/options:** Preserve the explicit arcade contract and mark unsupported capabilities, or select which independent motion axes matter before expanding stats. Do not claim that non-Newtonian/arcade design itself contradicts the game; no live physics integrator was sourced. “Instant” currently means exponential approach with a finite tuning coefficient, which should remain clear in local terminology.

### COM-08 — Pulse Laser's damage type and ship class disagree

- **Local claim:** `ontology/instances/weapons.json:8–9`: `pulse-laser`, name “Pulse Laser”, size `fighter`, damage type `electromagnetic`, 180 damage, 5 shots/s. Local Lynx stocks it (`ship_models.json:22–25`); C7 enforces its fighter size (`domain.md:244`).
- **Sourced claim:** S7 explicitly identifies **thermal**, **interceptor**, also listed under thermal weapons in S6.
- **Classification:** **Verified same-name contradiction**, conditional on intending the Star Conflict item rather than an original item reusing its name.
- **Confidence / impact:** High sourced identity; medium source freshness. High fitting/type impact; changing only the item size would invalidate current stock loadout relationships.
- **Question/options:** Declare/rename a custom fighter EM pulse weapon, or reconcile the catalog identity and all stock references together in a later approved ontology pass. Do not mechanically copy a table's per-turret damage into an unspecified local per-shot aggregate.

### COM-09 — Exact local heat algorithm is not externally verified

- **Local claim:** `ontology/domain.md:130–133,252`: normalized heat added per shot, removed only while idle, then full cooldown **plus** extra penalty; all five records set positive penalties (`weapons.json:2–11`).
- **Sourced claim:** S6 defines full overheating time, cooling time until firing resumes, and additional penalty for **some** weapons. S7/S8/S14 show distinct overheating/cooling pairs. S14 describes temperature-dependent critical chance.
- **Classification:** **Broad agreement** on heat/forced downtime; **uncertainty / deliberate model choice** for the exact per-shot/idle/additive-timer algorithm; **coverage gap** for temperature-dependent effects.
- **Confidence / impact:** High broad agreement; low exact-live-algorithm confidence. Medium sustained-DPS impact.
- **Question/options:** Keep the normalized local rule explicitly custom, or verify firing-time/idle-time/overheat transitions before claiming fidelity. Do not claim that `4/1 s` automatically means a particular `heat_per_shot`, or that the table's cooling already excludes/includes a separate penalty. No external evidence here shows every weapon has a positive penalty.

### COM-10 — Weapon firing, projectiles, targeting and range are under-specified

- **Local claim:** `ontology/domain.md:123–133`; `ontology/model.rs:154–164` provide size/type/damage/rate/heat only. `Loadout.weapon_id` is one ID (`domain.md:177`). Runtime state (`domain.md:186–189`) has no targeting or projectile state.
- **Sourced claim:** S6 defines beam/projectile differences, turret firing/aggregation, lead/aim, spread, projectile velocity, criticals, optional energy draw and optimum/maximum range. S8 is a concrete finite-speed projectile example; S7 a close-range laser example. S6's unlocked-fire damage penalty is another unrepresented targeting rule.
- **Classification:** **Coverage gap**, not proof the one-weapon-ID abstraction is false. No range field is not a positive assertion of infinite range.
- **Confidence / impact:** High existence of missing facets; exact ranges/formulas source-dependent. High before implementing hit resolution.
- **Question/options:** Define what “shot” aggregates and select minimum required behaviors (beam vs projectile, range, targeting) if implementation is authorized later. Keep advanced crit/spread/unique weapon behaviors out of scope if unnecessary. Resolve conflicting range statements before adopting a global formula.

### COM-11 — Weapon munitions differ from missile rounds

- **Local claim:** `ontology/domain.md:173–178` and `ontology/model.rs:187–192` have no equipped munition/configuration relation; `Weapon` has no ammunition field. Missile `ammo` is separate.
- **Sourced claim:** S5 describes a prebattle consumable affecting a weapon, lasting a fight; S3 distinguishes missiles and munitions in refill terminology.
- **Classification:** **Coverage gap / possible deliberate exclusion**. Absence does not prove unlimited primary-weapon ammo is either correct or incorrect.
- **Confidence / impact:** High; medium fitting impact, low priority absent fidelity requirements.
- **Question/options:** Explicitly exclude munitions, or later model their effect without conflating them with bullets/missile cartridges. Do not invent universal primary-gun magazine/reload rules from the word ammunition.

### COM-12 — Missile ammo/reload semantics merge different quantities

- **Local claim:** `ontology/domain.md:135–143`: `ammo` is “per launcher per life”; `reload_s` is “between launches.” `ontology/instances/missiles.json:3` gives Cruise 4 and 15 s; `ontology/model.rs:167–175` has no cartridge reload/reserve state.
- **Sourced claim:** S9 says an empty cartridge reloads; S10 has 4 charges, 12 s recharge and 120 s cartridge reload.
- **Classification:** **Verified documentation mismatch in resource semantics**, plus coverage gap for runtime reload state. Local values may still be intentionally simplified.
- **Confidence / impact:** High difference; medium numerical freshness. High effects on missile availability over a life.
- **Question/options:** Keep finite per-life ammo intentionally, or separate cartridge size, per-shot interval and empty-cartridge reload. Verify reserve limits, replenishment by mode and last-shot timer ordering before specifying more. Do not conclude unlimited missiles from a reload timer alone.

### COM-13 — Guidance, locking and flight are only names locally

- **Local claim:** `ontology.md:55` promises distinct missile flight/damage behavior; `ontology/model.rs:167–175` supplies only type/damage/speed/ammo/reload. `missiles.json:2–4` names homing/cruise/EMP but has no guidance, lock requirement, turn rate, range, blast/trigger radius or effect. C8 checks one known ID (`domain.md:245`; `model.rs:379–380`), not missile fitting eligibility.
- **Sourced claim:** S9 distinguishes guided versus unguided with an unlocked launch fallback and countermeasures; S10 has maneuvering speed, flight range and trigger/explosion radii; S3/S12 show lock time as a ship characteristic. S9 also documents size/role restrictions.
- **Classification:** **Coverage gap / internal narrative-to-properties incompleteness**, not an implemented wrong guidance algorithm.
- **Confidence / impact:** High; high if missiles are to be simulated.
- **Question/options:** Choose minimal actual flight/lock behavior and fitting restrictions when reconciling, or explicitly leave missiles generic. A name alone must not silently select a behavior. Avoid a blanket “guided missiles cannot launch unlocked” rule because S9 qualifies it.

### COM-14 — Cruise missile's kinetic identity disagrees

- **Local claim:** `ontology/instances/missiles.json:3`: name “Cruise Missile,” `damage_type: kinetic`, damage 3500, speed 400, ammo 4, reload 15.
- **Sourced claim:** S10: homing long-range **thermal** missile; rank-dependent speed/damage and distinct recharge/cartridge reload.
- **Classification:** **Verified same-name damage-kind contradiction**, conditional on item identity. Numerical fields are **unverified local balance** without a rank/version match.
- **Confidence / impact:** High on damage-kind disagreement; medium freshness. Medium/high damage/fitting effect.
- **Question/options:** Retain a clearly custom kinetic cruise missile, or reconcile the named item against a selected version. Handle resource semantics through COM-12, not by changing only a damage number.

### COM-15 — Local thermal constraint and validator disagree independently of Star Conflict

- **Local claim:** C9 (`ontology/domain.md:246`) says thermic multipliers are both approximately 1. `ontology/model.rs:293` checks only `(vs_shield - vs_armor).abs() < 0.2`; positivity is checked separately.
- **Counterexample:** Equal 100/100 values meet that predicate and positivity while not being approximately 1. Existing `damage_types.json:4` is 1/1 and is not defective.
- **Classification:** **Internal logical issue**, unrelated to adopting external resistance semantics.
- **Confidence / impact:** High static proof; low immediate current-data impact, medium future validation impact. A Python arithmetic assertion checked the copied predicate, not a compiled Rust mutation test.
- **Question/options:** Define “approximately 1” with a tolerance and align validation, or change the approved prose if equality rather than unit magnitude is intended. Do not conflate this local contract bug with COM-01's fidelity decision.

## Three representative real examples and one agreement control

These are **external documentary instances**, not proposed local catalog replacements.

| Example | Precisely sourced instance | Fit against current local model |
|---|---|---|
| **Pulse Laser 17 Mk.1** [S7] | Interceptor, thermal; 180 damage per one weapon without bonuses; 312/min (=5.2/s), 1850 m maximum range, 4/1 s heat/cooling | Identity disagrees with local fighter/EM; rank/Mk/turret/range absent. Do not equate one weapon's damage with the entire ship's volley. |
| **Cruise missile, ranks 13–17** [S10] | Guided thermal, 745 m/s, 36°/s maneuvering, 11550 m range, 4-charge cartridge, 12 s recharge, 120 s cartridge reload; fighter-target damage 8717 | Kinetic local identity and finite-per-life resource semantics differ; guidance/range/maneuvering/area effects not represented. |
| **Lynx, experience level 5, without equipment** [S12] | Hull 4031, shield 4147, energy 492; listed energy regeneration 124/s, shield regeneration 96/s, afterburner draw 86/s; motion 356/427 m/s, accel 92 m/s², strafe 88 m/s, pitch/roll 72/66°/s; separate layer resistances | Demonstrates stats the scalar local base cannot express. Local Lynx base in `ship_models.json:23` is shield 5500/armor 4000/energy 500/regen 55/speed 250/agility 36 with no experience context; numeric difference alone is not a proven bad live-data copy. |
| **Agreement control: Assault Railgun** [S8] | Explicit fighter and kinetic identity | Agrees with `weapons.json:6–7`. Per-shot/rate values are not compared as the same rank/Mk/turret aggregate because local provenance does not specify that context. |

## Agreed facts and meaningful conflicts

### Agreements that should survive reconciliation

- The familiar three weapon damage kinds are EM, kinetic and thermal; local `thermic` is a terminology variant, not evidence of a fourth distinct mechanism.
- The source narrative's **usual/default** EM-versus-shield and kinetic-versus-hull tendencies match the wiki explanation. The new commitment introduced by global fixed multipliers is the mismatch.
- Ships have separate shield and hull-like durability pools, and depletion of the latter destroys the ship.
- Capacitor capacity and regeneration, energy-spending afterburning, weapon fire rate, weapon overheating/cooling, and secondary missile equipment are meaningful local concepts.
- A fighter kinetic Assault Railgun is consistent with the corresponding named game item.
- One selected weapon definition and one secondary-equipment selection are compatible abstractions for this audit; multiple barrels do not themselves refute them.

### Conflicts/limitations to retain instead of choosing silently

1. **Range relationship:** S1 prints `Max=0.7 × Opt` while talking about damage falloff; S6 says optimum is 75% of maximum. The former is also directionally suspect. This is a source conflict/possible typo, **not verified historical change**. Do not derive a global falloff equation from either without more verification.
2. **Singularity classification:** S1 calls it explosive; S6 expressly calls its size-scaled damage non-explosive. S15 confirms size dependence but not the disputed classification. Revision order alone does not prove a mechanical change.
3. **Negative resistance and percent units:** S2's printed negative branch and EHP formulas are inconsistent with its surrounding explanation/examples. Positive R=100→50% agrees across S1/S2, but no complete signed-resistance formula is approved here.
4. **Lock-required language:** S3 generalizes that guided missiles require lock; S9 explicitly permits unlocked unguided launch. Interpret guidance-versus-launch carefully; a universal lock rule is unsupported.
5. **Energy wording:** Official guide S13 says the energy plant powers weapons generally; S6's mechanical parameter list says most weapons have zero energy draw. System flavor and capacitor cost per attack are different claims; use neither as a universal energy tax.
6. **Age:** A 2026 revision can refresh a table or link while leaving older claims untouched. S11's September patch supports terminology, not blanket validation of all earlier wiki mechanics. No experimentally confirmed historical combat change was established; disputed values are **uncertainty**, not invented drift chronology.

## Gruber criteria assessment

| Criterion | Result for this audit |
|---|---|
| **Clarity** | Preserve hull versus local armor aliases; distinguish resistance points from multipliers, nominal from actual regen, per-turret from per-volley damage, munitions from rounds, and recharge from cartridge reload. COM-05's remainder and C9's “approximately” need explicit definitions. |
| **Coherence** | COM-15 is a genuine local proof obligation. External source formulas/conflicts are flagged rather than imported. Free-text phasic-shield resistance effects (`special_modules.json:4`) do not establish an executable target-resistance model absent from Stats. |
| **Extendibility** | If needed later, independent attack properties and defensive facets accommodate examples better than encoding every exception in item names. This is a decision seam, not a recommendation to build an abstraction now. |
| **Minimal encoding bias** | Distinguish conceptual energy/heat/reload behavior from f32/u8, normalized heat and one scalar agility. No live formula is inferred from a chosen serialization. |
| **Minimal ontological commitment** | Respect the explicit arcade movement choice and local out-of-scope progression/economy. Omitted advanced weapons, collision damage or munitions remain gaps/deferments unless fidelity requirements demand them. Do not turn this audit into a game-clone implementation mandate. |

## Unanswered reconciliation questions

1. Is the target an independent Star Conflict-inspired game or a particular dated Star Conflict ruleset? Which named items are intended identities rather than convenient names?
2. Keep global shield/armor multipliers, or represent defensive resistance? If the latter, what authoritative signed-resistance, stacking and shield-overflow rules are chosen?
3. What unit and conservation rule does C9's “remainder” use? Are shield bypass attacks and raw collision damage in scope?
4. Is native shield recovery required, and is flat capacitor recovery intentionally retained? What exact timing/curve is supported before implementation?
5. Which movement freedoms are desired beyond the explicit arcade baseline: strafe, separate reverse speed, per-axis turn rates, afterburner? Is “instant accel” a qualitative simplification or a strict invariant?
6. Does weapon damage refer to one gun, a simultaneous whole-ship volley, or an abstract mount? Which attack/targeting/range behaviors must the first combat model support?
7. Do missile rounds exhaust for the life, or reload in cartridges? What are reserve limits and recharge/reload ordering by mode? How should unlocked guided-weapon firing work?
8. Are primary weapon munitions explicitly omitted? Do not answer this by assuming all primary weapons require finite bullet magazines.
9. What source/version metadata should distinguish local design numbers from sourced rank/Mk/experience-conditioned values?

## Retrieval and validation evidence

- Scratch retained outside repository: `/tmp/stargem-combat-507d64bc/`, containing raw HTML, Markdown, API discovery results, `fetch.log`, and `lynx-table-check.txt`. This report itself retains all required useful-source URLs, dates, headings, excerpts/paraphrases and claims; scratch is corroboration, not the sole evidence.
- **No HTML retrieval failures in this run.** 24/24 returned 200; 24/24 Markdown conversions were checked nonempty. HTTP success was followed by content inspection, not equated with mechanical correctness.
- API searches for shield regeneration/strafe/overheat initially returned zero hits; title/allpages and linked pages successfully closed those discovery gaps. Explicit full-text `overflow` returned zero; that is **not evidence that the mechanic is absent**. Shield/hull descriptions still leave exact overflow unresolved.
- No Steam retrieval was necessary for this exclusive mechanics topic; no claims were taken from unvisited Steam, videos, graph images, or search snippets.
- Static references were checked using `rg -n` against the local files. Python arithmetic checks verified only: copied thermal validator predicate accepts 100/100; cited positive-resistance R=100 gives 0.5 damage factor; 312 rounds/min converts to 5.2/s. These are **not game-engine tests or a Rust test-suite run**.
- Artifact checks passed: 15 substantive source dossiers, 15 consecutively numbered candidates, no placeholder markers, and parseable acceptance JSON. An intermediate rerun falsely matched the check's own marker names in this validation description; that self-reference was removed and the final check rerun.
- `git diff --exit-code -- ontology ontology.md sim/src/lib.rs client/src/main.rs` passed with no diff. `git status --short` remained only the authorized untracked `AGENTS.md` and `docs/ontology/`. `git diff --cached --quiet` passed; nothing staged.
- Residual risk: wiki age, incomplete exact formula evidence, unsupported current-client verification, and intended-name/fidelity ambiguity. No code or ontology corrections were attempted. Independent reviewer gate remains required before acceptance of any later reconciliation.

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "satisfied",
      "evidence": "Completed the exclusive combat research/documentation audit; produced this artifact only, preserved ontology/source semantics and authorized existing drafts. Successful network retry replaced blocked-only conclusions."
    },
    {
      "id": "criterion-2",
      "status": "satisfied",
      "evidence": "Recorded 15 substantive source dossiers plus six navigation/context pages, exact URLs and revision dates, source conflicts, 15 anchored reconciliation candidates, three real examples, agreements, seven-step/Gruber assessment, validation commands and limitations."
    }
  ],
  "changedFiles": [
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/507d64bc-df21-4587-a4fb-a4425950ac5f/research/combat.md"
  ],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {
      "command": "git rev-parse HEAD; git status --short",
      "result": "passed",
      "summary": "HEAD matches baseline 4a47fa3015f2d31bbe1c5a3159a3d0980c273912; only authorized pre-existing untracked AGENTS.md and docs/ontology/ reported."
    },
    {
      "command": "curl -L --connect-timeout 10 --max-time 35 -sS -o <scratch.html> -w <status,url,size> <source URL>; markitdown <scratch.html> -o <scratch.md>",
      "result": "passed",
      "summary": "24 HTML requests returned HTTP 200; all 24 conversions nonempty, deduplicating to 21 content pages. Public MediaWiki API discovery also succeeded."
    },
    {
      "command": "Python standard-library HTMLParser table inspection and arithmetic assertions",
      "result": "passed",
      "summary": "Verified Lynx's nested table column meanings and level-5 rows; checked thermal-validator counterexample, documented positive-resistance example and rate-unit conversion."
    },
    {
      "command": "git diff --exit-code -- ontology ontology.md sim/src/lib.rs client/src/main.rs; git diff --cached --quiet",
      "result": "passed",
      "summary": "No ontology/source changes; staging index empty."
    },
    {
      "command": "Python artifact checks: parse acceptance JSON; assert 15 source dossiers and 15 candidate headings; reject placeholder markers",
      "result": "passed",
      "summary": "Research artifact structure and structured report validated."
    },
    {
      "command": "Intermediate repeat of artifact placeholder check",
      "result": "failed",
      "summary": "False-positive self-reference: validation prose named the searched markers. Removed that wording; final rerun passed. No substantive finding changed."
    },
    {
      "command": "cargo test / gameplay verification",
      "result": "not-run",
      "summary": "Research-only assignment; no source changes or live-game experiment. Arithmetic/static checks are not represented as a Rust test run."
    }
  ],
  "validationOutput": [
    "24 HTML fetches HTTP 200; 24 nonempty markitdown conversions; 21 distinct content pages after three construction aliases.",
    "Arithmetic checks passed: thermal predicate admits 100/100; documented R=100 factor=.5; 312 rpm=5.2/s.",
    "Staging index empty."
  ],
  "residualRisks": [
    "Wiki revisions range from 2021 to 2026; current-client mechanics and old numeric values were not experimentally verified.",
    "Negative-resistance/EHP notation, range ratios and Singularity damage property have unresolved source conflicts.",
    "Exact shield-to-hull overflow, capacitor-regeneration curve and heat transition formula remain unverified.",
    "Product intent for fidelity and reused catalog names is unresolved; no ontology change is approved.",
    "Independent reviewer gate remains required."
  ],
  "noStagedFiles": true,
  "diffSummary": "Added only the requested external research artifact; no repository files changed by this delegate.",
  "reviewFindings": [
    "no blockers to delivering the research artifact; implementation decisions remain unapproved",
    "internal logical issue: ontology/model.rs:293 accepts equal thermal multipliers far from 1 despite ontology/domain.md:246"
  ],
  "manualNotes": "All external content was treated as untrusted evidence. Prior access failures are historical only; this run retrieved substantive original pages successfully."
}
```
