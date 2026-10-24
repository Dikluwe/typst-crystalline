# L0 — compiler/layout/grid/cell_emission
Hash do Código: 1a3aa394

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:2e0595153eaf379004fd35fe6a3887a7d62532bec78700833e8b8569e672137e
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

## Evidence before specification

Original behavior is at `01_core/src/compiler/layout/grid.rs:276-882`, HEAD `6fc546566`, clean tree,2026-10-01 01:04:22 UTC. This is extraction, not new parity/performance evidence. `tracks.rs:175`, `cell_body.rs:155` and `lab/results/construction/diagnostics/reuso-sem-transformacao-2026-10-05.md` precede private pool forwarding.

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC/diffstat/status/sources/binaries/arguments/results. Oracle SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`. Pinned `layout/grid/resolve.rs:204–278,1500–1530,1665–1703,1945–1988` resolves fields/conflicts/padding/breakability. Tekt `grid_placement.rs:91–112,262–270`, `realize/mod.rs:160–200`, `table.rs:127–152` expose explicit-first/Grid-only placement, bottom-up recipes and discarded wrappers. order measures no cell effects under replaced Table/no Heading effects under hidden cell; aliases measures LOCAL a, OUTER LOCAL a, NEW a. Language/effects, not copied mechanics/bytes/Rust equality. Unknown fields/hidden effects/wrong order refute correction; cache/pagination equivalence is not inferred.

Selected Clippy sites `cell_emission.rs:49,50` are preserved with source/owners/references/binary/UTC/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`/uncommitted tree/status/diffstat and checked spans in `lab/results/construction/diagnostics/iteracao-parametros-ci-2026-10-13/before.json.gz`; mechanics/uses establish no new bilateral parity or performance gain.

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/before/results.json.gz`records the same HEAD, uncommitted tree/diffstat/UTC/sources/binaries/argv and historical pin. All 24 root/Box pairs after 12 prefixes differ: Linebreak/Parbreak require y22.25pt, Shape y27pt, Block/Grid/VSpace20pt y35pt, Tekt 15pt. Rotate also overlaps independently. `line_commit.rs:721–733` prepares next baseline; `shape.rs:196–206` adds below/cap; `block.rs:465` separates boundary/margin; `place.rs:91–99` recognizes still-pending line only. Pinned flow/distribute.rs:703–715 uses physical flow offset for axis-unspecified Place. Weak margins/overlap/new page/rollback/subframe leakage can refute the causal-end hypothesis.

`block-auto-width-2026-10-14/before/results.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC/diffstat/status/sources/binaries/argv and historical pin 9f2b6e871. The initial extractor sought blue as #0000ff; blue-geometries-corrected.json reads actual #0074d9 from the same SVGs without rerun. For Shape, Empty, Place-only, Grid and Stack, Tekt backing width is 160 versus pin54,14,14,68,68 respectively. `block.rs:222,416,453` uses available width; `line_commit.rs:753` loses cursor on closure; `sub_frame.rs:354` saves terminal cursor only. Pinned `flow/block.rs:145–166,281–290,339–363` distinguishes Auto from expansion. Geometry is language; witness structure is internal mechanics. Place contaminating width, leaking scopes, lost literal advances or changed explicit-container dimensions refute the hypothesis.

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz` records HEAD, working tree, diffstat/UTC and sources/owners before migration. `layout_types.rs:510–511` still transports Paint/Stroke; `shape.rs:151–191` materializes only fill in groups; `tiling.rs:85` limits/clones the grid. `prepare_cell` already returns TilingFrame once per occurrence. Probes `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz` establish that dense-open and stroke-open still diverge in all three targets. The contract was approved in manifest `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Preparation/object counts are transport mechanics; render, contours, text, geometry and diagnostics are observable language obligations.

The owner approved insertion-time resolved Table (historical D08B3-A) on 2026-10-07 (`comfirmo`), archived at `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`: five public fields, two public carriers, one public facade and logical preparation before Cell show. This supersedes its stale proposed-only gate only within the approved scope. Standalone Cell defaults/generation (historical D08B3-B) were separately ratified/materialized (`confirmo`), archived at `lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3`, manifest `3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`. A did not authorize B's added public field/type; B's separate confirmation did. Neither closes broader incomplete scope.

## Exclusive ownership and contract

Layer L1. Sole consumer: `01_core/src/compiler/layout/grid/cell_emission.rs`. Emit placed Grid/Table cells row by row with page breaks, inherited align/inset, subframes, overflow tails, fill/stroke order and border flush. Return emitted row starts for explicit visual rules. Preserve Layouter restoration, item order, path fixups, bounded-region support, public contract/phase/default/compatibility/L1 purity/static dispatch.

Forward consumable tracks pool to each cell_body without changing pagination/order. Define neither key nor eligibility here; `00_nucleo/prompts/core/compiler/layout/grid/measurement_cache.md` owns invariants.

Emit plan origin cells at plan row/column in appropriate order without duplicate merged slots. Physical header/footer repeat prepared visual rather than recipe; preserve pagination/tails/stroke/fill/ranges without new execution/context.

Fill grid_owner via mutable slot references, retaining explicit row/column bounds and origin skip: origin beyond bounds produces empty iteration, never invalid slice. Compute identical endpoints with identical arithmetic, not saturating addition. Each cell assigns Some(i) at same slots/order, including later-write precedence on overlap; no border/pagination/content change.

After each completed row/cursor advance, record row_start_y+row_h as committed-flow end, never import cell-body local boundary. Publish horizontal extent of positioned tracks during row emission, distinct from inner paint/stroke/cell ink.

Forward self to private border/flush helpers, preserving every emission time, declarative coalescing field and row/page geometry.

## Acceptance and incomplete scope

L1 suite/build/crystalline-lint pass, with no inferred speed gain. Equivalent internal ADR-0127 iteration preserves expectations, public contract/default/phase and does not warrant tests mirroring loops or new suppressions. Validate core conflicts/spans/fixups/markers/differential selection, every target/build/docs/Clippy/lineage with restricted reseal.

Require RED→GREEN, suites/build/lint/restricted preservation and D08A/B2, D05/D06/D07/query collections. Accept real language/effects/geometry, not byte equality or whitespace normalization to close render. General Table defaults/fields (table-root-equality), rich style functions/paint/strokes/insets, rich lines/gutters, multipage spans and PDF/UA are not certified. Preserve extra legacy D08B2 casts, syntax/flags and global nonlocatable policy.

Causal-end correction authorizes only necessary private state, superseding extraction-only no-state restrictions to that extent; LIFO save/restore is mandatory. L0 first/RED→GREEN/reseal without public field/flag/default/phase change; revalidate full HTML/context/PDF/SVG/build/doc/fmt/check/lineage/warning inventory. Certified line/shape/Block/Align/Grid/Raw/VSpace anchors do not close Block/inset/fragmentation or late inline Box/Transform and Heading/List/Stack/Equation/Columns producers. Invalid witness stays legacy fallback/pending, never parity credit; broad Place/tiling production scope remains separately gated.

This internal parity correction under ADR-0127 adds no public contract/default/phase. L0 precedes RED→GREEN/restricted resealing. Causal width is only part of the ongoing block-region/width/transport correction: Auto height/backing, late alignment, fractions, fragmentation, outset and PDF Parent retain their own criteria. Testing a carrier alone does not approve render. Preserve full bilateral controls/refuters, suite/build/doc/fmt/lineage/inventories.

Apply the pinned nucleus.Ratification supersedes older clauses awaiting approval or requiring declarative Paint/Stroke inside Shape only within the approved contract. Preserve all other existing behavior. L0 comes first, followed by RED→GREEN; explicitly migrate fixtures/consumers without a total From<Paint> conversion, PartialEq on carriers or a solid fallback merely to obtain a green build. Do not raise the old limit or move layout into L3. Validate fill/stroke transport, resources, clipping, affine Parent, native targets, Solid/Gradient controls, page semantics and the complete suite/build/doc/fmt/lineage. Prepared transport, native target integration and verification (historical T1c/T2/T3) are complete only together; a broken or partial intermediate state does not certify production. The remaining historical T4 scope and publication remain outside this package.


## Approved physical source routing

### Evidence before decision

Exact source/owner bytes, HEAD, dirty-tree state and UTC before these amendments are archived in `lab/results/set-page-header-implementation-2026-10-21/agents/index/grid-original-before.json.gz`. The finite direct-site classification is `lab/results/set-page-header-2026-10-21/agents/parity/descendant-site-classification.json`. Source anchors include `01_core/src/compiler/layout/grid.rs:212-325`, `01_core/src/compiler/layout/grid_placement.rs:232-399`, `01_core/src/compiler/layout/grid/row_groups.rs:22-64`, `01_core/src/compiler/layout/grid/tracks.rs:224,543-561`, `01_core/src/compiler/layout/grid/cell_body.rs:285-346`, and `01_core/src/compiler/layout/table.rs:83-126`. Placement visibly reorders inputs; Auto-height products may be consumed later; Align/Block/prefix constructors wrap particular real source children. These establish transport ownership, not a new geometry/parity claim. The exact public/phase package is ratified in `lab/results/set-page-header-implementation-2026-10-21/approval.json`; no further public contract is introduced here.

### Exclusive routing contract

Transport the already-proved optional source cursor for each placed result into the existing actual cell paint in the same row/index order. Generated holes have None. Retain all pagination, border, semantic, overflow and deferred transport; neither reconstruct nor deduplicate source facts from cell frame items.

Preserve legacy unplanned rendering, source parents, explicit aliases, all existing source/geometry/style/diagnostic expectations and the bounded support/debts above. Measurement or generated-only output has no source fact; actual selected source descendants cannot silently become unplanned. Keep the exhaustive static dispatcher and owner-local child decisions. Use nominal plan/edge provenance, never Content/Func equality, text or frame scanning. Deferred physical positions remain separate from causal source anchor/order; preserve the actual source admission through page movement. Missing/conflicting edges fail closed before Complete.

### Acceptance

Require original-index preservation after explicit-first placement, resolved/raw/standalone selected children, header/footer and padding, aligned/inset generated wrappers retaining their source descendants, no phantom measurement effect or duplicate cache admission, source parents/aliases and deferred page anchors. Preserve prior tests and the original native path byte-for-byte in expectation. Actual source-language and unchanged legacy geometry/refusal controls plus full build/suites/strict lint/Clippy/Rustdoc/fmt/forward/reverse lineage are required before closure; no speed or broader Table/PDF-UA parity is inferred.


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

Após emissão de corpos da linha, consultar alturas já resolvidas e predicado
do owner row_pagination com row_start_y+row_h e altura seguinte. Informar ao
owner borders se existe próxima linha na mesma página. Não antecipar layout
de corpos/efeitos nem alterar paginação, cursor, tracks, rotas ou tails.
