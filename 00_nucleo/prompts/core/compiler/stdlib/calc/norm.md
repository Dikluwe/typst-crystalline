# L0 — stdlib/calc/norm
Hash do Código: 3c814df1

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/calc/norm.rs`
**Governance:** ADR-0101, ADR-0107, ADR-0127, ADR-0129, ADR-0132.

## Contract

Variadic p-norm of Int/Float values, named `p` default 2.0. p <= 0 fails with
`p must be greater than zero` at p's value span. No values gives 0.0. Infinite p
gives the largest magnitude. Otherwise scale by the largest magnitude before
raising, so that a representable norm does not overflow. All zeros give 0.0.
A NaN value gives NaN. NaN p follows the
formula. Otherwise any infinite value gives infinity,
deliberately: the reference divides infinity by infinity there and returns NaN.

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

RED→GREEN for infinite p, 1e300 scale, negative and zero p with span, infinite
and NaN values, NaN p, empty and all-zero input. Preserve ordinary norms.

## Evidence

Package approved by the owner on 2026-11-24 (“Aprovo A–F”). Measurement, causes,
scope and retained divergences: `00_nucleo/diagnostics/calc-numeric-package-2026-11-24.pt-BR.md`.
The previous text of this owner, with its extraction and earlier repair records,
is `00_nucleo/prompts/core/compiler/stdlib/calc/norm.md` at commit `f2c737125`.
