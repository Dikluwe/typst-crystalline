# L0 — compiler/layout/grid/row_groups
Hash do Código: 11a24651


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf
- 00_nucleo/prompts/_nuclei/table/cell-style-defaults.toml sha256:041abef9e300ff0412d9631dced12480f8e0e6e65fb9cc3733f290e6664f649b


## Evidence before specification

Original logic is at `01_core/src/compiler/layout/grid.rs:185-282`, HEAD `4e2a321ce`, clean tree,2026-09-30 23:49:20 UTC. This is internal extraction, not new parity/performance proof. Complete historical measurements/provenance and repeated cross-owner qualifications remain in `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/compiler/layout/grid/row_groups.md`.

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC/diffstat/status/sources/binaries/arguments/results. Oracle SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`. Pinned `layout/grid/resolve.rs:204–278,1500–1530,1665–1703,1945–1988` resolves fields/conflicts/padding/breakability. Tekt `grid_placement.rs:91–112,262–270`, `realize/mod.rs:160–200`, `table.rs:127–152` expose explicit-first/Grid-only placement, bottom-up recipes and discarded wrappers. order measures no cell effects under replaced Table/no Heading effects under hidden cell; aliases measures LOCAL a, OUTER LOCAL a, NEW a. Language/effects, not copied mechanics/bytes/Rust equality. Unknown fields/hidden effects/wrong order refute correction; cache/pagination equivalence is not inferred.

`lab/results/construction/diagnostics/table-cell-styles-d08b3c-2026-10-07/{before,refuters-before,extra-first}.json.gz` records same HEAD/uncommitted state, UTC2026-10-07T15:24:58.038947+00:00–15:25:13.727872+00:00,58 probes, full source/argv/cwd/environment, historical oracle pin/binary and Tekt hashes. Pinned `model/table.rs:734–800`, `layout/grid/resolve.rs:203–278` distinguish style defaults, explicit Auto delegation and padding spans 1; measured show-set changes observed colspan after physical occupancy, not a second occupancy. Stroke getter-versus-resolve and captured inherited base are separately measured rather than inferred from a raw constructor.

`row_groups.rs:82` unused calculation and pre-edit uses/dispatchers/source/L0/HEAD/uncommitted state/UTC/status/diffstat are in `lab/results/construction/diagnostics/caminhos-residuais-ci-2026-10-13/before.json.gz`; no new bilateral parity proof.

`lab/results/construction/diagnostics/tekt-v16-table-cells-2026-10-17/before.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC/status/diffstat/source/L0/config/hashes including original two-owner snapshot. contexts.json identifies ten V16/latent projections before code. public-before.json records actual API/rlib/probe/binary/SHA/argv: Source(Cell) loses x/colspan/rowspan/kind/span in header, footer loses Source, Source members alone cause different-level rejection, Source footer changes a/b→ab/empty. snapshot-before.json measures context_floor 77→0 in both Source restores. cli-before/extra/controls preserve bilateral sources/full binary hashes at historical pin `9f2b6e8715237cb086899a42873660fe744622e8`: one footer argument [a#strong[b]] produces two Tekt cells a/b versus vanilla ab/empty, show table:none wrongly rejects, [] hides header/footer without Cell panic, alias colspan retains message/caret. Spanning/level row differences remain unclosed. These are semantics/morphology/origin/error, not Rust/bytes/intended-mechanics obligations.

The owner approved insertion-time resolved Table (historical D08B3-A) on 2026-10-07 (`comfirmo`), archived at `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`: five public fields, two public carriers, one public facade and logical preparation before Cell show. This supersedes its stale proposed-only gate only within the approved scope. Standalone Cell defaults/generation (historical D08B3-B) were separately ratified/materialized (`confirmo`), archived at `lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3`, manifest `3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`. A did not authorize B's added public field/type; B's separate confirmation did. Neither closes broader incomplete scope.

## Exclusive ownership and current contract

Layer L1. Sole consumer: `01_core/src/compiler/layout/grid/row_groups.rs`. Normalize whole Grid/Table header/footer rows, detect explicit body-cell conflicts with header, preserve diagnostic/hint and assemble header/body/footer order. Return conflicts through existing error channel without changing cell_align restoration. Preserve public contract/phase/default/compatibility/static dispatch/L1 purity.

Logical whole-group/role/conflict normalization remains exclusively here, exposed through a pure helper to Table preparation; logical output is not Frame. Prepared physical path consumes known ranges without concatenating/padding groups again. Legacy Grid keeps helper/diagnostics. Rich header/levels/lines retain measured limits, without a new PDF/UA promise.

Header conflicts use effective Cell y/rowspan and same source_span, while Grid retains prior contract. Realization owner supplies cloned prepared properties; this unit reads no styles or recipes, preserving messages/hints/lexical source. Remove unused rowspan extraction/calculation from the splice conflict test only: header occupies[0,header_rows), y>=0 and normalized span>0, so intersection is exactly y<header_rows. Preserve GridCell/TableCell, explicit y and first-conflict selection. Do not change placement/footer scope.

Only in table_groups, project Source-wrapped Header/Footer and bodies through private table_group_view to a normalized logistical view with nominal complement. Clone existing structs and preserve repeat; original Table retains Source. Use the view in existing combine_cells calls. Do not change occupancy policy/algorithm/padding or Grid path, and do not ratify already measured spanning/level discrepancies.

C01 combine_cells (`row_groups.rs:76`): only GridCell/TableCell supply explicit conflict y; nominal complement supplies none, preserving Grid policy. C02 table_groups (`:129`): only TableCell with matching placement.y supplies error source_span; every other case, including failed TableCell guard, stays None. Preserve guards/first diagnostic/order.

Cross-owner effective-property boundaries (`00_nucleo/prompts/core/entities/elements/table_cell.md`, `00_nucleo/prompts/core/compiler/realize/table_cells.md`, `00_nucleo/prompts/core/compiler/layout/grid_placement.md`): physical TableCellResolved remains occupancy plan while private standalone projection may alter observed lexically omitted spans after show-set; padding has lexical spans 1. Retain causal guards/stage, no raw-source mutation/public change. Successive declarative Stroke values fold specified components without carriers/callbacks in Styles; standalone constructor getter remains whole, positioned resolution folds constructor with captured Cell then Table base through private stroke_inherited:Option<Stroke>. ObservationEq includes base, canonicalization ignores derived render base, resolved Table base=None, clone/aliases preserve base and explicit None wins. Row-group normalization neither recomputes those properties nor changes occupancy from getters.

## Acceptance and incomplete scope

Require RED→GREEN, suites/build/lint/restricted preservation and D08A/B2, D05/D06/D07/query collections. Accept real language/effects/geometry, not byte equality or whitespace normalization to close render. General Table defaults/fields (table-root-equality), rich style functions/paint/strokes/insets, rich lines/gutters, multipage spans and PDF/UA are not certified. Preserve extra legacy D08B2 casts, syntax/flags and global nonlocatable policy.

Declarative-default corrections follow internal continuous-flow ADR-0127, no new public/external/default/phase, private props/helpers in externally nonconstructible carrier and unchanged lexical values. L0 first/restricted reseal/RED→GREEN; bilateral error/caret/effects with exact state/binaries/UTC and actual geometry, debug/release builds/suites/lint, prior tree preserved without commit/reset/global reseal. Certified subset remains integers/autos, Color/None/Auto, uniform Stroke/None, alignment, Length/partial Length-or-Auto inset, Bool/Auto and existing typed schemas. Gradient/tiling/functions/per-side strokes/percentage inset/rich constructor casts/multipage groups-spans/PDF-UA/positional set-text/general Table defaults need own measurement/L0. Fallback warnings do not certify support or license copying upstream defects. Span-getter/stroke folds require preimplementation RED→GREEN with geometry/regressions; standalone constructor precedence is not positioned-stroke resolution.

Dead-calculation removal preserves language/test expectations and all targets/core suite/build/Clippy/lineage, without suppressing warnings.

Source/cast correction preserves Table show→defaults/placement→Cell show→body. Constructor settles raw Cell only: no recipe/with_defaults/placement/fonts/layout. No public field/type/signature/trait/flag/default/phase/compatibility change. Keep informative justified V19, never claim all-alternative execution; no allow/expect/@ capture/if-let hiding exhaustiveness. Preserve unmodified legacy errors/casts, failed guards, callbacks and success/error restoration.

Owner RED→GREEN covers explicit/Source Cell properties/presence/kind/origin; footer casts by argument rather than fragment; unprepared raw forms; Source groups/two snapshot restores; type/Auto/None negatives/invalid-local-key shadow; show None without allowing other values. Revalidate hidden effects/late-rule CLI and pinned structural observables without reduced tests/whitespace normalization. Separate disposable real-workspace Content/Value/TableCellKind future variants with cfg(test) compiled require E0004 in all 18 final matches, counted once. Preserve inputs/cfg(test) witness error; use private target and no mutant artifacts in primary target.

Require build/full workspace suite/fmt/Clippy all-targets -D warnings/-D unfulfilled_lint_expectations/Rustdoc -D warnings with four indexes/strict-reverse V1/V5/V15/V26/replay/full SARIF diff/diff-check. Expected V16 141→131, only prescribed V19, no V20 growth and remove depth from two optional projections. MemoryMax12GiB/swap 0/one Cargo job/sequential heavy commands; no commit/push/merge/release/new CI pin.

Header/footer spanning and level boundaries remain for table-row-group-contract with own measurement/L0. Group level differs from semantic Cell kind/level; current inference is not a complete contract. Do not repair occupancy through Vec length or Source/Styles boundary markers. Preserve legacy Grid. Other rich casts/paint/inset/historical commitments, overall objective, math-stretch-composite and both matching obligations remain open.


## Approved physical source routing

### Evidence before decision

Exact source/owner bytes, HEAD, dirty-tree state and UTC before these amendments are archived in `lab/results/set-page-header-implementation-2026-10-21/agents/index/grid-original-before.json.gz`. The finite direct-site classification is `lab/results/set-page-header-2026-10-21/agents/parity/descendant-site-classification.json`. Source anchors include `01_core/src/compiler/layout/grid.rs:212-325`, `01_core/src/compiler/layout/grid_placement.rs:232-399`, `01_core/src/compiler/layout/grid/row_groups.rs:22-64`, `01_core/src/compiler/layout/grid/tracks.rs:224,543-561`, `01_core/src/compiler/layout/grid/cell_body.rs:285-346`, and `01_core/src/compiler/layout/table.rs:83-126`. Placement visibly reorders inputs; Auto-height products may be consumed later; Align/Block/prefix constructors wrap particular real source children. These establish transport ownership, not a new geometry/parity claim. The exact public/phase package is ratified in `lab/results/set-page-header-implementation-2026-10-21/approval.json`; no further public contract is introduced here.

### Exclusive routing contract

Expose an owner-private source routing projection matching exactly the current Grid/Table group peel and Sequence extraction. Preserve the exact Header/Footer body and SequencePart index paths, source wrapper order and generated padding None. Do not infer identity from a projected Content value or change occupied row policy/diagnostics. A selected source cell remains addressable even when legacy visual simplification would remove an attribute-free wrapper.

Preserve legacy unplanned rendering, source parents, explicit aliases, all existing source/geometry/style/diagnostic expectations and the bounded support/debts above. Measurement or generated-only output has no source fact; actual selected source descendants cannot silently become unplanned. Keep the exhaustive static dispatcher and owner-local child decisions. Use nominal plan/edge provenance, never Content/Func equality, text or frame scanning. Deferred physical positions remain separate from causal source anchor/order; preserve the actual source admission through page movement. Missing/conflicting edges fail closed before Complete.

### Acceptance

Require original-index preservation after explicit-first placement, resolved/raw/standalone selected children, header/footer and padding, aligned/inset generated wrappers retaining their source descendants, no phantom measurement effect or duplicate cache admission, source parents/aliases and deferred page anchors. Preserve prior tests and the original native path byte-for-byte in expectation. Actual source-language and unchanged legacy geometry/refusal controls plus full build/suites/strict lint/Clippy/Rustdoc/fmt/forward/reverse lineage are required before closure; no speed or broader Table/PDF-UA parity is inferred.


## Fallible Source and file-registration integration

Human approval on 2026-10-22 is retained in
`lab/results/panic-elimination-2026-10-22/source-contract-approval.json`;
its review SHA-256 is `2c879e8afa66721a8952dc8b9e7cf3c92dce58bf91aae5e28da5d603f5aa4d42`.
Source::{new,new_with_parser,detached,detached_with_parser} now return the existing
SourceResult<Source>; SystemWorld::register_file returns FileResult<FileId>.
Preserve this unit's successful content/id/span/order/diagnostic behavior while
propagating construction/registration failures through its existing error channel.
A tuple-returning entry returns its error in the existing result component and
retains already accumulated warnings. A synthetic source slot stores a failed
construction as FileResult rather than inventing a Source. Test-only fixture
helpers must explicitly require their valid construction/registration result;
all previous substantive assertions remain. No productive unwrap/expect, panic
catch, default Source, identity reuse or silent error is permitted for this migration.
This clause supersedes only the old infallible return/panic expectations for the
five approved APIs; it does not authorize unrelated signatures, fields, phases
or broadening/loss of existing diagnostics or negative controls.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.


## Issue 8954 — ratified ordered group normalization

Evidence/approval: before.json.gz, measurements.json.gz and approval.json in
lab/results/construction/diagnostics/issue-8954-2026-10-31 retain exact state,
source, UTC, diagnostics, order and the approved entity package. Own the pure
ordered-items projection to cells and typed source group boundaries, preserving
Header.level/repeat independently of Cell kind and distinct adjacent groups.
Do not compute occupancy or physical heights from vector length; pass boundary
ordinals to the existing placement owner, which returns real row ranges. Preserve
legacy table_groups/Grid unchanged. Ordered explicit-line integration remains
on its old constructor path until separately measured; do not silently drop lines.
Footer remains unique/final under current controls. Reject malformed groups
through SourceDiagnostic, never panic. No callback/Engine/layout here.
L0/reseal precede RED/code; preserve all prior approvals and source origins.
