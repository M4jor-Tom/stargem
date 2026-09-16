# Handoff: research-ships — f0850e7a

Checkpoint: 2026-09-16, user-requested pause. Parent-curated from runtime records and actual files; not a new completed child report.

- **State:** COMPLETED; reuse output, do not rerun by default
- **Role:** `delegate`
- **Child run:** `f0850e7a-239c-4ccd-b741-f7fcf4c34c95`
- **Workflow / key:** `507d64bc-df21-4587-a4fb-a4425950ac5f` / `research-ships`
- **Mission:** `28a33a06-b296-4a78-8ae0-079881816489`
- **Repository:** `/home/theta/repos/stargem.nix`, branch `master`
- **Source baseline:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`
- **Session (machine-local, optional for recovery):** `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/2026-09-16T18-47-01-030Z_01a0ab8b-30a5-761c-90d3-cc64125796e1/ecf9c95d-7fab-4fc6-86cc-e80d20d5f7fc/run-0/session.jsonl`
- **Runtime status (machine-local):** `/tmp/pi-subagents-uid-1000/async-subagent-runs/f0850e7a-239c-4ccd-b741-f7fcf4c34c95/status.json`

## Authority and boundaries

Read root `AGENTS.md` and `docs/HANDOFF.md` before work. `ontology/` remains canonical and unchanged. This is documentation/research, not authorization to change gameplay. No commit/push/merge/install is authorized. One writer per checkout. Preserve untracked audit files: they are the current work, not disposable clutter. Other agents' full research is intentionally file-backed to protect parent context.

## Resumption (for Agent)

Research is complete for this pass. Reuse the original report below and `../research/ships.md`; do not refetch everything. Synthesis/review are still pending. Only resume this child for a specific missing citation, disputed claim or fresh evidence request. Keep publication/patch date distinct from retrieval date and deliberate local scope distinct from defects.

## Native continuation or fallback

Native resumability was verified with children.list at checkpoint. Recheck it in the next session; retained session availability can change.

```javascript
subagent({ action: "status", id: "f0850e7a-239c-4ccd-b741-f7fcf4c34c95" })
subagent({ action: "children.list" })
// Only when resumable, and no original writer is still active:
subagent({ action: "resume", id: "f0850e7a-239c-4ccd-b741-f7fcf4c34c95",
  message: "Read AGENTS.md, docs/HANDOFF.md and docs/ontology/handoffs/research-ships-f0850e7a.md. Continue ONLY the remaining work recorded there; preserve completed outputs and current files." })
```

If the retained session is missing or not resumable, do not claim a native resume or rerun all research. Launch a fresh `delegate` as a **same-role fallback continuation**, give it this handoff and the preserved reports, and start at the exact next step above. A resumed run may return a new run ID: record it and continue from the latest identity.

## Verification and acceptance

The original report below states exactly what this child checked and what remains uncertain. Completion of a research child does not mean that document synthesis, independent evidence review, ontology review or user reconciliation has completed. Follow [the main checkpoint](README.md) for dependency order and pending reviews.

## Preserved original output

Original managed artifact: `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/507d64bc-df21-4587-a4fb-a4425950ac5f/research/ships.md`

SHA-256 of original artifact: `ce192eec72e41f36c2df8b2f2000ff6bbf9ded43ba5d0e0e082c26a8d7d3965d`

The complete output is embedded below so recovery does not depend on session/temp-file retention. Original relative code paths refer to the repository root unless the report says otherwise. Historical access failures stay historical.

---

# Star Conflict ship taxonomy and special-module audit

**Audit/retrieval date:** 2026-09-16. **Repository:** `/home/theta/repos/stargem.nix`, baseline `4a47fa3015f2d31bbe1c5a3159a3d0980c273912` (HEAD verified). Research/documentation only; no approved ontology decisions or gameplay changes.

## Executive findings

- Network retry succeeded: **29 HTTP-200 retrievals**, including official announcements and official-hosted wiki pages. The sole HTTP 404 was a case-sensitive guessed wiki title, corrected through the main-page link. Earlier firewall-blocked conclusions are historical, not this audit's outcome.
- **Eight of nine exact-name local ship examples have a different role in the retrieved wiki.** Anaconda agrees. Four also have a different derived class. This is a verified documentary mismatch, not proof that an independently designed Stargem catalog must change.
- The familiar **nine roles map to the same three classes locally and in the wiki**, but the game documentation additionally lists **destroyers / Suppressor**. Missing destroyers are a coverage/scope question, not an internal inconsistency.
- **One role → exactly one immutable special is not a universal Star Conflict rule.** Long Range has distinct disintegrator and guided-torpedo specials; ordinary named ships have selectable alternatives; Ellydium ships have multiple special-module forms.
- The largest behavioral mismatch is ECM: local “freezes every ship around” versus a self-protective immobile phase followed by an **enemy-only** paralysis pulse. Engineer healing is a genuine but incomplete part of Combat Drones. Phase Shield and Diffusion Shield require clearer activation semantics; cloak documentation includes more termination conditions and an initial protected interval.
- Wiki edit dates range from **2019 to 2026**. These are retrieved documentary facts, **not independently confirmed running-client behavior on 2026-09-16**. An official 2026-09-14 announcement says server shutdown is forthcoming and a local-server version is being developed; neither a shipped local-server ruleset nor completed shutdown is established here.

## Authority, inspection, and research method

Root `AGENTS.md` was read first; `docs/HANDOFF.md` was checked immediately afterward and is absent. Read the requested ontology skill completely and applied the Stanford seven steps and Gruber criteria below. Read the supplied inventory, then independently read all of `ontology/domain.md`, `ontology.md`, `ontology/model.rs`, `ontology/instances/ship_models.json`, and `ontology/instances/special_modules.json`. Local line references below refer to those baseline files.

`ontology/` is canonical. `ontology.md` supplies the project's narrative, not an authoritative Star Conflict specification. `domain.md:10` explicitly says **“Star Conflict-like”**; faction/tier and progression are deferred/excluded at `domain.md:19–23`. Existing untracked `AGENTS.md` and `docs/ontology/` were preserved. No commits, staging, branches, package installs, source changes, ontology edits, or nested agents.

Four starting angles: wiki Main Page (navigation), Ships (taxonomy), special-module index discovery, and official news homepage (freshness). Followed the known nine local ship names and Ellydium taxonomy. One targeted gap-closing pass fetched original module pages for drones, faction-specific Long Range specials, ECM, cloak, shield activation, and named drone variants, plus the official recent announcement. This produced 26 substantive wiki pages, one official article, and two navigation pages; the extra pages beyond the approximate 10–20 target specifically close nine-name and nine-role requirements, not a crawl quota. No search-engine snippets were used as evidence; successful official indexes made search/API fallback unnecessary. Steam was not needed for this specialized lane.

Every network request used `curl -L --connect-timeout 10 --max-time 40 -sS`, with HTTP status and effective URL recorded. Every fetched HTML page was converted with installed `markitdown` before reading. Conversion retained the relevant class/role labels, linked special identities, prose, and comparison tables; no HTML-parser fallback was needed. Large equipment/evolution lists outside this lane were not exhaustively analyzed. Scratch HTML, Markdown and fetch log reside at `/tmp/stargem-ships-20260916/`; this report contains the durable findings and citations, so acceptance does not depend on scratch retention. Fetched pages were treated solely as untrusted evidence.

### Classification convention

- **Verified contradiction/documentary mismatch:** the explicit local statement and explicit fetched statement cannot both describe the same ship/rule. Confidence refers to the documentary comparison; game-version fidelity remains separately qualified.
- **Deliberate simplification:** the narrative/canonical model explicitly chooses a narrower rule. Evidence can show non-equivalence without invalidating that design.
- **Coverage gap:** a documented concept/behavior has no local representation, without assuming exhaustive fidelity was promised.
- **Uncertainty:** ambiguous meaning, unsupported freshness, or conflicting sources prevents resolution.
- **Historical change:** requires dated change evidence; a page's edit date alone does not establish when mechanics changed.
- **Internal logical issue:** a local definition/constraint and its own representation/enforcement disagree, independent of game fidelity.

## Source register

**Common metadata for every source below:** retrieved **2026-09-16**, HTTP **200**, converted with `markitdown`. **W** means *Star Conflict Wiki, community-maintained official-hosted wiki on `wiki.star-conflict.com`*, not a signed developer announcement. **O** means *official Star Conflict publisher/developer website, announcement/editorial*, not community evidence. Wiki “updated” dates are visible last-edit dates, not patch dates; underlying patch applicability is **unknown** unless specifically stated. `oldid` is the revision marker visible in the fetched page, allowing later reconstruction by adding `&oldid=...` to its canonical URL. Short excerpts are quoted; unquoted descriptions are precise paraphrases.

| ID | Title, exact retrieved URL, type | Visible date / revision | Relevant heading; short evidence; claim supported |
|---|---|---|---|
| W01 | **Ship** (redirect from Ships), https://wiki.star-conflict.com/index.php?title=Ships ; W | Updated 2025-12-28 15:17; `Ship` oldid 16206 | Class: lists Interceptors, Fighters, Frigates, Destroyers and linked role icons. Supports four-class taxonomy and the nine familiar role assignments plus Suppressor. |
| W02 | **Special module** (redirect from Special Modules), https://wiki.star-conflict.com/index.php?title=Special_Modules ; W | Updated 2026-05-09 09:52; `Special_module` oldid 16431 | Introduction/Main/Unique replaceable special modules: normally attached to a ship, with explicit replaceable exceptions. Supports role defaults rather than a universal role-special bijection. Individual behaviors below use original module pages where fetched. |
| W03 | **Destroyers**, https://wiki.star-conflict.com/index.php?title=Destroyers ; W | Updated 2026-01-25 11:54; oldid 16284 | Unique characteristics / General / Faction and rank features: heavy fire-support ships with destructible components; faction variants, Ze'Ta and unique ships. Supports omitted class and differences within that class. |
| W04 | **Factions**, https://wiki.star-conflict.com/index.php?title=Factions ; W | Updated 2019-10-23 06:31; oldid 10084 | Faction Traits / ALIENS: Empire emphasizes hull/firepower, Federation speed/maneuverability, Jericho shields; Ellydium uses alien technology; Aliens described separately. Supports design tendencies, not universal numerical axioms or a current exhaustive roster. |
| W05 | **Ellydium**, https://wiki.star-conflict.com/index.php?title=Ellydium ; W | Updated 2026-04-26 14:25; oldid 16362 | Features of Ellydium ships / Ellydium ships: selectable evolution nodes change characteristics and specials; roster spans all four classes. Supports a faction/technology dimension distinct from class and role. |
| W06 | **Anaconda**, https://wiki.star-conflict.com/index.php?title=Anaconda ; W | Updated 2024-01-31 17:21; oldid 14755 | General information / Special module: Federation, Frigate, Engineering, rank 8; Combat Drones. Supports the one agreeing local ship-role example. |
| W07 | **Styx**, https://wiki.star-conflict.com/index.php?title=Styx ; W | Updated 2024-01-31 17:31; oldid 14776 | General information / Special module: Empire, Frigate, Engineering, rank 9; Combat Drones and 'Styx' Combat drones. Contradicts local Long Range role and supplies a same-ship variant example. |
| W08 | **Mammoth**, https://wiki.star-conflict.com/index.php?title=Mammoth ; W | Updated 2026-06-10 17:33; oldid 16523 | General information / Description / Special module: Federation, Frigate, Engineering, rank 15, premium; Combat Drones. Contradicts local Guard role. |
| W09 | **Hydra**, https://wiki.star-conflict.com/index.php?title=Hydra ; W | Updated 2022-05-27 14:41; oldid 12738 | General information / Special module: Empire, Frigate, Long Range, rank 6; Disintegrator. Contradicts local Tackler/fighter identity. This is Hydra, not Hydra 2. |
| W10 | **Phoenix**, https://wiki.star-conflict.com/index.php?title=Phoenix ; W | Updated 2026-06-10 17:30; oldid 16514 | General information / Description / Special module: Empire, Frigate, Engineering, rank 15, premium; Combat Drones or 'Phoenix' Drones. Lore describes pirate engineering. Contradicts local Gunship/fighter and cautions against equating origin lore with infobox faction. |
| W11 | **Lynx**, https://wiki.star-conflict.com/index.php?title=Lynx ; W | Updated 2025-06-07 10:18; oldid 15773 | General information / Special module: Federation, Fighter, Tackler, rank 1; ODG 'Chameleon'. Contradicts local Command, while agreeing on class. |
| W12 | **Shrike**, https://wiki.star-conflict.com/index.php?title=Shrike ; W | Updated 2025-11-27 11:17; oldid 16063 | General information / Manufacturing → Special module: Federation, Frigate, Guard, rank 17; choose Phase shield or 'ELM-50' generator. Contradicts local Covert Ops/interceptor. |
| W13 | **Wolfhound**, https://wiki.star-conflict.com/index.php?title=Wolfhound ; W | Updated 2023-12-26 14:37; oldid 14574 | General information / Manufacturing → Special module: Federation, Fighter, Gunship, rank 16; choose Overdrive or Archangel. Contradicts local Recon/interceptor. |
| W14 | **Kite**, https://wiki.star-conflict.com/index.php?title=Kite ; W | Updated 2019-12-10 15:23; oldid 10283 | General information / Special module: Federation, Interceptor, Covert Ops, rank 9; Plasma web. Contradicts local ECM, while agreeing on class. Old page: patch currency unconfirmed. |
| W15 | **Thar'Ga**, https://wiki.star-conflict.com/index.php?title=Thar%27Ga ; W | Updated 2025-04-26 19:35; oldid 15701 | General information / Development history / Nodes: Ellydium Gunship fighter, ranks 5–15; five alternative special forms including Hive. Supports stable role alongside configurable specials. |
| W16 | **Tai'Kin**, https://wiki.star-conflict.com/index.php?title=Tai%27Kin ; W | Updated 2026-04-26 19:59; oldid 16410 | General information / Development history / Nodes: Ellydium Recon interceptor, ranks 5–15; five alternative movement-oriented specials, including Quantum leap. Supports alternatives to a single universal microwarp. |
| W17 | **Waz'Got**, https://wiki.star-conflict.com/index.php?title=Waz%27Got ; W | Updated 2025-04-26 19:51; oldid 15703 | General information / Development history / Nodes: Ellydium Engineering frigate, ranks 5–15; five special forms, including wreckage-driven drones and Green mist. Supports behavior unlike uniform shield-healing drones. |
| W18 | **Combat Drones**, https://wiki.star-conflict.com/index.php?title=Combat_Drones ; W | Updated 2022-04-27 15:20; oldid 12554 | Description: up to two periodically generated drones attack enemies, repair the host below half shield, and expend a drone to restore nearby allied shields on activation. Supports passive + active lifecycle, not just a heal button. |
| W19 | **Disintegrator**, https://wiki.star-conflict.com/index.php?title=Disintegrator ; W | Updated 2022-04-30 10:44; oldid 12622 | For ship type / Description / Features: Empire Long Range special, thermal sniper weapon; ship cannot move, first shot charges, laser sight and fixed zoom. Lists Hydra as compatible. |
| W20 | **Guided torpedo**, https://wiki.star-conflict.com/index.php?title=Guided_torpedo ; W | Updated 2025-11-13 16:23; oldid 16054 | For ship type / Description / Features: Jericho Long Range special launches a controllable explosive torpedo and leaves a damaging cloud; cloud duration **8 s**. Supports distinct Long Range identity and conflicts with W02's 10-second summary. |
| W21 | **Metastable Energy Field Generator**, https://wiki.star-conflict.com/index.php?title=Metastable_Energy_Field_Generator ; W | Updated 2025-08-16 09:41; oldid 15932 | Description: invincible but immobile/unable to act for 6 s, then paralyzes enemies within 1500 m for 2 s; repeated control has diminishing returns. Supports staged ECM behavior and enemy-only targeting. |
| W22 | **Phase shield**, https://wiki.star-conflict.com/index.php?title=Phase_shield ; W | Updated 2024-06-13 15:30; oldid 15139 | Description / Characteristics: selected damage resistance, retaliation damage bonus, activation speed bonus; table says “Energy consumption per each switch.” Supports mode switching with per-switch cost, not necessarily on/off toggling. |
| W23 | **Diffusion Shield**, https://wiki.star-conflict.com/index.php?title=Diffusion_Shield ; W | Updated 2023-12-22 11:33; oldid 14534 | Description / Characteristics: absorbs damage using energy for up to 15 s; disappears if energy depletes; damage absorbed per energy varies by rank. Supports timed damage absorption, not an established constant per-second upkeep. |
| W24 | **ODG 'Chameleon'**, https://wiki.star-conflict.com/index.php?title=ODG_%27Chameleon%27 ; W | Updated 2024-12-25 16:24; oldid 15498 | Description: invisibility lasts 18 s; damage/opening fire end it; “The first 2 s. the invisibility can't be interrupted by any action”; post-cloak damage bonus. Supports more nuanced cloak behavior than C14. |
| W25 | **'Styx' Combat drones**, https://wiki.star-conflict.com/index.php?title=%27Styx%27_Combat_drones ; W | Updated 2022-04-27 16:25; oldid 12574 | Description: activation restores shield to the most damaged ally within 2000 m, destroys enemy devices, and consumes a drone. Supports different targeting from ordinary area restoration. |
| W26 | **'Phoenix' Drones**, https://wiki.star-conflict.com/index.php?title=%27Phoenix%27_Drones ; W | Updated 2023-03-06 12:11; oldid 13387 | Description / Characteristics: three drones, manual Attack/Repair modes, shield and hull restoration, projectiles reduce hull resistance. Supports a non-press-heal engineer special. |
| O27 | **Star Conflict: local server and a new indie game set in the same universe**, https://star-conflict.com/en/news/3933-star-conflict-local-server-and-a-new-indie-game-set-in-the-same-universe-en ; O | Published 2026-09-14; patch date not specified | Opening / Special version of Star Conflict: shutdown forthcoming, local-server version being developed. Supports audit version caution only; does not establish new ship mechanics. |
| N28 | **Main Page**, https://wiki.star-conflict.com/index.php?title=Main_Page ; W, navigation | Updated 2024-01-19 14:24; oldid 14636 | Ships / Ship equipment: navigable links to four classes and correctly cased Special Modules. Discovery source; not evidence of current balance. |
| N29 | **Star Conflict official homepage**, https://star-conflict.com/en/ ; O, news index | Page update unknown; article dates visible through 2026-09-14 | News links to O27 and other dated updates. Discovery/freshness source only; article body, not index snippet, used for O27 claims. |

### Retrieval failures and chronology

- Initial `https://wiki.star-conflict.com/index.php?title=Special_modules` returned HTTP **404**, curl exit 0 (curl without `--fail` reports transport success). This is a missing/case-sensitive title, **not network denial**. The discovered `Special_Modules` URL returned 200 and redirected internally to the article titled Special module. No repeated host-failure retries.
- All other 29 requests returned HTTP 200; all `markitdown` conversions exited 0. No TLS, firewall, timeout, or installation blocker in this lane.
- Prior blocked-only reports are superseded for these sources. Their earlier access limits do not support any present conclusion about inaccessible Star Conflict documentation.

## Stanford step 1 — scope and competency questions

**Purpose:** compare Stargem's ship-model/class/role/special conceptualization to fetched Star Conflict documentation, supplying reconciliation choices without deciding game rules. Users are the ontology owner, content authors, and later implementation/review agents. Economy, matchmaking, combat balance formulas, complete progression implementation, and NPC AI are outside this lane.

| Question | Answer from this audit / modeling requirement |
|---|---|
| Which ship classes and roles exist in the compared documentation? | Four player-ship classes, familiar nine roles plus Suppressor [W01/W03]. Distinguish current local subset from purported game completeness. |
| Does role determine class? | The nine shared roles have the same class mapping [W01]; Suppressor is associated with destroyers. No contrary role-class example found in this bounded sample. This is not a proof across every historical/NPC object. |
| Does role uniquely determine the exact special module? | No, in the documented game [W07/W12/W13/W15–W17/W19/W20]. Role may supply a default family; a model/configuration resolves the exact special. |
| Are the nine local model names faithful examples? | Only Anaconda's role agrees; eight differ, four across classes (table below). Shared names do not by themselves prove intended imported identity. |
| Are special behaviors adequately identified by press/toggle plus one sentence? | Enough for deliberately approximate design labels, not enough for staged ECM, per-switch phase shield, controllable torpedo, or persistent drones. |
| Are faction, class, role, acquisition category, and alien technology interchangeable? | No. Ellydium spans four classes and several roles; Phoenix has Empire classification and pirate-origin lore [W05/W10]. Keep independent dimensions. |
| Which differences are actual contradictions rather than omissions? | Exact-name role discrepancies and ECM target/sequence mismatch are positive comparisons; absent destroyers/factions/variants are scope/coverage issues unless universal fidelity is promised. |
| What dated source/version should future fidelity target? | Unanswered. Wiki dates are not patch guarantees, O27 announces a transition, and the local narrative specifies no Star Conflict release. |

## Stanford steps 2–6 — reuse, vocabulary, hierarchy, properties, constraints

### Reuse existing conceptual structure

The existing `ShipSize`, `ShipRole`, `SpecialModuleKind`, `ShipModel`, `SpecialModule`, JSON instances, and validation functions already provide the local framework. No new package, schema language, inheritance framework, or imported game database is needed to make the product decisions. Reuse the readable IDs and existing role-class table where fidelity is desired; retain a mapping between local aliases and sourced terminology. Do not silently change canonical enums from this report.

### Domain terms and relationships

- **Entities:** ship model, owned ship, ship configuration/evolution form, class, role, faction/technology origin, special module, drone, selected damage type, controlled torpedo, effect phase.
- **Properties:** class/size, role, named special identity, available special choices, faction affiliation, activation input, duration, targeting policy, damage type, energy cost, cooldown, drone count, self-repair condition, rank/version/provenance. Rank is evidence context here, not a proposed progression implementation.
- **Relations/actions:** model has role/class; model permits special; configuration selects special; faction supplies technology; module creates drones/projectiles, absorbs damage, switches resistance, hides ship, suppresses enemies, warps/teleports ship.

**Hierarchy/is-a test:** a destroyer is a ship class, not a role sibling of Gunship; Suppressor is its role. An Ellydium ship is still a fighter/interceptor/frigate/destroyer: faction/technology is an independent property or relation, not a replacement size. A special module is something a ship **has**, not a superclass of the ship. A named drone variant is not a new Engineering role. Premium/crafted/customizable labels likewise should not become size categories.

| Relation/facet | Local commitment | What evidence permits saying |
|---|---|---|
| Model → role | Exactly one, `domain.md:91–101,197` | Each inspected model has one listed role; no need to loosen it based on this research. |
| Role → size/class | Functional many-to-one, `domain.md:195,214–218`; `model.rs:30–35` | Shared mapping agrees. “Class” is the game term; local “size” is a usable abstraction but not a measured dimension. |
| Role ↔ special | Bijection, `domain.md:196,220–232`; exhaustive `model.rs:38–49` | Exact identity is not universally functional in Star Conflict. Long Range has at least two baseline special types, and named models may offer alternatives. |
| Model → available specials | Not represented | At least one; can be several [W07/W12/W13/W15–W17]. Bounded sample establishes multiplicity, not a global maximum. |
| Configured ship → selected special | Derived solely from role; absent from loadout [C17] | W12/W13 explicitly select one alternative, not equip all listed specials simultaneously. Preserve this distinction if scope changes. |
| Special → lifecycle | `press` or `toggle`; effect string, `domain.md:145–153`, `model.rs:67,104–109` | Input gesture, mode selection, duration, phases, and passive behavior are distinct concepts. Evidence does not require a particular implementation encoding. |
| Special → strength | No typed rank/weapon-level relation | Several pages tie damage to installed main-weapon level [W18–W20/W26]. Omission is expected with deferred progression, not a current logical failure. |
| Faction → ship tendencies | Deferred, no local field | W04 gives qualitative tendencies; W03 gives destroyer-specific differences. Neither justifies fixed universal stat multipliers for all models. |

Facets were checked against Rust + serde/JSON + `Catalog::validate`, not fictitious TypeScript/Zod/SQL layers. Exhaustive matches correctly enforce the local nine-role design; the final role test enforces distinct special kinds (`model.rs:473–478`). It does **not** validate Star Conflict realism. Local catalog-vs-role coherence and external named-ship fidelity must remain separate.

## Agreed taxonomy and nine-role special comparison

The shared mapping agrees [W01]: **interceptor → Recon, Covert Ops, ECM; fighter → Gunship, Command, Tackler; frigate → Engineering, Long Range, Guard**. Local singular `engineer` corresponds to the wiki's Engineering label, and `ecm`/`covert-ops` are normalization choices, not substantive defects.

Each table row compares role defaults, **not** a universal claim that every ship of that role has that special. Numerical values are source fingerprints, not recommendations for Stargem balance.

| Role; local special row | Retrieved identity and behavior | Assessment |
|---|---|---|
| Engineer; `special_modules.json:2`: Repair Drones / press / “Heals shield of allies in range on press.” | **Combat Drones**: two persistent drones, periodically produced, attack or repair the host when shield falls below half; activation expends one for allied shield restoration [W18]. Styx prioritizes a damaged ally and clears devices [W25]; Phoenix switches three drones between attack and shield/hull repair [W26]. | Local heal statement agrees with one baseline action. Drone lifecycle, passive action, targeting variants and hull repair are coverage gaps. “Repair Drones” is a local label, not established exact identity. |
| Long Range; `special_modules.json:3`: Sniper Mode / toggle / regular-style firing | **Disintegrator** is an Empire sniper special; it immobilizes the ship, charges its first shot, and has laser sight/zoom [W19]. **Guided torpedo** is a Jericho controllable explosive special with residual cloud [W20]. | Sniper abstraction roughly fits one family, not the whole role. Torpedo is a special, not merely the ordinary missile-slot model. |
| Guard; `special_modules.json:4`: Phasic Shield / toggle / cycles three resistance types | **Phase shield** selects shield resistance; switching costs energy, with additional weapon and speed effects [W22]. | Core selected-resistance identity agrees. Terminology and “toggle” need definition; bonus omissions are simplification, not contradictions. |
| Tackler; `special_modules.json:5`, C14: Cloak ends on damage | **ODG 'Chameleon'** has duration, damage/open-fire cancellation, an initially uninterruptible interval as phrased by the page, and an exit damage bonus [W24]. | Core concealment agrees. C14 is stronger than the sourced qualified behavior if intended universally; firing cancellation is missing. Exact scope of “any action” deserves clarification. |
| Gunship; `special_modules.json:6`: Overclock / press / boosts weapons and motors | **Overdrive** boosts firing rate, energy regeneration and movement for a duration [W02, corroborated W13's Overdrive choice]. | Good approximate identity; local name differs, and exact affected stats are underspecified. Not proof of wrong mechanics. |
| Command; `special_modules.json:7`: Command Shield / toggle / drains energy instead of shield HP | **Diffusion Shield** absorbs damage through energy, expires after 15 seconds or depleted energy [W23]. | Core resource substitution agrees. Local word “drains” does not settle damage-driven vs constant upkeep. Toggle alone does not establish or refute a duration cap. |
| Covert Ops; `special_modules.json:8`: Plasma Web / press / target damage-over-time | **Plasma web** deals thermal damage over time, with a target-speed condition affecting damage [W02]. | Core DoT agrees; damage typing and conditional strength absent. No need to infer a contradiction. |
| Recon; `special_modules.json:9`: Hyper-Propulsion / press / distant warp | **Microwarp engine** briefly accelerates the ship after preparation [W02]; Tai'Kin instead has alternative temporal/teleportation specials [W16]. | Approximate battlefield relocation agrees. Ordinary microwarp should not automatically inherit Tai'Kin's teleport/time-rewind semantics. |
| ECM; `special_modules.json:10`: Electromagnetic Surge / press / “Freezes every ship around for a short while.” | **Metastable Energy Field Generator** first makes its host invulnerable and unable to act; when that phase ends it paralyzes nearby **enemies**, with control stacking limitations [W21]. | Positive behavioral mismatch in who is affected and the sequence. Not simply missing numbers or renamed artwork. |

Aliases worth recording if retained: local **Overclock → Overdrive**, **Command Shield → Diffusion Shield**, **Hyper-Propulsion → Microwarp engine**, **Electromagnetic Surge → Metastable Energy Field Generator**, **Phasic Shield → Phase shield**, **Cloak → ODG 'Chameleon'**, **Sniper Mode → Disintegrator-like behavior**. These are analytical correspondences, not approved renames; notably the last is not all Long Range ships.

## Stanford step 7 — concrete ship-instance comparison

All nine local instances were inspected directly. Class is derived from role locally, so a role correction can also affect weapon-size validation; it is not always a cosmetic data edit.

| Model / exact local anchor | Local role → derived class / special | Wiki role → class / faction / special | Classification and confidence |
|---|---|---|---|
| Anaconda, `ship_models.json:2–5` | engineer → frigate / drones | Engineering → Frigate / Federation / Combat Drones [W06] | Agreed identity, high documentary confidence; behavior only partial. |
| Styx, `ship_models.json:6–9` | long-range → frigate / sniper-weapon | Engineering → Frigate / Empire / Combat Drones or 'Styx' Combat drones [W07/W25] | Verified role/special contradiction under same-name fidelity, high. |
| Mammoth, `ship_models.json:10–13` | guard → frigate / phasic-shield | Engineering → Frigate / Federation / Combat Drones [W08/W18] | Verified role/special contradiction under same-name fidelity, high. |
| Hydra, `ship_models.json:14–17` | tackler → fighter / cloak | Long Range → Frigate / Empire / Disintegrator [W09/W19] | Verified role/class/special contradiction under same-name fidelity, high. |
| Phoenix, `ship_models.json:18–21` | gunship → fighter / overclock | Engineering → Frigate / Empire / Combat Drones or 'Phoenix' Drones [W10/W26] | Verified role/class/special contradiction under same-name fidelity, high. |
| Lynx, `ship_models.json:22–25` | command → fighter / command-shield | Tackler → Fighter / Federation / ODG 'Chameleon' [W11/W24] | Verified role/special contradiction under same-name fidelity, high. |
| Shrike, `ship_models.json:26–29` | covert-ops → interceptor / plasma-web | Guard → Frigate / Federation / Phase shield or 'ELM-50' generator [W12] | Verified role/class/special contradiction under same-name fidelity, high. |
| Wolfhound, `ship_models.json:30–33` | recon → interceptor / hyper-propulsion | Gunship → Fighter / Federation / Overdrive or Archangel [W13] | Verified role/class/special contradiction under same-name fidelity, high. |
| Kite, `ship_models.json:34–37` | ecm → interceptor / em-surge | Covert Ops → Interceptor / Federation / Plasma web [W14] | Verified role/special contradiction against that page, high documentary confidence; currency weaker because page last edited 2019. |

**Three representative validation cases for a later ontology decision:**

1. **Anaconda / baseline Engineering frigate:** the local role/class fits W06. W18 adds passive drone attack and self-repair before the shield-restoration command. Validates that “simplified” need not mean “wrong”; preserve the accepted baseline if narrow scope is intentional.
2. **Wolfhound / selectable special:** W13 explicitly requires choosing one of **Overdrive or Archangel**. Archangel adds missile volleys and a different strength boost. Correcting only its local Recon role to Gunship still cannot represent the choice with `ShipRole::special()`; role identity and model/configuration choice are separate issues.
3. **Thar'Ga / Ellydium Gunship fighter:** W15 keeps the Gunship role while offering Crystal hunger, Hive, Combat reconstructor, Condensing crystals, or Crystal predator. Hive produces drones that return to repair hull; this is not an Engineering role change. The current enum can classify the role but cannot record its actual special form.

Additional cross-checks: **Tai'Kin** remains Recon/interceptor while Quantum leap restores an earlier position/state [W16]; **Waz'Got** remains Engineering/frigate while Green mist protects allies from projectiles and explosions [W17]. Those are missing configurations, not counterexamples to the existing role-size relation.

## Destroyers, faction differences, and alien/special ships

### Destroyers

W01/W03 provide affirmative evidence for a fourth class, paired with **Suppressor**. Destroyers are not simply renamed Guard frigates: W03 describes heavy fire support, 12 installed turrets with at most 8 firing concurrently, destructible ship components, and special handling/control modes. None of those numeric/mechanical details is a recommended local requirement.

W02 describes **Energy router** variants that prioritize movement, weapon damage, or shield recovery, and **Spatial stabilizer** for some higher-rank destroyers. The router's faction variants differ. This supports treating exact special identity separately from the class and role; it does not require representing every subsystem now.

W03 lists Invincible/Brave/Vigilant/Emperor (Empire), Procyon/Antares/Sirius/Albireo (Federation), Archon/Sibyl/Tyrant/Relic (Jericho), **Ze'Ta** (Ellydium), and unique destroyers including Saturn. These are evidence of coverage, not a promise of an exhaustive September 2026 roster. Ze'Ta has evolution-node special selection; Saturn also has configurable progress-tree specials, so configurability is **not necessarily exclusive to Ellydium**.

### Faction tendencies and classification cautions

W04's older general page describes Empire hull/firepower, Federation mobility/capacitor, and Jericho shielding; W03 independently describes corresponding destroyer-specific distinctions. Treat those as qualitative design tendencies, not universal invariants overriding a model's own stats. Local per-model bases can intentionally approximate ships without storing faction.

Do not conflate:
- **Faction affiliation vs lore origin:** Phoenix's infobox says Empire while its description calls it pirate-engineered [W10].
- **Alien technology vs alien NPC species:** Ellydium is the corporation producing customizable alien-tech player ships [W05]. W04 separately discusses Crystallides and Biomorphs. This audit did not inspect a complete NPC-ship taxonomy; no alien NPC role/class rules are inferred.
- **Acquisition/status vs combat role:** premium Mammoth and Phoenix are Engineering frigates [W08/W10]; customizable Wolfhound and Shrike retain Gunship/Guard roles [W12/W13]. No pricing/currency conclusions are part of this lane.

The 2026 Ellydium page lists **seven ships**: Tai'Kin/Recon and Tau'Kita/Covert Ops interceptors; Thar'Ga/Gunship and Yith'Mor/Command fighters; Waz'Got/Engineering and Drag'Thir/Guard frigates; Ze'Ta/Suppressor destroyer [W05]. This is that page's roster, not a verified final-service census. W02 lists special alternatives for these later forms too. Their absence locally is a coverage question, not evidence that nine local roles are logically invalid.

## Candidate reconciliation ledger

No item authorizes implementation. Resolve the intended identity/scope/version in `ontology/` before changing Rust or JSON. **Impact** describes consequences *if Star Conflict fidelity is chosen*; independent-game divergence may be accepted instead.

### SHIP-01 — Eight exact-name role assignments disagree

- **Local:** `ship_models.json:6–37` supplies the eight mappings quoted in the instance table; `model.rs:30–49` derives their class and special.
- **External:** W07–W14 give explicit different roles. Four also change class (Hydra, Phoenix, Shrike, Wolfhound).
- **Classification:** verified documentary contradiction under an identity-preserving interpretation; **not an internal logical issue**. **Confidence:** high for statements read; unproven for all current patches. **Impact:** high—wrong default ability, eligible role modules, and sometimes size-based weapon compatibility.
- **Question/options:** Are these imported Star Conflict identities, or local archetypes merely borrowing names? If imported, reconcile individual roles/class-sensitive stock equipment canonically; if independent, label/rename the examples or explicitly document the divergence. Do not silently replace eight entries based on this report.

### SHIP-02 — Role-special bijection is overgeneralized for game fidelity

- **Local:** `domain.md:146` says “Built into the role; one per role”; `domain.md:196` cardinality `1 ↔ 1`; C17 `domain.md:254` says derived from role; `ontology.md:44` says every model inherits its role's special. `model.rs:38–49,473–478` hard-code/test this.
- **External:** W19/W20 distinguish two Long Range specials; W12/W13 explicitly select among alternatives; W15–W17 offer multiple specials while preserving role.
- **Classification:** **deliberate simplification** in local rules; a verified counterexample to a universal Star Conflict interpretation. **Confidence:** high. **Impact:** high—new faithful ship instances cannot be expressed by JSON alone.
- **Question/options:** Keep a role default and explicitly exclude variants; permit model-level allowed specials plus a selected special; or model only fixed per-model special identity if selection is intentionally omitted. Retain one selected special versus many available choices; do not assume simultaneous use.

### SHIP-03 — Missing destroyer/Suppressor is a scope gap

- **Local:** `domain.md:60,63–64,214–218`, `model.rs:12,16,30–35`, `ontology.md:20–23`: three sizes and nine roles only.
- **External:** W01/W03 list Destroyers and Suppressor, plus mechanically distinctive ships.
- **Classification:** coverage gap / bounded simplification, **not** a logical defect or false statement about the chosen local subset. **Confidence:** high. **Impact:** medium/high only for broader game coverage.
- **Question/options:** Declare the three-class roster intentional; or approve destroyer/Suppressor coverage before considering components and specials. No claim established about introduction date; “later” must not be assigned an invented patch.

### SHIP-04 — ECM targeting and sequence differ

- **Local:** `special_modules.json:10`: “Freezes every ship around for a short while”; `ontology.md:33` agrees. Role mapping is `model.rs:48`.
- **External:** W21: host invulnerability/immobility and action lock first; then nearby **enemy** paralysis.
- **Classification:** verified behavioral contradiction if these describe the same special, with additional coverage gaps. **Confidence:** high documentary comparison. **Impact:** high—friendly control, survivability, and timing.
- **Question/options:** Keep Electromagnetic Surge as an original area-freeze ability and label the divergence; or approve Metastable semantics including target filter and distinct phases. Numerical tuning is separate.

### SHIP-05 — Long Range lacks faction-specific identity

- **Local:** `special_modules.json:3`: sniper weapon that “fires like a regular weapon while on”; `domain.md:225`, `ontology.md:26` assign it to all Long Range.
- **External:** W19 has charged immobile Disintegrator firing; W20 has controlled explosive torpedo and lingering cloud.
- **Classification:** deliberate simplification / coverage gap; the universal sniper interpretation contradicts documented torpedo identity. **Confidence:** high identity, lower for precise patch parameters. **Impact:** high—different player interaction, projectile/control state, targeting and hazards.
- **Question/options:** Explicitly support the Disintegrator-like branch only; or distinguish Long Range special variants. Do not treat special torpedo as ordinary `missile_id`, and do not import a cloud duration until source conflict is resolved.

### SHIP-06 — Engineer “drones” collapses several real behaviors

- **Local:** `special_modules.json:2` and `ontology.md:25` only specify shield healing on press.
- **External:** W18's ordinary drones also attack/repair the host and are consumed/replaced; W25 prioritizes one ally; W26 uses manual attack/repair modes and hull restoration.
- **Classification:** coverage gap / simplified baseline, not contradiction to ordinary activation healing. **Confidence:** high. **Impact:** medium/high if persistent units or faithful variants matter.
- **Question/options:** Keep an abstract heal and say drones are presentation only; or approve lifecycle/target policy and named variants. Decide whether drones are entities before creating runtime state.

### SHIP-07 — Cloak rule lacks qualified termination semantics

- **Local:** `domain.md:251` C14: “`cloak` turns off when the cloaked ship takes damage”; `special_modules.json:5` repeats it.
- **External:** W24 adds firing cancellation, finite duration, post-cloak bonus, and states the first two seconds cannot be interrupted by any action.
- **Classification:** firing/duration are coverage gaps; unconditional C14 versus the initial interval is a **candidate contradiction with wording uncertainty**. **Confidence:** medium for the exception's exact damage scope, high that documentation contains it. **Impact:** medium—stealth timing and offense.
- **Question/options:** Keep intentionally simple damage-break cloak; or define the initial protection, what “any action” covers, fire/damage cancellation and expiration against a chosen release. Do not infer damage immunity from invisibility protection.

### SHIP-08 — Phase Shield activation vocabulary is ambiguous

- **Local:** `special_modules.json:4`: `activation: "toggle"` yet “Cycles improved resistance”; `domain.md:152` only enumerates press/toggle, while the distinct *active-module* flow at `domain.md:123–125` associates toggling with ongoing drain.
- **External:** W22 explicitly uses selected resistance and energy **per switch**, plus timed side effects.
- **Classification:** uncertainty / clarity issue, **not proven internal contradiction** because special `Activation` has no on/off formal semantics. **Confidence:** high ambiguity. **Impact:** medium—state transitions and resource accounting.
- **Question/options:** Does special toggle mean on/off, cycle among modes, or merely repeat activation? Clarify those meanings before implementing; do not reuse ongoing active-module drain semantics by analogy alone.

### SHIP-09 — Diffusion Shield is timed damage absorption, not established upkeep

- **Local:** `special_modules.json:7`: toggle, “Additional shield that drains energy instead of shield HP”; C16 `domain.md:253`; narrative `ontology.md:30` says activation on a key press.
- **External:** W23 gives finite duration and energy spent to absorb damage; depletion ends it.
- **Classification:** uncertainty plus duration/ratio coverage gap; no proof that manual early deactivation is impossible. **Confidence:** high documented core, medium implied local behavior. **Impact:** medium—resource and duration semantics.
- **Question/options:** Specify damage-driven absorption vs constant upkeep and whether toggle permits early cancellation; preserve a deliberately abstract energy shield if desired. Do not equate `Activation::Toggle` with `ActivationFlow::Ongoing` automatically.

### SHIP-10 — Ellydium/special identity crosses role boundaries

- **Local:** `domain.md:95` stores one role with derived size; `domain.md:146,196,254` fixes special by role; no evolution configuration. Progression and faction/tier are explicitly excluded/deferred (`19–23`).
- **External:** W05/W15–W17 show class/role-stable ships with selectable forms; Thar'Ga's Hive is a Gunship drone special, Waz'Got's Green mist is Engineering protection, Tai'Kin includes temporal return.
- **Classification:** coverage gap, not permission to implement deferred progression. **Confidence:** high. **Impact:** high if these ships are later admitted unchanged.
- **Question/options:** Explicitly exclude evolving/unique ships; or decide a minimal special-selection representation independently of implementing their entire research/economy systems. Do not infer role from the visible presence of drones alone.

### SHIP-11 — Faction traits require scoped statements

- **Local:** faction/tier deferred `domain.md:23`; `ShipModel` has no faction property (`model.rs:112–120`), but individual bases already exist.
- **External:** W04's qualitative traits and W03's destroyer-specific differences; W10 has Empire classification and pirate-origin lore.
- **Classification:** explicit out-of-scope / future coverage, not current defect. **Confidence:** high deferral, medium current generality of old faction page. **Impact:** low today; higher for provenance or faction-dependent specials.
- **Question/options:** Keep faction absent; or later distinguish affiliation, manufacturer/lore origin, and alien technology rather than forcing them into one exclusive enum without definitions.

### SHIP-12 — One-special cardinality not fully validated in the local catalog

- **Local:** `domain.md:146,196` expects one special per role; `Catalog.special_modules` is a public vector (`model.rs:235`); `model.rs:277–282` applies duplicate-ID checks to six families but not specials; `model.rs:299–304` requires only `.any(...)` presence for each role. `model.rs:474–477` tests mapping uniqueness, not uniqueness of actual catalog rows.
- **External:** none needed; this is an internal inspection finding.
- **Classification:** internal logical issue at the validation boundary / cardinality enforcement gap. A duplicate row with the same special kind and conflicting effect text is representable and not rejected by the inspected checks. Current nine instances are unique.
- **Confidence:** high from code inspection; **not executed as a Rust mutation test** in this research lane. **Impact:** medium for malformed content; no demonstrated failure of shipped data.
- **Question/options:** Confirm the one-record-per-special-kind invariant and validate it, or explicitly define whether multiple definitions are allowed. Do not confuse this duplicate-record concern with legitimate separately identified game variants. No fix authorized here.

## Conflicts, historical evidence, and remaining questions

1. **Guided torpedo duration conflict:** W02 (wiki edit 2026-05-09) says cloud **10 s**; W20 (2025-11-13) says **8 s**. Same retrieval date, different article detail. A newer index edit is not proof its row is newer or correct. Identity/control/cloud existence are consistent; precise duration stays unresolved.
2. **Summary omission is not necessarily conflict:** W02 cloak summary lacks W24's initial interval; the detailed page qualifies the simpler description. Phase speed bonus is described more precisely as decreasing in W22 and W12 than in the W02 summary. Preserve the precision instead of asserting a patch change.
3. **Faction is not a clean lore partition:** W04 lists Ellydium in Federation subfactions and separately under additional factions; W05 describes its alien-tech identity. W10 labels Phoenix Empire while its lore calls it pirate-derived. These motivate disambiguating affiliation/technology/origin, not selecting one source and discarding another.
4. **Historical change with actual dated evidence:** O27, 2026-09-14, announces forthcoming server shutdown and work on a local-server version. This changes the freshness question; it does **not** establish that either transition was complete on audit day or that ship mechanics changed. In-universe years in ship lore are not real release/patch dates.
5. **Unresolved product scope:** Which specific Star Conflict release, if any, is the fidelity target? Are the catalog's exact names meant to preserve real identities or only inspire placeholders? Is the nine-role/class subset intentionally permanent? Is special selection allowed at model construction, hangar refit, or not at all? These are report questions for ontology reconciliation, not requests for immediate gameplay work.
6. **Unverified boundaries:** No live-client test, exhaustive ship census, comprehensive NPC/alien taxonomy, full patch-history reconstruction, or numeric balance certification was performed. None is needed to establish the positive name/role and special-cardinality counterexamples above. Do not invent historical “used to be” explanations for those mismatches.

## Gruber review

| Criterion | Assessment tied to this lane |
|---|---|
| **Clarity** | Shared role-size mapping is clear. “Class” vs “size,” locally renamed specials, activation gesture vs lifecycle, and copied ship names require explicit identity notes. W24's wording and W20/W02 conflict need annotations, not assumptions. |
| **Coherence** | Current local instances are consistent with their declared local roles even where external identities disagree. The duplicate special-definition validation gap is a separate internal issue. No cyclic class hierarchy was found. |
| **Extendibility** | Data arrays admit more ships, but exact special variants and destroyer roles require changes to exhaustive enums/matches. Decide whether that intentionally bounded design is adequate rather than introducing a general ability framework preemptively. |
| **Minimal encoding bias** | Kebab-case IDs and composition are readable. `press`/`toggle` and “5th slot” are input/layout encodings, not complete semantics of phased effects, selectable modes, or module identity. Clarify concepts before adding code. |
| **Minimal ontological commitment** | Keep three classes/nine roles and omit factions/alien evolution if that meets Stargem's intended scope. Conversely, do not describe the simplification as universal Star Conflict truth. A role default is a weaker and better-supported claim than a universal exact-special bijection if fidelity is required. |

## Validation and acceptance evidence

- Independent direct inspection of the five relevant canonical/narrative/model/instance files, plus prescribed inventory and skill.
- Bounded curl/markitdown retrieval in three batches: 30 attempts, 29 HTTP 200, one corrected-title 404. The source register records useful originals, headings, excerpts/paraphrases, dates and revision markers.
- Read-only Python assertion loaded all nine local ship rows and compared them with manually sourced exact-name roles: **9 rows, 8 role mismatches**, Anaconda agrees. This is a reproducible documentary comparison, not an independently authoritative game-data import or executable game test.
- `git diff --exit-code` and `git diff --cached --exit-code` both succeeded. `git status --short` retained only authorized untracked `AGENTS.md` and `docs/ontology/`; no repository files were changed by this lane.
- Artifact validation parsed the fenced acceptance JSON, checked all 29 source IDs and citations, and reconciled the 30 fetch-log entries to 29 HTTP 200 / one HTTP 404. Branch rechecked as `master`.
- No gameplay tests added or run; no gameplay/source logic changed. Review should check the source/date qualifications and canon-first reconciliation boundaries, not interpret this artifact as an approved rules patch.

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "satisfied",
      "evidence": "Completed ship taxonomy/special-identity research only; one configured output artifact, no repository or ontology edits."
    },
    {
      "id": "criterion-2",
      "status": "satisfied",
      "evidence": "29 successful fetched sources with URLs, publisher types, retrieval/update dates, revisions and claim evidence; all nine local ships compared, 12 anchored reconciliation items, Stanford/Gruber review and explicit limitations."
    }
  ],
  "changedFiles": [
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/507d64bc-df21-4587-a4fb-a4425950ac5f/research/ships.md"
  ],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {
      "command": "git status --short; git rev-parse HEAD; command -v markitdown; command -v curl",
      "result": "passed",
      "summary": "Baseline verified; authorized untracked files preserved; required tools available."
    },
    {
      "command": "curl -L --connect-timeout 10 --max-time 40 -sS -w '%{http_code} %{url_effective}' -o <scratch.html> <source URL> (30 bounded requests)",
      "result": "passed",
      "summary": "29 HTTP 200; one missing-title HTTP 404, resolved using the correctly cased wiki navigation link. No network blocker."
    },
    {
      "command": "markitdown <scratch.html> -o <scratch.md> (each fetched HTML)",
      "result": "passed",
      "summary": "All conversions exited 0; required taxonomy and special descriptions readable."
    },
    {
      "command": "python3 read-only catalog assertion: assert len(rows)==9 and len(role_mismatches)==8",
      "result": "passed",
      "summary": "Eight of nine exact-name model roles differ from the cited wiki; Anaconda agrees."
    },
    {
      "command": "git diff --exit-code && git diff --cached --exit-code && git status --short",
      "result": "passed",
      "summary": "No tracked or staged changes; only pre-existing authorized untracked work."
    },
    {
      "command": "python3 artifact/source-log assertions; git branch --show-current",
      "result": "passed",
      "summary": "Valid acceptance JSON, 29 unique source IDs, consistent source references, 29 HTTP 200 plus one HTTP 404, branch master."
    }
  ],
  "validationOutput": [
    "Sources accessible on retry; former blocked-only conclusion superseded.",
    "9 ship instances checked; 8 role mismatches and 4 derived-class mismatches.",
    "Conflicting guided-torpedo cloud durations retained as unresolved."
  ],
  "residualRisks": [
    "Wiki last-edit dates do not certify September 2026 live-client mechanics; source freshness varies.",
    "Product fidelity/version and borrowed-name intent remain unresolved; no semantic change is authorized.",
    "Special-row duplicate validation finding is static inspection, not an executed Rust mutation test.",
    "Independent reviewer gate remains required."
  ],
  "noStagedFiles": true,
  "diffSummary": "Added only the external ship-research artifact; no repository diff.",
  "reviewFindings": [
    "No infrastructure or authority blockers; substantive candidate findings are recorded for reconciliation, not implementation."
  ],
  "manualNotes": "Preserved AGENTS.md and docs/ontology drafts. Economy excluded. Scratch captures are temporary; source evidence needed by the later writer is embedded in this report."
}
```
