# L0 — stdlib/text/case
Hash do Código: 4382da9f

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/text/case.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/text/case.rs:23`, `01_core/src/compiler/stdlib/text/case.rs:45`, `01_core/src/compiler/stdlib/text/case.rs:69` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

upper/lower require exactly one Str or Content and no named arguments; return the same input kind transformed by Unicode upper/lowercase. Content::map_text changes text nodes while preserving surrounding structure.

replace requires source: Str|Content, pattern: Str, replacement: Str as three positionals and optional count: Int, rejecting other named arguments and empty patterns. Without count replace all occurrences; with count the budget is global across the whole Content traversal, not per text node. A local FnMut counter decreases across nodes; after exhaustion later nodes remain untouched. Preserve exact arity/type diagnostics and Str/Content results. Do not accept Regex patterns in this native, introduce elements, read World/context or use mutable global state.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.
