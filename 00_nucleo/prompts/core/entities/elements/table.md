# Prompt L0 — `entities/elements/table` — `TableElem`
Hash do Código: cce6fde5

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf
- 00_nucleo/prompts/_nuclei/table/declarative-alignment.toml sha256:011aad6ca3199e192d8eb8eecd55d718b4d7ce7097f7987f982cb7a5f12f9a10

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/elements/table.rs`

## Base carrier and evidence

The archived base model uses a Content::Table Arc carrier with optional caption, real header/footer row groups and manually Debug-based Hash because TrackSizing/Stroke/Color contain f64. Upstream model/table.rs:495,525 identifies header/footer as child elements, not named arguments. Preserve the original base fields:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TableElem {
    pub columns:  Vec<TrackSizing>,
    pub rows:     Vec<TrackSizing>,
    pub children: Vec<Content>,
    pub hlines:   Vec<TableHLineElem>,
    pub vlines:   Vec<TableVLineElem>,
    pub header:   Option<Content>,
    pub footer:   Option<Content>,
    pub stroke:   Option<Stroke>,
    pub fill:     Option<Color>,
    pub caption:  Option<Content>,
}
```

Header/footer each hold at most one corresponding TableHeader/TableFooter, with an explicit duplicate error during native-table child resolution. Layout consumes them as row groups through the same mechanism as Grid. Constructors table(columns,rows,children) and table_with_caption retain compatibility; table defaults caption to None. A numbered caption does not turn Table into Figure or match figure.where(kind: table).

The old nonlocatable heading conflicts with its own element_kind/to_payload exception and the later Table kind contract. Preserve the existing Table payload/indexing boundary; do not use that stale heading to remove locatability. Exact extraction remains with its owner. Derived PartialEq covers every field, not only the original ten; Hash retains the established mechanics.

For raw tables plain_text prefixes caption when present and joins children with spaces; is_empty is children.is_empty() && caption.as_ref().is_none_or(|c| c.is_empty()). Maps recurse through children/header/footer/caption while retaining tracks, lines, stroke, fill and metadata. The prepared contract below qualifies which contributions are visited.

## Accessibility summary proposal and outstanding approval

Historical table.rs:29–42 lacked summary; upstream table.rs:271–277 stores optional text and pdf/accessibility.rs:137–143 changes only it through pdf.table-summary. The archived accessibility proposal requires optional semantic summary separate from caption/visible text: omission→None, explicit none rejected by the cast, empty string retained. Constructors/clone/hash/mechanical equality/maps preserve it; plain_text/is_empty ignore it. Transport to tagged PDF /Table must not change text/pages/boxes/geometry. The original P1288 record still describes a public gate; this consolidation has not established its approval receipt and does not promote the proposal to a newly authorized contract. Keep the requirement and approval-verification debt visible.

## Prepared table evidence and approval

`lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/diffstat/status/sources/binaries/argv, historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, oracle SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea` and Tekt `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`. Upstream grid resolve.rs:204–278 resolves fields,1500–1530 ordered conflicts,1665–1703 empty cells and 1945–1988 breakable. Local grid_placement.rs:91–112 prioritizes explicit cells and 262–270 only knows GridCell; realize/mod.rs:160–200 visits bottom-up and table.rs:127–152 strips wrappers without attributes. Probes show zero cell effects when Table is replaced, zero hidden-heading effects and aliases `LOCAL a`, `OUTER LOCAL a`, `NEW a`. Unknown fields, hidden effects or wrong order refute the hypothesis. Cache/pagination equivalence is not inferred, and Rust mechanics/bytes are not the acceptance target.

The original D08B3-A gate was satisfied by `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-2026-10-07.md`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`. It covers the specified five public fields, two carriers, facade and logical phase before Cell show, without authorizing later extensions.

## Prepared carrier contract

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TableGridResolved {
    pub cells: Vec<Content>,
    pub num_cols: usize,
    pub num_rows: usize,
    pub header_rows: usize,
    pub footer_start: usize,
    pub(crate) complete: bool,
}
```

TableElem additionally retains public source_span:Span, rows_explicit:bool and resolved:Option<Arc<TableGridResolved>>. Raw construction has resolved=None. Explicit rows differ from omitted fallback [Auto]; no children and no explicit rows must not fabricate a cell. Raw children/header/footer remain lexical source. The plan holds actual/empty cell origins once, not duplicate span entries, in row-major order. header_rows describes [0,n); absent footer uses footer_start=num_rows. Getters/serialization expose no new language fields for these carriers.

With resolved Some, visual traversal/emptiness/plain_text consume complete cells once, not raw children/header/footer in parallel. Maps/clone preserve lexical source and transport visual content without executing recipes.

## Change-aware traversal evidence and contract

`lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/change-aware-red-gate.json.gz` records the same HEAD/uncommitted provenance plus UTC/script/fixtures/interpositor/CPU/argv, release `29745a9e80ca43160b34f4807949f00e803070392653df0fc5e214b2f42ac33a`:483823 glibc requests on local-set-1600 exceeded the predefined 470000 gate. traversal.rs:25–55,235–264 and table.rs:85–118/table_cell.rs:308–344 rebuild unchanged containers/metadata. Pinned upstream realize/lib.rs:242–291,353–410 visits references and materializes after change; this motivates a refutable cost hypothesis without requiring its vtable/equality.

Provide only a crate-private reconstruction helper returning None when every visited child is unchanged. Raw order is children,header,footer,caption; prepared order is exclusively resolved.cells then caption, preserving raw source/metadata. Delegate descent to the explicit proof in `00_nucleo/prompts/core/entities/content/traversal.md`, allocating Table/plan only after first change. Existing Element signature/semantics reuse this implementation without a new trait method. Do not use equality or process effects twice. Verify child/prepared/caption/order tests and paired gates, without certifying excluded variants. The pinned no-change nucleus shares only the proof; this owner retains algorithm/fields/order/preservation.

## Prepared table acceptance and incomplete scope

Require RED→GREEN, suites/build/lint, restricted preservation and the original D08A/B2, D05/D06/D07/query collections. Accept language, effects and actual geometry, never byte equality or whitespace normalization to close render defects. The isolated-cell package separately completes raw-cell-no-placement/outside-fields defaults and fresh show-cell-reconstruct generation; the first prepared-table package alone is explicitly incomplete.

Generic Table defaults/fields (`table-root-equality`), function styles, rich paints/strokes/insets, lines/gutters, multipage spans and PDF/UA remain outside certification. Do not change legacy extra casts, syntax, flags or global nonlocatable policy through this slice.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/elements/table.md` preserves the original evidence and chronology.


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

Own #[derive(Debug, Clone, PartialEq)] pub enum TableAlignment { Auto, Uniform(Align2D), Columns(Vec<Option<Align2D>>) } and change only pub align to Option<TableAlignment>. None is omission; Some(Auto) is explicit reset; typed column None means auto. Preserve declaration through clone/map/hash, without baking style defaults. Keep Content constructor signatures and all other carriers unchanged. Entities contain no rendering or new public helper methods.

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


## Issue 8954 — public contract proposal, NOT APPROVED

### Measurement before proposal

`lab/results/construction/diagnostics/issue-8954-2026-10-31/before.json.gz`
preserves HEAD, exact dirty diffstat/status, UTC and original owner/source bytes.
`measurements.json.gz` preserves the fresh RAM binary, ratified vanilla identity,
exact native inputs/commands/UTC/results and glyph positions. The official input
returns a vanilla panic but a controlled Tekt rejection. The auto-height control
prints 0,A,B,C,1 in vanilla. The accepted distinct-level control prints 0,A,B,1
in vanilla versus A,B,0,1 in Tekt, with actual glyph baselines confirming order.
Pinned model/table.rs:503-512 specifies ascending concurrent levels and replacement
of equal/higher levels. Repetition controls confirm A remains active when C
replaces B at level 2; nonrepeating C removes B without repeating C later.
`structural/table_grid.rs:134-161` rejects duplicate inferred Cell levels and
merges distinct groups globally; `table_header.rs:17-20` lacks group level;
`table.rs:39,57-64` has only one raw header and one prefix row count. These
carriers cannot retain repeated adjacent equal-level group boundaries independently
of explicit Cell Header/Data semantics. Header group level is not Cell kind.

### Gate

This section is a draft for ADR-0127 section 2.1 approval, not current code
authority. Existing approved contracts remain in force until an explicit human
receipt approves this exact package. Do not implement or reseal consumers for
this proposal before that receipt. The proposal changes public entity contracts
and requires migration of public Rust struct literals; parity corrections alone
do not waive that gate. No new product mode/flag or pipeline phase is proposed.

### Proposed exclusive entity amendment

Add `pub items: Option<Vec<Content>>` to TableElem as an ordered, lexical native
input stream. Some, including Some(empty), is authoritative raw input; preserve
all admitted source item order/wrappers and group boundaries. None selects the
existing raw children/header/footer/lines representation. Native ordered values
do not populate competing legacy body/group streams. Existing ergonomic Rust
constructors keep signatures and initialize items=None; legacy synthetic callers
retain their behavior. Prepared traversal still consumes complete resolved cells
only, never both lexical and visual streams. Raw traversal consumes the selected
raw representation once. No recipes/layout in entities or constructor.

Add this public data carrier in the same owning module:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TableHeaderRange {
    pub start: usize,
    pub end: usize,
    pub level: u32,
    pub repeat: bool,
}
```

Add `pub header_groups: Vec<TableHeaderRange>` to TableGridResolved. Ranges are
logical row intervals [start,end), ordered, nonoverlapping, nonempty and bounded
by num_rows. Adjacent equal-level groups remain distinct. Copy the declaration's
level/repeat, independently of Cell Header/Data classification. Keep header_rows
for the legacy initial-prefix contract; it is not a substitute for the new list.
Only the existing logical placement/row-group owners compute ranges from actual
occupancy, not Vec length, tags in Styles/Source, inferred text or rendered frames.
Entities do not schedule repetition. Empty list retains legacy synthetic plans.

After approval, constructor, realization, traversal, source-routing and physical
layout owners must be amended/resealed before their respective RED/code. Keep
Table show -> logical placement/defaults -> Cell show -> emitted body, source
identity, guards, cache admission and restoration. Repeated prepared headers
create no extra logical member or repeated effect. No new Location or phase.

### Proposed bounded language acceptance

Preserve original row order, including body cells before the first header. At
a later header occurrence, remove active equal/higher repetition levels; keep
strictly lower levels. Enroll the new group only if repeat=true. Paint each
original group once even when immediately replaced, and do not duplicate an
original group beginning a new page. Short-lived groups and lack of intervening
body rows must not cause invalid ranges, slicing panics or nonprogress loops.
A region unable to make progress must fail through the existing diagnostic
channel, not panic or silently omit content.

Accept the official fixed-height input safely with 0,A,B,C,1; preserve the
measured distinct-level/single-late ordering, page-level A/B->A/C replacement
and repeat:false A-only continuation. Require independent native and co-located
module RED before implementation, GREEN, physical page/glyph/raster inspection,
source effect/order/alias controls, existing Grid/raw/default/header/footer/line
controls, build/suites and complete forward/reverse lineage/lint. HTML consumes
logical source/groups without invoking paginated layout; this package does not
claim semantic HTML or PDF/UA completeness. Rich spans/gutters/line interactions
need measurement before extending the certified subset. Preserve previously
supported legacy controls; never silently discard unsupported group items.

Approval covers only the three existing-entity field additions and this one
public carrier with its four fields. No further public API or phase relocation
is licensed; if integration requires one, stop for a new ADR-0127 decision.


## Issue 8954 ratification

The human approved this exact public package with `autorizo`; receipt and review
manifest are in `lab/results/construction/diagnostics/issue-8954-2026-10-31/approval.json`.
This ratification supersedes the proposal-only gate above for that package.
All prior approvals and all other boundaries remain.


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

Implement approved fields and raw selection for text/emptiness/maps/hash. Complete traversal retains ranges and visits cells once; raw items=Some selects only items plus caption. Preserve legacy None and all existing Table fields/defaults.

L0 and restricted reseal precede RED/code. Preserve prior ratifications, purity and
static dispatch. Validate original source order, adjacent/replaced/nonrepeating
groups, effects and no hidden admission, spans/conflicts/padding, finite region
progress, legacy controls, native PDF/SVG/HTML, proportional suites/build and full
forward/reverse lineage/lint. All compilation/tests use only the exclusive RAM
target /dev/shm/typst-crystalline-8954-target; no fallback to disk. Rich interactions
remain bounded; no global parity or PDF/UA claim. Additional public/phase changes
require a new gate; the approved fields are not inferred from diagnostics.

Raw Some(items) retains the existing structural emptiness rule: test vector
emptiness, not whether every cell's body is empty. Complete plans keep their
existing visual emptiness rule.
