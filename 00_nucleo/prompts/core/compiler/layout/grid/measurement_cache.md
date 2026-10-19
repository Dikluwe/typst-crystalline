# L0 — grid/measurement_cache
Hash do Código: 110ebc9c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/grid/measurement_cache.rs`
**Architecture:** ADR-0107/0108/0109/0127/0129/0130.

## Evidence before specification

`grid/tracks.rs:175` builds an Auto-height subframe then discards items; `grid/cell_body.rs:155` rebuilds them at emission; `grid.rs:218–239` measures before placement. `lab/results/construction/diagnostics/reuso-sem-transformacao-2026-10-05.md` measures context differences and historical cache removal after reordering. Earlier initial-layout work removed a complete relayout, not this measure/emit pair. Baseline/current source: `lab/results/construction/diagnostics/frames-celulas-2026-10-05/before-state.json.gz`. `paragraph_plan.rs:48–56` rejects subframes; `line_commit.rs:493` consumes no root counter there; `word_layout.rs:21–55` emits a kept word without callback/identity; `sub_frame.rs:314–346` restores parent and returns local products plus last_sub_frame_width/bottom. This motivates a limited experiment, not assumed gain.

`frames-celulas-2026-10-05/width-probe.json.gz` preserves exact state/cfg(test) probe for Beta/FixedMetrics: last_sub_frame_width 26.400000000000006 versus body_w 26.4, absolute cursor-origin cancellation caused strict rejection. Original discovery log stays exploratory; only eligibility tolerance is justified, never geometry adjustment.

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC/diffstat/status/sources/binaries/arguments/results. Oracle SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`. Pinned `layout/grid/resolve.rs:204–278,1500–1530,1665–1703,1945–1988` resolves fields/conflicts/padding/breakability. Tekt `grid_placement.rs:91–112,262–270`, `realize/mod.rs:160–200`, `table.rs:127–152` expose explicit-first/Grid-only placement, bottom-up recipes and discarded wrappers. order measures no cell effects under replaced Table/no Heading effects under hidden cell; aliases measures LOCAL a, OUTER LOCAL a, NEW a. Language/effects, not copied mechanics/bytes/Rust equality. Unknown fields/hidden effects/wrong order refute correction; cache/pagination equivalence is not inferred.

`lab/results/construction/diagnostics/celulas-prioridade5-2026-10-07/before.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/diffstat/code/binaryfc5b2571. audit.log/audit-test.rs.txt show Alpha/12.34/item-2/abc_def/a/b each yield one identical Text leaf; old alphanumeric predicate alone rejects punctuation. CJK yields more leaves, spaces yield three items. `word_layout.rs:21–66`, `word_segment.rs:28–45`, `sub_frame.rs:314–346`, `paragraph_plan.rs:48–64` locate Keep/Wrap/Hyphenate/segmentation/restoration/subframe refusal. This is execution mechanics/cost with language geometry/effects/text obligations, not new global parity/intent.

Initial ASCII candidate ca7b2257, release `f2eea959438e8b4a626abd4d04578ca667089ab537d8f390bf2eae63204662b9`, retained 640 subframes for literal/interpolated cells including alphanumeric control (count-before-source.json.gz). carrier.log finds Source(Text("item-0.25")); inspect.log records enabled 320-entry pool/single-leaf product. These receipts belong to pre-Source final-state.json.gz, not final-candidate proof. `mod.rs:1158` directly delegates Source body with no style/identity/bidi/context; `entities/content/source.rs:11-12` removes only Source and pinned source-origin requires transparency/single occurrence. Unchanged count after classification extension would refute wrapper-blocking hypothesis.

`lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/stacks-current.json.gz` records exact HEAD/dirty tree/diffstat/status/UTC/sources/binary/interposer. Across 1,600 Cell, inclusive style_scope::layout_styled allocation samples are 5,435 local-set versus 1,471 block-no-set; FontBook::select_pattern 1,597 versus 477, not exclusive time. `measurement_cache.rs:60-90` rejects all Styled despite some prepared wrappers carrying only cell.* and eval.table.local.*; `style_scope.rs:24-61` pushes that delta without changing leaf TextStyle for those names. No hits/gain after extension refutes lost-product hypothesis; visual/effect preservation must be measured on 23 fixtures, not presumed from wrapper shape.

V17 T134/T135/T136: `measurement_cache.rs:69,80,83`. Sources/L0 read first, hash-reconfirmed 28-guard/21-owner inventory and full SHA/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`/uncommitted tree/UTC/status/diffstat are in `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`. Pinned CI requires splitting/body conditions; Rust structure proves no functional/AI-readability/upstream-intent claim.

The owner approved insertion-time resolved Table (historical D08B3-A) on 2026-10-07 (`comfirmo`), archived at `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`: five public fields, two public carriers, one public facade and logical preparation before Cell show. This supersedes its stale proposed-only gate only within the approved scope. Standalone Cell defaults/generation (historical D08B3-B) were separately ratified/materialized (`confirmo`), archived at `lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3`, manifest `3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`. A did not authorize B's added public field/type; B's separate confirmation did. Neither closes broader incomplete scope.

## Exclusive ownership and eligibility

Own a consumable measured-product cache local to one grid invocation. Static descendant dispatch, no new Layouter/entity fields, I/O/unsafe/globals or cross-layout/compilation retention. No visual Content Hash/PartialEq, raw-address key or IDs. Grid coordinates, tracks captures after measurement, cell_body consumes before installing cell region; delegate eligibility rules here.

Only accept a nonempty graphic ASCII token 0x21–0x7E, no spaces/controls, in Text (including under Source), Sequence with one material Text and neutral Empty, or sequential GridCell/TableCell wrappers without special alignment/coordinates/spans. Every grid cell must have simple placement: absent x/y and absent-or 1 spans; otherwise construct no cache. Fixed/Fraction rows are not captured. No Realized/Located/context/locatable/other carrier. Mathematical environments retain fallback.

The narrowly proved Styled exception supersedes only blanket Styled refusal: token traversal may cross Styled when every typed delta field is absent, at least one custom key exists, and every key is cell.*, exact eval.table.local or exact eval.table.local.recipe. No text.*/par.*/raw.*/typed/unknown key or other effect source. Keep original Styled document unchanged; only reuse measured product. The body still must produce exactly one identical Text leaf and effective TextStyle must match at consumption.

Classify placement, prepared coordinates and token through source_body(), removing only Source for inspection, never Styled/Realized/Located/ShowRecipe beyond the explicit Styled exception. Do not change/remove original carrier/span/product or treat origin as cache key/different body. Source-wrapped ineligible content or special placement remains ineligible for the whole set.

Capture/consume only outside subframe/cell/column mode/page columns, without smallcaps/pending floats/suffix transaction/checkpoint, active par.line.numbering or cell alignment. Save parent style, side margins and decoration-collector presence on cache creation and reject changes. Compare current effective TextStyle to measured leaf's emitted style as mechanical render-data equality, never language identity.

Entry records input row/column, finite positive origin x/body width, exact token, height and complete subframe result. Capture only one FrameItem::Text matching entire token, last_sub_frame_width fitting body region, and no orphan alignments. Sole 1e-9pt tolerance classifies cancellation in absolute x; input origin/width/product/geometry remain exact. Preserve decoration/last_sub_frame_width/bottom.

On consumption require same PlacedCell row/column and unit spans, exact origin/width/token, matching style/environment. Remove entry on attempt and consume at most once; mismatch/missing falls back and cannot later hit. Pool row/column index is valid only under simple-placement proof, never as general key. Prepared plan alone proves no eligibility: same sequential coords/unit spans/pure ASCII visual/styles/insets/origin/metrics/no metadata-locatable-context-effect guards remain, with new-representation proof. Callback/body/environment changes force fallback; do not relax original two-subframe control.

Reuse restores last_sub_frame_width/bottom and returns complete product to existing inset/Y-rebase/decoration/fill-stroke/clip/overflow/tail pipeline; emit nowhere else. No ad hoc x translation for different regions. Wrap/hyphenation/segmentation changing or multiplying product remains refused. No Unicode/space/control/compound carrier expansion merely because one probe appears simple; global text/break/render/font policy is unchanged.

V17 token Text/GridCell/TableCell arms move only eligibility into body and return None on failure. No then_some with eager recursion. Preserve nonempty graphic ASCII, simple_placement before align and resolved visual only with its align absent; do not broaden cache.

## Acceptance, refutation and scope

Internal same-phase continuous-flow ADR-0127 work changes no public API/default/compatibility. RED→GREEN: two Auto token cells use two total subframes rather than four with unchanged content/side metrics. Fixed rows/alignment/explicit placement/identities retain fallback. Cover multipage/inset/RTL/style/overflow/font/diagnostic controls. Prove capture/consumption through tests/external counts; decide gain through paired CLI with full source/binary/state/timestamps. Output failure/no repeated benefit refutes experiment and must be documented; extensions need new proof.

Test changed origin/width consumes invalid attempt with no later hit; custom tracking must invalidate through effective style even when flattened parent style is unchanged. These complement grid integration and justify mechanical guards, not Content-equality parity.

Graphic-ASCII extension needs punctuation decimal/ID once-only consumption with preserved metrics; control/Unicode/compound products/overwide/transformed products keep fallback. Count release subframes separately from timing; paired CLI versus frozenfc5b2571 includes corpus/table/grid/RTL/inset/pages/show/fonts/negatives, restricted owner reseal/build/lint/suite/full provenance. Source requires RED→GREEN simple Source, refused Source(Styled) outside the later specifically approved local-key exception, special-placement rejection, recount/timing and text/effect/geometry preservation. No change to C2 origin contract.

Local-key Styled experiment requires owner RED→GREEN allowed/refused wrappers, predefined external allocation gate and paired rebuilt Tekt/vanilla local-set/block-no-set/no-fill plus all 23 fixture outputs. If no work reduction or changed content/geometry/raster, remove experiment and record refutation. Build/suites/lint only after qualification, with exact-state provenance.

Require RED→GREEN, suites/build/lint/restricted preservation and D08A/B2, D05/D06/D07/query collections. Accept real language/effects/geometry, not byte equality or whitespace normalization to close render. General Table defaults/fields (table-root-equality), rich style functions/paint/strokes/insets, rich lines/gutters, multipage spans and PDF/UA are not certified. Preserve extra legacy D08B2 casts, syntax/flags and global nonlocatable policy.

V17: run unchanged existing functional controls before code; actual structural RED 28/GREEN zero V17. No helper hiding conjunction/suppression/linter change. Check success/fallback/error/short-circuit/effects and restricted reseal; full build/workspace suite/Clippy all-targets warnings and unfulfilled expectations as errors/fmt/strict Rustdoc/strict-reverse V1/V5/V15/V26/SARIF no new warnings under 12GiB/zero swap/one job. No unmeasured memory/performance/global-parity claim.


## V16-031 nominal boundary preservation

### Measurement before decision

Source audit at HEAD `a9f03073809fa15ff11d86c532f2c803ee892e47`, UTC `2026-10-19T16:03:25.152301+00:00`;
exact working-tree diffstat and source/L0 SHA-256 are retained in
`lab/results/v16-corrections-2026-10-19/batch-d/agents/grid/phase1-before.json`.
Evidence: 01_core/src/compiler/layout/grid/measurement_cache.rs:40-61,150-169,185-266; 01_core/src/entities/content/source.rs:11. The owning decision is `simple_placement`.
Observed: Cells are simple only with unit spans/no explicit coords; every other outer Content currently returns true.
This is source-level evidence of existing behavior, not a new current-pin
vanilla comparison or an inference about upstream intent.

### Decision and refutation

True means absence of outer placement fields only, not cache eligibility. token/capture/take separately refuse context, arbitrary wrappers and non-single-leaf products. Source is stripped transparently; Styled is not stripped here. Do not promote this true into a generic purity certificate.
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


The current nominal complement (placement-field-neutral only) is: `Source`, `ShowRecipe`, `Realized`, `Located`, `Empty`, `Text`, `Space`, `SpaceFromNewline`, `Parbreak`, `Sequence`, `HtmlElem`, `HtmlFrame`, `Par`, `Heading`, `Title`, `Strong`, `Emph`, `Raw`, `ListItem`, `EnumItem`, `Link`, `Equation`, `MathSequence`, `MathIdent`, `MathText`, `MathFrac`, `MathAttach`, `MathPrimes`, `MathRoot`, `MathStretch`, `MathDelimited`, `MathLr`, `MathMid`, `MathOverbrace`, `MathUnderbrace`, `MathUnderbracket`, `MathUnderparen`, `MathUndershell`, `MathOverbracket`, `MathOverparen`, `MathOvershell`, `MathAlignPoint`, `Linebreak`, `MathMatrix`, `MathCases`, `MathAccent`, `MathCancel`, `MathUnderline`, `MathOverline`, `MathVec`, `MathClassOverride`, `MathLimitsOverride`, `MathUnderover`, `MathOp`, `MathStyled`, `Label`, `Ref`, `CounterDisplay`, `CounterUpdate`, `Outline`, `OutlineEntry`, `Figure`, `FigureCaption`, `Image`, `Shape`, `Curve`, `Transform`, `Grid`, `GridHeader`, `GridFooter`, `SetPage`, `PageRun`, `Align`, `Place`, `Flush`, `Styled`, `Divider`, `Terms`, `TermItem`, `Quote`, `Document`, `Asset`, `SmartQuote`, `PdfAttach`, `PdfArtifact`, `Underline`, `Strike`, `Overline`, `SmallCaps`, `Pad`, `Hide`, `HSpace`, `VSpace`, `Pagebreak`, `Colbreak`, `Stack`, `Boxed`, `Block`, `Bibliography`, `Cite`, `Footnote`, `FootnoteEntry`, `TableHeader`, `TableFooter`, `GridHLine`, `GridVLine`, `TableHLine`, `TableVLine`, `Table`, `Repeat`, `Columns`, `Metadata`, `State`, `StateUpdate`, `StateDisplay`, `CounterDisplayCallback`, `ContextBlock`, `Dynamic`.

## V16-032 nominal boundary preservation

### Measurement before decision

Source audit at HEAD `a9f03073809fa15ff11d86c532f2c803ee892e47`, UTC `2026-10-19T16:03:25.152301+00:00`;
exact working-tree diffstat and source/L0 SHA-256 are retained in
`lab/results/v16-corrections-2026-10-19/batch-d/agents/grid/phase1-before.json`.
Evidence: 01_core/src/compiler/layout/grid/measurement_cache.rs:63-102,104-146,185-266. The owning decision is `token`.
Observed: None refuses cached-product reuse; it does not suppress layout. Miss returns to normal cell sublayout.
This is source-level evidence of existing behavior, not a new current-pin
vanilla comparison or an inference about upstream intent.

### Decision and refutation

The positive certificate is intentionally tiny: one graphic ASCII Text through exact admitted wrappers. The complement includes actual semantic/locatable/callback/style contexts that cannot safely reuse a measured product. Styled guard failure must remain refusal, not recursive fallback.
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

The current nominal complement (cache refusal) is: `Source`, `ShowRecipe`, `Realized`, `Located`, `Empty`, `Space`, `SpaceFromNewline`, `Parbreak`, `HtmlElem`, `HtmlFrame`, `Par`, `Heading`, `Title`, `Strong`, `Emph`, `Raw`, `ListItem`, `EnumItem`, `Link`, `Equation`, `MathSequence`, `MathIdent`, `MathText`, `MathFrac`, `MathAttach`, `MathPrimes`, `MathRoot`, `MathStretch`, `MathDelimited`, `MathLr`, `MathMid`, `MathOverbrace`, `MathUnderbrace`, `MathUnderbracket`, `MathUnderparen`, `MathUndershell`, `MathOverbracket`, `MathOverparen`, `MathOvershell`, `MathAlignPoint`, `Linebreak`, `MathMatrix`, `MathCases`, `MathAccent`, `MathCancel`, `MathUnderline`, `MathOverline`, `MathVec`, `MathClassOverride`, `MathLimitsOverride`, `MathUnderover`, `MathOp`, `MathStyled`, `Label`, `Ref`, `CounterDisplay`, `CounterUpdate`, `Outline`, `OutlineEntry`, `Figure`, `FigureCaption`, `Image`, `Shape`, `Curve`, `Transform`, `Grid`, `GridHeader`, `GridFooter`, `SetPage`, `PageRun`, `Align`, `Place`, `Flush`, `Divider`, `Terms`, `TermItem`, `Quote`, `Document`, `Asset`, `SmartQuote`, `PdfAttach`, `PdfArtifact`, `Underline`, `Strike`, `Overline`, `SmallCaps`, `Pad`, `Hide`, `HSpace`, `VSpace`, `Pagebreak`, `Colbreak`, `Stack`, `Boxed`, `Block`, `Bibliography`, `Cite`, `Footnote`, `FootnoteEntry`, `TableHeader`, `TableFooter`, `GridHLine`, `GridVLine`, `TableHLine`, `TableVLine`, `Table`, `Repeat`, `Columns`, `Metadata`, `State`, `StateUpdate`, `StateDisplay`, `CounterDisplayCallback`, `ContextBlock`, `Dynamic`.
`Styled` with a failed local-cell-only guard also refuses reuse; keep this residual inside the Styled arm.
