# Handoff — Star Conflict ontology audit

**State: deliberately paused at the user's departure request, 2026-09-16. The audit is NOT finished.**

## Start here — continue, do not restart

1. Read `./AGENTS.md`, this file, and [the per-subagent index](ontology/handoffs/README.md).
2. Continue the [paused documentation writer](ontology/handoffs/write-ledgers-14ba2ed5.md). Six successful research passes, the inventory and both local audits are already complete. **Do not rerun them wholesale.**
3. Finish synthesis and verification, then run the two **not-yet-started** independent reviews: [evidence](ontology/handoffs/review-evidence-pending.md) and [ontology/simplicity](ontology/handoffs/review-ontology-pending.md). Apply accepted documentation corrections and recheck.
4. Only after the audit is reviewed, reconcile the open items below with the user. No ontology or gameplay implementation is authorized by these reports.

## Exact reconciliation paths

- `./docs/ontology/drift_developer_vs_internet.md` — **46 draft root items**, including differences, deliberate simplifications, excluded scope and uncertain comparisons; not 46 proven bugs.
- `./docs/ontology/issues.md` — **20 draft internal issue/ambiguity items**. Start with ISSUE-001 (passive counter overflow), ISSUE-002 (nonfinite numeric values), ISSUE-003 (validation boundary) and ISSUE-004 (thermic constraint).
- `./docs/ontology/questions.md` — **23 draft user-owned questions**. Start with QUESTION-001 (fidelity/reference era), QUESTION-002 (scope) and QUESTION-004 (source/version policy).
- `./docs/ontology/README.md` — checkpoint navigation; the writer must finish the methodological/CQ coverage and final verification summary.
- `./docs/ontology/research/` — six curated, sourced topic dossiers, all retrieved 2026-09-16.
- `./docs/ontology/handoffs/README.md` — **one checkpoint per each of 17 launched subagent runs**, plus two pending review briefs. Completed original reports are embedded, including details that curated dossiers may have omitted.

Counts describe the paused draft and must be recomputed after further edits. Neither independent review nor final parent acceptance has happened.

## Runtime checkpoint and native continuation

- Repository: `/home/theta/repos/stargem.nix`; branch: `master`.
- Unchanged source HEAD: `4a47fa3015f2d31bbe1c5a3159a3d0980c273912`.
- Mission: `28a33a06-b296-4a78-8ae0-079881816489`.
- Last workflow: `507d64bc-df21-4587-a4fb-a4425950ac5f`; enclosing workflow ended as **failed due to the requested interrupt**, not a new source failure.
- Current writer: **`14ba2ed5-3446-45fe-8484-8c296684d847`**, role `worker`, state **paused**, process terminal observed. `children.list` explicitly reported it **resumable** at checkpoint.
- Previous writer `4842c607-c5b6-4c01-9233-d0baa4e6b926` was **stopped**, is **not resumable**, and must not be used.

Before resuming, inspect live state and ensure there is only one writer:

```javascript
subagent({ action: "list", capabilities: true })
subagent({ action: "status", id: "14ba2ed5-3446-45fe-8484-8c296684d847" })
subagent({ action: "children.list" })
// Only if still resumable:
subagent({ action: "resume", id: "14ba2ed5-3446-45fe-8484-8c296684d847",
  message: "Read AGENTS.md, docs/HANDOFF.md and docs/ontology/handoffs/write-ledgers-14ba2ed5.md first. Continue the remaining documentation synthesis, preserve completed research and all checkpoint files, and report verification and remaining review gates. Do not change canonical ontology or gameplay code." })
```

The parent added this handoff, the checkpoint README and per-agent files **after** the writer paused: a resumed writer must read them before editing. It must not overwrite them using stale session assumptions. Resume returns a new run identity; retain the newest identity for later continuation.

If a new session cannot access the retained child, use a fresh `worker` explicitly labeled **same-role fallback continuation**, with the writer handoff and preserved reports. That is not a native resume, but it starts at the same remaining task. Do not replay the whole old workflow. Keep subsequent multi-step execution in one async workflow and bind durable child outputs through `output`.

## What is preserved / what remains

- [x] Local inventory and semantic audit.
- [x] Executable audit and original test/probe evidence.
- [x] Successful internet research across ships/specials, combat, equipment, modes, progression/economy and history/version scope.
- [x] Draft drift/issues/questions and six curated research dossiers.
- [x] Mandatory AGENTS → HANDOFF instruction.
- [x] Per-child checkpoints, original completed reports and validation source/log snapshot.
- [ ] Finish/check document synthesis, method/CQ coverage and exact source/anchor traceability.
- [ ] Independent evidence review.
- [ ] Independent ontology/simplicity review, accepted adjustments, final verification.
- [ ] User decisions and subsequent ontology-first reconciliation.

The audit files were untracked at the pause; the user subsequently requested **commit + push of this unfinished checkpoint**. Preserve them and use Git history for the checkpoint commit. This publication does not complete the audit or authorize gameplay changes. No merge or branch change is involved. `ontology/`, `ontology.md`, `sim/`, `client/` and the production Cargo files remain unchanged.

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

Before claiming final delivery, check all Markdown paths/anchors and ID references, recompute counts, verify original-report checksums, inspect new files explicitly (ordinary `git diff` omits untracked files), then:

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
- Source/ontology changes remain behind user reconciliation. Finish this **documentation audit**, not a gameplay slice.
