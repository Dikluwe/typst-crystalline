# L0 — core/compiler/stdlib/structural/math/math_frac_tests
Hash do Código: ef599b12

Layer: L1 (test-only)
Exclusive consumer: `01_core/src/compiler/stdlib/structural/math/math_frac_tests.rs`

## Evidence and classification

The consumer's `@prompt` at `01_core/src/compiler/stdlib/structural/math/math_frac_tests.rs:2` establishes this exclusive ownership. Implementation inspection is not a new bilateral parity measurement; original historical qualifications remain in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Test obligations

Own only tests of the frozen math.frac contract cases P01–P07, P16 and N01–N11, without candidate inspection. Observe the closed binding and positional required num/denom, content casts (none/Str accepted, Int rejected), only named style with vertical/skewed/horizontal values, omission remaining None versus explicit vertical, canonical repr and all causal fields. Duplicate named values must use ArgOccurrence sequence, never a collapsing map. Clone/equality/hash/map_content/map_text preserve style, both unparenthesizing bits and line, transforming only contents recursively. repr must not expose the bits even on a directly constructed carrier with both true. Missing binding, non-callability or inability to observe the carrier fails; no textual approximation, other math binding or productive implementation is authorized.

## Limits

Preserve existing public observations and metadata. This documentary revision does not authorize productive-code, test-code, signature, default, pipeline-phase or compatibility changes. Respect exclusive ownership and pinned shared claims under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
