# L0 — stdlib/text/smallcaps
Hash do Código: afc07ca0

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/text/smallcaps.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/text/smallcaps.rs:32` identifies the current ownership boundary. This inspection is not a new bilateral parity measurement or proof of upstream intent. Historical decision evidence retains its original snapshot qualifications in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

smallcaps requires exactly one Content|Str body and no named arguments. Convert Str to Content::text and emit Content::SmallCaps through Content::smallcaps; preserve element identity for show selectors. Missing body: `smallcaps() exige body como argumento posicional`; wrong type: `smallcaps() espera content ou string, recebeu {type}`; extra: `smallcaps() recebeu {n} argumentos posicionais (espera 1)`; named uses the existing façade expect_no_named helper.

Rendering, nested inheritance and scaling/shaping belong to `00_nucleo/prompts/core/compiler/layout/text.md`; this native must not substitute a transparent body or a new style wrapper. No private argument parser, World/context/file reads or global state is introduced.

## Acceptance

Preserve the specified syntax, semantics, morphology, errors/hints/spans, defaults and public identities, with no productive-code change. Preserve pinned shared claims and the exclusive owner boundary under ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130. Historical evidence does not establish parity against a newer snapshot.
