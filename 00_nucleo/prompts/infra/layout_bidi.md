# L0 — Visual bidirectional line ordering
Hash do Código: f5d9128f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

**Layer:** L3
**Exclusive consumer:** `03_infra/src/layout_bidi.rs`

## Current evidence

- `03_infra/src/layout_bidi.rs:23`, `:39`, `:49`, `:128` — metrics, document/page/line passes.
- `03_infra/src/layout_bidi.rs:183`, `:305`, `:334`, `:409`, `:485` — visual ordering, space coalescing and suffix separation.
- `03_infra/src/layout_bidi.rs:646`, `:782` — controls preserving L1 wrapping.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Preserve `reorder_bidi_document(doc: PagedDocument, metrics: &dyn FontMetrics) -> PagedDocument`. Own horizontal visual ordering of top-level page items before shaping. The paged pipeline owner places this pass; paragraph direction and line breaking remain owned by L1 layout.

Cluster page items by baseline with `Y_TOLERANCE_PT=0.01`. Semantic envelopes retain their existing first-child coordinate convention. Detect an RTL base paragraph from concatenated plain Text with Unicode bidi auto direction; an LTR-base line remains unchanged. Reordering operates on plain Text only. Nontext items retain their geometry even when vector ordering changes.

For an RTL line, use `FontMetrics::text_width` for word widths and `line_content_right` for the occupied right edge. Compute nonnegative inter-item gap from occupied width minus total word widths, divided by the number of intervals. Resolve visual runs over item text ranges; explicit RTL text style supplies the RTL default level inside this ordering helper. Preserve text/style pairing, unmapped-item handling, empty-paragraph fallback and left-to-right positioning from the existing `x_min`.

Sort the line's items by increasing x. A standalone whitespace item can be coalesced as one trailing space into the preceding nonspace Text in visual order; retain the existing empty-item placeholder behavior rather than remove entries while indices are in use. Separate a terminal LTR suffix from an RTL base using L/EN/AN classes and existing ASCII punctuation handling. Preserve original trailing whitespace on the base, keep its style/y, and place the separate suffix at `x_base - suffix_width`. Pure-suffix text is not split.

L1 baselines are authoritative: never merge lines, recompute wrapping, infer paragraph boundaries from distance, estimate width from byte length or infer the region width from emitted ink. `ExplicitLinebreakBoundary` and `ParbreakBoundary` remain transparent carriers; no cross-baseline fusion consults them. Preserve page dimensions, pagination, leading and nontext positions. This pass neither supplies vertical writing nor guarantees an extractor-specific logical text order.

`00_nucleo/prompts/infra/shaper/bidi.md` owns character-level shaping runs. This owner does not duplicate its run data or shaping policies.

## Verification boundary

Existing tests cover Latin identity, Arabic/mixed visual order, real widths, whitespace and suffixes, empty text, nontext geometry and preserved wrapping/explicit breaks. Sequential extraction must not justify changing glyph order inside the shaper.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/layout_bidi.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Transportar coordenadas do carrier sem coalescer sua origem em Text. Após reordenação de texto RTL, posicionar separadores no intervalo visual do slot precedente; não inventar carriers para gaps.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Preservar o envelope opaco nas operações que coalescem/splitam folhas Text; x/y vêm dos filhos. Não incluir texto substituto no shaper/reflow ou inferir novas quebras. Não certificar RTL completo nesta rodada.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.
