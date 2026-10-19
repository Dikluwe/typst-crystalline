# L0 — stdlib/text/shift
Hash do Código: 2b4f560d

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/text/shift.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/text/shift.rs:28`, `01_core/src/compiler/stdlib/text/shift.rs:89` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

sub and super require one Content|Str body and optional size: Length. Convert Str to Content::text; emit sub_with_size/superscript_with_size with their existing styles and optional size. Preserve the language name super even though the Rust native uses superscript. Missing body: `{fn}() exige body como argumento posicional`; wrong body: `{fn}() espera content ou string, recebeu {type}`; extra: `{fn}() recebeu {n} argumentos posicionais (espera 1)`; wrong size: `{fn}(size:) espera length, recebeu {type}`; other named: `{fn}() argumento nomeado inesperado '{name}'`.

Preserve the paired validation/surface/transport; configurable offset remains unsupported. Layout owns size fallback and baseline displacement under `00_nucleo/prompts/core/compiler/layout/text.md`. Do not move rendering here, factor the paired bodies as part of this documentary revision, read World/context or change phase ownership.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.
