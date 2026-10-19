# L0 — core/entities/elements/math_accent_tests
Hash do Código: 99382f90

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/entities/elements/math_accent_tests.rs`

## Contract and acceptance

Construct carriers through the frozen public boundary. Discriminate base,
normalized accent, `size`, `dotless`, and the two presence bits separately.
Equality, hash, clone, `plain_text`, `map_content`, `map_text`, and `repr`
preserve complete causality. Only base is a recursive child of `map_content`;
terminal transformation must not alter the structural carrier.

Equal values with different presence remain distinct. `repr` shows `size` and
`dotless` only when explicitly supplied, in public order; it never prints
effective cascading values or derived orientation.

Exercise clone and hash on complete carriers. Pairs differing only in accent,
size, dotless, size-presence, or dotless-presence must not collapse in equality
or causal hash.

## Opacity policy

Zero or multiple attributable `MathAccent` carriers, ambiguous extraction,
or divergent protected identity are `Unknown`, with no acceptance or mutation
credit.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/math_accent_tests.md`. Historical observations are not a new parity certification.
