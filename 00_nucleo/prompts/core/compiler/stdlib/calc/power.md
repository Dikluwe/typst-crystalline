# L0 — stdlib/calc/power
Hash do Código: 4aa05a4b

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/calc/power.rs`
**Governance:** ADR-0101, ADR-0107, ADR-0127, ADR-0129, ADR-0132.

## Contract

`pow` validates base then exponent before every guard. Base accepts Int, Float or
Decimal; exponent Int or Float. A Decimal exponent is rejected with
`expected integer or float, found decimal` and the exact hint
`if loss of precision is acceptable, explicitly cast the decimal to a float with `float(value)``.
Reject 0^0, an Int exponent outside i32 and a non-normal Float exponent other than
zero. Int to nonnegative Int is checked Int; Int to negative Int is Float; Decimal
to Int is checked Decimal; Decimal to Float has its dedicated error and hint; the
rest is Float. An infinite result is returned. A NaN base or result fails
with `the result is not a real number`; the reference turns a NaN base with an Int
exponent into 0.0 or infinity, which is not reproduced.

`sqrt` rejects negatives and otherwise returns the IEEE root, NaN and infinity
included. `root` takes the radicand then an Int index: index zero fails, a negative
radicand requires an odd index and gives the negative real root; results follow IEEE.

Project Value to the private closed base and exponent enums; numeric dispatch is
exhaustive and the nonnumeric group stays explicit, so a future Value forces
classification. No V16 suppression; the nonnumeric V19 group is intentional.

Preserve native signatures, binding, phase, eager argument order and the
operands' kinds stated above. No I/O or mutable global state. Errors not named
here keep their current text; the message language is a separate decision.

## Acceptance

Preserve the base-before-exponent, Decimal hint, kind, 0^0 and overflow controls.
RED→GREEN for infinite results with sign, NaN result, sqrt and root of non-finites.

## Evidence

Package approved by the owner on 2026-11-24 (“Aprovo A–F”). Measurement, causes,
scope and retained divergences: `00_nucleo/diagnostics/calc-numeric-package-2026-11-24.pt-BR.md`.
The previous text of this owner, with its extraction and earlier repair records,
is `00_nucleo/prompts/core/compiler/stdlib/calc/power.md` at commit `f2c737125`.
