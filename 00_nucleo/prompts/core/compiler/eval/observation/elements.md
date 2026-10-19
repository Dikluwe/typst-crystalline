# L0 — compiler/eval/observation/elements
Hash do Código: e0ab26b9

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:6f6d86b0ee72d314a84d86b12f9524b2959e075b29f99fcf6d2de9f30a4b338a
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:93054c60b918734522155fc8d1140982c656478ffccbc1ce55b14911309e1a5c
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c
- 00_nucleo/prompts/_nuclei/table/cell-occurrence.toml sha256:bba33b336e651ccd6d41f88b0e16449932d0a986de1ee2cc3c1fdf5c8fb8c6dc
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:f7d3050154813e1ea46c26e235383ae337781a4b28d10ef3248f6eaec96ce221
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf
- 00_nucleo/prompts/_nuclei/table/cell-style-defaults.toml sha256:041abef9e300ff0412d9631dced12480f8e0e6e65fb9cc3733f290e6664f649b
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:6345ad9c825988872fb53b94c574f61e85a7c21074597400c33bd13ae6a3f68d

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/observation/elements.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/observation/elements.rs:11` — `impl ObservationEq for crate::entities::elements::align::AlignElem {`.
- `01_core/src/compiler/eval/observation/elements.rs:19` — `impl ObservationEq for crate::entities::elements::underline::UnderlineElem {`.
- `01_core/src/compiler/eval/observation/elements.rs:29` — `impl ObservationEq for crate::entities::elements::v_space::VSpaceElem {`.
- `01_core/src/compiler/eval/observation/elements.rs:38` — `impl ObservationEq for crate::entities::elements::enum_item::EnumItemElem {`.

## Contract

Own observation of the remaining non-math, non-page element carriers. Compare
variant identity and every causal field, including BlockHeight and ImageElem.alt.
Distinguish absent alt, an empty string and nonempty text. Do not compare
derived frames, execute callbacks/layout or substitute textual names for
identity. Dynamic elements without immutability guarantees remain Unproven.
Nominally exhaustive destructuring must prevent silent field omissions.

BlockHeight compares Relative/Fractional nominal variants and their payloads;
different variants are Different without resolving a reference or allocation.
Image alt remains distinct independently of identical image bytes/geometry.
Boxed observation includes its fractional width carrier.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.


## Ratified semantic SmartQuote observation

### Evidence before decision

smartquote-2026-10-06/{before,fields-before,partial-and-html,proposal-controls}.json.gz
records historical pin `9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA,
HEAD/tree/exact diffstat/UTC/sources/argv/cwd/environment/stdout-stderr. PDF uses
extracted text, HTML DOM/text/diagnostics, not PDF-byte equality. Eval converts
markup quotes to Text, losing double/enabled. Stored quotes keep English glyph
when used with German/French or enabled:false.6'/2" primes become Tekt quotes.
Native show/public name were separately fixed/passing before gate; markup still
not SmartQuote. Empty constructor has empty vanilla fields; double:false known,
omitted double Unset. enabled:false remains SmartQuote. set double:false/filtered
show add positives. Literal HTML works from Text; programmatic SmartQuote rejects.
Literal title renders quotes; script takes string/rejects Content pre-export in
both.

markup_stream.rs:79–128 picks glyph in eval; layout/smartquote.rs alternates flags
without prior text; stdlib/text/smartquote.rs:112–117 erases disabled element;
observation/elements.rs:242–250 compares existing fields. Pinned markup.rs:141–146
creates explicit double; inline/collect.rs:196–211 resolves context/style;
text/smartquote.rs:119–158 separates prime/apostrophe/nesting.

### Ratification, exclusive observation and limits

Human approved2026-10-06: double public bool→Option<bool>, enabled:Option<bool>,
pure helper/context, glyph selection eval→render. Applies only to quotes,
superseding conflicting historical clauses; other ADR/nuclei/proposals intact.
L0/pins/sole owner before materialization RED→GREEN.

Eval retains SmartQuote identity/fields; each target renders glyph. Paged owns its
unit, HTML remains semantic/no paginated quote layout under ADR-0128. One pure
L1 context selector with flow/compilation-local state; no I/O/globals/vtable/
property registry/upstream memoization copy. Tekt stack mechanics distinct from
language/field presence/glyph/style acceptance. Escaping stays HTML owners.
No new mode/flag/unsafe/watch API/math phase.

Exhaustively destructure SmartQuoteElem double/enabled/alternative/quotes.
Observe each nominal Option through ObservationEq, None distinct from Some(true/
false) even if render equal. No callback/render/omitted field.

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

Exhaustively destructure TableCellElem and compare nominal presence plus body/
current values. Identical pixels with different presence cannot prove Equal.
No callback/render.

Require RED→GREEN for body/presence/methods/JSON,
clone/map/observation and span-anchored negatives, build/suites/lint, D08A/D05/D07
and new refuters with exact provenance. The human approved new public fields/
types on 2026-10-07; receipt
`lab/history/construction/records/00_nucleo/materialization/table-cell-presenca-aprovacao-2026-10-07.json`.
This changes no render phase/default. Position/style/padding/show realization is
the separately ratified insertion package, not implicit scope of lexical presence.


## Ratified Table insertion and incomplete Cell scope

The approval receipt `lab/history/construction/records/00_nucleo/materialization/table-cell-realizacao-d08b3-aprovacao-2026-10-07.json` records user confirmation `comfirmo`, manifest `5fec42835e38e9a28ee2ea2714b30135f069f111c140bb5e918a0902a06cc82b`, and materialization with documented metric residuals. This satisfies the original D08B3-A gate; it does not complete the Cell domain or promote the excluded scope. Retain the receipt’s limitations on raw default typography, D08B3-B isolated Cell/fresh reconstruction, and generic Table equality/rich properties.

### Measurement before the proposal

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree. `lab/results/construction/diagnostics/table-cell-realizacao-d08b3-2026-10-07/{before,additional,order,aliases}.json.gz` records pin `9f2b6e8715237cb086899a42873660fe744622e8`, UTC, diffstat/status, sources/binaries, arguments, and results. Oracle binary SHA-256: `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; Tekt binary: `7c0957f45fe40290aa809e3358c9a87979aa1e28058511ad0ee978feea6713a2`.

Vanilla `lab/typst-original/crates/typst-library/src/layout/grid/resolve.rs:204–278` resolves fields, `:1500–1530` rejects conflicts in order, `:1665–1703` creates empty cells, and `:1945–1988` derives breakability. Tekt `grid_placement.rs:91–112` prioritizes explicit placement and `:262–270` knows only `GridCell`; `realize/mod.rs:160–200` visits recipes bottom-up; `table.rs:127–152` discards wrappers without attributes. `order` measures zero cell effects when a table is replaced and zero heading effects when a cell hides it; `aliases` measures `LOCAL a`, `OUTER LOCAL a`, and `NEW a`. These are language observables, not requirements to copy mechanics, bytes, or Rust equality. Unknown fields, hidden effects, or wrong order after a change would refute the hypothesis; cache/pagination equivalence is not inferred.

### Exclusive prepared-carrier observation

Exhaustively destructure Table/Cell/new carriers; observe source_span/rows_explicit/
plans/properties/visual/nominal stages without callback/render. Neither text nor
language Hash/PartialEq proves reuse.

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

Exhaustively destructure TableCellElem/TableCellStandaloneResolved/TableCellResolved.
Observe presence/visual/stage/causal guards plus all other fields. ObservationEq
for reuse must retain state language equality ignores. No text/pixel/Hash proof
or callback/render.

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

### Exclusive private-property observation

Exhaustively destructure standalone carrier with properties/closed private schema.
Observe every value/presence/edge/causal state without callback/layout. Reuse proof
is not language equality; silently omitting private data is forbidden.

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
stroke:red then 2pt gives 2pt+red in vanilla standalone/positioned getter.
stroke-render-inherits-cell-style resolves constructor 2pt with Cell red before
Table 3pt fallback, although standalone stroke-constructor-fold exposes constructor
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

## Ratified Raw lexical presence

content-serialization-2026-10-14/{before,raw-controls}/results.json.gz records
HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree/UTC/sources/
binaries at historical pin9f2b6e871. raw.rs:21–25 loses omission;
native_raw markup.rs:78–136 collapses absent/false/none. Pin fields/equality/
encoders distinguish them: language semantics/morphology, not Rust mechanics.

ObservationEq includes set_fields with exhaustive destructuring, distinguishing
presence even with equal value. Approved manifest
`b799576490d06bb6fde27c32c78572b63e1279b599d4684629fe4484bc4e0a45`,
`lab/history/construction/records/00_nucleo/materialization/raw-presenca-2026-10-14/approval-receipt.json`.
Completes only Raw's earlier incomplete presence, superseding no-mask clauses.
Keep other contracts/no new phase/default/render; require RED→GREEN/suite/build/
lineage.

## Approved typed-transform observation

transformacoes-2026-10-14/results.json.gz/supplemental controls record identity/
presence lost in matrix/body. Premigration source/owner in
`lab/history/construction/records/00_nucleo/materialization/transformacoes-tipadas-2026-10-14/base-files.json.gz`,
hashes/lines consumers.json. Approved manifest
`960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d`
authorizes new representation/layout resolution. Language semantics/morphology
versus Rust/reconstruction mechanics.

Conservative Transform comparison distinguishes variant/presence/every typed
value before body observation. Matrix compares all coefficients. Do not equate
operations via final matrix. RED→GREEN/dependent revalidation; build closes no
X1–X5.
