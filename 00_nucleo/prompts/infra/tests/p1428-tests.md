# L0 — Public fraction representation and provenance tests
Hash do Código: 5c5f7ae8

**Layer:** L3 (integration tests)
**Exclusive consumer:** `03_infra/tests/p1428_math_frac.rs`

## Current evidence

- `03_infra/tests/p1428_math_frac.rs:141` — one-pass HTML text extraction.
- `03_infra/tests/p1428_math_frac.rs:240`, `:271` — evaluated carriers and unique fraction bits.
- `03_infra/tests/p1428_math_frac.rs:303`, `:358` — public repr and route/diagnostic/MATH/binom controls.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Extract HTML text in one pass from text nodes only. Decode known named and valid numeric entities once; unknown/malformed entities, absent or unbalanced markup fail observably. Do not recursively decode or compare escaped markup as the language repr.

Evaluate through eval_to_module_with_sink and traverse the resulting Content. Require exactly one MathFracElem to observe num_deparenthesized and denom_deparenthesized, without inferring state from fixture spelling. Preserve coverage of the three public frac forms, explicit/omitted style and cascade, signature diagnostics, OpenType constants and binom independence.

Acceptance concerns public behavior, domain carriers and layout invariants. Do not execute mutants/external runners, implementation hashes, artificial orders or administrative artifacts. The complementary public controls belong to `00_nucleo/prompts/infra/tests/p1427-tests.md`.
