# ADR-0107 — Parity is with the language, not the mechanics

**Status:** ACCEPTED
**Date:** 2026-06-17

---

## Context

One principle governed the project’s sound decisions without ever being written down: **crystalline’s parity with Typst is with the *language* — not with the execution *mechanics* that produce it.** Because it remained implicit, it was rediscovered — and rediscovered **incorrectly** — three times:

- **β1 (P339)** — a style-transport wrapper (local numbering’s `Content::Styled`) was treated as part of the language’s form.
- **“The night” (P340)** — fidelity was assumed to require copying vanilla’s pass mechanics (fixpoint/multi-pass), when the contract is the output, not the algorithm.
- **Content `==` (P342)** — content equality was measured with Rust’s `PartialEq` (structural/mechanical), which observes baked render style (Finding 2: `it.body == [a]` fails because of render `bold`, not a language difference).

Each stumble is the **inverse error of P329**. P329 says, “do not copy vanilla’s *structure*; fidelity is behavioral.” This ADR closes the other half: **do not *measure* the language by structure** (or Rust equality, output bytes, or algorithm steps). Both halves are the same principle.

## Decision

Parity between crystalline and Typst is defined at **three language levels**:

- **Semantics** — what a construct **means**. Example: `1 == 1.0` is `true` (ADR-0025); content `==` compares content **by what it is**, not by its data structure.
- **Syntax** — how a construct is **written** (the `.typ` grammar).
- **Morphology** — *(term introduced by this ADR; not previously used in the project)* — the **form of content as an object of the language**: text, markup structure, and **semantic** style (`*bold*`, `_italic_`) — **distinct from** resolved **render** style baked or derived by the implementation.
  - Example boundary: in `= a`, the morphology of `it.body` is the text **“a”**; the heading’s **bold** is **render** style, **not** part of morphology. Therefore `it.body` and `[a]` have the **same morphology**, and the language **must** treat them as equal.

**What is NOT parity (it is mechanics and deliberately diverges — P329 / ADR-0033 / ADR-0026):**

- restricted Rust equality (`#[derive(PartialEq)]` / structural comparison of structs);
- exact bytes of rendered output;
- algorithm steps (pass order, eager vs. multi-pass, …);
- internal data-structure form (which variant, which fields, which wrappers).

**How parity is measured.** By the construction’s semantics, syntax, and morphology — **not** by output-byte diffs or whether mechanical `==` returns the same boolean. **The compiled vanilla is an oracle for *semantics* when the source is ambiguous** (second-level trigger), **not** an oracle for bytes. Measuring parity through mechanics confuses implementation with language.

## Foundational note — this ADR was subjected to its own rule at birth

The first draft of this ADR (in P344) cited a **“Principle V”** and **ADR-0029** as precedents. Checking against the source — the same discipline this ADR establishes — refuted both:

- **“Principle V” does not exist** in the project: it appeared only in the step text drafting this ADR; ADR-0025 never names it. The citation was corrected to ADR-0025’s **actual decision** (the separation between Rust equality and Typst equality).
- **ADR-0029 is “physical purity in L1” (Arc)**, not behavioral fidelity. The correct behavioral precedent is **ADR-0033** (+ **ADR-0026** for `Content`’s structural divergence), with **P329** as the **north star** — a step, not an ADR.

This is recorded **not as an embarrassed footnote but as the first evidence**: the definition of parity was tested against drift at birth — trusting the narrative instead of the source — and held up because it was measured against the source. This is the zero case of this ADR’s rule applied to itself.

## Precedents unified by this ADR (not created by it)

- **P329** — the **north star** (a step, not an ADR): fidelity is **behavioral** (same `.typ` → same output), not structural; Rust structure is free.
- **ADR-0033** — “Functional parity with vanilla as an architectural invariant.” It establishes that **observable behavior** (visible output, error meaning, visible order) is the contract and the **internal form diverges** — the behavioral precedent inherited here. **Note on its name (verified against the source, P344):** ADR-0033’s content is **behavioral**, not “functionality” in the sense this ADR rejects — ADR-0033 itself **excludes byte identity**: “different bytes are acceptable only if the difference is invisible.” In ADR-0033, “functional” means **observable behavior** (what the language does); the rejected sense would mean “make mechanics/bytes equal,” which ADR-0033 also rejects. This ADR **refines where to measure**: at the language level (semantics/syntax/morphology), not by diffing observable output. The ADRs agree — 0033 views the contract from observable output; 0107 views it from the language.
- **ADR-0026** (+ R1) — `Content` deliberately diverges from the original’s structure (**enum vs. vtable**): data form is free.
- **ADR-0025** — Typst equality is **semantic** (`1 == 1.0`), **separate from** mechanical Rust equality (derived `PartialEq`, for tests/data structures; Typst `==`, for eval).
- **Language/render separation (P342)** — the language (eval, semantics, morphology) is the contract; rendering is the output; mechanics between them diverge.

This ADR supplies the missing face: **parity is with the language; mechanics are free.**

## Consequences

**Positive:** the principle stops being rediscovered (and rediscovered incorrectly) in every batch; acceptance criteria are written at the three language levels; the term **morphology** names the boundary between language content and render style.

**Recorded for immediate application, NOT executed by this ADR:** content `==` currently falls through to Rust `PartialEq` (`content.rs:1711`, via `value.rs:17`) — mechanics — and observes baked render style (Finding 2, P342). Extending ADR-0025 to content means it should be **semantic/morphological**. **That correction is a subsequent step, not this ADR.**

**Neutral:** this ADR changes no data structure; it defines a rule, not a refactor.

## Alternatives considered

| Alternative | Advantages | Disadvantages |
|-------------|------------|---------------|
| Leave the principle implicit (status quo) | No work | Rediscovered incorrectly in every batch (β1, “the night,” `==`) |
| Define parity by **output** (bytes/render) | Simple test | Confuses language with rendering; fails morphology (`it.body == [a]`); mechanics are deliberately divergent |
| **Define parity by language (semantics/syntax/morphology)** | **Names the real contract; unifies P329/0033/0026/0025/P342; provides a measurement criterion** | **Requires the new term “morphology” and discipline when writing criteria** |

## References

- **P329** — north star for behavioral fidelity (`lab/history/construction/records/00_nucleo/materialization/typst-passo-329*`; reaffirmed in `typst-passo-341-relatorio.md`, `prompts/entities/content.md`, `f_fronteira_e1.md`).
- **ADR-0033** — functional parity with vanilla as an invariant (observable behavior).
- **ADR-0026** / **ADR-0026-R1** — `Content` as an enum; intentional structural divergence.
- **ADR-0025** — `Int == Float`; Typst’s semantic equality is separate from Rust mechanics.
- **P342** — `lab/results/construction/diagnostics/typst-passo-342-relatorio.md` (language/render separation; Finding 2; `==` falling through to Rust `PartialEq`).
- **P339–P343** — the stumbles (β1, “the night,” `==`, bake inventory) that motivate the definition.
- **P344** — the step that drafted this ADR (and the foundational note above).
