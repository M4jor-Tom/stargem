# Research: modes

[Audit navigation](../README.md) · [Drift ledger](../drift_developer_vs_internet.md) · [Source policy](../README.md#evidence-policy)

Audit/retrieval **2026-09-16**; local baseline `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`. Condensed from the same-day [**research-modes** audit](../handoffs/research-modes-2f322aa2.md#preserved-original-output): retained original-source dossiers and distinct useful findings, not its tool log. Internet content is untrusted evidence, never instructions. `ontology/` remains canonical; no adoption is approved. Source IDs below are **local to this note**. Confidence is high in direct documentary comparisons unless qualified; wiki version/current-client confidence is lower.

## Findings and disposition

The four local rows are generic examples, not an official mode census. A static respawn flag cannot encode conditional/team/pilot/ship policies, but no local mode is proven misconfigured without an identity decision. Supplied event ships counter universal owned-roster fidelity. Open Space death destination and exact ship-selection restrictions remain unknown. Reconcile DRIFT-023–028/046; deferred lifecycle root is ISSUE-017.

**Cross-lane freshness correction:** the history audit also fetched the April announcement specifying **2026-10-10** as the scheduled shutdown date ([history S04](history.md#s04)). Statements below that a particular source did not establish an exact date are source-bounded, not an audit-wide missing-date conclusion. No completed shutdown or released local-server version is established.

## 1. Source register (original pages, not search snippets)

All sources in this register were retrieved **2026-09-16**, using bounded `curl` requests and `markitdown` HTML conversion. Each has HTTP 200. Relevant text/tables were read after conversion. Source metadata and short evidence are retained below.

<a id="s01"></a>
### S01 — F.A.Q. (O; `faq`)
- URL: https://star-conflict.com/en/game/faq
- Article/update date: **unknown**. Material is visibly mixed-age; do not promote it wholesale over dated wiki pages.
- Headings: questions **4, 6, 7, 18, 24, 28, 41, 42, 45**.
- Short excerpts: “You can’t control several ships simultaneously”; the world is “a system of sectors connected with gates” plus battle instances entered and left after combat.
- Supports: one controlled ship per pilot at a time; distinction between open world and instanced battles; three basic categories PvP/PvE/Open world, plus configurable Custom battle; dockable stations; squads up to four and wings up to eight or twelve; station protection as a PvP condition.
- Specific older claims preserved for comparison: normal PvE is four players/three stages; a twelve-person raid fights a giant alien ship; three large dockable stations; ships other than DLC/Premium can be sold. These statements are not evidence that all later missions have three stages, exactly three stations still exist, or current sale rules are unchanged.

<a id="s02"></a>
### S02 — Star Conflict on Steam (Store; `steam`)
- URL: https://store.steampowered.com/app/212070/Star_Conflict/
- Publisher shown: **Gaijin Network Ltd**; developer **Star Gem Inc.** Release date displayed: **2013-02-27**; description update date **unknown**.
- Headings: **About This Game / The whole world for PVP and PVE! / Key Features**.
- Short excerpt: “Unique sandbox mode with extensive PVP and PVE capabilities.”
- Supports: correct product identity; broad modes; undocking from a station, collecting resources, fighting other players or grouping against pirates/aliens. Marketing “command your own fleet” is not evidence of simultaneous multi-ship control; S01 explicitly denies that.

<a id="s03"></a>
### S03 — PvP Arena (W; `pvp`)
- URL: https://wiki.star-conflict.com/index.php?title=PvP_Arena
- Last edit: **2026-04-09 16:21**; observed revision **16360**.
- Headings: **Intro; PvP Arena game modes; Brawls; Locations**.
- Short excerpts: Beacon Hunt's controlling team “can not respawn until control is lost or new beacon is activated”; Four lives gives “3 attempts to relaunch the ship, regardless of the number of ships in the slots.”
- Supports: regular mode names/objectives and rank bands; Arena clearance level 3; dynamic ordinary teams described as 3v3 growing to 10v10 during a three-minute joining period; conditional respawn; per-player versus per-ship life limits; supplied-ship brawls; seasonal/unavailable mode status. The page itself warns rank availability and maps can change irregularly/between patches.
- Source conflict to preserve: introductory “all players are always split into 2 teams” is too broad for its own brawl entries (Last man standing, Fight Club free-for-all; Orion's Belt three teams). Apply that introduction only to its ordinary Arena context, not all PvP experiences.

<a id="s04"></a>
### S04 — Missions (W; `pve`)
- URL: https://wiki.star-conflict.com/index.php?title=Missions
- Last edit: **2025-11-01 16:52**; revision **16024**.
- Headings: **Intro; Mission List**.
- Short excerpts: missions are passed by “a team consisting of 4 pilots”; destroyed allies can be recovered by approaching their wreck if reconstruction has not already begun.
- Supports: clearance level 4; ship ranks 1–17; progressively unlocked difficulty starting at 1 (described as continuing to infinity); level chosen on ship-selection screen; PvE-only upgrade bonuses/Insignia; full four-pilot squad commander selects a specific mission, solo queue chooses among two currently random missions; several conditional stages; self-recovery no more than five times (premium once free); all-player death starts a failure countdown. This is **not** proof that ally recovery consumes the same budget, or that switching to another ship is allowed on every ordinary mission reconstruction.
- Mission list distinguishes ordinary raids and operations by mission title, not by a separate compulsory game-mode category.

<a id="s05"></a>
### S05 — SPECIAL OPERATIONS (W; `specops`)
- Requested URL: https://wiki.star-conflict.com/index.php?title=Special_Operation
- Wiki redirect target: https://wiki.star-conflict.com/index.php?title=SPECIAL_OPERATIONS
- Last edit: **2021-02-18 11:46**; revision **11622**.
- Headings: **Information; List of spec ops**.
- Short excerpt: “If your ship was destroyed, you can be reborn 2 times and change the ship if needed.”
- Supports: special PvE tasks for groups of 10–12 ships; two post-destruction returns with ship change; performance-dependent dynamic difficulty; The Alien Destroyer (7–17, sub-bands 7–9/10–12/13–17), The Defiler (10–17), Return to Eden (13–17).
- Scheduling text says one-hour windows with hour breaks; general “alternate every other day” coexists with a weekday table, including all three on Sunday. Preserve that imprecision and the 2021 age; **not a verified 2026 queue schedule**.

<a id="s06"></a>
### S06 — PvE mission «Temple of Last hope» (W; `temple`)
- URL: https://wiki.star-conflict.com/index.php?title=PvE_mission_%C2%ABTemple_of_Last_hope%C2%BB
- Last edit: **2023-04-06 09:18**; revision **13441**.
- Headings: **Intro; Stages; Special enemies**.
- Short excerpt: “The stage is only one”; enemies appear “infinitely in small groups” and become stronger with each wave.
- Supports: a real near-endless, one-stage defensive mission; clearance level 5, ship ranks 1–17; all-ship loss fails the mission. S04 identifies its objective as protecting the generator. This is a suitable *example* for comparison with local `waves-survival`, not proof they denote the same mode. One stage contradicts a universal reading of the older official FAQ's three-stage description.

<a id="s07"></a>
### S07 — Return to Eden (W; `eden`)
- URL: https://wiki.star-conflict.com/index.php?title=Return_to_Eden
- Last edit: **2024-06-28 12:27**; revision **15167**.
- Headings: **Intro; Task; The Devourer; Stages**.
- Short excerpt: contract “valid for groups of 10 to 12 pilots”; cover dreadnought **Defiant** while scientists obtain data.
- Supports: clearance level 7 and ship ranks 13–17 are separate gates; a real cooperative multi-stage Special Operation; players damage growths, collect pearls and activate the dreadnought's main gun. The page labels five stages, including a repeated final sequence. Objective entities and NPC dreadnought are not additional player-owned deployed ships. Fine-grained strategy is wiki guidance, not new local requirements.

<a id="s08"></a>
### S08 — Open Space (W; `open`)
- URL: https://wiki.star-conflict.com/index.php?title=Open_Space
- Last edit: **2022-09-17 09:10**; revision **12987**.
- Headings: **What is Open Space; Overview / Navigation; Station Protection (PvE-mode); Loot**.
- Short excerpts: “All zones are separated by warp gates”; players can undock from their current faction's station, fight others or team up.
- Supports: zone/sector graph; travel between stations; map warp to unlocked sectors (home/corporation sectors free, other sectors cost iridium according to this revision); PvE and PvP coexist inside Open Space; station protection disabled in PvP zones and restored on return to non-PvP zones; both players need it off to damage one another; destroyed player cargo drops as loot.
- Explicit history: Alien Destroyer/alien invasion activity is described in past/present narrative but then says the attack was stopped and the destroyer is no longer seen. **Do not report that event as active in September 2026.**
- Does **not** specify ordinary Open Space respawn destination, reconstruction limits or which ship slot is eligible at undock.

<a id="s09"></a>
### S09 — Station 'Guardian-17' (W; `station`)
- URL: https://wiki.star-conflict.com/index.php?title=Station_%27Guardian-17%27
- Last edit: **2021-05-19 07:33**; revision **11756**.
- Heading: **Information** table.
- Precise paraphrase: location in Former Imperial sectors / Ontregos; PvE-mode active; clearance minimum 3; transitions to Ellydium Theta **one way**, Imperial Transport Hub, Tracking Station, Ontregos Pass and Corporate Shipyard (Empire); not present in PvP Arena or Missions.
- Supports: an actual station-sector example, clearance-controlled locations, and **potentially directed** travel edges. A location can be in Open Space without being an Arena/PvE mission map; “station,” “sector/location,” and “battle instance” are not interchangeable.

<a id="s10"></a>
### S10 — Community (W; `community`)
- URL: https://wiki.star-conflict.com/index.php?title=Community
- Last edit: **2024-03-14 13:16**; revision **14839**.
- Headings: **Squad; Wing; Corporation**.
- Short excerpt: “A Squad is a temporary group of players flying into battle together.”
- Supports: squads 1–4; wings described as up to 8–12, with use in Sector Conquest, CO-OP and Special Operations; corporations persistent social groups distinct from those temporary parties. S01 additionally names Attacked Sectors for wings. These texts do not establish a single universal wing limit for every mode, nor all contemporary Open Space grouping restrictions.

<a id="s11"></a>
### S11 — Advancement (W; `advancement`)
- URL: https://wiki.star-conflict.com/index.php?title=Advancement
- Last edit: **2025-11-01 17:00**; revision **16026**.
- Headings: **Clearance level required for different unlocks; Rank; Ships and Experience**.
- Short excerpt: “12: 4-th combat slot.”
- Supports: clearance controls mode/sector access and unlocked deployed-slot count; ranks differ from clearance and individual ship experience; prerequisite ships unlock non-premium purchases. Listed unlocks: Arena 3, Missions 4, Brawls/Custom Battle 5, Special Ops/Tournament/Conquest 7, Portals 10, fourth slot 12, ship preset saving bar 15. High-rank ship access is also gated: 10–15 at clearance 5, 16–17 at 10. These are dated wiki statements; progression remains out of local scope.

<a id="s12"></a>
### S12 — Power, defence and difficulty levels in PvE missions (W; `difficulty`)
- URL: https://wiki.star-conflict.com/index.php?title=Power,_defence_and_difficulty_levels_in_PvE_missions
- Last edit: **2023-09-26 12:47**; revision **14242**.
- Heading: initial **Level / Attack boost / Defence boost / Enemy att/def boost** table.
- Precise paraphrase: row 2 lists player attack ×1.02, defence ×1.03 and enemy attack/defence ×1.15, in different columns. Player columns continue to level 300; enemy columns are blank after 100.
- Supports: player PvE bonus levels and enemy difficulty scales are distinct, not ship rank or one shared multiplier. The table's end/blank cells do **not** prove a hard game cap or a formula for missing values. No numeric balance imported into the ontology.

<a id="s13"></a>
### S13 — Skirmish (CO-OP) (W; `coop`)
- URL: https://wiki.star-conflict.com/index.php?title=Skirmish_(CO-OP)
- Last edit: **2019-10-23 06:17**; revision **10052**.
- Heading: **Information**.
- Short excerpt: one team is players, the enemy team “AI controlled bots”; gameplay follows Arena rules.
- Supports: objective/ruleset and opponent kind are separable; ranks 1–17; wings of eight according to this old page. Co-op is not synonymous with scripted four-person Missions. Mode availability in the 2026 service is not independently verified here.

<a id="s14"></a>
### S14 — FAQ (W; `wikifaq`)
- URL: https://wiki.star-conflict.com/index.php?title=FAQ
- Last edit: **2023-08-15 16:39**; revision **13783**.
- Explicit warning: **“THIS FAQ IS NOT UP TO DATE. WORKS IN PROGRESS.”**
- Headings: **In the hangar / Ships; Matches / Battle Modes**.
- Short excerpt: choose a battle ship by selecting “a ship slot on the left side of the Hangar” and then a ship from the Ship Tree.
- Supports **historical/low-confidence UI detail**: collection/tree versus occupied combat slots; moving the same ship to a different slot removes its earlier slot assignment; ship selling/rebuying existed in this description. Do not transplant its old “Arcade” versus “Regular” respawn taxonomy into the current mode list. Sale prices, implant rules and old tier restrictions are not adopted as current evidence.

<a id="s15"></a>
### S15 — Maintenance (W; `maintenance`)
- URL: https://wiki.star-conflict.com/index.php?title=Maintenance
- Last edit: **2023-09-26 12:53**; revision **14247**.
- Heading: **Repairing**.
- Short excerpt: “If a ship reaches 0 durability it must be repaired before it can be fielded in battle again.”
- Supports: destruction does not necessarily delete an owned ship; repair eligibility/durability is distinct from combat hull HP, with premium exemption described. This is an economy/progression-adjacent deferred feature, not grounds to implement repairs in the local game.

<a id="s16"></a>
### S16 — Star Conflict: local server and a new indie game set in the same universe (O; `shutdown`)
- URL: https://star-conflict.com/en/news/3933-star-conflict-local-server-and-a-new-indie-game-set-in-the-same-universe-en
- Published: **2026-09-14**.
- Headings: introductory announcement; **Special version of Star Conflict**.
- Short excerpt: “Very soon, the game servers will be shut down”; special version's feature “will be the ability to run a local server.”
- Supports: forthcoming service transition, not an already-released local server; version/fidelity choice needs a date. The separate Burrownauts announcement is deliberately excluded from gameplay comparison.

<a id="s17"></a>
### S17 — Star Conflict 1.14.15. “Quarter for thirty” marathon update (O; `marathon`)
- URL: https://star-conflict.com/en/news/3934-star-conflict-1-14-15-quarter-for-thirty-marathon-update-en
- Published/patch date: **2026-09-11**; version **1.14.15**.
- Headings: opening; **Assignments for all players; Two blocks of PvP and PvE assignments given out randomly**.
- Short excerpt: assignments contain puzzles “within the Open World.”
- Supports: very recent official use of PvP/PvE and **Open World** terminology; the local phrase Open World is not inherently wrong merely because the wiki mode is titled Open Space. A seasonal marathon is an assignment/event layer, not necessarily a new combat ruleset. This patch does not revalidate every older mode rule.

<a id="s18"></a>
### S18 — Ship (W; `ship`)
- URL: https://wiki.star-conflict.com/index.php?title=Ship
- Last edit: **2025-12-28 15:17**; revision **16206**.
- Heading: **Technological level (old)**.
- Precise paraphrase: former Tier I–V terminology is explicitly marked old; the T2 example corresponds to ranks 4–6.
- Supports: avoid confusing obsolete tiers, ship rank, clearance, ship experience, PvE upgrade level and wave index. Size/role taxonomy itself belongs to the other audit topic and is not expanded here.

## 2. Mode and deployment comparison

### 2.1 Ordinary Arena and brawls are not one timeless list

The following is **the S03 wiki revision's classification**, not a tested September 2026 queue schedule. Regular Arena is described as two teams with dynamic joining; brawl-specific limits override that general description. The page says rank-to-mode availability is variable. “Not stated” is not a guess of unlimited respawns.

| Documented name / placement | Objectives and teams | Death / ship selection | Access / status qualifier |
|---|---|---|---|
| **Team Battle**, ordinary Arena | Deplete reinforcement points; higher remaining points wins at timeout. Ordinary death loses 1 team point, death in own spawn loses 3. Team boosts exist. | Reinforcement scoring is described; no complete ship-selection/respawn timing rule is stated in this section. | Listed ranks 1–17. Closest researched analogue to local Team Deathmatch, not an approved alias. |
| **Domination**, ordinary Arena | Three initially neutral beacons; teams start with 999 control points; fewer controlled beacons drains points; kills cost enemy 10 points. | Capture and drone defence are objectives, not extra player lives. Section does not give a complete respawn rule. | Listed ranks 1–17. |
| **Beacon Hunt**, ordinary Arena | One active beacon among A/B/C; each team starts at 200 control points; controlling team drains opponent. | **Controlling team cannot respawn** until control is lost/new beacon activates. | Listed ranks 4–17. A mode-level `true` cannot alone answer eligibility now. |
| **Detonation**, ordinary Arena | Three stations per team; deliver/plant neutral EM bombs; stations only destroyed by planting; station count then kills break ties. | Carrier death drops bomb; cloak/invulnerability/warp effects can also drop it. No complete respawn rule given here. | Listed ranks 4–17. Objective stations are not necessarily dockable home stations. |
| **Four lives**, ordinary Arena | Capture all enemy beacons or eliminate all ships; beacon count then kills at timeout. | **Three relaunch attempts per pilot regardless of slot count** (four total sorties including initial launch). | No rank range stated in this section. Distinct from per-ship exhaustion below. |
| **Beacon capture**, brawl; also used in Portals per source | Similar beacon/elimination objective. | **Each ship usable once**; after destruction choose another. Up to four lives if four ships equipped. | Listed under Brawls, not a blanket removed mode. |
| **Combat Recon**, brawl | Kill enemy captain, then remaining pilots; captain survival then kills decide timeout. | Captain's death disables teammates' further respawn. Captain is a session role, not a ship role. | Brawl in this revision; older FAQ lists Combat Reconnaissance without this distinction. |
| **Survival**, brawl | Destroy as many enemies as possible; team reinforcement points; damage tripled. | Choose among **four provided ships**, same selection for everyone. | **PvP**, not local PvE Waves Survival. |
| **Spaceball**, brawl | **3v3**; first to five goals. | Special sports ships/Harpoon, not normal owned loadouts. | Brawl; no audit-date active rotation established. |
| **Last man standing**, brawl | Every pilot for themselves; shrinking playable area; survive longest. | Own ship selected in hangar; random spawn; destroyed ship cannot be taken again. | Free-for-all, not universally two teams. Do not infer extra alternate-ship lives beyond text. |
| **Beetle in the Anthill**, brawl | One alien ship against other pilots; alien destroys generator/opponents, defenders stop it/survive ten minutes. | Ships selected once, **no respawns**; alien side chooses provided alien types. | Asymmetric PvP. |
| **Close encounter**, brawl | Attack/defend three navigation stations; special bomb. | Provided upgraded ships; unlimited returns with destroyed ship. | **Temporarily unavailable** per source, not proved permanently removed. |
| **Fight Club**, brawl | Free-for-all; most kills at timeout or first to 30; own ship and seed-chips. | Respawn details not established by this section. | **Temporarily unavailable**. |
| **Orion's Belt**, brawl | **Three teams**, first to 30 points or most kills at timeout. | Own ships, unlimited reuse after destruction. | Temporarily unavailable in April S03; the later [history S06](history.md#s06) announcement schedules **2026-06-16–2026-06-30**. Scheduled return, not observed occurrence or September availability. |
| **Paper Conflict**, event brawl | Four players per team, Domination objective. | Special paper ships. | April Fool's classification in S03; [history S06](history.md#s06) also schedules it for **2026-09-22–2026-10-05**, so not an exclusive annual window. |
| **Curse of the Leviathan**, event brawl | Same starting ships; randomly infected pilot converts others to its side; survivors await rescue. | Teams can change during battle. | Halloween-only in April S03; the later [history S06](history.md#s06) announcement schedules **2026-07-28–2026-08-10**. Dated exception, not observed occurrence or September availability. |
| **Spherical Conflict**, event brawl | High-inertia spherical ships, collision-focused combat. | Experimental provided fleet. | Explicit **2026-04-09–2026-04-21** window; do not list as active on audit date. |

Brawls are described as time-limited daily rotations from clearance level 5. Do not hardcode the hours from a wiki paragraph as current availability. Portals/Tournaments/Sector Conquest are discoverable named activities, but this pass does not establish their complete contemporary rules. Custom battle, per S01, can configure PvP/PvE; therefore mode template and queue/custom configuration should not be conflated if fidelity is later requested.

### 2.2 PvE, raids and waves

**Ordinary Missions [S04]** are four-pilot cooperative scenarios with objectives, conditional stages and reconstruction. Rank 1–17, pilot clearance, mission-specific access level and selected mission difficulty are independent axes. Difficulty unlocks by finishing earlier levels; player power/defence improvements apply specifically in PvE. The documented five-self-recovery cap is not a rule for every PvE activity.

**Special Operations [S05,S07]** are separate cooperative group activities: 10–12 pilots, operation-specific rank range, two returns and possible ship change. The word “operation” in an ordinary mission title does not automatically make it a Special Operation. Examples in S04: Operation «Crimson Haze» (minimum clearance 6), Operation 'Ice Belt' (7), Operation 'Monolith' (8). Likewise **Pirate Fort Raid** is an ordinary mission-list entry (minimum clearance 12), not proof of a twelve-player raid; “raid” is not a cardinality type by itself.

**Waves [S04,S06]** are encounter structure within a mission. Temple of Last hope is one almost-endless stage with increasingly strong waves; Captured Dreadnought is separately described as having endless arriving enemies in all its stages. Neither proves that all PvE is wave survival. **Skirmish (CO-OP) [S13]** reuses PvP-style objectives against bots, which is a third important distinction from ordinary Missions and Special Operations.

### 2.3 Open Space, docking, death and groups

- **Confirmed at documented-source level:** connected sector/location graph, warp gates, stations from which players undock, movement between stations, both player and AI opponents, safety/protection conditions, cargo loss on player destruction, temporary groups and persistent corporations [S01,S02,S08,S09,S10]. Location gates can be **one-way** [S09]; do not infer all travel relations symmetric.
- **Partial agreement with C12:** undocking from a current station matches the local statement. S01 supports only one controlled ship at once. The exact rule “currently selected ship” in `ontology.md:65` / `domain.md:249` is **plausible but not independently specified by the retrieved Open Space text**. Keep it as a local rule, not a newly verified live-game claim.
- **Not established:** whether selected ship must occupy an unlocked combat slot; whether another owned ship can be selected while undocked; docking interaction/range/cooldown; exact respawn destination after death; whether in-place reconstruction or consumables are available in Open Space; cargo recovery exceptions; party/wing cap and join restrictions specifically for current Open Space. A successful station-page fetch does not answer these.
- Sector content and interiors remain local out-of-scope. A minimal future `current_station`/`current_sector` relation could support C12 without modeling interiors, but no such semantic addition is approved by this audit.
- Squad/wing membership is not the same as session team. Friends may enter together; a team can contain more players than one squad, and event rules can reassign sides. NPC ships/objectives are not all owned player-ships.

### 2.4 Hangar, roster and progression

The narrative already says **“up to 4 Ships in his Hangar, leaving the other Ships unused”** (`ontology.md:61`). This agrees conceptually with a selected deployment roster drawn from a larger owned collection, not a four-ship ownership cap. `Hangar::validate(owned)` separately receives the owner's ships (`model.rs:215–225`), reinforcing that separation.

“Hangar” nevertheless names both a whole UI/location in sources and the local four-slot object. Use **combat/deployment slots** as a clarifying gloss, not an automatic schema rename. Four slot positions can exist while fewer are unlocked; S11's clearance-12 fourth-slot unlock is a progression limitation, not disproof of maximum four. Existing local scope excludes progression. No evidence here fixes the unlock schedule for the first three slots.

The sources also distinguish individual ship experience, prerequisite purchases, repairability and sale. These can affect deployment eligibility without changing the maximum roster capacity. The local permanent ownership rule and pure `buy` helper are deliberate narrower rules until their fidelity is decided. S15's destruction/repair distinction supports separating owned ship identity from one destroyed runtime incarnation.

## 6. Conflicting evidence and unanswered reconciliation questions

Meaningful conflicts retained rather than silently resolved:

- **S01 three-stage normal PvE versus S06 one-stage Temple / S07 five labelled Special Operation stages.** Different scopes and document ages; reject universal three-stage inference, not the authenticity of the older FAQ.
- **S03 intro always two teams versus S03 specific free-for-all/three-team/infection brawls.** Scope general description to ordinary Arena; preserve exceptions.
- **S14 explicitly outdated Arcade/Regular distinction versus S03's named ordinary/brawl/event taxonomy.** Historical vocabulary, not a current two-mode taxonomy.
- **S05 alternating schedule wording versus its weekday table**; neither independently verified for September 2026.
- **S08 historical alien invasion/world boss paragraphs explicitly say the attack stopped.** Do not confuse Open Space Alien Destroyer event status with the separately listed Special Operation **The Alien Destroyer** in S05.
- **S01 undated official FAQ / S02 store marketing versus recent S16 service transition.** Marketing availability and downloadable pages do not establish indefinite live service. Conversely, upcoming shutdown does not prove servers already shut on the audit date.

Product questions for the reconciliation ledger, **not questions requiring user interruption**:

1. Is the target an independent Star Conflict-like game, a specifically dated live-game snapshot, or eventual local-server behavior? Which fidelity reference/build?
2. Are the four local modes examples, a complete launch set, or placeholders for real named rulesets?
3. Does `respawn_allowed` mean any possible return, automatic return, or current eligibility? How should ordinary resurrection differ from taking another owned ship?
4. Are provided-ship/event modes intentionally excluded by C12, or does it need a narrowly stated exception?
5. Does “Hangar” intentionally mean the four-slot combat roster? Are all slots unlocked by design because progression is excluded?
6. Is permanent ownership a local design rule, or intended to mirror a game where selling may be possible?
7. Does “per match” include an Open Space session? Which identity survives death, docking and sector transition?
8. What exact Open Space selection/death/respawn semantics are intended? Further evidence is necessary before asserting Star Conflict fidelity.
9. Which restrictions—clearance, rank, mission difficulty, group size—must future deployment validate despite wider progression/matchmaking remaining out of scope?
10. Are seasonal/unavailable modes research-only context, or intended playable catalog entries with availability/provenance?
