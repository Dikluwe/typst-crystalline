# L0 — core/compiler/stdlib/structural/math/math_root_tests
Hash do Código: 64c351a6

Layer: L1 (test-only)
Exclusive consumer: `01_core/src/compiler/stdlib/structural/math/math_root_tests.rs`

## Evidence and classification

The consumer's `@prompt` at `01_core/src/compiler/stdlib/structural/math/math_root_tests.rs:2` establishes this exclusive ownership. Implementation inspection is not a new bilateral parity measurement; original historical qualifications remain in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Test obligations

Own only tests of qualified math.root, not implementation selection. Verify public function name root, the required positional index/radicand signature, Content|none casts and diagnostics N01–N07. Directly observe None/false, None/true and Some(content)/true carriers, including none radicand becoming empty. Require all three canonical repr morphologies without inferring presence from Option. Verify clone/equality/hash/map_content/map_text/plain_text/is_empty preserve existing semantics and the causal bit. Missing identity, contract-unattributable build failure, timeout, opaque/unreadable observations are fail-closed Unknown; tests must not edit a candidate to obtain GREEN.

## Limits

Preserve existing public observations and metadata. This documentary revision does not authorize productive-code, test-code, signature, default, pipeline-phase or compatibility changes. Respect exclusive ownership and pinned shared claims under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
