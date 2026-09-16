# Handoff: research-modes — 2f322aa2

Checkpoint: 2026-09-16, user-requested pause. Parent-curated from runtime records and actual files; not a new completed child report.

- **State:** COMPLETED; reuse output, do not rerun by default
- **Role:** `delegate`
- **Child run:** `2f322aa2-738e-47a4-81b6-ea7eb5b4bfd3`
- **Workflow / key:** `507d64bc-df21-4587-a4fb-a4425950ac5f` / `research-modes`
- **Mission:** `28a33a06-b296-4a78-8ae0-079881816489`
- **Repository:** `/home/theta/repos/stargem.nix`, branch `master`
- **Source baseline:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`
- **Session (machine-local, optional for recovery):** `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/2026-09-16T18-47-01-030Z_01a0ab8b-30a5-761c-90d3-cc64125796e1/62e2ab82-e2a7-47b9-a631-603f82967f14/run-0/session.jsonl`
- **Runtime status (machine-local):** `/tmp/pi-subagents-uid-1000/async-subagent-runs/2f322aa2-738e-47a4-81b6-ea7eb5b4bfd3/status.json`

## Authority and boundaries

Read root `AGENTS.md` and `docs/HANDOFF.md` before work. `ontology/` remains canonical and unchanged. This is documentation/research, not authorization to change gameplay. No commit/push/merge/install is authorized. One writer per checkout. Preserve untracked audit files: they are the current work, not disposable clutter. Other agents' full research is intentionally file-backed to protect parent context.

## Resumption (for Agent)

Research is complete for this pass. Reuse the original report below and `../research/modes.md`; do not refetch everything. Synthesis/review are still pending. Only resume this child for a specific missing citation, disputed claim or fresh evidence request. Keep publication/patch date distinct from retrieval date and deliberate local scope distinct from defects.

## Native continuation or fallback

Native resumability was verified with children.list at checkpoint. Recheck it in the next session; retained session availability can change.

```javascript
subagent({ action: "status", id: "2f322aa2-738e-47a4-81b6-ea7eb5b4bfd3" })
subagent({ action: "children.list" })
// Only when resumable, and no original writer is still active:
subagent({ action: "resume", id: "2f322aa2-738e-47a4-81b6-ea7eb5b4bfd3",
  message: "Read AGENTS.md, docs/HANDOFF.md and docs/ontology/handoffs/research-modes-2f322aa2.md. Continue ONLY the remaining work recorded there; preserve completed outputs and current files." })
```

If the retained session is missing or not resumable, do not claim a native resume or rerun all research. Launch a fresh `delegate` as a **same-role fallback continuation**, give it this handoff and the preserved reports, and start at the exact next step above. A resumed run may return a new run ID: record it and continue from the latest identity.

## Verification and acceptance

The original report below states exactly what this child checked and what remains uncertain. Completion of a research child does not mean that document synthesis, independent evidence review, ontology review or user reconciliation has completed. Follow [the main checkpoint](README.md) for dependency order and pending reviews.

## Preserved original output

Original managed artifact: `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/507d64bc-df21-4587-a4fb-a4425950ac5f/research/modes.md`

SHA-256 of original artifact: `680955ff36633172c8170df2baa9029ca4cbfdfa6e156342744da632a547461d`

The complete output is embedded below so recovery does not depend on session/temp-file retention. Original relative code paths refer to the repository root unless the report says otherwise. Historical access failures stay historical.

---

# Star Conflict modes, deployment and session ontology audit

Audit/retrieval date: **2026-09-16**. Repository: `/home/theta/repos/stargem.nix`; branch `master`; inspected HEAD `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`.

## Conclusions and authority

- **Network access works on this retry.** Official website, official-hosted wiki and Steam returned readable HTTP 200 pages. Earlier access-blocked conclusions are historical only, not the present research result. Search services were less useful; failures are recorded below.
- The ontology is explicitly **Star Conflict-like**, not a declared replica. Its four modes are local examples, not a verified enumeration of Star Conflict modes. `Team Deathmatch` resembles the documented `Team Battle`; `Waves Survival` must not be confused with the game's **PvP** brawl `Survival`; `Operation Scenario` is not established as an alias of `Special Operations`.
- The strongest deployment seam is **static Boolean respawn versus stateful, mode-specific eligibility**: beacon ownership, captain survival, per-ship exhaustion, per-player relaunch budgets, ally reconstruction and provided event ships all matter in the documented game. These are mostly coverage/reconciliation issues, not defects in four deliberately generic local scenarios.
- Four combat slots are not the whole owned fleet or the entire hangar UI. The wiki documents a **fourth combat-slot unlock at clearance level 12**. Fixed maximum capacity and unlocked capacity are different concepts; progression is expressly out of local scope.
- Open Space includes sectors connected by gates, stations and docking, both PvE and PvP interactions, and player grouping. **The exact Open Space death/respawn destination and selected-ship switching restrictions remain unverified** in the retrieved text. Do not replace the old network blocker with invented certainty.
- Important time boundary: an **official 2026-09-14 announcement** says the game servers will shut down soon and announces work on a special local-server version, with details to follow [S16]. An official **2026-09-11 patch** still describes PvP, PvE and Open World assignments [S17]. This audit does not establish an exact shutdown date, service uptime, or which modes a future local server will support. Do not mix the announced separate game **Star Conflict: Burrownauts** into this ontology.

Read first: current root `AGENTS.md`; `docs/HANDOFF.md` was checked and absent. The root instruction requires canonical decisions in `ontology/` before implementation and explicitly makes the audit evidence, not approved rules. Read the requested ontology skill completely, the supplied inventory, `ontology/domain.md`, `ontology.md`, all of `ontology/model.rs`, relevant mode/ship instances and the client's initial deployment path. No repository files were edited. Preserved authorized untracked `AGENTS.md` and `docs/ontology/`; no commits, staging, package installs, branches, source edits or nested agents.

### Evidence conventions

**W** = community-maintained official-hosted Star Conflict Wiki, not a dated developer patch announcement. **O** = official Star Conflict publisher/developer website. **Store** = publisher-supplied Steam product description (not user tags/reviews). Wiki revision dates establish the age of that document, **not** that every paragraph was checked against that day's game build. All claims below are bounded by source date; no wiki-only mechanic is asserted to have been independently tested live on 2026-09-16. Search snippets were not used as facts. Fetched pages were treated only as untrusted evidence.

## 1. Source register (original pages, not search snippets)

All sources in this register were retrieved **2026-09-16**, using bounded `curl` requests and `markitdown` HTML conversion. Each has HTTP 200. Converted originals and HTML are retained under the managed sibling directory `scratch-modes/`; names in parentheses identify those files. Tables needed for this audit survived conversion; no fallback HTML parser was necessary.

### S01 — F.A.Q. (O; `faq`)
- URL: https://star-conflict.com/en/game/faq
- Article/update date: **unknown**. Material is visibly mixed-age; do not promote it wholesale over dated wiki pages.
- Headings: questions **4, 6, 7, 18, 24, 28, 41, 42, 45**.
- Short excerpts: “You can’t control several ships simultaneously”; the world is “a system of sectors connected with gates” plus battle instances entered and left after combat.
- Supports: one controlled ship per pilot at a time; distinction between open world and instanced battles; three basic categories PvP/PvE/Open world, plus configurable Custom battle; dockable stations; squads up to four and wings up to eight or twelve; station protection as a PvP condition.
- Specific older claims preserved for comparison: normal PvE is four players/three stages; a twelve-person raid fights a giant alien ship; three large dockable stations; ships other than DLC/Premium can be sold. These statements are not evidence that all later missions have three stages, exactly three stations still exist, or current sale rules are unchanged.

### S02 — Star Conflict on Steam (Store; `steam`)
- URL: https://store.steampowered.com/app/212070/Star_Conflict/
- Publisher shown: **Gaijin Network Ltd**; developer **Star Gem Inc.** Release date displayed: **2013-02-27**; description update date **unknown**.
- Headings: **About This Game / The whole world for PVP and PVE! / Key Features**.
- Short excerpt: “Unique sandbox mode with extensive PVP and PVE capabilities.”
- Supports: correct product identity; broad modes; undocking from a station, collecting resources, fighting other players or grouping against pirates/aliens. Marketing “command your own fleet” is not evidence of simultaneous multi-ship control; S01 explicitly denies that.

### S03 — PvP Arena (W; `pvp`)
- URL: https://wiki.star-conflict.com/index.php?title=PvP_Arena
- Last edit: **2026-04-09 16:21**; observed revision **16360**.
- Headings: **Intro; PvP Arena game modes; Brawls; Locations**.
- Short excerpts: Beacon Hunt's controlling team “can not respawn until control is lost or new beacon is activated”; Four lives gives “3 attempts to relaunch the ship, regardless of the number of ships in the slots.”
- Supports: regular mode names/objectives and rank bands; Arena clearance level 3; dynamic ordinary teams described as 3v3 growing to 10v10 during a three-minute joining period; conditional respawn; per-player versus per-ship life limits; supplied-ship brawls; seasonal/unavailable mode status. The page itself warns rank availability and maps can change irregularly/between patches.
- Source conflict to preserve: introductory “all players are always split into 2 teams” is too broad for its own brawl entries (Last man standing, Fight Club free-for-all; Orion's Belt three teams). Apply that introduction only to its ordinary Arena context, not all PvP experiences.

### S04 — Missions (W; `pve`)
- URL: https://wiki.star-conflict.com/index.php?title=Missions
- Last edit: **2025-11-01 16:52**; revision **16024**.
- Headings: **Intro; Mission List**.
- Short excerpts: missions are passed by “a team consisting of 4 pilots”; destroyed allies can be recovered by approaching their wreck if reconstruction has not already begun.
- Supports: clearance level 4; ship ranks 1–17; progressively unlocked difficulty starting at 1 (described as continuing to infinity); level chosen on ship-selection screen; PvE-only upgrade bonuses/Insignia; full four-pilot squad commander selects a specific mission, solo queue chooses among two currently random missions; several conditional stages; self-recovery no more than five times (premium once free); all-player death starts a failure countdown. This is **not** proof that ally recovery consumes the same budget, or that switching to another ship is allowed on every ordinary mission reconstruction.
- Mission list distinguishes ordinary raids and operations by mission title, not by a separate compulsory game-mode category.

### S05 — SPECIAL OPERATIONS (W; `specops`)
- Requested URL: https://wiki.star-conflict.com/index.php?title=Special_Operation
- Wiki redirect target: https://wiki.star-conflict.com/index.php?title=SPECIAL_OPERATIONS
- Last edit: **2021-02-18 11:46**; revision **11622**.
- Headings: **Information; List of spec ops**.
- Short excerpt: “If your ship was destroyed, you can be reborn 2 times and change the ship if needed.”
- Supports: special PvE tasks for groups of 10–12 ships; two post-destruction returns with ship change; performance-dependent dynamic difficulty; The Alien Destroyer (7–17, sub-bands 7–9/10–12/13–17), The Defiler (10–17), Return to Eden (13–17).
- Scheduling text says one-hour windows with hour breaks; general “alternate every other day” coexists with a weekday table, including all three on Sunday. Preserve that imprecision and the 2021 age; **not a verified 2026 queue schedule**.

### S06 — PvE mission «Temple of Last hope» (W; `temple`)
- URL: https://wiki.star-conflict.com/index.php?title=PvE_mission_%C2%ABTemple_of_Last_hope%C2%BB
- Last edit: **2023-04-06 09:18**; revision **13441**.
- Headings: **Intro; Stages; Special enemies**.
- Short excerpt: “The stage is only one”; enemies appear “infinitely in small groups” and become stronger with each wave.
- Supports: a real near-endless, one-stage defensive mission; clearance level 5, ship ranks 1–17; all-ship loss fails the mission. S04 identifies its objective as protecting the generator. This is a suitable *example* for comparison with local `waves-survival`, not proof they denote the same mode. One stage contradicts a universal reading of the older official FAQ's three-stage description.

### S07 — Return to Eden (W; `eden`)
- URL: https://wiki.star-conflict.com/index.php?title=Return_to_Eden
- Last edit: **2024-06-28 12:27**; revision **15167**.
- Headings: **Intro; Task; The Devourer; Stages**.
- Short excerpt: contract “valid for groups of 10 to 12 pilots”; cover dreadnought **Defiant** while scientists obtain data.
- Supports: clearance level 7 and ship ranks 13–17 are separate gates; a real cooperative multi-stage Special Operation; players damage growths, collect pearls and activate the dreadnought's main gun. The page labels five stages, including a repeated final sequence. Objective entities and NPC dreadnought are not additional player-owned deployed ships. Fine-grained strategy is wiki guidance, not new local requirements.

### S08 — Open Space (W; `open`)
- URL: https://wiki.star-conflict.com/index.php?title=Open_Space
- Last edit: **2022-09-17 09:10**; revision **12987**.
- Headings: **What is Open Space; Overview / Navigation; Station Protection (PvE-mode); Loot**.
- Short excerpts: “All zones are separated by warp gates”; players can undock from their current faction's station, fight others or team up.
- Supports: zone/sector graph; travel between stations; map warp to unlocked sectors (home/corporation sectors free, other sectors cost iridium according to this revision); PvE and PvP coexist inside Open Space; station protection disabled in PvP zones and restored on return to non-PvP zones; both players need it off to damage one another; destroyed player cargo drops as loot.
- Explicit history: Alien Destroyer/alien invasion activity is described in past/present narrative but then says the attack was stopped and the destroyer is no longer seen. **Do not report that event as active in September 2026.**
- Does **not** specify ordinary Open Space respawn destination, reconstruction limits or which ship slot is eligible at undock.

### S09 — Station 'Guardian-17' (W; `station`)
- URL: https://wiki.star-conflict.com/index.php?title=Station_%27Guardian-17%27
- Last edit: **2021-05-19 07:33**; revision **11756**.
- Heading: **Information** table.
- Precise paraphrase: location in Former Imperial sectors / Ontregos; PvE-mode active; clearance minimum 3; transitions to Ellydium Theta **one way**, Imperial Transport Hub, Tracking Station, Ontregos Pass and Corporate Shipyard (Empire); not present in PvP Arena or Missions.
- Supports: an actual station-sector example, clearance-controlled locations, and **potentially directed** travel edges. A location can be in Open Space without being an Arena/PvE mission map; “station,” “sector/location,” and “battle instance” are not interchangeable.

### S10 — Community (W; `community`)
- URL: https://wiki.star-conflict.com/index.php?title=Community
- Last edit: **2024-03-14 13:16**; revision **14839**.
- Headings: **Squad; Wing; Corporation**.
- Short excerpt: “A Squad is a temporary group of players flying into battle together.”
- Supports: squads 1–4; wings described as up to 8–12, with use in Sector Conquest, CO-OP and Special Operations; corporations persistent social groups distinct from those temporary parties. S01 additionally names Attacked Sectors for wings. These texts do not establish a single universal wing limit for every mode, nor all contemporary Open Space grouping restrictions.

### S11 — Advancement (W; `advancement`)
- URL: https://wiki.star-conflict.com/index.php?title=Advancement
- Last edit: **2025-11-01 17:00**; revision **16026**.
- Headings: **Clearance level required for different unlocks; Rank; Ships and Experience**.
- Short excerpt: “12: 4-th combat slot.”
- Supports: clearance controls mode/sector access and unlocked deployed-slot count; ranks differ from clearance and individual ship experience; prerequisite ships unlock non-premium purchases. Listed unlocks: Arena 3, Missions 4, Brawls/Custom Battle 5, Special Ops/Tournament/Conquest 7, Portals 10, fourth slot 12, ship preset saving bar 15. High-rank ship access is also gated: 10–15 at clearance 5, 16–17 at 10. These are dated wiki statements; progression remains out of local scope.

### S12 — Power, defence and difficulty levels in PvE missions (W; `difficulty`)
- URL: https://wiki.star-conflict.com/index.php?title=Power,_defence_and_difficulty_levels_in_PvE_missions
- Last edit: **2023-09-26 12:47**; revision **14242**.
- Heading: initial **Level / Attack boost / Defence boost / Enemy att/def boost** table.
- Precise paraphrase: row 2 lists player attack ×1.02, defence ×1.03 and enemy attack/defence ×1.15, in different columns. Player columns continue to level 300; enemy columns are blank after 100.
- Supports: player PvE bonus levels and enemy difficulty scales are distinct, not ship rank or one shared multiplier. The table's end/blank cells do **not** prove a hard game cap or a formula for missing values. No numeric balance imported into the ontology.

### S13 — Skirmish (CO-OP) (W; `coop`)
- URL: https://wiki.star-conflict.com/index.php?title=Skirmish_(CO-OP)
- Last edit: **2019-10-23 06:17**; revision **10052**.
- Heading: **Information**.
- Short excerpt: one team is players, the enemy team “AI controlled bots”; gameplay follows Arena rules.
- Supports: objective/ruleset and opponent kind are separable; ranks 1–17; wings of eight according to this old page. Co-op is not synonymous with scripted four-person Missions. Mode availability in the 2026 service is not independently verified here.

### S14 — FAQ (W; `wikifaq`)
- URL: https://wiki.star-conflict.com/index.php?title=FAQ
- Last edit: **2023-08-15 16:39**; revision **13783**.
- Explicit warning: **“THIS FAQ IS NOT UP TO DATE. WORKS IN PROGRESS.”**
- Headings: **In the hangar / Ships; Matches / Battle Modes**.
- Short excerpt: choose a battle ship by selecting “a ship slot on the left side of the Hangar” and then a ship from the Ship Tree.
- Supports **historical/low-confidence UI detail**: collection/tree versus occupied combat slots; moving the same ship to a different slot removes its earlier slot assignment; ship selling/rebuying existed in this description. Do not transplant its old “Arcade” versus “Regular” respawn taxonomy into the current mode list. Sale prices, implant rules and old tier restrictions are not adopted as current evidence.

### S15 — Maintenance (W; `maintenance`)
- URL: https://wiki.star-conflict.com/index.php?title=Maintenance
- Last edit: **2023-09-26 12:53**; revision **14247**.
- Heading: **Repairing**.
- Short excerpt: “If a ship reaches 0 durability it must be repaired before it can be fielded in battle again.”
- Supports: destruction does not necessarily delete an owned ship; repair eligibility/durability is distinct from combat hull HP, with premium exemption described. This is an economy/progression-adjacent deferred feature, not grounds to implement repairs in the local game.

### S16 — Star Conflict: local server and a new indie game set in the same universe (O; `shutdown`)
- URL: https://star-conflict.com/en/news/3933-star-conflict-local-server-and-a-new-indie-game-set-in-the-same-universe-en
- Published: **2026-09-14**.
- Headings: introductory announcement; **Special version of Star Conflict**.
- Short excerpt: “Very soon, the game servers will be shut down”; special version's feature “will be the ability to run a local server.”
- Supports: forthcoming service transition, not an already-released local server; version/fidelity choice needs a date. The separate Burrownauts announcement is deliberately excluded from gameplay comparison.

### S17 — Star Conflict 1.14.15. “Quarter for thirty” marathon update (O; `marathon`)
- URL: https://star-conflict.com/en/news/3934-star-conflict-1-14-15-quarter-for-thirty-marathon-update-en
- Published/patch date: **2026-09-11**; version **1.14.15**.
- Headings: opening; **Assignments for all players; Two blocks of PvP and PvE assignments given out randomly**.
- Short excerpt: assignments contain puzzles “within the Open World.”
- Supports: very recent official use of PvP/PvE and **Open World** terminology; the local phrase Open World is not inherently wrong merely because the wiki mode is titled Open Space. A seasonal marathon is an assignment/event layer, not necessarily a new combat ruleset. This patch does not revalidate every older mode rule.

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
| **Orion's Belt**, brawl | **Three teams**, first to 30 points or most kills at timeout. | Own ships, unlimited reuse after destruction. | **Temporarily unavailable**, even though its maps remain in the table. |
| **Paper Conflict**, event brawl | Four players per team, Domination objective. | Special paper ships. | **April Fool's event only**. |
| **Curse of the Leviathan**, event brawl | Same starting ships; randomly infected pilot converts others to its side; survivors await rescue. | Teams can change during battle. | **Halloween only**. |
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

## 3. Candidate reconciliation ledger (no decisions applied)

Classification vocabulary: **verified contradiction** means an explicit incompatible statement, bounded to the cited document—not proof that the independent local design is erroneous; **deliberate simplification** means local scope/design explicitly permits less; **coverage gap** is unrepresented relevant structure; **uncertainty** means mapping or freshness not settled; **historical change** means temporal status matters; **internal logical issue** needs local inconsistency rather than absent external features.

| ID | Exact local claim and anchors | Sourced comparison | Classification / confidence | Impact and reconciliation question/options |
|---|---|---|---|---|
| M01 | `ontology/instances/game_modes.json:2`: `team-deathmatch`, name `Team Deathmatch`, pvp, respawn true; narrative `ontology.md:2`. | S03 calls ordinary reinforcement-point mode **Team Battle**; special scoring includes spawn-area deaths. No source establishes an exact synonym. | **Uncertainty / deliberate generic label**; high confidence in names, medium in intended equivalence. | Medium naming/rules risk. Keep independent name, or explicitly map to a dated Team Battle ruleset; do not silently rename IDs (domain.md:6 makes them stable). |
| M02 | `ontology/instances/game_modes.json:3`: `Waves Survival`, pve, true; `domain.md:258–260` defers wave generator. | S06 Temple is a real PvE waves example; S03 **Survival** is a PvP provided-ship brawl. | **Coverage gap + deliberate simplification**, high. | Medium. Is local wave survival an original scenario or Temple-inspired? Document that; never equate it with PvP Survival from the word alone. |
| M03 | `ontology/instances/game_modes.json:4`: `Operation Scenario`, pve, **false**; narrative `ontology.md:3`. | S05 Special Operations permits **two returns and ship change**; S04 ordinary Missions also support reconstruction. | **Uncertainty** of mapping; **verified contradiction only if** this local item is claimed to reproduce those documented modes. High textual, low mapping confidence. | High if used for fidelity. Keep an original no-respawn operation, or add/map specific real operation with an explicit death policy. No current claim proves that this boolean is a local bug. |
| M04 | `domain.md:155–160`; `model.rs:177–183`: game-mode has only id/name/category/`respawn_allowed: bool`; Q9 `domain.md:37`. | S03 Beacon Hunt depends on beacon state; Combat Recon on captain life; Four lives on pilot budget; Beacon capture on exhausted ship identities. S04 adds ally rescue/self-reconstruction distinction. | **Coverage gap / deliberate simplification**, high. | High runtime design seam. Does bool mean “ever supports respawn” or “allowed now”? If faithful modes enter scope, separate stable policy from session eligibility/budget; do not create many speculative fields for four generic examples. |
| M05 | C12 `domain.md:249`: “`pvp`/`pve`: deploy from hangar”; C11 at 248 requires held ships owned by user. | S03 Survival, Spaceball and event brawls use **provided** ships; alien player in Beetle also chooses special alien fleet. | **Verified contradiction against the documented universal PvP deployment claim**, high for cited revision, medium for audit-date availability; independent local design may deliberately exclude these modes. | High if all Star Conflict PvP is intended. Scope C12 to owned-roster modes or document provided-ship exception; do not manufacture owned PlayerShip records solely to preserve an overbroad rule. |
| M06 | C11 `domain.md:248`, slots definition 184, `model.rs:203–210`: exactly four optional positions; no unlock state. | S11 unlocks fourth combat slot at clearance 12. | **Deliberate simplification / deferred progression**, high. Not a contradiction: capacity ≠ availability. | Medium. Declare all four locally available, or add unlock state only if progression enters scope. Do not change four-slot maximum based on this evidence. |
| M07 | Q8 `domain.md:36` answered by slots/C11; `ontology.md:61` says other owned ships unused. | S14 historical UI and S03 per-ship brawl show deployment slots drawn from collection; S11 access rules can further restrict eligibility. | **Agreement plus coverage gap**, high for local separation; medium for dated UI details. | Medium clarity. Call the local Hangar a deployment roster; keep ownership separate. Q8 answers local eligibility, not every live-game access condition. |
| M08 | `domain.md:163,200`: “owned forever” / permanent ownership; narrative `ontology.md:12`. | S01 Q28 explicitly permits sale except DLC/Premium; S14 describes selling/rebuying but warns it is outdated. | **Documented-source contradiction / current uncertainty**; high that source conflicts, medium/low that current sale semantics remain. | Medium; scope economy excludes more than ship price. Keep permanent ownership as deliberate local rule or verify dated sale semantics before changing it. Combat death itself is not proof of permanent ownership loss. |
| M09 | `ontology/instances/game_modes.json:5`: open-world respawn true; C12 `domain.md:249`: selected ship/current station; selected-ship relation `domain.md:209` is 0..1. | S08 verifies undocking/travel/protection and cargo drop; S01 one controlled ship. Neither establishes the exact death destination or selected-slot restriction. | **Agreement + uncertainty**, high on partial evidence, intentionally no respawn conclusion. | High before implementing Open Space lifecycle. Preserve local rule; decide respawn/checkpoint/selection semantics explicitly, with further targeted evidence if fidelity required. |
| M10 | `domain.md:186–189`: ship-state “Extrinsic, per match”; no typed session/player participation/team/life identity. `model.rs:196–210` has only owned ship and roster. | S01 separates ongoing open world from entered/exited battle instances; S03–S07 require participant budgets, team objectives and multiple lives for the same pilot/ship. | **Coverage gap**, high; not a logical contradiction in intentionally future runtime prose. | High for later server design. Clarify scope of “match”; distinguish persistent ship from runtime incarnation and session participation before netcode, but no implementation now. |
| M11 | `domain.md:159`; `model.rs:71`: one category `pvp|pve|open-world`. | S08/S02 Open Space contains both PvE/PvP; S13 CO-OP reuses Arena objectives with AI opponents. | **Clarity/encoding uncertainty**, high. Categories are coherent if they mean entry contexts, not mutually exclusive opponent relationships. | Medium. Define category meaning; keep simple enum if an activity selector, or distinguish opponent kind from world/session context if required. No automatic need to replace enum. |
| M12 | `domain.md:19–23`: progression, AI, maps excluded/deferred; game modes lack ranks, minimum clearance, difficulty, party size, objective data. | S04–S13 document those restrictions and structures. | **Explicit out-of-scope plus coverage gaps**, high. | Low as current defect; high only with a future fidelity requirement. Decide whether mode eligibility is in scope even while broader progression/maps remain deferred. Missing a live feature is not an internal inconsistency. |
| M13 | Instances `ontology/instances/game_modes.json:2–5` have no provenance or availability interval; runtime category names generic. | S03 event window/unavailable notices, S08 stopped invasion, S16 planned service transition. | **Historical change / provenance gap**, high. | High risk of falsely calling all researched modes current. Choose reference date/build and keep event availability separate from reusable rulesets. Do not infer local-server feature parity. |
| M14 | `domain.md:4` links source narrative as `../onthology.md`; actual source is `ontology.md` at this baseline. | Local inspection and supplied inventory's rename evidence; no external claim needed. | **Internal documentary/reference issue**, high. Not a gameplay logical defect. | Low; fix link in an authorized documentation reconciliation pass. There is **no demonstrated internal logical contradiction** in the local mode instances themselves: the limitations above are intentional breadth or undefined runtime details. |

## 4. Ontology audit using the Stanford seven steps

### Step 1 — Scope and competency questions

Domain for this pass: **how a Star Conflict player enters an activity, selects/controls ships, belongs to a team/party, completes objectives and returns after destruction**, compared to the bounded local ontology. Users: later ontology reconciler, server/runtime designer and hangar UI author. Excluded: implementing mechanics, complete map catalogs, economy, matchmaking algorithms, source-wide balance or class-taxonomy audit.

| Competency question | Evidence-backed answer / local consequence |
|---|---|
| 1. Is a game category a specific objective ruleset? | No: S01 three basic modes, S03 several Arena rulesets, S13 objectives reused against AI. Define local category as context, not complete rules. |
| 2. Which ships may a participant choose? | Ordinary local roster works for many descriptions, but S03 provided-ship exceptions exist; S11 unlocked slots/access gates matter. Exact Open Space selection constraint remains unknown. |
| 3. Can a player respawn now, how often, and on which ship? | Mode/session/team/pilot/ship-dependent in S03–S05, not just one bool. Separate “permits some return” from current eligibility. |
| 4. How many pilots/teams are involved? | Missions four; Special Ops 10–12; Spaceball 3v3; ordinary Arena dynamic; some brawls free-for-all/three-team. No universal size. |
| 5. Is raid/wave/operation one class hierarchy? | No: raids can be mission names; waves are stage structure; Special Operations are a distinct activity. S04–S07 demonstrate overlap. |
| 6. What gates admission/difficulty? | Pilot clearance, ship rank, mission difficulty and purchased PvE bonuses differ. S11/S12; mostly explicitly deferred locally. |
| 7. Does ship death destroy ownership? | S15 repair system and multiple returns show runtime destruction need not remove collection entry; sale is separate and historically documented. |
| 8. Where do Open Space ships exist/return? | Sector graph and station undocking/docking supported; exact death return not verified. Do not claim complete answer to local Q9. |
| 9. Are party, team, corporation and controlled ship the same relationship? | No. S10 temporary squads/wings versus persistent corporations; S03 team rules/captain/infection; S01 one controlled ship. |
| 10. Which date/version do “current” mechanics mean? | Needs product decision: public service at audit date, an older snapshot, independent inspiration, or future local-server version. S16/S17 make this material. |

### Step 2 — Reuse existing representations

Reuse canonical `game-mode`, `player-ship`, `hangar`, and documented `ship-state`; model/JSON already encode persistent catalog and owned-ship references. The local inventory was read, not merely copied as external game evidence. No new library or external schema is required by this documentation audit. Real-game vocabulary is evidence to reconcile, not replacement authority. The source narrative describes a design; it is not an official game guide.

### Step 3 — Terms

- **Entities/nouns:** activity/mode, objective ruleset, session/instance, participant/pilot, team, squad, wing, corporation, owned ship, combat/deployment slot, provided ship, controlled runtime ship/life, station, sector/location, warp gate, beacon, captain assignment, bomb, mission, stage, wave, difficulty level.
- **Properties:** category/context, opponent kind, objective, rank band, clearance minimum, available/unavailable/event-limited, unlocked slot, remaining relaunches, exhausted ship, station protection, current location, selected ship, reconstruction pending.
- **Relations/actions:** owns, equips-in-slot, selects, joins, belongs-to-party, assigned-to-team, controls, deploys-as, destroyed, reconstructs, relaunches, captures, carries, docks, undocks, warps-to, unlocks, completes-stage.
- **Ambiguous terms to qualify:** hangar (UI/station versus deployment roster); operation (ordinary mission title versus Special Operations); survival (generic wave play versus named PvP brawl); rank (ship/faction rank versus clearance); level (ship experience, mission difficulty, upgrade level, wave number); station (home dock versus destructible objective).

### Step 4 — Hierarchy and relationships, not implementation proposals

`Mission` and `Special Operation` can both be cooperative activities; **wave is not a subtype of mission**, but part of an encounter/stage. A ship model is not a player-owned ship, and an owned ship is not its live spatial state. Party membership is not an is-a relationship to a team. A station sector is not automatically an instance of every battle mode using a similarly named map.

The current enum remains acceptable for a small entry-context vocabulary. Supplied event ships and NPC objectives do not justify forcing every combat entity into `PlayerShip { owner_id }`. If such activities are adopted later, ownership and current control would need separate meanings; no new class is authorized now.

### Steps 5–6 — Properties, references and cardinalities

These are **source-implied conceptual relations for review**, not approved additions:

| Relation | Cardinality / constraint supported or inferred | Evidence and qualification |
|---|---|---|
| Pilot → owned collection | Many available over progression; maximum not established. | S11/S14; local already separates owned slice from slots. Do not invent ownership cap. |
| Pilot → combat slot positions | Up to four selected ships documented; unlock availability can be lower. | S03/S11. Local fixed four optional positions are coherent. |
| Pilot → actively controlled ship | **At most one at a time**; zero while dead/not deployed is reasonable lifecycle inference, not an explicit numeric source statement. | S01 Q18; multiple stored ships ≠ multiple simultaneous player avatars. |
| Session → participants/teams | Activity-specific; ordinary missions four participants, Special Ops 10–12, etc. | S03–S07. Do not make all sessions exactly two teams. |
| Participant → deployed runtime incarnations | Multiple sequential incarnations possible; current controls at most one. | S03–S05; conceptual inference for storage/runtime identity. |
| Runtime incarnation → owned ship | One origin for ordinary owned-ship modes; provided-ship modes require another origin. | S03; not every combat entity is user-owned. |
| Participant → relaunch budget | Four lives: three returns after initial; Special Ops: two returns. | S03/S05. Budget belongs to participant/session, not immutable ship model. |
| Participant+ship+session → exhausted status | Beacon capture: each ship once. | S03. Distinct from per-participant total life budget. |
| Team+session → captain / beacon control | Captain assignment and objective state affect team respawn. | S03. Captain is not permanent pilot class or ship role. |
| Party → pilots | Squad 1–4; wing limit is mode-dependent 8 or 12 in texts. | S01/S10/S13. No universal 12-player squad. |
| Mission run → difficulty/stages | Selected difficulty separate from rank; stages can vary; Temple one stage. | S04/S06/S07. No universal three-stage constraint. |
| Sector/location → adjacent destination | Multiple possible; some directed/one-way. | S08/S09. Gate graph not necessarily undirected. |
| Runtime ship → current sector/protection | Needed to express documented navigation/PvP protection; exact formal cardinality not prescribed by sources. | S08. Local has no such runtime fields yet. |
| Pilot → current station / selected ship | Local selected relation 0..1; source supports station undocking but does not complete selection/respawn facets. | `domain.md:209,249`, S01/S08. Explicit unresolved relation, not implied persistence schema. |

Facet alignment in existing stack: fixed four positions enforced by Rust array; uniqueness and ownership checked by `Hangar::validate`; mode category is serde enum and respawn a bool; JSON contains four mode rows. C12/runtime is assigned to future server, not enforced by present Rust. No SQL layer exists. There are no approved numeric limits for all future activities, so broad `u8`/fixed array changes would be premature. The existing `buy` helper and demonstration client are not evidence that a server currently executes deployment/respawn rules.

### Step 7 — Representative real examples and local fit

1. **Beacon Hunt participant in an ordinary Arena session [S03].** Three map beacons but only one active; participant's team captures it and cannot respawn while retaining control. When beacon rotates or ownership is lost, eligibility changes without changing the catalog mode. Local bool can label capability but cannot derive that transition. This is a concrete fit test, not a request to add the mode.
2. **Temple of Last hope mission [S04,S06].** Four-person mission family; clearance 5; ranks 1–17; one near-endless defensive stage, stronger successive waves. Local `waves-survival` is a plausible intentionally simplified inspiration, but it omits generator objective, scenario identity, access and wave runtime. The source's mission-wide death condition must coexist with S04's recovery/failure countdown rather than being rewritten as “all PvE has no respawn.”
3. **Return to Eden Special Operation [S05,S07].** A 10–12-pilot group, clearance 7, ranks 13–17 protects Defiant and advances the Devourer objective through stages. S05 permits two returns and ship changes. Mapping this to local `operation-scenario/false` without changing/documenting its rules would be inaccurate; retaining it as an unrelated original scenario is coherent.
4. **Guardian-17 Open Space visit [S08,S09].** Location has clearance minimum 3, protection/PvE status, several outgoing links including one-way Ellydium Theta transition, no listed Arena/Mission presence. Local Open World category covers context but does not model the sector graph. **No exact death/respawn behavior is added to this example**, because it was not found.

Examples were checked conceptually against the existing fields/constraints. No fictitious ship ranks were assigned to the local nine sample ship models, which have no rank field. No executable real-mode instance was added: that would be a semantic change beyond the audit authorization.

## 5. Gruber criteria

| Criterion | Assessment |
|---|---|
| **Clarity** | Define generic local mode versus real named activity; distinguish roster/hangar/collection, rank/clearance/difficulty and capability/current respawn eligibility. Exact source dates prevent “current” from silently meaning different versions. |
| **Coherence** | Current four example rows are not mutually inconsistent. Real-game reconstruction/conditional respawn would exceed their expressive power if imported. Do not infer all PvP is two teams or all PvE three stages from overbroad source summaries. No mode-specific internal logical defect was demonstrated. |
| **Extendibility** | New catalog modes can be added as instances, but a bool and universal owned-hangar source cannot express all documented policies without semantic refinement. Refine only when a selected activity is actually adopted. |
| **Minimal encoding bias** | Stable string IDs should not be casually renamed to match marketing; fixed four positions are a roster-capacity encoding, not proof of four unlocked lives. Team/captain and life budgets are runtime relationships, not ship-role enum cases. |
| **Minimal ontological commitment** | Preserve the stated independent-game scope and deferred progression/maps/AI. Record missing facts as gaps/questions, not mandatory replica features. Do not model future local-server modes before they are announced. |

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

## 7. Retrieval chronology, limitations and independent-review evidence

### Navigation and bounded gap-closing

Initial angles: **official current-news index**, **official-hosted wiki mode indexes**, and **Steam product overview**. Followed mode, mission, Open Space and Community links; expanded to progression, Special Operations and representative mission/location originals. One targeted gap-closing pass pursued hangar/slot and Open Space death details through wiki categories/API, FAQs, a tutorial and bounded search-engine requests. No unbounded crawl or repeated attempts against a blocked host.

Navigation-only/low-yield pages (retrieved 2026-09-16, not used to assert otherwise unsupported mechanics):
- **Main Page**, W, https://wiki.star-conflict.com/index.php?title=Main_Page — HTTP 200, last edit 2024-01-19 14:24, revision 14636; Game modes/General information links led to originals (`main`).
- **Star Conflict home/news**, O, https://star-conflict.com/en/ — HTTP 200, dynamic index update date unknown; dated 2026-09-14/11 links led to S16/S17 (`official`).
- **Category:Main**, W, https://wiki.star-conflict.com/index.php?title=Category:Main — HTTP 200, category description last edit 2019-10-23 07:45, revision 10189; listed 59 main pages including FAQ/Ship/Maintenance (`maincategory`). Category membership itself is dynamic.
- **Game guide**, O, https://star-conflict.com/en/game/gameguide and https://star-conflict.com/en/game/gameguide/2 — HTTP 200, dates unknown; tutorial indexes (`guide`, `guide2`).
- **[Tutorial] Open Space**, O, https://star-conflict.com/en/game/gameguide/14/current/ — HTTP 200, date unknown; converted text only announces the video, with no death/docking transcript. No claim based on unviewed video (`opentutorial`).
- **Features**, W, https://wiki.star-conflict.com/index.php?title=Features — HTTP 200, last edit 2023-09-26 12:36, revision 14234; broad mode/undock marketing duplicates S02, does not close death-selection gap (`features`).

Failures/non-evidence:
- https://wiki.star-conflict.com/index.php?title=Hangar — **HTTP 404**, guessed page absent, not a host access failure (`hangar`).
- https://wiki.star-conflict.com/index.php?title=Duplicator — **HTTP 404**, guessed page absent. No inference made about whether a corresponding in-game feature exists (`duplicator`).
- MediaWiki searches returned **HTTP 200, zero hits** (not proof absence):
  - https://wiki.star-conflict.com/api.php?action=query&list=search&srsearch=Hangar&srlimit=5&format=json
  - https://wiki.star-conflict.com/api.php?action=query&list=search&srsearch=docking&srlimit=5&format=json
  - https://wiki.star-conflict.com/api.php?action=query&list=search&srsearch=respawn&srlimit=5&format=json
- https://www.google.com/search?q=site%3Awiki.star-conflict.com+%22open+space%22+%22respawn%22 — **HTTP 200 but JavaScript/redirect interstitial**, no usable results (`search`).
- https://html.duckduckgo.com/html/?q=Star+Conflict+Open+Space+death+respawn+station+selected+ship — **HTTP 202 challenge**, no bypass attempted (`search-ddg`).
- https://www.bing.com/search?q=%22Star+Conflict%22+%22Open+Space%22+%22respawn%22 — **HTTP 200 but irrelevant results** (Rennes public transport “STAR”), discarded (`search-bing`).

Thus **no continuing official/wiki/Steam network blocker** exists in this pass. Residual uncertainty is content age/coverage, especially Open Space lifecycle, not the prior firewall condition. Search-engine failures should not replace already-fetched primary evidence with a blocked-only report.

### Commands and checks

- Repository checks: `git rev-parse HEAD`, `git branch --show-current`, `git status --short`, `git diff --exit-code`, `git diff --cached --exit-code`; baseline/master matched; no tracked or staged changes; only the authorized pre-existing untracked paths remained.
- Fetch pattern: `curl --location --connect-timeout 10 --max-time 35|40 --silent --show-error --write-out ... URL -o SCRATCH/key.html`, followed by `markitdown SCRATCH/key.html > SCRATCH/key.md`. API queries used bounded GET and standard-library JSON decoding. Successful status alone was not accepted without reading useful content.
- Read canonical/narrative/model and relevant instances directly; line-numbered anchors inspected with `nl -ba`. One inspection command mistakenly used `tail -n sixty` and failed; immediately corrected to `tail -n 60`, no files affected.
- No executable code or ontology changed, so no gameplay tests added/run and no package realization attempted. A runnable Python standard-library documentation check passed: 18 sequential source IDs and no undefined source references, 14 ledger rows, exact four local mode tuples, six local anchor assertions, all 36 useful original/converted source files present, and valid acceptance-report JSON. Final `git diff --exit-code` and `git diff --cached --exit-code` passed; authorized untracked paths unchanged.

```acceptance-report
{
  "criteriaSatisfied": [
    {"id": "criterion-1", "status": "satisfied", "evidence": "Produced only the requested modes research artifact plus managed fetched scratch evidence; preserved ontology semantics and repository files."},
    {"id": "criterion-2", "status": "satisfied", "evidence": "18 original useful sources with exact URLs, dated excerpts/paraphrases and freshness limits; local file:line reconciliation ledger, mode matrix, representative examples, Stanford/Gruber review, retrieval failures and repository checks."}
  ],
  "changedFiles": ["/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/507d64bc-df21-4587-a4fb-a4425950ac5f/research/modes.md", "Managed sibling scratch-modes/ HTML, converted Markdown and API JSON evidence only; no repository files changed"],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {"command": "git rev-parse HEAD; git branch --show-current; git status --short; git diff --exit-code; git diff --cached --exit-code", "result": "passed", "summary": "master at requested baseline, no tracked/staged changes; authorized AGENTS.md and docs/ontology/ preserved"},
    {"command": "Bounded curl fetches and markitdown conversions for source register and navigation originals", "result": "passed", "summary": "Official site/wiki/Steam readable; original source contents and visible dates inspected"},
    {"command": "Bounded guessed-page/API/search-engine gap-closing requests", "result": "passed", "summary": "Completed and recorded: two wiki 404s, three zero-hit API searches, Google interstitial, DuckDuckGo challenge and irrelevant Bing results"},
    {"command": "nl -ba ... | tail -n sixty", "result": "failed", "summary": "Harmless inspection typo; corrected to tail -n 60"},
    {"command": "nl -ba ontology/model.rs | head -n 228 | tail -n 60", "result": "passed", "summary": "Confirmed GameMode/PlayerShip/Hangar anchors"},
    {"command": "python3 standard-library documentation assertions plus final git diff/index checks", "result": "passed", "summary": "18 sources, 14 ledger rows, 4 exact mode tuples, 6 anchor checks, 36 retained source files and acceptance JSON validated; repository and index unchanged"}
  ],
  "validationOutput": ["No current official/wiki/Steam access blocker", "Four local mode instances directly inspected; no gameplay-rule mutation", "PASS: 18 source records, 14 ledger rows, 4 local mode tuples, 6 anchor assertions, 36 retained source files; acceptance JSON valid"],
  "residualRisks": ["Wiki revision freshness is not live-game verification", "Exact Open Space selected-ship and death/respawn rules remain unresolved", "Future local-server version and service shutdown date not established by retrieved announcement", "Independent reviewer gate remains required"],
  "noStagedFiles": true,
  "diffSummary": "Research artifact and managed scratch only; zero repository diff",
  "reviewFindings": ["No repository-edit blockers; semantic candidates are questions, not approved rules", "Do not automatically equate Operation Scenario with Special Operations or PvE Waves Survival with PvP Survival"],
  "manualNotes": "No nested agents, installations, builds, commits or staging. This retry supersedes historical access-blocked conclusions with fetched evidence. Review gate not self-approved."
}
```
