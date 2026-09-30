# L0 — stdlib/structural/quote
Hash do Código: c0729e2c

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/quote.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/quote.rs:26` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

quote requires one Content|Str body and named attribution: Content|Str|none=none, block: Bool=false and quotes: Bool=true. Emit Content::Quote preserving all fields. Reject missing/wrong body, wrong named types and unknown names with the current diagnostics. Block/inline rendering, attribution placement and quote drawing remain with layout; this native only transports their fields. No context/World/file reads or new helpers.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.
