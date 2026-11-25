# L0 — stdlib/calc/logarithms
Hash do Código: 3ba7180b

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/calc/logarithms.rs`
**Governance:** ADR-0101, ADR-0107, ADR-0127, ADR-0129, ADR-0132.

## Contract

`exp` returns the exponential. A finite exponent may overflow to infinity and
negative infinity gives 0.0. Positive infinity and NaN exponents fail with
`exponent may not be infinite, subnormal, or NaN`.

`ln` and `log` require a strictly positive value. `log` defaults to base 10; the
named base and the legacy second positional are mutually exclusive; the base must
be finite, positive and different from 1. Bases e, 2 and 10 use their exact
branches before the generic quotient. A non-finite `log` result fails with
`the result is not a real number`. An infinite `ln` result fails with the
same message: the reference says `result close to -inf` for positive infinity, which
is wrong and is not reproduced. NaN propagates through `ln`.

`log10` (positive value, same result rule as `log`), `deg` and `rad` (plain numbers,
IEEE result) are deliberate crystalline extensions.

Preserve native signatures, binding, phase, eager argument order and the
operands' kinds stated above. No I/O or mutable global state. Errors not named
here keep their current text; the message language is a separate decision.

## Deterministic transcendental implementation

Use the pure functions of the project-pinned libm 0.2.11, named in the
`crystalline.toml` whitelist. No platform-dependent native transcendental and no
local `disallowed_methods` exception is permitted in this consumer. The choice of
implementation is mechanics; types, values, argument order and errors are
language observables. Neither universal bit equality nor general parity is claimed.

## Acceptance

RED→GREEN for exp overflow and each non-finite exponent, ln/log/log10 of infinity
and NaN, deg/rad of non-finites. Preserve base selection and domain controls.

## Evidence

Package approved by the owner on 2026-11-24 (“Aprovo A–F”). Measurement, causes,
scope and retained divergences: `00_nucleo/diagnostics/calc-numeric-package-2026-11-24.pt-BR.md`.
The previous text of this owner, with its extraction and earlier repair records,
is `00_nucleo/prompts/core/compiler/stdlib/calc/logarithms.md` at commit `f2c737125`.
