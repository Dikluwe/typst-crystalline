# L0 — stdlib/calc/extrema
Hash do Código: 26c068db

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/calc/extrema.rs`
**Governance:** ADR-0101, ADR-0107, ADR-0127, ADR-0129, ADR-0132.

## Contract

`min` and `max` take at least one positional value and fold left. Each later value
is compared with the retained extremum, later value on the left, by the
language ordering owned by
`00_nucleo/prompts/core/compiler/eval/operators/ordering.md`. Replace only when it is
strictly less for min or greater for max, so equality retains the first.
The returned value is the original argument: kind,
precision and sign are untouched. A comparison failure keeps the ordering's
message and is reported at the later argument's value span. One argument is
returned without comparison. Resolve no em against fonts and invent no order.

`clamp` takes three DecNum arguments. All Int gives Int; any Decimal gives Decimal;
otherwise Float. Reject a NaN minimum with `min must not be NaN`, then a NaN
maximum with `max must not be NaN`. When the bounds share a comparable kind,
reject reversed bounds with `max must be greater than or equal to min`. Anchor
the minimum error at the second positional value span and the others at the
third; detached stays detached. Only then reject Decimal with Float. A NaN value
with ordered bounds stays NaN; signed zero and infinite bounds are preserved. Use
total selection; no clamp with a runtime precondition, no panic path.

Preserve native signatures, binding, phase, eager argument order and the
operands' kinds stated above. No I/O or mutable global state. Errors not named
here keep their current text; the message language is a separate decision.

## Acceptance

RED→GREEN for mixed Int/Float identity above 2^53, NaN against another value,
every comparable family, incomparable em/pt and cross types with message and
span, Decimal combinations, reversed and NaN bounds with exact origins, signed
zero, extreme integers and singletons.

## Evidence

Package approved by the owner on 2026-11-24 (“Aprovo A–F”). Measurement, causes,
scope and retained divergences: `00_nucleo/diagnostics/calc-numeric-package-2026-11-24.pt-BR.md`.
The previous text of this owner, with its extraction and earlier repair records,
is `00_nucleo/prompts/core/compiler/stdlib/calc/extrema.md` at commit `f2c737125`.
