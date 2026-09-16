# Ontology audit — checkpoint index

**Paused, not final:** 2026-09-16. Read [../HANDOFF.md](../HANDOFF.md) before continuing.

| File | Paused draft contents |
|---|---|
| [drift_developer_vs_internet.md](drift_developer_vs_internet.md) | 46 root reconciliation items; includes intentional simplifications/scope gaps, not only contradictions |
| [issues.md](issues.md) | 20 internal issue/underspecification items |
| [questions.md](questions.md) | 23 user-owned decisions |
| [research/ships.md](research/ships.md) | Ship taxonomy, named models and specials |
| [research/combat.md](research/combat.md) | Damage, defense, movement, energy and weapons |
| [research/equipment.md](research/equipment.md) | Loadout eligibility, slots, stacking and payloads |
| [research/modes.md](research/modes.md) | PvP/PvE/Open Space, deployment and respawn |
| [research/progression.md](research/progression.md) | Acquisition, economy, progression and organizations |
| [research/history.md](research/history.md) | Source versions, chronology and reference scope |
| [handoffs/README.md](handoffs/README.md) | 17 per-child checkpoints with full original reports, plus two pending review briefs |

## Method and remaining gates

The inventory and local audits applied the `ontology` skill: scope/competency questions, reuse, vocabulary, classes, properties, facets and instances; and Gruber's clarity, coherence, extendibility, minimal encoding bias and minimal commitment. For this Rust project, align prose, Rust/Serde, runtime validators and JSON instances; do not invent a missing SQL requirement.

- [x] Inventory, semantic audit and executable audit completed.
- [x] Six internet research passes completed after firewall recovery.
- [x] Draft ledgers and dossiers written; full original reports preserved.
- [x] Paused writer and each earlier subagent have an explicit continuation handoff.
- [ ] Writer finishes detailed seven-step/Gruber/CQ coverage matrix, verifies completeness and deduplicates findings.
- [ ] Fresh evidence review and ontology/simplicity review.
- [ ] Apply accepted documentation corrections; verify links, IDs, counts, evidence and intended-only changes.
- [ ] User reconciles choices; only then update canonical ontology before implementation.

## Evidence policy

`ontology/` is the local source of truth. These reports are evidence and questions, not approved rules. Keep verified external differences separate from intentional design choices, excluded scope, historical changes and unverified comparisons. Record exact source URLs, retrieval date (2026-09-16), visible publication/revision date and uncertainty. Do not equate passing existing tests with sound facets or documentary evidence with tested live-game behavior.

Read [HANDOFF verification](../HANDOFF.md#evidence-and-verification) for exact command/provenance limits. Existing tests passed on the audit child's installed fallback toolchain; the pinned Nix shell did not run. No final review or new checkpoint-time test pass is claimed.
