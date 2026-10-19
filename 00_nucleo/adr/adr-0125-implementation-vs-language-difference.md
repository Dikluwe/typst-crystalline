# ADR-0125 — Implementation difference vs. language difference

**Status:** ACCEPTED
**Date:** 2026-07-09
**Applies to:** any step that introduces syntax, an argument, a property, or behavior that a `.typ` document can invoke.
**Numbering history:** no dedicated file/number until P910; reconciled in that step by an actual scan of `00_nucleo/adr/` (slot `0125` was free, the last in the batch after `0124`).

---

## The distinction

**Implementation difference (acceptable):** crystalline solves a problem differently from vanilla internally — different cache, algorithm, or data structure — but a valid `.typ` document produces the same visible result in both. The user never knows, and does not need to know, that the implementation differs.

**Language difference (unacceptable, except by deliberate decision):** syntax, accepted arguments, or observable behavior of a construct differs between crystalline and vanilla. A document written for one may fail or behave differently in the other. This breaks portability — the reason a parity project exists.

## What went wrong

`variant: (axis: value)` (P660) was implemented from a request that assumed, without checking, that this was actual vanilla syntax. The P660 probe itself confirmed the opposite — vanilla rejects that syntax. Implementation proceeded anyway, accidentally creating a language extension instead of making a deliberate decision.

This was caught only because P660’s report was read carefully, not because any existing rule had prevented it.

## Rule

Before implementing any syntax, named argument, property, or behavior that a `.typ` document can invoke, check it against the vanilla reference binary **before** writing code, not afterward:

```bash
lab/typst-original/target/release/typst compile <test-document>.typ output.pdf
```

If vanilla accepts it: this is a normal parity correction.

If vanilla rejects it: stop before implementation and make an explicit decision:

1. **Is this a deliberate extension with independent value?** (For example, `table.numbering`, P459 — a real convenience within the spirit of the language, even though it diverges from the architecture.) Document it as such with a written rationale, and clearly state that it is not portable to real Typst.
2. **Was it an initial mistake?** (For example, `variant: (axis: value)`, P660.) Do not implement it, or revert it if already implemented.

The difference between these cases is not technical — it is whether someone looking at the proposal before any code consciously decided, with a written rationale, to introduce the divergence, or whether the divergence appeared because the check was never done before proceeding.

## How to apply

In the step format already used by this project, any step introducing new syntax adds an explicit item to its probe section:

```markdown
## Confirmation against vanilla (required before implementation)

- [ ] Syntax/behavior tested directly against `lab/typst-original/target/release/typst`.
- [ ] If vanilla accepts it: proceed as a normal parity correction.
- [ ] If vanilla rejects it: record an explicit decision — deliberate extension (with rationale) or do not implement.
```

## Relationship to previous rules

- **New decision required** (cited by name; not materialized as its own ADR — see the reconciliation note in ADR-0119), **verification discipline** (ADR-0119), **measurement provenance** (ADR-0121), **parity defects** (ADR-0122), and the **sub-layout checklist** (ADR-0124) all concern how to verify a correction already within the scope of parity.
- **This rule** asks a question that comes before all of them: before implementation even begins, does the thing about to be built exist in vanilla, or is new language about to be invented unknowingly?
