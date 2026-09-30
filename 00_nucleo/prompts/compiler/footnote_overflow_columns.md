# L0 — footnote_overflow_columns
Hash do Código: 16b22afa

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

```text
```

## Ownership and evidence

Layer: L1. Sole productive consumer:
`01_core/src/compiler/layout/footnote_flush.rs` (lineage header at `:2`).
The flush body at `:40`, safe boundary at `:91`, greedy fitting at `:128`
and placement at `:188` implement the current behavior.
`01_core/src/compiler/layout/mod.rs:2187` forwards warning strings;
`01_core/src/entities/layout_types.rs:989` declares the existing document
warning carrier; `03_infra/src/pipeline/paged.rs:439` converts it to diagnostics.
Those files are evidence of integration, not additional consumers owned here.

Column placement/translation is specified in
`00_nucleo/prompts/compiler/columns.md`. Layouter state, document fields and
pipeline warning conversion retain their own L0s; this prompt does not authorize
new fields or changes to those modules.

## Flush and bounded overflow

Preserve flush_pending_footnote_bodies(bottom_y: Option<f64>).
It consumes pending numbered bodies, measures their marker plus body in a local
sub-frame of the current usable page/column width, and emits fitting bodies in
order. bottom_y optionally supplies the container's footnote-area bottom;
otherwise use the region height minus bottom margin.

Compute top_safe as the maximum of top margin, current cursor_y, the current
positioned-item Y values and the open line's item Y values, using the existing
FrameItem projection (including Semantic children).
available_h is max(area_bot - top_safe, 0).
This measured projection is not a general bounding-box/ink-extent proof.

Fit greedily by cumulative measured height. At the first non-fitting body,
defer it and subsequent bodies in order for the caller's next flush.
Force the first body only when its own measured height exceeds the full usable
page/column height, not merely the remaining current space.
This defensive escape prevents non-progress on a body that cannot fit anywhere.

Place accepted bodies top-down from
max(area_bot - accumulated_height, top_safe), compensating the sub-frame's
ascender in physical item translation. Supply the actual left origin to the
sub-frame and do not add it again as a full horizontal translation.
Retain decoration and orphan alignment-fixup rebasing, distinguishing logical
origins from ascender-adjusted physical baselines. Preserve causal numbering,
remainder order and pending-note coordination by the caller.

## Warning and purity contract

On forced emission, append the existing pure string, substituting the number:

```text
footnote body [N] exceeds available column/page height and may be truncated or overlap content
```

L1 writes no Sink, file, environment or clock. Existing Layouter warnings move
to PagedDocument.layout_warnings; the paged pipeline drains them into its
returned warnings vector as SourceDiagnostic::warning with detached span.
This consumer owns warning production, not the carrier declaration or conversion.

The user must be able to recover the note or be warned of placement failure;
mere presence of bytes in a PDF is not success. A fitting/deferred note must
not silently disappear. The forced oversized fallback explicitly permits
truncation/overlap with a warning: do not promise impossible no-overlap or
vanilla-style note splitting/continuation for that case.

## Acceptance and boundaries

Verify empty queues, notes that fit, deferral when only remaining space is too
small, inclusion of unflushed line/cursor state in top_safe, oversized-body
termination and exact warning production, optional bottom_y, and single-origin
translation with nested placement/alignment fixups.
Integration checks cover preserved warning forwarding and existing small-note
column placement. No new source edits are implied by these checks.

The old quarantined-vanilla observation did not establish that current vanilla
lacks page(columns:). Its incomplete provenance is historical, not an exception
to the active parity target or a new default. Relevant history and outstanding
evidence limits are in
`00_nucleo/diagnosticos/sanitization-introspection-columns-history.md`.

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`01_core/src/compiler/layout/footnote_flush.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

Transportar posição, avanço já consumido, estilo e origem do TextSeparator sem criar tinta, baseline ou novo avanço; manter ordem dos filhos. Excluir separador do limite vertical seguro.

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

Limite seguro consulta filhos visuais; a tradução preserva text e desloca filhos pelo delta do posicionamento, sem avanço novo.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.
