# Handoff: research-progression — 47f4cf33

Checkpoint: 2026-09-16, user-requested pause. Parent-curated from runtime records and actual files; not a new completed child report.

- **State:** COMPLETED; reuse output, do not rerun by default
- **Role:** `delegate`
- **Child run:** `47f4cf33-1679-42f5-a84b-8f546ccf3b69`
- **Workflow / key:** `507d64bc-df21-4587-a4fb-a4425950ac5f` / `research-progression`
- **Mission:** `28a33a06-b296-4a78-8ae0-079881816489`
- **Repository:** `/home/theta/repos/stargem.nix`, branch `master`
- **Source baseline:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`
- **Session (machine-local, optional for recovery):** `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/2026-09-16T18-47-01-030Z_01a0ab8b-30a5-761c-90d3-cc64125796e1/403a3a6c-951f-4cf5-a9ba-f54a857fda12/run-0/session.jsonl`
- **Runtime status (machine-local):** `/tmp/pi-subagents-uid-1000/async-subagent-runs/47f4cf33-1679-42f5-a84b-8f546ccf3b69/status.json`

## Authority and boundaries

Read root `AGENTS.md` and `docs/HANDOFF.md` before work. `ontology/` remains canonical and unchanged. This is documentation/research, not authorization to change gameplay. No commit/push/merge/install is authorized. One writer per checkout. Preserve untracked audit files: they are the current work, not disposable clutter. Other agents' full research is intentionally file-backed to protect parent context.

## Resumption (for Agent)

Research is complete for this pass. Reuse the original report below and `../research/progression.md`; do not refetch everything. Synthesis/review are still pending. Only resume this child for a specific missing citation, disputed claim or fresh evidence request. Keep publication/patch date distinct from retrieval date and deliberate local scope distinct from defects.

## Native continuation or fallback

Native resumability was verified with children.list at checkpoint. Recheck it in the next session; retained session availability can change.

```javascript
subagent({ action: "status", id: "47f4cf33-1679-42f5-a84b-8f546ccf3b69" })
subagent({ action: "children.list" })
// Only when resumable, and no original writer is still active:
subagent({ action: "resume", id: "47f4cf33-1679-42f5-a84b-8f546ccf3b69",
  message: "Read AGENTS.md, docs/HANDOFF.md and docs/ontology/handoffs/research-progression-47f4cf33.md. Continue ONLY the remaining work recorded there; preserve completed outputs and current files." })
```

If the retained session is missing or not resumable, do not claim a native resume or rerun all research. Launch a fresh `delegate` as a **same-role fallback continuation**, give it this handoff and the preserved reports, and start at the exact next step above. A resumed run may return a new run ID: record it and continue from the latest identity.

## Verification and acceptance

The original report below states exactly what this child checked and what remains uncertain. Completion of a research child does not mean that document synthesis, independent evidence review, ontology review or user reconciliation has completed. Follow [the main checkpoint](README.md) for dependency order and pending reviews.

## Preserved original output

Original managed artifact: `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/507d64bc-df21-4587-a4fb-a4425950ac5f/research/progression.md`

SHA-256 of original artifact: `29d5db4d91c7f4e86a3c33573039f222ecee739d1297820c7cabf2e458a856ce`

The complete output is embedded below so recovery does not depend on session/temp-file retention. Original relative code paths refer to the repository root unless the report says otherwise. Historical access failures stay historical.

---

# Star Conflict contextual audit: progression, economy, factions and social ownership

**Audit/retrieval date:** 2026-09-16. **Repository:** `/home/theta/repos/stargem.nix`, branch `master`, verified HEAD `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`. Research/documentation only; no ontology decisions or implementation changes are approved by this report.

## Executive findings

1. **Network access worked.** All three seeds and 20 additional useful pages returned HTTP 200 and were converted with the installed `markitdown`; two MediaWiki discovery queries also returned 200. One guessed article title, `Trading`, returned 404; actual trading evidence was found under `Galactic Standard`. Earlier access-blocked reports describe history, not this run's evidence.
2. **The most important freshness correction is official, not inferred:** patch 1.14.10, dated **2026-06-01**, says project monetization is disabled from that date; in-game currencies/resources remain usable until shutdown. A **2026-09-14** announcement says servers will shut down soon and a special local-server-capable version is in development. Do not describe old wiki/store payment instructions as a current paid-service offering, or assume this forthcoming version preserves existing progression. [S13–S14]
3. **A single positive integer price plus unconditional permanent ownership is not a faithful universal description of documented Star Conflict ships.** Evidence includes priced-and-sellable Anaconda, component-manufactured Brokk, evolving/manufactured Thar'Ga, a historical zero-price Dvergr, and official event ship rewards/unlocks. That does **not** make Stargem's deliberately restricted rules wrong. Resolve whether its promises concern a selected arcade catalog or faithful Star Conflict coverage. [S13, S18–S19, S21, S23]
4. Most findings here are **explicit scope decisions or future coverage gaps**, not bugs: `ontology/domain.md:19–23` excludes progression and economy beyond ship price, and defers faction/tier, the ship tree, module pricing and purchase. Social organizations are not currently represented. Do not implement them from this audit.
5. Freshness varies substantially. Official-hosted wiki pages are community-maintained evidence, not guaranteed current rules. Several foundational pages were last edited in 2019–2023. The report retains their revision IDs, conflicts and historical qualifications rather than merging them into an invented single live ruleset.

## Authority, boundaries and method

Read first: root `AGENTS.md` (present); `docs/HANDOFF.md` (absent). The earlier kickoff inventory said both were absent at its inspection; the newly present authorized `AGENTS.md` now governs the audit. Read the requested ontology skill in full, the supplied inventory, `ontology/domain.md`, `ontology.md`, `ontology/model.rs`, `ontology/instances/ship_models.json` and `ontology/instances/game_modes.json`. Canonical authority remains `ontology/`; `ontology.md` is local source narrative, not external game evidence. Existing untracked `AGENTS.md` and `docs/ontology/` were preserved.

Three initial navigation angles were productive: **(a)** official-hosted wiki main page → progression/economy/social categories; **(b)** official homepage → dated service/monetization announcements; **(c)** Steam product identity/features → independent storefront corroboration of crafting/corporations. A bounded gap-closing pass followed concrete ship examples, experience, resources, Ellydium and trading; MediaWiki full-text search resolved the missing trading title. No search snippet is used as factual evidence. No unbounded crawl, package installation, external AI CLI, branch, commit, push, source edit or nested agent was used.

Retrieval command pattern: `curl -sS -L --connect-timeout 10 --max-time 40 --retry 0 -A 'Mozilla/5.0' -o <scratch>.html -w '%{http_code}' <exact-url>`, followed by `markitdown <scratch>.html > <scratch>.md`. Initial seed requests used a 35-second maximum; API discovery used 30 seconds. HTML was read only after conversion. Tables used below retain sufficient labels, quantities and currency image-link targets in Markdown; no fallback parser was needed. Some large rank/stat tables have flattened rowspans; this report deliberately does not derive numeric progress curves from those malformed conversions.

Managed scratch: `/tmp/stargem-progression-507d64bc/` contains HTML, converted Markdown, `fetch.py`, two API responses and a JSONL manifest. It is supporting scratch, not the durable research deliverable. Exact URLs, revision IDs, excerpts/paraphrases and claims are retained below so this report stands independently.

## Stanford step 1 — competency questions and scope answers

| Audit question | Answer established by this research | Local consequence |
|---|---|---|
| Is every ship acquired by paying one positive scalar price? | No as a universal documentary description: credits, GS, manufacturing components, event rewards and historical zero-price ships occur. [S05, S13, S18–S19, S21, S23] | Q1/price/C10 describe the local selected purchase model, not all acquisition routes. |
| Does an acquired ship remain usable across battles; can it be sold? | Repair after destruction supports persistence across battles; Anaconda has an explicit selling price, and UMC's description includes selling ships. [S04, S07, S21] | Persistence is compatible; unconditional “owned forever” is stronger than those sources. |
| Are ship rank, ship experience and pilot clearance the same? | No: separate ship performance/eligibility, per-ship development and account unlock systems. Old tier grouping is separately labeled historical. [S05, S15–S16] | All remain progression exclusions, not missing mandatory fields. |
| Do faction, manufacturer and player corporation mean one thing? | No: political/lore affiliation, ship producer and player social organization are different relationships; Ellydium overlaps organization and manufacturer functions. [S04, S11, S17, S19, S21, S23] | Avoid one mutually exclusive enum for all meanings if later adopted. |
| Are all effective ship statistics immutable model base plus fitted passive deltas? | That is sufficient for the local rule, but not universal external fidelity: experience, implants and evolution nodes can change combat properties. [S06, S16–S17, S23] | Intentional simplification; optional expansion only after scope approval. |
| Is construction just another cash purchase? | Recipes require component quantities; some have research/manufacturing charges and configurable results. [S09, S19–S20, S23] | Do not replace multi-resource requirements with an unexplained integer. |
| Are dreadnoughts ordinary individual-owned hangar ships? | The wiki describes corporation-only construction and up to three dreadnoughts per corporation, used in territorial conflict. [S12] | Separate ownership category if ever in scope, not a reason to break ordinary player-ship ownership now. |
| Can pre-2026 monetization and progression documentation be called current? | Not without qualification; June 2026 explicitly changes monetization and September announces an impending service transition. [S13–S14] | Choose a reference era before any fidelity reconciliation. |

Primary consumers are later ontology reviewers/content authors, not an implementation pipeline. No gameplay adoption is implied by the answers.

## Source register — fetched original evidence

**Metadata convention:** Every source below was retrieved **2026-09-16**, HTTP 200, and converted with `markitdown`. **W** means Star Conflict Wiki, **community-maintained, official-hosted** at `wiki.star-conflict.com`, not a publisher announcement. **O** means the official Star Conflict site, published under the Gaijin Games Kft./StarGem site branding. **P** means the Steam product listing, developer/publisher-supplied product description hosted by Valve (listing identifies Star Gem Inc. and Gaijin Network Ltd). Wiki dates are displayed **last-edit dates**, not proof that every sentence was reverified then. Original article dates are unknown unless stated. `oldid` identifies the fetched wiki revision; append `&oldid=…` to the corresponding canonical title URL for revision addressing. Unquoted evidence below is a precise paraphrase, not a quotation.

### S01 — Main Page (navigation/context)
- URL: https://wiki.star-conflict.com/index.php?title=Main_Page
- Type: **W**. Last edit **2024-01-19 14:24**; `oldid=14636`.
- Heading: introductory “About the game”; “General information”.
- Evidence: describes corporations taking sectors and gathering resources for advanced ships; links factions, advancement, implants, crafting, workshop and community.
- Supports: discovery structure and broad contextual systems. Not evidence that all details on linked pages remain current.

### S02 — Star Conflict homepage (dated index)
- URL: https://star-conflict.com/en/
- Type: **O**, official news/navigation index; page update date unknown. Entries visibly dated **2026-09-14**, **2026-06-01**, and others.
- Heading: “News”.
- Evidence: links original local-server and disabling-monetization announcements; index says the latter starts June 1, 2026.
- Supports: discovery and service chronology; important claims were checked against S13/S14 originals, not inferred from headlines alone.

### S03 — Star Conflict on Steam
- URL: https://store.steampowered.com/app/212070/Star_Conflict/
- Type: **P**. Visible release date **2013-02-27**; description update date unknown.
- Heading: “About This Game” / “The whole world for PVP and PVE!” / “Key Features”.
- Short excerpt: “Production of modules and ships!”
- Evidence: also explicitly lists corporations' battles for influence and crafting items/ships, plus creation of corporations to fight for territories.
- Supports: these are genuine Star Conflict concepts, not Star Citizen systems. Store text also says “9 tactical roles”; that marketing description should not be treated as an exhaustive current taxonomy in preference to detailed ship documentation. No current commercial availability conclusion is drawn from its “Free to Play” button.

### S04 — Factions
- URL: https://wiki.star-conflict.com/index.php?title=Factions
- Type: **W**. Last edit **2019-10-23 06:31**; `oldid=10084`.
- Headings: “General information”; “Main”; “Additional”; “Faction Traits”; “Subfactions”.
- Short excerpt: “an arbitrary combination of ships of different factions.”
- Evidence: Empire, Federation and Jericho are the main factions; UMC, Ellydium and pirates are additional categories. The page lists numerous lore subfactions, e.g. Empire's Steel Legion/Wardens of the Emperor, Federation's Armada/Ellydium/United Trade Union, Jericho's Sentinels/Mendes Family/Revenants/Cybers. UMC's traits include buying, **selling** and repairing ships; a pilot starts as a Center member. Ellydium appears both under Federation subfactions and as an additional faction.
- Supports: mixed-faction fleets, a richer affiliation graph than “one pilot chooses exactly one faction forever,” and a selling capability. Does not establish a universal one-parent subfaction tree or a current exhaustive faction list. Faction performance descriptions are broad flavor, not hard constraints on every ship.

### S05 — Advancement
- URL: https://wiki.star-conflict.com/index.php?title=Advancement
- Type: **W**. Last edit **2025-11-01 17:00**; `oldid=16026`.
- Headings: opening; “Clearance level required for different unlocks”; “Rank”; “Currency”; “Ships and Experience”; “Specialized Vessels”.
- Short excerpt: “Before you can buy a ship you must earn enough experience with one of its prerequisite ships to unlock it.”
- Evidence: 25 clearance levels; clearance points from ship levels, Ellydium nodes, module upgrades and achievements. Examples: workshop/Federation at 6; Jericho/conquest at 7; UMC at 8; Ellydium at 9; fourth combat slot at 12; extra crews at 15. Faction rank is described as highest owned ship rank for that faction. Credits buy non-premium ships; premium ships are described as GS purchases without a specific ship prerequisite and at maximum experience; special-project ships/destroyers require manufacturing. Premium components may be battle rewards.
- Supports: distinct prerequisite, currency, clearance and experience concepts. Real-money GS wording is superseded for current availability by S13; do not infer every ship still uses the listed acquisition path in September 2026.

### S06 — Implant (requested alias: Implants)
- URL fetched: https://wiki.star-conflict.com/index.php?title=Implants
- Canonical title URL: https://wiki.star-conflict.com/index.php?title=Implant
- Type: **W**. Last edit **2023-09-26 12:41**; `oldid=14239`.
- Headings: “Implants and Crews” → “Crews”, “Implants”.
- Short excerpt: “There are three different implants at each rank and you can only choose one”.
- Evidence: four crew cards store implant selections and can be assigned to combat ship slots. The highest owned ship rank limits equippable implants; an implant only takes effect on a ship of its rank or higher. Rows correspond to Empire/Federation/Jericho themes, not a rule that ship faction must equal implant faction. Example: rank-1 Empire “Gigas” is listed as +25% hull strength.
- Supports: pilot/crew configuration distinct from physical passive fitting and additional sources of effective stats. Acquisition text mentions credits/GS for extra crews; compare S05/S22's clearance unlocks rather than assuming a single presently verified purchase rule.

### S07 — Ship construction
- URL: https://wiki.star-conflict.com/index.php?title=Ship_construction
- Type: **W**. Last edit **2021-08-08 15:24**; `oldid=11874`.
- Headings: “Tech levels (old) and ship ranks”; “Ship equipment”; “Ship repairing”.
- Short excerpt: “When you buy a new ship, you get a pre-set with it”.
- Evidence: that preset includes weapons, modifiers, active/passive modules and missiles/mines. Old tiers grouped ranks in threes, except T5 ranks 13–17. Destruction produces damage that can be repaired between battles; repair cost and auto-repair are described.
- Supports: historical tier terminology, nonempty stock fittings and persistence after combat loss. Its reputation-to-rank explanation is not silently merged with the newer clearance/prerequisite account in S05. Despite its title, this page is chiefly anatomy/fitting; S09 is the actual manufacturing guide.

### S08 — Premium content
- URL: https://wiki.star-conflict.com/index.php?title=Premium_content
- Type: **W**. Last edit **2023-09-26 12:29**; `oldid=14229`.
- Headings: “Premium License”; “Premium Ships”; “Gaijin Store”; “Bundle Store”.
- Evidence: documents premium licenses, DLCs, premium ships, customization, random-content bundles, and real-money/GS acquisition. Premium ships can also be assembled from special parts. It uses “synergy” for progression gains while S05/S16 use “experience.” Its judgments about competitive balance are community commentary, not an established no-pay-to-win axiom.
- Supports: historical monetization categories and alternate premium acquisition; NOT current purchasability. S13 is an explicit later change, not an unexplained contradiction to be hidden.

### S09 — Crafting
- URL: https://wiki.star-conflict.com/index.php?title=Crafting
- Type: **W**. Last edit **2019-10-23 06:20**; `oldid=10059`.
- Headings: “General”; “How to create your ship”.
- Evidence: recipes use components acquired through Open Space, containers and disassembly. Manufacture requires sufficient clearance and components; some ships also use unique event/player-traded components. The construction UI can select a special, experience bonuses and modifier slot layout (at most three of any type); choices vary by ship. Refit costs credits without reacquiring all parts.
- Supports: structured recipes, unlock requirements and instance configuration beyond paying a scalar base price. Not proof that all manufactured ships permit every choice or that the 2019 fee schedule is current.

### S10 — Workshop
- URL: https://wiki.star-conflict.com/index.php?title=Workshop
- Type: **W**. Last edit **2026-06-10 17:49**; `oldid=16550`.
- Headings: introduction; “Weapon systems”; “Components” → “Ship Parts”.
- Short excerpt: “Available from 6 clearance level.”
- Evidence: manufacture of advanced weapons, modules, missiles and ship parts; an Mk.4 weapon is an input to an available Mk.5 upgrade; ship parts can be made for manufactured and Ellydium ships.
- Supports: crafting is not synonymous with a ship storefront, and progression gates manufacturing. Only topic-relevant sections were used; the long catalog is not treated as a verified exhaustive September 2026 inventory.

### S11 — Community
- URL: https://wiki.star-conflict.com/index.php?title=Community
- Type: **W**. Last edit **2024-03-14 13:16**; `oldid=14839`.
- Headings: “Social Menu”; “Squad”; “Wing”; “Corporation”; “Corporate wars”.
- Evidence: friendship requires consent; following does not; ignoring hides communication. Squads are temporary groups of 1–4; wings can be 8–12 depending on context. Corporations are persistent player groups, with 12 capacity levels from 30 to 500, a listed 1500-iridium creation cost and iridium-funded upgrades. Corporate wars use corporate defense and iridium; corporate titles and war points are not identical to account clearance or ship rank.
- Supports: separate social edge types and temporary/permanent organizations. Capacities/prices are wiki-dated, not freshly playtested. Do not force every wing to 12 when S12 describes 5–8 for its specific mode.

### S12 — Conquest (requested alias: Sector Conquest)
- URL fetched: https://wiki.star-conflict.com/index.php?title=Sector_Conquest
- Canonical title URL: https://wiki.star-conflict.com/index.php?title=Conquest
- Type: **W**. Last edit **2023-11-12 11:21**; `oldid=14386`.
- Headings: “Dreadnoughts”; “Types of dreadnoughts”; “Dreadnought building”; “Battle results. Owning a location”.
- Short excerpt: “Construction of a dreadnought is a long and expensive process that is only available to corporations.”
- Evidence: each corporation can have three dreadnoughts; the used dreadnought determines conflict affiliation, with influence lost on change. Types include Maelstrom, Liberator and Nemesis; Nemesis is attributed to Jericho Techs. Construction lists four iridium-funded stages totaling 6500 corporate iridium. Defeated dreadnoughts evacuate rather than being destroyed. Location control depends on influence; owner income, challengers' shares and pilot distributions are distinct relationships.
- Supports: corporation-owned strategic assets, territorial control/rewards and separate corporate resource accounting. The page contradicts itself on build timing (“a few days to a week” versus 16h48m per stage); no exact duration should be adopted from it without reconciliation.

### S13 — Star Conflict 1.14.10. Disabling project monetization
- URL: https://star-conflict.com/en/news/3928-star-conflict-1-14-10-disabling-project-monetization-en
- Type: **O**, official dated announcement/patch note. Published **2026-06-01**, version **1.14.10**.
- Heading: “Disabling project monetization”.
- Short excerpt: “the monetization of the project is disabled starting on June 1, 2026.”
- Evidence: in-game resources/currency remain usable until shutdown; ongoing events make nearly all ships obtainable, with some premium/event ships unlocked for everyone. “Legacy of the Precursors” describes limited-time Xenochips, sequential reward levels, ship parts/resources/free experience, and withdrawal of the event currency at event end. Marathon rewards include named ships, GS, iridium, monocrystals, xenocrystals and credits.
- Supports: strong dated historical change, event acquisition/reward routes, expiring event currency, and the need to separate real-money acquisition from continued currency use. “Nearly all” must not be rewritten as “all”; a 24-hour store reward availability window is not evidence of a 24-hour ship rental.

### S14 — Star Conflict: local server and a new indie game set in the same universe
- URL: https://star-conflict.com/en/news/3933-star-conflict-local-server-and-a-new-indie-game-set-in-the-same-universe-en
- Type: **O**, official announcement. Published **2026-09-14**.
- Heading: opening / “Special version of Star Conflict”.
- Short excerpt: “the main feature of which will be the ability to run a local server.”
- Evidence: announces servers will shut down soon and a special version has been in development for months; details are promised later. A different game, Burrownauts, is also announced.
- Supports: reference-version/service-lifecycle uncertainty as of the audit date. Does NOT establish released local-server functionality, transfer of owned ships/account data, or an exact shutdown date. Burrownauts rules were not imported into this Star Conflict audit.

### S15 — Ship (requested alias: Ships)
- URL fetched: https://wiki.star-conflict.com/index.php?title=Ships
- Canonical title URL: https://wiki.star-conflict.com/index.php?title=Ship
- Type: **W**. Last edit **2025-12-28 15:17**; `oldid=16206`.
- Heading: opening parallel classifications; “Technological level (old)”.
- Evidence: class is one classification axis; technological level is explicitly old and was shown I–V; T2 is exemplified by ranks 4–6.
- Supports: tier is not simply interchangeable with role, ship experience or pilot clearance. No requirement follows to add tier to the current local model.

### S16 — Experience
- URL: https://wiki.star-conflict.com/index.php?title=Experience
- Type: **W**. Last edit **2023-09-26 12:18**; `oldid=14218`.
- Headings: “Types of experience”; “How to use experience?”; “What does the pumping of the ship do?”; “How to earn experience?”.
- Evidence: actual/unspent experience is tied to a specific ship; free experience is not. Ship levels confer performance bonuses; reaching maximum yields “Elite.” Ordinary ships earn free experience equal to 5% of earned experience, premium ships 20%; the page says a ship never flown in battle receives no experience. Elite ships are said to earn double experience.
- Supports: per-player/per-ship development and account-level spendable experience are distinct from immutable catalog balance. The exact meaning of “Fleet Power” here versus S05's “Fleet strength” wording merits verification; no production curve was copied.

### S17 — Ellydium
- URL: https://wiki.star-conflict.com/index.php?title=Ellydium
- Type: **W**. Last edit **2026-04-26 14:25**; `oldid=16362`.
- Heading: “Features of Ellydium ships”; “Ellydium ships”.
- Evidence: pilots toggle previously researched evolution nodes; first research costs resources, later toggling is free. Nodes consume a limited evolution-point budget, affect characteristics/specials, and increase ship rank. The listed ships include Thar'Ga (gunship, V–XV) and Ze'Ta (suppressor, IX–XV).
- Supports: unlock state differs from active configuration; model rank/layout is not universally one immutable value in the external game. Lore describes Ellydium as a corporation studying Alien technology, not a player-created corporation.

### S18 — Dvergr (historical counterexample)
- URL: https://wiki.star-conflict.com/index.php?title=Dvergr
- Type: **W**. Last edit **2019-10-23 05:04**; `oldid=9962`.
- Headings: price table; “General information”; “Description”.
- Evidence: buying price **0 credits**, selling price **0 credits**; Empire recon interceptor, **rank 2**; described as a pilot's first Empire ship, but explicitly **“The ship is no longer in production.”**
- Supports: documented historical zero-price records cannot pass a universal positive-price facet. Does **not** establish that Dvergr is obtainable free in September 2026 or that its displayed current rank was always its historical starting rank.

### S19 — Brokk
- URL: https://wiki.star-conflict.com/index.php?title=Brokk
- Type: **W**. Last edit **2026-06-10 17:30**; `oldid=16512`.
- Headings: “Manufacturing” infobox row; “General information”; “Description”.
- Evidence: manufacturing requires **300 ship parts** (Brokk-specific component icon); Empire engineering frigate, **rank 13**, premium technology, maximum experience on acquisition. Header lists 20% free experience, +50% credits, no repair requirement and +2 cargo spaces. Lore calls it a project of **Shipyards of Earth**.
- Supports: premium is not synonymous with direct GS purchase; manufacturer differs from faction; acquisition/reward modifiers do not belong in plain ship price. Availability and benefits remain a dated wiki description, not a live account test.

### S20 — Raw materials and Parts
- URL: https://wiki.star-conflict.com/index.php?title=Raw_materials_and_Parts
- Type: **W**. Last edit **2023-11-17 20:46**; `oldid=14457`.
- Headings: “Raw materials”; “Parts”.
- Evidence: resource examples include vanadium, impure graphite/silicon/osmium, crystal shards, enriched monocrystals and xenocrystals. The unambiguous Metal blank row lists **3 vanadium**, **free learning**, **5000-credit manufacturing cost**, and availability from other pilots, contracts and conquest. Non-Mk.1 equipment can be salvaged for parts.
- Supports: recipe inputs, unlock cost and per-manufacture cost are different properties; resource acquisition can cross combat/economy/social systems. Rowspans in other entries were not extrapolated to unsourced per-item acquisition lists.

### S21 — Anaconda
- URL: https://wiki.star-conflict.com/index.php?title=Anaconda
- Type: **W**. Last edit **2024-01-31 17:21**; `oldid=14755`.
- Headings: price table; “Equipment upon purchase”; “General information”; “Description”.
- Evidence: **2,000,000-credit buying price**, **500,000-credit selling price**; Federation engineering frigate, **rank 8**, maximum experience level 9; a product of **General Reactors**. Purchased fitting lists active modules and ship modifiers, not merely weapon/missile. Four named actives include Energy-compensating projectile, Warp Gate, Static Barrier and Energy Emitter.
- Supports: selling and nonempty stock modules are documented for a ship present locally; engineer/frigate identity agrees. Local `price: 12000` is different, but no local currency unit or game-version fidelity commitment licenses calling that a balance bug.

### S22 — Galactic Standard (requested alias: Galactic Standards)
- URL fetched: https://wiki.star-conflict.com/index.php?title=Galactic_Standards
- Canonical title URL: https://wiki.star-conflict.com/index.php?title=Galactic_Standard
- Type: **W**. Last edit **2019-11-14 15:37**; `oldid=10261`.
- Headings: “Trade”; “Fourth combat slot”; “Crew”; “How can I buy GS?”.
- Short excerpt: “GS can be used to trade resources, items, blueprints with other players.”
- Evidence: describes progressive weekly sales taxation; fourth combat slot at clearance 12 or earlier with GS at 7; extra crews at 15 or earlier with GS. Also describes GS exchange, customization and store purchases.
- Supports: restricted item/resource/blueprint trading, not general transfer of whole owned ships. Exact tax boundaries and present rates are unverified; old real-money purchase instructions are superseded by S13. Crew/payment wording is not fully aligned with S06.

### S23 — Thar'Ga
- URL: https://wiki.star-conflict.com/index.php?title=Thar%27Ga
- Type: **W**. Last edit **2025-04-26 19:35**; `oldid=15701`.
- Headings: “General information”; “Manufacturing”; “Nodes”.
- Evidence: Ellydium gunship fighter, **ranks 5–15**. Construction lists **1 Fighter cabin + 100 Xenocrystals**, with **500,000 credits** for blueprint study and **1,500,000 credits** for cabin production. Later upgrades require credits, experience, Xenocrystals and Alien composite blocks. Nodes alter rank, equipment availability and characteristics; auxiliary nodes add slots, and research precedes activation. Special-module options include Crystal hunger, Hive and others.
- Supports: a concrete multi-input manufacture/evolution example; cannot honestly be reduced to one universally sufficient purchase price and unchanging per-model configuration. Exact recipe and node costs are dated evidence, not independently current-tested.

## Stanford steps 2–6 — vocabulary, relationships and facets

### Reuse and conceptual separation

The existing local split between **ship-model**, **player-ship**, **loadout** and **hangar** remains useful. It already distinguishes catalog identity from ownership and fitting. No new package or external ontology is necessary for this audit, and no schema was added. The following vocabulary is a research map for possible later decisions, not a requested implementation:

- **Nouns/entities:** faction, subfaction, manufacturer/lore organization, pilot, ship model, owned ship, acquisition offer, recipe, component/resource, currency, clearance level, ship rank, experience balance/level, prerequisite, evolution node, crew preset, implant, corporation, membership, squad, wing, dreadnought, sector/location, influence, event reward.
- **Properties:** amount/currency, rank/clearance threshold, available/retired, premium/elite, research cost, manufacture cost, component count, node activation/evolution cost, sale eligibility/value, event expiry, social consent, membership capacity.
- **Relations/actions:** owns, produces, affiliated-with, unlocks, requires, researches, activates, manufactures, rewards, buys, sells, salvages, trades, assigns-crew-to-slot, follows, befriends, ignores, joins, controls-location.

**Hierarchy checks:** a manufacturer is not a subclass of a ship; a ship has an affiliation and can be produced by an organization. “Premium” and “manufactured” are not demonstrated disjoint ship species (Brokk is both). A lore corporation such as Ellydium is not automatically an instance of the player-group corporation class. “Rank,” “level,” and “tier” cannot be one undifferentiated numeric property. These are relationship/composition distinctions, not grounds to widen the local hierarchy now.

### Relationship/cardinality evidence

| External relationship/facet | Evidence-backed shape | Qualification / local comparison |
|---|---|---|
| Pilot → owned fleet → faction | Fleet may contain ships from multiple factions. [S04] | Does not prove one exclusive pilot-faction membership; local has no faction. |
| Ship → manufacturer / faction | Anaconda: General Reactors / Federation; Brokk: Shipyards of Earth / Empire; Thar'Ga: Ellydium. [S19, S21, S23] | Single values in these examples, not proof every historical design has exactly one producer. |
| Unlock ship → prerequisite ships | S05 says sufficient experience on **one of** prerequisite ships. | A prerequisite list may be alternatives, not “must own every predecessor”; exact graph edges not researched. |
| Pilot → clearance | S05 describes levels 1–25 with threshold points and campaign tasks. | Separate from rank 1–17 ship scale documented in rank tables; no local progression requirement. |
| Owned ship → experience | Ship-specific unspent experience and level; free experience can improve any ship. [S16] | Account-global free balance is not intrinsic immutable ShipModel data. |
| Crew → implant choice per rank | Three options at a rank, at most one selected; four crew cards; rank-gated effects. [S06] | Accessible/selected/active are different conditions. Faction rows are not exclusive fleet allegiances. |
| Recipe → inputs / charges | Multiple input quantities; research and manufacture fees can differ. [S20, S23] | Cannot imply all inputs interchangeable with a quoted credit price. |
| Pilot → social contact | Many contacts; reciprocal consent for friendship, unilateral following/ignoring. [S11] | No need to represent this to validate a present loadout. |
| Temporary group → pilots | Squad 1–4; wing limit mode-dependent, general 8–12 vs conquest 5–8. [S11–S12] | Do not turn “wing” into one unconditional fixed array. |
| Corporation → pilots / assets | Capacity levels 30–500; up to 3 dreadnoughts per corporation. [S11–S12] | Exact one-corporation-per-pilot simultaneous membership is not formally proved here. |
| Corporation → sector | Influence, owner, challengers and resource shares. [S12] | Territorial control is not ownership of a personal ship. |
| Money/resource → time/account | June 2026 resources remain usable; event Xenochips expire; corporate iridium differs from personal. [S12–S13] | Neither “currency is permanent” nor “all iridium accounts interchangeable” is safe. |

The skill's type/runtime/storage alignment is applied to existing **Rust + JSON + validator/server prose**, not imaginary TypeScript/Zod/SQL layers. `ShipModel.price` is `u32` (`model.rs:111–120`) and `Catalog::validate` rejects zero (`308–309`); a free ship cannot be represented faithfully as such. The integer width itself is not disproved: no fetched price establishes a value beyond `u32`. The issue is **required positive scalar with no denomination/acquisition semantics**, not integer overflow speculation.

`Catalog::buy` (`model.rs:405–419`) resolves a model and constructs stock equipment but has no wallet/prerequisite/debit/persistence inputs. C10 explicitly assigns charging to the future server (`domain.md:247`). This is an **implementation coverage boundary**, not proof that C10 logically permits free purchases in a completed game. There is no selling API or persisted acquisition transaction to audit.

## Candidate reconciliation ledger

Classification refers to evidence, not an instruction to fix code. “Verified documentary contradiction” means the quoted local universal claim does not describe the fetched external example; confidence in September-2026 live behavior is separately qualified. Local design intent can resolve it by explicitly retaining the simplification.

| ID | Exact local claim / anchor | Sourced comparison | Classification, confidence, impact | Reconciliation question / options |
|---|---|---|---|---|
| P01 | `domain.md:96`: `price` is `u32 > 0`; Q1 at `29`; narrative `ontology.md:11`: buy a new PlayerShip for its ShipModel price. | Brokk requires 300 parts; Thar'Ga needs cabin/Xenocrystals plus fees; official events reward/unlock ships; historical Dvergr price is zero. [S13, S18–S19, S23] | **Coverage gap / deliberate simplification** for selected local catalog; **verified documentary counterexamples** to an all-Star-Conflict acquisition claim. High documentary confidence; exact present recipe availability medium. High impact only if importing broad ship data. | Keep “selected buyable ships, local price” scope; or, if later approved, distinguish acquisition routes/requirements from display price. Do not assign fake price 1 to free/crafted ships. |
| P02 | `domain.md:163`: “owned forever by one user”; relation `200`: “1 → many, permanent”; narrative `ontology.md:12` repeats forever. | Anaconda has selling price 500,000 credits; Factions explicitly describes selling. Combat destruction is repairable. [S04, S07, S21] | **Verified documentary contradiction** if forever forbids voluntary disposal; otherwise **wording uncertainty** between durable unlock and inventory object. High documentary confidence, medium current fidelity. High ownership-semantics impact. | Does “forever” mean no permadeath/no transfer, or no disposal under any circumstance? Retain nonsellable local ships explicitly, or later separate account entitlement from currently owned inventory. Do not infer player-to-player whole-ship transfer or rentals. |
| P03 | `domain.md:247` C10: stock weapon/missile and **no modules**; `model.rs:413–417` constructs empty passive/active vectors. | Anaconda's “Equipment upon purchase” lists actives and modifiers; S07 describes a complete preset. [S07, S21] | **Verified documentary contradiction**, consistent with a **deliberate minimal starter-fitting simplification**. High against dated sources; medium live freshness. Medium content/import impact. | Is empty fitting a chosen Stargem onboarding rule? If yes document it as local. If external fidelity is desired, approve per-model stock fitting semantics before any code change. |
| P04 | `instances/ship_models.json:2`: Anaconda price `12000`; `domain.md:96` gives no currency or reference date. | Anaconda page: 2,000,000 credits. [S21] | **Uncertainty / likely local balance simplification**, NOT a demonstrated numeric bug because currency/unit and fidelity intent are unspecified. High evidence of differing numbers. Low standalone impact. | Name the local price unit and intended era; keep independent balancing, or source a dated credit price. Never blindly overwrite with a wiki number. |
| P05 | `domain.md:19–23`: progression/ship ranking excluded; ship-tree prerequisites, faction/tier deferred. `model.rs:111–120,195–201` has no development state. | Clearance, rank, experience, unlock prerequisites and Mk upgrades are distinct. [S05, S15–S16] | **Explicit out-of-scope / deliberate simplification**, not internal defect. High. Potential future import/UI impact. | Keep excluded. If later included, choose which competency question requires which progression axis; do not add one generic `level` for all. |
| P06 | `domain.md:98`: slots fixed per model; `242` C5: effective stats = base + passive modifiers; `ontology.md:10`: models immutable. | Experience/implants add effects; Thar'Ga node selection changes rank/slots/specials and stats. [S06, S16–S17, S23] | **Deliberate simplification / coverage gap** for evolved/configurable models, not an internal arithmetic contradiction. High evidence, medium exact live numbers. High if Ellydium is imported unchanged. | Keep only supported fixed-configuration models, or later put researched/active configuration on owned ships rather than mutating global catalog definitions. Combat-specific effect fidelity belongs to the combat audit. |
| P07 | `domain.md:184,248`: four optional hangar slots, no locked/unlocked distinction. | S05/S22 say fourth combat slot is progression-gated; crew cards also unlock. | **Explicit progression exclusion / deliberate simplification**. High wiki evidence; medium freshness. Low current impact. | Is every local user intentionally given all four slots? Keep so unless progression is approved. An empty slot and a locked slot are not the same concept if that changes. |
| P08 | `domain.md:23`: faction/tier future; current model has role but no faction or manufacturer. | Main/subfactions and manufacturers are separate; Ellydium overlaps faction/corporation terminology. [S04, S17, S19, S21] | **Coverage gap deliberately deferred**, not a missing mandatory taxonomy. High. Medium future naming/import impact. | Preserve exclusions; if expanded, distinguish affiliation, production provenance and social membership, and avoid assuming a disjoint exhaustive hierarchy. |
| P09 | `domain.md:19`: economy beyond ship price excluded. | Credits, GS, iridium, crafting resources, recipes, salvage, player trading, event rewards and expiries. [S05, S09–S10, S13, S20, S22] | **Explicit out-of-scope**, not bugs. High system-existence confidence; rates/caps/taxes medium/low freshness. Low present impact. | Keep the local single-price abstraction. If economy becomes in scope, choose acquisition/accounting needs before modeling every resource. |
| P10 | `domain.md:168,200`: owner is one external user; `game_modes.json:2–5` has no conquest organization model. | Corporation-built dreadnoughts, social groups and territorial rewards. [S11–S12] | **Coverage gap / scope decision**. High documentary confidence. No contradiction: local player-ship need not mean every space object in Star Conflict. | Exclude corporate strategic assets explicitly; if later supported, separate group-owned assets and relationships rather than reusing a fake user ID. |
| P11 | `domain.md:10`: “Star Conflict-like”; no reference version in `domain.md` or ship catalog. | June disables monetization; September announces shutdown/local-server work. Old wiki payment guides remain readable. [S08, S13–S14, S22] | **Historical change + uncertainty**, not local code drift by itself. High official dated evidence. High audit/provenance impact. | Choose independent inspiration, a dated historical ruleset, final online-era rules, or a later documented local-server version. Add provenance only through an approved ontology/documentation decision. |
| P12 | `model.rs:405–419` helper labeled “C10: a freshly bought ship”; C10 assigns actual price charging to server (`domain.md:247`). | Helper constructs an object only; no balance or progression enforcement exists in this path. This observation is local, not inferred from an external game. | **Implementation coverage gap**, not an established **internal logical issue** under the explicit server boundary. High static confidence. Important before a real purchase endpoint exists. | Treat it as construction, not a completed transactional purchase service. Any future purchase boundary must apply approved rules atomically; no service implementation authorized now. |

**Internal logical issue conclusion:** no new internal logical contradiction in this exclusive topic is established merely by the external systems being absent. The separate inventory's validation concerns are not duplicated or reclassified as economy defects here. Price denomination and “forever” are clarity/commitment questions; the existing nine positive-price records are internally representable.

## Agreed facts worth preserving

- Star Conflict has a catalog of ships, player acquisition, persistent progression, fitting and fleets; the local model/owned-instance separation is a useful abstraction, not itself drift. [S03, S05, S07]
- **Anaconda really is an engineering frigate**, agreeing with local `instances/ship_models.json:2–5` and role-derived size. The contextual metadata missing locally—Federation, General Reactors, rank 8, experience—is separate from that agreement. [S21]
- Ships survive as account assets beyond individual combat destruction in the repair workflow; “not consumed on every death” is defensible, even though “can never leave ownership” is not the same claim. [S07]
- Four combat ship slots are an external documented maximum/preset context, matching the local four-slot shape, while access timing is intentionally omitted locally. [S05–S06, S22]
- Ship construction can use a maximum of three modifier slots of a given type, consistent with the local per-family cap for those examples; it does not establish immutable layout for every external ship. [S09]
- Both games can use a ship with stock weapon/missile; the disagreement is whether the stock fitting also includes modules, not whether stock equipment exists. [S07, S21]

## Stanford step 7 — three representative real examples tested conceptually against the model

No JSON/catalog instances were added; these are evidence-backed representability checks.

1. **Anaconda — ordinary priced/sellable ship.** Federation engineering frigate, General Reactors product, rank 8; wiki buy/sell 2,000,000/500,000 credits; max experience 9; fitted modules upon purchase. Local engineer → frigate and catalog/owned split fit. Local `12000` has no matching currency/provenance; permanent ownership and empty stock modules do not describe the wiki lifecycle. Rank, manufacturer and experience are intentionally outside scope. [S21; local `ship_models.json:2–5`]
2. **Brokk — premium yet component-manufactured.** Empire engineering frigate, Shipyards of Earth, rank 13, 300 Brokk parts, maxed experience, reward/repair benefits. Existing `ShipRole::Engineer` can describe the role, but one positive cash-like scalar cannot preserve the parts recipe or premium benefits. This is a coverage test, not a request to add Brokk. [S19]
3. **Thar'Ga — evolving manufactured ship.** Ellydium gunship fighter ranks 5–15, cabin + 100 Xenocrystals, blueprint/cabin credit fees and research/activation nodes. Role/class fit the existing enum subset; acquisition, evolving rank/layout and alternative specials do not fit fixed per-model data without deliberate flattening into selected configurations. Whether flattening is acceptable is a product decision. [S17, S23]

**Historical edge case:** Dvergr's zero-credit record is rejected by the local positive-price validation. The page says it is no longer in production; this is a historical representability counterexample, not a claim of a presently free starter option. [S18]

## Conflicting evidence, freshness and unanswered questions

### Conflicts retained rather than silently resolved

- **Monetization:** S08/S22, and part of S05, describe real-money GS/DLC purchases; S13 explicitly disables monetization from June 2026. This is a dated **historical change**. Existing in-game currencies remaining usable does not resurrect the real-money purchase channel.
- **Rank/unlocks:** S07's older reputation-to-rank explanation and S05's newer clearance/prerequisite account differ. S05 also uses faction rank as highest owned rank. These may describe different eras or different concepts; no exact transition patch was established.
- **Crews:** S06 says extra crew cards can be unlocked using credits or GS; S05 says extra crews at clearance 15; S22 describes clearance 15 with earlier GS unlock. Plausible era/early-unlock differences exist, but current credit purchase eligibility is not verified.
- **Dreadnought construction timing:** S12's days-to-week description conflicts within the same page with 16h48m per unaccelerated stage. Do not encode either as authoritative current timing.
- **Wing size:** S11's general “up to 8–12” and S12's conquest 5–8 are context-dependent rather than automatically a contradiction.
- **Terminology:** S08 uses synergy, S16 experience; likely overlapping legacy vocabulary, but the exact rename date and complete equivalence were not established. Do not claim a verified patch-level rename. Likewise S16 “Fleet Power” and S05 “Fleet strength” should not silently be given identical numerical semantics.
- **Faction hierarchy:** S04 lists Ellydium both as Federation subfaction and an additional faction. This is overlapping lore/gameplay categorization, not proof the wiki is logically inconsistent.

### Open product/reconciliation questions (for the ledger, not an interview)

1. Is Stargem independent Star Conflict-inspired combat, or a faithful reconstruction of a specific dated ruleset? The local phrase “Star Conflict-like” favors not assuming exhaustive fidelity.
2. Does positive `price` mean local credits, an abstract balancing value, or actual source-game purchase price? Is Q1 meant to enumerate only the local buyable subset?
3. Does “owned forever” express no combat permadeath, immutable original owner, an account unlock, or inability to sell/dispose? These are distinct commitments.
4. Are stock empty modules, four immediately accessible hangar slots and absence of progression intentional onboarding simplifications?
5. If acquiring more real ships later, should the catalog omit unsupported crafted/evolving ships, select fixed variants, or add approved acquisition/configuration semantics?
6. Should manufacturer/faction lore be descriptive provenance only, with no gameplay constraints, if introduced?
7. Are corporation/dreadnought systems explicitly excluded from “ship” scope, or merely unplanned? Nothing currently demands their implementation.
8. Which service era matters after the June monetization change? The September announcement does not answer whether accounts, inventories or acquisition systems transfer to a future local-server version.

### Research limitations

- No live client/account operation was performed. Wiki purchase prices, sale behavior, upgrade costs, tax brackets, corporation capacities and precise rewards are dated documentary evidence, not runtime acceptance tests.
- No evidence supports rentals, whole-ship player trading, guaranteed post-shutdown ownership transfer, or a `u32` numeric ceiling failure. Those possibilities must not be asserted as established drift.
- No exhaustive faction/manufacturer or prerequisite-edge catalog was attempted. The sources already establish the relevant conceptual distinctions; crawling every ship would widen scope.
- `https://wiki.star-conflict.com/index.php?title=Trading` returned **404** once. This is a missing guessed title, not an inaccessible host. `Galactic Standard#Trade` provided original trading evidence. Both discovery API calls succeeded; no firewall/DNS/TLS failure occurred in this run.
- History: earlier research was reported blocked before firewall reconfiguration; parent confirmed wiki HTTP 200 at 19:36 UTC on the audit date; this run independently fetched the evidence above. Old blocked-only conclusions are obsolete for these sources.

## Gruber criteria applied

| Criterion | Audit conclusion |
|---|---|
| **Clarity** | Define price denomination/reference era and the intended meaning of permanent ownership. Keep pilot clearance, ship rank, experience level, tier, faction and manufacturer distinct. |
| **Coherence** | Current narrow rules are not contradicted merely by intentionally omitted systems. Universal external claims about positive-price acquisition/forever ownership have documented counterexamples; keep local rules and claims of fidelity separate. |
| **Extendibility** | The catalog/owned-instance distinction can support later progression without mutating global catalog data, but do not implement that speculative extension now. Premium/manufactured/evolving categories should not be prematurely disjoint. |
| **Minimal encoding bias** | A `u32` is an encoding choice; its observed limitation is missing currency/acquisition meaning, not proven insufficient numeric width. Recipes and entitlements are not naturally single integers. |
| **Minimal ontological commitment** | Retain explicit exclusions. Prefer documenting chosen simplifications/reference dates over copying crafting, monetization and corporate systems merely because the source game contains them. |

## Evidence and validation

- Local checks: `git rev-parse HEAD`, `git status --short`, `git diff --name-only`, `git diff --cached --name-only`; HEAD matched the baseline, tracked/staged diffs were empty, and only the already-authorized untracked `AGENTS.md`/`docs/ontology/` appeared.
- Web checks: 23 HTTP-200 HTML pages converted; one HTTP-404 guessed title; two HTTP-200 MediaWiki discovery responses. Exact successful URLs are S01–S23. The two discovery requests were:
  - `https://wiki.star-conflict.com/api.php?action=query&list=search&srsearch=trade&srlimit=6&format=json`
  - `https://wiki.star-conflict.com/api.php?action=query&list=search&srsearch=trading&srwhat=text&srlimit=10&format=json`
- Scratch manifest assertion checked 21 follow-up records: 20 successful conversions, sole non-200 title `trading`, all curl process exit codes 0, every successful Markdown nonempty. Three seeds were separately successfully fetched/converted before that manifest.
- Artifact structural validation passed: source IDs S01–S23 are complete and ordered; twelve P01–P12 candidates exist; the acceptance JSON parses and names exactly this output artifact; branch is `master`; tracked and staged diffs are empty.
- No software tests were added or run: this assignment authorizes research/documentation, not gameplay/source changes. No dependency installation was needed.

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "satisfied",
      "evidence": "Only the configured research artifact and external scratch were written; canonical ontology, source, branches and authorized drafts were not changed."
    },
    {
      "id": "criterion-2",
      "status": "satisfied",
      "evidence": "23 source records include exact URLs, source type, retrieval/update dates, headings and supporting evidence; twelve anchored reconciliation candidates distinguish scope decisions, uncertainty, historical changes and documentary contradictions."
    }
  ],
  "changedFiles": [
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/507d64bc-df21-4587-a4fb-a4425950ac5f/research/progression.md"
  ],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {
      "command": "git rev-parse HEAD; git status --short; git diff --name-only; git diff --cached --name-only",
      "result": "passed",
      "summary": "Baseline matched; no tracked or staged diff; authorized existing untracked files preserved."
    },
    {
      "command": "Bounded curl retrievals followed by markitdown conversion for S01-S23",
      "result": "passed",
      "summary": "23 original HTML pages retrieved with HTTP 200 and converted; two MediaWiki discovery calls also returned 200."
    },
    {
      "command": "curl https://wiki.star-conflict.com/index.php?title=Trading (bounded, one attempt)",
      "result": "failed",
      "summary": "HTTP 404 missing guessed title; resolved with fetched Galactic Standard Trade section, not a network blocker."
    },
    {
      "command": "Python scratch-manifest assertions",
      "result": "passed",
      "summary": "20 successful follow-up conversions and one expected missing title; successful Markdown files nonempty."
    },
    {
      "command": "Python artifact structure/acceptance-JSON assertions and git branch/diff checks",
      "result": "passed",
      "summary": "23 ordered source records, 12 candidates, valid acceptance JSON; master branch; no tracked or staged diff."
    }
  ],
  "validationOutput": [
    "Observed successful network access replaces obsolete access-blocked conclusions.",
    "No source/ontology edits or staged files."
  ],
  "residualRisks": [
    "Wiki revision dates do not guarantee September 2026 live behavior; no client/account tests performed.",
    "Official June monetization shutdown and September upcoming local-server announcement require an explicit reference-era decision.",
    "Review gate remains for the parent reviewer; this research does not approve semantic changes."
  ],
  "noStagedFiles": true,
  "diffSummary": "One comprehensive external research artifact; no repository diff.",
  "reviewFindings": [
    "No research-delivery blocker; product reconciliation questions are recorded rather than enacted."
  ],
  "manualNotes": "Most topic omissions are explicit domain.md exclusions. Do not convert this evidence ledger into gameplay implementation without ontology reconciliation."
}
```
