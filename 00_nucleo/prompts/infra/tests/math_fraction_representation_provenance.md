# L0 — Public fraction representation and provenance tests
Hash do Código: 32e720c5

**Layer:** L3 (integration tests)
**Exclusive consumer:** `03_infra/tests/math_fraction_representation_provenance.rs`

## Current evidence

- `03_infra/tests/math_fraction_representation_provenance.rs:141` — one-pass HTML text extraction.
- `03_infra/tests/math_fraction_representation_provenance.rs:240`, `:271` — evaluated carriers and unique fraction bits.
- `03_infra/tests/math_fraction_representation_provenance.rs:303`, `:358` — public repr and route/diagnostic/MATH/binom controls.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Extract HTML text in one pass from text nodes only. Decode known named and valid numeric entities once; unknown/malformed entities, absent or unbalanced markup fail observably. Do not recursively decode or compare escaped markup as the language repr.

Evaluate through eval_to_module_with_sink and traverse the resulting Content. Require exactly one MathFracElem to observe num_deparenthesized and denom_deparenthesized, without inferring state from fixture spelling. Preserve coverage of the three public frac forms, explicit/omitted style and cascade, signature diagnostics, OpenType constants and binom independence.

Acceptance concerns public behavior, domain carriers and layout invariants. Do not execute mutants/external runners, implementation hashes, artificial orders or administrative artifacts. The complementary public controls belong to `00_nucleo/prompts/infra/tests/math_fraction_public_controls.md`.


## Explicit decoder domain guard

### Evidence

`lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`
retains current sources/L0/SHA-256, HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/status/diffstat.
Archived item T361 is math_fraction_representation_provenance.rs:100. The hash-
reconfirmed inventory records 28 guards across 21 owners; pinned CI requires
splitting or moving conditions into bodies. This is Rust structure, no functional-
defect, better-AI-readability or upstream-intent proof.

### Contract and acceptance

The test decoder recognizes hex prefix through
get(..2).is_some_and(eq_ignore_ascii_case("#x")), accepting only ASCII #x/#X
without slicing UTF-8. Preserve radix 16, decimal handling, char::from_u32,
Unknown::HtmlEntity failure and once-only decoding. Before replacement add controls
for uppercase X, Unicode scalar/surrogate/above-maximum/truncated/invalid-UTF8 prefix.

Continuous ADR-0127 changes no public/default/compatibility/phase contract. Owner
scope remains exclusive. No helper merely hiding conjunction, lint suppression
or linter-configuration change. Run existing functional controls before code and
preserve expectations. Real RED is V17 gate with 28 warnings; GREEN requires zero
V17. Check success/fallback/error, short-circuit/effects and scoped resealing.
Revalidate build/full workspace suite/all-target Clippy with warnings and
unfulfilled expectations as errors/fmt/Rustdoc warnings as errors/strict V1/V5/
V15/V26/reverse lineage/SARIF without new warnings under 12 GiB, zero swap, one job.
No unmeasured memory/performance/global-parity claims.
