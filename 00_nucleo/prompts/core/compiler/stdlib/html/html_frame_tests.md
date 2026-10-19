# L0 — core/compiler/stdlib/html/html_frame_tests
Hash do Código: 731d3eb2

Layer: L1 (test-only)
Exclusive consumer: `01_core/src/compiler/stdlib/html/html_frame_tests.rs`

## Evidence and classification

The consumer's `@prompt` at `01_core/src/compiler/stdlib/html/html_frame_tests.rs:2` establishes this exclusive ownership. Implementation inspection is not a new bilateral parity measurement; original historical qualifications remain in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Test obligations

Own only tests of public html.frame, not productive code or intrinsic entrypoint selection. Observe frame as a function with repr frame, separate from tag constructors; exactly one required positional Content and the three missing/named/extra-positional diagnostics. Verify Content::HtmlFrame(HtmlFrameElem { body }), distinct from HtmlElem, raw body, empty output or SVG bytes. plain_text/map_text/map_content must traverse body and preserve the variant. Simple/nested frames in paged layout remain observationally equal to bare body in supported pages/geometry/items/positions. Do not claim HTML/SVG export verification or adapt expectations to candidate implementation; fixture changes require public-contract justification, not authorship/seals/agent order.

## Limits

Preserve existing public observations and metadata. This documentary revision does not authorize productive-code, test-code, signature, default, pipeline-phase or compatibility changes. Respect exclusive ownership and pinned shared claims under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
