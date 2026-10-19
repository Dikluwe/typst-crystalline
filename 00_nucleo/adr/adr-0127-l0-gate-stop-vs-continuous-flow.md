# ADR-0127 — L0 gate: mandatory stop for contract or behavior changes; continuous flow for internal fixes

**Status:** ACCEPTED
**Date:** 2026-08-04
**Decision owner:** Repository owner (in response to the question about P962–P964 — the distinction was already practiced; the owner accepted it as valid and requested that it be written as a rule).
**Recorded in:** Step 965.

---

## 1. Context

The “Golden Rule” (`CLAUDE.md`, Nucleation Protocol) says: the AI drafts L0 and **STOPS**, proceeding only after human confirmation. In practice (P893–P964), the actual stop occurred only in two kinds of situations, while internal corrections followed continuous flow (edit L0 first, without stopping). The original wording did not distinguish the two modes, and that ambiguity allowed the P952 deviation: new fields in `MathConstants` and a new method in `FontMetrics` were added **without** a stop, then corrected in a retrospective review. This ADR formalizes the criterion so it does not depend on case-by-case judgment.

## 2. Decision — the criterion

### MANDATORY STOP before writing code (confirm with the owner)

1. **Public contract:** add, remove, or change a public field of any entity (`entities/`, e.g. `MathConstants`, `GlyphVariant`); add or change a method on any public trait (e.g. `FontMetrics`); or change a public function signature in L1–L4.
2. **Product default behavior:** introduce a new default mode or flag, change the compiler’s default path (e.g. P956 — verbose emission as the default), or make any deliberate change to the user-facing surface. Parity corrections (item 3 below) are excluded.
3. **Architectural change to pipeline phase/order:** move logic between phases (e.g. eval ↔ layout, the type of decision in P966), even if the final implementation is small.
4. **Compatibility:** any change that, if later reverted, would break code already written against it (binary/API).

### CONTINUOUS FLOW (edit L0 first + reseal its hash; NO stop)

1. Correct a formula/value inside an existing function without changing its signature (P957, P959-formula, P963).
2. Add or correct entries in mapping tables — symbols, names, shorthand (P958, P962, P964).
3. **Parity corrections against vanilla in general**, even when they change rendered output (P958: `Gamma(z)` → Γ(𝑧); P961: reduced legend). Their gate is RED→GREEN tests plus revalidation (visual/`compare.py`/benchmark), not prior confirmation. This is the daily work of the effort; requiring a stop for every correction would halt progress without review benefit.
4. Any change whose reversal would break nothing outside its own module.

## 3. Verification against executed steps (P965 Phase A)

| Step | Type | Applied mode | Correct? |
|---|---|---|---|
| P893/896/906/909/915/918/922/937 | contract (fields/methods) | stop | ✓ |
| P956 | default behavior + public signatures | stop | ✓ |
| P959 | contract (2 `MathConstants` fields) | stop | ✓ |
| P952 | contract (field + trait method) | **no stop — deviation** | ✗ (corrected by retrospective review) |
| P957 | internal formula | flow | ✓ |
| P958 | table + fallback arm (parity) | flow | ✓ |
| P961 | annotation style (parity) | flow | ✓ |
| P962 | wrapper registration (parity) | flow | ✓ |
| P963 | internal formula (`is_text_like`) | flow | ✓ |
| P964 | table (Greek code points) | flow | ✓ |

The criterion cleanly separates all executed cases; no boundary case remained uncovered. One boundary case was discussed and resolved in the wording: a parity correction that changes visible output (P958) appeared to conflict with “default behavior.” The distinction is **product intent vs. convergence with vanilla**: the former stops; the latter flows.

## 4. Collision check (actual scan of `00_nucleo/adr/`)

A scan for `paragem|confirma|gate|PARAR` found no existing ADR that decides when to stop after editing L0. ADR-0114/0117 (probe before specification) concern investigation method; they complement this ADR and do not collide. The slot was confirmed by listing: the last file was `typst-adr-0126-*` → **free slot = 0127**.

## 5. Consequences

- **Positive:** the Golden Rule is no longer ambiguous; the criterion is verifiable (public signature? default behavior? pipeline phase? — if none apply, flow continues).
- **Positive:** the P952 deviation is classified as a violation of item 1 and can be detected by inspecting the diff (new fields/traits without a recorded stop).
- **Neutral/cost:** when a change class is uncertain, the rule of thumb is to **stop** — an unnecessary stop costs a message; a missed stop costs retrospective review (the P952 lesson).
- **Operational:** record the stop in the step report (“gate: confirmed by the owner on <date>”), as already done in P956/P959.

## 6. References

- `lab/history/construction/records/00_nucleo/materialization/typst-passo-965.md` (statement + owner’s proposed criterion).
- `lab/results/construction/diagnostics/typst-passo-952-relatorio.md` §6 (the deviation that motivated formalization).
- ADR-0114/0117 (probe before specification — method), ADR-0108 (measure before deciding), ADR-0121 (measurement provenance).
