# Handoff: audit-validation — 7c6d92a5

Checkpoint: 2026-09-16, user-requested pause. Parent-curated from runtime records and actual files; not a new completed child report.

- **State:** COMPLETED; reuse output, do not rerun by default
- **Role:** `delegate`
- **Child run:** `7c6d92a5-b982-49e7-8cd0-feef1f74d5d4`
- **Workflow / key:** `08aa52d7-d2a2-4091-b856-28049dd1b39f` / `audit-validation`
- **Mission:** `28a33a06-b296-4a78-8ae0-079881816489`
- **Repository:** `/home/theta/repos/stargem.nix`, branch `master`
- **Source baseline:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`
- **Session (machine-local, optional for recovery):** `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/2026-09-16T18-47-01-030Z_01a0ab8b-30a5-761c-90d3-cc64125796e1/2b0a6135-c779-4d8b-be76-1deb700bdd43/run-0/session.jsonl`
- **Runtime status (machine-local):** `/tmp/pi-subagents-uid-1000/async-subagent-runs/7c6d92a5-b982-49e7-8cd0-feef1f74d5d4/status.json`

## Authority and boundaries

Read root `AGENTS.md` and `docs/HANDOFF.md` before work. `ontology/` remains canonical and unchanged. This is documentation/research, not authorization to change gameplay. No commit/push/merge/install is authorized. One writer per checkout. Preserve untracked audit files: they are the current work, not disposable clutter. Other agents' full research is intentionally file-backed to protect parent context.

## Resumption (for Agent)

Executable audit is complete: seven existing tests passed on fallback Rust 1.92.0; negative probes reproduced defects. Preserve exact qualifications and inspect the full evidence below. The pinned offline Nix shell did not run. Portable checkpoint files/logs are in validation-checkpoint/; no build caches copied. Only resume for a focused repro or validation question.

## Native continuation or fallback

Native resumability was verified with children.list at checkpoint. Recheck it in the next session; retained session availability can change.

```javascript
subagent({ action: "status", id: "7c6d92a5-b982-49e7-8cd0-feef1f74d5d4" })
subagent({ action: "children.list" })
// Only when resumable, and no original writer is still active:
subagent({ action: "resume", id: "7c6d92a5-b982-49e7-8cd0-feef1f74d5d4",
  message: "Read AGENTS.md, docs/HANDOFF.md and docs/ontology/handoffs/audit-validation-7c6d92a5.md. Continue ONLY the remaining work recorded there; preserve completed outputs and current files." })
```

If the retained session is missing or not resumable, do not claim a native resume or rerun all research. Launch a fresh `delegate` as a **same-role fallback continuation**, give it this handoff and the preserved reports, and start at the exact next step above. A resumed run may return a new run ID: record it and continue from the latest identity.

## Verification and acceptance

The original report below states exactly what this child checked and what remains uncertain. Completion of a research child does not mean that document synthesis, independent evidence review, ontology review or user reconciliation has completed. Follow [the main checkpoint](README.md) for dependency order and pending reviews.

## Preserved original output

Original managed artifact: `/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/audit/validation.md`

SHA-256 of original artifact: `00176f7234d9b21f2dc911ac23e7a7a03b0a8341968c6410e858cc31d4d217ff`

The complete output is embedded below so recovery does not depend on session/temp-file retention. Original relative code paths refer to the repository root unless the report says otherwise. Historical access failures stay historical.

---

# Local executable and representation audit

**Repository:** `/home/theta/repos/stargem.nix`  
**Baseline / inspected HEAD:** `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`, branch `master`  
**Audit date:** 2026-09-16  
**Scope:** local Rust/JSON/documentation only; no web or live Star Conflict claims.

## Executive findings

The supplied catalog is internally usable: all **46 records**, all **9 stock purchases**, and **7,686 capacity-legal passive multisets** passed the checks described below. All enum categories have seed representation. Existing tests pass on a preinstalled fallback toolchain, although the documented Nix shell could not be realized without installing missing dependencies.

The validators nevertheless admit malformed inputs:

| ID | Priority | Classification | Finding |
|---|---|---|---|
| V1 | P1 | Verified internal logical issue | 256 identical passives panic in debug and bypass slot limits in release. |
| V2 | P1 | Internal logical issue + finite-value coverage gap | Valid JSON `1e39` becomes infinite `f32`, passes both validators, and poisons flight state; NaN and finite-sum overflow also bypass facets. |
| V3 | P2 | Validation-boundary coverage gap + documentation/code contradiction | Client never validates loaded data; `effective_stats` does not enforce the positivity attributed to it by C5. |
| V4 | P2 | Verified local contradiction | Thermic `100/100` satisfies the validator but not canonical C9's “both ≈ 1.” |
| V5 | P2 | Identity/cardinality coverage gap | Duplicate damage/special records pass; an invalid duplicate damage row is invisible to validation until reordered. |
| V6 | P2 | Authoring coverage gap; policy not specified | Unknown fields are ignored; a misspelled modifier silently becomes zero. Duplicate slot-map keys are last-value-wins. |
| V7 | P3 | Definition/validator mismatch | Empty/non-kebab IDs pass despite the explicit ID convention. |
| V8 | P3 | Historical change / documentation drift | Narrative link was not updated after rename; crate-level constraint attribution overstates/contradicts implemented responsibility. |

**Priority meaning:** P1 = fix before these interfaces accept untrusted loadouts or newly authored data; P2 = correctness/authoring issue to resolve before expanding use; P3 = documentation/convention cleanup. No current network exploit is claimed: there is no server or loadout-submission path in the inspected tree. None of these findings says the present seed catalog is invalid.

## Authority, method, and files inspected

Root `AGENTS.md` and `docs/HANDOFF.md` were checked first and are absent. The requested ontology skill and supplied inventory were read. Canonical authority remains `ontology/`; `ontology.md` is a source narrative, not independently verified external-game truth. The audit made no commits, branches, pushes, package installations, source/ontology edits, nested-agent calls, or repository-local audit files. Reproductions and build output are only in managed scratch outside the repository. `git diff --exit-code`, `git diff --cached --exit-code`, and final `git status --short` confirmed no changes; HEAD remained the baseline.

Read in full: `ontology/domain.md`, `ontology.md`, `ontology/model.rs`, all eight `ontology/instances/*.json`, all four Cargo manifests, `sim/src/lib.rs`, `client/src/main.rs`, `README.md`, `flake.nix`, and `flake.lock`. Cargo lock versions/checksums were inspected programmatically for the fallback harness. Asset submodules were not traversed. The supplied inventory was used for orientation, not substituted for reading source or executing checks.

### Stanford seven steps applied to this audit

| Step | Application and outcome |
|---|---|
| 1. Domain/scope | Reviewed users, exclusions, Q1–Q10 and C1–C17 (`domain.md:8–38,234–254`). Separated a flight prototype and declarative combat vocabulary from a future authoritative server. CQ matrix below records what is executable. |
| 2. Reuse | Inspected every tracked Rust source and manifest. `ontology` is already shared by `sim` and `client`; no competing runtime catalog/schema or SQL persistence exists to reuse or validate. No external reuse research was attempted under the local-only scope. |
| 3. Terms | Checked three sizes, nine roles/special kinds, five passive families, three damage kinds, six stat fields, two active flows, two special activations, and three mode categories against every seed family. |
| 4. Classes/hierarchy | Documentary catalog/equipable abstractions map to separate structs and composition; role → size/special uses exhaustive matches (`model.rs:24–51`). PlayerShip has a model/loadout, rather than inheriting a model. No is-a cycle or invalid inheritance was found. Specials deliberately remain outside equipable catalog families. |
| 5. Properties | Traced intrinsic numeric properties and ID relationships, default-zero modifiers, external owners, four hangar slots and role-derived specials. Free-text effects are descriptive, not executable behavior. |
| 6. Facets | Compared prose facets with Rust domains, serde boundary behavior, catalog/loadout/hangar validators and actual consumers. Rust + JSON + validators are the relevant three layers; there is no SQL layer. Findings V1–V7 identify concrete misalignment or missing boundary policies. |
| 7. Instances | Read all 46 catalog records; ran existing tests, stock checks, complete capacity-legal passive-multiset enumeration and malformed-input probes. Current seeds pass, but negative tests expose validator weaknesses. Real-world fidelity is outside this local audit. |

### Gruber criteria

| Criterion | Assessment |
|---|---|
| Clarity | Units, relations, C/Q references and role mappings are clear. Weak points: vague “≈ 1,” damage spillover wording, activation-vs-state terminology, free-text effects and raw string ID convention. |
| Coherence | Present seeds and stock loadouts cohere. Counter overflow, NaN acceptance, thermal-profile checking, duplicate enum records and C5 attribution are evidenced breaks/gaps. |
| Extendibility | New catalog rows are straightforward; new enum concepts require explicit code changes. Tests pin 9 models/4 modes (`model.rs:435–436`), so adding examples also requires updating those fixture assertions, not a semantic change. Effects have no structured extension mechanism yet, appropriate until simulation requirements are chosen. |
| Minimal encoding bias | Readable string enum values and ID relations avoid magic numeric codes. `u8` counting leaks an encoding limit into cardinality validation (V1); `f32` conversions leak nonfinite values (V2). Missing slot keys → zero is an encoding convention, not a separate gameplay rule. |
| Minimal ontological commitment | Scope explicitly defers auth, progression, maps, AI and most economy. Do not add those merely to complete this audit. The role-special bijection, size-specific weapons and slot limits are current local commitments; sparse examples and missing runtime behavior do not establish external contradictions. |

## Execution evidence and limitations

Managed scratch, abbreviated **`$S`** below:

```sh
S=/home/theta/.pi-game-dev/sessions/--home-theta-repos-stargem.nix--/subagent-artifacts/outputs/08aa52d7-d2a2-4091-b856-28049dd1b39f/audit/scratch-validation
```

### Exact commands and results

All package/build commands used bounded timeouts and offline operation. No dependencies were downloaded or installed.

1. **Documented Nix environment attempted, unavailable.** From repository root:

```sh
timeout 240s nix develop --offline --no-write-lock-file --max-jobs 0 --option substitute false \
  -c env CARGO_TARGET_DIR="$S/target" cargo test --offline --locked -p stargem-ontology -p sim
```

**Exit 1; 0 tests executed.** The pinned shell closure is incomplete locally. First failure: `/nix/store/mhrxc5w6drwi4m5ykbdrayz7869i9mx9-bzip2-1.0.8.tar.gz.drv`, “local builds are disabled (max-jobs = 0)”; failures propagate to cargo 1.97.1 and `nix-shell-env`. Disabling builds/substitution intentionally preserved the no-install/no-fetch authorization. This is an infrastructure limitation, not an ontology test failure. Full evidence: `$S/existing-tests.log`, `$S/existing-tests.exit`.

2. **Preinstalled fallback discovery.** `cargo` was absent from PATH. Direct invocation of `/home/theta/.rustup/toolchains/1.92.0-x86_64-unknown-linux-gnu/bin/{cargo,rustc} --version` initially returned **127 each**, “cannot execute: required file not found” (unavailable normal ELF interpreter). Existing Nix glibc/runtime libraries allowed use without changing/installing the toolchain. A scratch wrapper `rust-tool` executes:

```sh
/nix/store/ias8xacs1h3jy7xgwi2awvim61k2ji6c-glibc-2.42-67/lib/ld-linux-x86-64.so.2 \
  --library-path /home/theta/.rustup/toolchains/1.92.0-x86_64-unknown-linux-gnu/lib:/nix/store/0iv8glcslgfcgn371lbjr5jjw5a6cqir-gcc-15.3.0-lib/lib:/nix/store/78x9i5x1wpqw4kq0h39b8f35abcv156h-zlib-1.3.2/lib \
  /home/theta/.rustup/toolchains/1.92.0-x86_64-unknown-linux-gnu/bin/"$tool" "$@"
```

`$S/rustc` and `$S/rustdoc` invoke that wrapper with their respective tool names. Verified versions: **rustc 1.92.0 (ded5c06cf 2025-12-08)** and **cargo 1.92.0 (344c4567c 2025-10-21)**. This is **not** a successful run inside the pinned Nix shell.

3. **Initial fallback linker attempt.** From repository root:

```sh
timeout 240s env RUSTC="$S/rustc" RUSTDOC="$S/rustdoc" \
  CARGO_TARGET_DIR="$S/fallback-target" \
  CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=/nix/store/adcz0m6qq2flmshdf0zz2xwjr5zbq1gr-gcc-wrapper-15.3.0/bin/cc \
  "$S/rust-tool" cargo test --offline --locked -p stargem-ontology -p sim
```

**Exit 101; 0 tests executed.** Preinstalled toolchain's `ld.lld` wrapper refers to missing `/nix/store/w8yihglsjnym11kbsc4yv1k7mq5m7vlj-rustup-1.28.2/nix-support/ld-wrapper.sh`. Log: `$S/fallback-tests.log`. The next command selected the already-installed BFD linker rather than altering the toolchain.

4. **Existing source tests, successful fallback.** From repository root:

```sh
timeout 240s env RUSTC="$S/rustc" RUSTDOC="$S/rustdoc" \
  RUSTFLAGS='-C link-arg=-fuse-ld=bfd' CARGO_TARGET_DIR="$S/fallback-target" \
  CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=/nix/store/adcz0m6qq2flmshdf0zz2xwjr5zbq1gr-gcc-wrapper-15.3.0/bin/cc \
  "$S/rust-tool" cargo test --offline --locked -p stargem-ontology -p sim
```

**Exit 0; 7 tests passed, 0 failed:** ontology 4, sim 3; both doc-test suites had 0 tests. Log: `$S/fallback-bfd-tests.log`. No client graphics/build run was needed or claimed.

5. **Disposable path-dependency harness.** `$S/probe/Cargo.toml` references the unchanged repository `ontology` and `sim` crates plus cached `serde_json`. No copied/reimplemented validator was tested. Lock preparation:

```sh
timeout 30s "$S/rust-tool" cargo generate-lockfile --offline --manifest-path "$S/probe/Cargo.toml"
```

**Exit 0.** All **49 external harness dependencies** were compared with the repository lock: matching name/version/source/checksum tuples. Logs: `$S/probe-lock.log`, `$S/probe-lock.exit`.

Debug execution:

```sh
timeout 240s env RUSTC="$S/rustc" RUSTDOC="$S/rustdoc" \
  RUSTFLAGS='-C link-arg=-fuse-ld=bfd' CARGO_TARGET_DIR="$S/fallback-target" \
  CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=/nix/store/adcz0m6qq2flmshdf0zz2xwjr5zbq1gr-gcc-wrapper-15.3.0/bin/cc \
  "$S/rust-tool" cargo test --offline --locked --manifest-path "$S/probe/Cargo.toml" -- --nocapture
```

**Exit 0; 12 observation tests passed, 0 failed; 0 doc tests.** `$S/probe-debug.log` records the caught overflow panic at `ontology/model.rs:355:28`, infinity propagation, and 7,686 enumerated loadouts. These tests deliberately assert observed flawed behavior; “passed” does **not** mean the validators are correct.

Release execution, same environment:

```sh
timeout 240s env RUSTC="$S/rustc" RUSTDOC="$S/rustdoc" \
  RUSTFLAGS='-C link-arg=-fuse-ld=bfd' CARGO_TARGET_DIR="$S/fallback-target" \
  CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=/nix/store/adcz0m6qq2flmshdf0zz2xwjr5zbq1gr-gcc-wrapper-15.3.0/bin/cc \
  "$S/rust-tool" cargo test --offline --locked --release --manifest-path "$S/probe/Cargo.toml" -- --nocapture
```

**Exit 0; 12 observation tests passed, 0 failed; 0 doc tests.** `$S/probe-release.log` records `256 repeats: validator accepted; shield=393000`. Full reproducible observation source: `$S/probe/src/lib.rs`.

6. **Independent current-seed representation scan.**

```sh
timeout 20s python3 "$S/check_seed.py"
```

**Exit 0; one assert-based scan passed:** 8 files, 46 records, 172 finite numeric inputs within finite `f32` magnitude, 389 nonduplicate object keys; every family has unique kebab-case IDs. Log: `$S/check-seed.log`. This is a JSON/seed check, not a substitute implementation of Rust validation.

### Existing test coverage versus audit probes

Existing ontology tests (`model.rs:431–478`) cover current catalog success, one stock ship, shield overfill by one, duplicate passive addition, one disallowed/allowed active case, weapon-size mismatch, hangar duplicate/foreign ownership, and distinct role specials. They do not cover overflow, nonfinite values, enum-family duplicates, malformed JSON fields, or all stock models. Sim tests (`sim/src/lib.rs:122–152`) cover ordinary cap/drag, yaw and one collision.

The additional 12 observation tests cover those seams, all stock models, unknown loadout references, unknown/missing enum records, slot/count/type boundaries, unknown ID spelling, exact hangar length, and all capacity-legal passive multisets. Enumeration treats duplicate passive IDs as legal, as the existing test explicitly does. It covers **multisets**, not every order of the same loadout; all current modifier sums remain finite and in range. It does not exercise future active combat effects, deployment, money, network input, or graphics.

## Complete relevant caller trace

All tracked Rust sources were searched, not only the functions named in the inventory. There are exactly three tracked `.rs` files.

| API | All repository callers / significance |
|---|---|
| `Catalog::load` (`model.rs:245–261`) | Client `client/src/main.rs:14`; ontology test fixture `model.rs:428`. Neither loader nor fixture implicitly validates; the catalog test explicitly does so. |
| `Catalog::validate` (`271–340`) | Only `model.rs:434`, the current-catalog test. No production caller. |
| `Catalog::validate_loadout` (`343–388`) | Only test calls at `443,447,449,454,456,460`. No production or server caller. |
| `Catalog::effective_stats` (`391–403`) | Client `main.rs:18`; loadout validator `model.rs:382`; additive-stat assertion `450`. Thus the client reaches the calculator without either validator, while loadout validation applies range checks after calculation. |
| `Catalog::buy` (`406–419`) | Client `main.rs:16`; tests `model.rs:442,466,467`. This constructs stock equipment; no caller treats it as a persisted monetary transaction. |
| `Hangar::validate` (`215–227`) | Only `model.rs:469`. No hangar runtime/UI/persistence consumer. |
| `ShipModel::slot_count` (`124`) | `validate_loadout:359`. Missing map entry becomes zero. |
| `Catalog::damage` (`268`) | Only `Catalog::validate:286`. Uses first matching vector row. No damage simulation caller. |
| `ship_model/passive/active/weapon/missile` lookup helpers (`263–267`) | `ship_model`: client `main.rs:17`, validator `345`, calculator `392`, purchase `407`. `passive`: `353,394`; `active`: `365`; `weapon`: `314,374`; `missile`: `319,380`. All use first matching ID. |
| Role mappings (`30–50`), `ShipModel::size` (`123`) | `size` is used in stock/loadout weapon checks (`316,376`) and client radius/HUD (`main.rs:20,90`); `special` in catalog presence validation (`301–302`) and mapping test (`476`). No special effect executor. |
| `FlightParams::from` / `sim::step` | Client `main.rs:19,62`; step also called by sim tests (`127,130,139`). `sim/src/lib.rs:54–57` copies speed and converts agility to radians without validation. Client calls step with frame delta (`main.rs:54`), not a fixed-step accumulator. |

This trace bounds impact: validator weaknesses are real library issues, but current loadouts are only stock client constructions or tests. The live client does consume mutable on-disk catalog data without validation, so invalid authoring can reach movement parameters today.

## Detailed findings

### V1 — Passive counting overflows and invalidates C4

**P1; verified internal logical issue.** C4/Q3 (`domain.md:31,241`), bounded loadout relation (`domain.md:202`).

**Source:** `ontology/model.rs:351–362` counts an unbounded `Vec<Id>` into `BTreeMap<PassiveSlotType, u8>`; `+= 1` at line 355 overflows before comparison with the model's 0–3 slots. Vector length itself is not constrained by serde (`model.rs:187–192`).

**Minimal reproduction using unchanged seeds:**

```rust
let c = Catalog::load(Path::new("ontology/instances")).unwrap();
let mut s = c.buy("anaconda", "owner", "ship").unwrap();
s.loadout.passive_module_ids = vec!["shield-extender".into(); 256];
let errors = c.validate_loadout(&s);
```

Observed: debug panics with `attempt to add with overflow`; default release returns **zero errors** because 256 wraps to 0. `effective_stats` adds all 256 modifiers and reports shield **393,000**, versus stock 9,000 and a legitimate three-extender maximum of 13,500. This is not a duplicate-item rule: duplicates are deliberately permitted by `model.rs:447–450`; the violated rule is per-family quantity.

**Impact:** a future service relying on this validator can panic or accept dramatically overfilled equipment. No network input reaches it currently. Catalog validation cannot prevent this; the fault is runtime loadout counting, not seed data.

**Smallest fix option:** count into `usize` and compare against `usize::from(slot_count)`; optional early total-length rejection can additionally limit work. Do not narrow/ban repeated IDs instead—that changes semantics without fixing the actual cardinality arithmetic.

**Smallest regression:** the above 256-passive case must return an error without panic in both debug/release; retain existing 3-accepted/4-rejected checks.

### V2 — Numeric validation admits NaN/infinity and overflowed effective stats

**P1; verified internal logical issue for NaN facets, plus missing finite-value boundary.** Stats/property facets (`domain.md:69–90,122–149`), C5/Q7, downstream motion (`sim/src/lib.rs:54–57,70–85`).

**Source:** catalog ranges at `model.rs:288–296,306–338` and effective ranges at `383–385` mostly reject using `x <= 0` or `x < 0`. These comparisons are false for NaN. Positive infinity passes most positive ranges. No finiteness check covers passive modifiers or the post-addition result (`390–403`). Heat's bounded conjunction at `325`, and the damage-profile comparisons, are important exceptions: they reject NaN rather than universally accepting it.

**Three independent executed reproductions:**

1. Replace only Anaconda's `"speed": 180` with valid JSON numeric literal `"speed": 1e39` in an outside-repository copy/string of `ship_models.json`. `serde_json::from_str::<Vec<ShipModel>>` converts it to **`f32::INFINITY`** on the locked dependency version. Both `Catalog::validate` and `validate_loadout` return no errors. Passing the stats into actual `FlightParams::from` and one `sim::step` at `dt=1/60`, throttle 1, yields **`Vec3(NaN, NaN, -inf)`** position. This does **not** rely on nonstandard JSON `NaN`/`Infinity` tokens.
2. In public Rust objects, set base armor, weapon fire rate, missile reload time, active energy/cooldown or passive shield modifier to `f32::NAN`. The probe combining these changes passes catalog validation; equipping the affected passive passes loadout validation while effective stats are NaN. A JSON `NaN` token is rejected by serde; direct public-object construction is a distinct path.
3. Set the shield-extender delta to **finite** `f32::MAX`, then equip two (within Anaconda's three shield slots). Catalog validation passes; summation produces infinite shield HP; loadout validation still passes. Guarding only parsed inputs would not fix arithmetic overflow.

**Impact:** authored JSON can poison today's flight state even if the currently omitted validators are added. This is independently significant from V3. All current 172 numeric seed inputs are finite, and all enumerated seed passive combinations remain finite/in-range; no present stock ship is affected.

**Fix options:** enforce `is_finite()` plus existing sign/range facets on each physical numeric input, allow finite negative passive deltas, and enforce finiteness/ranges on final effective stats. A single shared stats check can align base/effective validation. Authoritative input/state checks later remain a separate server responsibility; do not use hypothetical networking to defer catalog/stat checks.

**Smallest regression:** valid JSON `1e39`, a NaN base stat, and two finite `f32::MAX` deltas must each fail validation before reaching flight. Preserve zero shield/regen, positive armor/energy/speed/agility, and legitimate negative modifiers. No arbitrary balance ceilings are proposed.

### V3 — Validation is opt-in, and C5 names a calculator as its checker

**P2; boundary coverage gap plus verified documentation/code mismatch.** C1–C2/C5–C8, Q1/Q3–Q7; `domain.md:238–245`, `model.rs:245–261,342–419`, `client/src/main.rs:14–19`.

`Catalog::load` checks files and serde representation only. `buy` resolves the model but copies stock weapon/missile IDs without checking them. `effective_stats` returns `Some(sum)` whenever model/passive lookups succeed, regardless of positivity or loadout capacity. Only `validate_loadout` checks effective ranges afterward. Domain C5 explicitly assigns this invariant to `Catalog::effective_stats`; the helper's own line-390 documentation promises `None` for unknown references only. These are different contracts.

**Executed boundary reproduction:** outside-repo catalog copy changes Anaconda `price=0`, `speed=-1`, and stock weapon to `"missing"`. Load succeeds, `buy` succeeds, and `effective_stats` returns speed −1. Explicit catalog validation finds **3 errors**; explicit loadout validation finds **2**. The actual client calls neither. The graphical client was not launched; propagation into its parameter construction is a source proof.

**Executed C5 reproduction:** change the existing light-armor delta to −7,000 armor; the catalog remains valid because finite negative deltas are allowed. Equip one on Anaconda (one armor slot available). `effective_stats` returns `Some` with armor **−1,000**; `validate_loadout` correctly reports `effective stats out of range`. Thus the range check exists, but not at the function attributed in C5.

**Impact:** invalid authoring currently reaches the client; callers can mistake successful load/calculation for a valid entity. This is not evidence that every calculator must validate every unrelated weapon/ownership rule, nor that `buy` must implement payments inside this library.

**Fix options:** choose and document one boundary contract. Minimal integration option: validate the catalog once immediately after loading, validate a loadout before use, and describe `effective_stats` as an unchecked calculator with positivity checked by `validate_loadout`. Alternatively provide checked loading/stat computation and use it in the client. Avoid repeated ad hoc guards only in the renderer; future consumers need the same explicit contract. Finite guards from V2 are needed whichever option is chosen.

**Smallest regression:** malformed stock catalog must be rejected before constructing `FlightParams`; a legal-count negative-effective-stat loadout must be rejected by the documented checked API. No money/account fixture is necessary.

### V4 — Thermic validation checks similarity, not proximity to one

**P2; verified contradiction between canonical C9 and executable validator.** C9/Q6 (`domain.md:34,246`), `model.rs:283–296`, seed `instances/damage_types.json:4`.

```rust
let d = c.damage_types.iter_mut().find(|d| d.id == DamageKind::Thermic).unwrap();
d.vs_shield = 100.0;
d.vs_armor = 100.0;
assert!(c.validate().is_empty()); // observed
```

The validator tests `abs(vs_shield - vs_armor) < 0.2`, which permits any equal positive scale. Canonical text says **both ≈ 1**. Present seed values 1/1 are correct. The source narrative's weaker “Balanced power over Armor and Shields” (`ontology.md:19`) may explain the implementation, but cannot override the canonical requirement in this audit.

**Impact:** Q6 can expose a 100× thermal profile as validated content. There is no damage simulation yet, so this is a catalog contract fault, not an observed combat result.

**Fix options:** obtain/document the intended tolerance around 1 and check each multiplier against it. If “equal at any scale” was actually intended, changing canonical C9 is a product decision, not an authorized audit fix. Existing cross-difference 0.2 is not sufficient evidence of the desired around-one tolerance.

**Smallest regression:** 1/1 accepted and 100/100 rejected; add tolerance-edge cases only after that tolerance is specified.

### V5 — Damage and special IDs are presence-checked, not unique

**P2; verified identity/cardinality validation coverage gap.** Damage properties/C9 and role-special 1↔1 relation (`domain.md:67–72,145–155,196–198,246,254`). C1 may be intended broadly, but the hierarchy explicitly excludes specials from catalog items and lists damage separately (`domain.md:46–54`); this finding does not silently broaden C1.

**Source:** uniqueness at `model.rs:277–282` covers ship/passive/active/weapon/missile/game-mode IDs, not damage/special enum IDs. `damage()` returns the first match (`268`); validation examines that one per enum kind (`285–297`). Special validation checks only existence (`300–304`).

**Executed reproduction:** append a second electromagnetic row with `vs_shield=-1`, after the valid row; append a second drones record with conflicting effect text. Catalog validation returns zero errors. Swap the invalid electromagnetic row to the front: catalog validation now fails. Thus even a positivity-invalid stored row is ignored depending on list order. Enum typing excludes unknown names but cannot ensure row uniqueness in a `Vec`.

**Impact:** multiple incompatible definitions can masquerade as a valid catalog; first-match lookup and future consumers can disagree. All current damage/special IDs are unique, and every required kind is present.

**Smallest fix option:** reject duplicate enum IDs in these two vectors, retaining existing required-kind checks. A map representation is an alternative, not needed for this small fix and not automatically safer if deserialization silently overwrites duplicate keys.

**Smallest regression:** appending a duplicate kind must error even if its other values match; a conflicting duplicate cannot disappear from validation when reordered. Check one damage and one special family.

### V6 — Authoring typos can become successful, ineffective data

**P2; coverage gap, not a violation of an explicit unknown-field policy.** Q7 / stat-modifier encoding (`domain.md:84–86`, `model.rs:83–93`). No relevant struct opts into `deny_unknown_fields`.

**Executed minimal reproduction:**

```rust
let m: StatModifiers = serde_json::from_str(r#"{"sheild_hp":1500}"#).unwrap();
assert_eq!(m, StatModifiers::default()); // all zeros
```

Omitted modifier fields intentionally default to zero; unknown fields are silently discarded. Combined, a typo erases the intended modifier without an error. There is no subsequent modifier-shape check in `Catalog::validate`. Ordinary required fields still fail when missing: missing base `speed` and missing loadout `missile_id` were tested. Unknown enum values, misspelled slot-type keys and count 256 in a `u8` slot were rejected. This is **not** a claim that serde accepts arbitrary malformed data.

A second observed representation seam: duplicate slot-map JSON keys such as `"shield":3,"shield":1` deserialize as 1. Derived struct-field duplicate handling and typed-map-key duplicate handling must not be conflated. All current seed files have nonduplicate object keys.

**Impact:** authors can ship silent no-op modifiers or unintentionally overwritten slot declarations. Forward-compatible readers may deliberately ignore new fields, so strictness is a product/format policy rather than a proven universal requirement.

**Fix options:** simplest strict authoring policy is unknown-field rejection on catalog/value structures, especially defaulted modifiers. If forward-compatible readers are intentional, add a strict authoring/CI check instead. Decide whether duplicate JSON keys must be rejected there as well. Preserve intended default-zero omitted fields.

**Smallest regression:** misspelled `sheild_hp` is rejected by the chosen authoring boundary while `{}` remains a valid zero modifier. A separate duplicate-key sample should fail only if that stricter policy is adopted.

### V7 — ID grammar is documentation-only

**P3; verified definition/validator mismatch.** `domain.md:6` says IDs are stable kebab-case; `model.rs:8` aliases `Id = String`; six-family uniqueness does not validate lexical shape (`273–282`).

**Executed reproduction:** replacing `game_modes[0].id` with `"BAD ID"`, then with `""`, independently leaves `Catalog::validate()` empty. No other reference needs updating for this isolated example. Present seed IDs are all nonempty kebab-case.

**Impact:** new authored data can violate documented naming conventions, harming predictable references/tooling. This is not an observed duplicate-ID failure or an auth exploit. Stability over time cannot be checked from one catalog snapshot, and owner IDs reference an external auth system whose syntax is not defined here.

**Fix options:** validate the documented catalog-ID grammar at the content boundary; a newtype is optional, not required. Separately clarify whether external owner identifiers share the lexical rule. Do not impose external-user syntax without that decision. ID permanence belongs to lifecycle/migration policy.

**Smallest regression:** empty and space/uppercase catalog IDs are rejected; existing kebab-case IDs remain accepted; same textual ID in different catalog families remains allowed by C1.

### V8 — Documentation drift obscures the executable boundary

**P3; historical change / internal attribution issue.**

- `domain.md:4` still links to `../onthology.md`; the baseline commit `4a47fa3` renames `onthology.md` → `ontology.md` with zero content changes. Verified by `git show --stat HEAD -- ontology.md onthology.md ontology/domain.md`. This is a broken local reference explained by repository history, not an external-game historical claim.
- `model.rs:1–2` says the crate checks C1–C11 and runtime-only C12–C17 belong to the server. In reality C3 and the payment portion of C10 are assigned to the server by `domain.md:240,247`; C17 is assigned to the type system (`254`) and special presence is checked at `model.rs:300–304`. C5's specific attribution is V3.
- `sim/src/lib.rs:3` speaks of sharing with the authoritative server; `README.md:11` correctly says **future** server. Workspace members (`Cargo.toml:3`) are only ontology/sim/client.

**Fix option:** repair the narrative path and accurately describe which catalog/loadout facets are checked versus deferred. No gameplay change is needed.

**Smallest regression/check:** the documented narrative path exists; a short responsibility review matches C3/C5/C10/C17 to actual APIs. Do not fabricate server tests to satisfy a prose-only future responsibility.

## Invariant and representation coverage matrix

| Area / constraint | Verified present behavior | Limits / classification |
|---|---|---|
| C1 catalog references | Stock weapon/missile IDs resolve; stock weapon size checked (`model.rs:314–321`). Every damage enum kind and role-derived special is required, so weapon/missile damage references resolve when catalog validation succeeds. Unknown stock missile, wrong-size stock weapon, removed damage/special kind were rejected by probes. | Validation opt-in (V3); duplicate enum-family identities (V5). User references are external, not a missing local auth catalog. |
| C1 uniqueness | Six string-ID families have explicit duplicate checks; current all eight families are unique. Duplicate ship model is rejected. Family-local uniqueness intentionally permits the same string in different families. | No globally unique typed ID namespace is promised. PlayerShip global uniqueness is a future store precondition, discussed below. |
| C2 slots | `u8` rejects negative/>255 representation; validator rejects 4–255. Unknown slot family rejects. Missing key means zero (`124`); empty map is accepted. | Missing-as-zero is a code convention worth documenting, not proven invalid data. Duplicate keys overwrite (V6). |
| C4 passives | Each occurrence consumes a slot of its module family; repeats permitted. Ordinary overfill rejects; all references checked. | Counter overflow V1. No extra “unique module” constraint should be invented. |
| C5 stats | Six additive fields, finite negative deltas allowed. Unknown model/passive makes calculation return `None`; loadout validation reports the unresolved reference. Zero shield/regen allowed; other effective stats must be positive. All 7,686 seed multisets passed. | Numeric facets V2, checked-vs-unchecked calculator V3. Do not demand that every possible catalog modifier be positive or useful on every ship. |
| C6 active equipment | Length uses `usize` and rejects >4; each reference and role restriction checked; empty allowed-roles means unrestricted. Four repeats of shield-boost accepted, five rejected. | Active uniqueness is **not** required by the current ontology. Active effect execution/energy affordability is future C16, not equip-time validation. |
| C7/C8 weapon/missile | Required scalar IDs encode one of each; lookup + size check for weapon, lookup for missile. Missing required missile field and unknown references reject. | No missile-size restriction is defined; absence of such a check is not a bug. Helpers rely on catalog validation for weapon/missile numeric facets. |
| C9 profiles | EM/kinetic directional profile checks, positive multipliers, all three kinds present. Q6 can read data. | Thermal V4, duplicate rows V5, nonfinite edge cases V2. Actual damage/overflow into armor is not implemented. |
| C10 purchasing | `buy` selects model, copies stock weapon/missile, starts with no passive/active modules. All nine tested stock results satisfy validators on present data. | No money, debit, persistence, account lookup, ship-ID allocation or uniqueness transaction. Explicit future-server responsibility, not a hidden incomplete payment routine. |
| C11 hangar | Rust/serde require exactly four optional slots; 3/5 reject. Validator detects repeated slot IDs, missing ship and wrong owner for a unique-ID supplied ship list (`215–225`). Empty slots permitted. | Owner authorization and permanent ownership cannot be inferred from string equality. No player/hangar persistence exists. See duplicate-store precondition below. |
| C17 specials | No special field in Loadout; passive/active/weapon/missile IDs resolve in separate families. Role determines special exhaustively. | An unknown incoming `special_module_id` field would be ignored, not actually equipped. Missing special records caught by catalog validation; duplicates V5. Same string in two separate families does not make them the same entity. |
| Required vs optional | Required struct fields reject missing data; serde enums restrict named domains. Modifier fields are intentionally optional/default-zero. Hangar entries are optional, not the four-element shape. | Unknown data policy V6. Optional/default/absent distinctions are not blanket validation failures. |
| Numeric storage | Integer domains enforce representation limits; exact positive/range facets are runtime checks. All current numeric seed values are finite. | Comparisons alone do not establish finiteness or protect addition (V2). No arbitrary gameplay maximum is inferred beyond explicit facets. |
| Enum/sample coverage | Seeds cover 3 sizes, 9 roles, 9 special kinds, 3 damage kinds, 5 slot families, 2 active flows, 2 special activations and 3 mode categories. Role-to-size/special matches are exhaustive. | One model/role and one interceptor weapon are sparse sample coverage, not a proven demand for exhaustive live-game representation. |

### Ownership/hangar boundary: observed ambiguity, not a present auth bypass

`Hangar::validate(owned)` searches `owned.iter().find` by ship ID (`model.rs:221`). The probe supplied two rows with the same `PlayerShip.id`, one owned by the hangar owner and one foreign. Owned-first passes; foreign-first fails. This demonstrates an undocumented **unique PlayerShip-ID repository precondition**, not that a unique foreign ship passes the owner check. There is no production caller or storage implementation, C1 only expressly assigns catalog identity checks, and auth is out of scope.

Recommendation: require unique player-ship IDs at the eventual authoritative persistence boundary and document the validator's input precondition. Optionally reject ambiguous duplicate IDs defensively if this API will receive arbitrary lists. Smallest future check: storage rejects two owned-ship rows with one ID, regardless of owner/order. Likewise, permanent ownership and authorization to mutate a hangar are server responsibilities; public mutable Rust fields alone do not establish a violation of C3/C10/C11.

## Competency questions: promised versus executable

| CQ | Current answer mechanism | Executable status / caveat |
|---|---|---|
| Q1 buyable models/prices | `ship_models`, `price`, `buy` | Catalog enumeration and stock construction work; no eligibility/progression or charge implemented. Those exclusions/deferred server rules must not be read into this CQ. |
| Q2 size and special from role | `ShipRole::size/special` | Fully executable mappings; catalog special-presence check. No execution of the special effect. |
| Q3 passive in model slot | `module_type`, `slots`, `validate_loadout` | Executable but V1 breaks large counts. No individually numbered slot assignment is required because slots are family counts. |
| Q4 active role eligibility | `allowed_roles`, `validate_loadout` | Executable, including any-role empty list and ≤4 limit. Cooldown/energy/effect behavior is separate. |
| Q5 weapon fit by size | `Weapon.size`, derived model size | Executable stock/loadout checks. Catalog has 2 frigate, 2 fighter and 1 interceptor weapon. |
| Q6 shield versus armor damage | DamageType multipliers | Numeric data/query available; V4/V5/V2 weaken validity. Applying hits, mixed-layer spillover and resistance/special effects are not executable. |
| Q7 effective stats | `effective_stats` | Additive calculation executable; legal current seed combinations validated. Returned `Some` is not a validity guarantee (V2/V3). |
| Q8 deployable owned ships | four hangar slots, `Hangar::validate` | Representable and validator available, but no stored user examples, deploy selector or production hangar caller. |
| Q9 respawn and deployment source | category/respawn bool, C12 | Fields are readable; hangar respawn versus station/selected-ship deployment is only prose. No match/space-station/selected-ship runtime object or executor exists. Explicit server gap, not a contradictory implemented rule. |
| Q10 destruction/cloak/overheat | C13–C15 | Declarative only. `sim` runs motion/collision, not armor damage, cloak or weapon heat. Current HUD displays initial shield/armor (`main.rs:90–91`), not changing combat state. |

C3 immutability enforcement, C10 payment, C12 deployment, C13 destruction, C14 cloak drop, C15 heat and C16 energy/payment-over-time are future authoritative behaviors. C9's damage application also depends on that simulation. These are **executable coverage gaps explicitly delegated by the ontology**, not evidence that current code enforces the opposite behavior. There is no reason to implement them during this audit.

## Deliberate simplifications, sparse seeds, and unresolved semantics

- **Explicit deliberate simplifications:** arcade motion (`ontology.md:52`); common tunable acceleration/drag (`sim/src/lib.rs:54–57`); spherical collision hulls (`61–68,94–96`); one placeholder art hull (`client/src/main.rs:25–32`); omitted wave/operation generators (`domain.md:256–260`). These are not validator defects. Client frame-time stepping versus `step`'s “fixed-step” wording is documentation/future deterministic-server integration work, not evidence that today's finite-input flight tests fail.
- **Sparse seed coverage, intent not explicitly documented:** nine ship models and only one interceptor weapon. Narrative says multiple weapon types per size (`ontology.md:51`), but canonical constraints impose no minimum count of weapons by size. Record as sample coverage/clarification, not an invalid seed or external-game contradiction. No player/hangar persisted examples are expected: `domain.md:275` expressly delegates them to the server.
- **Descriptive effects, not typed/executable mechanics:** active effects are designer-facing strings (`domain.md:116`); specials also have effect strings. Emergency barrier's “3 seconds” (`instances/active_modules.json:11–13`) has no typed duration; phasic shield describes resistances absent from base Stats; missile names “homing”/“EMP” do not encode guidance/status algorithms. This is future executable coverage, not proof that names or effects are wrong.
- **Damage spillover uncertainty:** C9 does not explicitly define conversion from shield-consumed scaled damage back to raw damage. Example: raw 100, remaining shield 50, EM profile 1.5/0.7. Raw-budget conservation would apply `(100 - 50/1.5) * 0.7 ≈ 46.67` to armor; reusing leftover shield-scaled damage would give `(150 - 50) * 0.7 = 70`. No implementation resolves which “remainder” means. Product decision and eventual one-case regression, not a current numerical contradiction.
- **Activation vocabulary uncertainty:** phasic-shield is `toggle` but cycles three resistance types (`special_modules.json:4`); source narrative also calls this a toggle (`ontology.md:27`). Clarify whether the enum names input interaction or a two-state behavior before implementing. Command-shield `toggle` is additional canonical instance specificity, not automatically a contradiction with narrative “activate.”
- **No external provenance conclusion:** ship names/roles, balance values, special effects and game-mode names were not compared with web sources or a dated game version. No verified Star Conflict contradiction or external historical change is claimed. The only verified historical change here is the local narrative-file rename.

### Product decisions to retain in the report, not block this audit

1. Is catalog/JSON loading a raw parse API requiring explicit validation, or should successful loading imply a validated catalog? Which API guarantees C5-valid effective stats?
2. What tolerance formally defines thermic “≈ 1”? Preserve the canonical intent unless separately authorized to change it.
3. Are authored JSON files strict schemas, or must unknown fields be tolerated for forward compatibility? What is the duplicate-key policy?
4. Does the kebab-case ID rule cover external user IDs, or only locally controlled entity IDs? Which future store allocates and enforces permanent unique player-ship IDs?
5. Are the nine models/weapon subsets examples or an intended exhaustive local launch catalog? Which CQs are declarative design answers versus near-term executable acceptance criteria?
6. Before combat implementation, choose the raw-damage spillover formula and distinguish special activation input from effect state. Do not infer either from prose labels alone.

## Minimal follow-up order

1. Repair C4 counting and finite-value validation, with the small negative checks above; preserve existing duplicate-passive and negative-delta semantics.
2. Establish a single explicit validated-data boundary, align C5 attribution, reject ambiguous damage/special definitions, and settle C9 tolerance.
3. Choose strict authoring policy, enforce/document ID grammar, and repair documentation links/responsibility claims.
4. Keep future server/combat responsibilities and sparse seed expansion separate from these local validator fixes. No ontology semantics or gameplay implementation was changed by this audit.
