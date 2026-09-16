# Handoff: research-modes — beaf67dc

Checkpoint: 2026-09-16, user-requested pause. Parent-curated from runtime records and actual files; not a new completed child report.

- **State:** COMPLETED / SUPERSEDED: network-blocked attempt
- **Role:** `delegate`
- **Child run:** `beaf67dc-12eb-436d-adcc-621ca63573b8`
- **Workflow / key:** `08aa52d7-d2a2-4091-b856-28049dd1b39f` / `research-modes`
- **Mission:** `28a33a06-b296-4a78-8ae0-079881816489`
- **Repository:** `/home/theta/repos/stargem.nix`, branch `master`
- **Source baseline:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`
- **Session (machine-local, optional for recovery):** `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/2026-09-16T18-47-01-030Z_01a0ab8b-30a5-761c-90d3-cc64125796e1/1a58eb63-a713-4185-8a0a-fc58ab9db8d0/run-0/session.jsonl`
- **Runtime status (machine-local):** `/tmp/pi-subagents-uid-1000/async-subagent-runs/beaf67dc-12eb-436d-adcc-621ca63573b8/status.json`

## Authority and boundaries

Read root `AGENTS.md` and `docs/HANDOFF.md` before work. `ontology/` remains canonical and unchanged. This is documentation/research, not authorization to change gameplay. No commit/push/merge/install is authorized. One writer per checkout. Preserve untracked audit files: they are the current work, not disposable clutter. Other agents' full research is intentionally file-backed to protect parent context.

## Resumption (for Agent)

No remaining work for this attempt. Use [research-modes-2f322aa2.md](research-modes-2f322aa2.md) instead. The user fixed the firewall; do not propagate this attempt's access-blocked conclusion as current.

## Native continuation or fallback

Recheck live availability before any resume. Completed work normally requires no resume.

Not rechecked in the latest retained-child list; no resume is needed. Use the preserved report below. If genuinely necessary, inspect status/children.list first; otherwise start a same-role focused continuation from this handoff.

## Verification and acceptance

The original report below states exactly what this child checked and what remains uncertain. Completion of a research child does not mean that document synthesis, independent evidence review, ontology review or user reconciliation has completed. Follow [the main checkpoint](README.md) for dependency order and pending reviews.

## Preserved original output

Original managed artifact: `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/research/modes.md`

SHA-256 of original artifact: `b8b3248f46809d48e17b92bc9887c13d5a78eab9ce4706c2ce87e65454318148`

The complete output is embedded below so recovery does not depend on session/temp-file retention. Original relative code paths refer to the repository root unless the report says otherwise. Historical access failures stay historical.

---

# Star Conflict modes, deployment and session relations — PARTIAL AUDIT

**Audit/retrieval-attempt date:** 2026-09-16. **Repository:** `/home/theta/repos/stargem.nix`, branch `master`, baseline/observed HEAD `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`.

**Result: internet verification blocked.** All five bounded HTTP attempts failed to connect. No external article, wiki page, Steam description, search result or update date was retrieved. Therefore this report establishes **no verified Star Conflict contradiction, no verified live mode rules, and no current/removed/seasonal status**. Its useful output is an independently checked local baseline, exact research questions, source-access record, and a scoped reconciliation queue. It must not be presented as completed live-game research.

Root `AGENTS.md` and `docs/HANDOFF.md` were checked first and are absent. The requested ontology skill and supplied inventory were read, followed by the relevant files directly. `ontology/` remains canonical; `ontology.md` is local source narrative, not evidence of the actual game. No repository files, ontology semantics, branches, commits, dependencies or staging were changed. No nested agents or external AI tools were used. Supervisor confirmed a broad outbound-network blocker and instructed this lane to stop retries and deliver a partial report; no product decisions were requested.

## 1. Method and scope: Stanford seven steps

### Step 1 — Domain, boundaries and competency questions

Purpose: compare actual Star Conflict mode/deployment rules with this project's local concepts, without importing gameplay or silently expanding scope. Intended readers are the subsequent research-note writer and ontology owner, not a gameplay implementation agent.

Exclusive scope: PvP objectives/teams/respawn/ship choice; PvE missions/special operations/raids/waves and entry restrictions; Open Space sectors/stations/gates/docking/death/groups; ownership versus selected combat ships and rank/progression limits. Combat balance, ship-role accuracy and equipment mechanics are other research topics.

| Competency question | Locally answerable result | Actual-game answer |
|---|---|---|
| Which named modes exist, and which names are generic local labels? | Four local records below; no official-name provenance. | Blocked; names cannot be certified as official. |
| What teams, objectives and victory conditions distinguish PvP modes? | Not represented in the mode schema. | Blocked; no player-count/objective claims retained. |
| Does destruction permit another life, and can the pilot change ships? | Mode boolean plus conditional hangar selection under C12. | Blocked; no universal respawn rule established. |
| How do missions, special operations, raids and waves differ? | Two generic PvE instances; generator/script work deferred. | Blocked; no equivalence between local labels and actual categories established. |
| Which rank/difficulty/roster restrictions determine entry? | Progression/ranking explicitly out of scope; no eligibility fields. | Blocked; no numerical restrictions asserted. |
| Is a hangar the owned fleet, station UI or deployed-ship subset? | Local Hangar is a four-position subset of owned PlayerShips. | Blocked; actual terminology and slot-unlock rules unknown. |
| Which ship launches into Open Space, and where? | C12 says selected ship from current space station. | Blocked; actual launch, docking and ship-switch rules unknown. |
| What happens to identity, position and selection on death or sector travel? | Ship-state is per match; no lifecycle/session/travel structure. | Blocked; respawn destinations and persistence unknown. |
| What are group sizes and group-to-team/session relations? | No group/team/member structures. | Blocked; do not invent cardinalities. |
| Which rules are dated, seasonal or removed? | Local records have no provenance/availability fields. | Blocked; no historical chronology established. |

Explicit local exclusions remain authoritative: authentication, matchmaking, history, economy beyond price, progression/ship-tree ranking, station interiors, AI and map/level content (`ontology/domain.md:18–23`). Missing game facts in these areas would be scoped exclusions or possible future requirements, **not contradictions**.

### Step 2 — Reuse existing definitions and evidence

Read directly: `ontology/domain.md` (all), `ontology.md` (all), `ontology/model.rs` (all), `ontology/instances/game_modes.json` (all), `ontology/instances/ship_models.json` (all), `client/src/main.rs` (all), `sim/src/lib.rs` (all), and `README.md` (all). The supplied inventory was used for navigation, not as a substitute for reading relevant code. The existing Rust/serde model and JSON catalog are sufficient for this audit; no packages or new ontologies are proposed.

External reuse was attempted through three initial angles: official homepage/navigation for authoritative announcements; official-hosted wiki navigation for detailed mode/deployment rules; and Steam's product page for a broad publisher-description cross-check. One bounded gap-closing diagnostic tried a public search HTML endpoint and a candidate wiki mode index over IPv4. Neither returned content. There was no unbounded crawl and no repeated search fallback after the blocker was confirmed.

### Step 3 — Terms, without presumed official equivalence

**Local nouns:** user (external ID), player-ship, ship-model, loadout, hangar, hangar slot, game-mode, category, ship-state, game instance (narrative), current space station (C12 only), selected ship (relation only).

**Local properties/verbs:** owns permanently, holds, selects, deploys, respawns, picks another ship, is destroyed; `owner_id`, `ship_model_id`, `respawn_allowed`, `armor_hp`.

**Requested research vocabulary, not verified game taxonomy:** PvP team/objective, mission, special operation, raid, wave, difficulty, ship rank, Open Space, sector, gate, docking station, group/party, combat slot, slot unlock, seasonal availability. In particular, `open-world` is the repository's category; its exact mapping to the game's possible UI name “Open Space” is **unverified** here.

### Step 4 — Classes and hierarchy

The local catalog has one `GameMode` struct, classified by disjoint `Pvp | Pve | OpenWorld` enum values (`ontology/model.rs:69–71,177–183`). There are no PvP/PvE subtype structures. A PlayerShip **has** a ShipModel and a Loadout; it is not a subclass of its owner. Hangar **holds** PlayerShip IDs; it is not the complete owned fleet. Ship-state describes transient state and is not another purchased ship.

A potential mode definition → running session → participating pilot → deployed ship chain is useful terminology for future verification, but **is not a sourced assertion about Star Conflict or a proposal to add classes now**. Likewise, do not equate a queue/category, scenario, map, difficulty selection and running instance without evidence.

### Steps 5–6 — Properties, relationships and constraints

| Concept/relation | Exact local representation/cardinality | Important boundary |
|---|---|---|
| Mode definition | `id`, `name`, `category`, `respawn_allowed`; `ontology/domain.md:155–160`, `ontology/model.rs:178–183` | No teams, objectives, victory conditions, duration, availability or difficulty. |
| PlayerShip → user | Exactly one `owner_id`; permanent ownership; `domain.md:162–170,200`, `model.rs:196–201` | User is external. No complete user/fleet container or user-existence validator. |
| PlayerShip → ShipModel | Exactly one model ID; `domain.md:169,201` | Model identity is not runtime deployment identity. |
| Hangar → user | One owner ID in each Hangar; `domain.md:180–184` | No global constraint establishes exactly one Hangar per user. |
| Hangar → ship positions | Exactly four optional positions, hence 0–4 occupied; no repeated held ID; all resolve and belong to owner; `domain.md:208,248`, `model.rs:203–227` | This is a bound on selected positions, not total owned ships. No unlock/progression condition. |
| User → selected ship | 0–1 PlayerShip, Open World only; `domain.md:209` | Declarative relation only; no selected-ship Rust field or validation of ownership/slot membership. |
| PvP/PvE deployment | Select from Hangar; another Hangar ship may be selected for a permitted next life; `domain.md:249` | No per-session participant roster or eliminated-ship state. |
| Open-world deployment | Selected ship from current space station; `domain.md:249` | No typed station/current-location relation, sectors, gates or docking lifecycle. |
| Ship-state | Position, velocity, rotation, shield/armor/energy/heat, cloak and cooldowns; per match, not persisted; `domain.md:186–189` | No runtime state identity, pilot/ship/session foreign key, life counter or lifecycle protocol yet. |
| Destruction | Armor reaches zero; `domain.md:250` | Does not specify respawn delay, cost, place, replacement ship, recovery or persistence. |
| Group/team/session membership | Not defined | No supported local or external cardinality. |

Intrinsic mode properties (category, local respawn flag) are separated from relationship properties (owner, selected ship, held ships). A future rule depending on a session phase, selected roster or pilot progression would not automatically be an intrinsic property of the catalog mode, but its actual need is still unverified.

Facet alignment checked against the existing stack, not the skill's illustrative TypeScript/Zod/SQL stack: Rust enums constrain categories; `[Option<Id>; 4]` fixes storage width; `Hangar::validate` checks duplicates, resolution and owner match; JSON persists the four mode definitions. C12/C13 are expressly server obligations (`domain.md:249–250`), not implemented Rust runtime checks. Catalog validation checks unique mode IDs (`model.rs:282`), not gameplay semantics. No database/storage layer beyond JSON was inferred.

### Step 7 — Instances and examples

**Verified local catalog, not verified official Star Conflict modes:**

| Local exact ID / name | Category | Respawn flag | Anchor |
|---|---|---:|---|
| `team-deathmatch` / `Team Deathmatch` | `pvp` | true | `ontology/instances/game_modes.json:2` |
| `waves-survival` / `Waves Survival` | `pve` | true | `ontology/instances/game_modes.json:3` |
| `operation-scenario` / `Operation Scenario` | `pve` | false | `ontology/instances/game_modes.json:4` |
| `open-world` / `Open World` | `open-world` | true | `ontology/instances/game_modes.json:5` |

Three representative **local** examples establish what a later writer must compare, without inventing real-game examples:

1. **Local Team Deathmatch:** `respawn_allowed=true` plus C12 describes initial selection from the Hangar and a possible next life using another Hangar ship. It does not establish team size, kill target, map or actual game terminology.
2. **Local Operation Scenario:** `category=pve`, `respawn_allowed=false`. Its label is not evidence that a Star Conflict special operation forbids revival or replacement. No real mission name or rank bracket was verified.
3. **Local Open World:** `respawn_allowed=true`, selected-ship cardinality 0–1, station launch under C12. Nothing specifies the death destination or whether that selection must be one of the four Hangar entries.

A read-only Python assertion confirmed all four `(id, category, respawn_allowed)` tuples and ID uniqueness. This is a local data check, not a game-rule validation. **The requested three representative externally grounded real-game examples could not be supplied; Step 7's external realism check remains blocked.** No remembered mode/mission names are substituted for retrieved evidence.

## 2. Observed runtime versus declared deployment model

The present client loads the catalog, selects a model from a CLI argument (default `hydra`), constructs a stock PlayerShip with owner `local` and ID `local-ship`, and computes flight parameters (`client/src/main.rs:12–20`). It creates one `sim::Body` (`client/src/main.rs:34`), then runs movement and collision (`client/src/main.rs:54–64`). `Body` only contains position, rotation, velocity and radius (`sim/src/lib.rs:23–30`). There is no mode selection, team, group, Hangar selection, station launch, death or respawn implementation in these consumers.

That is a **declared-runtime coverage gap, not an implementation bug established by this audit**: the README describes a future authoritative server (`README.md:10–12`), and canonical runtime rules already defer to that server. The flight demo's direct catalog selection does not prove that the intended game rules allow all catalog ships to enter every session.

## 3. Evidence register and retrieval failures

### External useful sources

**None successfully retrieved.** Consequently there are no external excerpts, supported external claims, visible article headings, revision dates or patch dates to quote. Search snippets were not used; none were received. Publisher/type classifications below identify intended endpoints, not authenticated fetched content. A timeout does not establish that a site is globally down, removed or obsolete.

All attempts were on **2026-09-16**. Exact URLs and outcomes:

| ID / intended title | Exact URL | Intended publisher/type | Method / outcome | Article/update date, heading, excerpt and supported claim |
|---|---|---|---|---|
| F1 — Star Conflict official homepage (actual title unobserved) | https://star-conflict.com/en/ | Official game site; homepage, not an announcement itself | GET, `curl -L --connect-timeout 10 --max-time 45`; curl 28, connection timeout after 10002 ms | Unknown / none / none / **none** |
| F2 — Main Page (actual title unobserved) | https://wiki.star-conflict.com/index.php?title=Main_Page | Official-hosted wiki, to be treated as community-maintained secondary documentation | GET, same bounds; curl 28, connection timeout after 10002 ms | Unknown / none / none / **none** |
| F3 — Star Conflict Steam product listing (actual title unobserved) | https://store.steampowered.com/app/212070/Star_Conflict/ | Steam-hosted product page; publisher-description section would be distinguished from community reviews | GET, same bounds; curl 28, connection timeout after 10003 ms | Unknown / none / none / **none** |
| F4 — Google search for Star Conflict game modes | https://www.google.com/search?q=Star+Conflict+game+modes | Search-engine discovery only | HEAD diagnostic, `curl -4 -I -L --connect-timeout 8 --max-time 15`; curl 28 after 8000 ms | Unknown / none / none / **none** |
| F5 — Game_modes, candidate wiki title | https://wiki.star-conflict.com/index.php?title=Game_modes | Candidate official-hosted wiki index; **page existence unverified** | HEAD diagnostic, `curl -4 -I --connect-timeout 8 --max-time 15`; curl 28 after 8002 ms | Unknown / none / none / **none** |

`getent ahostsv4 wiki.star-conflict.com` resolved `23.109.154.234`. No proxy environment variable names were present. Explicit IPv4 did not solve connectivity. No alternate tooling, unauthorized proxy, external AI service, package installation or execution-mode workaround was attempted.

`markitdown` is installed and was invoked for the three planned HTML captures. All three conversions failed with `FileNotFoundError` because curl had not produced an HTML input file. Empty conversion-output placeholders are **not** evidence. No HTML table was available to parse; no standard-library HTML extraction fallback was used. Further index/API requests were stopped after supervisor confirmation of the common network blocker, rather than retrying inaccessible hosts.

### Freshness and conflict handling

- Retrieval-attempt date is not article date and cannot establish current game availability.
- No wiki revision or official patch date was observed. No wiki claim is called current.
- No external conflicting-source pair can be evaluated; “no conflict found” would misleadingly imply a completed search.
- Current/removed/seasonal status for every potential real mode remains unknown. The four local records contain no dates/status fields and cannot answer that question.
- URLs F1–F3 are **UNRETRIEVED supplied seeds**, F5 is an **UNRETRIEVED candidate title**, and F4 is a failed discovery probe. None may be cited later as support for gameplay facts from this report.

## 4. Candidate reconciliation ledger — local facts, not confirmed external drift

Confidence is separated deliberately: **H-local** means the local text/code was inspected; **U-external** means no actual-game comparison is established. Impact is conditional on desired fidelity. Every external-claim cell is explicitly empty of asserted game rules.

| ID | Exact local claim and anchor | Sourced external comparison | Classification / confidence | Potential impact and reconciliation question/options |
|---|---|---|---|---|
| MOD-01 | Names are `Team Deathmatch`, `Waves Survival`, `Operation Scenario`, `Open World`; `instances/game_modes.json:2–5`; narrative `ontology.md:1–4`. | **Unavailable.** No official naming correspondence or availability verified. | **Uncertainty**, H-local/U-external; not a naming contradiction. | Medium: readers may mistake examples for an official mode inventory. Are these generic local scenarios or intended mappings to dated actual modes? Options: preserve independent names and document that; or later attach verified exact UI names and provenance without silently renaming IDs. |
| MOD-02 | `game-mode.respawn_allowed` is a bool; `domain.md:160`; `model.rs:182`. | **Unavailable.** No evidence for or against conditional respawn, revival, finite lives or phase rules. | **Coverage gap / uncertainty**, H-local/U-external. | High if detailed mode fidelity is wanted. Does the boolean mean any allowed new life, unconditional self-respawn, or default eligibility? Preserve the simple local contract unless verified requirements exceed it. |
| MOD-03 | C12: “if `respawn_allowed` the next life may pick another hangar ship”; `domain.md:249`; narrative says respawn “may allow” another Hangar ship, `ontology.md:64`. | **Unavailable.** No actual per-mode reselection/elimination rule established. | **Uncertainty**, H-local/U-external; no internal contradiction proved. | High: permission to respawn and permission to switch ships are distinct questions. Does canonical “may pick” always authorize switching in each respawn-enabled local mode, or leave it mode-dependent? Options: clarify existing wording; later separate policies only if required. |
| MOD-04 | PvP definition has category and boolean but no objective/team properties; `domain.md:155–160`; `model.rs:178–183`. | **Unavailable.** No actual PvP objective or team count verified. | **Coverage gap**, H-local/U-external. | Medium/high for a later runnable mode. Which objectives/teams are in scope beyond the generic label? Absent objective data is not proof that existing data is false. |
| MOD-05 | PvE examples are `waves-survival=true` and `operation-scenario=false`; `instances/game_modes.json:3–4`; wave generator and operation scripts deferred, `domain.md:22–23,256–260`. | **Unavailable.** No mapping to actual missions, raids or special operations. | **Deliberate deferral** plus **uncertainty**, H-local/U-external. | Medium: avoid treating “operation-scenario” as a factual claim about all actual special operations. Keep generic scenarios or later document a specifically sourced mode mapping; generator deferral is explicit. |
| MOD-06 | Hangar is exactly four optional PlayerShip positions; C11 requires uniqueness and owner match; `domain.md:184,248`; `model.rs:203–227`. | **Unavailable.** No actual combat-slot cap or unlock schedule verified. | **Uncertainty** about fidelity; explicit local constraint, H-local/U-external. | Medium/high: distinguish fixed representational capacity from entitlement to occupy every position. Do all local players immediately have all four? Progression is out of scope, so do not add unlocks merely because they might exist elsewhere. |
| MOD-07 | “up to 4 Ships in his Hangar, leaving the other Ships unused”; `ontology.md:61`; ownership and Hangar holding separate, `domain.md:200,208`. | **Unavailable.** Actual hangar/roster/deployment UI vocabulary unknown. | **Clarity uncertainty**, H-local/U-external; local ownership/subset separation is coherent. | Medium: does “Hangar” intentionally mean deployed combat selection rather than total fleet or station screen? Keep it as local vocabulary or later add a glossary alias; never infer a four-owned-ship limit. |
| MOD-08 | Progression/ship-tree ranking is out of scope; `domain.md:18–23`; Q8 points only to Hangar/C11, `domain.md:35`. | **Unavailable.** No rank or difficulty entry restriction verified. | **Deliberate simplification / explicit exclusion**, H-local/U-external. | Medium only if Q8 is later read as reproducing all actual eligibility rules. Options: qualify Q8 as local ownership/selection eligibility; reconsider scope separately if fidelity is chosen. Missing progression is not a logical defect. |
| MOD-09 | Open-world C12: deploy selected ship from current space station; `domain.md:249`; selection is 0–1 at `domain.md:209`. | **Unavailable.** No actual launch/docking/selected-ship relationship verified. | **Coverage gap / uncertainty**, H-local/U-external. | High before station lifecycle implementation: must selected ship be owned, occupy a Hangar position, or merely exist? Where is current station recorded? Define only when runtime ownership/location is authorized. No claim that actual-game selection matches or differs. |
| MOD-10 | Open-world has respawn true, `instances/game_modes.json:5`; ship destruction at zero armor, `domain.md:250`; state per match and nonpersisted, `domain.md:186–189`. | **Unavailable.** No actual death recovery, respawn destination, sector persistence, gates or transfer rules verified. | **Coverage gap**, H-local/U-external. | High for runtime lifecycle; low for present flight demo. What is a “match” in a persistent/open session? Options: document the intended local session boundary or defer, without importing a presumed real-world topology. |
| MOD-11 | Narrative: “User Participates into a Game instance,” `ontology.md:63`; no corresponding typed participant/session/team/group data in `model.rs:177–228`. | **Unavailable.** No actual group sizes, team placement, session admission or party persistence verified. | **Coverage gap / declared runtime deferral**, H-local/U-external. | Medium/high before netcode. Which identity owns a running ship state: PlayerShip, deployment/life or participant? Multiplayer scope alone does not justify a particular group/team cardinality. |
| MOD-12 | Current client directly constructs one PlayerShip from a CLI model, `client/src/main.rs:12–20`, and one Body, `:34`; C12 assigns deployment to future server, `domain.md:249`. | **Unavailable** and not needed for this implementation observation. | **Coverage gap**, H-local; **not a live-game contradiction**. | Low for prototype, high if demo is mistaken for implemented mode semantics. Describe it as flight/collision only; do not “fix” deployment during this research task. |

### Classification summary

- **Verified contradiction with actual Star Conflict:** none established; external evidence unavailable.
- **Deliberate simplification/deferral:** progression/ranking, map/AI content and later wave/operation generators are explicit scope decisions. The four-mode catalog is a local representation, not proven exhaustive official coverage.
- **Coverage gaps:** session/team/objective/life state and station/selection lifecycle have no typed implementation. Whether to fill them is a product/runtime-scope decision.
- **Uncertainty:** official names, all actual numeric mode restrictions, conditional respawn/selection, terminology and freshness.
- **Historical change:** none about the actual game verified. Do not label a mode removed or seasonal based on omission locally.
- **Internal logical issue:** no new contradiction in the topic's existing mode/Hangar invariants was proven. Missing constraints and undefined runtime relations are not promoted to logical inconsistency. The broader inventory's unrelated validation issues are outside this lane.

## 5. Agreements and non-agreements

**Verified internal agreements:**

1. Narrative `ontology.md:61` says up to four selected ships while others remain unused; canonical Hangar has four optional positions (`domain.md:184`), and Rust uses the same capacity (`model.rs:203,210`). This does not restrict total owned fleet size.
2. Narrative deployment (`ontology.md:64–65`) and canonical C12 (`domain.md:249`) agree at the broad distinction: Hangar-based PvP/PvE choice versus selected-ship station launch for Open World. The conditional meaning of switching remains worth clarifying.
3. Rust and all four mode JSON records agree on the three category values and boolean respawn representation. The read-only assertion passed.
4. Canonical permanent owned-ship identity and transient state are distinct (`domain.md:162–170,186–189`); there is no rule that destruction deletes ownership.

**Externally agreed facts:** none can responsibly be claimed from this run. Even plausible high-level similarities are not counted without successful retrieval.

## 6. Gruber criteria

| Criterion | Topic-specific assessment |
|---|---|
| **Clarity** | Local enum/category and four-position Hangar are explicit. “Hangar,” “respawn,” “match,” selected ship and station remain narrower or less specified than their possible game/UI meanings. Local names have no official provenance. |
| **Coherence** | Existing records conform to the declared categories/booleans; capacity, ownership and uniqueness rules align across prose and Rust. No actual-game coherence verdict possible. C12 is a declared future runtime contract, not an implemented selection engine. |
| **Extendibility** | New mode instances fit the existing catalog, but objective/eligibility/lifecycle policies do not have fields. This is a known coverage boundary, not permission to expand now. |
| **Minimal encoding bias** | Readable IDs and enum values are clear; boolean respawn and fixed array width encode strong representational choices. Avoid deriving an actual-game rule from the convenience of these encodings. |
| **Minimal ontological commitment** | Respect explicit exclusion of progression/map/AI content. Do not force actual-game raids/groups/sectors into classes without sourced competency questions and authorization. Distinguish a mode definition from a session conceptually before any implementation. |

## 7. Unanswered questions and bounded next research pass

These are report questions, not requests for an immediate user answer:

1. Is fidelity intended to a particular Star Conflict date/version, or are the four records independent game scenarios?
2. Is the catalog illustrative or exhaustive within the chosen scope? Should local labels carry a glossary/provenance note rather than official aliases?
3. What does `respawn_allowed` guarantee, and is ship reselection an independent policy?
4. Does four-slot Hangar mean always-unlocked combat selection? Is selected Open World ship required to belong to that set?
5. Which sessions exist conceptually: match, PvE scenario run and Open World presence? Which runtime identity survives death, docking or sector transfer?
6. Should Q8 remain limited to ownership/selection eligibility, given the explicit progression exclusion?
7. Are groups/teams part of eventual runtime semantics or intentionally outside this modeling pass?

**After authorized connectivity returns**, use the supplied wiki Main Page and official homepage as actual navigation sources rather than assuming candidate page titles exist. Keep the next pass bounded:

- Obtain a mode index plus a few contrasting original mode-rule pages: objective-based PvP, a mode with distinctive death/selection restrictions, a named PvE mission and a special operation/raid if the sources distinguish them.
- Obtain Open Space entry/death/docking/group documentation and a hangar/combat-slot/progression page. Follow official dated update links that materially alter those rules; one targeted current/removed/seasonal check should close freshness gaps.
- Reuse Steam only for the publisher overview, not detailed constraints or community-review claims. Treat all wiki prose as community-maintained documentation unless authorship is independently established.
- For each useful page retain exact title, URL, publisher/type, retrieval date, visible revision/article/patch date or unknown, heading, short excerpt/paraphrase and the exact claim. Fetch the original behind a search hit. Preserve conflicting dated rules rather than silently choosing one.
- Only then populate actual numeric cardinalities/rank brackets and three genuine real-game examples, and promote a candidate to “verified contradiction” if both a precise local assertion and suitably dated external rule directly conflict.

No guessed mission names, team counts, slot-unlock ranks, gate destinations or respawn rules are supplied in this partial report.

## 8. Validation and change-control evidence

Commands/actions performed:

- `pwd`; initial `test -e` checks for `AGENTS.md` and `docs/HANDOFF.md`; `git status --short`; `git rev-parse HEAD`; `command -v markitdown` — passed. Baseline matched; files absent; working tree clean; converter available.
- Three bounded curl GET commands and corresponding `markitdown` conversions — failed as detailed in F1–F3; no page text obtained.
- Proxy-variable-name inspection and `getent ahostsv4 wiki.star-conflict.com` — completed; no proxy variables, DNS resolution present.
- Two bounded IPv4 curl HEAD diagnostics — failed as detailed in F4–F5.
- Read-only Python `json.loads`/assert check of four mode records — passed: `Read-only local check passed: all four mode IDs, categories and respawn flags match audited claims.` Printed source line anchors were checked against this report.
- `git diff --stat`; `git diff --cached --name-only`; `git status --porcelain=v1`; `git branch --show-current` — clean unstaged/staged outputs; branch `master`.

No project tests were added/updated. No Cargo/build tests were run: this was a research-only audit with no gameplay/source changes. The temporary assertion checked local evidence, not application behavior. Only this managed report and failed-retrieval scratch placeholders were written outside the repository. Independent reviewer must treat external research acceptance as **not satisfied**, not infer success from completion of the local audit.

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "not-satisfied",
      "evidence": "Read-only local audit and scoped partial artifact completed without widening scope, but requested actual-game research and three sourced real examples were blocked by outbound connection timeouts."
    },
    {
      "id": "criterion-2",
      "status": "satisfied",
      "evidence": "Report retains exact local anchors, attempted URLs and failure details, command outcomes, uncertainty classifications and review limitations; no external claims are falsely certified."
    }
  ],
  "changedFiles": [
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/research/modes.md"
  ],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {"command": "git rev-parse HEAD; git branch --show-current", "result": "passed", "summary": "master at requested baseline 4a47fa3015f2d31bbe1c5a3159a3d0980c273912"},
    {"command": "curl -L --connect-timeout 10 --max-time 45 for official homepage, wiki Main_Page and Steam app 212070", "result": "failed", "summary": "All three connection attempts timed out; exact URLs and timings in source register."},
    {"command": "markitdown on three planned HTML captures", "result": "failed", "summary": "No HTML input files existed after curl failures; no converted source content."},
    {"command": "getent ahostsv4 wiki.star-conflict.com; inspect proxy-variable names", "result": "passed", "summary": "IPv4 resolution present; no proxy environment variable names."},
    {"command": "curl -4 -I --connect-timeout 8 --max-time 15 for search and candidate wiki mode index", "result": "failed", "summary": "Both connection attempts timed out; no further retries after supervisor direction."},
    {"command": "python3 read-only JSON assertions for four game mode tuples and ID uniqueness", "result": "passed", "summary": "All reported local mode IDs, categories and respawn flags confirmed."},
    {"command": "git diff --stat; git diff --cached --name-only; git status --porcelain=v1", "result": "passed", "summary": "No repository changes or staged files."},
    {"command": "Cargo/build tests", "result": "not-run", "summary": "Research-only task; no source changes, no packages installed."}
  ],
  "validationOutput": [
    "Local mode tuple assertions passed.",
    "Five external HTTP attempts failed; zero useful external sources retrieved.",
    "Repository clean; no staged files."
  ],
  "residualRisks": [
    "Actual Star Conflict names, rules, cardinalities, freshness and historical availability remain unverified.",
    "Three representative externally sourced game examples are missing due to infrastructure blocker.",
    "Later research must not cite failed URLs as evidence for gameplay claims."
  ],
  "noStagedFiles": true,
  "diffSummary": "Managed partial research report only; no repository/source/ontology edits.",
  "reviewFindings": [
    "blocker: outbound network failure prevents acceptance as completed actual-game research",
    "no verified external gameplay contradiction; local coverage and uncertainty ledger supplied"
  ],
  "manualNotes": "Supervisor confirmed the shared network blocker and requested a partial report without further retries. Independent review remains required."
}
```
