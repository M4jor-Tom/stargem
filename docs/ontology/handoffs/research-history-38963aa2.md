# Handoff: research-history — 38963aa2

Checkpoint: 2026-09-16, user-requested pause. Parent-curated from runtime records and actual files; not a new completed child report.

- **State:** COMPLETED; reuse output, do not rerun by default
- **Role:** `delegate`
- **Child run:** `38963aa2-6129-4a94-8406-10414fd67c51`
- **Workflow / key:** `507d64bc-df21-4587-a4fb-a4425950ac5f` / `research-history`
- **Mission:** `28a33a06-b296-4a78-8ae0-079881816489`
- **Repository:** `/home/theta/repos/stargem.nix`, branch `master`
- **Source baseline:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`
- **Session (machine-local, optional for recovery):** `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/2026-09-16T18-47-01-030Z_01a0ab8b-30a5-761c-90d3-cc64125796e1/e014c032-e922-4a2f-b695-e4b07d54af1e/run-0/session.jsonl`
- **Runtime status (machine-local):** `/tmp/pi-subagents-uid-1000/async-subagent-runs/38963aa2-6129-4a94-8406-10414fd67c51/status.json`

## Authority and boundaries

Read root `AGENTS.md` and `docs/HANDOFF.md` before work. `ontology/` remains canonical and unchanged. This is documentation/research, not authorization to change gameplay. No commit/push/merge/install is authorized. One writer per checkout. Preserve untracked audit files: they are the current work, not disposable clutter. Other agents' full research is intentionally file-backed to protect parent context.

## Resumption (for Agent)

Research is complete for this pass. Reuse the original report below and `../research/history.md`; do not refetch everything. Synthesis/review are still pending. Only resume this child for a specific missing citation, disputed claim or fresh evidence request. Keep publication/patch date distinct from retrieval date and deliberate local scope distinct from defects.

## Native continuation or fallback

Native resumability was verified with children.list at checkpoint. Recheck it in the next session; retained session availability can change.

```javascript
subagent({ action: "status", id: "38963aa2-6129-4a94-8406-10414fd67c51" })
subagent({ action: "children.list" })
// Only when resumable, and no original writer is still active:
subagent({ action: "resume", id: "38963aa2-6129-4a94-8406-10414fd67c51",
  message: "Read AGENTS.md, docs/HANDOFF.md and docs/ontology/handoffs/research-history-38963aa2.md. Continue ONLY the remaining work recorded there; preserve completed outputs and current files." })
```

If the retained session is missing or not resumable, do not claim a native resume or rerun all research. Launch a fresh `delegate` as a **same-role fallback continuation**, give it this handoff and the preserved reports, and start at the exact next step above. A resumed run may return a new run ID: record it and continue from the latest identity.

## Verification and acceptance

The original report below states exactly what this child checked and what remains uncertain. Completion of a research child does not mean that document synthesis, independent evidence review, ontology review or user reconciliation has completed. Follow [the main checkpoint](README.md) for dependency order and pending reviews.

## Preserved original output

Original managed artifact: `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/507d64bc-df21-4587-a4fb-a4425950ac5f/research/history.md`

SHA-256 of original artifact: `1c8fc3d5cb84b0efb4b5b81873cc9aad7a5abaa1298f4b068c1105f16403f1f0`

The complete output is embedded below so recovery does not depend on session/temp-file retention. Original relative code paths refer to the repository root unless the report says otherwise. Historical access failures stay historical.

---

# Star Conflict: identity, release chronology, setting, and scope reconciliation

Audit date and retrieval date: **2026-09-16**. Repository: `/home/theta/repos/stargem.nix`; branch `master`; inspected HEAD `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`.

## Executive findings

1. **Network retry succeeded.** Official Star Conflict, its official-hosted wiki, Steam, and Gaijin announcement pages returned substantive HTTP 200 responses. Previous access-blocked research is historical, not a present limitation. This report uses fetched originals, not search snippets.
2. **The current temporal boundary materially matters:** the newest numbered update visible on the inspected official news index is **1.14.15, dated 11 September 2026** [S09]. An April announcement schedules official server shutdown for **10 October 2026** [S04]; the 14 September announcement says a **special version supporting local servers is being developed**, with details forthcoming [S10]. Do not describe that version as released, downloadable, open source, or mechanically identical.
3. **Target identity is the desktop Star Conflict**, Steam app **212070**, not Star Citizen or the mobile **Star Conflict Heroes**. Heroes has its own 2017 release announcements, fleet-based combat and asynchronous Arena [S11–S12]. A newly announced **Star Conflict: Burrownauts** is another distinct game, not a new name for the desktop game [S10].
4. **The local ontology explicitly says “Star Conflict-like,” not “an exact Star Conflict simulator.”** Its three sizes, nine roles and deferred factions are valid local commitments unless a fidelity requirement is approved. A dated 2026 official patch names suppression destroyers, which cannot be represented by the closed local enums; this is a verified *coverage difference*, not automatically a local logical defect.
5. **Do not flatten different eras or source genres:** undated marketing still says nine tactical roles; dated 2026 patches name suppression destroyers. Old setting summaries describe three powerful states; the official “Ruins” storyline describes their collapse or transformation. Wiki “Biomorphs” was last edited in 2019. None of these establishes a single comprehensive September 2026 rulebook.

No source, ontology, instance, gameplay, dependency, branch, or Git-index changes were made. Existing untracked `AGENTS.md` and `docs/ontology/` were preserved. Root `AGENTS.md` was read first; `docs/HANDOFF.md` is absent. The requested ontology skill and inventory were read, followed by direct reading of `ontology/domain.md`, `ontology.md`, the complete `ontology/model.rs`, relevant ship/mode/special-module instances and `README.md`. Product questions below are reconciliation items, not requests to implement or questions sent to the user.

## Method and evidence boundary

Three initial navigation angles were used: official home/news/devblog navigation; wiki main/world/faction navigation; and Steam product identity/platforms. A fourth, limited search angle found the original Heroes announcements. Important claims were checked on original pages. One targeted closing pass checked the actual shutdown notice, Heroes originals/catalog identity, the latest event patch, and the official “Ruins” page. No unbounded archive crawl, authenticated access, package installation, external AI CLI, or nested agent was used.

Retrieval used `curl -L --connect-timeout 10 --max-time 35 --retry 0 -A 'Mozilla/5.0 (research audit)'`, storing headers and HTML in `/tmp/stargem-history-507d64bc/`; every available HTML response was converted with the installed `markitdown` before reading. Required text and relevant tables survived conversion; no substitute HTML parser was necessary. The 33 requests included discovery/failure pages, not 33 independent substantive sources. There are 21 useful original sources below. HTTP 200 alone was not accepted as success when the returned content was a login or JavaScript interstitial.

**Source priority proposed for later reconciliation, not approved rules:** local `ontology/` remains authoritative for Stargem; for claims about actual Star Conflict, prefer a dated release/patch for the identified version over an undated marketing summary. Devblog previews establish intent, not delivery. Official-hosted community wiki pages are useful descriptive evidence with revision dates, not automatically current official specifications. Store listings establish product identity and advertised platforms, not tested client compatibility. Fictional years, article dates, retrieval dates and software versions are distinct properties.

## Source register

All sources below were retrieved **2026-09-16**. “Unknown” means no article/update date was visible; a 2026 copyright footer is not an article date. Short quotations retain the source's wording; other text is explicitly a paraphrase. Star Conflict site sources are official StarGem/Gaijin publications; the wiki is classified separately as community-maintained, official-hosted.

### Identity and product family

**S01 — Star Conflict on Steam**  
URL: https://store.steampowered.com/app/212070/Star_Conflict/  
Publisher/type: developer/publisher-supplied Steam product listing hosted by Valve, not independent editorial reporting. Article/update date unknown; displayed release date **27 February 2013**.  
Headings: product metadata; “About This Game”; “The whole world for PVP and PVE!”; “Story”; “System Requirements.”  
Evidence: developer **“Star Gem Inc.”**, publisher **“Gaijin Network Ltd”**; requirement tabs **Windows / macOS / SteamOS + Linux**. Describes a “massively multiplayer space simulation game,” PvP/PvE/sandbox, corporation territory battles, crafting, and **“9 tactical roles.”** Story names Sector 1337.  
Supports: desktop product identity, advertised platform families, developer/publisher strings, broad domain. Does **not** establish up-to-date nine-role completeness, currently working legacy macOS binaries, or successful new-user registration. The same requirements section lists Snow Leopard/Mavericks and contains Steam's warning that macOS 10.14 or lower and 32-bit games are no longer supported by its client from 15 February 2024.

**S02 — Star Conflict — Gaijin Entertainment**  
URL: https://gaijinent.com/game/starconflict  
Publisher/type: official publisher product catalog. Article/update date unknown; displayed release date **September 4th, 2014**.  
Headings: introduction; “Key features”; “Developer”; “Release date.”  
Evidence: **“Developer — StarGem Inc.”** linking to Targem's website; **“Release date — September 4th, 2014.”** Describes the ruins in Sector 1337 and “Precursors’ lost treasures.”  
Supports: official developer attribution and a release milestone different from Steam's displayed date. Preserve both; the retrieved pages do not themselves explain whether this is beta/Steam availability versus full launch. The catalog shows Windows and Steam icons; that does not negate the other platform families still advertised by S01.

**S03 — About the game**  
URL: https://star-conflict.com/en/game/about  
Publisher/type: official evergreen game overview. Article/update date unknown.  
Headings: “Game Info”; “The whole world for PVP and PVE!”; “Key Features.”  
Evidence: **“Star Conflict is a dynamic MMO action game”**; **“9 tactical roles”**; players undock from stations, travel colonies, complete quests, craft and fight or cooperate; corporations fight for territories.  
Supports: broad agreement with local multiplayer space-combat inspiration, plus a nonexhaustive survey of omitted areas. Its nine-role wording conflicts with treating it as an exhaustive description alongside S09's suppression destroyer; freshness unknown.

**S11 — Star Conflict Heroes released!**  
URL: https://gaijinent.com/news/178  
Publisher/type: official Gaijin announcement, **09 February 2017**.  
Headings: title/body; “About Targem Games.”  
Evidence: Targem Games and Gaijin announce a **“brand-new Action RPG available for Apple mobile devices”**; players can **“field up to four starships”**; Arena pits fleets against each other **“in asynchronous battles.”** Android is described as coming later.  
Supports: distinct mobile game, iOS announcement date, different control/unit model, and separation from the local single-deployed-ship hangar. Its launch ship count and modes are historical, not 2026 totals.

**S12 — Star Conflict Heroes: Now on Android!**  
URL: https://gaijinent.com/news/180  
Publisher/type: official Gaijin announcement, **23 February 2017**.  
Heading: title/body.  
Evidence: **“the new mobile Space Action RPG by Targem Games and Gaijin Entertainment, is now available on Android!”** Each battle uses four selected spaceships; links Google Play package **`com.gaijin.scm`** and Apple app **`1165393158`**.  
Supports: product/platform distinction and original Android release announcement. Current store availability and Heroes' service lifecycle were not verified; the desktop shutdown notice must not be extended to Heroes without evidence.

### Dated current lifecycle and releases

**S04 — Star Conflict shutdown announcement**  
URL: https://star-conflict.com/en/news/3917-star-conflict-shutdown-announcement-en  
Publisher/type: official service announcement, **16 April 2026**.  
Headings: “Important dates”; “What’s next”; “Key points.”  
Evidence: **“October 10, 2026: Complete shutdown of the game servers”**; registration disabled May 14; monetization disabled June 1. After monetization ends, no new paid content is to be added and attention shifts to server stability.  
Supports: announced service schedule as known at audit date, not proof that future shutdown has occurred. Its promise of obtaining “all paid content” is broader than the “nearly all ships” wording in S06–S07; do not silently convert either into a precise entitlement rule.

**S05 — Star Conflict: registering new accounts restricted**  
URL: https://star-conflict.com/en/news/3926-star-conflict-registering-new-accounts-restricted-en  
Publisher/type: official service announcement, **14 May 2026**.  
Heading: title/body.  
Evidence: **“restricting the ability to create new accounts and download the game from the official website and the Gaijin Entertainment website starting today.”**  
Supports: implemented policy announcement, stronger than the earlier schedule. Does not itself say Steam installs for existing owners are blocked. Static download/play links on other pages are not contrary proof of access.

**S06 — Summer season event schedule**  
URL: https://star-conflict.com/en/news/3927-summer-season-event-schedule-en  
Publisher/type: official event schedule, **25 May 2026**.  
Headings: “Legacy of the Precursors”; “25 for 30”; “Black relay race”; “Summer season brawls and marathons.”  
Evidence: the final summer season includes free battle-pass/archive events; **“Each player’s unique progress in the ‘Black relay race’ is tracked.”** Lists “Return to Eden” special rewards for September 8–21 and “Paper Conflict” September 22–October 5.  
Supports: event/quest/progress/time-window domain exists beyond a mode enum; schedule is not proof every promised event ran unchanged. The date window places the audit within the *scheduled* Return to Eden reward period.

**S07 — Star Conflict 1.14.10. Disabling project monetization**  
URL: https://star-conflict.com/en/news/3928-star-conflict-1-14-10-disabling-project-monetization-en  
Publisher/type: official patch/event announcement, **1 June 2026**, version **1.14.10**.  
Headings: “Disabling project monetization”; “Legacy of the Precursors”; “‘25 for 30’ marathon”; “‘Way of Kindness’ marathon.”  
Evidence: **“monetization of the project is disabled starting on June 1, 2026”** while **“All in-game resources and currency will remain available for use until the project shuts down.”** Some premium/event ships unlock; nearly all ships can be obtained through events.  
Supports: major historical commercial-policy change without evidence that in-game prices/currencies disappeared. The patch's “25 for 30” assignments are 30 offered, 25 required; rewards include faction-labelled ships. “Federation cover frigate ‘Archelon’” illustrates translation vocabulary needing corroboration before inventing a new `cover` role.

**S08 — Star Conflict 1.14.14. UMC Archives: Shards of Eden**  
URL: https://star-conflict.com/en/news/3932-star-conflict-1-14-14-umc-archives-shards-of-eden-en  
Publisher/type: official patch/event announcement, **11 August 2026**, version **1.14.14**.  
Headings: “The ‘UMC Archives: Shards of Eden’ event”; “‘Quarter for thirty’ marathon update.”  
Evidence: one stage of 30 levels available to all pilots, lasting until shutdown. Reward list names **“Jericho command fighter ‘Kirishima’”**, **“Unique suppression destroyer ‘Saturn’”**, and Ellydium suppression destroyer **“Ze'Ta.”**  
Supports: dated faction/class/role examples and presence of destroyers; rewards/progression are not identical to mode definitions. Does not date when these ships or their classes were first introduced.

**S09 — Star Conflict 1.14.15. “Quarter for thirty” marathon update**  
URL: https://star-conflict.com/en/news/3934-star-conflict-1-14-15-quarter-for-thirty-marathon-update-en  
Publisher/type: official patch/event announcement, **11 September 2026**, version **1.14.15**.  
Headings: main marathon; “Changes to the ‘Ways of kindness’ marathon.”  
Evidence: **“Unique suppression destroyer ‘Bashe’”**, Empire recon interceptor “Psiloi,” engineering frigate “Salamander,” gunship fighter “Tornado,” rank 16 tackler fighter “Amber.” Tasks refer to **“Open World”**, shield restoration and **“hull”** restoration.  
Supports: freshest directly retrieved numbered-patch evidence of these terms/types, plus continuing PvP/PvE/Open World terminology. Not a complete mechanics specification and not an assertion that every older wiki mechanic remains valid at 1.14.15.

**S10 — Star Conflict: local server and a new indie game set in the same universe**  
URL: https://star-conflict.com/en/news/3933-star-conflict-local-server-and-a-new-indie-game-set-in-the-same-universe-en  
Publisher/type: official forward-looking announcement, **14 September 2026**.  
Headings: “Special version of Star Conflict”; “Star Conflict: Burrownauts”; “See the familiar universe from a new angle.”  
Evidence: a special version is being worked on whose main feature **“will be the ability to run a local server”**; more details soon. Burrownauts is a **“new indie game”**, a **“roguelite action game”** reimagining familiar ships/technologies; planned release **beginning of 2027**; Steam app link **4928770**.  
Supports: roadmap, distinction between service shutdown and preservation plans, and explicit separation of a new same-universe title. No license grant, released server binary, save-transfer promise, protocol documentation, or rules compatibility is stated.

### Setting and historical terminology

**S13 — Storyline**  
URL: https://star-conflict.com/en/game/storyline  
Publisher/type: official lore index/summaries. Article/update date unknown.  
Headings: “Signal era”; “Galactic war era”; “Season 0 ‘Contact’, Year 4610”; “Season 1 ‘Invasion’, Year 4613-4615”; “Season 2 ‘War’”; “Season 3 ‘Ruins’.”  
Evidence: **“The Alien invasion ended in collapse of humanity.”** Identifies Empire/Federation/Jericho, Crystallid attacks, United Mercenary Centre pilots, Sector 1337 and the Signal.  
Supports: era-dependent setting; fictional years are not software release dates. Its Invasion range and Season 3 label differ from S17's headings; preserve that editorial disagreement.

**S14 — Season 3 ‘Ruins’**  
URL: https://star-conflict.com/en/game/ruins  
Publisher/type: official original lore chapter. Article/update date unknown.  
Heading: title; body.  
Evidence: **“The empire was destroyed after the solar system was captured by biomorphs.”** Federation survives with corporations controlling effective power; Jericho largely disappears, leaving Mendes and AI-run infrastructure; Sentinels are **“automatic systems remaining from the Jericho faction of Technologists.”**  
Supports: old three-state descriptions are not timeless political invariants; distinguishes origin/heritage/ship branding from current political existence. This does not prove that Empire-labelled ship models were retired; S09 still advertises an Empire recon ship.

**S15 — Main Page — Star Conflict Wiki**  
URL: https://wiki.star-conflict.com/index.php?title=Main_Page  
Publisher/type: community-maintained official-hosted wiki. Last edited **19 January 2024, 14:24**; revision **14636**.  
Headings: “About the game”; “Factions”; “Game world”; “Pilot”; “Ships”; “General information.”  
Evidence/paraphrase: navigates to Interceptors/Fighters/Frigates/**Destroyers**, Open Space, missions, Special Operation, custom battle, campaign/contracts/tasks, corporations/wing/squad, implants/achievements/medals and crafting/workshop. Introduces **Sector 1337** and ruins of the **Precursors**.  
Supports: broad coverage inventory and source navigation, not current rules for all linked systems. No mechanics from unvisited linked pages are claimed here.

**S16 — Factions — Star Conflict Wiki**  
URL: https://wiki.star-conflict.com/index.php?title=Factions  
Publisher/type: community-maintained official-hosted wiki. Last edited **23 October 2019, 06:31**; revision **10084**.  
Headings: “General information”; “Main”; “Additional”; “Other”; “ALIENS.”  
Evidence: player can use **“an arbitrary combination of ships of different factions.”** Main: Empire/Federation/Jericho; additional: UMC/Ellydium/Pirates. Ellydium is also listed among Federation subfactions. UMC issues missions/licenses; Ellydium investigates alien technology and establishes Ellydium Theta on Leviathan.  
Supports: faction is not a simple exclusive player alignment or biological species; organization/heritage/ship-line senses overlap. Trait prose and acquisition rules are 2019 evidence, not verified 2026 balance. “Fraction/subfraction” in the text is translation variation, not a new mathematical domain.

**S17 — History — Star Conflict Wiki**  
URL: https://wiki.star-conflict.com/index.php?title=History  
Publisher/type: community-maintained official-hosted wiki. Last edited **26 September 2023, 12:54**; revision **14248**.  
Headings: “SEASON 0 — ‘CONTACT’, Year 4610”; “SEASON 1 — ‘INVASION’, Year 4614”; “SEASON 2 — ‘WAR’, Year 4616”; “SEASON 3 — ‘ONGOING CONFLICT’, Year 4618”; “Age of Destroyers”; “Evolution, the rise of Ellydium”; “A Galaxy in Ruins.”  
Evidence: **“a new type of ship is made available … Destroyers”**; **“Ellydium appears as a fourth faction.”** Later describes collapse, AI Sentinels and hostile encounters between Crystallids and biomorphs.  
Supports: historical expansion of world and ship taxonomy, but does **not** give reliable real-world introduction dates in these headings. “4618” is fictional. The page's “release of the game” parenthesis under fictional year 4614 must not become a calendar-release claim.

**S18 — Biomorphs — Star Conflict Wiki**  
URL: https://wiki.star-conflict.com/index.php?title=Biomorphs  
Publisher/type: community-maintained official-hosted wiki. Last edited **23 October 2019, 06:31**; revision **10083**.  
Heading: body definitions.  
Evidence: humans initially used biomorphs collectively, later distinguished **“Crystallides”** and **“Biomorphs”**; the latter commonly means infected/captured human ships. **“In different sectors, alien ships of the same species may be called differently.”** Bions are giant living planets.  
Supports: naming ambiguity and historical/in-universe knowledge qualifiers. The page itself mixes “Crystallides” and “Crystallids”; statements about alien agency remain qualified speculation, not proven equivalence of aliens, Precursors, Liu-virus and Crystallids.

### Developer-blog evidence: useful, but not safely dated

**S19 — Developer blog. Entry 121. - “Storm season”**  
URL: https://star-conflict.com/en/community/diary/123/current/  
Publisher/type: official development preview/lore. Article/update date **unknown**. Heading: “New ship ‘Cinquedea’.”  
Evidence: **“Jericho rank 16 ECM interceptor ‘Cinquedea’”** made by Sentinels; Techs conveyed shipbuilding principles to AI Sentinels. Preview refers to upcoming August/September dates without a year.  
Supports: concrete faction/subfaction/ship association and Techs/Sentinels distinction. Highest listed diary entry is not proof it is a September 2026 publication. URL number 123 is not entry number 121, and `/current/` is not a patch version.

**S20 — Developer blog. Entry 119. New open world adventure raid**  
URL: https://star-conflict.com/en/community/diary/122/current/  
Publisher/type: official developer explanation/lore. Article/update date **unknown**. Heading: body.  
Evidence: narrative had recently developed outside the game through stories; an adventure combines puzzles, combat, named contacts and paced quests across the Frontier sectors. **“An anomaly … causing a massive failure of the Seed-chips.”**  
Supports: narrative/adventure state and world effects are distinct from a simple combat-mode flag; the seed-chip claim is event-contextual and undated, not a universal removal of seed-chips. Article recommendations are evidence of gameplay intent, not instructions to this audit.

**S21 — Developer blog. Entry 118. - “Allies and rivals”**  
URL: https://star-conflict.com/en/community/diary/121/current/  
Publisher/type: official preview mixing feature description with explicitly fictional leaked-report narrative. Article/update date **unknown**. Headings: “New Ellydium ship ‘Drag'Thir’”; “‘Atlas’: new player on the weapons market”; “New adventure raid and plot development.”  
Evidence: introduces Ellydium guard frigate **“Drag'Thir”**, weapon-maker **“Atlas”**, and lore involving samples from living Crystallids. Mentions fictional years **4620, 4621, 4624**.  
Supports: expansion within familiar roles and distinctions between manufacturing organizations, factions and models. Allegations about Liu-virus connections and disappeared journalists are presented as rumors/report fiction, not objective mechanics. No mapping from fictional 4624 to publication year is justified.

## Publicly supported chronology and freshness

| Date or era | What is directly supported | Audit consequence |
|---|---|---|
| 27 February 2013 vs 4 September 2014 | Steam release metadata [S01] versus Gaijin catalog release metadata [S02]. | Preserve two attributed milestones. Do not invent a universal launch date or an unsupported beta explanation. |
| Fictional 4610 / 4613–4615 or 4614 / 4616 / 4618 | Official and wiki story-season headings [S13,S17]. | Separate lore chronology from calendar/version chronology; label conflicting headings. |
| By wiki's 2019 factions/aliens revision | Ellydium, UMC, pirates, subfactions, and differentiated aliens documented [S16,S18]. | Three principal political powers are not the entire organizational universe. No first-introduction patch date established. |
| 9 and 23 February 2017 | Heroes iOS and Android announcements [S11,S12]. | Mobile fleet RPG chronology is not the desktop game's chronology. |
| 2023–2024 wiki revisions | History and Main Page provide later editorial snapshots [S15,S17]. | A later page edit does not date every paragraph or establish 2026 mechanics. |
| 16 April → 14 May → 1 June 2026 | Shutdown plan → registration/download restriction announcement → monetization-disabled patch [S04,S05,S07]. | Old monetization/access descriptions require time qualification. In-game currency still exists; this is not evidence that all models now have price zero. |
| 25 May 2026 schedule | Final-season schedule extends into early October [S06]. | Events are time-bounded, separate from static game modes; schedule is prospective evidence. |
| 11 August 2026, 1.14.14 | Archive event and ship rewards including suppression destroyers [S08]. | Dated confirmation of broader taxonomy and continuing Ellydium/Jericho terminology. |
| 11 September 2026, 1.14.15 | Latest numbered patch visible in inspected official index; assignments/rewards changed [S09]. | Safe label: “officially documented through 1.14.15 for these claims,” not “all rules validated at 1.14.15.” |
| 14 September 2026 | Local-server special version in development; Burrownauts announced [S10]. | New preservation/product branches, not an already available replacement simulator. |
| 10 October 2026 / beginning of 2027 | Announced server shutdown / planned Burrownauts launch [S04,S10]. | Future at audit date; not observed completed events. |

The news index also lists **1.14.12 (2 July 2026)** and **1.14.13 (21 July 2026)**. These are *index-only chronology leads*, not independently read patch evidence; no mechanics are attributed to them. No claim is made that the inspected index exposes every hotfix. The newest visible diary is Entry 121, but no publication year was visible in the original or date-related HTML inspection. It must remain undated.

## Terms, relationships, and important non-equivalences

| Term / relationship | Supported interpretation and cardinality caution |
|---|---|
| Star Conflict / Star Conflict Heroes / Burrownauts | Distinct products [S01,S10–S12]. Shared universe/assets/names do not imply shared rules. Local “Stargem” is the repository/game name; official “StarGem Inc.” is a developer attribution, not evidence of affiliation. |
| Developer / publisher / rights-holder | Steam: Star Gem Inc. / Gaijin Network Ltd. Official footer: developed by StarGem Inc., copyright Gaijin Games Kft.; publisher announcements brand Gaijin Entertainment. Preserve these roles/names separately rather than replacing all with one organization. Targem links/announcements show association, not a proved corporate-history timeline. |
| Faction / subfaction / corporation | Empire/Federation/Jericho are principal human political traditions; Ellydium is called a corporation, a faction and a Federation subfaction in different passages [S16,S17]. This is overlapping membership/history, not evidence for disjoint subclasses. A player-to-faction exclusive 1:1 would be too strong; the 2019 wiki explicitly permits mixed-faction fleets. |
| Corporation | Can mean a lore organization/manufacturer (Ellydium, Atlas) or a player social organization [S03,S15,S21]. Do not identify them solely by shared display noun. |
| Model → role → class | Local cardinalities many→1 and functional role→size are explicit (`domain.md:195–197`). S08–S09 confirm examples command→fighter, recon→interceptor, engineering→frigate and suppression→destroyer. Examples do not prove every ship obeys a single immutable role or special-module mapping. |
| Player-owned ship / selected ship / fleet in combat | Local hangar is 0..4 owned ships and selected ship 0..1 (`domain.md:208–209`). Heroes fields up to four simultaneously [S11]; same number, different relation. Do not import the latter as support for desktop deployment rules. |
| Open World / Open Space | Current official patch uses Open World [S09]; wiki navigation uses Open Space [S15]. Local `open-world` is therefore an acceptable descriptive identifier, not a demonstrated obsolete error. “Sandbox” is broader marketing wording, not necessarily a separate mode. |
| Mode / event / assignment / campaign / adventure | S06–S09 show events can span PvP, PvE and Open World; therefore one event need not equal one `GameModeCategory`. Quest progress, timed availability and story branches are independent concepts. Exact general cardinalities were not established. |
| Empire after collapse | Political collapse in S14 does not remove Empire ship heritage; Psiloi is still Empire-labelled in S09. Avoid a rule that entity inactivity deletes all models bearing its name. |
| Techs / Technologists / Sentinels | S14: Sentinels are remaining automatic systems of Technologists. S19 distinguishes Techs transmitting principles from AI Sentinels using them. Related historical concepts, not safely interchangeable IDs. |
| Precursors / Forerunners / Aliens | Precursors name the ancient civilization/legacy; “Forerunners” appears in Jericho discussion [S16]; generic “Aliens” appears in storefront lore [S01,S02]. Treat apparent translation aliases as attributed labels, not proof every hostile alien is a Precursor. |
| Biomorphs / Crystallids / Crystallides / Liu-virus / Bions | Historically overloaded collective label versus differentiated groups; infected human ships, crystal-like entities and living planets are not one class [S17,S18]. Preserve in-universe uncertainty. “Crystallides” and “Crystallids” are spelling variants within the same page, not evidence of two species. |
| Frontier / Divide / Frontier sectors | Older history translates religious movement/structure language variably [S17]; later devblogs use Frontier sectors as geography [S20,S21]. Do not merge all occurrences into a single geographic ID. |
| Armor / hull; engineer / engineering; covert ops / “Cover Ops” | Local `armor_hp`, `engineer`, `covert-ops` versus current patch “hull,” “engineering frigate,” “covert ops” [S08,S09]; narrative `Cover Ops` at `ontology.md:23,31`. Likely label normalization issues, not evidence of different damage mechanics or a separate Cover Ops role. Detailed combat reconciliation belongs in its own lane. |
| “25 for 30” / “Quarter for thirty”; Way/Ways of kindness | June and September official texts use variant English names [S07,S09]. June and September patch bodies both describe 30 assignments with at least 25 required; May schedule instead summarizes 25 assignments over 30 days [S06]. Treat names/descriptions as versioned/translated event labels; do not derive cardinality from a title. |

## Candidate reconciliation ledger

Confidence distinguishes **confidence in the sourced observation** from **confidence that a local change is needed**. All options require an ontology decision first; none authorizes code changes.

### H01 — Target era and fidelity are unstated
- **Local anchor / exact claim:** `ontology/domain.md:10–11`: “multiplayer space-ship combat (Star Conflict-like)”; `README.md:3–4` establishes local ontology authority. No cited external version/provenance accompanies `domain.md:59–64` or `model.rs:12–16`.
- **External comparison:** S04–S10 distinguish live-service wind-down, 1.14.15 and an upcoming local-server version; S11–S12 identify a separate mobile ruleset.
- **Classification:** uncertainty / provenance coverage gap; **not a verified contradiction**.
- **Confidence / impact:** high that target era is unspecified; high cross-cutting impact, low confidence any gameplay change is needed.
- **Question/options:** Is Stargem independently inspired, a dated approximation of an older desktop era, a 1.14.15 simulator, or eventually a compatible local-server client? Prefer documenting the chosen boundary before using any audit finding as a rule.

### H02 — Three sizes/nine roles are not exhaustive September 2026 desktop coverage
- **Local anchor / exact claim:** `ontology/domain.md:60`: “Enumeration: frigate (big), fighter (medium), interceptor (small)”; `domain.md:63–64` lists nine roles; `ontology/model.rs:12,16,25–34` closes those enums/maps; `ontology.md:20–23` agrees.
- **External comparison:** S09 explicitly names suppression destroyer Bashe; S08 also names Saturn and Ze'Ta. Undated S01/S03 still advertise nine tactical roles.
- **Classification:** verified coverage difference, potentially deliberate simplification. A contradiction only **if** local scope is later asserted to exhaustively model modern Star Conflict. Marketing-versus-patch conflict is an external source freshness problem, not a Rust inconsistency.
- **Confidence / impact:** high source confidence; high schema impact for exact fidelity, none required for a deliberately limited game.
- **Question/options:** Explicitly limit the model to the three selected classes, choose an earlier era with separately verified boundaries, or approve destroyer/suppression expansion. Do not infer their introduction date from fictional “Age of Destroyers.”

### H03 — Faction and world coverage is deliberately postponed
- **Local anchor / exact claim:** `ontology/domain.md:22–23`: “Might add later (v2) … faction/tier”; `domain.md:19–20` excludes AI and map/level content; `ShipModel` at `ontology/model.rs:111–120` has role/base/slots but no faction relationship.
- **External comparison:** S08–S09 use faction-qualified models; S16 includes main/additional/other organizations; S14 describes a changed political world.
- **Classification:** explicit deferral / coverage gap, not logical defect.
- **Confidence / impact:** high; medium if catalog provenance or lore UI is needed, low for current flight prototype.
- **Question/options:** Keep faction absent, add only provenance labels later, or model faction/manufacturer/heritage separately. Avoid a prematurely closed three-faction enum or exclusive player allegiance rule.

### H04 — World events and narratives are not just combat modes
- **Local anchor / exact claim:** `ontology/domain.md:155–160` gives mode only id/name/category/respawn; `domain.md:258–259` defers wave/operation generators; `ontology/instances/game_modes.json:2–5` contains team-deathmatch, waves-survival, operation-scenario, open-world.
- **External comparison:** S06 has tracked story choices, S09 assigns tasks across PvP/PvE/Open World, S20 describes paced adventures and context-specific world effects.
- **Classification:** scope/coverage gap and deliberate simplification, **not** proof that local generic mode names are wrong.
- **Confidence / impact:** high observation; low present impact, substantial only for faithful campaigns/events.
- **Question/options:** Are local modes generic original scenarios or exact named Star Conflict activities? Keep events outside scope unless competency questions require them; do not introduce every advertised event as another enum variant.

### H05 — Terminology aliases need a glossary, not automatic stable-ID renames
- **Local anchor / exact claim:** `ontology/domain.md:6`: IDs “never change once referenced”; `domain.md:69–71,78–86,159` uses thermic/armor/open-world; `ontology.md:23,31` says “Cover Ops”; `model.rs:16` uses `CovertOps`.
- **External comparison:** S08 uses covert ops; S09 uses hull, engineering and Open World; S15 uses Open Space. S07's “cover frigate” is ambiguous translation wording, not sufficient evidence for a distinct role.
- **Classification:** terminology uncertainty and local narrative inconsistency (`Cover Ops` vs canonical covert-ops), not proven mechanics contradiction.
- **Confidence / impact:** high for exact observed strings; medium on intended aliases. Medium migration risk if identifiers are renamed casually.
- **Question/options:** Keep stable internal IDs and document display/source aliases; decide preferred English vocabulary separately. Thermal/thermic mechanics and special-module names need their own detailed evidence, not this history pass.

### H06 — “Owned forever” is not a claim of eternal official service
- **Local anchor / exact claim:** `ontology/domain.md:163`: “owned forever by one user”; `domain.md:200`: ownership “permanent”; `ontology.md:12`: “PlayerShips are owned forever by User.”
- **External comparison:** S04 schedules server shutdown; S10 announces future local-server capability but no save-transfer or ownership guarantee.
- **Classification:** semantic uncertainty, **not a contradiction**: permanent ownership within a game model and indefinite server availability are different propositions.
- **Confidence / impact:** high distinction; low runtime impact now, medium documentation/compatibility impact.
- **Question/options:** Define permanence as nontransferability/nonconsumption within the local game's lifetime; do not imply continuity of official accounts or import rights into Stargem.

### H07 — Real-money monetization ending does not refute local ship prices
- **Local anchor / exact claim:** `ontology/domain.md:19` excludes economy beyond price; `domain.md:96` has positive ship price; `domain.md:247` says buying costs price; `ontology/instances/ship_models.json:2–37` has positive local prices.
- **External comparison:** S07 simultaneously disables monetization and preserves in-game resources/currency, with event unlocks. S04's “all paid content” and S06/S07's “nearly all ships” are different scope statements.
- **Classification:** historical policy change; no verified local pricing contradiction. Actual catalog price fidelity is outside this lane.
- **Confidence / impact:** high; prevents erroneous blanket price-zero changes.
- **Question/options:** Are prices independent balancing data or a snapshot of official acquisition rules? Preserve them pending explicit economy reconciliation; separately timestamp any imported availability policy.

### H08 — Product-family collision can manufacture false agreement
- **Local anchor / exact claim:** `ontology/domain.md:248–249` defines four hangar slots, then one deployed/selected ship per life/context; `ontology.md:61,64–65` describes the same flow.
- **External comparison:** S11–S12's four-ship Heroes fleet is an in-battle selection, with asynchronous Arena, not a desktop hangar.
- **Classification:** source-selection uncertainty/preventive distinction; no evidence local code was actually derived from Heroes.
- **Confidence / impact:** high; high if cross-product evidence contaminates future reconciliation.
- **Question/options:** Restrict external mechanics evidence to desktop app 212070 and explicitly label any cross-product comparison. Do not use Burrownauts' mining/roguelite loop or Heroes' four-ship battles to fill desktop gaps.

### H09 — Immutable catalog versus changing upstream game is not inherently incoherent
- **Local anchor / exact claim:** `ontology/domain.md:240`: “Catalog items are immutable; only player-ship and hangar mutate.”
- **External comparison:** dated patches change events/content [S07–S09]; announcements introduce new product variants [S10].
- **Classification:** uncertainty about time scope of immutability, not a verified logical issue.
- **Confidence / impact:** high; medium for future catalog versioning and reproducible fixtures.
- **Question/options:** Is immutability per released local catalog/session rather than across all authored versions? Record snapshot provenance in research now; add migration/version machinery only if a real import/compatibility requirement appears.

### H10 — Broken source-narrative reference is an internal documentation issue
- **Local anchor / exact claim:** `ontology/domain.md:4` links `../onthology.md`; actual narrative is `ontology.md` at baseline, and HEAD subject is `refactor(onthology.md): fix typo`.
- **External comparison:** none needed; this is local evidence only.
- **Classification:** verified internal logical/document-link issue caused by historical rename; not a gameplay contradiction.
- **Confidence / impact:** high; low impact, obstructs evidence traceability.
- **Question/options:** A later authorized documentation edit can correct the reference without changing semantics. No edit made in this audit.

No verified in-scope gameplay contradiction is asserted by this history lane. No detailed ship-role/name remapping, special activation, damage formula, combat balance or acquisition catalog is proposed here; those need their dedicated evidence lanes.

## Agreed facts and three representative reality checks

**Agreement:** local PvP/PvE/open-world categories (`domain.md:159`, `ontology.md:1–4`) align at a broad level with official descriptions [S03,S09]. Station-undock wording (`ontology.md:65`, `domain.md:249`) agrees with S03. Recon/interceptor, command/fighter and engineering/frigate examples remain present in dated 2026 announcements [S08,S09]. Shared terminology does not validate local numerical stats, special-module rules, respawn flags or the completeness of local catalog entries.

1. **Psiloi — Empire recon interceptor**, S09 (11 September 2026). Local `recon → interceptor` (`model.rs:34`) represents its role/class pair without change. Empire identity has no local slot because faction is deferred. Its stats/price/loadout are not supplied by this source and must not be fabricated to force a complete JSON instance.
2. **Kirishima — Jericho command fighter**, S08 (11 August 2026). Local `command → fighter` (`model.rs:33`) also fits. The source names its associated weapon/active module, but does not establish that all command fighters share the local `command-shield` special. Result: partial descriptive fit, not full loadout validation.
3. **Bashe — unique suppression destroyer**, S09. Neither size nor role is representable by `ShipSize`/`ShipRole`. Result: demonstrable modern coverage ceiling, not evidence of an internal type error or permission to add a new variant.

These are sourced conceptual test instances, not persisted catalog additions. No unavailable source values were invented and no claim is made that they satisfy all current local numeric/ownership facets. The existing nine local ship records were directly read, but their detailed identity fidelity is intentionally left to the ship research lane.

## Missing areas: scope survey, not implementation backlog

| Publicly documented area | Local status / restraint |
|---|---|
| Faction heritage, subfactions, lore manufacturers, human/alien distinctions, historical eras [S13–S21] | Faction deferred, AI/map content excluded. Important if lore or faithful catalog attribution becomes a goal; not needed to fly the existing prototype. |
| Quest/adventure contacts, story branches, assignment state, campaign and time-limited events [S06,S09,S20] | No corresponding classes; generators explicitly deferred. Do not overload static mode category. |
| Player corporations, wings/squads, territorial relationships [S03,S15] | Not represented. Names confirmed as domain areas, detailed mechanics not researched here. |
| Pilot implants/advancement/achievements/titles/medals [S15] | Progression expressly excluded. No claim about their detailed present-day effects. |
| Sector graph, stations, travel, exploration and environmental story state [S03,S15,S20] | Map content/AI excluded; only selected-ship station deployment is stated. |
| Service era, platform support, product identity, source/version provenance [S01–S12] | Research metadata absent from local model; documentation may suffice. No need for new runtime entities merely because the audit uses them. |
| Modern ship classes and heritage [S08,S09] | Three-class/nine-role abstraction is a confirmed ceiling. Full combat/economy details remain outside this exclusive topic. |

## Stanford seven steps applied

1. **Domain and scope.** Audited a bounded, Star Conflict-like multiplayer combat ontology against product identity, release eras and world vocabulary. Competency questions answered: (a) Which product? Desktop app 212070, not Heroes/Burrownauts. (b) Which publicly documented current patch? At least 1.14.15 for the retrieved changes. (c) Is a new service era imminent? Yes, scheduled closure plus unreleased local-server plan. (d) Are exactly three classes/nine roles complete for that era? No, suppression destroyers are documented. (e) Are factions simple exclusive player alignments? Sources do not support that; historical mixed-faction fleets and overlapping organizational senses are documented. (f) Can wiki dates establish current mechanics? No. (g) Does local scope promise exact simulation? No; target fidelity remains a product decision. Primary users/content authors remain those identified at `domain.md:13–14`.
2. **Reuse.** Reused canonical prose, typed Rust enums/maps and JSON catalog rather than designing new schemas. Read the supplied inventory but checked relevant files directly. Official indexes, original announcements and wiki revision IDs provide reusable source evidence; no dependency or external ontology was added.
3. **Terms.** Enumerated identity, release/version/date, faction/subfaction/manufacturer, alien/lore terms, role/class, mode/event and platform vocabulary above. Synonyms and source-specific senses remain explicit.
4. **Classes/hierarchy.** A suppression destroyer is an example of a ship class/role absent locally; a faction is not a ship role; organization membership/history is not necessarily disjoint inheritance. Techs/Sentinels and Biomorphs/Crystallids must not be merged solely by word resemblance. Existing local role/class hierarchy can remain valid for its selected scope.
5. **Properties/slots.** Version/date/source and fiction-era are research annotations; faction/manufacturer membership is relational, unlike display name. No evidence justifies forcing service/platform attributes onto each combat ship. Keep intrinsic stats separate from political and historical provenance.
6. **Facets/cardinalities.** Local closed enums and 1:1 role-special relation are explicit commitments; example pairs verify only partial coverage. Historical faction mixing cautions against exclusive allegiance. Heroes' four concurrent ships and local four hangar slots are distinct cardinalities. General upstream cardinalities not proven by the sources remain unknown.
7. **Instances.** Psiloi, Kirishima and Bashe above test semantic representability with dated evidence. They are intentionally incomplete as runtime JSON because sources here do not supply every required value. Existing instances were not rewritten to force fidelity.

### Gruber criteria

| Criterion | Assessment |
|---|---|
| Clarity | Strong local scope statement; weakest point is unspecified target era/fidelity and undocumented terminology aliases. Distinguish real date, fictional date, release and revision. |
| Coherence | No contradiction follows merely from excluded factions/events. Broken narrative link is a verified internal documentation issue. Upstream marketing and patch/lore differences must not be collapsed into one timeless assertion. |
| Extendibility | New ship records fit existing roles easily; destroyer/suppression requires intentional enum/map changes. Avoid committing now to an exclusive three-faction hierarchy or one product-wide ruleset. |
| Minimal encoding bias | Stable kebab-case IDs need not mirror translated display strings or URL spellings. Preserve apostrophes/case in source names and keep product IDs/version labels distinct from invented local IDs. |
| Minimal ontological commitment | “Star Conflict-like” permits an original simplified game. Record provenance and boundaries first; do not import every lore/economy/social feature or write speculative compatibility machinery. |

## Open reconciliation questions and recommended decisions to record

- **Era/fidelity:** independent inspired game, a historically pinned subset, current desktop 1.14.15, or future local-server compatibility? The last option cannot be specified from an unreleased announcement.
- **Source priority:** which versioned official mechanics outrank wiki and marketing, and how should conflicts be retained? Recommend claim-level date/type/revision rather than a blanket “official wiki is current” rule.
- **Coverage promises:** is the nine-model catalog demonstrative or intended as faithful named content? Are destroyers intentionally excluded? Do not infer a promised feature from familiar names.
- **Lore target:** pre-invasion, invasion, Ruins, later Frontier stories, or original Stargem setting? Which faction sense is required: affiliation, manufacturer, historical heritage, player allegiance, or all separately?
- **Terminology policy:** stable internal IDs plus aliases, or migration to preferred English labels? A translation edit is not a new mechanic; “Biomorphs” may be either historical umbrella or narrower group depending on the source.
- **Identity/platform scope:** does Stargem seek any official-client compatibility, or merely similar play? README's raylib/“14 platforms” description concerns Stargem's renderer stack, not proof of Star Conflict platforms.
- **Preservation:** no evidence here authorizes copying server code, importing accounts, or assuming the forthcoming local-server license. Research public details when released, only if compatibility is actually desired.

## Retrieval accounting and limitations

Successful substantive originals: S01–S21, all HTTP 200. Successful navigation pages, also retrieved 2026-09-16, with no inferred mechanics from snippets:
- **Official home**, https://star-conflict.com/en/ — official news navigation, dates visible on listed articles; unknown page-update date. Latest visible headline was S10. Footer credits Gaijin Games Kft. and StarGem Inc.
- **News**, https://star-conflict.com/en/news/ — official first index page; 14 September announcement and 11 September numbered patch at top. No independent page date.
- **News page 2**, https://star-conflict.com/en/news/2 — official navigation to S04. No independent page date.
- **Developer's diary**, https://star-conflict.com/en/community/diary — official navigation to Entries 121/119/118; publication dates not shown. Preview wording is not current-release proof.
- **DuckDuckGo Heroes search**, https://html.duckduckgo.com/html/?q=Star+Conflict+Heroes+site%3Agaijinent.com — search-engine discovery only, publication date unknown; located S11/S12/S02. No snippets were used as final factual support.
- **Star Conflict tag**, https://gaijinent.com/tag/starconflict — official catalog index, no date; returned only game navigation, not the hoped-for historical news archive. No additional claim supported.

Failed or unproductive retrievals (not generalized to other hosts):
- https://www.google.com/search?q=Star+Conflict+Heroes+Gaijin+Targem+2017 — HTTP 200, redirected URL gained `&ucbcb=1`; returned JavaScript/retry interstitial, no usable results.
- https://starconflictheroes.com/ — guessed discovery host; curl error **6**, DNS resolution failed, HTTP **000**. No retry; this does not establish the official mobile game's availability.
- https://gaijin.net/en/games/ — HTTP 200 after redirect to https://login.gaijin.net/fr ; login page, not a usable public game catalog. Correct public catalog was subsequently found through search at S02.
- https://html.duckduckgo.com/html/?q=site%3Agaijinent.com%2Fnews+%22Star+Conflict%22+%22release%22+%222014%22 and https://html.duckduckgo.com/html/?q=site%3Agaijinent.com%2Fnews+%22Star+Conflict%22+%22Evolution%22 — both HTTP **202**, human-verification challenges; submitted in one bounded batch, not retried/bypassed.
- https://gaijinent.com/en/games/starconflictheroes — historical official link from S12; redirected to https://gaijinent.com/game/starconflictheroes and returned **404**. Historical announcements still fetched successfully. No conclusion about current mobile service status follows.

Totals: **29 HTTP 200**, **2 HTTP 202**, **1 HTTP 404**, **1 HTTP 000**. Three of the HTTP-200 pages were navigation/search redirects or unproductive catalog discovery, not independent game facts. Raw responses, converted text and `fetch.tsv` remain in the scratch directory for this session; the durable claims, dates, excerpts, exact URLs and wiki revision IDs are retained above so a later writer need not rely on scratch retention.

Freshness limitations: no installed game-client observation; no authentication, account creation, purchases or server play; exact old introduction dates for destroyers/Ellydium not established; undated devblogs cannot safely date seed-chip changes or new ships; last wiki edit does not prove per-paragraph version applicability; Heroes current availability not checked; future shutdown/preservation plans remain announcements. Earlier network failures belong only to prior-attempt chronology and do not justify an access-blocked conclusion in this retry.

## Acceptance evidence

Only this configured research artifact was written persistently. No project files changed; no tests were added because this is research-only and no executable logic changed. All source claims above are tied to originals or explicitly labelled navigation-only/uncertain. Scope, source conflicts, noncontradictions, excluded areas and unanswered product questions are retained for independent review.

Validation completed: a Python standard-library assertion check verified all **21** source IDs, all **21** source-record URLs against the retrieval log, nonempty Markdown conversions for all **29 HTTP-200 responses**, and **eight critical claims** against converted original content. It also verified the obsolete narrative path is absent and the correct narrative exists. Final `git diff --stat` and `git diff --cached --name-only` were empty; branch and HEAD remained `master` and the specified baseline. The only status entries remained the authorized pre-existing untracked `AGENTS.md` and `docs/ontology/`. No Rust/build tests were run: there is no code change, and package installation/build setup is outside this research task.

```acceptance-report
{
  "criteriaSatisfied": [
    {"id":"criterion-1","status":"satisfied","evidence":"Completed only the assigned identity/history/setting research artifact; no repository or ontology edits."},
    {"id":"criterion-2","status":"satisfied","evidence":"21 attributed original-source records with exact URLs, dates, excerpts, revision caveats; 10 anchored reconciliation items; successful evidence assertions and empty Git index."}
  ],
  "changedFiles": ["/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/507d64bc-df21-4587-a4fb-a4425950ac5f/research/history.md"],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {"command":"curl -L --connect-timeout 10 --max-time 35 --retry 0 -A 'Mozilla/5.0 (research audit)' ...; markitdown downloaded.html","result":"passed","summary":"Fetched and read 21 substantive originals; 33 total requests and every failure/unproductive result recorded."},
    {"command":"python3 source-record, converted-content, critical-claim and local-path assertions","result":"passed","summary":"21 source records, 21 fetched URLs, 29 nonempty HTTP-200 conversions, eight critical source claims checked."},
    {"command":"git diff --stat; git diff --cached --name-only; git status --short; git branch --show-current; git rev-parse HEAD","result":"passed","summary":"No tracked changes or staged files; master at baseline; authorized untracked work preserved."}
  ],
  "validationOutput": ["Evidence assertions passed; official and wiki access succeeded; obsolete blocked-only conclusion replaced by fetched evidence."],
  "residualRisks": ["No live-client validation; wiki/devblog freshness differs; announced local-server version is unreleased; historical introduction dates and current Heroes availability remain unverified."],
  "noStagedFiles": true,
  "diffSummary": "Added only the configured research artifact; no repository diff.",
  "reviewFindings": ["No research-scope blocker; independent reviewer gate remains required."],
  "manualNotes": "The crucial new dated finding is planned 2026-10-10 shutdown plus a 2026-09-14 local-server preservation announcement, not an available replacement release. No semantic reconciliation decision is approved by this report."
}
```
