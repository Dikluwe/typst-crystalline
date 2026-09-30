# L0 — columns
Hash do Código: 850b2f14

```text
```

## Ownership and evidence

Layer: L1. Sole productive consumer: `01_core/src/compiler/layout/columns.rs`.
ADRs: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.
Current source evidence precedes the contracts: layout/geometry at `:310`,
segment splitting/direction at `:192` and `:244`, sibling regions at `:407`,
continuous flow at `:638`, marginal patterns at `:36`, and physical column
operations at `:715`. The producer/transform boundary is
`01_core/src/entities/content/page_structure.rs:181`, not this consumer.

This owner specifies column layout, local geometry and consolidation only.
ColumnsElem and page_columns are existing input data owned by
`00_nucleo/prompts/entities/elements/columns.md`; page-body transformation
belongs to `00_nucleo/prompts/entities/content/page-structure.md`.
Cursor decides when to advance; this consumer owns close_current_column,
merge_column_items, finish_columns, start_next_column and start_column,
with descendant visibility for layout coordination.
Footnote body fitting/overflow is owned exclusively by
`00_nucleo/prompts/compiler/footnote_overflow_columns.md`.

## Geometry and dispatch

Use the existing page margins as sides, not an obsolete scalar assumption:

```text
usable_width = page_width - margins.horizontal()
count = max(input_count, 1)
gutter = explicit_length.resolve_pt(text_size)
         or page_width * COLUMNS_DEFAULT_GUTTER_RATIO
column_width = (usable_width - (count - 1) * gutter) / count
column_region_width = column_width + margins.horizontal()
column_x[i] = margins.left + i * (column_width + gutter)
dx = column_x[i] - margins.left
```

Start each local working region at the left margin. Its mini-page width
includes the margins so the inline right bound reaches the full usable column
width. Translation applies the horizontal delta once; do not narrow text again
or edit layout_word/layout_chunk to compensate.

Split explicit Colbreaks at the inspected body Sequence, looking through one
outer Styled and reapplying that same Styles once to each segment.
Pad missing segments with Empty; fold excess segments into the final one
without dropping content. A detected separator selects segmented layout;
otherwise continuous flow uses the body and the existing column/page transitions.

Direction discovery descends through Sequence/Styled, preferring an inner
direction over its outer wrapper and ignoring wrappers without visible body.
The first relevant child direction wins; absent body direction falls back to
the active chain's text.dir. Only Value::Dir participates.
RTL reverses physical offsets, not item order inside a column; LTR/absence
retain their order. Page sections must arrive with the producer's own
Pagebreak segmentation; this module does not acquire ownership of that transform.

## Vertical boundaries and sibling regions

Participate in the existing auto-Block margin protocol. Flush an open inline
line before establishing pending par.spacing from the previous baseline;
defer initial top-to-baseline conversion until real content arrives.
Collapse adjacent entry/bottom margins by maximum rather than addition.
At exit retain par.spacing for the next block/paragraph.
At a region top do not add a strong margin. Respect active par.spacing,
par.leading and text size rather than historical measured coordinates.

Each explicit segment starts at column_start_y, without adding ascender twice,
and receives the same entry snapshot of logical baseline and collapse state.
A sibling's resulting state never initializes the next sibling.
Only regions with material items contribute the deepest output
baseline-plus-descent; cursor output uses the greatest consumed bottom.
An empty or HTML-carrier-only segment cannot invent a material baseline;
retain its carriers for extraction and preserve behavior when real items exist.
SpaceFromNewline is non-visible like Space without normalizing its origin.

Pass each segment to the dedicated cursor-relative fractional Sequence region.
Fixed content and fr weights are local to that segment; use its current
page_bottom_limit. The single outer style transport does not license a recursive
rewrite of nested Styled content or the use of page_columns as an overflow
sentinel to open fractional layout in arbitrary sublayouts/footnotes.

## Footnote coordination

Numbering comes from the shared introspector, never a saved/restored local
footnote_counter. Segmented page columns (page_columns true) flush pending
notes at the end of the referencing column and translate them with its items.
Segmented explicit containers (false) accumulate their notes and flush once
below the container's deepest consumed content at the first logical column
offset. That offset need not mean the physical left edge in RTL.
Preserve the exterior pending-note buffer when entering/leaving sibling regions.

Continuous flow currently flushes per physical column through
close_current_column, for both forms. Do not turn the old intention to make
explicit continuous containers stack all notes at the end into an implemented
contract. The body flush accepts optional bottom_y; its fitting policy,
warnings and remainder handling are child-owned by the overflow L0.

## Marginal line-number consolidation

Complete exterior pending width fixups before replacing the exterior item
buffer. Drain each column's own fixups before extraction/translation, both in
segmented mode and close_current_column; never resolve a sibling's indexes
against another buffer.

Segmented page columns carry a private index/count for marginal numbering only;
it does not activate fitting/overflow transitions. The last physical column
forces the end margin. Explicit containers instead retain the entry
counter/page snapshot across siblings, suppress the already emitted ordinal
prefix and keep maximum ordinal progress rather than summing siblings.

For string patterns in segmented explicit containers, defer realization and
consolidate siblings jointly in stable ascending baseline order. Accept a
candidate only when its baseline is not below previous_bottom.
Use number height at least 1 pt and advance only accepted candidates according
to the existing formatter/scope handling. Capture common outer TextStyle for
the number; marginal properties remain those of the occurrence's chain.
Candidate buffers participate in layout rollback and are drained on exit.

For page columns, consolidate each physical column separately before translation:
reset its vertical suppression boundary, not the document counter/scope.
The first available candidate captures the shared immutable TextStyle for that
composition. In continuous close_current_column, if the first physical column
closes without a candidate, capture active TextStyle once before translating
and advancing; a later candidate must not replace it.
Explicit containers keep joint sibling consolidation instead.
Restore private contexts/buffers at exit without additional callbacks.

## Acceptance and evidence limits

Verify full usable width, asymmetric margins, segmented/continuous transitions,
LTR/RTL offsets, independent page-section direction, both segmented footnote
policies, unchanged note numbering, and warning transport via the overflow owner.
Verify auto-Block boundaries, sibling logical origins, empty/material output
state, independent fr weights and HTML carrier transport.
Check region-local width fixups, shared marginal style, stable baseline
suppression, last-column margin and counter scope/progress.

These are language/layout observables, not fixed page totals for a particular
font or proof from PDF bytes. No global formatting, new public API or new
default is authorized. Historical scope-outs are not evidence of current
absence. Nested wrappers/columns, multi-region fractional cases, heterogeneous
callbacks, local par.line divergence and RTL marginal-numbering parity remain
outside demonstrated coverage; do not claim them complete or universally
unsupported. Producer support for mid-document page columns and page gutter
must be audited separately, as recorded in
`00_nucleo/diagnosticos/sanitization-introspection-columns-history.md`.

## Continuidade lexical do tracking — 2026-09-29

### Medição anterior à decisão

`00_nucleo/diagnosticos/correcao-tracking-escopos-2026-09-29/before-pdf/results.json` e `summary.json` registram 30 fontes/90 PDFs, comandos, fontes e estado exato em initial.json e before-pdf/provenance.json, contra vanilla pin 9f2b6e8715237cb086899a42873660fe744622e8. Escopos iguais repetidos acumulam cerca de 0.4pt e aninhados 0.8pt; wrap positivo muda a linha. `text_tracking.rs:50` compara apenas TextStyle resolvido. Vanilla `inline/collect.rs:287-295` conserva segmentos de cadeias distintas; `inline/shaping.rs:1269-1295` soma tracking apenas dentro de cada run. Isto mede fronteiras textuais observáveis, não exige copiar PartialEq ou representação da chain. Hide/Box também são fronteiras: hide.rs:21-24 e boxed.rs:157-208 substituem/restauram buffers sem preservar essa distinção.

### Contrato interno da correção

Na composição segmentada de cada coluna irmã, salvar State de tracking junto à linha exterior e restaurá-lo com essa linha. Não deixar proveniência textual da irmã contaminar o prefixo exterior; demais estados/counters/widths permanecem.

Correção interna de paridade ADR-0127 em fluxo contínuo. Aceitação RED→GREEN, controles bilaterais de texto/posições/linhas e preservação dos controles de tracking uniforme; build e lint sem erros, resselo somente destes owners. Demais limites abertos mantidos.
