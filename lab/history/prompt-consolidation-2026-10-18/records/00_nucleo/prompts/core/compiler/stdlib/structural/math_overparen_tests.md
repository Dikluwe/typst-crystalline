# L0 — core/compiler/stdlib/structural/math_overparen_tests
Hash do Código: ed3c1310

Layer: L1 (test-only)
Exclusive consumer: `01_core/src/compiler/stdlib/structural/math_overparen_tests.rs`

## Evidence and classification

The consumer's `@prompt` at `01_core/src/compiler/stdlib/structural/math_overparen_tests.rs:2` establishes this exclusive ownership. Implementation inspection is not a new bilateral parity measurement; original historical qualifications remain in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Test obligations

Verify math.overparen and the bare route converge to Content::MathOverparen. Require a positional content-convertible body and only one optional second positional annotation; named/missing/extra/invalid casts fail with frozen diagnostics. Annotation omission differs from explicit none even with equal geometry, preserving value/presence through equality, fields and reconstruction. repr overparen(body: ..., annotation?: ...) preserves nesting and never exposes the generic MathUnderover/xover carrier as public identity. Structural walkers descend into body/annotation; carrier is non-locatable and inherits left/right body classes. Internal reduction uses U+23DC above the body and an external upper annotation, reusing the MathUnderover render owner without changing mature geometry. Own tests only, not productive implementation or rendering.

## Limits

Preserve existing public observations and metadata. This documentary revision does not authorize productive-code, test-code, signature, default, pipeline-phase or compatibility changes. Respect exclusive ownership and pinned shared claims under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
