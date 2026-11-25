# L0 — stdlib/calc/numeric
Hash do Código: 2c86970d

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/calc/numeric.rs`
**Governance:** ADR-0101, ADR-0107, ADR-0127, ADR-0129, ADR-0132.

## Contract

Own the private numeric projections shared by the calc children:

- Int/Float conversion to f64, with its existing type error.
- `DecNum`, the closed projection of a positional argument to Int, Float or
  Decimal. Any other type fails at that argument's value span with
  `expected integer, float, or decimal, found TYPE`, TYPE being the canonical
  long name. Decimal and Int combine as Decimal, Int and Float as Float. Decimal
  and Float never combine: `cannot apply this operation to a decimal and a float`
  with the hint `if loss of precision is acceptable, explicitly cast the decimal to a float with `float(value)``.
- Checked Float to Int conversion of an already rounded value: NaN fails with
  `the result is not a real number`, the closed-open interval [-2^63, 2^63) converts exactly, anything else fails with
  `the result is too large`.
- The NaN-result rejection `the result is not a real number`, only for the
  owners that name it. There is no general finite-result guard.
- The value span of the nth positional occurrence, detached when origins are absent.

Preserve native signatures, binding, phase, eager argument order and the
operands' kinds stated above. No I/O or mutable global state. Errors not named
here keep their current text; the message language is a separate decision.

## Acceptance

Cover each projection, both mixed-kind orders, the i64 boundaries, NaN and
infinities, and origin selection with and without occurrences.

## Evidence

Package approved by the owner on 2026-11-24 (“Aprovo A–F”). Measurement, causes,
scope and retained divergences: `00_nucleo/diagnostics/calc-numeric-package-2026-11-24.pt-BR.md`.
The previous text of this owner, with its extraction and earlier repair records,
is `00_nucleo/prompts/core/compiler/stdlib/calc/numeric.md` at commit `f2c737125`.
