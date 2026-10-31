# L0 — table
Hash do Código: 26ae6d54

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:1f0c020521a9654478f7ba3325005ba621fe5f26b307b56dc49e8c526fceb59f
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf
- 00_nucleo/prompts/_nuclei/table/declarative-alignment.toml sha256:011aad6ca3199e192d8eb8eecd55d718b4d7ce7097f7987f982cb7a5f12f9a10

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/table.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/table.rs:27`, `01_core/src/compiler/layout/table.rs:127`, `01_core/src/compiler/layout/table.rs:155`, `01_core/src/compiler/layout/table.rs:174`, `01_core/src/compiler/layout/table.rs:229`, `01_core/src/compiler/layout/table.rs:243`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. The former reference `9f2b6e8715237cb086899a42873660fe744622e8` and older vanilla pins are historical snapshots. They do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC/diffstat/status/sources/binaries/arguments/results. Oracle SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`. Pinned `layout/grid/resolve.rs:204–278,1500–1530,1665–1703,1945–1988` resolves fields/conflicts/padding/breakability. Tekt `grid_placement.rs:91–112,262–270`, `realize/mod.rs:160–200`, `table.rs:127–152` expose explicit-first/Grid-only placement, bottom-up recipes and discarded wrappers. order measures no cell effects under replaced Table/no Heading effects under hidden cell; aliases measures LOCAL a, OUTER LOCAL a, NEW a. Language/effects, not copied mechanics/bytes/Rust equality. Unknown fields/hidden effects/wrong order refute correction; cache/pagination equivalence is not inferred.

geometry-fifth.json (binary 59da863f, tested-fifth snapshot, pin/HEAD/state/UTC) measures exterior CELL yMin68.27018 and interior82.65818 versus vanilla 68.27016/88.70814, an extra 6.05pt gap at 11pt text. Pinned layout/container.rs:342 and vanilla_defaults.rs:14–23 establish BLOCK_SPACING1.2em versus PAR_LEADING0.65em; `block.rs:106–134` uses prior baseline/above/geometric initialization. Missing block gap after inline nested prepared Table is a hypothesis refutable through recollected glyphs and no-prefix/cache/line/font controls.

geometry-eighth.json/tested-eighth.json.gz, binary `4495f38a78cd27484991c83272b331b73bf089783acb001b9d58fa34cbc386d6`, preserves RED 6.05pt gap: `sub_frame.rs:137–142` resets block_chain_active=false while `line_commit.rs:460` updates baseline without activating it. That flag cannot suppress margin after an actual inline line. geometry-ninth.json/after-ninth.json.gz refutes the next candidate by lost CELL a: `tracks.rs:202` measures without regions.cell, `cell_body.rs:129` installs it only on emission, making measurement shorter than emission.

`box-contexto-2026-10-14/probe.json.gz`preserves sources/binaries/HEAD/working tree/UTC: Box, inset2pt, prefix and nesting publish (0,0) despite displaced paint. The direct-context baseline difference is separate; do not copy vanilla merely because it differs. `mod.rs:1131-1148/1180-1200` writes local points to runtime.positions; Box groups its body in another coordinate system; `finish.rs:295-302` seals the map without recomposing those points. Realized writes through or_insert; labels have a parallel map. HTML already carries private markers through the frame at `html_frame.rs:30-199`. Preserve single-pass registration and per-sub-store sealing under ADR-0073/0074 position/hybrid-model sections; do not adopt vanilla global introspection. Inspection of coordinates and its owners found absolute uses in Grid and explicitly local uses in Place/Box; nucleus qualification preserves both and requires page transformation without doubled origin. Earlier snapshot: `posicoes-compostas-2026-10-14/before.json.gz`. Geometry/identity are observables; carrier/index are mechanics.

`lab/results/construction/diagnostics/clippy-aridade-final-2026-10-16/before.json.gz` preserves source/L0/HEAD/uncommitted tree/UTC/status/diffstat/global hashes; callers.txt includes fixtures. Same-named math::MathLayouter calls are a different unchanged engine. Prior inventory has 22 warnings, five corresponding arities; structure is private Rust, not parity/intent evidence.

V17 T166: `01_core/src/compiler/layout/table.rs:157`; `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz` preserves sources/L0/SHA-256/same HEAD/uncommitted state/UTC/status/diffstat. Hash-reconfirmed inventory has 28 guards/21 owners; pinned CI requires split/body condition. No functional-defect/AI-readability/upstream-intent inference.

The owner approved insertion-time resolved Table (historical D08B3-A) on 2026-10-07 (`comfirmo`), archived at `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md:3`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`: five public fields, two public carriers, one public facade and logical preparation before Cell show. This supersedes its stale proposed-only gate only within the approved scope. Standalone Cell defaults/generation (historical D08B3-B) were separately ratified/materialized (`confirmo`), archived at `lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-2026-10-07.md:3`, manifest `3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`. A did not authorize B's added public field/type; B's separate confirmation did. Neither closes broader incomplete scope.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Render an optional caption above the table, adding a prefix only when table.numbering is a String. Read the causal Table counter from the injected introspector with current fallback one, register its page for LoT, format or use decimal fallback, and retain the existing English `Table number: ` extension prefix. Caption remains outside the table carrier. This direct table-numbering extension is not a Figure and does not acquire figure selectors/show behavior by documentation.

Project visual children/header/footer and delegate all tracks, lines, align/inset, stroke/fill and visual row-group/pagination behavior to `00_nucleo/prompts/core/compiler/layout/grid.md`. Build the current parallel, visually empty Table semantic carriers after that layout, preserving summary and explicit cell kind/level/scope/spans in their current representation. Header Auto cells receive the current level-one Column classification; explicit Data remains Data. Uniform automatic headers produce grouped head/body carriers, mixed explicit Data retains direct rows. Place header logical rows only on the first affected page; retain the source's current sequential logical-row projection and even partition by affected pages.

The source does not construct logical footer rows, derive arbitrary header levels from TableHeader or align logical partition to measured visual breaks. Full Headers applicability relations and PDF/UA/AT/reflow proof remain unresolved, not promises of this sanitation. The older enclosing-frame proposal must not hide the actual parallel-empty-carrier behavior. Exporters own MCIDs and relation serialization; this owner creates no new layout pass.

Prepared Table consumes complete plan cells/coordinates/roles and delegates common Grid engine. Do not dump wrappers, recompute occupancy, execute recipes or index hidden raw bodies. Unprepared fixtures/callers retain historical fallback. Parallel visually empty semantic carriers use plan coordinates/spans/kind, retaining existing PDF/UA limits and no duplicate header in body; caption stays here.

For prepared nested Table in a subframe without caption and after a nonempty inline line, close that line and use its fresh baseline/descent plus BLOCK_SPACING above collapsed with pending margin, preserving initial_baseline_pending. Use physical is_sub_frame rather than regions.cell so measure/emission match. Do not require block_chain_active for this actual preceding line, use final height in the formula, bake coordinates in eval, call Engine/Func, or change raw Table/no-prefix first-line path. Carrier-only direct position markers are not material for preliminary closure; preserve all other items, including separators.

Pass super::grid::GridLayoutInput with e.columns/rows, visual_children cells, e rules, gutter None, e align/inset, visual_header/footer.as_ref and stroke/fill.as_ref. Keep resolved/fallback/visual preparation and later semantic carriers. This materializes only the nominal arity scope and supersedes its earlier pending state.

Merge TableCell arms in visual_content: resolved clone remains first; recursively remove only a wrapper with none of the nine optional attributes. Preserve full content/presence/semantics otherwise; never realize body here.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Require RED→GREEN, suites/build/lint/restricted preservation and D08A/B2, D05/D06/D07/query collections. Accept real language/effects/geometry, not byte equality or whitespace normalization to close render. General Table defaults/fields (table-root-equality), rich style functions/paint/strokes/insets, rich lines/gutters, multipage spans and PDF/UA are not certified. Preserve extra legacy D08B2 casts, syntax/flags and global nonlocatable policy.

Nested-table acceptance requires all three terms/glyphs survive, not only pairwise deltas among available terms; recollect glyphs before closure. Preserve nested no-prefix/cache/font/line controls, full geometry and no fake paint/height/identity/order/clone/rollback position effects; revalidate HTML/context/query/full suite.

Arity correction follows continuous-flow ADR-0127 without public/default/phase/compatibility change; L0 first, restricted resealing, too_many_arguments RED→GREEN without suppression/threshold change and satisfied prior expectations. Check bodies/value transport, preserve fixtures/expected outputs without carrier-mirror tests. Run existing Grid/Table/outline and SVG/tiling before/after, full workspace suite/build/Clippy all-targets/Rustdoc/fmt/strict-reverse lineage/Tekt without new warnings under 12GiB/zero swap/one job. No global parity/PDF-UA claim.

V17 maintenance preserves public/default/compatibility/phase and owner boundary. No helper hiding conjunction, suppression or linter change. Run functional controls before code unchanged; structural RED 28, GREEN zero V17; check success/fallback/error/short-circuit/effects. Revalidate full build/workspace suite, Clippy all-targets warning/unfulfilled-expectation errors, fmt, strict Rustdoc, strict/reverse V1/V5/V15/V26 and SARIF no new warning under 12GiB/zero swap/one job; no unmeasured global gain/parity.


## Approved physical source routing

### Evidence before decision

Exact source/owner bytes, HEAD, dirty-tree state and UTC before these amendments are archived in `lab/results/set-page-header-implementation-2026-10-21/agents/index/grid-original-before.json.gz`. The finite direct-site classification is `lab/results/set-page-header-2026-10-21/agents/parity/descendant-site-classification.json`. Source anchors include `01_core/src/compiler/layout/grid.rs:212-325`, `01_core/src/compiler/layout/grid_placement.rs:232-399`, `01_core/src/compiler/layout/grid/row_groups.rs:22-64`, `01_core/src/compiler/layout/grid/tracks.rs:224,543-561`, `01_core/src/compiler/layout/grid/cell_body.rs:285-346`, and `01_core/src/compiler/layout/table.rs:83-126`. Placement visibly reorders inputs; Auto-height products may be consumed later; Align/Block/prefix constructors wrap particular real source children. These establish transport ownership, not a new geometry/parity claim. The exact public/phase package is ratified in `lab/results/set-page-header-implementation-2026-10-21/approval.json`; no further public contract is introduced here.

### Exclusive routing contract

On the planned path preserve source wrappers when projecting raw visual children; the same existing Grid/default engine renders their selected body once. Resolved cells retain their exact plan index. Route source caption through TableCaption, including when the owner constructs a generated prefix Sequence; prefix and spacing remain unplanned. Header/Footer routes use exact selected source fields only in the raw fallback; resolved Table still consumes its complete known cells. Parallel empty semantic carriers remain generated-only and all prior support limits remain.

Preserve legacy unplanned rendering, source parents, explicit aliases, all existing source/geometry/style/diagnostic expectations and the bounded support/debts above. Measurement or generated-only output has no source fact; actual selected source descendants cannot silently become unplanned. Keep the exhaustive static dispatcher and owner-local child decisions. Use nominal plan/edge provenance, never Content/Func equality, text or frame scanning. Deferred physical positions remain separate from causal source anchor/order; preserve the actual source admission through page movement. Missing/conflicting edges fail closed before Complete.

### Acceptance

Require original-index preservation after explicit-first placement, resolved/raw/standalone selected children, header/footer and padding, aligned/inset generated wrappers retaining their source descendants, no phantom measurement effect or duplicate cache admission, source parents/aliases and deferred page anchors. Preserve prior tests and the original native path byte-for-byte in expectation. Actual source-language and unchanged legacy geometry/refusal controls plus full build/suites/strict lint/Clippy/Rustdoc/fmt/forward/reverse lineage are required before closure; no speed or broader Table/PDF-UA parity is inferred.


## Readiness: prepared table flow boundary (2026-10-28)

### Evidence before decision

`lab/results/construction/diagnostics/readiness-fixes-2026-10-28/before.json`
and source-before.json.gz record HEAD 212aed0449150dd296ceea13fc2be665cc5db01a,
UTC, exact uncommitted diffstat and source hashes, matching the prior readiness
inputs/oracle. baseline/receipt.json and heading-candidate/receipt.json preserve
bilateral PDF/SVG runs against pin 64c5e8c02fe0394d913ead36e42ef8c1334d2107.
In code-data, table CELL Item yMin50.04201 versus45.58705, final Placa101.75601
versus97.30104; raw starts43.183013 versus125.335023 and following prose85.28482
versus167.43684. The heading-only correction leaves these defects intact.
`table.rs:32-54` admits only nested prepared tables after inline material;
`grid/tracks.rs:577-589` recovers a generic text top, while grid/cell_emission.rs:194
advances rows. `table.rs:153-171` delegates and never publishes the completed
row end to the block protocol. `raw.rs:201-228` therefore reads an older baseline.
Current-pin typst-layout/src/rules.rs and typst-library/src/layout/container.rs
wrap Table in the ordinary Block spacing contract. Measured overlap/order and
geometry are language observables; row storage is mechanics. The inference is
refuted by first-table/no-prefix, nested, preceding prose/heading, following
text/raw, row-height and multipage controls, not by successful compilation.

### Exclusive correction

For a resolved prepared Table without the legacy caption extension, admit its
geometric start through the existing generic prepared-block admission before
Grid measurement/emission; the Grid must not subtract a text top a second time.
After Grid returns publish its actual current-region row end as baseline,
physical bottom and valid flow end, clear stale equation/line ascent state,
and retain the existing automatic paragraph/block below margin. Restore generic
paragraph indentation around the table. Preserve the raw/unprepared path,
caption extension, source routes, cell paint, pagination and parallel semantic
carriers. No new public field/signature, phase, default mode or compatibility:
ADR-0127 continuous parity repair. Do not fix table arrays or figure captions
through this scope. Test text/raw after a tall prepared row RED→GREEN; revalidate
source code-data, other everyday controls, nested tables, prior math/image/JPEG
controls and full gates. Do not claim global Table parity.


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

Project borrowed declarative Table alignment into the existing private Grid request for raw fallback. Prepared cells consume Complete alignment without root fallback/reselection. Preserve source routes, caption, row flow and semantic carriers.

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


### Legacy raw placement preservation after full-core refutation

The frozen table-alignment-core-before-repair log records a regression in the
unchanged layout_table_cell_dentro_de_table_renderiza_como_cell: legacy synthetic
None alignment intentionally ignores TableCell x=2 in two columns and retains
all four texts. Applying TableDocumentOrder to every raw Table instead rejects
that existing fixture. Preserve this old None/Uniform raw path and its original
Grid placement; only newly represented raw Auto/Columns declarations use the
Table indexed adapter before measurement. Complete prepared source Tables keep
their authoritative alignment/placement and do not reinstall lexical root axes.
This bounded correction preserves the original test rather than changing its
invalid-coordinate fixture/expectations. Legacy synthetic placement remains a
separate compatibility limitation, not global Table parity.


### Raw insertion style consumption under the approved contract

The signed declaration review requires omission to inherit at actual insertion,
including the raw synthetic physical path; source late/set controls already
measure that language behavior. Current table.rs:135 only projects e.align and
never reads chain custom table.align for raw omission. A synthetic raw Table
under the same existing validated style therefore cannot inherit it. This is
local source evidence of a missing consumer, not a new API/phase proposal.
Before implementation add co-located physical RED comparing raw styled omission
with the same explicit Columns declaration; controls retain explicit Auto reset,
explicit Uniform override and transparent sourced text. Malformed synthetic
style must yield the shared caster's diagnostic without executing a function.

For raw e.resolved=None and omitted e.align only, consume the existing causal
chain.custom("table.align") through the one compiler-private declarative caster,
using e.source_span as available causal origin. Hold the typed declaration in a
local variable only; no new Layouter state, callback, public signature or second
parser. Accumulate an invalid declaration through existing layout_errors and
stop that raw occurrence. Explicit declaration wins; Complete plans never read
or reselect root style. Feed Auto/Columns through the new Table path and Uniform
through the preserved legacy raw uniform path. Keep originals/Source unchanged,
all cache controls and lexical restoration. Require actual RED→GREEN plus
source/native comparisons, full core and unchanged final batch gates.


## Issue 8954 — approved integration

### Evidence before implementation

`lab/results/construction/diagnostics/issue-8954-2026-10-31/implementation-before.json.gz`
retains this exact owner/source, UTC, HEAD and dirty diffstat/status; measurements.json.gz
contains the fresh combined native probes and actual glyph baselines. The original
case is rejected, while distinct levels reorder 0,A,B,1 into A,B,0,1. Ordered
replacement/repeat controls establish group-level language behavior independently
of Cell kind or upstream panic mechanics. Human ratification is approval.json
in that directory, patch SHA 18e5019618e3c08e71c3a9d2f51a4f50d9b4a2e624fc70ab6d8de6c4aac6ee20.

### Exclusive amendment

Forward prepared header_groups to the private Grid input. Keep all raw fallback, caption and semantic carrier behavior and existing conformance limits. New header ranges do not imply PDF/UA certification.

L0 and restricted reseal precede RED/code. Preserve prior ratifications, purity and
static dispatch. Validate original source order, adjacent/replaced/nonrepeating
groups, effects and no hidden admission, spans/conflicts/padding, finite region
progress, legacy controls, native PDF/SVG/HTML, proportional suites/build and full
forward/reverse lineage/lint. All compilation/tests use only the exclusive RAM
target /dev/shm/typst-crystalline-8954-target; no fallback to disk. Rich interactions
remain bounded; no global parity or PDF/UA claim. Additional public/phase changes
require a new gate; the approved fields are not inferred from diagnostics.

For ordered prepared source tables project every resolved logical cell once
into the existing parallel semantic rows, retaining explicit Header/Data roles;
do not derive an empty body from dormant legacy children or duplicate headers.
Keep the existing PDF/UA limits. Unprepared new ordered input must diagnose its
missing logical preparation instead of silently rendering an empty table.
