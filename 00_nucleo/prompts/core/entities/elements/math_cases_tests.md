# L0 — core/entities/elements/math_cases_tests
Hash do Código: 45046410

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/entities/elements/math_cases_tests.rs`

## Contract and acceptance

Construct `MathCases` only through the frozen public boundary. Distinguish
children and their order, both optional delimiter sides, `reverse`,
`Rel<Length>`, and every bit of `MathCasesExplicit`. Absence and an explicit
value equal to the default must not collapse in equality, hash, or `repr`.

Clone, `plain_text`, `map_content`, `map_text`, and observable reconstruction
must preserve every field. `map_content` transforms children in order;
`map_text` is terminal. A `MathAlignPoint` stays inside its child until the
layout consumer; it never becomes a row/cell in the public carrier.

## Opacity policy

Missing binding, zero or multiple attributable carriers, divergent identity,
ambiguous extraction, or unreadable execution are `Unknown`. They grant neither
acceptance nor mutation credit.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/math_cases_tests.md`. Historical observations are not a new parity certification.
