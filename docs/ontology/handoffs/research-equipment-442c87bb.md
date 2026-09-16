# Handoff: research-equipment — 442c87bb

Checkpoint: 2026-09-16, user-requested pause. Parent-curated from runtime records and actual files; not a new completed child report.

- **State:** COMPLETED; reuse output, do not rerun by default
- **Role:** `delegate`
- **Child run:** `442c87bb-8886-4094-a7f8-abe8e0f88bf8`
- **Workflow / key:** `507d64bc-df21-4587-a4fb-a4425950ac5f` / `research-equipment`
- **Mission:** `28a33a06-b296-4a78-8ae0-079881816489`
- **Repository:** `/home/theta/repos/stargem.nix`, branch `master`
- **Source baseline:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`
- **Session (machine-local, optional for recovery):** `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/2026-09-16T18-47-01-030Z_01a0ab8b-30a5-761c-90d3-cc64125796e1/3e9273f2-40ec-46a7-9ee1-6178137126c8/run-0/session.jsonl`
- **Runtime status (machine-local):** `/tmp/pi-subagents-uid-1000/async-subagent-runs/442c87bb-8886-4094-a7f8-abe8e0f88bf8/status.json`

## Authority and boundaries

Read root `AGENTS.md` and `docs/HANDOFF.md` before work. `ontology/` remains canonical and unchanged. This is documentation/research, not authorization to change gameplay. No commit/push/merge/install is authorized. One writer per checkout. Preserve untracked audit files: they are the current work, not disposable clutter. Other agents' full research is intentionally file-backed to protect parent context.

## Resumption (for Agent)

Research is complete for this pass. Reuse the original report below and `../research/equipment.md`; do not refetch everything. Synthesis/review are still pending. Only resume this child for a specific missing citation, disputed claim or fresh evidence request. Keep publication/patch date distinct from retrieval date and deliberate local scope distinct from defects.

## Native continuation or fallback

Native resumability was verified with children.list at checkpoint. Recheck it in the next session; retained session availability can change.

```javascript
subagent({ action: "status", id: "442c87bb-8886-4094-a7f8-abe8e0f88bf8" })
subagent({ action: "children.list" })
// Only when resumable, and no original writer is still active:
subagent({ action: "resume", id: "442c87bb-8886-4094-a7f8-abe8e0f88bf8",
  message: "Read AGENTS.md, docs/HANDOFF.md and docs/ontology/handoffs/research-equipment-442c87bb.md. Continue ONLY the remaining work recorded there; preserve completed outputs and current files." })
```

If the retained session is missing or not resumable, do not claim a native resume or rerun all research. Launch a fresh `delegate` as a **same-role fallback continuation**, give it this handoff and the preserved reports, and start at the exact next step above. A resumed run may return a new run ID: record it and continue from the latest identity.

## Verification and acceptance

The original report below states exactly what this child checked and what remains uncertain. Completion of a research child does not mean that document synthesis, independent evidence review, ontology review or user reconciliation has completed. Follow [the main checkpoint](README.md) for dependency order and pending reviews.

## Preserved original output

Original managed artifact: `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/507d64bc-df21-4587-a4fb-a4425950ac5f/research/equipment.md`

SHA-256 of original artifact: `c0487b792739f9c36b4409b01a603a81d5b4c34d1477b0088efa8642c2b586c1`

The complete output is embedded below so recovery does not depend on session/temp-file retention. Original relative code paths refer to the repository root unless the report says otherwise. Historical access failures stay historical.

---

# Equipment audit — Star Conflict evidence versus Stargem

**Audit/retrieval date:** 2026-09-16. **Repository:** `/home/theta/repos/stargem.nix`, `master`, baseline `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`.

**Authority:** research only. `ontology/` is canonical for Stargem; `ontology.md` is its source narrative, not a claim to reproduce every Star Conflict rule. No ontology, source, project documentation, or existing draft was edited. This report identifies reconciliation choices, not approved rules or implementation instructions.

**Retry result:** access now works. The wiki main page, equipment indexes, original item/ship articles, MediaWiki search API and official website returned HTTP 200. The sole missing article was an incorrectly guessed `Equipment` title (404), resolved by discovery of `Ship_Equipment`. There was no network/firewall failure in this pass. Earlier blocked-only reports are historical, not evidence of present unavailability.

## Executive findings

1. The local five passive families and role-filtered active modules resemble the game, but family/role alone is insufficient for actual equipment eligibility: rank bands, ship class, named-model exceptions, premium-ship eligibility, faction and per-item limits matter [S1–S4, S9, S10, S12, S17].
2. Local `Emergency Barrier` is a manually activated guard/command module. The identically named wiki item is an automatic **capacitor modifier**, restricted to one copy, with a hull-threshold trigger and a ramming exception [S9]. This is the clearest equipment-category divergence.
3. Local flat additive modifiers and full-value repeated passives are deliberate simple rules, not the wiki's percent/effect and repeated-modifier mechanics [S2, S8, S21]. Do not silently replace the local arithmetic.
4. A “missile slot” is not restricted to moving damaging missiles: Anaconda lists Minelayer and Attack drone alongside missiles; these have cartridge reloads distinct from individual-use recharge [S4, S6, S12, S22].
5. One fixed special per **role** is a local simplification. The wiki describes generally fixed ship specials but explicit exceptions; Phoenix has alternatives and Thar'Ga has development-dependent specials [S5, S7, S23].
6. Capacity, availability and stock fill must remain separate. Local active/passive equipment is optional up to limits. Stock wiki listings do not establish mandatory fill or exact capacity; explicit examples establish zero-slot families and configurable active capacity [S6, S7, S14, S23].

### Freshness warning

“Verified contradiction” below means a directly verified difference between **local text/data and the cited retrieved documentation**, not an independently tested September 2026 game-client result. Individual wiki page edits range from 2019 to 2026; shared templates can make a rendered page newer than its own last-edit date. No wiki edit timestamp establishes the patch in which a mechanic changed. No installed game, account or replay was used.

The official 2026-09-14 announcement says the existing servers will shut down soon and a special version supporting a local server is being developed [S16]. Thus “current Star Conflict” needs an explicit version/service target. This announcement says nothing about changing the equipment rules below; do not transfer them automatically to the upcoming special version or to the separate *Star Conflict: Burrownauts* game.

## Method and local inspection

Read root `AGENTS.md` first; it exists in this retry. `docs/HANDOFF.md` remains absent. Preserved the authorized untracked `AGENTS.md` and `docs/ontology/` work. Read the supplied inventory as orientation, then independently read `ontology/domain.md`, `ontology.md`, all of `ontology/model.rs`, and the six relevant instance files: ship models, passives, actives, weapons, missiles, specials. No source claim rests solely on the older inventory.

Applied the requested ontology skill at `/home/theta/.pi-game-dev/git/github.com/M4jor-Tom/claude-ontology-skill/skills/ontology/SKILL.md`. Four initial navigation angles were active-module eligibility; passive families/stacking; weapons; missile-slot payloads. Followed their original item links and representative ship pages. A bounded gap-closing pass used MediaWiki full-text search for slots, repeated equipment and upgrades, then inspected Ship Equipment, Crafting, Atlas, Lightweight Hull, Minelayer, Engine Suppressor and Thar'Ga. Search snippets were discovery only. No unbounded crawl or package install.

HTML was fetched with `curl --connect-timeout 10 --max-time 40 -L -sS`, then converted with installed `markitdown` before reading. Tables retained the relevant values and icon-link titles; no substitute HTML parser was required. Nested tables are visually messy in Markdown, so this report uses clearly labeled rows or cautious lower-bound counts, not inferred hidden slot layouts.

### Stanford steps 1–2: scope, competency questions and reuse

**Domain:** equipment selection and effects on player ships, with Star Conflict as external comparison and Stargem as the locally authoritative design. **Users:** later reconciliation writer, content authors and maintainers. Economy/progression implementation, full ship taxonomy, combat damage formulas and game modes are not assigned here.

| Competency question | Result / evidence |
|---|---|
| What equipment families are distinct? | Main weapon, munitions, missile equipment, actives and five modifier families; special is separate [S1–S5, S14]. |
| What determines whether an item fits? | Family, class/role, rank band and sometimes model/premium/faction, rather than size or family alone [S1–S3, S9–S13, S17, S21]. |
| How many slots exist and must they be filled? | Explicit local capacities; external examples, not an exhaustive rank table. Zero families and Thar'Ga's 2→3→4 actives are documented; mandatory fill is not established [S6, S7, S14, S23]. |
| Are repeated modules permitted and how do they combine? | Repeated modifiers exist, with penalties; item-specific singleton example Emergency Barrier. General duplicate-active policy remains unresolved [S2, S8, S9]. |
| Are bonuses flat, percentages or behavioral effects? | All three occur, including conditional triggers and flat plus percentage development bonuses [S8–S10, S21, S23]. |
| Are rank, quality and ship experience the same? | No: item rank bands, Mk level, and ship experience/development are separate source concepts [S6, S10, S11, S15, S20, S23]. |
| Is energy payment equivalent to effect duration or cooldown? | No: one payment can sustain a timed effect; per-second cost can have finite active time; an automatic modifier can have recharge [S9–S11, S17]. |
| Does the missile slot always launch missiles? | No: minefield and attack drone examples are explicitly listed [S4, S6, S12, S22]. |
| Are stock modules empty and specials always derived from role? | Local yes; retrieved game examples include stock actives/passives and alternative/configurable specials [S5–S7, S15, S23]. |

Reuse rather than redesign: existing `PassiveSlotType`, role restrictions, `ActivationFlow`, loadout arrays and ID references already express the selected simple game. This audit does not propose a new generic effects framework or import external schemas/packages. The local Rust+JSON+validator arrangement is the applicable equivalent of the skill's typed/runtime/storage alignment; there is no SQL layer to invent.

## Source register

**Common metadata applying to every S-record:** retrieved **2026-09-16**, HTTP 200, original HTML retained and converted with `markitdown`. **W** = community-maintained, official-hosted Star Conflict Wiki, not an official patch announcement; **O** = official Star Conflict website announcement; **N** = official navigation/index. Wiki dates below are visible **last-edited** dates, not patch dates; mechanic introduction/patch date is **unknown** unless explicitly stated. Exact retrieval URLs are given, including redirect aliases. `oldid` identifies the revision reported by the rendered page, not a separately fetched historical revision.

### S1 — Active module [W]
- URL: https://wiki.star-conflict.com/index.php?title=Active_Modules (renders **Active module**); oldid **16433**; edited **2026-05-09 09:53**.
- Headings: introduction; Types of modules; role groups.
- Short excerpt: “Unique modules can only be installed on the ship which they were created for.”
- Evidence: role and multipurpose actives; common, unique, unique-and-premium-same-role, and Ellydium eligibility families. Multipurpose's broad statement excludes destroyers, followed by a separate destroyer multipurpose group. Modules use energy; insufficient energy may prevent activation or end operation. Manual or automatic shutdown depends on the module. Mk.1–Mk.5 level indication. Destroyer actives have physical placement/durability, so are an additional scope, not automatically equivalent to ordinary actives.
- Claim supported: `allowed_roles` is a useful but incomplete real-game eligibility predicate; activation flow is module-specific. No general active-duplicate prohibition appears here.

### S2 — Modifiers [W]
- URL: https://wiki.star-conflict.com/index.php?title=Ship_Modifiers (renders **Modifiers**); oldid **16438**; edited **2026-05-09 09:56**.
- Heading: introduction; Engine/Capacitor/Shield/Hull/CPU modifiers.
- Excerpts: “Some ship modifiers are capable of active actions”; “If several same modifiers are installed, the effectiveness of each one is reduced”.
- Evidence: five types; suitable slot family; slot distribution described as dependent on rank/faction. Automatic actions require no pilot input. Repeated copies are described as up to 90% effectiveness with two, up to 80% with three. Common engine modifiers exclude Ellydium; model/premium-role/rank exceptions exist. Mk.1–Mk.5 levels.
- Claim supported: passive does not mean stat-only; repeats are possible but not universally full strength; family alone is not sufficient eligibility. Exact penalty implementation needs verification, not extrapolation.

### S3 — Main weapon [W]
- URL: https://wiki.star-conflict.com/index.php?title=Main_weapon ; oldid **16428**; edited **2026-05-09 09:46**.
- Headings: introduction; Parameters of weapons; List of weapons.
- Excerpt: “Common weapons are available for installation on all ships of the appropriate class and role.”
- Evidence: model-exclusive, premium-same-role/rank and Ellydium distinctions; some weapons consume energy or repair allies. Multiple turrets: interceptors two, fighters four, frigates four with stated exceptions; not multiple independent selected weapon types. Some weapons have extra full-overheat penalties.
- Claim supported: size is not the entire fit rule; “one selected weapon” need not contradict multiple physical guns. Six local ship stats do not cover every equipment parameter.

### S4 — Missiles [W]
- URL: https://wiki.star-conflict.com/index.php?title=Missiles ; oldid **16376**; edited **2026-04-26 15:42**.
- Headings: introduction; Types of missiles and similar devices.
- Excerpt: “Besides the missiles on ship can be installed” bombs, mines and other devices.
- Evidence: guided/unguided; size/class separation and role-restricted types; cartridges reload after exhaustion, distinct from launching a single charge. Categories list Attack drone, Static Shield, Heavy Repair Drone and other devices as well as mines/bombs.
- Claim supported: missile-slot payload is broader than a projectile with positive flight speed and direct damage. This index alone does not settle mandatory occupancy or all special-ship restrictions.

### S5 — Special module [W]
- URL: https://wiki.star-conflict.com/index.php?title=Special_Modules (renders **Special module**); oldid **16431**; edited **2026-05-09 09:52**.
- Headings: introduction; Main; Special modules for Ellydium ships.
- Excerpt: fixed to the ship and not removable/replaced, “except for some unique ships with cratable special modules” [source spelling].
- Evidence: generally separate fixed ship special, default F activation; explicit exceptions. Long Range main entries differ for Empire (Disintegrator) and Jericho (Guided torpedo). Thar'Ga has several special variants. Combat Drones combine automatic drone behavior with activated restoration; Phase shield has a flat 120-point resistance plus percentage/conditional bonuses.
- Claim supported: local separation of special from ordinary actives is broadly agreed, but universal role↔special bijection and universal nonequipability overcommit.

### S6 — Anaconda [W]
- URL: https://wiki.star-conflict.com/index.php?title=Anaconda ; oldid **14755**; edited **2024-01-31 17:21**.
- Headings: General information; Equipment upon purchase; Available equipment.
- Evidence (precise paraphrase): Federation rank-8 Engineering frigate. Purchase table lists Beam Cannon, Xenon Lamp, Cruise missile; four actives; modifier entries engine 1, capacitor 2, shield 1, hull 3, CPU 2. Missile-slot choices include **Minelayer** and **Attack drone**. Experience-level bonus reduces Engineering-module energy consumption by 20% at level 4.
- Claim supported: local role/class agrees; stock is not module-empty; slot-family examples and nonmissile payload availability. Counts above are **listed stock items**, not independently proven maximum capacity. Rendered equipment templates may be newer than 2024.

### S7 — Phoenix [W]
- URL: https://wiki.star-conflict.com/index.php?title=Phoenix ; oldid **16514**; edited **2026-06-10 17:30**.
- Headings: General information; Description; Equipment upon purchase; Available equipment; Unique ship equipment.
- Excerpt: “Ship has no shield slots”.
- Evidence: rank-15 premium Engineering frigate; stock actives/passives; Combat Drones and 'Phoenix' Drones listed in special row; named unique actives include Shield Sync. Premium description allows “unique and for the premium ships of the same role” weapons/modules/modifiers with a parenthetical “maximum 2”. It does not clearly explain the combined/category counting boundary.
- Claim supported: local Phoenix shield capacity 1 differs; local gunship role also differs (not a full ship-taxonomy audit here). Model-specific active eligibility and special variants exist. Preserve the ambiguous maximum-two qualification rather than generalizing it.

### S8 — Modifier effect addition [W]
- URL: https://wiki.star-conflict.com/index.php?title=Modifier_effect_addition ; oldid **14535**; edited **2023-12-22 11:34**.
- Headings: introductory formulas; Example 1; Example 2; Notes.
- Short excerpt: formulas apply to percentage-changing weapons/actives too; resistance modifiers are excepted from the introductory general rule.
- Evidence: for positive percentage Z, `mod = Z/100`; for negative Z, `mod = 1 - 100/(100+Z)` (Z expressed in percentage points). Sum contributions into MOD, then positive total uses `base*(1+MOD)` and negative total `base/(1-MOD)`. Example 1 explicitly computes three −50% collision modifiers as 25% damage **while excluding the repeated-module penalty**, which its footnote reiterates. Example 2 enhances an Engine Suppressor effect using an implant.
- Claim supported: simple flat sums are not a faithful universal external formula. The example must not be quoted as the complete three-copy result. MOD=0 behavior, mixed effect order and special exceptions are not exhaustively specified.

### S9 — Emergency Barrier [W]
- URL: https://wiki.star-conflict.com/index.php?title=Emergency_Barrier ; oldid **13647**; edited **2023-06-16 12:03**.
- Headings: type/group/ranks; Description; Characteristics.
- Excerpts: “Type: Ship modifier”; “Group: Capacitor modifiers”; “You can place only one modifier.”
- Evidence: rank variants 5–7, 7–11, 10–14, 13–17; listed for all nine ordinary roles and Suppressor. Below 15% hull, automatically grants one second of invulnerability **except ramming**. Recharge varies with rank/quality; 13–17 Mk.1 row is 84 seconds. This is neither a manual ability nor a permanent additive stat delta.
- Claim supported: named local item disagrees in family, restriction, trigger, effect exception and duration. Balance numbers have older-source freshness uncertainty; classification is also corroborated by the 2026 modifier index [S2].

### S10 — Shield Sync [W]
- URL: https://wiki.star-conflict.com/index.php?title=Shield_Sync ; oldid **16565**; edited **2026-06-20 14:39**.
- Headings: type/group/ranks/ship; Description; Characteristics.
- Evidence: active Engineering module specifically **for Phoenix**, ranks 12–17; continuously restores 125 shield points/sec; activation links an ally in direct view. Active time **5 s**, energy **40/sec**, Mk.1 recharge **23 s**, transfer **1625/sec**, range **3000 m**; quality columns improve transfer and recharge. Own critical-hit chance +50% for 5 s.
- Claim supported: active item may also have an always-on benefit; continuous payment does not imply indefinite toggle lifetime; role alone cannot represent exact model eligibility. Recharge start instant/manual cancellation is not established by the table.

### S11 — Shield Booster S [W]
- URL: https://wiki.star-conflict.com/index.php?title=Shield_Booster_S ; oldid **8690**; edited **2019-06-23 08:10**.
- Headings: type/group/ranks; Description; Characteristics.
- Excerpt: “Repairs your interceptor's shield for 8 s.” with small portions once per second.
- Evidence: interceptor multipurpose active, rank bands 3–5 / 6–8 / 8–12 / 11–17. Version 5 Mk.1: 2070 shield restoration, 117 energy, 40 s recharge; quality columns increase restoration. A one-time energy field is separate from finite effect duration.
- Claim supported: a shield-repair analogue is not necessarily instant or universally size-independent. This is old balance evidence, not proof that local generic `Shield Boost` was intended as this exact item.

### S12 — Attack drone [W]
- URL: https://wiki.star-conflict.com/index.php?title=Attack_drone ; oldid **13350**; edited **2023-02-09 16:15**.
- Heading: main characteristics table.
- Evidence: Engineering-only, ranks 4–17; launches a drone pursuing/attacking current target. **2 charges/cartridge**, **15 s recharge**, **120 s cartridge reload**; drone speed 300 m/s; firing range 1000 m; durability varies with rank. Category is Missiles, and Anaconda lists it under Missile slot [S6].
- Claim supported: a deployed entity, not direct missile damage, can occupy this equipment context; reload layers and role restrictions are distinct.

### S13 — Pulse Laser [W]
- URL: https://wiki.star-conflict.com/index.php?title=Pulse_Laser ; oldid **15371**; edited **2024-08-30 12:38**.
- Headings: type/ship/ranks; Description; Characteristics.
- Excerpt: “Type: Close range thermal weapon”; ship type Interceptor (Recon/Covert Ops/ECM).
- Evidence: rank bands 4–7, 7–11, 10–14, 13–17, plus a named Champion variant qualifier; Mk levels and munitions. Header restricts to interceptors, despite lore sentence loosely saying “light fighter or interceptor.”
- Claim supported: local identically named fighter/EM Pulse Laser differs. Treat the equipment header/table as structured eligibility evidence and retain the imprecise lore wording as a source caveat, not silently reconcile it.

### S14 — Ship Equipment [W]
- URL: https://wiki.star-conflict.com/index.php?title=Ship_Equipment ; oldid **14987**; edited **2024-03-14 16:31**.
- Heading: introductory list and paragraphs.
- Excerpt: “Available slots increase with ship rank and tier.”
- Evidence: five equipable categories including munitions; separate role special.
- Claim supported: equipment capacity is not a universal mandatory filled count. This is a coarse guide, not a numerical rank→slot table, and its role-special summary is less specific than [S5].

### S15 — Crafting [W]
- URL: https://wiki.star-conflict.com/index.php?title=Crafting ; oldid **10059**; edited **2019-10-23 06:20**.
- Headings: General; How to create your ship.
- Evidence: eligible Mk.4 weapon/some-module items can be upgraded to a researched Mk.5 version. Manufacturing interface can select special module (one of up to three, depending on ship), bonuses and modifier-slot distribution; each family cannot exceed three. Refit can change the build, with exceptions for ships not configurable.
- Claim supported: per-family cap three has a documented context; fixed layout for **every model** excludes configurable crafted ships. Does not establish that every item reaches Mk.5 or every ship can refit.

### S16 — Star Conflict: local server and a new indie game set in the same universe [O]
- URL: https://star-conflict.com/en/news/3933-star-conflict-local-server-and-a-new-indie-game-set-in-the-same-universe-en
- Publication date: **2026-09-14**; update/patch number not given.
- Heading: Special version of Star Conflict.
- Excerpt: “the ability to run a local server” is the main feature of the special version under development.
- Claim supported: time-sensitive interpretation of “current game”; announcement says existing servers will shut down soon. It does not provide a new equipment specification. Separate Burrownauts content is not equipment evidence for this audit.

### S17 — Engine Suppressor [W]
- URL: https://wiki.star-conflict.com/index.php?title=Engine_Suppressor ; oldid **14375**; edited **2023-11-09 16:51**.
- Headings: type/group/ranks; Description; Characteristics.
- Evidence: Tackler active; bands 4–6 / 7–9 / 9–13 / 12–17; 8-second speed reduction also prevents teleport/microwarp/Side booster. Version 6 Mk.1: 118 energy, 30 s recharge, 2000 m range, 41.9% reduction. Version 17 Mk.1: 171 energy, 24 s recharge. Quality increases range and reduction.
- Claim supported: local Tackler restriction and finite-cost control role agree broadly; effect duration, range, rank/quality and prevention effects are omitted locally. Do not label its arbitrary local 100 energy a proven balance regression without a target version.

### S18 — Swift [W]
- URL: https://wiki.star-conflict.com/index.php?title=Swift ; oldid **11651**; edited **2021-03-23 15:02**.
- Headings: General information; Equipment upon purchase; Available equipment; technical tables.
- Evidence: rank-3 Federation Covert Ops interceptor; **“Ship has no capacitor slots”**. Stock lists three actives ('Orion' Targeting Complex, Repair Kit S, IR Flares), and 2 engine/1 shield/1 hull/1 CPU modifiers. Base technical tables distinguish afterburner energy and march/afterburner speeds even when specs are stated without equipment.
- Claim supported: low-rank zero-family example; stock is not empty; built-in afterburner parameters are a reason to question the local active-module analogue. Three stock actives alone do **not** prove exactly three active slots. No universal afterburner equipability rule is inferred from this page.

### S19 — Navigation sources [N/W; discovery only]
- **Star Conflict official home:** https://star-conflict.com/en/ ; website-wide update date unknown; individual linked announcement dates visible. News section led to S16, and the official menu links the wiki. Not used instead of the original announcement.
- **Main Page:** https://wiki.star-conflict.com/index.php?title=Main_Page ; oldid **14636**, edited **2024-01-19 14:24**. Ship equipment section links Main weapon, Missiles, Munitions, Special Modules, Active Modules and Ship Modifiers. Used to identify original sources, not to establish an exhaustive catalog.

### S20 — Atlas [W]
- URL: https://wiki.star-conflict.com/index.php?title=Atlas ; oldid **10056**; edited **2019-10-23 06:19**.
- Heading: View of Atlas with equipment list.
- Excerpt: “Version (Mk.1 - Mk.5) of equipment.”
- Claim supported: equipment upgrade/quality level differs from ship rank and ship experience. General 2019 UI overview, corroborated by 2026 indexes and item tables; not proof of every item's available levels or upgrade costs today.

### S21 — Lightweight Hull [W]
- URL: https://wiki.star-conflict.com/index.php?title=Lightweight_Hull ; oldid **13684**; edited **2023-06-16 12:46**.
- Headings: type/group/ranks; Description; Characteristics.
- Evidence: Hull modifier, ranks 10–14 / 13–17; increases maximum speed and turn speed but reduces hull by **6.5%**. Rank 13–17 Mk.1 row: **3%** max speed and **14%** rotation. Higher Mk columns add improvements.
- Claim supported: the local light-armor cross-family tradeoff is conceptually appropriate, but its fixed −600 armor/+30 speed is not the sourced percent-based instance. Do not equate similar names automatically.

### S22 — Minelayer [W]
- URL: https://wiki.star-conflict.com/index.php?title=Minelayer ; oldid **14322**; edited **2023-11-09 15:10**.
- Heading: main characteristics table.
- Evidence: Mine field, ranks 4–17, for Engineering/Guard/Long Range; deploys field with 455 m trigger sensitivity for 80 s, up to eight damaging triggers; **2 charges/cartridge**, **15 s recharge**, **120 s cartridge reload**. No flight-speed property is stated. Anaconda lists it under Missile slot [S6].
- Claim supported: missile-slot equipment can be a stationary minefield. Do not force a fictional positive speed into the local `MissileModel` to make the example fit.

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

## Candidate reconciliation ledger

Confidence distinguishes certainty of the observed documentary comparison from uncertain live-game freshness. **High impact** means material to a fidelity/import effort, not permission to expand scope. “Deliberate simplification” is supported by the explicit local rule; it does not assert the author consciously considered every external exception.

### E01 — Emergency Barrier family/trigger mismatch
- **Local:** `ontology/instances/active_modules.json:11–13`: guard/command-only, one-shot, 200 energy, 60 s cooldown, “Absorbs all damage for 3 seconds.” `domain.md:103–116` gives passives stat modifiers and actives role/flow/effect.
- **Compared:** automatic singleton capacitor modifier, <15% hull trigger, 1 s, excludes ramming, broader roles [S9], category corroborated [S2].
- **Classification:** **verified contradiction** if this name denotes the game's item; otherwise deliberate same-name redesign. **Confidence:** high for category/trigger documentary mismatch, medium for current numeric balance. **Impact:** high; wrong slot and activation model.
- **Reconciliation:** retain and explicitly rename/document the independent ability, or authorize a sourced passive-trigger counterpart and decide effect/variant scope. Do not just move its JSON row: `StatModifiers` cannot express the actual behavior.

### E02 — Flat sums and repeated modifiers
- **Local:** `domain.md:85–86,242`, `model.rs:83–93,390–403`; local Anaconda + three Shield Extenders is explicitly `9000 + 3*1500 = 13500` in `model.rs:444–450`.
- **Compared:** percent transformations, different treatment of reductions, repeated-copy penalties; point and percentage bonuses coexist [S2, S8, S23].
- **Classification:** **deliberate simplification**, verified difference from broader game calculation, not internally inconsistent. **Confidence:** high on difference; medium on exact generalized game formula/penalty exceptions. **Impact:** high for fitting predictions.
- **Reconciliation:** keep flat-only simplified passives, or first decide effect units, stacking groups, repeated-copy policy and scope of conditional effects in canonical ontology. S8's penalty-excluding example is not a complete implementation specification.

### E03 — Singleton versus repeatable equipment not expressed
- **Local:** C4/C6 `domain.md:241,243`; `model.rs:351–372` has per-family count and role checks but no item-copy limit. Active duplicates are not forbidden locally; passive repeats are intentionally tested.
- **Compared:** repeated modifiers are valid in general, Emergency Barrier explicitly max one [S2, S9].
- **Classification:** **coverage gap** for per-item limits, not proof every duplicate is illegal. General duplicate-active rules are **uncertainty**. **Confidence:** high on the singleton example; no conclusion on all active duplicates. **Impact:** medium/high if real items imported.
- **Reconciliation:** decide repeatability per supported item; retain current permissive rule if independent-game intent. Do not add a blanket “all modules unique” invariant.

### E04 — Eligibility beyond active role/passive family
- **Local:** `domain.md:98,103–116,241,243`; `model.rs:126–150,351–372`; Shield Boost's `allowed_roles: []` at `active_modules.json:2`.
- **Compared:** Shield Booster S is interceptor/rank-limited [S11]; Shield Sync is Phoenix-specific [S10]; common engine modifier faction exception and premium/model/rank groups [S1, S2].
- **Classification:** **coverage gap/deliberate simplification**. The generic local Shield Boost is not proven identical to Shield Booster S. **Confidence:** high. **Impact:** high for real catalog validation.
- **Reconciliation:** keep role/family-only subset or select which eligibility axes matter. Rank/progression and faction are explicitly deferred at `domain.md:18–23`; representability need is not automatic approval to implement them.

### E05 — Weapon fitting and named Pulse Laser
- **Local:** `domain.md:127,244`, `model.rs:154–165,373–379`: exact size match only. `weapons.json:8–9`: **Pulse Laser**, fighter, electromagnetic. `ship_models.json:25` stocks it on Lynx.
- **Compared:** Pulse Laser header is interceptor/thermal/rank-banded [S13]; general weapons can also be role/model restricted [S3].
- **Classification:** **verified named-instance contradiction** (if identity intended) plus **coverage gap** in eligibility facets. **Confidence:** high for table comparison; lore wording is less precise and preserved. **Impact:** high for imported content.
- **Reconciliation:** distinguish independent homonym versus faithful item before changing class/damage; consider a restricted supported-weapon subset rather than universal game fit rules. Multiple turrets alone do not invalidate one selected weapon ID.

### E06 — Missile-slot payload ontology too narrow for the compared game
- **Local:** `domain.md:136–142,178,245`; `model.rs:168–177,329–330,380–381`: every missile has positive damage/speed/ammo/reload and all resolved missile IDs fit every ship. Three entries in `missiles.json:2–4` are direct missiles.
- **Compared:** Anaconda can install Minelayer or Attack drone in its Missile slot; drone role limitation; minefield has no engine [S4, S6, S12, S22].
- **Classification:** **coverage gap/deliberate simplification**, not a defect because mines are absent. **Confidence:** high. **Impact:** high for a full loadout ontology.
- **Reconciliation:** retain “missiles only” scope explicitly, or name the slot/payload concept more broadly and add only selected payload variants. Do not require artificial speed/direct-hit values for nonmissiles. Mandatory occupancy remains a separate local decision.

### E07 — Ammo per life conflates cartridge count with reserve/reload
- **Local:** `domain.md:141–142`: ammo “per launcher per life”; reload “between launches”; `model.rs:175–176` has only ammo and reload_s.
- **Compared:** missile cartridge exhaustion begins a separate reload; Attack drone and Minelayer each show 2 charges, 15 s recharge, 120 s cartridge reload [S4, S12, S22].
- **Classification:** **deliberate simplification / verified semantic divergence** if intended as actual cartridge mechanics. **Confidence:** high documentary, medium current timings. **Impact:** high for sustained-use simulation.
- **Reconciliation:** retain finite per-life ammunition, or distinguish per-use delay, magazine/cartridge size and replenishment. Do not infer unlimited total reserves merely from a cartridge-reload field.

### E08 — Fixed active capacity and model-fixed passive layout
- **Local:** `domain.md:98,176,243`, `model.rs:204–205,363`; no ship-specific active capacity or build-selected passive slots.
- **Compared:** Thar'Ga receives first two active slots immediately and unlocks third/fourth; crafted ships can select/refit passive slots, max three per family [S15, S23].
- **Classification:** **deliberate simplification**, with **coverage gap** if configurable ships enter scope. **Confidence:** high for explicit examples, no claim of a universal rank formula. **Impact:** medium for selected local models, high for catalog expansion.
- **Reconciliation:** keep static models; alternatively represent a selected configuration only when those ships are supported. Progression is excluded locally. Do not derive a rule “all rank-3 ships have three actives” from Swift's stock list.

### E09 — Concrete stock/layout discrepancies
- **Local:** `domain.md:247`, `model.rs:406–419` buy stock weapon/missile with no passive/active modules. `ship_models.json:2–5` Anaconda has CPU=1/hull=1/shield=3; `ship_models.json:18–21` Phoenix has shield=1.
- **Compared:** Anaconda's purchase table has four actives, two CPU and three hull modifiers [S6]; Phoenix explicitly has no shield slots and includes stock modules [S7]; Swift also includes stock modules [S18].
- **Classification:** **verified contradiction** for same-name faithful stock/configuration; otherwise intentional local starter policy. **Confidence:** high on displayed stock/zero-slot facts, medium on historical template freshness. **Impact:** medium/high for fitting/initialization.
- **Reconciliation:** choose empty local starter modules versus real stock template. Counts of listed stock modifiers are lower bounds on usable capacity, not exact maximum layout proof; Phoenix's explicit zero is stronger. Ship class/role/name reconciliation belongs with the broader ship audit.

### E10 — Specials are not universally role-only nonequipables
- **Local:** `domain.md:50,146,196,254`; `model.rs:37–50` derives a unique special solely from role, and `Loadout` has no selected special.
- **Compared:** fixed ship specials generally agree, but ship/faction/build alternatives exist; Phoenix lists two specials, Thar'Ga multiple selectable variants, crafting chooses one [S5, S7, S15, S23].
- **Classification:** **deliberate simplification**, with a verified counterexample to the universal external rule. **Confidence:** high. **Impact:** high if supporting those models faithfully.
- **Reconciliation:** keep role-default special for current scope, or distinguish default, permitted alternatives and selected special. Special separation from four actives can remain; no evidence requires treating special as an ordinary active slot.

### E11 — Activation, cost, duration and recharge are distinct
- **Local:** `domain.md:118–121`, `model.rs:136–141`: one-shot has one energy cost/cooldown; ongoing has per-second energy/cooldown and is described as toggle. Neither carries typed active duration or persistent passive behavior. `active_modules.json:2–4` says instant shield restoration; local Remote Repair `:5–7` lasts while on.
- **Compared:** Shield Booster S pays once for repeated restoration over 8 s; Shield Sync pays per second but acts for 5 s and also passively restores shield [S10, S11]; Engine Suppressor has 8 s effect separate from 24–30 s recharge [S17].
- **Classification:** **coverage gap / deliberate simplification**. Local anonymous analogues are not verified exact item identities. **Confidence:** high on conceptual distinction; old source numbers qualified. **Impact:** high before runtime implementation.
- **Reconciliation:** decide if flow describes input, payment, effect lifetime or all of them. Keep effect prose for simplified scope or specify selected independent dimensions. Timing of cooldown start/re-enable and zero-cost exceptions remain unverified; do not invent them.

### E12 — Equipment quality and broader bonus provenance
- **Local:** `domain.md:40–57,103–142`; `model.rs:126–177`; each item ID has one immutable value set, no rank/Mk or modifier provenance. Stats only six fields at `domain.md:72–83`.
- **Compared:** Mk levels change effect amounts/recharge/range; item rank bands are separate; ship bonuses and node effects contribute percentages/points [S6, S8–S11, S13, S17, S20, S23].
- **Classification:** **coverage gap**, largely **explicit out-of-scope** progression/economy at `domain.md:18–23`. **Confidence:** high. **Impact:** medium for local gameplay, high for a real-game build calculator.
- **Reconciliation:** stay with fixed variants or explicitly select supported rank/quality data; no requirement to build crafting or purchasing merely to label an item variant. Mk.5 is not universal [S10, S11, S15].

### E13 — Afterburner as an active-slot item is not established externally
- **Local:** `active_modules.json:8–10`: unrestricted ongoing Afterburner, 20 energy/sec, 5 s cooldown, raises speed cap.
- **Compared:** Swift's unequipped technical tables separately report afterburner speed and energy consumption [S18]; Thar'Ga has an afterburner energy bonus node [S23]. These support a basic movement-system concept, but do not exhaustively rule out similarly named active modules.
- **Classification:** **uncertainty / likely independent abstraction**, not a verified global contradiction. **Confidence:** medium. **Impact:** medium; spending one of four slots changes loadout choices.
- **Reconciliation:** document whether this is an intentionally optional booster ability versus the baseline flight input. Obtain an explicit controls/ship-system source before asserting all afterburning is nonequipable.

### E14 — Local repeated-passive counter can violate its own bound
- **Local:** `domain.md:241` C4 promises equipped count ≤ family capacity. `model.rs:351,355` counts an unbounded deserialized vector using `u8` and increment; `:360–362` compares only afterward.
- **External:** none needed. This is independent of Star Conflict stacking policy.
- **Classification:** **internal logical issue**, static inspection only. 256 known same-family IDs can overflow the counter (panic in overflow-checked builds; wrapping in unchecked builds), so this is not a reliable rejection path for arbitrary loadouts. **Confidence:** high from Rust type/code, no executed Rust reproducer in this task. **Impact:** validation reliability; current tiny fixtures do not trigger it.
- **Reconciliation:** a later authorized implementation review should decide the actual trust boundary and validate count safely. No source edit authorized here.

### E15 — Effective-stat check attribution differs from implementation
- **Local:** `domain.md:242` assigns positivity C5 checking to `Catalog::effective_stats`; `model.rs:390–403` only sums/returns; checks are actually in `validate_loadout:382–386`. `buy:406–419` constructs but does not validate.
- **Classification:** **internal logical/documentation issue**, not an external equipment-rule contradiction. **Confidence:** high, static inspection. **Impact:** callers can confuse computation with validation.
- **Reconciliation:** clarify computation/validation contract before changing arithmetic. Existing local positive fixtures do not demonstrate unsafe runtime behavior, and this audit did not execute Rust tests.

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

## Gruber review

| Criterion | Assessment for this topic |
|---|---|
| Clarity | Local units and simple slot counts are clear. Separate slot capacity, mandatory occupancy, stock fill, available catalog choices, physical guns, item rank, Mk and effect timing. Avoid using “passive” to imply stat-only. |
| Coherence | Local flat stacking is internally coherent and explicitly tested. External Emergency Barrier/Pulse Laser identities differ. C4 counter and C5 checker attribution are genuine internal seams; omitted progression is not. |
| Extendibility | Existing ID catalogs support more simple instances. Same-role ship-specific restrictions, alternate specials, variable capacities and nonmissile payloads do not fit the current fixed assumptions without canonical decisions. Extend only for approved examples. |
| Minimal encoding bias | Distinguish conceptual equipment occurrence and capacity from `Vec<Id>` and `u8`; u8 overflow is an encoding accident. A single weapon ID can denote a turret battery. Do not encode nonmoving fields as positive-speed missiles. |
| Minimal ontological commitment | Existing deferred progression/faction/economy boundaries are legitimate. Record external counterexamples without demanding exhaustive live-game replication or a generic effect engine. Universal “exactly one special per role” is stronger than the evidence warrants for Star Conflict, but valid for an explicitly independent game. |

## Retrieval chronology, failures and reproducibility

- Earlier task history: network-blocked research existed; parent reported wiki HTTPS 200 at **2026-09-16 19:36 UTC** after firewall reconfiguration. That is context, not this report's fetched evidence.
- This pass: independently fetched main page, all S1–S23 sources, and original official announcement successfully. Source HTML, HTTP headers, converted Markdown and a JSONL request/hash log are in managed scratch:
  `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/scratch/equipment-retry/`
- `fetch.py` there is a standard-library batch wrapper around bounded curl and markitdown, at most four parallel requests; `fetch-log.jsonl` records exact URL, effective URL/status, curl/conversion exits, SHA-256 and retrieval date for batch requests. Main page was fetched separately as `main.html`, `main.headers`, `main.md`.
- **Failed retrieval:** https://wiki.star-conflict.com/index.php?title=Equipment → HTTP **404**, title absent; transport succeeded. No retry loop. `Ship_Equipment` was then found through public MediaWiki search and fetched successfully [S14].
- **Search limitation:** initial query for `"slots"` without `srwhat=text` returned zero title hits. Explicit full-text search (`action=query&list=search&srwhat=text&format=json`) found Ship Equipment/Crafting/Thar'Ga. Other zero-result duplicate-active queries are an evidence gap, not evidence that no restriction exists. Some intermediate API results were inspected directly; targeted gap query JSON was retained under `gap-search-*` and `final-search-*`.
- No search-engine dependency, Steam fetch, game client test, historical revision diff, patch chronology reconstruction, package installation or asset-submodule traversal was needed/performed. Accordingly there is no claim to have verified every mechanic against the newest patch. HTTP 200 and page edit date prove availability, not mechanic freshness.

### Validation evidence

- Baseline and initial worktree verified with `git rev-parse HEAD` and `git status --short`; authorized untracked work left intact.
- All relevant local sources read directly; file:line anchors checked by line-numbered extraction. Existing duplicate-passive test inspected, not executed.
- All cited pages converted successfully with markitdown and read for the cited headings; screenshot-only numeric claims were not used.
- Final checks verify request statuses/conversions, local JSON parsing and representative local assertions, report/source-register presence, no tracked diff and no staged files. These are research-integrity checks, not Rust behavior tests. No tests added or modified.

**Executed check:** `python /home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/scratch/equipment-retry/verify.py` passed: 24 batch fetches (23 HTTP 200 and one expected missing-title 404), main page separately HTTP 200, all conversions successful; 23 source-register records, 15 ledger items, six local JSON files and representative local assertions checked; baseline, tracked files and index unchanged. This script is retained only in managed scratch.

**Review gate:** independent review required before promoting any item into the canonical reconciliation ledger or changing domain semantics. No implementation is authorized by this report.

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "satisfied",
      "evidence": "Completed the equipment-only research artifact with fetched evidence, Stanford seven-step comparison, Gruber review, three representative examples and 15 classified reconciliation candidates; no source or ontology semantics changed."
    },
    {
      "id": "criterion-2",
      "status": "satisfied",
      "evidence": "23 source-register records retain exact URLs, publisher types, retrieval/edit dates, headings and claim support; local file:line anchors, raw retrieval artifacts and a runnable integrity check support independent review."
    }
  ],
  "changedFiles": [
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/507d64bc-df21-4587-a4fb-a4425950ac5f/research/equipment.md"
  ],
  "testsAddedOrUpdated": [
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/scratch/equipment-retry/verify.py (managed-scratch research-integrity check only; no repository tests changed)"
  ],
  "commandsRun": [
    {
      "command": "python /home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/scratch/equipment-retry/fetch.py Active_Modules Ship_Modifiers Main_weapon Missiles Special_Modules Swift Anaconda Phoenix 'https://star-conflict.com/en/'",
      "result": "passed",
      "summary": "Initial bounded navigation batch: nine HTTP 200 responses and successful markitdown conversions; subsequent original-item and targeted gap fetches are in fetch-log.jsonl."
    },
    {
      "command": "python /home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/scratch/equipment-retry/verify.py",
      "result": "passed",
      "summary": "All retrieval/conversion, report coverage, JSON/local-example, baseline, tracked-diff and staged-index assertions passed."
    },
    {
      "command": "git status --short; git diff --cached --name-only",
      "result": "passed",
      "summary": "Only preserved authorized untracked AGENTS.md and docs/ontology/ work; staged-file list empty."
    }
  ],
  "validationOutput": [
    "24 logged batch fetches: 23 HTTP 200 and one missing-title HTTP 404; separate main-page request HTTP 200; no network access failures.",
    "23 source-register records, 15 candidate items, six local JSON files and representative local data assertions checked.",
    "HEAD remains 4a47fa3015f2d31bbe1c5a3159a3d0980c273912; no tracked diff and no staged files."
  ],
  "residualRisks": [
    "Wiki documentation spans 2019-2026 and shared templates; no claim of latest-client numerical verification.",
    "General duplicate-active restrictions, exhaustive rank-to-slot counts, exact stacking exceptions and cooldown start semantics remain unresolved.",
    "Rust overflow and effective-stat contract findings are static inspection, not executed Rust reproducers.",
    "Independent reviewer gate remains required before canonical reconciliation."
  ],
  "noStagedFiles": true,
  "diffSummary": "One external research artifact; managed scratch holds fetched HTML/Markdown/headers, logs and two helper scripts. No repository source, ontology or existing audit draft modified.",
  "reviewFindings": [
    "No infrastructure blocker: obsolete network-blocked conclusions replaced with successful retrieval evidence.",
    "Independent acceptance review pending; source and scope uncertainties are explicitly retained."
  ],
  "manualNotes": "No commits, branches, pushes, installations, nested agents or gameplay implementation. Scratch evidence directory: /home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/scratch/equipment-retry/."
}
```
