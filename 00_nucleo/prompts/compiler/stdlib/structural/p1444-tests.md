# L0 — stdlib/structural/p1444-tests
Hash do Código: 65f33e05

Layer: L1 (test-only)
Exclusive consumer: `01_core/src/compiler/stdlib/structural/p1444_tests.rs`

## Evidence and classification

The consumer's `@prompt` at `01_core/src/compiler/stdlib/structural/p1444_tests.rs:2` establishes this exclusive ownership. Implementation inspection is not a new bilateral parity measurement; original historical qualifications remain in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Test obligations

Verify math.primes takes exactly one nonnegative positional Int, including zero; named/missing/extra/incompatible types fail with the frozen primary messages. Content::MathPrimes preserves count, equality/hash/repr/plain_text/.count, no children, reconstruction and nominal identity. Apostrophe syntax has the same carrier; hat/underscore use t/b, directly attached primes tr, with AST nesting. Edge classes Normal; non-locatable and terminal. Counts 1–4 select U+2032/U+2033/U+2034/U+2057; zero and >4 use U+2032, width (count+1)/2 and half-width offsets. Own tests only.

## Limits

Preserve existing public observations and metadata. This documentary revision does not authorize productive-code, test-code, signature, default, pipeline-phase or compatibility changes. Respect exclusive ownership and pinned shared claims under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
