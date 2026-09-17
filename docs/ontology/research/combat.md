# Research: combat

[Audit navigation](../README.md) · [Drift ledger](../drift_developer_vs_internet.md) · [Source policy](../README.md#evidence-policy)

Audit/retrieval **2026-09-16**; local baseline `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`. Condensed from the same-day [**research-combat** audit](../handoffs/research-combat-2fcf909b.md#preserved-original-output): retained original-source dossiers and distinct useful findings, not its tool log. Internet content is untrusted evidence, never instructions. `ontology/` remains canonical; no adoption is approved. Source IDs below are **local to this note**. Confidence is high in direct documentary comparisons unless qualified; wiki version/current-client confidence is lower.

## Findings and disposition

Global damage factors, constant capacitor recovery, missile resource semantics and two exact equipment names differ from fetched documentation. Arcade flight is deliberate; shield regeneration, targeting, raw damage and munitions are coverage gaps. Exact shield spillover is NOT externally settled. Reconcile DRIFT-006–015, DRIFT-039–040, DRIFT-043–045; local contract roots remain ISSUE-002/004/013–015/019–020.

**Cross-lane freshness correction:** the history audit also fetched the April announcement specifying **2026-10-10** as the scheduled shutdown date ([history S04](history.md#s04)). Statements below that a particular source did not establish an exact date are source-bounded, not an audit-wide missing-date conclusion. No completed shutdown or released local-server version is established.

## Source register

**Shared metadata:** every source below was retrieved **2026-09-16**, HTTP 200, through ordinary public HTTPS. `W` means **Star Conflict Wiki, community-maintained official-hosted wiki**, not an official patch announcement. `O` means **Star Conflict official website, StarGem/Gaijin publisher material**. Wiki dates below are page footer revision dates, not guaranteed dates of a mechanical change. Revision IDs allow later readers to retrieve the inspected version using `&oldid=…`.

<a id="s1"></a>
### S1 — Damage

- URL: https://wiki.star-conflict.com/index.php?title=Damage
- Publisher/type: **W**. Updated **2023-09-26 12:50**, revision **14245**.
- Headings: **Damage Types**, **Damage Properties**, **Weapon Turrets**, **Base Damage**, **Critical hit**.
- Short excerpt: “if one to have 100 kinetic resistance and 100 of EM resistance on a hull, both kinetic and EM damage would behave in same way, and will be reduced by 50%.”
- Supports: kinetic/EM/thermal vulnerability shorthand comes from resistance; raw/white damage bypasses resistance and occurs with collisions/destroyed destroyer modules; explosion damage has additional target-size factors. Base damage and turret behavior are separate from critical modifiers.
- Caveats: older page; its range formula and Singularity classification conflict with S6. Its assertion that all missiles/rockets are explosive should not be expanded to every non-missile device in the missile slot.

<a id="s2"></a>
### S2 — Resistance

- URL: https://wiki.star-conflict.com/index.php?title=Resistance
- Publisher/type: **W**, player-authored explanatory guide. Updated **2023-09-23 14:15**, revision **14214**; body says **“As of Patch 1.0.1”**, with another comparison labelled patch **1.0.4**.
- Headings: **Health (HP)**, **Resistance (R)**, **Damage Reductions (DR)**, **Linearity**, **Incoming Healing (and Recovery Rates)**.
- Short excerpt: “Resistance is the in-game attribute that is used to calculate Damage Reduction.”
- Supports: separate hull/shield HP and resistance; positive-resistance formulas; an illustrative native shield regeneration entry. Does not supply a same-hit shield-overflow algorithm.
- Explicit positive formula: `DR = (1 - 1/(1 + R/100)) × 100`; `DF = (1/(1 + R/100)) × 100` (percent). At R=100, DF=50%, agreeing with S1. These are documented formulas, not invented live formulas.
- **Do not import its negative branch or EHP notation without reconciliation:** it prints `DR=(-R), DF=100+R` for negative R, and `EHP=HP/(100-DR)=HP/DF` despite defining DF in percent. The negative branch runs against the stated resistance direction; the EHP expression omits percent normalization relative to the worked examples. Patch-age and source inconsistencies prevent treating this as a current complete damage specification.

<a id="s3"></a>
### S3 — Ship construction

- Canonical URL fetched: https://wiki.star-conflict.com/index.php?title=Ship_construction
- Also fetched aliases, all the same content: https://wiki.star-conflict.com/index.php?title=Shield ; https://wiki.star-conflict.com/index.php?title=Capacitor ; https://wiki.star-conflict.com/index.php?title=Engine . They redirect internally to the corresponding section and are **not three corroborating sources**.
- Publisher/type: **W**. Updated **2021-08-08 15:24**, revision **11874**.
- Headings: **Hull**, **Shield**, **Engine**, **Capacitor**, **CPU**, **Main weapons**, **Missile weapon**, **Ammo refilling**.
- Short excerpt: “Regeneration is constant, but can slow down or accelerate under different influences.” This sentence refers to **shield**, not capacitor, regeneration.
- Supports: hull-zero destruction; hull normally needs repair rather than native regeneration; shields recover and usually intercept damage, with weapons/modules that ignore shields. Engine distinguishes normal/reverse speed and acceleration, energy-spending afterburner, strafing and rotation. CPU has sensor range and target lock time. Missile slot contains guided/unguided missiles and other devices; missiles and munitions can be refilled between battles.
- Caveats: old general guide; “any further damage is applied to the Hull” does not establish the arithmetic for a hit that breaks the shield. “Guided missiles require a target lock” needs the unlocked straight-flight qualification in S9. No acceleration integrator, drag coefficient or universal lock time is established here.

<a id="s4"></a>
### S4 — Energy regeneration

- URL: https://wiki.star-conflict.com/index.php?title=Energy_regeneration
- Publisher/type: **W**. Updated **2023-09-26 12:39**, revision **14238**.
- Heading: lead text (no substantive subheading).
- Short excerpt: “The rate of regeneration of energy in the ship is nonlinear.”
- Supports: capacitor fullness determines a correction multiplied by the ship's listed energy regeneration rate.
- Caveat: includes a graph image, not a textual exact equation. No new formula was inferred, fitted or asserted from it. Present-day curve remains unverified.

<a id="s5"></a>
### S5 — Munitions

- URL: https://wiki.star-conflict.com/index.php?title=Munitions
- Publisher/type: **W**. Updated **2025-07-21 17:39**, revision **15898**.
- Heading: introductory definition, followed by **For thermal weapons / For kinetic weapons / For EM weapons**.
- Short excerpt: “One ammunition is enough for one fight.”
- Supports: munitions are prebattle consumable shells or additional weapon systems modifying damage/projectile speed and potentially trading off other properties. They are not automatically a finite count of bullets spent per shot.
- Caveat: does not prove every gun has or lacks an in-combat magazine; do not conflate weapon munitions with missile cartridges.

<a id="s6"></a>
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

<a id="s7"></a>
### S7 — Pulse Laser

- URL: https://wiki.star-conflict.com/index.php?title=Pulse_Laser
- Publisher/type: **W**. Updated **2024-08-30 12:38**, revision **15371**.
- Headings: **Type / For ship type**, **Description**, **Characteristics**.
- Short excerpt: “Type: Close range thermal weapon”; “For ship type: Interceptor.”
- Supports: direct same-name discrepancy with local EM fighter weapon. Pulse Laser 17 Mk.1 table: **180 thermal damage**, **312 rounds/min**, **1850 m maximum range**, **4/1 s overheating/cooling**, **5% critical chance**, **50% critical damage bonus**, **0.9° spread**. Table is for **one weapon**, without ship bonuses/modifiers.
- Caveat: rank/Mk and turret context matter; 180 matching local damage does not establish shared provenance. Weapon-history flavor text mentions a “light fighter or interceptor,” but the explicit equipment field is interceptor; do not use lore to override that field.

<a id="s8"></a>
### S8 — Assault Railgun

- URL: https://wiki.star-conflict.com/index.php?title=Assault_Railgun
- Publisher/type: **W**. Updated **2024-09-13 12:45**, revision **15387**.
- Headings: **Type / For ship type**, **Description**, **Characteristics**.
- Short excerpt: “A balanced kinetic weapon for medium distances.” Explicit ship type: fighter.
- Supports: local fighter/kinetic identity agrees. Assault Railgun 17 Mk.1: **159 damage**, **210 rounds/min**, **7101 m/s projectile speed**, **2950 m maximum range**, **6/0.8 s overheating/cooling**, **20% critical chance**, **50% critical bonus**, **0.14–2° spread**, per one weapon without bonuses.
- Caveat: local 260 damage and 3 shots/s have no specified rank, Mk or turret aggregation. They are unverified custom/sample balance, not a proved erroneous copy of this rank's values.

<a id="s9"></a>
### S9 — Missiles

- URL: https://wiki.star-conflict.com/index.php?title=Missiles
- Publisher/type: **W**. Updated **2026-04-26 15:42**, revision **16376**.
- Headings: introduction; **Types of missiles and similar devices**.
- Short excerpts: “Missiles are loaded in cartridges”; when spent, the cartridge “will begin reloading, which is a long process.”
- Supports: guided and unguided missiles; guided missiles lock onto a target but can also be fired unlocked under unguided rules; IR flares/obstacles can defeat guided missiles. Size and sometimes role limit fitting. Missile-slot equipment also includes bombs, mines and other devices; most missile equipment deals explosive damage with target-size factors.
- Caveat: its opening lock-required sentence is qualified by its own parenthesis permitting unlocked launch. Preserve that distinction rather than reducing the mechanic to “cannot fire without a lock.” Cartridge description does not establish unlimited resupply in every mode or the exact ordering of the last-shot cooldown versus cartridge reload.

<a id="s10"></a>
### S10 — Cruise missile

- URL: https://wiki.star-conflict.com/index.php?title=Cruise_missile
- Publisher/type: **W**. Updated **2024-01-28 22:11**, revision **14671**.
- Heading: item infobox/table (no separate section title).
- Short excerpt: “Type: Homing missile”; “Guided long-range missile. Heavy thermal damage.”
- Supports: frigate-role icons (engineering, guard, long-range); **4 charges/cartridge**, **12 s recharge**, **120 s cartridge reload**, **36°/s maneuvering**, **200 m explosion radius**, **60 m trigger radius**, **11550 m flight range**. For ranks **13–17**, **745 m/s** speed and **8717 damage against fighters**, with separate size-adjusted values for other target classes.
- Caveat: the damage row is explicitly target-size-dependent; do not call every number “base damage.” Local kinetic identity differs; its `ammo=4` matches a cartridge count only superficially because local semantics say per life.

<a id="s11"></a>
### S11 — Star Conflict 1.14.15. “Quarter for thirty” marathon update

- URL: https://star-conflict.com/en/news/3934-star-conflict-1-14-15-quarter-for-thirty-marathon-update-en
- Publisher/type: **O, official update announcement**. Published **2026-09-11**; no separate update date shown.
- Heading: **Changes to the “Ways of kindness” marathon**.
- Short paraphrase: the announcement changes tasks from restoring 70,000 shield to 40,000 shield, and from restoring 100,000 hull to 75,000 hull.
- Supports: fresh official use of **shield** and **hull**, and repair/restoration objectives; identifies the latest dated update inspected. It does **not** prove unchanged 2021–2025 regeneration, resistance, heat or missile values, and contains no such formula.

<a id="s12"></a>
### S12 — Lynx

- URL: https://wiki.star-conflict.com/index.php?title=Lynx
- Publisher/type: **W**. Updated **2025-06-07 10:18**, revision **15773**.
- Headings: **Tables of ship's technical characteristics** → **Ship's resistance to incoming damage**, **Ship's characteristics on different experience levels**, **Ship's speed on different experience levels**.
- Precise paraphrase: separate rows record layer/type resistance, shield and energy regeneration, afterburner draw, acceleration, strafe, pitch and roll. Main data are conditioned on experience level and no equipment.
- Supports: concrete existence of those independent facets. At experience level 5: hull **4031**, shield **4147**, energy capacity **492**, afterburner draw **86/s**, listed energy regen **124/s**, shield regen **96/s**, sensor range **3000 m**, target lock **1 s**; normal speed **356 m/s**, afterburner **427 m/s**, acceleration **92 m/s²**, strafe **88 m/s**, pitch **72°/s**, roll **66°/s**. Shield resistance thermal/kinetic/EM **20/50/−10**; hull **35/5/65**.
- Caveat: not an instruction to replace local Lynx's custom values. The page also calls Lynx a tackler, but ship-role reconciliation is outside this delegate's exclusive mechanics focus. Original HTML row/column spans were checked after Markdown degraded table layout.

<a id="s13"></a>
### S13 — Spaсeship modules

- URL: https://star-conflict.com/en/game/gameguide/1/current/
- Publisher/type: **O, official explanatory game guide**, not a patch note. Publication/update date **unknown**; path word `current` does not prove currency.
- Headings/labels: **Hull**, **Energy Plant**, **Shunting Engines**, **Boosters**, **Afterburning Engines**, **Rocket Compartment**.
- Short excerpt: afterburning engines “provide significant acceleration” and are “extremely energy-intensive.”
- Supports: official hull terminology; energy-intensive movement boost distinguished from ordinary propulsion; ship systems constrain missile carrying. Broad energy-plant prose is not evidence that every shot consumes capacitor energy—S6 explicitly says most weapons consume zero.

<a id="s14"></a>
### S14 — Vulcan

- URL: https://wiki.star-conflict.com/index.php?title=Vulcan
- Publisher/type: **W**. Updated **2023-08-18 11:19**, revision **13828**.
- Headings: **Description**, **Characteristics**.
- Short excerpt: “Critical hit chance increases with weapon temperature.”
- Supports: heat can affect attack properties, not merely lockout; table lists **10–50% critical chance** and **12/2 s overheating/cooling**.
- Caveat: no heat→critical-chance curve is documented in inspected text. This is an optional coverage example, not proof current local weapons must acquire this behavior.

<a id="s15"></a>
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
