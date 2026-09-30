# L0 — stdlib/html/p1423-tests
Hash do Código: 73bb9243

Layer: L1 (test-only)
Exclusive consumer: `01_core/src/compiler/stdlib/html/p1423_tests.rs`

## Evidence and classification

The consumer's `@prompt` at `01_core/src/compiler/stdlib/html/p1423_tests.rs:2` establishes this exclusive ownership. Implementation inspection is not a new bilateral parity measurement; original historical qualifications remain in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Test obligations

Own only the test consumer, not production behavior. From the public html module, verify the exact 43 constructor bindings of this fixture family, disjoint from the earlier 69; the current module includes elem, all 112 typed constructors and frame. Verify the 43 body classes as 10 void, 2 raw and 31 Content, omission/supplied body/type errors, representative global/specific/Presence/range/enum/list/union casts and rejection of crossed attributes. Verify L1 construction independent of target and feature gating on public module access. frame is outside this fixture set but present in the current module; do not require its global absence.

Import the parent only under cfg(test), without inspecting a private candidate constructor, redefining product contracts or claiming CSS/MathML/browser/frame behavior. Expectations derive from `00_nucleo/prompts/compiler/stdlib/html.md`; authorship, mutation catalogs, agent order and frozen implementation hashes are not acceptance.

## Limits

Preserve existing public observations and metadata. This documentary revision does not authorize productive-code, test-code, signature, default, pipeline-phase or compatibility changes. Respect exclusive ownership and pinned shared claims under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
