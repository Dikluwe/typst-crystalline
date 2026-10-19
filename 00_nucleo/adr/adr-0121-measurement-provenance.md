# ADR-0121 — Record the provenance of every measurement

**Status:** ACCEPTED
**Date:** 2026-07-05
**Applies to:** the entire project and every step that produces a number used to make a decision.
**Numbering history:** no dedicated file/number until P910; reconciled in that step by an actual scan of `00_nucleo/adr/` (slot `0121` was free and assigned in the same batch as `0122`).

---

## What happened

P569 recorded “4 lines” for a test document. P574 and P575 later tried to reproduce that number from P569’s commit and got “2 lines” — both with and without the orphan L1 code thought to be the cause. P569’s number could not be reproduced, and the actual cause remained unexplained.

The problem is not the number itself. There is no record of **what exactly produced that number** — the code state at the exact time of measurement, whether there were uncommitted changes then, or anything else that distinguished that moment from the final commit left in history.

## Rule

Any number used in a report to decide whether something is open or closed (page, line, or word counts, positions, execution times) must include:

1. **The commit hash** on which the test ran, or “uncommitted working tree” when applicable, with the exact list of files changed at that time (`git diff HEAD --stat`).
2. **The exact time**, if there is reason to think the state may have changed between measurements within the same step.

This does not replace existing rules (new decision required; verification discipline). It complements them: those rules say to measure before deciding; this rule records enough about the measurement for someone else later to reproduce the number or understand why they cannot.

## How to apply

Whenever a number appears in a future report, check whether one can answer “from exactly what code state did this number come?” without guessing. If the answer is “I don’t know,” the number must not be used to close anything — only as an indication to measure again.

## Relationship to previous rules

- **New decision required** — no accepted item is permanent without a new decision (cited by name; not materialized as its own ADR — see the reconciliation note in ADR-0119).
- **Verification discipline** (ADR-0119) — a claim about performance or problem magnitude needs a number, not an adjective.
- **This rule** — for a number to serve as evidence, its provenance must be known.

Together, the three cover the full cycle: decide again, with evidence, and make that evidence findable later.
