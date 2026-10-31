# L0 — compiler/layout/grid/cell_body
Hash do Código: e2fcc5f8



Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/layout/cell-bidi-scope.toml sha256:5c733e4756935ef5e4841991852a5f60f0f00e2b76c988c6af340b52cc0ce977
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf
- 00_nucleo/prompts/_nuclei/table/declarative-alignment.toml sha256:011aad6ca3199e192d8eb8eecd55d718b4d7ce7097f7987f982cb7a5f12f9a10
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


## Evidence before specification

At HEAD `ea7ae25ab` (clean tree,2026-09-30), implementation is at `01_core/src/compiler/layout/grid/cell_emission.rs:95-583`; this defines extraction boundaries, not performance/language-parity change. `tracks.rs:175`, `cell_body.rs:155` and `lab/results/construction/diagnostics/reuso-sem-transformacao-2026-10-05.md` precede consumable-product reuse.

`lab/results/construction/diagnostics/block-tabela-fluxo-2026-10-06/{focused-candidate-verification,candidate-source}.json.gz` preserves correct width/anchor but wrong ABC/123/Arabic RTL order. `bidi_origin.rs:111–117` refuses every cell/subframe; `cell_body.rs:223–235` rebases Text/TextShaped but not same-line TextSeparator. Remaining wrong order/baseline after limited certification/consistent recomposition refutes the proposed internal parity correction, which adds no public contract/phase/product flag/unsafe.

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC/diffstat/status/sources/binaries/arguments/results. Oracle SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`. Pinned `layout/grid/resolve.rs:204–278,1500–1530,1665–1703,1945–1988` resolves fields/conflicts/padding/breakability. Tekt `grid_placement.rs:91–112,262–270`, `realize/mod.rs:160–200`, `table.rs:127–152` expose explicit-first/Grid-only placement, bottom-up recipes and discarded wrappers. order measures no cell effects under replaced Table/no Heading effects under hidden cell; aliases measures LOCAL a, OUTER LOCAL a, NEW a. Language/effects, not copied mechanics/bytes/Rust equality. Unknown fields/hidden effects/wrong order refute correction; cache/pagination equivalence is not inferred.

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{after-second,refuters-second}.json.gz` preserves full pin/binary/HEAD/state/source/UTC/argv/results: show-body is ab vanilla versus a b Tekt. This measures default inset/alignment survival, not fill/stroke. `table-cell-styles-d08b3c-2026-10-07/physical-before.json.gz` preserves PDF/SVG/full state/hashes at historical pin `9f2b6e8715237cb086899a42873660fe744622e8`: red fill persists with replaced/hidden body in vanilla, missing in Tekt; replaced-body stroke is vanilla red 2pt versus Tekt black 1pt, default black 1pt matches. Pinned `layout/grid/resolve.rs:218–237,270–282` resolves Cell fill/stroke before body show; layouter paints them independently. Earlier ab-based generalization of snapshot survival to paint is refuted and superseded only for paint/breakable.

`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz` records HEAD/working tree/UTC/sources/argv/binaries against historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Existing ShowRecipe/Located/Realized branches contain the corresponding transparent operation; Source must not leak as observable content. Human approval `confirmo` was received 2026-10-07; contract: `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`.

additional-debug3 refutes Source as a barrier to surviving default: recognize snapshot/zero metadata through transparent body without losing inset/align or changing external-plan paint.

The human approved manifest`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` for historical T1–T3; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Bilateral Parent evidence and Solid controls remain in the approved contract; structural inspection does not replace them. `lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz` preserves sources, dirty state, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC, diffstat and binary hashes; it covers containers, Place and Rotate with Solid and Gradient Parent. The before-source/syntax-nodes inventory records this consumer's Group constructions/destructurings before editing: [360]. `layout_types.rs:531–539` does not distinguish roots from technical groups; `helpers.rs:253–269` rebuilds Group and must preserve the field. Box groups background/body; Block leaves body flat except for clipping. Preserving coordinates is a language requirement; envelope form is mechanics. Approval does not close previously measured general Rotate/Box geometry divergences.

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz` records HEAD, working tree, diffstat/UTC and sources/owners before migration. `layout_types.rs:510–511` still transports Paint/Stroke; `shape.rs:151–191` materializes only fill in groups; `tiling.rs:85` limits/clones the grid. `prepare_cell` already returns TilingFrame once per occurrence. Probes `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz` establish that dense-open and stroke-open still diverge in all three targets. The contract was approved in manifest `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Preparation/object counts are transport mechanics; render, contours, text, geometry and diagnostics are observable language obligations.

The owner approved insertion-time resolved Table (historical D08B3-A) on 2026-10-07 (`comfirmo`), archived at `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`: five public fields, two public carriers, one public facade and logical preparation before Cell show. This supersedes its stale proposed-only gate only within the approved scope. Standalone Cell defaults/generation (historical D08B3-B) were separately ratified/materialized (`confirmo`), archived at `lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3`, manifest `3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`. A did not authorize B's added public field/type; B's separate confirmation did. Neither closes broader incomplete scope.

## Exclusive ownership and current contract

Layer L1. Sole consumer: `01_core/src/compiler/layout/grid/cell_body.rs`. Lay out positioned cell with inset/alignment/subframe/decoration/overflow tails. Preserve state save/restore, item order/deferred alignment indices, public contract/default/phase/compatibility/L1 purity/static dispatch.

After computing body_x/body_w/effective alignment, request measured product before installing cell region from `00_nucleo/prompts/core/compiler/layout/grid/measurement_cache.md`. Miss uses prior subframe; hit keeps all later decoration/rebase/clip/overflow/fill/tail paths exactly. Eligibility/lateral metrics belong only to cache owner.

Install/restore private cell-bidi capability only when col_starts.len()==1 and body_w is finite in the cell layout section. Rebase TextSeparator baseline with Text/TextShaped, without advance/height/paint. Cached ASCII keeps its existing proved certificate; never recertify frame through string/position.

Prepared TableCell uses resolved.visual/props, including known None fill/stroke without renewed container inheritance; preserve known alignment/inset/bounds/overflow/decorations/tails/state. Grid/raw fallback retains old or/fold behavior. No recipes; cache reuse requires its owner's updated equivalence proof.

Own pure prepared_default helper recognizing only surviving prepared snapshot through transparent semantic wrappers/Source body/zero metadata, shared by tracks. No text reflection/PartialEq/callback. Only default-derived inset/alignment depend on that snapshot; total replacement has none. Positioned TableCell fill/stroke/breakable come from external TableCellResolved independently of it/body/replacement recipe, and known None defeats fallback. Emit external fill before visual product; borders reads external stroke under its owner. Do not run hidden body/change lexical font/public API/phase/render default/Grid-raw policy. Preserve bounds/coalescing/clip/tails/effect order/causal state.

Migrate Group constructions/patterns to approved paint_root. New technical groups and legacy fixtures use false; reconstructions transport the received value rather than inferring root status from dimensions, clipping or transformation. Preserve other fields and drawing/resource behavior. Exporter pattern use of this metadata is a separate integration boundary, not certified here. Preserve captured paint_root unchanged in Group reconstruction, including projection/marker removal/rebase; never reset true to false. Convert only resolved backing fill Color to FramePaint::Solid, preserving cell plan/body/layout/clip/tails/semantics.

## Acceptance and limits

L1 tests/build/crystalline-lint pass, without inferred gain. Internal same-phase ADR-0127 corrections require L0 before RED→GREEN/restricted resealing. Compare bilateral output/effects-once/actual geometry without whitespace normalization or changed ratified contracts. Paint tests cover substituted/hidden visual, known None and actual visual SVG regression; builds/suites/lint/preservation remain required. Earlier fixed-row alignment/metric residues are not inferred resolved.

Preserve exclusive ownership/static dispatch without I/O or a new signature/trait/flag or eval↔layout phase. Source-origin tests/C2/C/macro collections/builds/lineage are required before certification.

Require RED→GREEN, suites/build/lint/restricted preservation and D08A/B2, D05/D06/D07/query collections. Accept real language/effects/geometry, not byte equality or whitespace normalization to close render. General Table defaults/fields (table-root-equality), rich style functions/paint/strokes/insets, rich lines/gutters, multipage spans and PDF/UA are not certified. Preserve extra legacy D08B2 casts, syntax/flags and global nonlocatable policy.

L0 comes first, with RED→GREEN and restricted resealing. Preserve geometry/text in source→PDF/SVG controls and revalidate suite/build/doc/check/fmt/lineage. This metadata migration neither replaces Shape.fill/stroke with FramePaint nor implements PDF/SVG/raster patterns. Prepared paint transport, native exporter integration and verification (historical T1c/T2/T3) remain explicitly open and already authorized. Build success does not establish tiling parity or production readiness.

Apply the pinned nucleus.Ratification supersedes older clauses awaiting approval or requiring declarative Paint/Stroke inside Shape only within the approved contract. Preserve all other existing behavior. L0 comes first, followed by RED→GREEN; explicitly migrate fixtures/consumers without a total From<Paint> conversion, PartialEq on carriers or a solid fallback merely to obtain a green build. Do not raise the old limit or move layout into L3. Validate fill/stroke transport, resources, clipping, affine Parent, native targets, Solid/Gradient controls, page semantics and the complete suite/build/doc/fmt/lineage. Prepared transport, native target integration and verification (historical T1c/T2/T3) are complete only together; a broken or partial intermediate state does not certify production. The remaining historical T4 scope and publication remain outside this package.


## V16-030 nominal boundary preservation

### Measurement before decision

Source audit at HEAD `a9f03073809fa15ff11d86c532f2c803ee892e47`, UTC `2026-10-19T16:03:25.152301+00:00`;
exact working-tree diffstat and source/L0 SHA-256 are retained in
`lab/results/v16-corrections-2026-10-19/batch-d/agents/grid/phase1-before.json`.
Evidence: 01_core/src/compiler/layout/grid/cell_body.rs:72-139,168-228,261-525; 01_core/src/compiler/layout/grid/cell_emission.rs:87. The owning decision is `Layouter::emit_grid_cell_body`.
Observed: Raw non-cell content yields five absent local overrides and still proceeds through layout_sub_frame/emission. Prepared plan fields bypass container fallback for known None.
This is source-level evidence of existing behavior, not a new current-pin
vanilla comparison or an inference about upstream intent.

### Decision and refutation

Raw content has no outer cell stroke/fill/align/inset/breakable; Grid/raw fallback must inherit while prepared TableCell uses its plan. This classification is not a license to erase wrapper contents.
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


## Approved physical source routing

### Evidence before decision

Exact source/owner bytes, HEAD, dirty-tree state and UTC before these amendments are archived in `lab/results/set-page-header-implementation-2026-10-21/agents/index/grid-original-before.json.gz`. The finite direct-site classification is `lab/results/set-page-header-2026-10-21/agents/parity/descendant-site-classification.json`. Source anchors include `01_core/src/compiler/layout/grid.rs:212-325`, `01_core/src/compiler/layout/grid_placement.rs:232-399`, `01_core/src/compiler/layout/grid/row_groups.rs:22-64`, `01_core/src/compiler/layout/grid/tracks.rs:224,543-561`, `01_core/src/compiler/layout/grid/cell_body.rs:285-346`, and `01_core/src/compiler/layout/table.rs:83-126`. Placement visibly reorders inputs; Auto-height products may be consumed later; Align/Block/prefix constructors wrap particular real source children. These establish transport ownership, not a new geometry/parity claim. The exact public/phase package is ratified in `lab/results/set-page-header-implementation-2026-10-21/approval.json`; no further public contract is introduced here.

### Exclusive routing contract

For actual planned cell paint, do not consume a source-unaware measured product. Retain unplanned measurement and the legacy cache behavior outside the planned path; actual source paint uses the same cell region/geometry and its exact cursor once. The known generated Align wrapper supplies only its declared AlignBody edge path before source admission. Every coordinate/decorative/clip/overflow/tail/state rule remains unchanged.

Preserve legacy unplanned rendering, source parents, explicit aliases, all existing source/geometry/style/diagnostic expectations and the bounded support/debts above. Measurement or generated-only output has no source fact; actual selected source descendants cannot silently become unplanned. Keep the exhaustive static dispatcher and owner-local child decisions. Use nominal plan/edge provenance, never Content/Func equality, text or frame scanning. Deferred physical positions remain separate from causal source anchor/order; preserve the actual source admission through page movement. Missing/conflicting edges fail closed before Complete.

### Acceptance

Require original-index preservation after explicit-first placement, resolved/raw/standalone selected children, header/footer and padding, aligned/inset generated wrappers retaining their source descendants, no phantom measurement effect or duplicate cache admission, source parents/aliases and deferred page anchors. Preserve prior tests and the original native path byte-for-byte in expectation. Actual source-language and unchanged legacy geometry/refusal controls plus full build/suites/strict lint/Clippy/Rustdoc/fmt/forward/reverse lineage are required before closure; no speed or broader Table/PDF-UA parity is inferred.


## Measured nonexpanding Repeat alignment — 2026-10-22

### Evidence before decision

`lab/results/remaining-parity-2026-10-22/repeat-refuters-before/measurement.json.gz`
retains exact source/font/binary/build inputs, UTC2026-10-22T12:02:34.973885+00:00
through12:02:51.345977+00:00, uncommitted HEAD
`cf21f47a3b63f0e9fc5308925e1d0b67925e7227` and its dirty-file stat. Product
SHA-256 `9f696f72294ea255cb5ef499e1882265c52e73b2d91dc1ca0bf76768ea803fcb`
and ratified oracle commit`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, executable
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`
are pinned. The eight-source manifest and full matrix are in the adjacent
`agents/repeat/` evidence. Nested fixed-child center/right and unequal-width
multiline center/right fail; decorated multiline also loses line paint.
Direct Shape, natural text and an explicit inner-left override are GREEN.
Repeat clears inherited alignment before composing its piece; pinned upstream
`typst-layout/src/repeat.rs:19,55–59` preserves lexical styles and separately
places copies. `shape.rs:251–283` emits flow geometry without line alignment,
refuting global translation. Nested-only inheritance does not fix the measured
multiline defect. Language geometry/paint are required; data structures and
iteration mechanics are free under ADR-0107. Classification and pre-edit hashes
are preserved in `agents/repeat/classification-before-L0.json`.

### Private contract

Preserve resolved decoration metadata while translating the already-completed cell product. An independent cell subframe cannot consume an outer Repeat alignment scope. No grid sizing/cache/source contract change.

Require genuine declared-metric RED→GREEN for nested center/right, unequal-width multiline and decorated multiline; preserve direct Shape/natural-text/inner-left controls, source effects once, rollback, complete bilateral PDF/SVG geometry at the unchanged bound, all tests and both lineage directions. No warning suppression, ignored case, threshold relaxation, fake source identity or public/phase expansion.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.


## Approved declarative Table alignment — 2026-10-29

### Measurement and authorization before decision

readiness-fixes-2026-10-28/table-align-before/receipt.json and
 table-align-extra-before/receipt.json under lab/results/construction/diagnostics/
retain the exact physical12/pinned64c5e8c0 binaries, HEAD, dirty input/diffstat,
UTC, source hashes and bilateral geometry/errors/effect order. The signed review
review-table-column-alignment.md SHA256
1e73466f33e416cb14541575124b16244272079ac336e04159ac89ce1bdef66e
was approved by the human as “Aprovar o contrato proposto”; receipt
lab/results/construction/diagnostics/readiness-fixes-2026-10-28/table-column-alignment-approval.json.
This gate covers exactly TableAlignment and TableElem.align plus the explicit
runtime-none rejection and unsupported-function diagnostic. It adds no other
public signature, Grid field, trait, callback, default mode or phase.

### Exclusive amendment

Consume prepared Complete alignment as authoritative. Raw Table fallback folds cell axes over declaration selected at placed.col; legacy Grid keeps uniform fold. Preserve source routes, body admission once, exact region, cache refusal without same-alignment proof, and restoration of parent state.

### Gates and bounded scope

This amendment supersedes only conflicting legacy scalar/None and name-based
Table-set clauses. General Table root reflection including .align remains
unsupported; functions, Grid column arrays, rich paints/insets, multipage spans
and PDF/UA remain negative scope. Require genuine source RED before code, GREEN
for arrays/cycle/auto/presence/aliases/late/local/condition/effects/errors/spans,
header/footer/padding/colspan and hidden/replaced Table/Cell, raw/prepared and
unchanged legacy controls. No relaxation of expected output/tolerance or cache
controls. Rebuild financial original and compare PDF/SVG, words/pages/0.001pt
and independent paint without normalization; compilation does not certify the
whole financial document. Final stable batch requires valid V15/V26, resealed
forward/reverse lineage, full warning-denying lint, twelve gates, historical
1720 flags and both original JPEG resource/timing campaigns under one job,
12GiB/High8GiB/swap0. Keep failures and unrelated readiness debt explicit.


## Aligned cell geometric origin — measured correction 2026-10-29

### Measurement before decision

The original immutable table-align-before receipt and fresh table-selectorless-after1
receipt show the same scalar-left yMin32.051 against native24.769, at11pt Nimbus
Roman:7.282pt extra. This is a pre-existing aligned-cell defect, not a regression
of the approved declaration. The new financial source now compiles but remains
negative in words/0.001pt and independent paint; do not certify it. Exact binaries,
source hashes, HEAD/dirty stat, inputs and UTC are in those immutable receipts.
cell_body.rs:394 installs a legacy ascender outer frame around generated Align;
placement.rs:313 lays its body geometrically then adds the caller cursor_y;
cell_body.rs:475 adds a text top after subtracting only the legacy ascender.
That repeats the top edge for an aligned text leaf. This is a refutable causal
hypothesis, not upstream intent or a requirement to copy frame mechanics.

### Exclusive internal correction and acceptance

For an actual effective alignment only, give the existing generated Align outer
subframe geometric zero, and translate its completed items with body_y+local_y
uniformly. Do not add a second top edge, infer types from glyphs/paint, change
alignment choice, rebuild the body, move eval/layout phases, or add public APIs.
No-alignment keeps the old measured-product/legacy translation path unchanged.
The cache owner already refuses aligned products because cell_align is Some;
retain that proof, all eligibility/refusals and original subframe-call tests.
Preserve source cursor/generated AlignBody edge, decorations, logical orphan
fixups, clipping, finite vertical alignment, region restoration, page anchors,
paint/insets and row-height measurement. Add co-located actual physical RED
showing horizontal alignment cannot add a baseline offset versus the same
unaligned body, before implementation. Test raw/prepared, scalar/columns, Top,
Bottom/Horizon and mixed text/shape/Box/decorated bodies; compare original
financial/native PDF/SVG/0.001pt, unchanged controls, full core and final gates.
Vertical explicit movement remains meaningful and is not normalized away.
This is ADR-0127 continuous formula/coordinate parity repair inside layout.
