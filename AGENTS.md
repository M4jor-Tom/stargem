# Agent instructions

Before doing work, if `./docs/HANDOFF.md` exists, every agent **MUST read it** to know what to do. When resuming delegated work, also read the assigned checkpoint in `./docs/ontology/handoffs/`; continue its remaining steps rather than restarting completed research. Verify native resumability before using a saved run ID; otherwise use the documented same-role fallback.

`ontology/` is the source of truth. Ontology decisions must be reconciled there before domain code changes. The audit in `docs/ontology/` is evidence and open questions, **not approved game rules**. Do not automatically start gameplay implementation from audit findings.
