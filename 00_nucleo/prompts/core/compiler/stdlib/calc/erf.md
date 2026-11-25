# L0 — stdlib/calc/erf
Hash do Código: 2dabca3c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/calc/erf.rs`
**Governance:** ADR-0101, ADR-0107, ADR-0127, ADR-0129, ADR-0132.

## Contract

Return libm's error function of one Int or Float as Float. NaN propagates,
infinities give ±1, signed zero is preserved.

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

RED→GREEN against constants measured on the pinned reference with tolerance 1e-14,
plus NaN. Preserve infinities, signed zero, arity and type controls.

## Evidence

Package approved by the owner on 2026-11-24 (“Aprovo A–F”). Measurement, causes,
scope and retained divergences: `00_nucleo/diagnostics/calc-numeric-package-2026-11-24.pt-BR.md`.
The previous text of this owner, with its extraction and earlier repair records,
is `00_nucleo/prompts/core/compiler/stdlib/calc/erf.md` at commit `f2c737125`.
