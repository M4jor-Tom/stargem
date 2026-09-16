# Handoff: research-history — 23c3abe8

Checkpoint: 2026-09-16, user-requested pause. Parent-curated from runtime records and actual files; not a new completed child report.

- **State:** COMPLETED / SUPERSEDED: network-blocked attempt
- **Role:** `delegate`
- **Child run:** `23c3abe8-5161-47a2-97c0-bd3231859dc7`
- **Workflow / key:** `08aa52d7-d2a2-4091-b856-28049dd1b39f` / `research-history`
- **Mission:** `28a33a06-b296-4a78-8ae0-079881816489`
- **Repository:** `/home/theta/repos/stargem.nix`, branch `master`
- **Source baseline:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`
- **Session (machine-local, optional for recovery):** `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/2026-09-16T18-47-01-030Z_01a0ab8b-30a5-761c-90d3-cc64125796e1/78324db4-2478-4218-b3f9-8f3850b14107/run-0/session.jsonl`
- **Runtime status (machine-local):** `/tmp/pi-subagents-uid-1000/async-subagent-runs/23c3abe8-5161-47a2-97c0-bd3231859dc7/status.json`

## Authority and boundaries

Read root `AGENTS.md` and `docs/HANDOFF.md` before work. `ontology/` remains canonical and unchanged. This is documentation/research, not authorization to change gameplay. No commit/push/merge/install is authorized. One writer per checkout. Preserve untracked audit files: they are the current work, not disposable clutter. Other agents' full research is intentionally file-backed to protect parent context.

## Resumption (for Agent)

No remaining work for this attempt. Use [research-history-38963aa2.md](research-history-38963aa2.md) instead. The user fixed the firewall; do not propagate this attempt's access-blocked conclusion as current.

## Native continuation or fallback

Recheck live availability before any resume. Completed work normally requires no resume.

Not rechecked in the latest retained-child list; no resume is needed. Use the preserved report below. If genuinely necessary, inspect status/children.list first; otherwise start a same-role focused continuation from this handoff.

## Verification and acceptance

The original report below states exactly what this child checked and what remains uncertain. Completion of a research child does not mean that document synthesis, independent evidence review, ontology review or user reconciliation has completed. Follow [the main checkpoint](README.md) for dependency order and pending reviews.

## Preserved original output

Original managed artifact: `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/research/history.md`

SHA-256 of original artifact: `70ec0c66e40c823b6464406d5c91d9a8f648f97328ed2b7a47d39d22c828d730`

The complete output is embedded below so recovery does not depend on session/temp-file retention. Original relative code paths refer to the repository root unless the report says otherwise. Historical access failures stay historical.

---

# Star Conflict identity, chronology, setting and terminology audit — partial, externally blocked

**Audit date:** 2026-09-16. **Repository:** `/home/theta/repos/stargem.nix`, branch `master`, baseline/observed HEAD `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`.

**Disposition:** Public-source research could not be completed: all four initial navigation/search requests failed before receiving HTTP responses. No official article, wiki page, Steam listing, or search snippet was retrieved. Consequently this report confirms **no live-game contradiction, current release number, official historical change, developer/publisher attribution, platform list, setting/faction/alien fact, or Star Conflict: Heroes product distinction**. It supplies reproducible access evidence, local comparisons, scope boundaries, and reconciliation questions without substituting remembered game facts.

This is documentation/research only. No repository file, ontology meaning, branch, index entry, package, or gameplay code was changed. Root `AGENTS.md` and `docs/HANDOFF.md` were checked first and remain absent. The requested ontology skill and inventory were read; Stanford's seven steps and Gruber's five criteria are applied below. Detailed combat/economy verification belongs to other audit lanes.

## 1. Evidence boundaries and retrieval record

### Starting angles and result

1. **Official home/news navigation:** seek latest dated release announcement, patch/devblog chronology, and dated historical milestones.
2. **Official-hosted wiki navigation:** seek setting/world terminology, factions/aliens, historical names, and revision dates.
3. **Steam product identity:** seek product title, developer/publisher, release date, and listed platforms; do not derive these from local renderer capabilities.
4. **Heroes disambiguation discovery:** search official-domain mentions of `Heroes` and `mobile`, then follow original announcements. The query is a research hypothesis, not evidence that a product has particular platforms or mechanics.

All failed at TCP connection establishment. DNS resolved; no HTTP(S)/ALL/NO proxy environment variables were found. A single additional wiki diagnostic confirmed a connection timeout to port 443. The supervisor confirmed no approved proxy route or fetched source material was known and instructed this lane to stop host retries and complete a clearly partial report. No alternative execution protocol, external AI tool, or agent was used.

**Successful useful public sources: 0.** The target of roughly 10–20 productive pages and the targeted gap-closing retrieval pass were not possible. No wiki freshness determination can be made. There is no evidence here that any host is generally down, that an article is absent, or that this is a content-level block; these are failures from this execution environment.

### Failed-source registry

These are **UNRETRIEVED discovery destinations**, not citations supporting game facts. Page titles below are intended destinations inferred from the supplied URL/query, not observed HTML titles. All attempts occurred on **2026-09-16**. Article/update/patch dates and relevant page headings are **unknown** in every row; excerpts are **unavailable**.

| ID | Intended destination; exact attempted URL | Intended publisher/type, not observed page attribution | Retrieval evidence | Supported claim |
|---|---|---|---|---|
| U1 | Star Conflict English home; `https://star-conflict.com/en/` | Official game site / navigation seed | HTTP `000`; 0 bytes; `curl: (28) Connection timed out after 8002 milliseconds` | Only that this request failed; no game fact. |
| U2 | Wiki Main Page; `https://wiki.star-conflict.com/index.php?title=Main_Page` | Official-hosted wiki; community-maintained reference material must not be equated with an official announcement | HTTP `000`; 0 bytes; `curl: (28) Connection timed out after 8002 milliseconds` | Only that this request failed; no game fact. |
| U3 | Steam app 212070 listing; `https://store.steampowered.com/app/212070/Star_Conflict/` | Steam storefront / prospective product metadata, not a patch-history authority | HTTP `000`; 0 bytes; `curl: (28) Connection timed out after 8002 milliseconds` | Only that this request failed; no identity/platform fact. |
| U4 | Google official-domain Heroes discovery; `https://www.google.com/search?q=site%3Astar-conflict.com+%22Heroes%22+mobile` | Search engine / discovery only | HTTP `000`; 0 bytes; `curl: (28) Connection timed out after 8001 milliseconds` | Only that this request failed; no snippet obtained. |

U2 diagnostic command: `curl -I -v --connect-timeout 5 --max-time 8 'https://wiki.star-conflict.com/index.php?title=Main_Page'`. DNS resolved `23.109.154.234`; output reached `Trying 23.109.154.234:443...`, then `curl: (28) Connection timed out after 5003 milliseconds`. It received no page content. No further request to that host was made by this lane.

Managed retrieval evidence is retained in `scratch/` beside this report: `home`, `wiki-main`, `steam`, `search-heroes`, each with `.result`, `.error`, and empty `.headers` files. No `.html` or converted `.md` files were produced. `markitdown` was installed and its help successfully checked; the fetch command would convert successful HTML with `timeout 30 markitdown INPUT.html -o OUTPUT.md`, but no HTML arrived. A standard-library HTML parser was unnecessary and not used.

### Public research questions that remain wholly unresolved

| Question | Evidence needed; present status |
|---|---|
| What is the latest publicly documented release as of 2026-09-16? | Latest dated original announcement plus nearby news-index entries; **unknown**, not inferred from today's date. |
| Which version do the represented mechanics describe? | Per-mechanic source/revision anchors and any change announcements; **unknown**. The local Git date is not a game patch date. |
| Who develops/publishes Star Conflict, and what platforms are supported now? | Retrieved product page/store attribution and dated support changes; **unverified**. |
| What is the setting's date, region/world structure, and faction/alien taxonomy? | Original setting material plus dated lore/change sources, keeping source narrative separate from mechanics; **unverified**. No faction or alien names are supplied from memory. |
| How is Star Conflict: Heroes distinct? | Separate original product announcement/listing and platform/gameplay descriptions; **unverified**. Do not merge either product's entities because of a shared title fragment. |
| Which names were retired, renamed, translated differently, or reworked? | Two identifiable historical/current sources, dates and exact spellings; **no public historical rename/rework established**. |

## 2. Local evidence read directly

The inventory was used for navigation, not as a substitute for reading the relevant definitions.

| File | Read coverage and relevance |
|---|---|
| `ontology/domain.md:1–275` | Complete canonical scope, vocabulary, relations, constraints and deferrals. |
| `ontology.md:1–65` | Complete source narrative; never assumed to describe a particular official patch. |
| `ontology/model.rs:1–479` | Complete typed mirror and inline validators/tests; inspected identity/vocabulary/cardinality commitments. No tests executed from this Rust file. |
| `ontology/instances/ship_models.json:1–38` | Complete nine local model records; checked representative local witnesses below. |
| `ontology/instances/special_modules.json:1–11` | Complete nine special records, including display-name versus identifier distinctions. |
| `ontology/instances/game_modes.json:1–6` | Complete four local mode records. |
| `README.md:1–25` | Project identity, canonical authority and local reboot date. |

Local `ontology/` is authoritative by `README.md:3–4` and `ontology/domain.md:3–4`. Its declared domain is **“multiplayer space-ship combat (Star Conflict-like)”** (`domain.md:10–11`), not an explicit promise to reproduce the commercial game's present or historical rules exactly.

There is no source-version/provenance property in the relevant `ShipModel`, `SpecialModule`, or `GameMode` structures (`model.rs:104–119,178–183`), nor a source/date annotation in their inspected JSON records. This does not prove that no author ever used external information; it means the inspected ontology cannot establish an external era or claim provenance.

## 3. Chronology: what can and cannot be dated

### Verified repository history — NOT official Star Conflict chronology

Read-only `git log`, `git show`, and rename inspection established:

| Repository date (commit timezone) | Commit | Verified local event | What this does not establish |
|---|---|---|---|
| 2026-09-06 23:05:41 +02:00 | `26cb6d6599a1f07abf99ff37851b676908fc6f0f` | Commit subject: `feat(ontology): domain model, Rust mirror and instances from onthology.md`. | The commercial game's version or release state at that date. |
| 2026-09-06 23:50:04 +02:00 | `00159bd85832187b53f861b71ebe008e5da690d3` | Commit describes a reboot retaining ontology, a raylib/parry3d client/sim, and removal of prior protos/stale server/client submodules. `README.md:6–12` also labels the stack reboot 2026-09-06. | That removed local systems reflect retired official gameplay, or that the repository is an official game implementation. |
| 2026-09-16 20:46:23 +02:00 | `4a47fa3015f2d31bbe1c5a3159a3d0980c273912` | `onthology.md` renamed to `ontology.md`; Git reports `R100`, zero insertions/deletions. | Any official terminology change. This is a local filename typo correction. |

**Temporal conclusion:** the audit can date the local specification's incorporation and a subsequent filename rename. It cannot attach the local rules to a publicly supported Star Conflict patch. No “current mechanics as of September 2026” assertion is justified by this lane.

### Temporal discipline for a later successful pass

- Record **publication date**, **explicit patch/version**, **wiki revision timestamp**, and **retrieval date** separately; none substitutes automatically for another.
- Treat an announcement as evidence for what it announces at its date, not proof that every item still exists unchanged at audit time.
- Treat a wiki page as a possibly mixed-era synthesis until its relevant claim is corroborated or a revision is pinned. A recent page edit is not proof every paragraph was revalidated.
- Distinguish a dated announcement of an upcoming feature from a dated release confirming it shipped.
- Preserve conflicting sources with their dates and languages. A newer general marketing page need not override an older detailed rule if it does not discuss that rule.
- For translation or renaming, retain original term, language, display label and stable local identifier separately. A spelling resemblance is not proof of historical identity.

These are proposed evidence-handling rules, not schema edits or claims that particular official changes occurred.

## 4. Candidate reconciliation ledger

**Legend:** “verified contradiction” requires retrieved external evidence incompatible with an exact local claim; none is established here. “Historical change” below refers only to explicitly identified local Git history. “Coverage gap” is not a logical defect. “Deliberate simplification” means a documented local scope/design commitment, not a claim that the commercial game behaves differently. Confidence is stated separately for the local finding and external comparison.

| ID / classification | Exact local anchor and claim | External evidence / confidence | Impact and reconciliation question/options |
|---|---|---|---|
| H01 — uncertainty: identity/fidelity/era | `domain.md:10–11`: “Star Conflict-like”; `README.md:1–4`: project `stargem`, ontology is source of truth. | U1/U3 unavailable. **High** confidence in local wording; **unassessed** external fidelity. | **High audit impact:** without a target, genuine differences may be intentional design. Is Stargem an inspired independent game, a bounded adaptation, or a dated exact simulator? Options: preserve local authority and annotate inspiration; identify a target release snapshot; or explicitly maintain claim-by-claim rolling fidelity. No option selected here. |
| H02 — coverage gap / uncertainty: temporal provenance | `domain.md:91–100`, `model.rs:112–119`, `ship_models.json:2–37`: model properties include role, price, stats, slots and stock items, but no external source/era property; inspected special/mode records likewise have none. | No dated official source retrieved. **High** local confidence; external version **unknown**. | **High research impact:** local numerical/content choices cannot be dated from public support. Should later research notes carry source/version annotations without widening the runtime schema? Do not add provenance fields automatically. |
| H03 — historical change + internal documentation issue | `domain.md:3–4`: source narrative `../onthology.md`; current file is `ontology.md`. `git diff --name-status HEAD^ HEAD` reports `R100 onthology.md ontology.md`. | Local Git proof; no external comparison required. **High**. | **Low semantic, moderate traceability impact:** the relative link no longer resolves. A later authorized change could correct only the path. It provides no evidence of semantic drift. |
| H04 — deliberate scope restriction, not contradiction | `domain.md:19–23`: excludes progression, station interiors, AI and map/level content; defers “faction/tier”. `ShipModel` at `model.rs:112–120` contains no faction, origin, manufacturer or era relation. | U1/U2 unavailable. **High** confidence in scope; no claim about official completeness. | **Low within current scope; potentially high for lore/full-catalog goals.** Is faction eventually a political allegiance, ship origin, availability restriction, or another concept? Keep these distinct until sourced and required. Retain the deferral unless competency questions require expansion. |
| H05 — uncertainty: world terminology / coverage boundary | `ontology.md:4,65`: “Multiplayer Open World”, exits current “Space Station”; `domain.md:249`: open-world deployment from current space station. `game_modes.json:5` supplies only `open-world`, its label, category and respawn boolean. | U2 unavailable. **High** local confidence; official names and history **unverified**. | **Moderate identity/terminology impact:** no location, sector, station, affiliation or world-history entities are defined. Are the labels generic local concepts or intended aliases of specific official features? Preserve generic wording unless exact mapping is supported; map/content is explicitly excluded. |
| H06 — uncertainty: product collision, not demonstrated contamination | `domain.md:10`: only “Star Conflict-like”; `README.md:1–4`: Stargem. Inspected canonical/narrative files make no `Star Conflict: Heroes` claim. | U4 produced no snippets; U1/U3 unavailable. **High** confidence that inspected files do not assert equivalence; product distinction externally **unverified**. | **High provenance impact if external research resumes:** establish the intended commercial product before importing names/rules. Options: separate reference namespaces or simply a source-label column in research notes; neither requires a new runtime product ontology now. |
| H07 — uncertainty: label normalization, not verified retirement/rework | `ontology.md:23,31`: “Cover Ops”; `domain.md:63–64` and `model.rs:16`: `covert-ops` / `CovertOps`. `ontology.md:27` and `special_modules.json:4` use “phasic” / `phasic-shield`; `ontology.md:19` and `domain.md:69` use “Thermic” / `thermic`. | No language-specific official terms retrieved. **High** for exact local spellings; external equivalence/translation **unverified**. | **Moderate clarity impact:** record narrative-to-canonical correspondence separately from any future official alias. Is `Cover Ops` merely a narrative typo? Are other labels intended translations or local names? `domain.md:6` says referenced IDs never change, so even a supported display-label correction would not authorize silently renaming IDs. |
| H08 — deliberate conceptual restriction; historical completeness uncertain | `domain.md:60–64`: exactly three size values and nine roles; `domain.md:196,223–232`: one special per role; `model.rs:12,16,24–50`: exhaustive enum/match implementations. | No official dated taxonomy/change source retrieved. **High** local confidence; no verified omission or historical introduction claim. | **High only if exhaustive fidelity is required:** the closed vocabulary needs an explicit coverage/era decision. Preserve the restriction for a bounded inspired game; evaluate extensions only after dated counterexamples and product approval. No combat-rule change proposed by this lane. |

### Findings specifically NOT asserted

- No verified mismatch between any named local ship and its actual-game role, faction, class, manufacturer, or debut date.
- No proof that any local special-module name is obsolete or belongs to another product.
- No official count of factions, alien groups, ship categories, worlds, modes, or supported platforms.
- No claim that missing setting/faction/social/progression areas are defects; many are explicitly excluded/deferred.
- No current Star Conflict developer or publisher is inferred from the name `stargem`.
- `README.md:12` describes the chosen client's rendering technology with “14 platforms”; that is not evidence that Star Conflict, or this client, has fourteen shipped/supported platforms.

## 5. Domain vocabulary, relationships and scope seams

### What is actually represented

| Term / relationship | Local meaning and cardinality | External status |
|---|---|---|
| Stargem | Project/domain name (`README.md:1`, `domain.md:1`). | Not an attribution of commercial-game authorship. |
| Star Conflict-like | Scope qualifier (`domain.md:10`). | Exact degree of inspiration/fidelity unspecified. |
| ship model → role → size | Each model has exactly one role; each role exactly one size (`domain.md:195–198`). Three sizes, nine roles. | Dated external taxonomy unavailable. |
| role ↔ special module | One-to-one (`domain.md:196`), role-derived and not an equipable item (`144–146,254`). | Do not infer historical universality from the local axiom. |
| game mode → category | One `pvp`, `pve`, or `open-world` category (`domain.md:155–160`; `model.rs:71,178–183`). Four local instances. | Local broad groupings, not an established exhaustive official taxonomy. |
| user → selected ship | Zero or one selected player ship for open world (`domain.md:209`). | A local deployment relationship; no evidence about official persistent-world rules. |
| user/ship → current station | Station appears in deployment prose (`domain.md:249`, `ontology.md:65`); there is no station class or typed relation in the inspected model. | A boundary concept, not a fully modeled world location. |
| faction/tier | Explicit v2 candidates (`domain.md:22–23`), no current slots/cardinality. | Political identity, technology origin and gameplay rank must not be conflated before evidence. |
| publisher, developer, product edition, patch, lore era, alien group | No classes/properties in the inspected ontology. | Potential research metadata/domain questions, not implicit requirements for combat runtime. |

### Missing-area survey — questions, not claims about actual-game features

These are **candidate research headings** whose existence/details in the commercial game remain unverified here:

- **Product provenance and temporal scope:** title, developer/publisher, platform availability, release channel, patch/version, document language.
- **Setting and world entities:** history/era, locations, stations, geography, political factions, non-human groups, organizations, and whether lore allegiance differs from playable availability.
- **Persistent-world organization:** locations and transitions versus the existing generic deployment prose; map/level content is excluded locally.
- **Social/group organization:** teams, player organizations, ownership/control relationships, if later competency questions need them. No specific official group system is asserted.
- **Narrative/event chronology:** temporary content versus persistent entities, future announcements versus shipped releases, and official naming changes.
- **Progression and content acquisition:** explicitly excluded/deferred; detailed treatment belongs to the economy/progression lane rather than this report.
- **Taxonomy additions and unique content:** only a dated official example could establish whether the local three-size/nine-role abstraction omits a live-game category; a partial catalog alone is not a contradiction.

## 6. Three representative examples: local witnesses only

**Important:** the task's three representative *real-game* examples could not be verified because no source content was obtained. The following are directly checked local instances to make follow-up falsifiable, not substitutes for real-game evidence. None should be described as an actual Star Conflict fact.

| Local witness | Exact local facts | Relation exercise | Required official comparison |
|---|---|---|---|
| `anaconda` | `ship_models.json:2–5`: display `Anaconda`, role `engineer`; `domain.md:216,224`: engineer → frigate, drones. | A concrete model reaches one size and one role-specific special. | Original ship/feature source identifying exact product, role/class, relevant origin/faction and date; distinguish any name variants. |
| `hydra` | `ship_models.json:14–17`: display `Hydra`, role `tackler`; `domain.md:217,227`: tackler → fighter, cloak. | Second size/role branch; suitable for testing whether a familiar name was deliberately reassigned locally. | Do not presume a naming match establishes same identity. Find dated original evidence before classifying any difference. |
| `wolfhound` | `ship_models.json:30–33`: display `Wolfhound`, role `recon`; `domain.md:218,231`: recon → interceptor, hyper-propulsion. | Third size branch; closes coverage of the local three-size taxonomy. | Verify product and exact ship identity, then source-era taxonomy. No debut date or official role is known from this lane. |

A standard-library assertion check verified these three **local** role mappings, nine total models/nine role values, and four local modes. This is not an external realism test or a Rust runtime test.

### Agreed facts

Externally corroborated local/official agreements: **none can be established with zero retrieved sources**. Internally agreed facts are narrower and useful:

- `README.md:3–4` and `domain.md:3–4` agree on canonical ontology authority.
- Narrative `ontology.md:20–23`, canonical `domain.md:60–64,214–218`, and Rust `model.rs:12,16,30–35` agree on the local three-size/nine-role structure, notwithstanding `Cover Ops` versus `covert-ops` spelling.
- Narrative `ontology.md:1–4`, canonical category enum `domain.md:159`, Rust `model.rs:71`, and mode JSON agree on the broad local PvP/PvE/open-world grouping. This does not prove exact official mode-name or respawn fidelity.
- Local file history confirms a filename rename without content change; it does not date the source narrative's mechanics to that commit.

## 7. Stanford seven-step application

### Step 1 — domain, scope and competency questions

Audit domain: **provenance, product identity, chronology, setting vocabulary and scope boundaries for a Star Conflict-inspired combat ontology**. Intended consumers: later research-note/ledger writer, ontology maintainer and reviewer; no implementation authority is implied.

| Competency question | Answer in this report |
|---|---|
| Which local authority controls semantics? | `ontology/`, explicit in README/domain; source narrative is secondary. |
| Does the local specification promise a current exact simulator? | No explicit such promise found; “Star Conflict-like” leaves fidelity unresolved (H01). |
| What public version/date supports mechanics? | Unknown, access blocked; repository dates cannot answer (H02). |
| Which product/platform/developer/publisher is supported by retrieved metadata? | None verified; U1/U3 failed. |
| What separates Star Conflict from Star Conflict: Heroes? | Public evidence unavailable; keep separate until sourced (H06). |
| Which factions/aliens/world locations are represented or deliberately deferred? | Faction/tier deferred, maps/station interiors excluded, station only in deployment prose (H04–H05). No external taxonomy verified. |
| Which terminology difference is a proven historical rename? | Only local `onthology.md` → `ontology.md`; no external rename established (H03/H07). |
| Can three real official instances validate the target-era mapping? | No; three local witness mappings prepared, external evidence absent. |

Detailed combat balance, equipment eligibility and economy research are out of this lane. Product decisions are recorded rather than asked of the user.

### Step 2 — reuse

Reused the canonical ontology, Rust mirror, three relevant instance catalogs and supplied inventory. No parallel ontology, package, dependency or source schema was created. Original announcements and revision-pinned wiki content would be reused as evidence if accessible; retrieval failure does not justify inventing substitutes.

### Step 3 — enumerate terms

Captured local product name; source-product qualifier; ship model, size, role, special; mode/category; player/selected ship; station; deferred faction/tier; and the proposed evidence vocabulary of product, edition, source language, announcement, patch, revision and audit date. Exact spellings are preserved in H07. No unverified external faction/alien names are introduced.

### Step 4 — classes/hierarchy

`ship-model` is a catalog item; role/size classify it through composition/functional mappings, not through faction or product inheritance. A product title is not a ship role, and a potential political faction is not automatically a ship-size subclass. Existing enums deliberately close the vocabulary. No hierarchy revision is authorized or proposed as an immediate fix.

### Step 5 — properties/relations

Distinguished intrinsic local `id/name/role` fields from proposed external source-product/version annotations and future faction/location relations. No publisher, patch or faction relation is present. Proposed research-note metadata does not need to become a runtime property.

### Step 6 — facets

Checked the explicit three-size/nine-role enumeration, functional model→role→size and bijective role↔special commitments, stable-ID rule, and broad mode-category domain. Public cardinalities remain unverified. This project uses Rust/serde/JSON rather than the skill's illustrative TypeScript/Zod/SQL stack; no SQL alignment claim is made. Full validator execution and detailed combat facets are outside this research lane.

### Step 7 — instances

Read the three relevant instance files completely. Verified three local representative mappings with runnable assertions. Three sourced real-game examples could **not** be validated; this step remains incomplete for external realism, explicitly rather than silently treating local instances as official data.

## 8. Gruber criteria

| Criterion | Assessment in this lane |
|---|---|
| Clarity | Explicit local authority and scope are strengths. Target era/fidelity, narrative label normalization and source-language aliases remain undefined. Distinguish project/product labels from authorship attribution. |
| Coherence | Local size/role examples align across narrative, canonical model and Rust. The stale source path is a verified documentation-reference issue. No externally grounded logical contradiction can be established. |
| Extendibility | Instance catalogs can accept further local examples; closed role/size/special enums constrain new categories. Decide intended breadth before expanding them, rather than treating any future official feature as a mandatory change. |
| Minimal encoding bias | Readable stable IDs help. Display spelling, translation alias and external canonical label need not all be encoded into a replacement ID. Research annotations can stay outside executable structs. |
| Minimal ontological commitment | Deferring faction/tier and excluding world/maps/progression is legitimate for the stated combat domain. Public-game completeness is not a valid inferred requirement. Conversely, exhaustive historical claims need evidence rather than stronger assumptions. |

## 9. Reconciliation questions for the later writer/maintainer

No user response or code change is requested during this audit.

1. **Target:** inspired independent game, subset adaptation, or exact simulator? If exact, which product/edition and explicit date or patch?
2. **Authority:** retain canonical local rules when sources disagree, or require an approved reconciliation decision before changing them? What priority should official release notes, official lore pages, storefront metadata and wiki revisions have for their respective subjects?
3. **Era:** one frozen snapshot or claim-specific temporal windows? How should future announcements, retired content and temporary events be represented in research notes?
4. **Names:** are local familiar ship names intended as exact identities, placeholders, or homages? Should source-language labels and aliases be maintained without changing stable IDs?
5. **Scope:** does the current project need faction/origin/world entities to answer any competency question? If not, preserve the documented exclusions/deferrals.
6. **Product separation:** should sources be explicitly labeled Star Conflict versus Star Conflict: Heroes in the research ledger before accepting evidence? No mechanics should cross this boundary merely through title similarity.
7. **Catalog commitment:** are the nine local ships a representative demo or intended exhaustive catalog for a chosen era? What evidence would justify widening the fixed taxonomy?
8. **Source retention:** is a citation/claim ledger sufficient, rather than introducing provenance properties in gameplay data? Suggested minimum note fields: exact product, URL/title, publisher/type, retrieval date, visible publication/patch/revision date, language, heading/excerpt, supported claim, local anchor, classification, confidence and unresolved conflict.

### Bounded recovery plan, not executed

When an approved functioning route exists: revisit the three supplied seeds once; locate the latest dated original release plus a short news/devblog interval; follow only historically relevant taxonomy/world announcements; obtain separate original Heroes product evidence; inspect setting/faction/wiki pages with revision dates; then close only gaps necessary to classify local candidates. Preserve unsuccessful requests and meaningful dated conflicts. Do not perform an unbounded crawl or backfill remembered facts.

## 10. Validation and change record

**Changed repository files:** none. **Tests added/updated:** none. **Staged files:** none. Only this managed report and the managed fetch-result scratch files were written outside the repository.

Commands executed, grouped for reproducibility:

- `git status --short`, `git rev-parse HEAD`, `git branch --show-current`, `git diff --cached --name-only`: confirmed clean `master`, expected baseline, empty index diff.
- `command -v markitdown`; `markitdown --help`; `date -u +%FT%TZ`: tool available; audit runtime reported `2026-09-16T19:03:27Z`.
- Four parallel requests, each using `curl -L --connect-timeout 8 --max-time 35 --max-filesize 6000000 -A 'Mozilla/5.0 (research audit)' -sS -D "$S/NAME.headers" -o "$S/NAME.html" -w 'NAME\t%{http_code}\t%{url_effective}\t%{size_download}\n' URL`, with stderr/result files and conditional bounded markitdown conversion. All failed; exact URLs and errors are in §1.
- Proxy-variable inspection and `getent ahostsv4 star-conflict.com wiki.star-conflict.com store.steampowered.com www.google.com`: no configured proxy observed; DNS names resolved. A lack of proxy configuration does not establish the correct network remedy.
- One bounded verbose wiki diagnostic, recorded in §1; failed at connection establishment.
- `git log -8 --format=...`, `git show --format=fuller --stat --find-renames HEAD`, `git log --follow ... -- ontology.md`, `git log ... -- ontology/domain.md ontology/model.rs ontology/instances/ship_models.json`, `git show --format=fuller --no-patch` for the two September 6 commits, and `git diff --name-status HEAD^ HEAD -- ontology.md onthology.md`: established only repository chronology.
- `python3` standard-library assertions checked baseline, clean tree, no staged files, missing AGENTS/HANDOFF, broken narrative path/current file presence, nine models/nine role values, three witness mappings, four modes. Output: **`PASS: baseline/clean tree/no staged files; missing AGENTS/HANDOFF; stale narrative link; 9 local models/9 roles, 3 witness mappings, 4 modes`**. This check executed successfully; it was not persisted as a new test file.

A final standard-library check parsed the fenced acceptance report as JSON, verified permitted status/result values and rechecked the clean repository/index; it passed. Local anchors for role/size/special and deployment relations were independently rechecked with `grep` before completion.

**Residual risks:** current official facts and external historical chronology remain wholly unverified; no source-backed contradiction/agreement or real-game instance validation is available. A later writer must not promote the failed-source registry, local sample names, or suggested research headings into sourced Star Conflict claims. Required independent review remains pending; this report is an honest partial deliverable, not completion of external research.

```acceptance-report
{
  "criteriaSatisfied": [
    {
      "id": "criterion-1",
      "status": "not-satisfied",
      "evidence": "Scoped partial research artifact delivered without repository/ontology edits, but requested official chronology, product identity and real-game examples could not be established because every initial public retrieval timed out."
    },
    {
      "id": "criterion-2",
      "status": "satisfied",
      "evidence": "Report retains exact failed URLs/errors, network diagnostic, local file:line comparisons, verified repository chronology, classifications, three explicitly local witnesses, method application and reproducible validation evidence."
    }
  ],
  "changedFiles": [
    "/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/research/history.md",
    "Managed research/scratch fetch-result, stderr and empty-header files only; no repository files changed"
  ],
  "testsAddedOrUpdated": [],
  "commandsRun": [
    {
      "command": "git status --short; git rev-parse HEAD; git branch --show-current; git diff --cached --name-only",
      "result": "passed",
      "summary": "Clean master at the specified baseline; no staged files."
    },
    {
      "command": "command -v markitdown; markitdown --help; date -u +%FT%TZ",
      "result": "passed",
      "summary": "Installed converter available; audit execution date confirmed. No source HTML arrived to convert."
    },
    {
      "command": "Four bounded curl GET requests to official home, official-hosted wiki, Steam and Google discovery; exact URLs/options in report section 1/10",
      "result": "failed",
      "summary": "Every request returned HTTP 000, zero bytes and curl error 28 during connection establishment."
    },
    {
      "command": "getent ahostsv4 for all four initial hosts; proxy environment inspection",
      "result": "passed",
      "summary": "DNS resolved; no proxy variables found."
    },
    {
      "command": "curl -I -v --connect-timeout 5 --max-time 8 'https://wiki.star-conflict.com/index.php?title=Main_Page'",
      "result": "failed",
      "summary": "Single diagnostic timed out connecting to 23.109.154.234:443 after 5003 ms; retries then stopped."
    },
    {
      "command": "Read-only git log/show and git diff --name-status HEAD^ HEAD -- ontology.md onthology.md",
      "result": "passed",
      "summary": "Verified September 6 local ontology/reboot commits and baseline R100 narrative rename, not official game chronology."
    },
    {
      "command": "python3 standard-library assertions for repository baseline, cleanliness, links and local instance mappings",
      "result": "passed",
      "summary": "Confirmed stale narrative link, 9 models/9 roles, 3 local witnesses, 4 modes and empty staged diff."
    }
  ],
  "validationOutput": [
    "PASS: baseline/clean tree/no staged files; missing AGENTS/HANDOFF; stale narrative link; 9 local models/9 roles, 3 witness mappings, 4 modes",
    "Public sources retrieved: 0; no external contradiction, agreement, version or historical change established."
  ],
  "residualRisks": [
    "Official public evidence unavailable; requested current/history/product research is incomplete.",
    "Real-game examples and wiki freshness could not be verified.",
    "Independent reviewer gate remains pending."
  ],
  "noStagedFiles": true,
  "diffSummary": "Managed partial audit report and network-failure evidence only; no source or ontology changes.",
  "reviewFindings": [
    "blocker: all initial public-source connections timed out; do not accept this as completed official chronology research.",
    "ontology/domain.md:4 - stale ../onthology.md reference verified by baseline R100 rename; documented only, not changed."
  ],
  "manualNotes": "Supervisor confirmed no known approved network route or fetched evidence and instructed completion of a partial report without more retries. Product questions are recorded, not escalated to the user."
}
```
