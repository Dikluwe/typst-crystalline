# L0 — grid
Hash do Código: 14a74d0a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/coordinates.toml sha256:3921c80230cfcc66a807429fbf8bc07ee090bae4988c351b6524023038bab84b
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/grid.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/grid.rs:125`, `01_core/src/compiler/layout/grid.rs:154`, `01_core/src/compiler/layout/grid.rs:536`, `01_core/src/compiler/layout/grid.rs:30`, `01_core/src/compiler/layout/grid.rs:31`, `01_core/src/compiler/layout/grid.rs:32`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

`tracks.rs:175`, `cell_body.rs:155` and `lab/results/construction/diagnostics/reuso-sem-transformacao-2026-10-05.md` precede measured-product reuse.

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC/diffstat/status/sources/binaries/arguments/results. Oracle SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`. Pinned `layout/grid/resolve.rs:204–278,1500–1530,1665–1703,1945–1988` resolves fields/conflicts/padding/breakability. Tekt `grid_placement.rs:91–112,262–270`, `realize/mod.rs:160–200`, `table.rs:127–152` expose explicit-first/Grid-only placement, bottom-up recipes and discarded wrappers. order measures no cell effects under replaced Table/no Heading effects under hidden cell; aliases measures LOCAL a, OUTER LOCAL a, NEW a. Language/effects, not copied mechanics/bytes/Rust equality. Unknown fields/hidden effects/wrong order refute correction; cache/pagination equivalence is not inferred.

block-auto-width-2026-10-14/green-first.log preserves integrated RED: inset7 Block with 20+30 tracks/gutter4 gives 64 width versus 68 in pinned before/grid-vanilla.svg. `grid.rs:171` receives _gutter but`:217` reserves/translates none; previous 160 backing width hid it. before-grid-owner preserves source/L0/HEAD/diffstat/UTC.

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz` records HEAD, working tree, diffstat/UTC and sources/owners before migration. `layout_types.rs:510–511` still transports Paint/Stroke; `shape.rs:151–191` materializes only fill in groups; `tiling.rs:85` limits/clones the grid. `prepare_cell` already returns TilingFrame once per occurrence. Probes `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz` establish that dense-open and stroke-open still diverge in all three targets. The contract was approved in manifest `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Preparation/object counts are transport mechanics; render, contours, text, geometry and diagnostics are observable language obligations.

Arity C014: `grid.rs:164` layout_grid has 13 Clippy arguments against threshold 7, captured at HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree,2026-10-15T16:39:18.873477+00:00 with full source/prior L0/signatures/hashes/status/diffstat in `lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz` and classification.json. Grid/Table repeat twelve positional fields for the same visual engine; a cohesive private named request is justified by those uses rather than a functional-defect claim from arity alone.

`lab/results/construction/diagnostics/clippy-aridade-final-2026-10-16/before.json.gz` preserves source/L0/HEAD/dirty tree/UTC/status/diffstat/global hashes; callers.txt includes fixtures. Same-named math::MathLayouter calls are another unchanged engine. Prior inventory has 22 warnings/five corresponding arities; this measures private Rust structure, not parity/upstream intent.

The owner approved insertion-time resolved Table (historical D08B3-A) on 2026-10-07 (`comfirmo`), archived at `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`: five public fields, two public carriers, one public facade and logical preparation before Cell show. This supersedes its stale proposed-only gate only within the approved scope. Standalone Cell defaults/generation (historical D08B3-B) were separately ratified/materialized (`confirmo`), archived at `lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3`, manifest `3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`. A did not authorize B's added public field/type; B's separate confirmation did. Neither closes broader incomplete scope.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own Grid dispatch and the shared visual grid/table engine. Delegate cell-row emission and pagination to `00_nucleo/prompts/core/compiler/layout/grid/cell_emission.md`. Delegate column/row track sizing and its pagination point to `00_nucleo/prompts/core/compiler/layout/grid/tracks.md`. Delegate header/footer normalization and collision detection to `00_nucleo/prompts/core/compiler/layout/grid/row_groups.md`, and visual h/v rules to `00_nucleo/prompts/core/compiler/layout/grid/rules.md`. Delegate row-border coalescing and h/v segment emission to `00_nucleo/prompts/core/compiler/layout/grid/borders.md`, retaining their invocation order and geometry. Preserve track resolution (fixed/auto/fractional), gutters, per-cell overrides and span measurement, row-group extraction/repetition, h/v rules, fill/stroke and row-border coalescing. Obtain occupied placement from `00_nucleo/prompts/core/compiler/layout/grid_placement.md`; no second placement algorithm is defined in the hub or cell wrappers.

Measure cell content with injected metrics/styles in the existing finite column widths, apply per-axis inherited alignment and inset, and lay out each cell once in its absolute cell origin/region. Recompose by incremental deltas, keeping parent bbox, cell alignment/context, decorations and orphan fixup paths consistent and restored. Preserve breakable-row splitting, repeated header/footer visual output and deferred cell tails within the source's bounded support; `00_nucleo/prompts/core/compiler/layout/slicing.md` performs partition/rebase and `00_nucleo/prompts/core/compiler/layout/cell_tail_commit.md` later emits tails. Table semantic metadata belongs to `00_nucleo/prompts/core/compiler/layout/table.md`; do not derive it from visual strokes/fills. Unsupported multipage/span/container combinations remain limited rather than silently promoted to parity.

Coordinate the invocation-local pool returned by tracks and forward it to emission; declare measurement_cache child, whose rules belong to `00_nucleo/prompts/core/compiler/layout/grid/measurement_cache.md`. No Layouter cache or legacy Grid placement-before-measurement change.

Keep one physical engine, with a private prepared-Table entry using already-known plan positions before measurement. This exception supersedes historical order prohibition only for prepared Table; legacy Grid still places after measurement. Delegate every occupancy decision to `00_nucleo/prompts/core/compiler/layout/grid_placement.md`, never a hub duplicate. Known properties/roles follow plan, while tracks/fonts/pagination stay physical; no Engine/Func or second measurement to obtain show.

Resolve scalar horizontal gutter Length in current style, reserve(n-1)*gap before track resolution and add i*gap to each column origin. Track widths exclude gap; spans use existing actual origins. No gap before first/after last column. Fractions receive postreservation remainder. This repairs only existing scalar horizontal axis, with no new public field/default/phase.

Forward Layouter to private rule emitter to prepare paint before Shape creation; preserve public traits, declarative Stroke comparison, tracks/cells.

Materialize C014 through borrowed GridLayoutInput, pub(super) for Grid/Table only, with twelve named fields columns, rows, cells, hlines, vlines, gutter, align, inset, header, footer, stroke, fill and unchanged types/H/V generics. layout_grid receives self/input, destructures once and keeps engine body. Grid/Table project exact fields, without deep Content copy, extra placement owner/algorithm or public export. This supersedes only the earlier pending materialization for the nominal arity scope.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Require RED→GREEN, suites/build/lint/restricted preservation and D08A/B2, D05/D06/D07/query collections. Accept real language/effects/geometry, not byte equality or whitespace normalization to close render. General Table defaults/fields (table-root-equality), rich style functions/paint/strokes/insets, rich lines/gutters, multipage spans and PDF/UA are not certified. Preserve extra legacy D08B2 casts, syntax/flags and global nonlocatable policy.

Horizontal gutter requires unchanged-expectation RED→GREEN plus fixed/fractional-track controls/exact-state provenance. Vertical/rich gutters, rules between gutters and fragmented spans remain incomplete for Grid integration, not certified by a one-row case.

Arity maintenance follows continuous-flow ADR-0127 without public/default/phase/compatibility change. L0 first/restricted reseal, selected too_many_arguments RED→GREEN without suppression/threshold change, with prior expectations fulfilled. Verify named field order/value transport/body, unchanged track/gutter/header/footer/span/style/Table semantics and no deep copies. Initial audit-only pending restrictions are superseded only by concrete nominal materialization; other required corrections stay pending. Private signature changes migrate callers/their L0 before code; newly required public/external compatibility needs a concrete ADR-0127 gate, not anticipatory permission. Preserve layers/static dispatch/no global mutable state, fixtures/tolerances and functional expectations without carrier-mirror tests.

Run existing Grid/Table/outline/SVG/tiling before/after, full workspace suite/build/Clippy all-targets/Rustdoc/fmt/strict-reverse lineage/Tekt no new warnings under 12GiB/zero swap/one job. No global parity/PDF-UA claim.

Apply the pinned nucleus.Ratification supersedes older clauses awaiting approval or requiring declarative Paint/Stroke inside Shape only within the approved contract. Preserve all other existing behavior. L0 comes first, followed by RED→GREEN; explicitly migrate fixtures/consumers without a total From<Paint> conversion, PartialEq on carriers or a solid fallback merely to obtain a green build. Do not raise the old limit or move layout into L3. Validate fill/stroke transport, resources, clipping, affine Parent, native targets, Solid/Gradient controls, page semantics and the complete suite/build/doc/fmt/lineage. Prepared transport, native target integration and verification (historical T1c/T2/T3) are complete only together; a broken or partial intermediate state does not certify production. The remaining historical T4 scope and publication remain outside this package.


## Approved physical source routing

### Evidence before decision

Exact source/owner bytes, HEAD, dirty-tree state and UTC before these amendments are archived in `lab/results/set-page-header-implementation-2026-10-21/agents/index/grid-original-before.json.gz`. The finite direct-site classification is `lab/results/set-page-header-2026-10-21/agents/parity/descendant-site-classification.json`. Source anchors include `01_core/src/compiler/layout/grid.rs:212-325`, `01_core/src/compiler/layout/grid_placement.rs:232-399`, `01_core/src/compiler/layout/grid/row_groups.rs:22-64`, `01_core/src/compiler/layout/grid/tracks.rs:224,543-561`, `01_core/src/compiler/layout/grid/cell_body.rs:285-346`, and `01_core/src/compiler/layout/table.rs:83-126`. Placement visibly reorders inputs; Auto-height products may be consumed later; Align/Block/prefix constructors wrap particular real source children. These establish transport ownership, not a new geometry/parity claim. The exact public/phase package is ratified in `lab/results/set-page-header-implementation-2026-10-21/approval.json`; no further public contract is introduced here.

### Exclusive routing contract

Carry a private source-cursor sidecar alongside the existing combined and placed cells. The index comes from exact original typed GridCell or Table resolved/raw child edges and the row-group owner's explicit extraction, never body equality, pointer comparison or coordinates. Preserve the existing GridLayoutInput/public PlacedCell types and occupied placement policy. Header/footer padding is generated with no source cursor; known source wrapper/Sequence spines may be omitted only when their complete nominal root fact is empty. Physical emission, not measurement, owns source admission.

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

Manter delegação de paginação/emissão/bordas nos owners existentes. Migrar
apenas chamadas privadas de testes para fornecer ausência de próxima linha
quando o fixture tem uma única linha. Não adicionar algoritmo no hub.
