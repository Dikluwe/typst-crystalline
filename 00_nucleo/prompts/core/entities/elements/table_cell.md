# Prompt L0 — `entities/elements/table_cell` — `TableCellElem`
Hash do Código: 65edc838

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/map-content-change.toml sha256:6343f553bd5f842f83190bd6533a9a9642118a34214550d8289c9daaba219043
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:bba33b336e651ccd6d41f88b0e16449932d0a986de1ee2cc3c1fdf5c8fb8c6dc
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:f7d3050154813e1ea46c26e235383ae337781a4b28d10ef3248f6eaec96ce221
- 00_nucleo/prompts/_nuclei/table/cell-projection.toml sha256:8388731d50379499612641a70a85695c0e7139ec605289c29161e12086abd78a
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf
- 00_nucleo/prompts/_nuclei/table/cell-style-defaults.toml sha256:041abef9e300ff0412d9631dced12480f8e0e6e65fb9cc3733f290e6664f649b


Layer L1. Sole productive consumer:
`01_core/src/entities/elements/table_cell.rs`. Origin: model D (ADR-0105),
historical grid/table-cell extraction P327. The former glossary reference
entities/elements/_comum.md §A.0 is historical, not another current productive
owner. Cell is nonlocatable. Container operations preserve lexical body and use
prepared visual contributions according to the operation-specific contracts below.

## Baseline data and invariant preservation

The original extraction shape, before ratified additions below, was:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TableCellElem {
    pub body:      Content,               // formerly Box<Content>
    pub x:         Option<usize>,
    pub y:         Option<usize>,
    pub colspan:   Option<usize>,
    pub rowspan:   Option<usize>,
    pub stroke:    Option<Stroke>,
    pub fill:      Option<Color>,
    pub align:     Option<Align2D>,
    pub inset:     Option<Sides<Length>>,
    pub breakable: Option<bool>,
}
```

Content::TableCell's former inline variant becomes Arc<TableCellElem>; preserve
ergonomic Content::table_cell constructor. Manual Debug-based Hash
format!("{self:?}").hash(state) accommodates floating Stroke/Color/Align2D/
Sides<Length>, retaining content_hash parity and −0.0≠0.0 caveat. PartialEq derives.
Original ten-field equality remains extended by all subsequently ratified fields;
no silent omission. Baseline plain_text/body.is_empty and recursive body maps
preserved nine cosmetics; prepared-visual contract below supersedes body-only
consumption for prepared Cells. Original get_field/element_kind/to_payload default
is superseded only by the measured getter contract below. Baseline source readings:
content.rs:1813,1662,2289,2530,1953 (runtime-only spans in old plain_text).

## Pending accessibility classification candidate

Before deciding, table_cell.rs:20–33 contains body/geometric-visual fields only.
Pinned vanilla pdf/accessibility.rs:274–310 measures closed
TableCellKind::{Header(NonZeroU32, TableHeaderScope), Footer, Data}, scopes
Both/Column/Row; table.cell.kind starts auto, model/table.rs:772–778.

Historical P1288 proposes explicit Auto, Header{positive level,scope Column|Row|
Both}, Data. Auto lets header/footer context derive class; explicit Header/Data
wins. pdf.data-cell inside header stays Data. map_content/map_text/constructors/
clone/Hash retain kind/level/scope; unchanged body/plain text/visual render.
Closed enum allowed instead of three fields, never strings/visual style/inferred
position. No explicit human receipt was verified for this public classification
candidate. Preserve complete candidate and approval-evidence gap; later Cell
presence/realization approvals do not authorize accessibility classification by
implication.

## Existing Cell projection mapping

Before deciding, table-cell-realizacao-2026-10-07/before.json.gz retains HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/exact diffstat/
source-binary hashes/argv/cwd/results at historical pin9f2b6e871. Vanilla
table.rs:734–778 declares body/properties; local table_cell.rs:62–109 lacks getter;
content/identity.rs:107 uses table.cell; content_methods.rs:226–254 lacks real
function projection; show.rs:300–320 omits native. body/fields/func-eq/explicit
show refute parity. Language semantics/morphology, not Rust/PDF equality or
structure-inferred intent.

Element getter exposes body/known x-y-colspan-rowspan-stroke-fill-align-inset-
breakable. Uniform inset projects Relative with zero ratio; nonuniform projects
ordered left/top/right/bottom dict. Internal kind is not public field.
RED→GREEN/bilateral collection/
build/crystalline-lint without error, no public type/signature/phase change.
Lexical omitted/auto/none and position/style/padding/show realization retain
separate measured contracts. Option None proves neither omission nor explicit
none; do not certify them through this mapping.



## Ratified lexical TableCell presence

### Evidence before decision

`lab/results/construction/diagnostics/table-cell-realizacao-2026-10-07/before.json.gz`
and presence-refuters.json retain HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree/diffstat/UTC/binaries/argv
against historical pin `9f2b6e8715237cb086899a42873660fe744622e8`. Upstream
model/table.rs:734–778 uses Smart/Option; local table_cell.rs:40–51 keeps only
value Option. x:auto/omitted collapse; fill:none/stroke:none lose presence.
Refuters: auto is accepted for x/y/fill/align/inset/breakable and rejected for
colspan/rowspan/stroke; none is accepted only for fill/stroke. These are language
observables, not required upstream representation.

### Exclusive responsibility and acceptance

This entity exclusively declares the ratified public closed types:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CellFieldPresence { #[default] Omitted, Auto, Explicit }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TableCellPresence {
    pub x: CellFieldPresence, pub y: CellFieldPresence,
    pub colspan: bool, pub rowspan: bool,
    pub fill: CellFieldPresence, pub stroke: bool,
    pub align: CellFieldPresence, pub inset: CellFieldPresence,
    pub breakable: CellFieldPresence,
}
```


Add pub presence: TableCellPresence, keeping all prior field types. Body always
known. Omitted exposes no field; Auto Value::Auto; Explicit stored value.
Only explicit fill/stroke may be Option None projecting Value::None.
Explicit x/y/align/inset/breakable require Some; Auto None. Present colspan/
rowspan require positive Some. No presence in kind/strings/generic Value/PropMap/
registry. map_content/map_text/clone/Hash retain presence; unknown getter None.

Require RED→GREEN for body/presence/methods/JSON,
clone/map/observation and span-anchored negatives, build/suites/lint, D08A/D05/D07
and new refuters with exact provenance. The human approved new public fields/
types on 2026-10-07; receipt
`lab/history/construction/records/00_nucleo/materialization/table-cell-presenca-aprovacao-2026-10-07.json`.
This changes no render phase/default. Position/style/padding/show realization is
the separately ratified insertion package, not implicit scope of lexical presence.


empty-body in table-cell-presenca-2026-10-07/after.json.gz records Tekt body
{func:empty} versus pinned{func:sequence,children:[]}, with HEAD/tree/source/UTC/
binaries. Getter projects stored Empty as language empty Sequence without body
mutation/phase/new property. Nonempty original morphology stays. RED→GREEN/fresh
bilateral measurement mandatory.

## Ratified Table insertion and incomplete Cell scope

The approval receipt `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-aprovacao-2026-10-07.json` records user confirmation `comfirmo`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`, and materialization with documented metric residuals. This satisfies the original D08B3-A gate; it does not complete the Cell domain or promote the excluded scope. Retain the receipt’s limitations on raw default typography, D08B3-B isolated Cell/fresh reconstruction, and generic Table equality/rich properties.

### Measurement before the proposal

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree. `lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` records pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC, diffstat/status, sources/binaries, arguments, and results. Oracle binary SHA-256: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt binary: `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`.

Vanilla `lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278` resolves fields, `:1500–1530` rejects conflicts in order, `:1665–1703` creates empty cells, and `:1945–1988` derives breakability. Tekt `grid_placement.rs:91–112` prioritizes explicit placement and `:262–270` knows only `GridCell`; `realize/mod.rs:160–200` visits recipes bottom-up; `table.rs:127–152` discards wrappers without attributes. `order` measures zero cell effects when a table is replaced and zero heading effects when a cell hides it; `aliases` measures `LOCAL a`, `OUTER LOCAL a`, and `NEW a`. These are language observables, not requirements to copy mechanics, bytes, or Rust equality. Unknown fields, hidden effects, or wrong order after a change would refute the hypothesis; cache/pagination equivalence is not inferred.

### Exclusive public positioned carrier

Declare exact types:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TableCellStage { Prepared, RecipeApplied, Complete }
#[derive(Debug, Clone, PartialEq)]
pub struct TableCellResolved {
    pub x: usize, pub y: usize,
    pub colspan: usize, pub rowspan: usize,
    pub fill: Option<Color>, pub stroke: Option<Stroke>,
    pub align: Option<Align2D>, pub inset: Sides<Length>,
    pub breakable: bool, pub visual: Content,
    pub(crate) stage: TableCellStage,
}
```


Add pub source_span: Span and pub resolved: Option<Arc<TableCellResolved>>.
Prior fields/presence stay lexical. Raw constructor resolved=None; causal source
span is not language property. Body getter keeps original/empty-Sequence projection.
Resolved Some makes other nine getters known; resolved align None known Auto,
fill/stroke None known None. Visual uses resolved.visual, never overwrites lexical
body. clone/maps/Hash/observation preserve carrier/stage. Only Complete publishes/
renders. Prepared is resolution stage; RecipeApplied it snapshot consumed own rule
but body must realize if emitted. Aliases never reapply own recipe. Nominal crate-
private stage, no Location/registry.

### Ratified gate, acceptance, and explicit scope split

The ratified package adds five public fields, two public carriers, and one new public facade, and changes the logical phase before Cell show. The receipt above establishes ratification for this scope. Continue to require RED→GREEN, suites/build/lint/restricted preservation, and D08A/B2, D05/D06/D07/query collections. Acceptance is language, effect, and actual geometry behavior; byte equality or whitespace normalization cannot close render acceptance.

This first package D08B3-A is incomplete in the Cell domain: D08B3-B is to complete isolated Cell defaults (`raw-cell-no-placement`/`outside-fields`) and the separate decision on fresh generation by `show-cell-reconstruct`. Generic Table defaults/fields (`table-root-equality`), rich styles/functions/paint/strokes/insets, rich rows/gutters, multi-page spans, and PDF/UA conformance remain outside certification. Do not change additional legacy D08B2 casts, syntax, flags, or global non-locatable policy in this package.



## Ratified prepared-Cell occurrence termination

### Evidence before decision

`lab/results/construction/diagnostics/table-cell-isolada-d08b3b-2026-10-07/complete.json.gz`
contains 53 probes collected 2026-10-07T14:47:56.340144+00:00 through
14:48:12.375245+00:00. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted
working tree, diffstat/status/hashes/sources/argv/cwd/environment and full outputs
are recorded. Historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`,
binary SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`;
Tekt `6ca44e46ae2c1b1f8b44207052ea2fd63e3b24b4136c3fc3d25b6d20e9b6a20f`.
Do not count before/refuters/complete as additional independent rounds.

isolated-default exposes vanilla x/y/inset/align/fill/breakable auto, spans 1 and
empty stroke dict; Tekt omits defaults and emits two metadata versus one.
isolated-x fails only in Tekt; raw-fields remains body-only in both and raw-x
fails in both. isolated-table-style proves Table defaults are not standalone
Cell defaults. raw-vs-prepared is false in vanilla, true in Tekt.
nested-effects/body-effects/late-rule refute once-only execution/lexical origin.
Tekt wrongly accepts fresh-isolated/table, fresh-block, fresh-styled and
multi-fresh; vanilla rejects `maximum show rule depth exceeded` at the new
constructor. identity/body/alias and fresh-conditional/hidden remain valid.
multi-identity/multi-return-table show that returning it must permit an earlier
unconsumed recipe. isolated-query remains `cell is not locatable` and location
none in both, without false Location. cell-set is separately excluded.

Upstream model/table.rs:734–778 declares defaults/properties;
typst-realize/src/lib.rs:367–386 prepares/guards arguments, :418–429 visits
products with depth, :491–505 distinguishes per-element guards, :571–586
materializes properties. Local table_cell.rs:111–143 projects only lexical or
positioned plans; realize/table_cells.rs:110–129 reapplies raw Cell unprepared;
:280–304 terminates the whole snapshot; show_rule_termination.rs:65–70 treats
equal form as fixed point; rules/apply.rs:139–166 revokes globally during callback.
Fields/effects/order/termination/causal error are language; upstream guard/storage
structure is mechanics. Infer no upstream intent. Aliases/two rules/new
generations/hidden content can refute the nominal policy.

### Ratification and exclusive responsibility

The human confirmed manifest
`3375b0a09a4d39d02c2f0bafbcaf3016c6afde27acdf632331da7a98de31e1df`;
`lab/history/construction/records/00_nucleo/materialization/table-cell-isolada-d08b3b-aprovacao-2026-10-07.json`
records 2026-10-07T14:53:19.127601+00:00 and materialization completed
2026-10-07T15:21:50.581533+00:00 with documented scope limits. The original public
field/type gate is satisfied by this receipt, not by the earlier Table package.
Historical candidate-only prose remains archived rather than a fresh stop gate.

Add only pub standalone: Option<Arc<TableCellStandaloneResolved>> to TableCellElem,
keeping source_span/resolved/body/presence/all prior fields. This owner declares
public closed carrier:

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TableCellStandaloneResolved {
    pub visual: Content,
    pub(crate) stage: TableCellStage,
    pub(crate) guards: Vec<crate::entities::show::RuleId>,
}
```


Existing TableCellResolved gains only crate-private guards: Vec<RuleId>; stage
remains Prepared/RecipeApplied/Complete private. No other public field/method/
signature. New constructor both carriers None/no consumed guard. Initially B
prescribed mutually exclusive positioned resolved versus standalone; later C
explicitly permits coexistence only for observed recipe-property projection, with
resolved retained as active physical plan. Do not retain two conflicting current
exclusivity rules. Placing lexical Cell replaces prior insertion state with new
occurrence state according to accepted origin; never fabricate auto x/y0. Prepared
argument aliases preserve guards; fresh constructor inherits no external Cell
guard even same body.

Prepared standalone getters project omitted x/y/fill/align/inset/breakable Auto,
omitted colspan/rowspan Int1, omitted stroke empty dict (not default Stroke/None).
Known lexical value wins, preserving None/Auto/typed values. Body remains lexical
morphology, no Table defaults outside occupancy. Raw unknown/omission follows
presence contract. Visual state exposes no new language property; no generic
carrier Value/string/styles/registry store (Value getter projection only).

Own carrier/default/getters. plain_text/is_empty/map_content/map_text consume
prepared visual, preserving lexical body; helper private to crate without compiler/
render dependency, no Element-trait/constructor public-signature change. Generic
maps' later standalone-first priority is defined separately below; direct visual()
uses active resolved plan before standalone as required by C coexistence.

### Acceptance and incomplete scope

Install only ratified candidates with language tests and valid RED before code;
reseal only changed owners. Require the collections/additional bilateral reviews,
once-only effects, equality/getters/selection, HTML product and real text/styles/
geometry, with causal messages/spans and no erased differences. Validate debug/
release builds and pertinent core/infra/shell suites; crystalline-lint without
errors and no owner V5/V6/V15/V26. Revalidate D08A/B2/B3-A, Raw defaults/blocks,
Equation/math/matrix/query. Preserve the previous tree through snapshot/restricted
diff, without commit/reset/global resealing.

The scope completes intrinsic standalone-Cell defaults and distinguishes alias
from a new show generation, not all Cell behavior. set table.cell/style defaults
remain the separately measured/specifiable Cell-style package; no deadline or
implementation promise follows from that scope-out. General Table defaults/
equality, rich casts/paint/strokes/insets, rows/gutters/multipage spans, PDF/UA
and general nonlocatable policy remain open. Do not fix/normalize the preexisting
empty-file eval warning merely to claim parity.


## Declarative Cell defaults

### Evidence before decision

`lab/results/construction/diagnostics/table-cell-styles-d08b3c-2026-10-07/before.json.gz`
records 58 probes, UTC `2026-10-07T15:24:58.038947+00:00` through
`2026-10-07T15:25:13.727872+00:00`, HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree,
full diffstat/status/sources/argv/cwd/environment. Historical vanilla pin
`9f2b6e8715237cb086899a42873660fe744622e8`, SHA
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`;
Tekt `721b7c2ae63b32b77e71913cc8f86788ce1c8bbc1090f54cd59138ac6ead5bf3`.
isolated/table-set, individual fields, late/local/reuse/spread and rejections
refute empty-target warning/missing defaults. padding-span-one/empty-fixed-rows
require padding spans1; auto-table-fallback preserves explicit Auto as delegation
to Table, not Cell set. show-cell-set-self changes standalone, not positioned
Cell's known fill. Stroke/alignment fold. refuters-before.json.gz adds header,
partial inset, arguments/condition and Table snapshot; alias-inside-table is valid
in vanilla but wrongly recurses in Tekt.

Upstream model/table.rs:734–778 declares fields; layout/grid/resolve.rs:203–278
reads styles before resolving/overwriting positions/fill; model/table.rs:785–800
pins packed padding spans1. Tekt rules/set.rs:23–27 uses target text_str, misses
FieldAccess; table_cells.rs:262–310 uses lexical properties only for placement/
inheritance; grid_placement.rs:80 directly reads x/y/spans; standalone Cell has
no private effective properties. These fields/order/effects/style/errors are
language. Private representation is Tekt's decision, not required Packed/Rust
equality. Refuters: any negative control, lost lexical source, mutated raw value,
repeated effect, lost alias guard or geometry/cache regression.

### Exclusive private effective-property schema

Add only pub(crate) TableCellProperties and pub(crate)
properties: Option<TableCellProperties> to standalone carrier, already private-
field/external-literal restricted. Closed schema x/y/colspan/rowspan Option<usize>,
fill Option<Color>,stroke Option<Stroke>,align Option<Align2D>,
inset Option<Sides<Option<Length>>>,breakable Option<bool>,presence TableCellPresence.
No new/changed public field/method/type/trait signature/TableCellElem shape in
this internal correction. Lexical fields remain source. Prepared standalone getter
delegates private effective properties/intrinsic defaults for omission. Private
projection/copy/placement helpers may share schema; map/clone retains it without
callbacks/render. Keep original Source/body/kind.

### Acceptance and bounded completeness

Internal mapping/parity correction, ADR-0127 continuous: no public contract/
external compatibility/product flag-default/new phase. Props/helpers are private
inside already externally nonconstructible carrier; lexical data unchanged.
L0 first, restricted reseal and RED→GREEN. Require bilateral errors/caret/effects,
source/binary/HEAD/UTC, real geometry, debug/release build/suites/lint without
errors. Preserve prior tree without commit/reset/global reseal. Certified subset:
integers/autos, Color/None/Auto, uniform Stroke/None, alignment, Length/partial
Length inset/Auto, Bool/Auto and existing typed schemas. Gradient/tiling/functions,
side strokes, percentage inset/rich constructor casts, groups/multipage spans/
PDF-UA, positional set/text and general Table defaults remain incomplete and need
measurements/L0 before expansion. Warning fallback proves no certification;
copy no upstream defect.

### Observed spans versus occupied spans

refuters-before/self-set-does-not-change-placement measures vanilla colspan2
for real Cell x0, colspan1 for padding x1/x2. resolve.rs:218–244 captures
occupancy spans without setting content fields; model/table.rs:785–800 sets
padding1. Thus occupancy and later getter are distinct. Keep positioned
TableCellResolved as physical plan. Private standalone carrier may coexist only
for recipe-observed properties, keeping resolved fields and overriding lexically
omitted spans from show-set. Getters/canonicalization read that projection;
complete-plan placement still uses TableCellResolved. Sourceless padding has
lexical spans1 to prevent show-set changes. Guards/stage remain causal, no
public contract/source mutation. Require span-getter RED then GREEN plus geometry/
regressions. stroke-constructor-fold/align-constructor-fold preserve whole
constructor against Cell set; observed colspan2 after placement with1 is no proof
of new occupancy.

### Stroke declaration and rendering folds

extra-first.json.gz (full source/HEAD/state/UTC/binaries) measures two-stroke-sets:
stroke:red then2pt gives2pt+red in vanilla standalone/positioned getter.
stroke-render-inherits-cell-style resolves constructor2pt with Cell red before
Table3pt fallback, although standalone stroke-constructor-fold exposes constructor
2pt alone. model/table.rs:765 has #[fold]; resolve.rs:225–237 uses
stroke.resolve(styles), unlike raw getter. Successive declarations fold specified
components when publishing declarative Value::Stroke, storing no carrier/callback
in Styles. Standalone constructor getter stays whole; positioned resolution folds
constructor with captured Cell base then Table. Add only private
stroke_inherited: Option<Stroke> in TableCellProperties for that causal base,
without baking into lexical getter. ObservationEq includes base; canonicalization
ignores render-derived base. Already resolved Table has base None. Clone/aliases
keep prior base; explicit None wins. RED→GREEN before code; private continuous
correction, no public/phase/default change. Whole-constructor precedence applies
to standalone getter, not positioned stroke fold.

show-fold-before.json.gz/geometry-final.json.gz record lost red in accumulated
show-set and black1pt drawing despite known red2pt. Use private pure Cell
chain_styles to accumulate cell.stroke deltas; borders owner draws known external
plan, never resolved Cell fallback. Keep other styles/guards/order. Helper folds
only declarative Value::Stroke, no carrier/layout/callback/public API/field/phase/
default. Require RED→GREEN and real stroke recollection.

### Table-plan paint independent of default show

physical-before.json.gz in Cell-style diagnostic retains PDF/SVG/source/HEAD/
working tree/UTC/argv/full hashes at historical pin. fill-with-body-recipe and
fill-with-hidden-recipe retain vanilla red rectangle absent Tekt.
stroke-with-body-recipe retains red2pt versus black1pt; default-with-body-recipe
keeps black1pt in both. resolve.rs:218–237,270–282 returns fill/stroke before body
show; typst-layout grid/layouter.rs paints independently of default show. Tekt
cell_body.rs:61–63 gates fill on snapshot survival; borders.rs:43–48 reads lexical
stroke. Earlier refinement measured spacing/ab, not paint: its generalization to
fill/stroke survival is refuted and superseded.

Positioned Cell uses external TableCellResolved fill/stroke/breakable regardless
of it/body/replacement recipe; known None beats every container fallback. Only
default-show-derived inset/alignment depend on surviving prepared_default snapshot,
with geometry/measurement in existing owners. Borders directly consumes known
external stroke; Cell body emits external fill before visual product. Execute no
hidden content; change no lexical source/public API/phase/new render default/
Grid-raw policy. Preserve bounds/coalescing/clipping/tails/effect order/causal state.
Require replaced-product paint RED→GREEN, known-None Cells, real SVG regression,
build/suites/lint/prior-tree preservation.


## Ratified Source empty-body projection

Before deciding, table-origens-d08b3c2-2026-10-07/baseline.json.gz records HEAD/
working tree/UTC/sources/argv/binaries at historical9f2b6e871. Existing carriers'
corresponding transparency forbids Source observable element. Human confirmo
2026-10-07 final record
`lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`;
nested contract.md preapproval history.
regressions/presence empty-body: Source(Empty) stops empty-Sequence body projection.
Getter canonicalizes Empty through Source.body as existing, no presence/cast/
preparation/public-field change; Source never changes previously-sequence body to
func empty. Sole owner/static/no I/O/new signature/trait/flag/eval-layout phase;
C2/C/macro tests/recollections/build/lineage required.

## Ratified compact Value carrier adaptation

Before deciding,
`lab/results/construction/diagnostics/value-compacto-memoria5-2026-10-08/before.json.gz`
and inventory.json freeze this consumer/all Value::Stroke file:line occurrences.
sizes.json measures external Value model272→144 bytes, no RSS proof. New public
carrier belongs only to `00_nucleo/prompts/core/entities/value.md`; this unit
consumes without shared ownership. Earlier implementation gate was satisfied by
human `confirmo` on2026-10-08, manifest
`6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237`;
proposed clauses are current for materialization.

Getter projects owned Stroke via From<Stroke>. Delta fold borrows Stroke references,
replacing only delta Arc with full result/preserving base snapshot. Public
TableCell/stroke remains Option<Stroke>, presence/specified unchanged.
Keep tests/oracles, pertinent RED→GREEN if new isolation behavior exists,
owner-only reseal/revalidation. No semantic/default/phase expansion.


## Change-aware generic reconstruction

Before deciding,
`lab/results/construction/diagnostics/table-local-scaling-followup-2026-10-09/change-aware-red-gate.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/
status/diffstat/UTC/script-fixtures-interposer-CPU-commands/release SHA
`29745a9e80ca43160b34f4807949f00e803070392653df0fc5e214b2f42ac33a`:
local-set-1600 glibc483823 exceeds pre-fixed470000 gate. traversal.rs:25–55,
235–264 rebuilds sequences/carriers/styles/Table/Cell even all callbacks None;
table.rs:85–118/table_cell.rs:308–344 clone metadata/visual contributions. Pinned
realize lib.rs:242–291,353–410 visits borrowed/materializes only changed. Cost
hypothesis, no required vtable/Rust equality under ADR-0107/0026/0109.

Private change-aware helper returns None if visual contribution unchanged. Generic
mapping descent priority standalone.visual, then resolved.visual, else raw body;
never visit hidden origin alongside visual. This mapping priority is distinct
from active visual()'s resolved-first physical plan; preserve documented mismatch
for separate audit, not silently reinterpret either operation. On change clone
Cell/applicable plan keeping presence/stage/guards/props/origin/noncontent fields.
No change lets `00_nucleo/prompts/core/entities/content/traversal.md` reuse original
Arc. Existing Element method/signature uses same implementation, no trait method/
Engine/Cell preparation in entity. Raw/resolved/standalone tests/paired gates
mandatory. Shared pinned map-content-change nucleus owns absence proof; this owner
retains algorithm/field/order/specific-preservation responsibility.
