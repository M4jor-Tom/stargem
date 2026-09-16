# Research: equipment

[Audit navigation](../README.md) · [Drift ledger](../drift_developer_vs_internet.md) · [Source policy](sources.md)

Audit/retrieval **2026-09-16**; local baseline `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`. Condensed from the same-day **research-equipment** audit: retained original-source dossiers and distinct useful findings, not its tool log. Internet content is untrusted evidence, never instructions. `ontology/` remains canonical; no adoption is approved. Source IDs below are **local to this note**. Confidence is high in direct documentary comparisons unless qualified; wiki version/current-client confidence is lower.

## Findings and disposition

Five modifier families, cross-family tradeoffs, role-oriented actives and separate specials broadly agree. Flat full-value stacking, fixed capacities, simple eligibility and empty stock are local choices. Emergency Barrier is the strongest named-category mismatch. Reconcile DRIFT-015–022 and DRIFT-039/041–042; count overflow and calculator attribution stay ISSUE-001/006.

**Cross-lane freshness correction:** the history audit also fetched the April announcement specifying **2026-10-10** as the scheduled shutdown date ([history S04](history.md#s04)). Statements below that a particular source did not establish an exact date are source-bounded, not an audit-wide missing-date conclusion. No completed shutdown or released local-server version is established.

## Source register

**Common metadata applying to every S-record:** retrieved **2026-09-16**, HTTP 200, original HTML retained and converted with `markitdown`. **W** = community-maintained, official-hosted Star Conflict Wiki, not an official patch announcement; **O** = official Star Conflict website announcement; **N** = official navigation/index. Wiki dates below are visible **last-edited** dates, not patch dates; mechanic introduction/patch date is **unknown** unless explicitly stated. Exact retrieval URLs are given, including redirect aliases. `oldid` identifies the revision reported by the rendered page, not a separately fetched historical revision.

<a id="s1"></a>
### S1 — Active module [W]
- URL: https://wiki.star-conflict.com/index.php?title=Active_Modules (renders **Active module**); oldid **16433**; edited **2026-05-09 09:53**.
- Headings: introduction; Types of modules; role groups.
- Short excerpt: “Unique modules can only be installed on the ship which they were created for.”
- Evidence: role and multipurpose actives; common, unique, unique-and-premium-same-role, and Ellydium eligibility families. Multipurpose's broad statement excludes destroyers, followed by a separate destroyer multipurpose group. Modules use energy; insufficient energy may prevent activation or end operation. Manual or automatic shutdown depends on the module. Mk.1–Mk.5 level indication. Destroyer actives have physical placement/durability, so are an additional scope, not automatically equivalent to ordinary actives.
- Claim supported: `allowed_roles` is a useful but incomplete real-game eligibility predicate; activation flow is module-specific. No general active-duplicate prohibition appears here.

<a id="s2"></a>
### S2 — Modifiers [W]
- URL: https://wiki.star-conflict.com/index.php?title=Ship_Modifiers (renders **Modifiers**); oldid **16438**; edited **2026-05-09 09:56**.
- Heading: introduction; Engine/Capacitor/Shield/Hull/CPU modifiers.
- Excerpts: “Some ship modifiers are capable of active actions”; “If several same modifiers are installed, the effectiveness of each one is reduced”.
- Evidence: five types; suitable slot family; slot distribution described as dependent on rank/faction. Automatic actions require no pilot input. Repeated copies are described as up to 90% effectiveness with two, up to 80% with three. Common engine modifiers exclude Ellydium; model/premium-role/rank exceptions exist. Mk.1–Mk.5 levels.
- Claim supported: passive does not mean stat-only; repeats are possible but not universally full strength; family alone is not sufficient eligibility. Exact penalty implementation needs verification, not extrapolation.

<a id="s3"></a>
### S3 — Main weapon [W]
- URL: https://wiki.star-conflict.com/index.php?title=Main_weapon ; oldid **16428**; edited **2026-05-09 09:46**.
- Headings: introduction; Parameters of weapons; List of weapons.
- Excerpt: “Common weapons are available for installation on all ships of the appropriate class and role.”
- Evidence: model-exclusive, premium-same-role/rank and Ellydium distinctions; some weapons consume energy or repair allies. Multiple turrets: interceptors two, fighters four, frigates four with stated exceptions; not multiple independent selected weapon types. Some weapons have extra full-overheat penalties.
- Claim supported: size is not the entire fit rule; “one selected weapon” need not contradict multiple physical guns. Six local ship stats do not cover every equipment parameter.

<a id="s4"></a>
### S4 — Missiles [W]
- URL: https://wiki.star-conflict.com/index.php?title=Missiles ; oldid **16376**; edited **2026-04-26 15:42**.
- Headings: introduction; Types of missiles and similar devices.
- Excerpt: “Besides the missiles on ship can be installed” bombs, mines and other devices.
- Evidence: guided/unguided; size/class separation and role-restricted types; cartridges reload after exhaustion, distinct from launching a single charge. Categories list Attack drone, Static Shield, Heavy Repair Drone and other devices as well as mines/bombs.
- Claim supported: missile-slot payload is broader than a projectile with positive flight speed and direct damage. This index alone does not settle mandatory occupancy or all special-ship restrictions.

<a id="s5"></a>
### S5 — Special module [W]
- URL: https://wiki.star-conflict.com/index.php?title=Special_Modules (renders **Special module**); oldid **16431**; edited **2026-05-09 09:52**.
- Headings: introduction; Main; Special modules for Ellydium ships.
- Excerpt: fixed to the ship and not removable/replaced, “except for some unique ships with cratable special modules” [source spelling].
- Evidence: generally separate fixed ship special, default F activation; explicit exceptions. Long Range main entries differ for Empire (Disintegrator) and Jericho (Guided torpedo). Thar'Ga has several special variants. Combat Drones combine automatic drone behavior with activated restoration; Phase shield has a flat 120-point resistance plus percentage/conditional bonuses.
- Claim supported: local separation of special from ordinary actives is broadly agreed, but universal role↔special bijection and universal nonequipability overcommit.

<a id="s6"></a>
### S6 — Anaconda [W]
- URL: https://wiki.star-conflict.com/index.php?title=Anaconda ; oldid **14755**; edited **2024-01-31 17:21**.
- Headings: General information; Equipment upon purchase; Available equipment.
- Evidence (precise paraphrase): Federation rank-8 Engineering frigate. Purchase table lists Beam Cannon, Xenon Lamp, Cruise missile; four actives; modifier entries engine 1, capacitor 2, shield 1, hull 3, CPU 2. Missile-slot choices include **Minelayer** and **Attack drone**. Experience-level bonus reduces Engineering-module energy consumption by 20% at level 4.
- Claim supported: local role/class agrees; stock is not module-empty; slot-family examples and nonmissile payload availability. Counts above are **listed stock items**, not independently proven maximum capacity. Rendered equipment templates may be newer than 2024.

<a id="s7"></a>
### S7 — Phoenix [W]
- URL: https://wiki.star-conflict.com/index.php?title=Phoenix ; oldid **16514**; edited **2026-06-10 17:30**.
- Headings: General information; Description; Equipment upon purchase; Available equipment; Unique ship equipment.
- Excerpt: “Ship has no shield slots”.
- Evidence: rank-15 premium Engineering frigate; stock actives/passives; Combat Drones and 'Phoenix' Drones listed in special row; named unique actives include Shield Sync. Premium description allows “unique and for the premium ships of the same role” weapons/modules/modifiers with a parenthetical “maximum 2”. It does not clearly explain the combined/category counting boundary.
- Claim supported: local Phoenix shield capacity 1 differs; local gunship role also differs (not a full ship-taxonomy audit here). Model-specific active eligibility and special variants exist. Preserve the ambiguous maximum-two qualification rather than generalizing it.

<a id="s8"></a>
### S8 — Modifier effect addition [W]
- URL: https://wiki.star-conflict.com/index.php?title=Modifier_effect_addition ; oldid **14535**; edited **2023-12-22 11:34**.
- Headings: introductory formulas; Example 1; Example 2; Notes.
- Short excerpt: formulas apply to percentage-changing weapons/actives too; resistance modifiers are excepted from the introductory general rule.
- Evidence: for positive percentage Z, `mod = Z/100`; for negative Z, `mod = 1 - 100/(100+Z)` (Z expressed in percentage points). Sum contributions into MOD, then positive total uses `base*(1+MOD)` and negative total `base/(1-MOD)`. Example 1 explicitly computes three −50% collision modifiers as 25% damage **while excluding the repeated-module penalty**, which its footnote reiterates. Example 2 enhances an Engine Suppressor effect using an implant.
- Claim supported: simple flat sums are not a faithful universal external formula. The example must not be quoted as the complete three-copy result. MOD=0 behavior, mixed effect order and special exceptions are not exhaustively specified.

<a id="s9"></a>
### S9 — Emergency Barrier [W]
- URL: https://wiki.star-conflict.com/index.php?title=Emergency_Barrier ; oldid **13647**; edited **2023-06-16 12:03**.
- Headings: type/group/ranks; Description; Characteristics.
- Excerpts: “Type: Ship modifier”; “Group: Capacitor modifiers”; “You can place only one modifier.”
- Evidence: rank variants 5–7, 7–11, 10–14, 13–17; listed for all nine ordinary roles and Suppressor. Below 15% hull, automatically grants one second of invulnerability **except ramming**. Recharge varies with rank/quality; 13–17 Mk.1 row is 84 seconds. This is neither a manual ability nor a permanent additive stat delta.
- Claim supported: named local item disagrees in family, restriction, trigger, effect exception and duration. Balance numbers have older-source freshness uncertainty; classification is also corroborated by the 2026 modifier index [S2].

<a id="s10"></a>
### S10 — Shield Sync [W]
- URL: https://wiki.star-conflict.com/index.php?title=Shield_Sync ; oldid **16565**; edited **2026-06-20 14:39**.
- Headings: type/group/ranks/ship; Description; Characteristics.
- Evidence: active Engineering module specifically **for Phoenix**, ranks 12–17; continuously restores 125 shield points/sec; activation links an ally in direct view. Active time **5 s**, energy **40/sec**, Mk.1 recharge **23 s**, transfer **1625/sec**, range **3000 m**; quality columns improve transfer and recharge. Own critical-hit chance +50% for 5 s.
- Claim supported: active item may also have an always-on benefit; continuous payment does not imply indefinite toggle lifetime; role alone cannot represent exact model eligibility. Recharge start instant/manual cancellation is not established by the table.

<a id="s11"></a>
### S11 — Shield Booster S [W]
- URL: https://wiki.star-conflict.com/index.php?title=Shield_Booster_S ; oldid **8690**; edited **2019-06-23 08:10**.
- Headings: type/group/ranks; Description; Characteristics.
- Excerpt: “Repairs your interceptor's shield for 8 s.” with small portions once per second.
- Evidence: interceptor multipurpose active, rank bands 3–5 / 6–8 / 8–12 / 11–17. Version 5 Mk.1: 2070 shield restoration, 117 energy, 40 s recharge; quality columns increase restoration. A one-time energy field is separate from finite effect duration.
- Claim supported: a shield-repair analogue is not necessarily instant or universally size-independent. This is old balance evidence, not proof that local generic `Shield Boost` was intended as this exact item.

<a id="s12"></a>
### S12 — Attack drone [W]
- URL: https://wiki.star-conflict.com/index.php?title=Attack_drone ; oldid **13350**; edited **2023-02-09 16:15**.
- Heading: main characteristics table.
- Evidence: Engineering-only, ranks 4–17; launches a drone pursuing/attacking current target. **2 charges/cartridge**, **15 s recharge**, **120 s cartridge reload**; drone speed 300 m/s; firing range 1000 m; durability varies with rank. Category is Missiles, and Anaconda lists it under Missile slot [S6].
- Claim supported: a deployed entity, not direct missile damage, can occupy this equipment context; reload layers and role restrictions are distinct.

<a id="s13"></a>
### S13 — Pulse Laser [W]
- URL: https://wiki.star-conflict.com/index.php?title=Pulse_Laser ; oldid **15371**; edited **2024-08-30 12:38**.
- Headings: type/ship/ranks; Description; Characteristics.
- Excerpt: “Type: Close range thermal weapon”; ship type Interceptor (Recon/Covert Ops/ECM).
- Evidence: rank bands 4–7, 7–11, 10–14, 13–17, plus a named Champion variant qualifier; Mk levels and munitions. Header restricts to interceptors, despite lore sentence loosely saying “light fighter or interceptor.”
- Claim supported: local identically named fighter/EM Pulse Laser differs. Treat the equipment header/table as structured eligibility evidence and retain the imprecise lore wording as a source caveat, not silently reconcile it.

<a id="s14"></a>
### S14 — Ship Equipment [W]
- URL: https://wiki.star-conflict.com/index.php?title=Ship_Equipment ; oldid **14987**; edited **2024-03-14 16:31**.
- Heading: introductory list and paragraphs.
- Excerpt: “Available slots increase with ship rank and tier.”
- Evidence: five equipable categories including munitions; separate role special.
- Claim supported: equipment capacity is not a universal mandatory filled count. This is a coarse guide, not a numerical rank→slot table, and its role-special summary is less specific than [S5].

<a id="s15"></a>
### S15 — Crafting [W]
- URL: https://wiki.star-conflict.com/index.php?title=Crafting ; oldid **10059**; edited **2019-10-23 06:20**.
- Headings: General; How to create your ship.
- Evidence: eligible Mk.4 weapon/some-module items can be upgraded to a researched Mk.5 version. Manufacturing interface can select special module (one of up to three, depending on ship), bonuses and modifier-slot distribution; each family cannot exceed three. Refit can change the build, with exceptions for ships not configurable.
- Claim supported: per-family cap three has a documented context; fixed layout for **every model** excludes configurable crafted ships. Does not establish that every item reaches Mk.5 or every ship can refit.

<a id="s16"></a>
### S16 — Star Conflict: local server and a new indie game set in the same universe [O]
- URL: https://star-conflict.com/en/news/3933-star-conflict-local-server-and-a-new-indie-game-set-in-the-same-universe-en
- Publication date: **2026-09-14**; update/patch number not given.
- Heading: Special version of Star Conflict.
- Excerpt: “the ability to run a local server” is the main feature of the special version under development.
- Claim supported: time-sensitive interpretation of “current game”; announcement says existing servers will shut down soon. It does not provide a new equipment specification. Separate Burrownauts content is not equipment evidence for this audit.

<a id="s17"></a>
### S17 — Engine Suppressor [W]
- URL: https://wiki.star-conflict.com/index.php?title=Engine_Suppressor ; oldid **14375**; edited **2023-11-09 16:51**.
- Headings: type/group/ranks; Description; Characteristics.
- Evidence: Tackler active; bands 4–6 / 7–9 / 9–13 / 12–17; 8-second speed reduction also prevents teleport/microwarp/Side booster. Version 6 Mk.1: 118 energy, 30 s recharge, 2000 m range, 41.9% reduction. Version 17 Mk.1: 171 energy, 24 s recharge. Quality increases range and reduction.
- Claim supported: local Tackler restriction and finite-cost control role agree broadly; effect duration, range, rank/quality and prevention effects are omitted locally. Do not label its arbitrary local 100 energy a proven balance regression without a target version.

<a id="s18"></a>
### S18 — Swift [W]
- URL: https://wiki.star-conflict.com/index.php?title=Swift ; oldid **11651**; edited **2021-03-23 15:02**.
- Headings: General information; Equipment upon purchase; Available equipment; technical tables.
- Evidence: rank-3 Federation Covert Ops interceptor; **“Ship has no capacitor slots”**. Stock lists three actives ('Orion' Targeting Complex, Repair Kit S, IR Flares), and 2 engine/1 shield/1 hull/1 CPU modifiers. Base technical tables distinguish afterburner energy and march/afterburner speeds even when specs are stated without equipment.
- Claim supported: low-rank zero-family example; stock is not empty; built-in afterburner parameters are a reason to question the local active-module analogue. Three stock actives alone do **not** prove exactly three active slots. No universal afterburner equipability rule is inferred from this page.

<a id="s19"></a>
### S19 — Navigation sources [N/W; discovery only]
- **Star Conflict official home:** https://star-conflict.com/en/ ; website-wide update date unknown; individual linked announcement dates visible. News section led to S16, and the official menu links the wiki. Not used instead of the original announcement.
- **Main Page:** https://wiki.star-conflict.com/index.php?title=Main_Page ; oldid **14636**, edited **2024-01-19 14:24**. Ship equipment section links Main weapon, Missiles, Munitions, Special Modules, Active Modules and Ship Modifiers. Used to identify original sources, not to establish an exhaustive catalog.

<a id="s20"></a>
### S20 — Atlas [W]
- URL: https://wiki.star-conflict.com/index.php?title=Atlas ; oldid **10056**; edited **2019-10-23 06:19**.
- Heading: View of Atlas with equipment list.
- Excerpt: “Version (Mk.1 - Mk.5) of equipment.”
- Claim supported: equipment upgrade/quality level differs from ship rank and ship experience. General 2019 UI overview, corroborated by 2026 indexes and item tables; not proof of every item's available levels or upgrade costs today.

<a id="s21"></a>
### S21 — Lightweight Hull [W]
- URL: https://wiki.star-conflict.com/index.php?title=Lightweight_Hull ; oldid **13684**; edited **2023-06-16 12:46**.
- Headings: type/group/ranks; Description; Characteristics.
- Evidence: Hull modifier, ranks 10–14 / 13–17; increases maximum speed and turn speed but reduces hull by **6.5%**. Rank 13–17 Mk.1 row: **3%** max speed and **14%** rotation. Higher Mk columns add improvements.
- Claim supported: the local light-armor cross-family tradeoff is conceptually appropriate, but its fixed −600 armor/+30 speed is not the sourced percent-based instance. Do not equate similar names automatically.

<a id="s22"></a>
### S22 — Minelayer [W]
- URL: https://wiki.star-conflict.com/index.php?title=Minelayer ; oldid **14322**; edited **2023-11-09 15:10**.
- Heading: main characteristics table.
- Evidence: Mine field, ranks 4–17, for Engineering/Guard/Long Range; deploys field with 455 m trigger sensitivity for 80 s, up to eight damaging triggers; **2 charges/cartridge**, **15 s recharge**, **120 s cartridge reload**. No flight-speed property is stated. Anaconda lists it under Missile slot [S6].
- Claim supported: missile-slot equipment can be a stationary minefield. Do not force a fictional positive speed into the local `MissileModel` to make the example fit.

<a id="s23"></a>
### S23 — Thar'Ga [W]
- URL: https://wiki.star-conflict.com/index.php?title=Thar%27Ga ; oldid **15701**; edited **2025-04-26 19:35**.
- Headings: General information; Nodes; List of nodes.
- Excerpt: “Active module slot, the third (120 points). The first two are given immediately.” Fourth slot is another 120-point node.
- Evidence: Ellydium Gunship fighter, ranks 5–15; nodes change rank, availability and specials; auxiliary nodes add modifier/active slots. Shield/hull/CPU first slots given immediately; second/third available as nodes; engine/capacitor list first/second/third nodes. Shield bonuses include **+1800 points**, **+500 points**, **+15%**, **+10%**; several specials are available by development.
- Claim supported: actual capacity can be two then three/four rather than fixed four; flat and percentage bonuses coexist; eligibility/layout/special can depend on selected build. Progression itself is expressly outside Stargem's present scope, so this is a deliberate simplification/coverage boundary, not an internal defect.

## Synthesis: terms, hierarchy, properties and cardinalities (Stanford steps 3–6)

### Vocabulary and relationships

- **Ship class/size, role, model, faction, rank, experience level, selected development/build, premium status** are distinct axes. A modifier family is a fitting location, not necessarily the only stat affected.
- **Equipment definition/variant** is not the same concept as **equipped occurrence**, **owned/unlocked item**, **quality/Mk level**, **stock configuration**, or **physical turret**. Local IDs presently collapse equipment variants into one balance record; that is a valid simplified choice.
- **Ship modifier/passive:** installed in Engine/Capacitor/Shield/Hull/CPU. Local terms Motor/Armor/Computer are analogous labels, not necessarily defects. May provide flat values, percentage changes, tradeoffs or event-triggered behavior.
- **Active combat module:** manually invoked, possibly with a continuous passive component. Input action, startup condition, payment pattern, duration, shutdown trigger, recharge and target/range are separate properties. A modifier having an automatic effect is not thereby an active-slot module.
- **Special module:** separate ship ability; normally fixed to ship, but alternate/configurable variants mean role→special is not globally functional in the compared game.
- **Missile-slot equipment:** guided missile, unguided missile, bomb, minefield, drone or other device. A drone can separately appear as special behavior, active-module deployment, or missile-slot payload: classify by item/fitting relation, not by the word “drone.”
- **Munitions:** distinct equipable category affecting weapons [S13, S14]; omitted locally. This audit records the coverage seam without demanding implementation.

These pass the is-a/has-a test: a ship **has** equipment; a missile-slot payload need not **be a missile**; an equipment definition **has** eligibility/effects rather than inheriting from a ship. The existing disjoint local equipable union is coherent for its selected subset; the external examples show where its covering assumption is too narrow for exhaustive fidelity.

| Relation / facet | Local contract | External evidence and limit |
|---|---|---|
| Ship→equipped active occurrences | 0..4 globally; `domain.md:176,203,243`, `model.rs:204,363` | Thar'Ga starts with capacity 2 and can unlock 3/4 [S23]; stock listings of 3 or 4 elsewhere do not establish rank-wide rules. |
| Ship→passive occurrences per family | 0..model capacity; each capacity 0..3; `domain.md:98,175,239–241` | Five families agree; zero capacitor on Swift, zero shield on Phoenix; crafted/build-dependent exceptions to universally fixed layout [S7, S15, S18, S23]. |
| Item→eligibility | Passives family only; actives allowed role set; weapons one size; missiles reference only | Additive real-world conditions include rank band, class/role, model and exceptions [S1–S4, S9–S13, S17, S21]. No complete universal eligibility algorithm inferred. |
| Repeated occurrences | Duplicate passives deliberately valid and fully summed; actives have no uniqueness check | Repeated modifiers documented with reduction; Emergency Barrier max one. General active repeated-install policy remains unknown [S2, S8, S9]. |
| Ship→selected weapon | Exactly 1 ID | Multiple physical turrets [S3] are compatible with one weapon choice; no contradiction solely from gun counts. |
| Ship→missile equipment | Exactly 1 missile ID/launcher; ammo per life | Broader payload categories and cartridge reloads [S4, S12, S22]; cited pages do not prove mandatory fill or independently verify every ship's slot count. |
| Role→special | One-to-one, always outside loadout | Role-associated usual special but ship/faction/build variants [S5, S7, S15, S23]. |
| Effective stat | Base + flat sum | Percentage composition, point bonuses, automatic and conditional effects [S8–S10, S21, S23]. |
| Quality | No represented facet | Mk.1–Mk.5 vocabulary; specific items may stop at Mk.4 and rank bands overlap [S9–S11, S13, S15, S20]. |

**Capacity is an upper bound, not compulsory fill.** Canonical C4/C6 allow empty module arrays, and stock `buy` deliberately produces them. `ontology.md:40–41` saying “filling”/“4 ActiveCombatModules” should not override `domain.md:175–178,202–203` or narrative `ontology.md:60` saying “up to 4.” Likewise, never turn all icons under a wiki “Available equipment” heading into equipped items, and never equate the stock-item count with maximum capacity without independent evidence.

## Three representative real equipment/build examples (Stanford step 7)

These are **research instances**, not new repository instances or accepted schema revisions. Values are source-qualified and deliberately do not force-fit unsupported concepts.

### R1 — Rank-8 Anaconda with a missile-slot drone

S6 identifies an Engineering frigate and lists Attack drone as a missile-slot option; S12 restricts the drone to Engineering and ranks 4–17. At rank 8 it falls in S12's 7–9 band: 300 m/s deployed drone, 2130 durability, 2 charges/cartridge, 15 s individual recharge and 120 s cartridge reload. Stock remains a **different relation**: Anaconda's listed stock payload is Cruise missile, not this optional drone.

**Local-fit result:** local Anaconda role/class agree. No local drone payload or magazine reload exists. Its listed stock two CPU modifiers and three hull modifiers exceed local capacities CPU=1/hull=1, if exact stock fidelity is intended. Do not relabel drone firing damage as missile direct-hit damage to fit `MissileModel`.

### R2 — Phoenix and Shield Sync: restricted active with both passive and timed effects

S7's Phoenix is rank-15 Engineering; S10's Shield Sync is for Phoenix, ranks 12–17. Its description gives continuous shield regeneration even outside its activated link, while active time is 5 s with 40 energy/sec and Mk.1 recharge 23 s. Ship bonuses may alter displayed/effective outcomes, so these are item-table baselines, not a fully calculated build.

**Local-fit result:** role-only restrictions cannot distinguish Phoenix from all other engineers; two-flow union lacks finite ongoing duration and typed passive component. Local Phoenix is a gunship and has a shield slot while the source says no shield slots; reconcile identity first, not just the module row. This example does not imply all active modules have passive effects.

### R3 — Thar'Ga capacity/build selection and mixed bonus kinds

S23's rank-5–15 Gunship fighter initially has two active slots; third and fourth require separate nodes. Modifier family additions are also selected nodes. Its shield improvements include +1800 points and +15%, and its special is a selected development variant rather than the one universal Gunship ability.

**Local-fit result:** fixed model layout/global four-active allowance/role-derived special cannot represent this build distinction. Flat deltas could represent a precomputed one-build result, but would erase provenance and not answer “what if another node changes?” That is acceptable if progression/configuration is intentionally excluded; no requirement to implement this ship follows.

Additional small calibration example: local Light Armor's armor-for-speed tradeoff agrees qualitatively with Lightweight Hull [S21], but a rank-13–17 Mk.1 record would require +3% speed, +14% turn rate and −6.5% hull rather than the local fixed deltas. This agreement is conceptual, not numeric provenance.

## Agreements, conflicts and uncertainties retained

**Agreed or compatible:** five passive-fitting families; modules can affect a stat outside their named family; role-oriented active groups; active energy consumption; manual activation distinguished from automatic modifier behavior; separate special ability; weapon heat/cooling exists; a three-per-family ceiling is documented for configurable crafting; one chosen main weapon can drive several turrets. Anaconda's class/role agree. Local `Engine Suppressor` is correctly Tackler-oriented at the concept level. None of these agreements authenticate the unversioned local balance numbers.

**Do not erase these source tensions:**
- General Ship Equipment says special is unique to role; specific Special module/ship/build articles document exceptions. Treat the former as overview, not an exhaustive axiom.
- Modifier addition's “25% damage” worked example explicitly ignores repeated-copy penalties; the same article's note and newer Modifiers index retain those penalties. It is not evidence the penalties vanished.
- Pulse Laser's equipment header restricts to Interceptor while its lore says “light fighter or interceptor.” Do not use lore alone to affirm local fighter eligibility.
- Active-module index calls multipurpose modules broadly usable except destroyers, then lists separate destroyer multipurpose modules. Distinguish equipment variants/categories rather than treating the overview as “destroyers have no multipurpose modules.”
- Phoenix's parenthetical maximum-two allowance lacks a clear counting boundary. No imported generic limit is justified here.
- Old ship revisions render shared equipment lists containing newer items. Last page edit is not the date every rendered equipment choice became valid.

**Remaining questions for product/reconciliation, not user interrupts:**
1. Is Stargem a deliberately independent “Star Conflict-like” game, or a faithful model of a specified dated version? Which official online/special-version target?
2. Are familiar ship/item names identities, homage or temporary placeholders? In particular Emergency Barrier, Pulse Laser and Phoenix.
3. Should local slot capacity stay fixed by model, and should stock modules stay empty? These are separate decisions from mandatory fill.
4. What active-copy/duplicate-variant restrictions actually apply in the target version? Current public sources here do not establish a universal rule.
5. Is the desired effect scope flat static deltas, percentages, triggered modifiers, passive components on actives, ship bonuses, munitions or only a selected subset?
6. What is the exact composition order for repeated penalties, mixed positive/negative bonuses and flat/percent contributions? Resistance is outside this task; S8 is not an all-effects specification.
7. Are item rank/Mk facets needed without implementing progression? Which instances/patch date should supply values?
8. What are cooldown start, cancellation, resource-failure and recharge semantics for each supported ability? Do not infer them from the presence of an energy-per-second column.
9. Is “missile” deliberately narrow or should the slot admit selected mines/drones/support devices, and how do magazines/reserves replenish?
10. Are special variants and crafted/Ellydium configurations explicitly excluded? If yes, preserve that simplification rather than treat every exception as an error.
