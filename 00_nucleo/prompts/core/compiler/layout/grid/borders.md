# L0 — compiler/layout/grid/borders
Hash do Código: 53d31e74


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


## Evidence before specification

Internal extraction was measured at HEAD `ce27af795`, clean tree,2026-09-30 23:12:44 UTC. The logic was at `01_core/src/compiler/layout/grid.rs:1262-1499`; this establishes existing ownership, not new bilateral parity or performance.

At historical vanilla pin 9f2b6e871, `lab/typst-original/crates/typst-layout/src/grid/layouter.rs:568-578,779-791` shifts start by minus half thickness and extends segment length by one thickness. `borders.rs:74-114` ended at geometric boundaries. Rasterization in `lab/results/construction/diagnostics/block-tabela-fluxo-2026-10-06.md` shows missing corner paint in six tables. Historical intent was not established.

`table-cell-styles-d08b3c-2026-10-07/show-fold-before.json.gz` compares two Cell/Table show-set declarations: vanilla combines red/2pt while Tekt loses red. geometry-final.json.gz/cell-stroke-two-sets/cell-stroke-constructor preserve PDFs/argv/binaries/HEAD/working tree/UTC; pdftocairo SVG shows Tekt black 1pt versus vanilla red 2pt. `borders.rs:43–48` reads lexical e.stroke; `apply.rs:244,465` collapses cell.stroke top-wins. Vanilla `model/table.rs:765` #[fold] and `layout/grid/resolve.rs:225–237` resolve components; typst-layout grid/layouter.rs:568–578 draws resolved stroke. Color/thickness and language fields are observable, without SVG/PDF byte equality or draw-command-count requirements.

`table-cell-styles-d08b3c-2026-10-07/physical-before.json.gz` preserves PDF/SVG/sources/HEAD/working tree/UTC/argv/full hashes against historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. fill-with-body-recipe/fill-with-hidden-recipe keep vanilla red rectangles absent in Tekt. stroke-with-body-recipe keeps vanilla red 2pt versus Tekt black 1pt; default-with-body-recipe is black 1pt in both. Vanilla `layout/grid/resolve.rs:218–237,270–282` returns Cell.fill/stroke before show processing of body, and grid layouter draws them independently of default show. Tekt `cell_body.rs:61–63` conditions fill on surviving snapshot while borders reads lexical stroke. Earlier refinement measured spacing/ab, not paint: generalizing snapshot survival to fill/stroke is refuted and superseded only in this scope.

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz` records HEAD, working tree, diffstat/UTC and sources/owners before migration. `layout_types.rs:510–511` still transports Paint/Stroke; `shape.rs:151–191` materializes only fill in groups; `tiling.rs:85` limits/clones the grid. `prepare_cell` already returns TilingFrame once per occurrence. Probes `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz` establish that dense-open and stroke-open still diverge in all three targets. The contract was approved in manifest `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Preparation/object counts are transport mechanics; render, contours, text, geometry and diagnostics are observable language obligations.

## Exclusive ownership and current contract

Layer L1. Sole consumer: `01_core/src/compiler/layout/grid/borders.rs`. Own cell bounds/effective strokes, horizontal/vertical segment emission, open-vertical flush and row-border coalescing. Preserve stroke priority, geometry, item order and pagination boundaries. Do not perform placement/cell layout or change public contract, pipeline phase, default behavior or compatibility.

Correct visible border morphology: every positive final segment starts half its thickness before the boundary and ends half after it on its own axis. Shape dimensions reflect extended length; perpendicular axis, stroke/fill and order remain. Apply only at final emission after coalescing, never accumulate extension in open-segment state. Empty segments emit nothing. This internal ADR-0127 correction introduces no API/default/phase/unsafe.

For prepared TableCell, consume known external TableCellResolved stroke, including known None, independently of recipe product. No lexical/container fallback; Grid/raw Table retain historical priority. Preserve endpoint geometry/coalescing. Positioned TableCell's external plan owns fill/stroke/breakable independent of it/body/replacement recipes; known None overrides fallback. Only inset/alignment derived from default show still depend on the surviving snapshot recognized by prepared_default. This owner's stroke read is independent of snapshot. `00_nucleo/prompts/core/compiler/layout/grid/cell_body.md` owns external fill before visual product; existing measurement/geometry owners retain their scope. Do not execute hidden content or change lexical font, public API, phase, render default or Grid/raw policy. Keep bounds/coalescing, clipping/tails, effect order and causal state.

Cross-owner stroke-fold boundary: pure TableCellProperties::chain_styles combines only declarative Value::Stroke in cell.stroke, retaining explicit None and other keys/topology/order/guards (`00_nucleo/prompts/core/entities/elements/table_cell.md`). `00_nucleo/prompts/core/compiler/eval/rules/apply.md` uses it in both existing show-set accumulation routes; realizer local deltas remain under `00_nucleo/prompts/core/compiler/realize/table_cells.md`. Do not store Cell/plan in Styles. These are boundaries, not extra productive consumers of this owner.

Keep declarative Stroke while choosing/coalescing borders. Only after closing a positive segment, prepare stroke through the tiling helper with Layouter and emit FrameStroke. Private emit/flush signatures receive Layouter rather than only a buffer, preserving coordinates/order/priority decisions. Do not lay out table-cell body; delegated preparation covers paint only. Fixtures check geometry/fields without requiring FrameStroke PartialEq.

## Acceptance and limits

RED→GREEN endpoint tests cover distinct horizontal/vertical thicknesses and preserved empty suppression; bilateral paint has no corner gap or text/position regression. Stroke folds require real SVG colors/thickness in physical units and default/None/replacement controls; paint requires substituted-product/known-None RED→GREEN and actual visual SVG regression. Builds/suites/lint/preservation remain required. Fixed-row alignment and earlier metric residues are not inferred solved.

Apply the pinned nucleus. Ratification supersedes older clauses awaiting approval or requiring declarative Paint/Stroke inside Shape only within the approved contract. Preserve all other existing behavior. L0 comes first, followed by RED→GREEN; explicitly migrate fixtures/consumers without a total From<Paint> conversion, PartialEq on carriers or a solid fallback merely to obtain a green build. Do not raise the old limit or move layout into L3. Validate fill/stroke transport, resources, clipping, affine Parent, native targets, Solid/Gradient controls, page semantics and the complete suite/build/doc/fmt/lineage. Prepared transport, native target integration and verification (historical T1c/T2/T3) are complete only together; a broken or partial intermediate state does not certify production. The remaining historical T4 scope and publication remain outside this package.


## V16-029 nominal boundary preservation

### Measurement before decision

Source audit at HEAD `a9f03073809fa15ff11d86c532f2c803ee892e47`, UTC `2026-10-19T16:03:25.152301+00:00`;
exact working-tree diffstat and source/L0 SHA-256 are retained in
`lab/results/v16-corrections-2026-10-19/batch-d/agents/grid/phase1-before.json`.
Evidence: 01_core/src/compiler/layout/grid/borders.rs:43-51; 01_core/src/compiler/layout/grid/borders.rs:191,239-241. The owning decision is `cell_effective_stroke`.
Observed: Non-cell means no cell-specific stroke, then grid_stroke is inherited; prepared TableCell returns resolved.stroke directly, including known None.
This is source-level evidence of existing behavior, not a new current-pin
vanilla comparison or an inference about upstream intent.

### Decision and refutation

All non-cell bodies may occupy a raw grid cell but have no outer cell override. They are not skipped from rendering. TableCell resolved/unresolved branches are both covered; the guard cannot fall into raw inheritance.
Maintain an explicit exhaustive nominal partition at this decision, including
guard-false residuals in their owning variant. Future variants must require
a fresh classification at this site. No wildcard relocation, syntax bypass,
suppression, new conversion, or public signature/default/phase change.
This is continuous-flow internal maintenance under ADR-0127.
Rendering, text, effects, placement and diagnostics are language obligations;
compiler exhaustiveness is an internal maintenance proof under ADR-0107.
Preservation controls must pass before and after the edit; structural RED→GREEN
requires an attributable E0004 at this match after adding a sentinel only in a
disposable copy of `Content`. Another match's error is insufficient.
Any changed accepted/rejected case, lost content, repeated effect, fallback
geometry or diagnostic refutes preservation. Broader parity limitations in
this owner remain open.

The current nominal complement (no outer cell-specific fields) is: `Source`, `ShowRecipe`, `Realized`, `Located`, `Empty`, `Text`, `Space`, `SpaceFromNewline`, `Parbreak`, `Sequence`, `HtmlElem`, `HtmlFrame`, `Par`, `Heading`, `Title`, `Strong`, `Emph`, `Raw`, `ListItem`, `EnumItem`, `Link`, `Equation`, `MathSequence`, `MathIdent`, `MathText`, `MathFrac`, `MathAttach`, `MathPrimes`, `MathRoot`, `MathStretch`, `MathDelimited`, `MathLr`, `MathMid`, `MathOverbrace`, `MathUnderbrace`, `MathUnderbracket`, `MathUnderparen`, `MathUndershell`, `MathOverbracket`, `MathOverparen`, `MathOvershell`, `MathAlignPoint`, `Linebreak`, `MathMatrix`, `MathCases`, `MathAccent`, `MathCancel`, `MathUnderline`, `MathOverline`, `MathVec`, `MathClassOverride`, `MathLimitsOverride`, `MathUnderover`, `MathOp`, `MathStyled`, `Label`, `Ref`, `CounterDisplay`, `CounterUpdate`, `Outline`, `OutlineEntry`, `Figure`, `FigureCaption`, `Image`, `Shape`, `Curve`, `Transform`, `Grid`, `GridHeader`, `GridFooter`, `SetPage`, `PageRun`, `Align`, `Place`, `Flush`, `Styled`, `Divider`, `Terms`, `TermItem`, `Quote`, `Document`, `Asset`, `SmartQuote`, `PdfAttach`, `PdfArtifact`, `Underline`, `Strike`, `Overline`, `SmallCaps`, `Pad`, `Hide`, `HSpace`, `VSpace`, `Pagebreak`, `Colbreak`, `Stack`, `Boxed`, `Block`, `Bibliography`, `Cite`, `Footnote`, `FootnoteEntry`, `TableHeader`, `TableFooter`, `GridHLine`, `GridVLine`, `TableHLine`, `TableVLine`, `Table`, `Repeat`, `Columns`, `Metadata`, `State`, `StateUpdate`, `StateDisplay`, `CounterDisplayCallback`, `ContextBlock`, `Dynamic`.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.


## R5 — pintura compartilhada e coordenadas de linha (2026-10-24)

### Medição anterior à decisão

`lab/results/construction/diagnostics/painting-r5-2026-10-24/baseline.json` e
`baseline-source.json.gz` preservam HEAD cf21f47a3b63f0e9fc5308925e1d0b67925e7227,
árvore não commitada, lista exata/diffstat, UTC, fontes/L0 e binários. O oráculo
continua bd5ebec285505889e8ff1d4f68804ab147a699f3, árvore
cac836c07e4dec2a89219370428292393a3216b5, SHA256
9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9.
`before/receipt.json` mede tabela: 18/10 pixels distintos a72dpi e1548/1532
a600dpi; os outros seis documentos coincidem a72dpi. `pdf-experiments/receipt.json`
isola deduplicação de linhas idênticas e transformação local: só ambas zeram
a diferença nas duas resoluções. Isso sustenta correção de pintura observável,
não igualdade de bytes/comandos. `borders.rs:332-360` emite fundo e topo
coincidentes de linhas adjacentes; `row_pagination.rs:26-32` contém a decisão
de quebra; `cell_emission.rs:124-190` conhece alturas e ordem física.
`draw.rs:483-495,914-932` emite endpoints absolutos arredondados e inverte
inclinação no espaço local já refletido. `before-local/receipt.json` registra
linha inclinada em Box recortado:183 pixels distintos a72dpi e5511 a600dpi.
Esses são efeitos de linguagem/render; intenção upstream não foi inferida.

### Classificação e aceitação

Correção interna de paridade, fluxo contínuo ADR-0127: nenhum campo, enum, trait,
API pública, default de produto, fase ou quebra de compatibilidade novo.
Atualizar L0 antes de testes/código, V15/V26 antes de reseal bidirecional,
RED funcional antes da implementação, GREEN sem enfraquecer expectativas.
Exigir PDF rasterizado bilateral a72/600dpi e SVG físico, transparência, fills,
None/zero, spans, gaps, strokes distintos e bordas em quebras de página;
linhas inclinadas/negativas/fracionárias/top/local e modos Compact/Verbose.
Preservar texto, geometria, efeitos, ordem dos corpos e demais gates A–H/R1–R4,
suítes/build/Clippy/doc/fmt/lint zero, uma tarefa pesada por vez,12GiB/zero swap.
Não certificar prioridades gerais de strokes distintos, gutters ricos, spans
multipágina ou paridade global por essa amostra. PNG nativo e nomes públicos
de fontes são superfícies separadas; nenhuma renomeação/normalização de
comparadores é autorizada para aparentar MATCH.

### Contrato exclusivo desta unidade

Suprimir um run horizontal de fundo somente se houver run de topo da próxima
linha com os mesmos endpoints e Stroke declarativo completo, sem quebra física
de página entre eles. Manter o topo posterior, depois do fill posterior.
Construir runs candidatos apenas das células que começam na próxima linha,
com a mesma fusão/contiguidade existente; não atravessar spans/gaps nem
fundir sobreposições parciais ou strokes diferentes. O caller fornece prova
da permanência na página; sem prova, preservar ambos. Comparar descrições
antes de preparar FrameStroke; não varrer/deduplicar FrameItem, não adicionar
PartialEq ao transporte. Extensão por meia espessura só na emissão final,
verticais e prioridades distintas preservadas. Célula conhecida None continua
None. Testes verificam cobertura/alpha de uma fronteira e refutadores.
