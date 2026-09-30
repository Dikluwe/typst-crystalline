# L0 — stdlib/structural/markup
Hash do Código: bdfe2355

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/structural/markup.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/structural/markup.rs:28`, `01_core/src/compiler/stdlib/structural/markup.rs:54`, `01_core/src/compiler/stdlib/structural/markup.rs:78` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own strong/emph/raw/link construction and validation, not show matching or layout. strong/emph wrap Content|Str with Bold(true)/Italic(true). raw accepts only Str, optional lang and block, and emits Content::Raw; never coerce Content to raw text. link requires a nonempty Str URL and optional Content body, defaulting the body to the URL text, with no named arguments. Preserve the existing exact diagnostics, lang/block casts/defaults and pure execution without Engine/I/O.

The original requirement that all these natives support no-argument show-selector sentinels remains a contract obligation. Code discrepancy: strong/emph/raw produce empty content on no arguments, but native_link rejects the missing URL at `01_core/src/compiler/stdlib/structural/markup.rs:164`. Do not silently weaken that requirement or change the code during sanitation; function-valued show selection is not proof that no-argument native calls satisfy it.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.
