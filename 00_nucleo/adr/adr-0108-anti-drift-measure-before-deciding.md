# ADR-0108 — Anti-drift discipline: measure before deciding

**Status:** ACCEPTED
**Date:** 2026-06-17

---

## Context

A pattern of error repeated on this branch: the assistant **decided or classified from accumulated narrative or a plausible framing instead of measuring the source first**. Each time, the owner manually inserted the same move — **measure before deciding** — and the decision changed. The cases (zero-evidence examples for this ADR, all from this branch):

- **β1 (P339)** — a transport wrapper (`Content::Styled`) was treated as language form; measurement showed it was transport mechanics.
- **“The night” (P340)** — fidelity was assumed to require copying vanilla’s pass mechanics; the contract was the **output**, not the algorithm.
- **Mechanical `==` (P342/P344)** — content parity was measured with Rust `PartialEq` (mechanics); ADR-0107 corrected the criterion to **morphology**.
- **The failure narrative (P347→P347c)** — “tekt found a vanilla bug” was constructed and became more exciting with each step; the source (commit #3327, “Support text show rules that match their own output”) showed **deliberate design**. The method had to retreat.
- **Revocation (P347d)** — it was nearly respected as a language feature (“it is a Typst feature”); measurement showed **internal mechanics** (constructed only by the engine, with no user-facing entry point).

**ADR-0107** already turned **one** of these brakes (parity ≠ mechanics) into a rule. This ADR **generalizes the mechanism**: it names the **class** of error and its failure modes so the next brake of the same class is anticipated by the **prompt’s form**, not only by the owner’s attention. **Stated goal: reduce dependence on the owner as a brake — not eliminate it.**

## Decision — rules (verifiable, not exhortative)

A Tekt prompt must have a form that makes violations **detectable**. Each rule includes its detection criterion.

1. **Measure before deciding.** A **decision/classification** section must be **preceded** by the measurement (Phase A, `file:line`) that **produces it**.
   *Detectable violation:* the decision comes before the measurement, lacks a measurement, or the measurement merely “confirms” a decision already written. (β1, “the night,” failure narrative.)
2. **Language vs. mechanics, explicit and sourced.** Before treating something as relevant to parity, classify it from the source: semantics/syntax/morphology = language (parity); mechanics (Rust equality, bytes, steps, data structures) = diverges (ADR-0107).
   *Detectable violation:* a parity/acceptance criterion uses mechanics without explicit classification. (The `==` case, Revocation.)
3. **Intent vs. behavior.** Before reproducing vanilla behavior, distinguish **intentional** (language) from **generated** (implementation accident); **do not infer intent from behavior**.
   *Detectable violation:* “vanilla does X” is used to claim “vanilla intended X” without evidence of intent. (The failure narrative.)
4. **Mark inference and what would refute it.** A finding states what is **measured** versus **inferred**, and what evidence would **refute it**.
   *Detectable violation:* a conclusion lacks the distinction or refutation criterion.
5. **Distrust comfortable framing.** A **flattering** framing (a win, “we found a bug”) or a **convenient** one (the humble choice, “respect everything that exists”) requires an **extra source check before acceptance**.
   *Detectable violation:* such framing is accepted without a recorded extra check. (The failure narrative — flattering; Revocation — convenient.)
6. **Acceptance at the language level.** Acceptance criteria use semantics/syntax/morphology, **never** mechanics (boolean, byte, `PartialEq`), **except** where mechanics **is** the observable (the error message).
   *Detectable violation:* an acceptance gate says “the boolean matches” or “the bytes match” outside that exception. (ADR-0107 operationalized as a gate.)

## Residual risk (what this discipline does NOT reduce — required honesty)

This discipline catches **known** failure modes. It does **not** catch:

- **new failure modes** — drift from a class not yet named; the owner remains the brake, and a newly caught mode becomes a rule (the mechanism of this ADR itself);
- **reframing “one level up”** — seeing that the real goal differs from what is being optimized; that requires the owner’s judgment, not a checklist;
- **ritual compliance** — following the form without substance (Phase A measures something that does not determine the decision; a marked inference that nobody checks). Against this, the owner’s role **changes**: from “catch all drift” to **audit whether the discipline is followed in substance**.

Therefore this ADR **reduces** dependence on the owner as a brake — **it does not eliminate it**. It aims for “as much as possible,” not “zero.”

## Foundational note (the writer’s blind spot)

The assistant wrote this discipline to **catch its own drift**. A rule written by its author to catch itself has a blind spot: it can be phrased to be comfortable to follow **without substance**. Therefore the rules are **verifiable** (their form is detectable), not exhortative; and the owner, not the assistant, **validates this ADR**. This is **rule 5** (distrust convenient framing) applied to itself: “a discipline that absolves me” is the most convenient framing of all.

## Consequences

**Positive:** a prompt that violates the discipline is **malformed by its form** (detectable without the owner’s attention); the owner’s role shifts from “brake on all drift” to “auditor of substance.”

**Negative/limits:** it does not catch new modes, goal reframing, or ritual compliance (the residual risk). Reducing is not eliminating.

**Neutral:** no code changes; this is method.

## Precedents generalized by this ADR

- **ADR-0107** — “Parity is with the language, not the mechanics”: the **first** brake turned into a rule (rules 2/6 here operationalize it). This ADR names the class and adds its other five faces.
- Zero cases: P339 (β1), P340 (“the night”), P342/P344 (`==`), P347→P347c (failure narrative), P347d (Revocation).

## References

- ADR-0107 (generalized precedent).
- Steps P339, P340, P342, P344, P347→P347d (the actual brakes; zero-evidence examples).
