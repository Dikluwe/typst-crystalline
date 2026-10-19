# L0 — compiler/layout/grid/tracks
Hash do Código: 152caa93

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf


## Evidence before specification

Original behavior is at `01_core/src/compiler/layout/grid.rs:212-451`, HEAD `51a027f99`, clean tree,2026-10-01 00:44:23 UTC. This establishes extraction, not new bilateral parity/performance. Historical full measurements and qualifications remain archived in `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/layout/grid/tracks.md`.

`tracks.rs:175`, `cell_body.rs:155` and `lab/results/construction/diagnostics/reuso-sem-transformacao-2026-10-05.md` precede measured-product reuse: Auto-height measurement discards a product emission rebuilds. `lab/results/construction/diagnostics/block-tabela-fluxo-2026-10-06/before.json` and espaco-terminal-2026-10-06/verification.json.gz preserve state/source/L0/binaries and six tables whose inline sequences break despite one vanilla line at historical pin 9f2b6e871. `measurement.rs:104-113` uses max-width/summed-height for inline sequence, `block.rs:315` excludes Literal separators from width, `tracks.rs:108-113` uses left margin even for a single RTL Auto column. Pinned grid/layouter.rs:967-1100 measures full frame; page_geometry.rs:143-154 separates Literal-auto extent from ink. Wrong remaining wraps/anchors or weak-space/fixed-column/style/line/paragraph regressions refute the hypothesis. `lab/results/construction/diagnostics/bordas-cantos-2026-10-06.md` exploratory Align controls expose Left-RTL one-column origin; `tracks.rs:104-116` limits offset to one column and uses page margin in a subregion.

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC/diffstat/status/sources/binaries/arguments/results. Oracle SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`. Pinned `layout/grid/resolve.rs:204–278,1500–1530,1665–1703,1945–1988` resolves fields/conflicts/padding/breakability. Tekt `grid_placement.rs:91–112,262–270`, `realize/mod.rs:160–200`, `table.rs:127–152` expose explicit-first/Grid-only placement, bottom-up recipes and discarded wrappers. order measures no cell effects under replaced Table/no Heading effects under hidden cell; aliases measures LOCAL a, OUTER LOCAL a, NEW a. Language/effects, not copied mechanics/bytes/Rust equality. Unknown fields/hidden effects/wrong order refute correction; cache/pagination equivalence is not inferred.

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{after-second,refuters-second,after-fourth,refuters-fourth}.json.gz`, nested-reuse-red.log and effect-order-red.json preserve full pin/binary/HEAD/state/source/UTC/argv/results. show-bare puts CELL/a on separate lines; measurement lacks TableCell at`:374`, and`:21–31` does not certify ShowRecipe in inline Sequence. aliases/show-cell-no-default-escape refute incomplete projection. geometry-fifth.json/local-bool-red.log (binary 59da863f with source/state/argv/glyph/tests) expose SECOND/d and LOCAL/a breaks: `break_selection.rs:24` strict absolute-coordinate comparison can diverge by last bit after transport. `sub_frame.rs:326` zeros terminal width for Table. width-refuter.json (binaryed82be14/tested-seventh/source/argv/UTC) puts OUTER LOCAL/a on Auto lines but one line at explicit 79.167pt; outer-alias-visual.json shows OUTER/space/LOCAL/space/a. Isolated measurement at `measurement.rs:75–92` has no proved equivalence for this transported composite flow.

`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz` records HEAD/working tree/UTC/sources/argv/binaries against historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Existing ShowRecipe/Located/Realized branches contain the corresponding transparent operation; Source must not leak as observable content. Human approval `confirmo` was received 2026-10-07; contract: `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07/contract.md`.

additional-debug3 puts FUNC/styled on separate lines: Source is not evidence a composite product is atomic. `lab/results/construction/diagnostics/performance-c2-2026-10-07/table-local-regression.json` and three samples/r*-table-local/samples.json preserve source/argv/UTC/state/releaseac889581 hash and historical vanilla pin: size grows 10%, second column x60.880→38.999 overlaps. `lab/results/construction/diagnostics/table-local-c2r1-2026-10-07/red.log` has failed 10pt/Auto separation with explicit 11pt passing. Pinned typst-eval/src/code.rs:32–43 applies local set declarations only; Tekt expression_dispatch.rs:132 collapses root defaults, style_chain.rs:304–315 roots size11/bold false/italic false, tracks.rs:328 misses standalone Complete Cell. Joint style/projection hypothesis is refuted by remaining source/size/position divergence after local-style isolation/full-visual measurement, without requiring upstream graph/mechanics.

Type-complexity C013 at `tracks.rs:225`: `lab/results/construction/diagnostics/type-complexity-2026-10-15/before.json.gz` preserves HEAD/uncommitted tree/UTC/diffstat/source/owners/hashes; red-clippy.log fails before editing. Positional tuples/repeated FontKey alias/static contracts were distinguished. This is representation maintenance, not AI benefit/functional bug/new parity.

The owner approved insertion-time resolved Table (historical D08B3-A) on 2026-10-07 (`comfirmo`), archived at `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`: five public fields, two public carriers, one public facade and logical preparation before Cell show. This supersedes its stale proposed-only gate only within the approved scope. Standalone Cell defaults/generation (historical D08B3-B) were separately ratified/materialized (`confirmo`), archived at `lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3`, manifest `3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`. A did not authorize B's added public field/type; B's separate confirmation did. Neither closes broader incomplete scope.

## Exclusive ownership and current contract

Layer L1. Sole productive consumer: `01_core/src/compiler/layout/grid/tracks.rs`. Own column/row track resolution, keeping Fixed/Auto/Fraction order, constrained cell measurements/insets, page break before fractional-row allocation, auto-height finite fallback and current cursor/style effects. Return widths/starts/heights without changing placement/emission, public contract/phase/default/compatibility/L1 purity.

Return private measurement_cache pool and transfer complete discarded Auto-height product into it after using height; `00_nucleo/prompts/core/compiler/layout/grid/measurement_cache.md` exclusively decides eligibility. Fixed/Fraction/geometric formulas remain. No extra cache policy here.

In finite single-column regions without horizontal override, anchor at lexical chain's End RTL using available width minus resolved-width sum, without changing widths/logical order. Explicit Left/Right/Center/Start/End resolves against outer direction for total column width in any column count, using finite region and current line_start_x. Without override keep single-column refinement and legacy multicolumn policy; mirroring/multicolumn spans need separate measurement. `00_nucleo/prompts/core/compiler/layout/placement.md` owns lexical override install/restore; no duplicated translation in cell emitter. Origins/insets/rules/borders consume existing col_starts. Auto delegates constrained measurement capable of inline Sequence; Fixed/Fraction/cache/pagination/phase stay.

For prepared input measure Complete visual at plan's correct column/row/colspan/rowspan, never chunk raw source cells as a grid or measure replaced raw body. Preserve track types/metrics/styles/formulas/insets/cache and legacy Grid order; no recipe resolution. Prepared visual uses surviving-default render properties through `00_nucleo/prompts/core/compiler/layout/grid/cell_body.md`; total replacement has zero inset, and known lexical Cell is not mutated.

measurement_visual projects every prepared TableCell with known stage, positioned or standalone, through e.visual(); preserve Styled/Source around text. Raw unstaged Cell retains fallback. For measurement recursively remove only Cell snapshots and ShowRecipe wrappers, keeping Styled and complete nested Table boundary. Never traverse/extract nested cells/coordinates; complete nested Table remains atomic and uses already-realized physical subframe without Func. Source/body/zero-metadata traversal in composed_visual discovers actual composite material, not fake atomicity.

A prepared visual with more than one inline material item is measured through a subframe in the causal Layouter instance, with full body, actual bounds and existing restoration. Metadata/Empty do not make it composite. Simple Text keeps static measurement/proven cache; nested Table and physical Bool Raw require physical fallback rather than zero static branch. When terminal subframe width is zero after Table, use real FrameItem bounds through metrics::line_content_right without losing bodies. Never infer width from getter text or abandon subframe-count controls.

Only positive finite Auto widths for prepared plans receive representation-error bound EPSILON × (available width + origin + natural width). This compensates binary transport rounding, not an editorial margin; preserve exact text and legacy Grid. Same projection serves Auto width and rows; external-plan inset/align, paint, placement and cache remain unchanged. No second recipe/callback, hidden-source emission or phase/public/flag change.

Private RowCell names cell_index, borrowed content, column, colspan, rowspan. Keep Vec<Vec<RowCell>> with Clone/Copy of same values/references; construct/destructure by name. Preserve partition/order/spans/bounds/heights/measured-product capture; never clone Content or alter cache/placement.

## Acceptance and limits

L1 suite/build/crystalline-lint pass; extraction proves no performance gain. Internal parity/maintenance uses continuous-flow ADR-0127 with no new public API/trait/entity/flag/default/phase. L0 before RED→GREEN/restricted resealing, build/suites/lint and physical/timing recolection. No intent inference or assumed cache/complex-cell parity.

Require RED→GREEN, suites/build/lint/restricted preservation and D08A/B2, D05/D06/D07/query collections. Accept real language/effects/geometry, not byte equality or whitespace normalization to close render. General Table defaults/fields (table-root-equality), rich style functions/paint/strokes/insets, rich lines/gutters, multipage spans and PDF/UA are not certified. Preserve extra legacy D08B2 casts, syntax/flags and global nonlocatable policy.

Projection/rounding/composite refinements require bilateral geometry/glyph/line/effect-once controls, cache preservation and all original refusals. No whitespace normalization; remeasure after every refuter before certification. Local-style correction covers separated nonoverlapping columns,10/11pt text, local aliases, physical PDF measurements and C/C2 corpus, without mutating realized-plan geometry.

Preserve exclusive ownership/static dispatch without I/O or a new signature/trait/flag or eval↔layout phase. Source-origin tests/C2/C/macro collections/builds/lineage are required before certification.

Type-complexity correction preserves resolved public signatures/default/phase. An alias is allowed only with proved identical expansion; public/new-incompatible types need another gate. Nominal type_complexity exceptions apply only specifically justified items in their package, not module/impl/crate/global threshold or other required corrections. Use unchanged functional expected outputs; selected lint RED→GREEN includes unfulfilled_lint_expectations as error. Revalidate build/Clippy all-targets/full workspace suite/fmt/Rustdoc/lineage/restricted reseal without tests mirroring field names/aliases. One-column and explicit-override refinements do not certify legacy multicolumn mirroring/spans, broader rich cells or global parity.


## V16-033 nominal boundary preservation

### Measurement before decision

Source audit at HEAD `a9f03073809fa15ff11d86c532f2c803ee892e47`, UTC `2026-10-19T16:03:25.152301+00:00`;
exact working-tree diffstat and source/L0 SHA-256 are retained in
`lab/results/v16-corrections-2026-10-19/batch-d/agents/grid/phase1-before.json`.
Evidence: 01_core/src/compiler/layout/grid/tracks.rs:77-97; 01_core/src/compiler/layout/grid/cell_body.rs:18-45. The owning decision is `Layouter::resolve_grid_columns`.
Observed: Non-cell inset None selects grid inset; a prepared TableCell with no surviving default uses zero inset rather than inherited grid inset.
This is source-level evidence of existing behavior, not a new current-pin
vanilla comparison or an inference about upstream intent.

### Decision and refutation

The complement has no outer cell-specific inset. Nested content still participates in measurement. The physical plan controls placement; do not invent metadata from the visual body or reexecute recipes.
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

## V16-034 nominal boundary preservation

### Measurement before decision

Source audit at HEAD `a9f03073809fa15ff11d86c532f2c803ee892e47`, UTC `2026-10-19T16:03:25.152301+00:00`;
exact working-tree diffstat and source/L0 SHA-256 are retained in
`lab/results/v16-corrections-2026-10-19/batch-d/agents/grid/phase1-before.json`.
Evidence: 01_core/src/compiler/layout/grid/tracks.rs:288-308; 01_core/src/compiler/layout/grid/cell_body.rs:18-45. The owning decision is `Layouter::resolve_grid_rows`.
Observed: Non-cell inset None selects grid inset; a prepared TableCell with no surviving default uses zero inset rather than inherited grid inset.
This is source-level evidence of existing behavior, not a new current-pin
vanilla comparison or an inference about upstream intent.

### Decision and refutation

The complement has no outer cell-specific inset. Nested content still participates in measurement. The physical plan controls placement; do not invent metadata from the visual body or reexecute recipes.
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

## V16-035 nominal boundary preservation

### Measurement before decision

Source audit at HEAD `a9f03073809fa15ff11d86c532f2c803ee892e47`, UTC `2026-10-19T16:03:25.152301+00:00`;
exact working-tree diffstat and source/L0 SHA-256 are retained in
`lab/results/v16-corrections-2026-10-19/batch-d/agents/grid/phase1-before.json`.
Evidence: 01_core/src/compiler/layout/grid/tracks.rs:461-491; 01_core/src/entities/content/traversal.rs:678-695. The owning decision is `measurement_visual`.
Observed: None means no replacement now: map_content_pre continues the ordinary recursive walk; it does not drop the node. Table returns itself atomically; staged TableCell projects visual, ShowRecipe projects body.
This is source-level evidence of existing behavior, not a new current-pin
vanilla comparison or an inference about upstream intent.

### Decision and refutation

Other variants must keep their existing reconstruction/traversal, especially Styled/Source and unstaged cells. Table is a deliberate recursion barrier so nested cells/coords are not extracted. A nominal no-replacement class preserves traversal, not zero measurement.
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

The current nominal complement (no replacement, continue traversal) is: `Source`, `Realized`, `Located`, `Empty`, `Text`, `Space`, `SpaceFromNewline`, `Parbreak`, `Sequence`, `HtmlElem`, `HtmlFrame`, `Par`, `Heading`, `Title`, `Strong`, `Emph`, `Raw`, `ListItem`, `EnumItem`, `Link`, `Equation`, `MathSequence`, `MathIdent`, `MathText`, `MathFrac`, `MathAttach`, `MathPrimes`, `MathRoot`, `MathStretch`, `MathDelimited`, `MathLr`, `MathMid`, `MathOverbrace`, `MathUnderbrace`, `MathUnderbracket`, `MathUnderparen`, `MathUndershell`, `MathOverbracket`, `MathOverparen`, `MathOvershell`, `MathAlignPoint`, `Linebreak`, `MathMatrix`, `MathCases`, `MathAccent`, `MathCancel`, `MathUnderline`, `MathOverline`, `MathVec`, `MathClassOverride`, `MathLimitsOverride`, `MathUnderover`, `MathOp`, `MathStyled`, `Label`, `Ref`, `CounterDisplay`, `CounterUpdate`, `Outline`, `OutlineEntry`, `Figure`, `FigureCaption`, `Image`, `Shape`, `Curve`, `Transform`, `Grid`, `GridHeader`, `GridFooter`, `GridCell`, `SetPage`, `PageRun`, `Align`, `Place`, `Flush`, `Styled`, `Divider`, `Terms`, `TermItem`, `Quote`, `Document`, `Asset`, `SmartQuote`, `PdfAttach`, `PdfArtifact`, `Underline`, `Strike`, `Overline`, `SmallCaps`, `Pad`, `Hide`, `HSpace`, `VSpace`, `Pagebreak`, `Colbreak`, `Stack`, `Boxed`, `Block`, `Bibliography`, `Cite`, `Footnote`, `FootnoteEntry`, `TableHeader`, `TableFooter`, `GridHLine`, `GridVLine`, `TableHLine`, `TableVLine`, `Repeat`, `Columns`, `Metadata`, `State`, `StateUpdate`, `StateDisplay`, `CounterDisplayCallback`, `ContextBlock`, `Dynamic`.
An unstaged `TableCell` continues traversal; keep this residual inside its own arm.
