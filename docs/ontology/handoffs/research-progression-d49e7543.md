# Handoff: research-progression — d49e7543

Checkpoint: 2026-09-16, user-requested pause. Parent-curated from runtime records and actual files; not a new completed child report.

- **State:** COMPLETED / SUPERSEDED: network-blocked attempt
- **Role:** `delegate`
- **Child run:** `d49e7543-07c8-4ca2-b0e5-502b13342be7`
- **Workflow / key:** `08aa52d7-d2a2-4091-b856-28049dd1b39f` / `research-progression`
- **Mission:** `28a33a06-b296-4a78-8ae0-079881816489`
- **Repository:** `/home/theta/repos/stargem.nix`, branch `master`
- **Source baseline:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`
- **Session (machine-local, optional for recovery):** `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/2026-09-16T18-47-01-030Z_01a0ab8b-30a5-761c-90d3-cc64125796e1/0677bb66-f481-44a4-b59f-71a7edd1aa54/run-0/session.jsonl`
- **Runtime status (machine-local):** `/tmp/pi-subagents-uid-1000/async-subagent-runs/d49e7543-07c8-4ca2-b0e5-502b13342be7/status.json`

## Authority and boundaries

Read root `AGENTS.md` and `docs/HANDOFF.md` before work. `ontology/` remains canonical and unchanged. This is documentation/research, not authorization to change gameplay. No commit/push/merge/install is authorized. One writer per checkout. Preserve untracked audit files: they are the current work, not disposable clutter. Other agents' full research is intentionally file-backed to protect parent context.

## Resumption (for Agent)

No remaining work for this attempt. Use [research-progression-47f4cf33.md](research-progression-47f4cf33.md) instead. The user fixed the firewall; do not propagate this attempt's access-blocked conclusion as current.

## Native continuation or fallback

Recheck live availability before any resume. Completed work normally requires no resume.

Not rechecked in the latest retained-child list; no resume is needed. Use the preserved report below. If genuinely necessary, inspect status/children.list first; otherwise start a same-role focused continuation from this handoff.

## Verification and acceptance

The original report below states exactly what this child checked and what remains uncertain. Completion of a research child does not mean that document synthesis, independent evidence review, ontology review or user reconciliation has completed. Follow [the main checkpoint](README.md) for dependency order and pending reviews.

## Preserved original output

Original managed artifact: `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/research/progression.md`

SHA-256 of original artifact: `6f4d9890cf5eb2dfc5560575e14e67a22a25077f6876c6bb9ae4a8d0fcda7ede`

The complete output is embedded below so recovery does not depend on session/temp-file retention. Original relative code paths refer to the repository root unless the report says otherwise. Historical access failures stay historical.

---

# Progression, acquisition, economy and social context — blocked/partial audit

**Audit/retrieval-attempt date:** 2026-09-16  
**Repository:** `/home/theta/repos/stargem.nix`  
**Branch / verified baseline:** `master` / `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`  
**Status:** Local analysis complete for this lane; external Star Conflict research blocked by network connectivity. **No external gameplay claim, live-game contradiction, or historical gameplay change has been verified.**

## Executive result

- Canonical `ontology/domain.md` explicitly excludes economy beyond ship price and progression, and defers ship prerequisites and faction/tier. Those omissions are **scope decisions**, not automatically bugs.
- All nine local ship records have a positive integer price and fit the local ownership abstraction. That establishes local consistency of those records, **not** that all actual Star Conflict ships are purchasable for one positive price or are permanently owned.
- The local contract makes stronger commitments than merely omitting economic systems: every modeled ship has `price: u32 > 0`; a purchase creates a stock-equipped player ship; ownership is permanent and single-user. Whether those commitments faithfully describe any particular Star Conflict version remains unanswered.
- Four independent initial navigation/search targets and one supervisor-approved IPv4 probe all failed before HTTP response. Consequently there are **zero useful fetched sources** and no defensible real-game examples in this report. Do not promote research questions below into sourced findings.

## 1. Authority, method and evidence boundary

Root `AGENTS.md` and `docs/HANDOFF.md` were checked first and are absent. Read the requested ontology skill in full at `/home/theta/.pi-game-dev/git/github.com/M4jor-Tom/claude-ontology-skill/skills/ontology/SKILL.md`; applied Stanford's seven steps and Gruber's five criteria below without modifying semantics.

Read directly, not solely via the inventory:

- `ontology/domain.md`, all 275 lines;
- `ontology.md`, all 65 lines, treated as local source narrative rather than live-game authority;
- `ontology/model.rs`, all 479 lines;
- `ontology/instances/ship_models.json`, `passive_modules.json`, `active_modules.json`, in full;
- `client/src/main.rs`, all 95 lines, including the sole production `Catalog::buy` call;
- supplied inventory at `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/69d652a5-b7d3-47be-83bd-4c3f7677ee17/audit/inventory.md`.

Searched price, ownership and `buy` references across local Markdown, Rust and JSON; all `buy` callers are the client and inline model tests. No commits, branches, pushes, package installations, repository/source/ontology edits, nested agents or network-setting changes were performed. Only this report and allowed managed scratch diagnostics were written.

### Classification discipline

| Classification | Result in this lane |
|---|---|
| Verified contradiction with actual Star Conflict | None established; external evidence unavailable. |
| Deliberate simplification | Positive scalar ship prices, stock creation, permanent single-user ownership are explicit local commitments; progression/economic breadth is explicitly excluded. Intent to match actual game is unknown. |
| Coverage gap | Several requested subjects are not modeled; most are expressly outside scope. Purchase charging is assigned to a future server rather than performed by the helper. |
| Uncertainty | Every external gameplay comparison, price provenance, currency unit, acquisition universality and meaning of ownership permanence. |
| Historical change | No externally supported historical change established. Do not assume that differences in future sources necessarily represent change rather than stale documentation or differing contexts. |
| Internal logical issue | None newly demonstrated within this exclusive topic. Missing enforcement assigned to a future server is not presented as an existing live-game contradiction. Other inventory findings are outside this lane. |

## 2. Retrieval record and network limitation

### Four starting angles

1. Official game's announcements/site navigation: `https://star-conflict.com/en/`.
2. Official-hosted wiki index for mechanics, factions, progression, construction and corporate play: `https://wiki.star-conflict.com/index.php?title=Main_Page`.
3. Steam product/storefront for monetization and product identity: `https://store.steampowered.com/app/212070/Star_Conflict/`.
4. Public search discovery combining progression and acquisition terminology: `https://www.google.com/search?q=site%3Astar-conflict.com+ship+selling+experience+clearance+factions`.

Each initial request used `curl -L --connect-timeout 10 --max-time 40 --max-filesize 8000000 -A 'Mozilla/5.0 research-audit' -sS`, saving status/error output to managed scratch. Requests were independent and concurrent. Every connection timed out; no response bodies were available. No proxy variables were present in the checked environment.

The supervisor authorized **one** IPv4-only diagnostic against the wiki, with an 8-second connection timeout and 15-second total limit. It also failed. Supervisor instruction was then to stop external retrieval, retain the limitation and complete a partial report. No repeated host probing, alternate execution mode, proxy change or further API crawling was attempted.

### Failed source ledger

Titles below are **target labels**, not fetched article titles. Intended publisher/type identifies why the target was chosen; the retrieval does not authenticate page contents. For every row: attempt date **2026-09-16**; article/update/patch date **unknown**; relevant heading **unavailable**; faithful excerpt/paraphrase **unavailable**; supported gameplay claim **none**.

| ID | Target label and exact requested URL | Intended publisher/type | Result |
|---|---|---|---|
| F1 | Star Conflict official homepage — `https://star-conflict.com/en/` | Official game site; not a particular announcement | curl exit 28, HTTP `000`, connect timeout after 10003 ms. |
| F2 | Star Conflict Wiki Main Page — `https://wiki.star-conflict.com/index.php?title=Main_Page` | Community-maintained, official-hosted wiki | curl exit 28, HTTP `000`, connect timeout after 10003 ms. |
| F3 | Star Conflict Steam product page — `https://store.steampowered.com/app/212070/Star_Conflict/` | Steam storefront; intended developer/publisher product description and platform commerce information | curl exit 28, HTTP `000`, connect timeout after 10003 ms. |
| F4 | Google discovery query — `https://www.google.com/search?q=site%3Astar-conflict.com+ship+selling+experience+clearance+factions` | Search-engine index; discovery only even if accessible | curl exit 28, HTTP `000`, connect timeout after 10002 ms. No snippets obtained. |
| F5 | IPv4 probe of the same wiki Main Page — `https://wiki.star-conflict.com/index.php?title=Main_Page` | Diagnostic retry of F2, not an additional source | DNS resolved IPv4 `23.109.154.234`; TCP port 443 connection timed out after 8002 ms; curl exit 28, HTTP `000`. |

This is evidence of inability to connect from this environment, **not** proof that the websites are down, have removed particular articles, block this user, or lack relevant documentation. HTTP `000` is curl's absence-of-response indicator, not an HTTP status returned by a site.

`markitdown` is installed at `/nix/store/f9hm8585mmsvl7rr72yfrb0lh5162m5b-python3.14-markitdown-0.1.7/bin/markitdown`; its help command succeeded. Conversion was chained after successful retrieval, but no HTML was retrieved, so no conversion or table-parsing fallback could be performed. No page content was read as instructions.

**Managed diagnostics:** `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/scratch/progression/`, with `main`, `official`, `steam`, `search`, and `ipv4-main` `.status`/`.err` files. These record the failed attempts; they are not source excerpts.

**Freshness limitation:** retrieval date alone would not establish game-version currency even with a successful wiki fetch. There is currently no retrieved revision timestamp, patch number, article date, conflicting source pair or current-game confirmation to assess. The requested productive broad source pass and targeted gameplay gap-closing pass could not be completed.

## 3. Stanford step 1 — scope and competency questions

Canonical purpose: multiplayer space-ship combat inspired by Star Conflict, serving server/client/content authors (`ontology/domain.md:10–16`). In scope are ship models, ownership/loadouts and combat, not an exhaustive simulation of Star Conflict's economy or social systems.

Exact exclusions and deferrals:

> `ontology/domain.md:19–20`: “economy beyond ship price, progression / ship tree ranking” are out of scope.
>
> `ontology/domain.md:22–23`: v2 candidates include “ship tree (prerequisites between ship models), module prices and purchase, faction/tier”.

The following are audit questions, not newly imposed product requirements:

| Question | Local answer | External answer / evidence needed |
|---|---|---|
| 1. What can the user buy, and for what price? | Q1 at `domain.md:29`; nine models, each with one unitless positive integer price. `Catalog::buy` only requires an existing model and supplied IDs. | Unknown: which actual ships support direct purchase, and under what eligibility/currency/version conditions? |
| 2. Can acquisition occur without an ordinary positive-price purchase? | No typed acquisition method, grant, construction recipe, bundle, rental or reward record. | Unknown: obtain original documented examples before alleging any exception. |
| 3. Is an acquired ship owned permanently, exclusively and without sale/expiry? | Canonical wording says owned forever by one user, `domain.md:163,200`. | Unknown: distinguish voluntary disposal, transfer, temporary access and combat destruction; do not infer one from another. |
| 4. Do ranks, tiers, pilot clearance, experience or prerequisites gate availability? | No such properties; progression excluded and ship tree deferred. | Unknown: distinguish account-level, ship-level and equipment-level concepts and dated terminology. |
| 5. Do factions, subfactions and manufacturers denote the same relation? | No such relation; faction/tier deferred. | Unknown: need separately defined affiliation, production and player allegiance, not a flat guessed enum. |
| 6. Do crew or implants affect an individual ship, loadout, slot or account? | Only ship base stats and equipped passive modifiers contribute to local Q7/C5. | Unknown: research effects and applicability without turning excluded progression into a local error. |
| 7. Which prices, resources, recipes, trade restrictions and rewards are relevant to acquisition? | Only `ShipModel.price`; no currency unit or resource inventory. | Unknown: capture exact recipe and payment semantics where documented. |
| 8. How do corporations, dreadnoughts, sector control and social memberships relate? | No classes or relationships representing them. | Unknown: require documented ownership/control/membership cardinalities and time context. |

## 4. Stanford steps 2–6 — local concepts and facets

### Reuse and local authority

Reuse the existing canonical definitions and Rust/JSON representation when deciding whether research should affect the product. Do not import an external ontology package or add economic abstractions solely because they exist in a reference game. No external vocabulary or data schema was fetched in this run. The supplied inventory is a code-navigation aid, not independent gameplay evidence.

### Terms and relationships actually represented

| Concept or relation | Exact local evidence | Cardinality / interpretation |
|---|---|---|
| Ship model | `domain.md:90–100`; `model.rs:110–120` | Catalog definition; one role and one required `u32` price. |
| Price | `domain.md:96`; `model.rs:115,306` | Integer `1..=4,294,967,295` when validated; currency unit unspecified. Type alone permits zero; validator rejects it. |
| Player ship | `domain.md:162–170`; `model.rs:195–201` | One owner ID, one ship-model ID, one loadout per instance. |
| User owns ship | `domain.md:200` | `1 → many, permanent`; user/auth represented only by external ID. No inverse owned-ship collection defined here. |
| Ship has model | `domain.md:201` | Many player ships → one model. No rule prohibits the same user from receiving multiple instances of a model. |
| Purchase | `domain.md:247`; `model.rs:405–419` | Canonical cost = model price; new ship gets stock weapon and missile, zero passive and active modules. Cost enforcement assigned to server. |
| Hangar | `domain.md:180–184,207,248`; `model.rs:204–228` | Four optional deployment references, at most four held ships; references resolve to owned ships and cannot repeat. Not an asserted limit of four total owned ships. |
| Effective stats | `domain.md:242`; `model.rs:390–403` | Base plus additive modifiers from equipped passive modules. No represented pilot/crew/implant progression. This is deliberate local scope, not a verified external mismatch. |
| Narrative ship tree | `ontology.md:59` versus `domain.md:19–23` | Narrative mentions a ships arborescence; canonical layer explicitly postpones prerequisites/ranking. Canonical narrowing wins. |

Nouns absent from the local schema but relevant to the requested research lane: faction, subfaction, manufacturer, rank, tier, pilot clearance, ship experience/synergy, crew, implant, prerequisite, unlock, acquisition offer, currency, resource, recipe, trade, reward, entitlement, corporation, dreadnought, sector ownership/control and membership. These are **research terms**, not established real-game class definitions or requirements.

Do not conflate:

- model intrinsic identity with player/account progression;
- manufacturer with faction membership;
- numeric ship rank with account clearance or historical tier grouping;
- ownership with selection into a deployment slot;
- acquisition eligibility with a price amount;
- a combat-destroyed runtime ship with deletion of the owned ship;
- a sale/transfer of ownership with recurring repair or operating cost;
- a player-owned ship with any possible corporation-owned asset.

The omitted relationships have **unknown external cardinalities**. In particular, no researched basis exists here for asserting one manufacturer per ship, one corporation per player, a particular number of factions, a rank cap, an implant count or a dreadnought ownership limit.

### Classes, properties and validation layers

The existing model correctly treats a player ship as **having a ship model**, not being a subclass of user. Ownership and loadout use ID references. Ship price is intrinsic to the local catalog abstraction; whether a real game's offer price instead depends on purchaser, currency, prerequisites or time cannot be settled without evidence.

Rust + serde JSON + runtime validation are this repository's relevant layers; there is no SQL schema in the inspected model. `Catalog::validate` enforces positive model prices (`model.rs:306`), but `Catalog::load` does not invoke validation (`245–261`) and the client directly calls `buy` (`client/src/main.rs:14–16`). Current records satisfy the facet. This is a boundary worth recording, **not evidence that a bad-priced current instance exists**.

`Catalog::buy` neither reads `price` nor checks funds, eligibility, unique ship IDs, pre-existing ownership or persistence (`model.rs:405–419`). It is currently a stock-loadout constructor, called by a flight demo and inline tests, not a functioning economic transaction service. Canonical C10 assigns charging to the server. Reporting “purchases are free in Star Conflict” or “the ontology permits free priced ships” from this helper would be incorrect.

## 5. Candidate reconciliation ledger

These are **local commitments awaiting comparison**, not verified external drift. “External claim” is deliberately empty of unsupported mechanics. Confidence distinguishes observed local facts from unavailable external corroboration.

| ID / classification | Exact local claim and anchors | External sourced claim | Confidence and impact | Reconciliation question / options |
|---|---|---|---|---|
| P01 — deliberate simplification; external uncertainty | Every ship model requires `price: u32 > 0` (`domain.md:96`; `model.rs:115,306`); users can buy a model for that price (`ontology.md:11`; `domain.md:29,247`). | None retrieved. | High local; no external confidence. Potentially high fidelity impact if actual acquisitions cannot be described by one positive scalar. | Is `price` a local balancing number or a purported real-game acquisition cost? Retain as explicit independent-game design; alternatively, only if fidelity becomes required and sourced exceptions exist, distinguish catalog identity from acquisition offers. |
| P02 — deliberate simplification; external uncertainty | “owned forever by one user” (`domain.md:163`), permanent ownership relation (`200`), narrative “owned forever” (`ontology.md:12`). | None retrieved. No verified selling, rental, expiry, transfer or permanent-loss rule. | High local; no external confidence. Potentially high lifecycle impact. | Does “forever” mean no combat loss, no expiry, or no voluntary disposal/transfer at all? Preserve the strong rule if intended; otherwise resolve wording before adding lifecycle behavior. |
| P03 — excluded coverage / deliberate simplification | “progression / ship tree ranking” out of scope, prerequisites v2 (`domain.md:19–23`); no rank/clearance/experience fields in `ShipModel` or `PlayerShip` (`model.rs:110–120,195–201`). | None retrieved. | High scope confidence. No current defect; medium future fidelity impact. | Keep progression excluded, or later define separate account/ship/item progression questions using dated evidence. Do not infer that a scalar rank covers all terms. |
| P04 — deferred coverage | “faction/tier” v2 (`domain.md:23`); local model has role, not manufacturer or faction (`model.rs:110–120`). | None retrieved. | High local; no external confidence. Low current impact. | Are affiliations needed for display, eligibility, statistics or lore? Keep absent until a use case exists; then distinguish affiliation from production and unlock relations. |
| P05 — excluded coverage / deliberate simplification | Economy beyond ship price excluded; module prices and purchase deferred (`domain.md:19–23`). No currency/resource/recipe/trade/reward fields in catalog/owned objects (`model.rs:110–120,186–201,232–242`). | None retrieved. | High local; no external confidence. Low present impact, potentially high acquisition-fidelity impact. | Retain an abstract price-only economy, or separately scope actual acquisition/monetization flows after evidence is available. Missing systems are not contradictions. |
| P06 — excluded progression coverage | Q7/C5 uses base + passive modifiers (`domain.md:35,242`; `model.rs:390–403`). No crew/implant/experience entities. | None retrieved. | High local; no external confidence. No standalone defect. | Is Q7 intentionally limited to this game's listed stat sources? If so, no expansion. If real-game stat reconstruction is later requested, research other sources and their attachment points first. |
| P07 — coverage gap outside current purpose | Class tree contains catalog/combat/owned-ship entities only (`domain.md:40–57`); ownership object is external user (`168,200`). No corporation, membership, dreadnought or sector-control relation. | None retrieved. | High local absence; no external mechanics confirmed. Low current impact. | Should social/corporate entities ever enter this combat ontology? Keep excluded unless explicit competency questions require them; do not force collective assets into `PlayerShip.owner_id`. |
| P08 — implementation coverage; not an external contradiction | C10 says purchase costs `price`, checked by server (`domain.md:247`); helper named `buy` constructs stock ship without charging (`model.rs:405–419`), client uses it directly (`client/src/main.rs:16`). | External evidence not needed for this local observation. | High. Low impact in current demo; material if helper is mistaken for an authoritative transaction. | Treat helper as construction only and C10 as future-server obligation. No gameplay implementation authorized by this audit. |
| P09 — uncertainty / provenance gap | Nine ship prices are 6000/8000/12000 (`ship_models.json:2–37`) with no currency or source/version field (`model.rs:110–120`). | None retrieved; no comparable real-game prices. | High local; unavailable external comparison. Medium documentation impact. | Are names/prices illustrative balancing data or intended reconstructions? Document that decision before trying to correct prices. No numeric drift claim is justified. |

### Direct answer to the all-ships question

**Local catalog:** yes, all nine records fit positive `u32` prices, and all conform structurally to the permanent single-owner model. No current positive-price facet violation was found.

**Actual Star Conflict:** not established. It would be dishonest to endorse “all ships can be bought for a positive `u32` and owned forever” as a researched game fact. It would equally be dishonest to reject it by inventing a free/gifted/crafted/rented/sold-ship example without a retrieved source.

**Representational limits, independent of game claims:** the current model cannot distinguish an unavailable or non-purchasable model from an ordinary purchasable one; represent zero-cost offers without violating the positive-price facet; name a currency; encode a multi-input recipe or eligibility condition; or record expiry/transfer/disposal. These are conditional expansion pressures, not requirements that the project copy every system. An abstract independent-game price can intentionally stand in for an acquisition system.

## 6. Stanford step 7 — examples and agreement checks

### Three concrete local examples, not verified real-game examples

| Example | Observed local record | What it exercises | Unverified real-game comparison |
|---|---|---|---|
| Anaconda | `ship_models.json:2–5`: engineer, price 12000, stock `heavy-laser` / `cruise-missile`. Inline purchase test at `model.rs:442–443` checks its stock loadout. | Positive catalog price, referenced model, generated owned ship, stock equipment. | Actual affiliation/manufacturer/rank, acquisition method/cost, availability and ownership lifecycle all unknown. |
| Hydra | `ship_models.json:14–17`: tackler, price 8000, stock `assault-railgun` / `homing-missile`; default demo model at `client/src/main.rs:15–16`. | Catalog selection is immediately instantiable locally without progression or payment service. | No external claim about the real ship's identity, class, price or unlock path. |
| Kite | `ship_models.json:34–37`: ecm, price 6000, stock `plasma-gun` / `homing-missile`; owned by `user-2` in hangar test `model.rs:467–470`. | Single-owner reference; foreign-owner hangar rejection is tested locally. | No evidence of actual faction/rank or whether real ownership is one instance per model/account. |

These examples satisfy local inspection, **not** the requested three representative real-game examples. That part remains blocked. When retrieval works, select three evidence-backed acquisition cases that genuinely exercise different rules rather than inventing diversity or assuming familiar names retain the same meaning.

### Agreed facts

- Narrative and canonical layer agree on model-versus-player-ship separation, purchase, and permanent ownership (`ontology.md:8–12`; `domain.md:90–100,162–170,200–201,247`).
- All nine instance prices satisfy the canonical positive integer range.
- Canonical ownership/hangar rules and `Hangar::validate` agree that deployment references must belong to the hangar owner and cannot repeat (`domain.md:248`; `model.rs:213–227`). Four deployment slots do not imply a four-ship ownership limit; narrative explicitly leaves other ships unused (`ontology.md:61`).
- No agreement between local data and actual Star Conflict has been verified by external evidence in this run.

## 7. Gruber assessment

| Criterion | Focused assessment |
|---|---|
| Clarity | Scope and permanent ownership are unusually explicit. Price lacks a currency/unit, and `buy` sounds transactional although it only constructs stock state. “Owned forever” needs a product interpretation before comparison with disposal or temporary-access evidence. |
| Coherence | Current nine prices meet their facet; model/instance ownership structure is consistent. C10 is a server obligation, not fully implemented by its helper. No external conclusion can contradict or corroborate the model without sources. |
| Extendibility | The model/instance separation and ID relationships can support later additions. Do not shoehorn future recipes/entitlements or corporate assets into price/user-owner fields if research later demonstrates a need. No new classes are recommended merely speculatively. |
| Minimal encoding bias | `u32` and a required positive scalar are concrete encoding commitments, not demonstrated truths about acquisition. Currency, eligibility and duration are independent concepts if they ever enter scope. |
| Minimal ontological commitment | Respect explicit progression/economy exclusions. The universal positive-price and permanent-ownership rules are stronger commitments than omissions; decide whether they are intentional local rules before treating reference-game differences as defects. |

## 8. Bounded follow-up research and unanswered questions

### Deferred navigation, not retrieved sources

Once normal network access is available, reuse the exact official and wiki seed URLs in section 2. If search is blocked but the wiki works, use its visible navigation/category links or public API. A candidate API discovery endpoint is `https://wiki.star-conflict.com/api.php?action=query&list=allcategories&aclimit=50&format=json` — **not fetched; route existence and response unverified**. Follow actual returned titles rather than citing guessed article URLs.

A focused sequence, not an unbounded crawl:

1. **Affiliation/progression:** find wiki overview and definitions for factions/subfactions/manufacturers, rank/tier/clearance, ship experience/synergy, crews/implants and ship-tree eligibility. Seek a dated official announcement when wording suggests a changed progression system.
2. **Acquisition/economy:** fetch original articles for direct purchase, construction and any claimed non-purchase access; distinguish credit-like balances, premium payments and materials only when supported. Include trade and monetization documentation relevant to acquisition, not every store offer.
3. **Social/collective assets:** use corporation, dreadnought and sector-conquest overviews to identify ownership/control/membership relationships; collect exact cardinalities only where documented.
4. **Targeted gap-closing pass:** obtain originals for the strongest purported exception to positive-price acquisition and strongest purported exception to permanent ownership. Check whether the exception is current, historical, event-specific or only a different meaning of “ownership.” Preserve conflicting sources and patch dates.

For each useful page retain title, exact URL, publisher/type, retrieval date, visible article/update/patch date (otherwise unknown), relevant heading, short faithful excerpt or precise paraphrase, and supported claim. Use search snippets only to discover originals; convert fetched HTML with markitdown before reading. No strong-page quota should substitute for source relevance.

### Product questions for reconciliation, not questions sent to the user

1. Is the product an independent Star Conflict-like game, or a reconstruction of a particular Star Conflict version?
2. Are the nine model names/prices illustrative? If fidelity is intended, what date/version and acquisition context apply?
3. Does the canonical positive price denote one abstract currency, a simplified balancing value, or an actual payable amount?
4. Does “forever” exclude only combat loss, or also disposal, transfer and expiration? Which interpretation is intended locally?
5. Does one owner permit multiple instances of the same model? Current helper permits this structurally; there is no explicit uniqueness contract.
6. Is a displayed ship catalog meant to imply every entry is immediately purchasable? Current Q1 lacks eligibility but progression is deliberately excluded.
7. If crew/implant/progression data is later introduced, should it modify account, ship instance, loadout or deployment selection? No external attachment/cardinality is established here.
8. Are faction/manufacturer and corporate/social concepts ever needed for existing competency questions, or should they remain contextual research only?

## 9. Verification and residual risks

### Commands/checks performed

- Initial `pwd`, presence checks for root `AGENTS.md`/`docs/HANDOFF.md`, `git status --short`, `git rev-parse HEAD`, `command -v markitdown` — passed; expected baseline, clean tree, requested instruction files absent, converter installed.
- Direct file reads and repository search for purchase/price/ownership callers — completed.
- `markitdown --help` — passed. No HTML conversion was possible because retrieval failed.
- Four bounded curl requests and one explicitly approved IPv4 diagnostic — **failed**, curl exit 28 / HTTP `000`, as recorded in section 2.
- Standard-library Python assertions over `ontology/instances/ship_models.json` — **passed**: exactly nine unique IDs; every price is an integer in positive `u32` range; distinct prices are `[6000, 8000, 12000]`; each record has exactly the expected eight model fields and no acquisition/progression/faction/currency fields. This is a local data check, not a Rust test or gameplay verification.
- `git diff --exit-code`, `git diff --cached --exit-code`, `git status --short`, `git branch --show-current`, `git rev-parse HEAD` — passed; no repository diff, no staged files, `master`, unchanged baseline.

No tests were added or updated. Existing Rust tests were read, not executed. No builds or installations were needed or attempted. Changed deliverable: only this external report plus managed retrieval diagnostics; no project files changed.

**Residual risks:** the main external research goal is incomplete; zero gameplay source claims were verified; real-game examples, mechanics/cardinalities, conflicting sources, historical evolution and currentness remain unassessed. An independent reviewer can accept the local analysis and honest retrieval diagnosis, but must not accept this report as a completed Star Conflict factual comparison. Future internet retrieval must redo the blocked external portion; local anchors, scope boundaries and test observations can be reused.
