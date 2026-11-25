# L0 — stdlib/calc/trig
Hash do Código: 002e2abc

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/calc/trig.rs`
**Governance:** ADR-0101, ADR-0107, ADR-0127, ADR-0129, ADR-0132.

## Contract

`sin`, `cos` and `tan` take Int, Float (radians) or Angle and return the Float IEEE
result, NaN included. `asin` and `acos` require [-1,1], rejecting NaN; with `atan`
they return Angle. `atan2` takes x then y and returns Angle. Keep the unary helper
shared by the transcendental families; it applies no finite-result guard.

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

RED→GREEN for NaN and infinite input of the three direct functions. Preserve
quadrants, domains, Angle composition and arity controls.

## Evidence

Package approved by the owner on 2026-11-24 (“Aprovo A–F”). Measurement, causes,
scope and retained divergences: `00_nucleo/diagnostics/calc-numeric-package-2026-11-24.pt-BR.md`.
The previous text of this owner, with its extraction and earlier repair records,
is `00_nucleo/prompts/core/compiler/stdlib/calc/trig.md` at commit `f2c737125`.
