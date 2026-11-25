# ADR-0101 — Numeric results of the standard library

**Status:** ACCEPTED
**Date:** 2026-05-20
**Revised:** 2026-11-24, approved by the owner (“Aprovo A–F”)
**Decision owners:** Typst Crystalline maintainers

## Evidence

The original decision kept a local guard that rejected infinite and NaN results in
the scalar `calc` functions, chiefly because it required no code change. It also
recorded that Float to Int conversion saturated in both compilers. At the current
pin the reference rejects an out-of-range conversion with `the result is too large`
and returns infinity where the guard failed. The same overflow was already accepted
through operators, so the result depended on the syntax used.
[Measurement and scope](../diagnostics/calc-numeric-package-2026-11-24.pt-BR.md)

## Decision

1. A numeric result follows the language at the pinned reference under
   [ADR-0107](adr-0107-language-parity-not-mechanics.md). Overflow, NaN, domain
   errors and conversions are decided per function. No general finite-result guard
   applies.
2. A conversion that cannot represent its value fails. It never saturates,
   and NaN has no integer value.
3. Each divergence is declared by the function's exclusive L0 owner with its
   measurement and reason. A clear reference defect is not reproduced.
4. Operators keep propagating IEEE values. Division by a literal zero keeps failing.

## Compatibility and verification

This revision replaces the categorical restriction and its function table. Programs
that relied on an error for an infinite or NaN `calc` result now receive a value;
out-of-range conversions now fail. Color constructor ranges, float parsing and the
language of diagnostics are outside this decision. The original text and its
P308–P310 records are this file at commit `f2c737125`.
The owners are under
[`00_nucleo/prompts/core/compiler/stdlib/calc/`](../prompts/core/compiler/stdlib/calc/numeric.md).
