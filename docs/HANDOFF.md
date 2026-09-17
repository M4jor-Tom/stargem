# Handoff — Star Conflict ontology audit

**Current state: documentation audit reviewed, corrected and accepted on 2026-09-17. User reconciliation remains open.** The paused synthesis was completed by a same-role fallback, not a native resume. Both independent reviews ran; one report required artifact recovery. The parent applied the single accepted P2 documentation correction and reran verification. No ontology/gameplay changes or new Rust test pass are claimed.

## Start here — continue, do not restart

1. Read `./AGENTS.md`, this file, and [the audit report index](ontology/handoffs/README.md).
2. Read the [completed synthesis and review disposition](ontology/README.md#review-disposition-and-parent-verification). Six successful research passes, both local audits, synthesis and independent reviews are complete. **Do not restart them or resume historical writers.**
3. Reconcile [QUESTION-001](ontology/questions.md#question-001) (fidelity/reference era), [QUESTION-002](ontology/questions.md#question-002) (scope) and [QUESTION-004](ontology/questions.md#question-004) (source/version policy) with the user before choosing repairs or features.
4. No ontology or gameplay implementation is authorized by these reports. Land explicitly approved decisions in `ontology/` before any domain code change. The user has explicitly requested commit + push of this completed documentation; that authorization does not extend to domain changes.

## Exact reconciliation paths

- `./docs/ontology/drift_developer_vs_internet.md` — **46 open root items**, including differences, deliberate simplifications, excluded scope and uncertain comparisons; not 46 proven bugs.
- `./docs/ontology/issues.md` — **20 open internal issue/ambiguity items**. Start with ISSUE-001 (passive counter overflow), ISSUE-002 (nonfinite numeric values), ISSUE-003 (validation boundary) and ISSUE-004 (thermic constraint).
- `./docs/ontology/questions.md` — **23 open user-owned questions**. Start with QUESTION-001 (fidelity/reference era), QUESTION-002 (scope) and QUESTION-004 (source/version policy).
- `./docs/ontology/README.md` — checkpoint navigation, completed Stanford/Gruber/CQ/constraint matrices, original-report crosswalk and continuation verification.
- `./docs/ontology/research/` — six curated, sourced topic dossiers, all retrieved 2026-09-16.
- `./docs/ontology/handoffs/README.md` — **15 original research/audit reports**. Consumed writer/review handoffs and their continuation reports have been deleted.

Final counts remain 46/20/23, all open. Documentation acceptance does not resolve these items or approve the external game's rules.

## Runtime checkpoint and native continuation

- Repository: `/home/theta/repos/stargem.nix`; branch: `master`; documentation checkpoint HEAD: `d8803a38a59f9985ee074cdd3dff2e6ae786fa41`.
- Unchanged canonical source baseline: `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`.
- Mission: `28a33a06-b296-4a78-8ae0-079881816489`.
- Historical workflow `507d64bc-df21-4587-a4fb-a4425950ac5f` ended after the requested interrupt. Writer `14ba2ed5-3446-45fe-8484-8c296684d847` was paused/resumable then; this session's `children.list` had no retained row, so a fallback was used. Earlier writer `4842c607-c5b6-4c01-9233-d0baa4e6b926` was stopped. Neither is a current work target.
- Continuation workflow: `d3d016e6-a3ed-4bb6-8611-3385a0e4816a`; three children completed their analysis. Its return was **`blocked-review`**, not a successful gate: the ontology review returned structured PASS but failed file-only report delivery.
- The parent recovered that complete independent report from saved structured JSON and applied/rechecked the evidence review's one P2 correction. The completed per-agent reports were subsequently deleted at the user's request. [Exact provenance, findings and final checks](ontology/README.md#review-disposition-and-parent-verification) record this manual acceptance without relabeling the failed run.

Any future delegated task must recheck live capabilities/resumability and keep one writer. Use these durable reports as the resume point; do not restart completed audit work because an old runtime receipt failed.

## What is preserved / what remains

- [x] Local inventory and semantic audit.
- [x] Executable audit and original test/probe evidence.
- [x] Successful internet research across ships/specials, combat, equipment, modes, progression/economy and history/version scope.
- [x] Draft drift/issues/questions and six curated research dossiers.
- [x] Mandatory AGENTS → HANDOFF instruction.
- [x] Original completed reports and validation source/log snapshot preserved; consumed writer/review handoffs deleted.
- [x] Finish/check document synthesis, method/CQ/constraint coverage, report crosswalk and source/anchor traceability (fallback continuation).
- [x] Independent evidence review: 22 original URLs across all six research areas; one P2 correction accepted and applied.
- [x] Independent ontology/simplicity review (recovered report), accepted adjustments and final parent verification.
- [ ] User decisions and subsequent ontology-first reconciliation.

The audit files were untracked at the pause; the user subsequently requested **commit + push of that unfinished checkpoint**, recorded as `d8803a3`. That publication did not complete the audit or authorize gameplay changes. This continuation completes documentation delivery only; the user requested its commit + push without a merge or branch change. Git history records publication. `ontology/`, `ontology.md`, `sim/`, `client/` and the production Cargo files remain unchanged.

## Evidence and verification

The completed validation child reported **7 existing tests passing** (4 ontology + 3 sim) using an already-installed Rust 1.92.0 fallback, plus malformed-input repros and seed checks. It did **not** successfully run the pinned Nix shell. These are historical audit results, not tests rerun during this checkpoint.

- [Complete validation report and exact commands](ontology/handoffs/audit-validation-7c6d92a5.md).
- [Saved test log](ontology/handoffs/validation-checkpoint/fallback-bfd-tests.log).
- [Saved debug probe log](ontology/handoffs/validation-checkpoint/probe-debug.log).
- [Saved release probe log](ontology/handoffs/validation-checkpoint/probe-release.log).
- Probe sources, wrapper scripts and remaining logs: `./docs/ontology/handoffs/validation-checkpoint/` (no build/dependency caches). Inspect machine-specific paths; copy/adapt to external scratch for reruns.

Normal project check, when the documented development shell is usable:

```sh
nix develop --no-write-lock-file -c cargo test --locked -p stargem-ontology -p sim
```

That command may fetch missing Nix dependencies; do not run it during a no-install checkpoint. The saved report documents the offline attempt and successful preinstalled fallback separately.

Final parent checks passed for Markdown paths/anchors, ID references/counts, original-report hashes, byte-preserved snapshots and intended-only scope. Recheck after later edits; inspect untracked files explicitly (ordinary `git diff` omits them), then:

```sh
git diff --check
git diff --exit-code -- ontology ontology.md client sim Cargo.toml Cargo.lock
git status --short
```

## Gotchas and chronology

- All first-round HTTPS fetches timed out. **The user reconfigured the firewall; the six retried research passes succeeded.** Do not resurrect the obsolete blanket network blocker.
- Official 2026 service/monetization announcements make the reference era particularly important. Preserve dates and uncertainty; never mix old wiki balance with newer announcements as one experimentally verified ruleset.
- Wiki pages are community-maintained even when officially hosted. Internet differences are evidence for a decision, not authority to replace intentional local choices.
- C12–C16 largely describe future runtime responsibilities. Missing implementation is not automatically an internal contradiction.
- The first orchestration failed on `emit` of undefined optional output metadata. Its inventory completed and was reused; no need to repeat it.
- The continuation's structured-output/file-only delivery failure did not erase the completed ontology analysis. Its recovered report is evidence, not a successful runtime receipt.
- Source/ontology changes remain behind user reconciliation. The **documentation audit is delivered**; choosing or implementing a gameplay slice is not the next authorized action.
