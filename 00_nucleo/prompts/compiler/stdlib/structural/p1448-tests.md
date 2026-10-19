# L0 — stdlib/structural/p1448-tests
Hash do Código: 6c2ad4c0

Layer: L1 (test-only)
Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1448_tests.rs`

## Evidence and classification

The consumer's `@prompt` at `01_core/src/compiler/stdlib/structural/p1448_tests.rs:2` establishes this exclusive ownership. Implementation inspection is not a new bilateral parity measurement; original historical qualifications remain in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Test obligations

Verify math.underbrace and the bare route converge to Content::MathUnderbrace. Require a positional content-convertible body and only one optional second positional annotation; named/missing/extra/invalid casts fail with frozen diagnostics. Annotation omission differs from explicit none even with equal geometry, preserving value/presence through equality, fields and reconstruction. repr underbrace(body: ..., annotation?: ...) preserves nesting and never exposes the generic MathUnderover/xunder carrier as public identity. Structural walkers descend into body/annotation; carrier is non-locatable and inherits left/right body classes. Internal reduction uses U+23DF below the body and an external lower annotation, reusing the MathUnderover render owner without changing mature geometry. Own tests only, not productive implementation or rendering.

## Limits

Preserve existing public observations and metadata. This documentary revision does not authorize productive-code, test-code, signature, default, pipeline-phase or compatibility changes. Respect exclusive ownership and pinned shared claims under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
