# L0 — grid placement
Hash do Código: 7150c74a


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/grid_placement.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf
- 00_nucleo/prompts/_nuclei/table/cell-style-defaults.toml sha256:041abef9e300ff0412d9631dced12480f8e0e6e65fb9cc3733f290e6664f649b


## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/grid_placement.rs:42`, `01_core/src/compiler/layout/grid_placement.rs:64`, `01_core/src/compiler/layout/grid_placement.rs:262`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

At HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, `lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` records historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC/diffstat/status/sources/binaries/arguments/results. Oracle SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`. Vanilla `lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278` resolves fields,`:1500–1530` rejects ordered conflicts,`:1665–1703` creates empties,`:1945–1988` derives breakable. Tekt `grid_placement.rs:91–112` prioritizes explicit cells and`:262–270` only knows GridCell; `realize/mod.rs:160–200` visits recipes bottom-up; `table.rs:127–152` emits wrappers without attributes. order measures zero cell effects when table is replaced and zero Heading effects when cell hides it; aliases measures LOCAL a, OUTER LOCAL a, NEW a. Language/effects, not Rust mechanics/bytes/equality. Unknown fields/hidden effect/wrong order refute proposed fix; cache/pagination equivalence is not inferred.

placement-refuters-red.json in that diagnostic preserves frozen B2 binary, pinned vanilla and complete sources/argv/cwd/UTC/results. Pinned `grid/resolve.rs:2182–2215` searches free origin without testing spans and advances whole area for full-width colspan;`:2269–2301` rejects full row;`:1500–1530` rejects conflicting spans. This is position/error semantics. Acceptance of conflicts, wrong message/span or post-full-span cursor refutes correction.

`lab/results/construction/diagnostics/table-cell-styles-d08b3c-2026-10-07/before.json.gz` records 58 probes, UTC2026-10-07T15:24:58.038947+00:00 through 15:25:13.727872+00:00, same HEAD/uncommitted tree/diffstat/status/full sources/argv/cwd/environment. Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binary `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt `721b7c2ae63b32b77e71913cc8f86788ce1c8bbc1090f54cd59138ac6ead5bf3`. isolated/table-set/individual fields/late/local/reuse/spread/negatives refute empty-target warning and absent defaults. padding-span-one/empty-fixed-rows fix padding spans at 1; auto-table-fallback preserves explicit Auto delegation to Table rather than Cell set. show-cell-set-self changes standalone but not positioned known fill. Stroke/alignment fold. refuters-before.json.gz adds header/partial inset/args/condition/Table snapshot; alias-inside-table is valid vanilla but improperly recursive Tekt.

Pinned `typst-library/src/model/table.rs:734–778` declares fields; `layout/grid/resolve.rs:203–278` reads styles before resolving/overwriting position/fill; packed padding defaults at `model/table.rs:785–800` fix spans 1. Tekt `rules/set.rs:23–27` text_str target misses FieldAccess; `table_cells.rs:262–310` uses lexical properties only in placement/inheritance; `grid_placement.rs:80` reads direct x/y/spans; standalone TableCell lacks private effective properties. Fields/order/effects/style/errors are language; private representation is Tekt's choice, not copied Packed/Rust equality. Any negative, lost lexical font, raw-value mutation, repeated effect, lost alias guard or geometry/cache regression refutes.

refuters-before stroke-constructor-fold/align-constructor-fold preserve whole constructor against Cell set. self-set-does-not-change-placement changes observed colspan to 2 after placement with 1, not new occupancy. Vanilla returns colspan 2 for real x0 cell but 1 for padding x1/x2: `layout/grid/resolve.rs:218–244` captures occupancy spans without settling their content fields; `model/table.rs:785–800` settles padding 1. Occupancy/getter are distinct observables.

extra-first.json.gz (full sources/HEAD/state/UTC/binaries) compares two-stroke-sets: red then 2pt produces red 2pt in standalone/positioned vanilla getter. stroke-render-inherits-cell-style folds constructor 2pt with Cell red before Table 3pt fallback, although standalone stroke-constructor-fold exposes only constructor 2pt. Pinned `model/table.rs:765` #[fold] and `resolve.rs:225–237` stroke.resolve(styles) differ from raw getter.

`grid_placement.rs:283` compiler-unused items are recorded with usage/dispatchers before editing in `lab/results/construction/diagnostics/caminhos-residuais-ci-2026-10-13/before.json.gz`, full sources/L0/same HEAD/uncommitted tree/UTC/status/diffstat. `grid_placement.rs:216,217,234,235` selected Clippy spans and source/owner/reference/binary/state provenance are in `lab/results/construction/diagnostics/iteracao-parametros-ci-2026-10-13/before.json.gz`. These are internal mechanics/uses, not new bilateral parity/performance.

## Ratified public gate and scope

The initial resolved-Table proposal required confirmation because it introduced five public fields, two public carriers, one public facade and logical phase before Cell show. The owner ratified that package on 2026-10-07 (`comfirmo`), as recorded at `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3`, manifest SHA-256 `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`. That recorded ratification supersedes the stale candidate-only gate for its approved scope; later internal corrections retain the same boundaries. Standalone Cell work was excluded from that approval and separately ratified in `lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3` (`confirmo`), manifest `3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`. Neither approval certifies the broader incomplete scopes below.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own PlacedCell construction and deterministic occupied-slot placement. Require a positive column count with the current diagnostic. Normalize spans to at least one. Place explicit GridCell x/y requests first, checking invalid columns, boundary-crossing colspan, duplicate origins and occupied-span overlap with their existing diagnostics/hints; a missing y searches the first free row at the specified/default x. Then place remaining cells in row-major free rectangles, skipping occupied slots and wrapping when the span would cross the column boundary. Preserve each original Content wrapper for later style handling and document order within each pass.

The current field extractor recognizes GridCell metadata; non-GridCell content is automatic unit-span content. Do not claim full TableCell-coordinate parity here or infer placements by appearance. No rendering, font measurement, page commit or public-contract expansion is introduced.

The one occupancy engine has closed policies GridExplicitFirst (legacy Grid) and TableDocumentOrder; typed adapters share algorithm without duplication. Table processes input origins in document order and rejects occupied/spanned origins. Fixup fills only real holes and explicitly requested minimum rows; row-major output determines recipe execution. Use checked arithmetic and causal source_span diagnostics, never blind allocation from huge coordinates. Prepared-Table layout consumes plan coordinates without rerunning occupancy or promoting cells into the index. Grid retains its API/policy/rules. This refines the earlier GridCell-only extractor description only for prepared Table's separate adapter.

TableDocumentOrder chooses the first free origin, never skips it because colspan cannot fit or later rowspan conflicts. Validate span afterwards with causal error/hint. X-only searches origins only in that column; a full Y-only row has dedicated error. Full-width colspan advances the area cursor as specified by measured controls. GridExplicitFirst keeps legacy search for a fitting rectangle.

Read only effective Cell x/y/spans through the nominal private helper for TableDocumentOrder, including style/prepared snapshots. Keep occupancy algorithm/order/padding/public types/signatures/Grid policy; no second engine or StyleChain/Func/Engine lookup in placement. Remove unused private extract_cell_fields only; preserve active extraction/realization metadata/spans.

Cross-owner observed-versus-physical boundary: positioned Cell retains external TableCellResolved physical plan. A private standalone carrier may coexist only for observed recipe properties, retaining resolved fields and overriding lexically omitted spans through show-set; getters/canonicalization use that projection, while complete-plan placement still uses TableCellResolved. Source-free padding has lexical spans 1 immune to show-set. Guards/stage stay causal, without public contract/raw-source mutation. These are boundary criteria for `00_nucleo/prompts/core/entities/elements/table_cell.md` and `00_nucleo/prompts/core/compiler/realize/table_cells.md`, not extra ownership here.

Stroke boundary: successive declarations fold specified components into declarative Value::Stroke, never storing carrier/callback in Styles. Standalone constructor stroke remains whole in getter; positioned resolution folds constructor with captured Cell base, then Table. Private TableCellProperties.stroke_inherited:Option<Stroke> carries causal base without baking it into lexical getter. ObservationEq includes base, canonicalization ignores render-derived base; already-resolved Table base is None. Clones/aliases retain prior base; explicit None wins. Whole-constructor precedence applies to standalone getter, not resolved stroke folding. This owner changes no extra public contract/phase/default through that private refinement.

Traverse validated occupied-area slices in row→column order. Normalize spans, checked arithmetic/columns and ensure_rows precede access. Enumerate offsets to reconstruct same absolute first-conflict coordinates/message/hint/causal span. Separate checking from marking so errors leave no partial area; mark true only in rectangle column slice. No silent truncation for invalid bounds, policy/cursor/padding change or new lint suppression.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

The confirmed public gate requires RED→GREEN, suites/build/lint/restricted preservation and D08A/B2, D05/D06/D07/query collections. Accept actual language/effect/geometry, never bytes or whitespace normalization to close render. Cell standalone defaults and fresh show-cell-reconstruct generation remain separately specified completion work. General Table defaults/fields (table-root-equality), rich style functions/paint/stroke/inset, rich lines/gutters, multipage spans and PDF/UA are not certified. Preserve extra legacy D08B2 casts, syntax/flags/global nonlocatable policy.

Declarative-default corrections follow continuous-flow ADR-0127 without public/external/default/phase change; private props/helpers remain in an already nonconstructible external carrier and lexical values stay unchanged. L0 first, restricted resealing and RED→GREEN; bilateral error/caret/effect controls with source/binaries/HEAD/UTC and real geometry, debug/release build/suites/lint. Preserve prior tree without commit/reset/global reseal. Certified subset: integers/autos, Color/None/Auto, uniform Stroke/None, alignment, Length/partial Length-or-Auto inset, Bool/Auto and existing typed schemas. Gradient/tiling/functions, per-side strokes, percentage inset/rich constructor casts, multipage groups/spans/PDF-UA, positional set/text and general Table defaults remain incomplete with measurement/L0 before expansion. Fallback warnings do not certify support; do not copy upstream defects.

Observed-span getter requires RED before implementation, then GREEN with geometry/regressions; stroke fold requires RED→GREEN. Preserve existing language results/test expectations while deleting unused helper. Equivalent iteration maintenance adds no public contract/default/phase; do not add tests merely mirroring loops. Validate core suite including conflicts/spans/fixups/markers/differential selection, all targets/build/docs/Clippy/lineage with restricted resealing. Do not suppress warnings instead of correcting code.


## V16-036 nominal boundary preservation

### Measurement before decision

Source audit at HEAD `a9f03073809fa15ff11d86c532f2c803ee892e47`, UTC `2026-10-19T16:03:25.152301+00:00`;
exact working-tree diffstat and source/L0 SHA-256 are retained in
`lab/results/v16-corrections-2026-10-19/batch-d/agents/grid/phase1-before.json`.
Evidence: 01_core/src/compiler/layout/grid_placement.rs:65-83,98-121,143-275. The owning decision is `place / fields closure`.
Observed: Only TableCell under TableDocumentOrder or GridCell under GridExplicitFirst supplies coordinates/spans/span; all other policy-content combinations are automatic unit cells.
This is source-level evidence of existing behavior, not a new current-pin
vanilla comparison or an inference about upstream intent.

### Decision and refutation

Wrong-policy cell variants are legitimate raw bodies, not another policy’s placement instructions. Raw Empty holes/text must remain unit-span and detached, while Table cells keep source-span errors. Need two complete policy partitions including guarded residuals, not an unguarded combined cell arm.
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
Wrong-policy `TableCell` and `GridCell` are also raw unit cells; classify each policy within the corresponding cell arm.


## Approved physical source routing

### Evidence before decision

Exact source/owner bytes, HEAD, dirty-tree state and UTC before these amendments are archived in `lab/results/set-page-header-implementation-2026-10-21/agents/index/grid-original-before.json.gz`. The finite direct-site classification is `lab/results/set-page-header-2026-10-21/agents/parity/descendant-site-classification.json`. Source anchors include `01_core/src/compiler/layout/grid.rs:212-325`, `01_core/src/compiler/layout/grid_placement.rs:232-399`, `01_core/src/compiler/layout/grid/row_groups.rs:22-64`, `01_core/src/compiler/layout/grid/tracks.rs:224,543-561`, `01_core/src/compiler/layout/grid/cell_body.rs:285-346`, and `01_core/src/compiler/layout/table.rs:83-126`. Placement visibly reorders inputs; Auto-height products may be consumed later; Align/Block/prefix constructors wrap particular real source children. These establish transport ownership, not a new geometry/parity claim. The exact public/phase package is ratified in `lab/results/set-page-header-implementation-2026-10-21/approval.json`; no further public contract is introduced here.

### Exclusive routing contract

Provide a private indexed adapter of the same existing placement engine. Retain original enumeration indices through the stable Grid explicit-first sort and all placement calculations; generated Table holes have no source index. Existing public and crate entrypoints discard the sidecar and return unchanged PlacedCell values/types. Do not add a field, second occupancy algorithm, numeric identity derivation or rendering dependency.

Preserve legacy unplanned rendering, source parents, explicit aliases, all existing source/geometry/style/diagnostic expectations and the bounded support/debts above. Measurement or generated-only output has no source fact; actual selected source descendants cannot silently become unplanned. Keep the exhaustive static dispatcher and owner-local child decisions. Use nominal plan/edge provenance, never Content/Func equality, text or frame scanning. Deferred physical positions remain separate from causal source anchor/order; preserve the actual source admission through page movement. Missing/conflicting edges fail closed before Complete.

### Acceptance

Require original-index preservation after explicit-first placement, resolved/raw/standalone selected children, header/footer and padding, aligned/inset generated wrappers retaining their source descendants, no phantom measurement effect or duplicate cache admission, source parents/aliases and deferred page anchors. Preserve prior tests and the original native path byte-for-byte in expectation. Actual source-language and unchanged legacy geometry/refusal controls plus full build/suites/strict lint/Clippy/Rustdoc/fmt/forward/reverse lineage are required before closure; no speed or broader Table/PDF-UA parity is inferred.


The indexed production consumer makes the old unindexed Grid-only shim a test-only compatibility adapter: source search finds every remaining call inside this owner's existing cfg(test) modules. Compile that shim only for tests, preserving all existing test calls and expected placement/errors. The Table entrypoint remains productive and retains its existing signature; no suppression or public surface is added.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.
