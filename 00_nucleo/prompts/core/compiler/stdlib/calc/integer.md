# L0 — stdlib/calc/integer
Hash do Código: c52db627

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/calc/integer.rs`
**Governance:** ADR-0101, ADR-0107, ADR-0127, ADR-0129, ADR-0132.

## Contract

Implement even, odd, rem, rem-euclid, div-euclid, quo, gcd and lcm.

The four division functions take two DecNum arguments, dividend first. A zero
divisor fails with `divisor must not be zero` at the divisor's value span. rem
follows the dividend's sign; rem-euclid is nonnegative; div-euclid is Euclidean;
quo is floored and always Int. Int uses checked operations and fails with
`the result is too large`; the remainder of the minimum Int by -1 is 0. Decimal
uses the checked operations of `00_nucleo/prompts/core/entities/decimal.md`:
remainder failure is `dividend too small compared to divisor`, quotient failure
is `the result is too large`. Float follows IEEE. quo of Float or Decimal outside
i64 fails with `the result is too large`; a NaN quotient fails with
`the result is not a real number`.

even and odd accept only Int. gcd(0,0) is 0; lcm with zero is 0; lcm overflow is
checked.

Preserve native signatures, binding, phase, eager argument order and the
operands' kinds stated above. No I/O or mutable global state. Errors not named
here keep their current text; the message language is a separate decision.

## Acceptance

RED→GREEN for Decimal with Decimal and Int in both orders, Decimal with Float and
its hint, zero divisors of each kind with span, signs, exact negative multiples,
minimum Int by -1, quo bounds and NaN, and the existing integer controls.

## Evidence

Package approved by the owner on 2026-11-24 (“Aprovo A–F”). Measurement, causes,
scope and retained divergences: `00_nucleo/diagnostics/calc-numeric-package-2026-11-24.pt-BR.md`.
The previous text of this owner, with its extraction and earlier repair records,
is `00_nucleo/prompts/core/compiler/stdlib/calc/integer.md` at commit `f2c737125`.
