# L0 — compiler/eval/bindings/field_access/content_methods
Hash do Código: eba9d458



Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/caption-projection.toml sha256:b57c37587932a65caedce0b0df5691463ad336b90126a978ea88a7c369432738
- 00_nucleo/prompts/_nuclei/content/sequence-realization.toml sha256:14ece1bcb7901da8d7e5048a82cd02ef9eafa258a328900a99636ff4ef74a843
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/math/equation-constructor-origin.toml sha256:947456f05619642c93f67721453e4ad5577557679dbc422a20a48f397ae97ded
- 00_nucleo/prompts/_nuclei/math/matrix-morphology.toml sha256:dced4bcb1e45b09423bb3b9739af4da3cd440d500d6a620e08e13219236d1735
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:93054c60b918734522155fc8d1140982c656478ffccbc1ce55b14911309e1a5c
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:f7d3050154813e1ea46c26e235383ae337781a4b28d10ef3248f6eaec96ce221
- 00_nucleo/prompts/_nuclei/table/cell-projection.toml sha256:8388731d50379499612641a70a85695c0e7139ec605289c29161e12086abd78a
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:6345ad9c825988872fb53b94c574f61e85a7c21074597400c33bd13ae6a3f68d

## Ownership and extraction evidence

Layer L1. Sole productive consumer:
`01_core/src/compiler/eval/bindings/field_access/content_methods.rs`. At clean
HEAD `ba5a4c7d305ed1444660d9eb80862f51b417efb6`, field_access.rs:1165–1589
contains extracted behavior; SHA/time/initial state in
`lab/results/construction/diagnostics/atomizacao-field-access-2026-09-30-before.json.gz`.

## Exclusive Content/LocatedContent projection

Read set fields/methods/static type wrappers preserving parent field_access
signatures/semantics, static same-layer L1/no I/O/mutable globals. Preserve L1/
field-access tests/crystalline-lint valid ownership/hashes; no inferred speedup.
Presence projects internal carriers, keeping undeclared versus unset distinct;
carrier is no native element. func/has/at/fields/location project body/visual.
Prepared occurrence Location is observable by recipe: never force none when
carrier supplies it. Preserve real native function/original fields.

## Closed reflection boundaries and causal Equation fields

### Evidence before decision

At HEAD `5e1a01066b9eef0eb3c478e8b49e15e1da15b0b9`, the read-only audit in
`lab/results/v16-corrections-2026-10-19/batch-e/agents/reflection/` records the
source/L0 hashes, UTC and working-tree state. `content_methods.rs:58–81` and
`:283–288` recognize styled Equation fields through the private recognizer at
`:318–327`, which only crosses Styled. The already ratified getter recognizer
in `entities/content/fields.rs:9–16` crosses Source, ShowRecipe, Located and
Realized as well. Thus the getter fallback can retain a nested Equation value
while the candidate list drops its key from `fields()`. This is a source-derived
prediction pending the local functional RED, not a completed test or a fresh
bilateral oracle result. The getter owner's closed-projection clause explicitly
requires this causal transparency and excludes other semantic containers.

The candidate fallback at `content_methods.rs:307` also groups richer, unknown
and fieldless families under body. Getter availability alone does not establish
lexical presence: Box body is deliberately excluded; OutlineEntry.fill returns
a value even without its explicit-presence flag. The native catalogue at
`value_methods/element_where.rs:64–587` admits identities beyond the reflection
map at `content_methods.rs:452–500`; admission is not a proof of constructor
identity retained by every Content variant. Shape and Matrix transform cannot
recover native identity from geometry. Preserve these incomplete scopes.

Current-pin measurements in the adjacent `agents/language/receipt.json` refute
unrestricted Styled-to-Equation transparency: an external
`text(fill: red, math.equation(numbering: "(1)", $x$))` must not expose Equation
body/block/numbering. `stdlib/structural/math/equation.rs:164–183` presently
transports constructor options in an unmarked Styled, while text constructor
`:275` creates an ordinary Styled. The producer has no Equation constructor
marker; property names alone cannot establish causal origin. The existing
Heading marker at `entities/content/fields.rs:135–153` is a bounded precedent,
not authorization to infer Equation identity. Resolving provenance requires
the producer and shared observation owners before materialization; retain this
refuter and do not present a carrier-only fix as complete Styled parity.

### Exclusive responsibility

Make the four Content decisions in candidate-field selection, Equation
recognition, element-function projection and method-receiver projection closed
and nominal. Keep payload decisions inside the relevant nominal arm, including
the rejected Styled Heading/Equation guards. Every future Content variant must
require a decision at each boundary; do not move an open default into a helper,
`if let`, or a boolean classifier. Existing `ContentField` filtering must name
Set, Unset and Undeclared explicitly. The separate getter-delegation fallback
in content_field must likewise retain a reviewed nominal complement if touched;
it is not permission to turn unavailable fields into known defaults.

Equation recognition must distinguish constructor transport from ordinary
semantic Styled before recursion is broadened. Within proven constructor
transport, the only transparent carriers are Source/ShowRecipe/Located.body
and Realized.visual. Never inspect
Realized.original, execute a recipe, flatten Sequence, or cross Strong, Emph,
ordinary semantic containers or arbitrary descendants. This completes the
existing carrier contract for strict fields/has/at; it does not change ordinary
Styled opacity, including an ordinary Styled enclosing an Equation. The
Sequence clause's opacity concerns Styled child-schema
exposure, not an exception to Source's required language transparency.

Accept a Styled constructor only if its last direct `equation.constructor`
value is Bool(true) and its body reaches Equation through causal carriers or
other valid marked constructor wrappers. Unmarked, false-marker, wrong-type and
marked non-Equation wrappers remain opaque. Check the outer Styled itself for
field candidates, field presence and native projection: a marked descendant
cannot authorize an ordinary enclosing style. Bare Equation and proven marked
constructor transport project the existing real native_math_equation; function
projection through carriers observes body/visual. The identity owner supplies
the matching public name. Use private local bounded classification, no new
public helper or signature. Keep the causal marker out of reflected fields.

The nucleus's external Styled child/styles schema remains an intended language
contract whose strict fields/has/at implementation is incomplete here. This slice
repairs Equation leakage without certifying legacy empty fields as parity;
negative controls reject Equation keys and do not require an empty dictionary.
A later ordinary Styled schema repair must remain possible without weakening
these controls.

### Canonical Equation native metadata

The current-pin candidate replay at
`lab/results/v16-corrections-2026-10-19/batch-e/agents/language/candidate-replay/receipt.json`
records `eq-native-equality` and `eq-alias` at
2026-10-19T19:15:13.247393+00:00 and 2026-10-19T19:15:13.382631+00:00:
both remain false in the candidate and true in the oracle after callback
callability was repaired. The receipt retains exact HEAD, tree diff, source and
binary provenance. `stdlib/structural/math.rs:121–124` registers the existing
native with canonical metadata name `math.equation`; this owner's final
Func::native construction still supplies public element name equation.
`eval/operators/equality.rs:23–60` performs language equality and
`eval/repr/value.rs:58–69` already prints the terminal name component.
Therefore callback-address agreement alone did not complete observable native
identity. This is language equality evidence, not a change to global equality.

For an Equation recognized by the existing bounded causal classifier, preserve
the registered native's canonical metadata name math.equation when constructing
the projected Func. Determine this from nominal Equation/valid marked transport,
never from an observed name, arbitrary option keys or geometry. Other families
retain their existing metadata; public repr remains equation through the existing
formatter. No catalogue, global Func equality, public API or constructor change.
Before code, add a failing language-operator test comparing projected bare,
marked and carrier Equation functions with the real math module's native and
its alias, retaining repr and unrelated-function/ordinary-Styled controls.
Use continuous internal ADR-0127 flow with L0 first and parent-controlled RED,
resealing and revalidation. The earlier callback-only tests remain controls.

### Known fields retained by their entities

The adjacent language `supplement-receipt.json` measures Figure's known caption,
Metadata.value and OutlineEntry's lexical fill against the current pin.
Figure getter at `entities/elements/figure.rs:76–88` retains present Caption;
Metadata getter at `entities/content/fields.rs:316–317` retains arbitrary Value.
OutlineEntry getter at `elements/outline_entry.rs:143–147` returns fill even
when its existing fill_set is false; its producer at
`stdlib/structural/outline.rs:64–96` records that distinction. These are
language field-loss and presence measurements, not a general reflection schema.

Enumerate Figure candidates body,caption; publish caption only when getter-known,
including Empty Caption body. Absent caption stays absent. Explicit caption:none
remains unrepresentable and is not certified or repaired by inferred presence.
Enumerate Metadata.value, retaining its Value without a body. Enumerate
OutlineEntry level,element,fill: level and element are known, fill is Set only
when fill_set is true (including explicit none), otherwise Unset; other names
are Undeclared. Keep snapshots authoritative and preserve the separate semantic
getter's resolved fill policy. No public field or signature changes are needed.

PdfArtifact.kind is intentionally not promoted in this correction. Although an
explicit kind is lost from fields, `stdlib/pdf.rs:331–355` collapses omitted kind
and explicit other in the same carrier. Keep that missing-presence debt visible;
never infer lexical presence from resolved Other or silently add a public bit.

Preserve candidate order and all Set/Unset/Undeclared distinctions, explicit
none/false/auto, outer Equation override precedence and existing omissions.
Preserve Box body exclusion, Matrix transform's empty schema, marked Heading,
Raw, Cell, SmartQuote, Matrix and immediate Sequence children. Do not promote
unmeasured rich schemas or infer callable identity from a name or geometry.
Any additional native mapping requires independently measured current-pin
language evidence and its owning constructor contract before specification.

The method pre-pass projects only the four causal carriers. Explicit supplied
Location wins over a Located carrier; a present fields snapshot is authoritative,
including absent keys and an empty snapshot. Native receivers retain ordinary
dispatch and argument validation. Function projection does not execute callbacks.

### Acceptance and limits

Continuous internal correction under ADR-0127: no public type, field, trait,
signature, default mode, phase or compatibility change. L0 first, then a failing
method-level nested-carrier Equation fields test and passing preservation controls
before implementation. Check ordered keys and values through fields/has/at,
explicit none, nested override precedence, ordinary Styled and Sequence opacity,
Realized visual selection, authoritative snapshots and Location precedence.
Preserve existing real-native identities through carriers and unrelated native
function distinctions. Exhaustiveness probes verify domain protection only;
they do not establish language parity. No suppression or reduced expectations.
Parent-controlled ownership/nucleus preflight precedes restricted resealing;
build, workspace, strict Clippy/Rustdoc, formatting and lineage gates remain.

## Ratified semantic SmartQuote fields

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

Declared SmartQuote fields are double,enabled,alternative,quotes. Some is Set,
None Unset, out-of-schema Undeclared. fields/has/at expose Set only. Omitted
.double outside context gives measured Unset error, no fabricated bool. Explicit
known fields return correct values. Keep Source/span/captured Some/other-element
fields. Public quotes Value uses existing canonical representation.

## Matrix morphology and strict field access

Before deciding, matrix-morfologia-2026-10-06/before.json.gz records HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/diff/sources/
binaries/cases at historical `9f2b6e8715237cb086899a42873660fe744622e8`.
Raw/query mat prints{func:math.matrix} without rows; getter/methods lose fields,
func equality false. Pinned matrix.rs:91–221 declares fields/rows. Local
math_matrix.rs:45–52 retains rows/explicit, but Element.get_field default;
content_methods.rs:151–191 omits mat; cli.rs:1154–1348 falls back body.
Language morphology/semantics, not Rust/PDF bytes or inferred intent.

rows is Set. Declared delim/align/augment/row-gap/column-gap are getter Set when
explicit, else Unset; rest Undeclared. fields order delim,align,augment,row-gap,
column-gap,rows reuses getter values. func uses real native_math_mat via private
bridge, not stub/name identity. This projection slice adds no mat selector/show/
where; later catalogue owner's scope remains separate.
RED→GREEN/bilateral fields-has-at-func/JSON-query/padding/
presence; build/lint/D05-D06-query regressions. Same public type/trait/signature/
phase, no mat-selector/render/general Styled/unmeasured-element certification.


## Existing Cell projection mapping

Before deciding, table-cell-realizacao-2026-10-07/before.json.gz retains HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/exact diffstat/
source-binary hashes/argv/cwd/results at historical pin9f2b6e871. Vanilla
table.rs:734–778 declares body/properties; local table_cell.rs:62–109 lacks getter;
content/identity.rs:107 uses table.cell; content_methods.rs:226–254 lacks real
function projection; show.rs:300–320 omits native. body/fields/func-eq/explicit
show refute parity. Language semantics/morphology, not Rust/PDF equality or
structure-inferred intent.

Ten declared fields use getter Set/Unset, ordered body,x,y,colspan,rowspan,inset,
align,fill,stroke,breakable. func projects real native_table_cell.
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

Set/Unset follows entity strict getter. Keep ten declared fields/order above;
func real native. No inherited default fabricates Set.

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

### Exclusive Table identity and prepared fields

order/table-replacement-before-cells/show-table-body require native Table identity
for selection. content_methods.rs:240–255 has Cell, not Table. Add Content::Table
→native_table to content_elem_func without constructor execution. fields/has/at
keep owner projection, no generic Table defaults. Resolved Cell projects known
properties; raw omission remains Unset, preserving snapshots/order.

### Ratified gate, acceptance, and explicit scope split

The ratified package adds five public fields, two public carriers, and one new public facade, and changes the logical phase before Cell show. The receipt above establishes ratification for this scope. Continue to require RED→GREEN, suites/build/lint/restricted preservation, and D08A/B2, D05/D06/D07/query collections. Acceptance is language, effect, and actual geometry behavior; byte equality or whitespace normalization cannot close render acceptance.

This first package D08B3-A is incomplete in the Cell domain: D08B3-B is to complete isolated Cell defaults (`raw-cell-no-placement`/`outside-fields`) and the separate decision on fresh generation by `show-cell-reconstruct`. Generic Table defaults/fields (`table-root-equality`), rich styles/functions/paint/strokes/insets, rich rows/gutters, multi-page spans, and PDF/UA conformance remain outside certification. Do not change additional legacy D08B2 casts, syntax, flags, or global non-locatable policy in this package.



## Ratified Source transparency

Before deciding, table-origens-d08b3c2-2026-10-07/baseline.json.gz retains HEAD/
working tree/UTC/sources/argv/binaries at historical pin9f2b6e871. Existing carrier
operations provide transparency; no observable Source element. Human confirmo
2026-10-07 final record
`lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`;
nested contract.md is preapproval history. Fields/func/methods observe Source.body without losing origin or converting Styled
to Cell.
Sole owner/static/no I/O/new signature/trait/flag/eval-layout phase;
C2/C/macro tests/recollections/build/lineage before certification.



## Ratified Raw lexical presence

content-serialization-2026-10-14/{before,raw-controls}/results.json.gz records
HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree/UTC/sources/
binaries at historical pin9f2b6e871. raw.rs:21–25 loses omission;
native_raw markup.rs:78–136 collapses absent/false/none. Pin fields/equality/
encoders distinguish them: language semantics/morphology, not Rust mechanics.

content_field recognizes text/block/lang as Set versus Unset, other names
Undeclared. content_set_fields lists known text,block,lang in that order.
Approved manifest
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

Project typed contract's ordered fields, distinguishing declared omission from
nonexistence. func links correct native per operation. Matrix keeps generic
noncallable fallback. RED→GREEN/dependent revalidation; build closes no
X1–X5.

## Existing caption projection completion

caption-shape-before.json/caption-after.json in
`lab/results/construction/diagnostics/tekt-v16-figure-casts-2026-10-17`
retain sources/UTC/binaries/HEAD-working tree from validation-result.json and
historical pin9f2b6e871. Projection makes carrier but CLI only func:figure.caption,
missing body/getters/fields; json.encode same. Kind metadata rejects valid caption
function. Pinned figure.rs:510 declares Locatable; :511–594 public position/
separator/body and synthesized kind/supplement/numbering/counter, internal
figure_location. Synthesize none here; element function existed since approved
P1473.

Candidate FigureCaption fields ordered position,separator,body, exposing known
getters only. content_elem_func returns native_figure_caption, retaining existing
internal name, no generic noncallable callback/name-equality proof. Change no
Unset/synthesis policy for missing fields in this slice.

Continuous internal parity ADR-0127, no public field/
trait/signature/phase/mode/default. L0 before tests/code, real getter/metadata/
encoder/CLI RED/GREEN/external corpus without relaxed body/name expectations.
All batch gates, 12 GiB/zero swap/jobs1, owner-only reseal after V15/V26. FG1/SY1
remain incomplete per follow-ups.md; no pending API authorized.


## Measured additional presence and text projection

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, state/UTC/sources in
lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz. Pin
`bd5ebec285505889e8ff1d4f68804ab147a699f3`; before-pdf-probe.json.gz freezes
bilateral field/admission/callback/init. Prior paridade-novo-vanilla inventory
E02–E04 records carrier limits/discriminating sources, not dump/Rust structure
acceptance. Language observables.

Text presence remains Set(text). H/V amount Set, weak Set iff weak_explicit,
else Unset. Block body Set; width/height Set iff transported Some, else Unset.
Box exposes only Option width/height including fractional width, never body as
specified below. fields order width,height,body for supported containers and
amount,weak for spaces, emitting Set only. Non-Equation Styled never projects
child schema: retain own behavior/Equation guard/authoritative snapshots. Other
container cosmetics lacking faithful presence remain excluded; infer no explicit
false/none from defaults or total-constructor parity.

L0 first/functional RED/bilateral GREEN; no public/
product-default/phase/compatibility. Earlier State proposal independent, later
separately ratified reader contract grants no authority here.1:1/pins valid before
restricted reseal. Build/full suite/strict Clippy/Rustdoc/fmt/direct-reverse lineage,
MemoryMax 12 GiB/High 8 GiB/swap0/jobs1/heavy sequential. debug0/incremental0 only
validation environment, measured consumption/no inferred product gain.


### Optional Box body limitation and partial supersession

box_elem.rs:29–45 accepts omitted body as Empty, indistinguishable from explicit
empty body. Therefore no promotion of Box body in has/at/fields or strict public
path: preserve prior absence until own presence-transport contract. Expose only
width/height already Option. Block requires body and can expose it. No
box().fields() regression or Empty-based presence inference. This supersedes
earlier generic Box/Block body-Set clause only for Box.

## Ratified Heading finalization before queries

### Evidence and approval

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree;
`final-freeze.json.gz` and `target-controls.json.gz` in
`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17`
record UTC/diffstat/sources/argv. Pin:
`bd5ebec285505889e8ff1d4f68804ab147a699f3`.
`heading-raw-constructor-func` passes on baseline/vanilla and fails on the
candidate because transport `Styled` changes `func`. Query also observes a
temporary causal wrapper; HTML ignores a panic callback. Sources:
`walk.rs:930–945`, `pipeline/paged.rs:419–427`, `pipeline/html.rs:101–106`
prepare queries through the pure walk. These are language values/errors, not a
requirement to copy vanilla's structure or algorithm. The owner confirmed A1-H2
on 2026-10-17 in
`lab/history/construction/records/00_nucleo/materialization/heading-supplement-runtime-2026-10-17/review.md`,
SHA-256 `66739b13596013f4343bdc3ffacbdb9081151e3d9863e5d962e8ff13fd68775c`.

### Exclusive raw Heading projection

Use nominal constructor_heading projection for func()==native_heading and
presence/at/has/fields, including explicit supplement/numbering. Raw order level,
depth,numbering,supplement,outlined,bookmarked,body; Set only, body last. Outside
transported properties delegate base Heading. Ordinary text Styled/external
wrapper is not Heading. Some snapshots authoritative, never reinfer from caller
styles.

Require H1–H8 in the ratified review, with bilateral RED before code. No public
contract beyond the Heading facade is authorized by H2; A1-S2 separately covers
five State helpers. No new public fields. Ordinary `Styled` and other callbacks
retain their boundaries. Require workspace/doctests/build/Clippy/Rustdoc, fmt,
V1/V5/V15/V26/reverse lineage and no V16 increase; use
12GiB/High 8 GiB/swap0/jobs1 with heavy commands sequential. RAM gain requires
measurement, not intention.


## Sequence and empty reflection

### Evidence before decision

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree;
sequence-reflection-2026-10-17/before.json.gz/before-sources.json.gz retain UTC/
diffstat/sources/binary SHA, vanilla
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Pinned content/mod.rs:724–728
requires children, :538–585 projects has/at/fields. empty-fields/func/children,
sequence-fields/children/single/for refute missing fields/empty name in Tekt.
fields.rs:317–322 puts Empty/Sequence in fieldless complement; identity.rs:24 uses
empty; content_methods.rs:242 fallback body-only. Language morphology/semantics,
no Rust-shape demand. Empty Text stays Text; ordinary Styled opaque.

### Exclusive reflection and scoped acceptance

Empty/Sequence have only candidate children, getter-owned Set even empty;
body/missing Undeclared. Keep Source/Styled opacity. func uses bridge/native
constructor below, public sequence supplied by elem_name; no name-based function
classification/selector-catalogue expansion here.

Initial reflection scope left Sequence construction/admission and generic invalid-
method diagnostic for separately measured owners. Later constructor contract below
completes its named subset, not universal sequence/func parity. Before.json.gz
retains refuters. No new public Rust field/type/trait/signature/phase/mode.
L0 before code/RED→GREEN/regressions/workspace/build/strict Clippy-Rustdoc/
ownership-nuclei/direct-reverse lineage/no V16 increase. Reseal after V15/V26;
MemoryMax 12 GiB/High 8 GiB/swap0/jobs1 sequential, record peak/no inferred gain.

## Sequence construction/selection facade

### Evidence before decision

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree.
`lab/results/construction/diagnostics/sequence-constructor-2026-10-17/{before,expanded}.json.gz`
and before-sources.json.gz retain UTC/diffstat/status/binaries/sources/bilateral
cases at `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Vanilla
content/mod.rs:724–728 requires children; value.rs:644–650 casts Content/string/
symbol/none; typst-macros/src/elem.rs:573–604 generates constructor. Tekt
field_access/content_methods.rs:358–410 still exposes noncallable fallback;
element_where catalogue/selector_matching.rs:272–337 lack Sequence identity;
rules/set.rs:55–76 rejects only transform With, misses Sequence set. Language
semantics/morphology/diagnostics, not required upstream storage. Earlier pending
children() message already matches current baseline: fixed getter lets existing
owner produce correct error. New evidence rectifies debt; edit no diagnostic code.

### Exclusive Sequence constructor

Empty/Sequence func returns exclusive crate-internal native_sequence, current
four-argument ABI. Required positional children: validate Array, cast each child
Content/LocatedContent/string/symbol/none through existing Content cast. Preserve
immediate children/order/zero-one-child Sequence without Content::sequence
normalization. Borrow arguments, no analysis-array clone; clone only resulting
Content, execute no child callbacks.

Named-only children: ``the argument `children` is positional`` with
``try removing `children:` `` hint. Missing: `missing argument: children`.
Invalid array: `expected array, found T`; invalid child: `expected content, found T`.
Argument/child cast precedes extras. First causal extra: `unexpected argument[: nome]`
at full occurrence; types use first positional value_span, missing args.span.
With calls work through existing route. func always real callback, no generic
fallback/global binding.

Completes earlier R1-named R2 responsibility. Continuous
internal table/composition/callback correction, no new external field/trait/Rust
signature/default mode/phase/compatibility break. L0 before tests/code, functional
RED/GREEN/bilateral corpus. Certify no global show guards/all casts-elements/
universal render parity. Residual refuters stay reportable with owner/L0 before
intervention; change no tests to hide them.

Require separate build/workspace/doctests, strict Clippy/Rustdoc/fmt,
V1/V5/V15/V26/reverse/differential SARIF without new error-warning. Prescribed
nominal V19 allowed visibly without suppression, alternatives tested. Heavy tasks
sequential, MemoryMax 12 GiB/High 8 GiB/swap0/jobs1; debug0/incremental0 in validation
environment only. Reseal after valid V15/V26/exact provenance.

## Sequence realization evidence and approval

before/expanded/controls.json.gz and candidate.log in
`lab/results/construction/diagnostics/sequence-constructor-2026-10-17` fix HEAD
`b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted working tree,
UTC/diffstat/sources/binaries and vanilla pin
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Eight initial REDs; candidate
passed five, with show/recursion unresolved. markup_stream.rs:240–260 flattens
before selecting Sequence; realize/mod.rs:170–210 visits children before recipe;
apply.rs:140–170 delegates nonlocatable elements to fixed point. Vanilla accepts
alias and rejects a new Sequence with the same children. These are language
effects/scope/values/diagnostics.

Owner approved
`lab/history/construction/records/00_nucleo/materialization/sequence-realization-2026-10-17/review.md`,
manifest `46132cb53898f2b126a08bf3b0495ed4c05b531c380846bccbd3d0906477c327`,
on 2026-10-17. ADR-0127 §2.3 gate covers realization order of this boundary
before children/Table/math, not a new public contract.


### Exclusive child-cast barrier and acceptance

Nominally name34 nonconvertible Value variants sharing `expected content, found T`,
so future variants require decision. No allow/expect. Empty Text remains Text,
None Empty. Symbol acceptance here is only existing glyph cast; distinct Symbol
identity requires own contract, not Text repr/equality inference.

Ratified S1–S20 require precode RED/bilateral GREEN/regressions, no reduced
expectation for mechanics. Valid1:1/pins before reseal, separate build/workspace/
doctests/strict Clippy-Rustdoc/fmt/direct-reverse lineage/SARIF no new error-warning.
12GiB/High 8 GiB/swap0/jobs1/heavy sequential; no publication/release.

## Completion boundaries and unresolved evidence

r2s-scope.json.gz/text-before.json.gz preserve the incomplete morphology/textual
scope after Sequence approval. Symbol cast identity/equality measured text/true
in Tekt versus symbol/false vanilla; a public variant needs its own gate, not
Sequence authorization. Markup must preserve interpolated Sequence as immediate
ContentBlock child without Sequence rule, previously flattened. Textual matching
must measure/correct cross-scope literals C vs B in its own owner. Generic show
traces for emitted children need their diagnostic owner. Historical package
names R3-Symbol/R3-Markup/R3-Textual/R3-Diagnostics identify these completion
scopes only; they are no L0 or authorization for additional code. Do not label
the differences accepted mechanics or proven parity. Later scoped ratification
does not silently certify every recorded debt.
