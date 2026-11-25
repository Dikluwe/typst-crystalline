# L0 — stdlib/calc/round
Hash do Código: 0a6d9896

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/decimal-rounding.toml sha256:dcf8dc8e474613bab33bbf5dff39124b8711f2facb85b731e5d63363a347e6d3

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/calc/round.rs`
**Governance:** ADR-0101, ADR-0107, ADR-0127, ADR-0129, ADR-0132.

## Contract

`floor`, `ceil` and `trunc` return Int. Int is identity. Float rounds and then
uses the checked conversion of `numeric`; NaN gives 0 and -2^63 is accepted.
Decimal rounds and converts. Out of i64 range fails with `the result is too large`
at the call's argument span. Never saturate.

`round` keeps Int, Float and Decimal kinds. `digits` accepts Int, saturates to
i16 for Int/Float and i32 for Decimal and never truncates modularly. Int with
negative precision rounds away from zero with checked overflow. Finite Float
rounding follows the pinned decimal-rounding claims, and a result that overflows
to infinity is returned. Non-finite Float operands are returned unchanged for
every precision. `fract` keeps its kind; fract(Int) is Int 0.

Preserve native signatures, binding, phase, eager argument order and the
operands' kinds stated above. No I/O or mutable global state. Errors not named
here keep their current text; the message language is a separate decision.

## Acceptance

Native argument path RED→GREEN for values at and beyond both i64 bounds,
infinities, NaN, Decimal out of range, round with non-finite operands and
overflow, extreme digits in the three kinds, ties and signed zero. Expectations
must not repeat the implementation's formula.

## Evidence

Package approved by the owner on 2026-11-24 (“Aprovo A–F”). Measurement, causes,
scope and retained divergences: `00_nucleo/diagnostics/calc-numeric-package-2026-11-24.pt-BR.md`.
The previous text of this owner, with its extraction and earlier repair records,
is `00_nucleo/prompts/core/compiler/stdlib/calc/round.md` at commit `f2c737125`.
