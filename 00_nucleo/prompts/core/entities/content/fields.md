# L0 — entities/content/fields
Hash do Código: fec2f98c


**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/fields.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/caption-projection.toml sha256:b57c37587932a65caedce0b0df5691463ad336b90126a978ea88a7c369432738
- 00_nucleo/prompts/_nuclei/content/projection-exhaustiveness.toml sha256:c7269dd3cf9d8084731457973737db5df87a3d428b80ab5701593deaecdea87e
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/math/equation-constructor-origin.toml sha256:947456f05619642c93f67721453e4ad5577557679dbc422a20a48f397ae97ded
- 00_nucleo/prompts/_nuclei/math/matrix-morphology.toml sha256:dced4bcb1e45b09423bb3b9739af4da3cd440d500d6a620e08e13219236d1735
- 00_nucleo/prompts/_nuclei/raw-field-presence.toml sha256:93054c60b918734522155fc8d1140982c656478ffccbc1ce55b14911309e1a5c
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:f7d3050154813e1ea46c26e235383ae337781a4b28d10ef3248f6eaec96ce221
- 00_nucleo/prompts/_nuclei/table/cell-projection.toml sha256:8388731d50379499612641a70a85695c0e7139ec605289c29161e12086abd78a
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:6345ad9c825988872fb53b94c574f61e85a7c21074597400c33bd13ae6a3f68d


## Contract

Own `Content::get_field` and its transparent styled-equation lookup. Preserve
field presence, omission, style precedence, and delegation to element owners.
Unsupported fields return `None`.

## Acceptance

Existing field-access and show-rule tests pass. The implementation remains a
closed static match and does not introduce a property registry.


## Transparent carrier projection

ShowRecipe/Located delegate field reads to body, Realized to visual, no new
public element. Source follows ratified transparency below. Ordinary Styled
remains opaque except specifically contracted Equation/marked Heading paths.

## Causal Equation constructor projection

### Evidence before decision

At HEAD `5e1a01066b9eef0eb3c478e8b49e15e1da15b0b9`,
`lab/results/v16-corrections-2026-10-19/batch-e/agents/language/receipt.json`
records current-pin source, binary, time and working-tree provenance for an
ordinary `text(fill: red, math.equation(numbering: "(1)", $x$))` exposing
Equation properties that the oracle keeps opaque. The private recognizer here
at `fields.rs:9–16` currently crosses any Styled. The Equation native at
`stdlib/structural/math/equation.rs:164–183` and text native at
`stdlib/text/constructor.rs:275` both create Styled, so neither an Equation
descendant nor an equation.* property name proves constructor provenance.
The existing Heading marker at `fields.rs:135–153` demonstrates an internal
causal marker without adding public fields. This evidence refutes only the
unrestricted Styled boundary; carrier transparency and outer override precedence
inside genuine constructor transport remain required.

### Exclusive observation and limits

Recognize the producer's private `equation.constructor` marker only when its
direct delta's last value is Bool(true) and its body leads to Equation through
the four causal carriers or other valid marked constructor wrappers. Source,
ShowRecipe and Located delegate body; Realized delegates visual, never original.
Unmarked Styled, including a wrapper containing equation.* values, is opaque.
A false or wrongly typed marker, a marked non-Equation body, Sequence, Strong
and other semantic containers do not confer Equation identity or fields.
Do not execute recipes or add a public helper, API, field or signature.

Both the recursive recognizer and the outer Styled getter admission must check
this causal boundary. A marked child does not make an unmarked parent
transparent. Preserve the four override keys, last-write/outer-first precedence,
explicit none, fallback to inner marked transport, and ordinary unknown-field
absence. The marker itself is private and never a reflected field. Keep the
closed Content match and explicit non-Equation nominal complement; string field
names remain open. This supersedes only the former unrestricted Styled recursion
and synthetic fixtures that lacked causal provenance, not the tested carrier,
precedence or non-Equation invariants.

Update existing synthetic constructor fixtures with the explicit marker and add
unmarked/false-marker/wrong-type/marked-non-Equation/outer-semantic-Styled REDs
before code. Functional acceptance observes field values and absence, not helper
output or Rust representation. Continuous internal ADR-0127 correction with L0
first, precode RED, parent-controlled shared-nucleus and lineage resealing, then
the existing build/workspace/strict Clippy/Rustdoc/fmt gates. Full ordinary Styled
child/styles reflection is not introduced by this bounded correction.

## Ratified semantic SmartQuote getter

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

Classify SmartQuote through static match. Semantic-matching getter retains default
double/enabled true, alternative false, quotes Auto where filter needs defaults,
without mutating set fields. Strict presence belongs to
`00_nucleo/prompts/core/compiler/eval/bindings/field_access/content_methods.md`,
never inferred from default. quotes projects existing canonical Value types:
Auto/dict single-double/array<string> pairs, no PropMap/registry. Validate empty-
constructor filtered show against oracle. New style/filter counterexample requires
measurement/L0 before code, no Unknown→false inference/entity mutation.

Pinned smartquote.rs:394 converts custom quotes dictionary double,single;
quotes-key-order-red.json measures keys() that order versus Tekt single,double.
keys is language, JSON bytes not gate. Canonical custom quotes inserts double
before single, keeps Auto for omitted member/pair arrays. No public schema change.

## Matrix morphology and strict field access

Before deciding, matrix-morfologia-2026-10-06/before.json.gz records HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/diff/sources/
binaries/cases at historical `9f2b6e8715237cb086899a42873660fe744622e8`.
Raw/query mat prints{func:math.matrix} without rows; getter/methods lose fields,
func equality false. Pinned matrix.rs:91–221 declares fields/rows. Local
math_matrix.rs:45–52 retains rows/explicit, but Element.get_field default;
content_methods.rs:151–191 omits mat; cli.rs:1154–1348 falls back body.
Language morphology/semantics, not Rust/PDF bytes or inferred intent.

Delegate Content::MathMatrix.get_field to MathMatrixElem owner, no matrix algorithm
here. Causal wrappers/snapshots keep contracts, fabricate no presence.
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

Delegate TableCell getter to its entity owner, no second Cell algorithm.
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

Delegate TableCell getter, never interpret presence through another hub algorithm.

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

### Exclusive prepared-field delegation

Delegate Cell getter; resolved known values not recalculated here. Wrappers/
snapshot remain authoritative.

### Ratified gate, acceptance, and explicit scope split

The ratified package adds five public fields, two public carriers, and one new public facade, and changes the logical phase before Cell show. The receipt above establishes ratification for this scope. Continue to require RED→GREEN, suites/build/lint/restricted preservation, and D08A/B2, D05/D06/D07/query collections. Acceptance is language, effect, and actual geometry behavior; byte equality or whitespace normalization cannot close render acceptance.

This first package D08B3-A is incomplete in the Cell domain: D08B3-B is to complete isolated Cell defaults (`raw-cell-no-placement`/`outside-fields`) and the separate decision on fresh generation by `show-cell-reconstruct`. Generic Table defaults/fields (`table-root-equality`), rich styles/functions/paint/strokes/insets, rich rows/gutters, multi-page spans, and PDF/UA conformance remain outside certification. Do not change additional legacy D08B2 casts, syntax, flags, or global non-locatable policy in this package.



## Ratified Source transparency

Before deciding, table-origens-d08b3c2-2026-10-07/baseline.json.gz retains HEAD/
working tree/UTC/sources/argv/binaries at historical pin9f2b6e871. Existing carrier
operations provide transparency; no observable Source element. Human confirmo
2026-10-07 final record
`lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`;
nested contract.md is preapproval history. Source delegates body transparently, never span as language identity/recipe
execution. Segmentation retains per-segment origin where applicable.
Sole owner/static/no I/O/new signature/trait/flag/eval-layout phase;
C2/C/macro tests/recollections/build/lineage before certification.



## Ratified Raw lexical presence

content-serialization-2026-10-14/{before,raw-controls}/results.json.gz records
HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree/UTC/sources/
binaries at historical pin9f2b6e871. raw.rs:21–25 loses omission;
native_raw markup.rs:78–136 collapses absent/false/none. Pin fields/equality/
encoders distinguish them: language semantics/morphology, not Rust mechanics.

Content::get_field delegates Raw to its entity, no inferred defaults.
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

Delegate Transform to Element.get_field, keeping only explicit values/body.
Matrix preserves prior generic projection. RED→GREEN/dependent revalidation; build closes no
X1–X5.

## Present Figure caption reflection

caption-before.json/more-cast-after.json in tekt-v16-figure-casts-2026-10-17
retain UTC/sources/binaries/historical pin9f2b6e871. Constructor keeps caption,
fields.rs:73 exposes only Figure body; FigureElem getter default None. Pin returns
FigureCaption for present caption including Empty, null explicit none, not-known
error for omission. Public Option<Content> cannot distinguish explicit None from
omission: FG1 presence incomplete, own public proposal required, never simulate
with Empty/strings.

Replace Figure.body special projection with static all-field delegation to Figure
entity getter. No caption construction/other-arm-transparency-pin change; same
public getter signature, no dynamic registry/body fallback. L0 before tests/code,
real caption-projection RED/GREEN/six-probe replay/symbolic cases; keep body/Source/
missing fields. Whole batch12GiB/zero swap/jobs1/owner-only reseal after V15/V26,
no new API/phase/default/release.

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

Also delegate FigureCaption to own get_field like Figure, no dispatcher
construction/validation, keeping other arms/pins.

Continuous internal parity ADR-0127, no public field/
trait/signature/phase/mode/default. L0 before tests/code, real getter/metadata/
encoder/CLI RED/GREEN/external corpus without relaxed body/name expectations.
All batch gates12GiB/zero swap/jobs1, owner-only reseal after V15/V26. FG1/SY1
remain incomplete per follow-ups.md; no pending API authorized.


## Explicit Content field projection barriers

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree;
tekt-v16-content-views-2026-10-17/before.json.gz records UTC/status/exact diffstat/
sources-L0/2129 checked hashes. contexts-before.json maps six V16 AST-lines;
projection-plan.json current120-variant domain/per-function complements.
before-probe-result.json17280 symbolic patterns-guards: future Content compiles
six sites, zero selected E0004. Domain-protection mechanics, no fresh parity/
AI-reading proof. Opaque payload model not replacement for real controls.

before-field-probe.json uses current rlib/hashed field-control.rs/argv/UTC/source
freeze:48/84 reads differ for Source/Located/ShowRecipe/Realized inside Styled/
nested Styled; outside already preserve fields. fields.rs:10–13 recognizes only
Equation/Styled, guard rejects carriers before delegation. source-origin nucleus
already requires transparency, field contract body/visual. Internal carriers have
no pinned syntax, metamorphic language-field invariant, not new bilateral vanilla
measurement (pin remains9f2b6e871). Infer no defective intent; lost precedence/
absence-none/extra non-Equation fields/visual→original/recipe execution refutes fix.

### Exclusive closed projection

styled_equation recursively recognizes Source/ShowRecipe/Located body and Realized
visual without recipe, keeping Equation/Styled behavior. Other114 nominal Content
variants None. get_field retains every arm/guard/value, replaces final wildcard
only with94-variant unknown-field nominal complement. Keep four Equation overrides/
outer-first precedence; no Equation schema for unrelated Styled. Two local
functional carrier/precedence/negative tests precode.

Per-function list derives from declared variants not unconditionally handled;
guard-rejectable Styled remains complement. Literal-name/guard variants retain
unknown-name case; any-field delegates excluded. String field domain stays open.
Pinned projection-exhaustiveness shares this rule, this owner exclusive and
source-origin obligation unchanged.

### Acceptance and limits

L0/tests before code/internal continuous ADR-0127/no public field-signature-trait/
compatibility/default/phase. Functional field RED, not probe-setup failure.
After all84 real carrier controls match; style precedence/non-Equation/Realized
visual-not-original. Disposable future-variant model changes zero→six selected
E0004;17280 classifications unchanged except96 helper cases now recognize four
carriers. Exercise each complement alternative separately; not payload equivalence:
replay old bodies/real regression/unchanged-algorithm invariance. No productive
match-mirroring matrix.

Expected exactly six V16 removed/six prescribed uniform V19 added, other source-
rule-severity-message diagnostics preserved/no V20/suppression. V15/V26 before
five-pair reseal. Build/focal-workspace/all-targets Clippy -D warnings/-D
unfulfilled_lint_expectations/strict Rustdoc/fmt/strict-reverse lineage/replay/
source freeze/diff --check, MemoryMax12GiB/High10GiB/swap0/jobs1/heavy sequential.
No commit/push/merge/release; language fronts/pending contracts/global goal open.

## Measured scalar, spacing and container getters

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, state/UTC/sources in
lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz; vanilla
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. before-pdf-probe.json.gz freezes
bilateral field/admission/callback/init; prior paridade-novo-vanilla inventory
E02–E04 carrier limits/discriminating sources, no dump/Rust-shape acceptance.
Language observables.

Semantic getters Text.text;HSpace/VSpace amount-weak;Boxed body-width-height;
Block body-width-height project existing carriers. Spacing Absolute→Length,
Fractional→Fraction. Box width_fraction before width. Block width retains whole
Rel, height Relative/Fractional. Non-Equation Styled keeps own identity/fields,
not child-schema transparent; keep outer-first Equation overrides/all causal
views. No truncated relative measures/lost fields/public masks. Strict presence
is content_methods' responsibility; None never fabricated set Auto. Box semantic
body availability does not authorize strict presence lacking transport.

L0 first/functional RED/bilateral GREEN/no public/default/phase/compatibility.
Earlier State proposal independent; later reader gate grants no authority here.
1:1/pins before reseal/build/full suite/strict Clippy-Rustdoc/fmt/direct-reverse
lineage/MemoryMax12GiB/High8GiB/swap0/jobs1/heavy sequential. debug0/incremental0
only validation environment, measured consumption/no product-gain inference.
Pinned content/mod.rs:761–768 declares Styled(child,styles), not child's schema:
[A].text does not authorize text(fill:red)[A].text.

value-controls.json.gz measures h/v absolute fields type length, but absolute
box.width/block.width relative. Pinned Sizing container.rs:518–527 retains Rel
as Value::Relative. Stored Box Length width/height projects Relative(rel0,absLength),
width fraction remains Fraction; Block already Relative/Fractional. Do not
normalize Box to Length because percent zero; test type as well as repr.

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

### Exclusive marked Heading inspection

Add crate-private constructor_heading recognizing only Styled marked
heading.constructor=true with direct child Heading after Source peeling.
Return Heading/styles. Ordinary Styled stays opaque. Getters project stored
explicit supplement, numbering string/none according to gate/pattern, then base
Heading fields. Remove no wrapper generally or import compiler into entities.

Require H1–H8 in the ratified review, with bilateral RED before code. No public
contract beyond the Heading facade is authorized by H2; A1-S2 separately covers
five State helpers. No new public fields. Ordinary `Styled` and other callbacks
retain their boundaries. Require workspace/doctests/build/Clippy/Rustdoc, fmt,
V1/V5/V15/V26/reverse lineage and no V16 increase; use
12GiB/High8GiB/swap0/jobs1 with heavy commands sequential. RAM gain requires
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

### Exclusive children projection and acceptance

Empty children→empty Value::Array; Sequence→array of Value::Content, immediate
children/order/wrappers/nominal identity retained. Never flatten children/turn
Text("") Empty/remove Styled/execute callback. Existing causal wrappers delegate;
unknown field None via exhaustive complement.

Initial reflection scope left Sequence construction/admission and generic invalid-
method diagnostic for separately measured owners. Later constructor contract below
completes its named subset, not universal sequence/func parity. Before.json.gz
retains refuters. No new public Rust field/type/trait/signature/phase/mode.
L0 before code/RED→GREEN/regressions/workspace/build/strict Clippy-Rustdoc/
ownership-nuclei/direct-reverse lineage/no V16 increase. Reseal after V15/V26;
MemoryMax12GiB/High8GiB/swap0/jobs1 sequential, record peak/no inferred gain.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.


## R4 — measured public mathematical projection / qualified conversion

### Measurement before decision

`lab/results/math-fields-r4-2026-10-23/baseline.json` and `before/receipt.json`
retain UTC, HEAD, complete uncommitted diff stat, source map, archived sources
and binary SHA-256. The oracle is pinned bd5ebec285505889e8ff1d4f68804ab147a699f3
(tree cac836c07e4dec2a89219370428292393a3216b5). The before probe records
fields/has/at and accepted/rejected constructor values rather than inferring
intent from implementation. Upstream math/matrix.rs:45–67,238–272 and
math/mod.rs:227–234 declare the already existing language fields. Local
content_methods.rs:151–158,378–380 omits these projections; frac.rs:43–53
and root.rs:27–38 reject supported operands. These are language semantics
and morphology, distinct from byte or Rust algorithm parity.

### Exclusive correction and gate

Delegate MathVec, MathCases and MathClassOverride field lookup to each owning Element::get_field. MathText exposes text as String, including numeric multi-character text. Keep static exhaustive dispatch and transparent carrier boundaries.

This R4 clause supersedes only conflicting historical operand/field clauses
above. Internal parity correction follows continuous ADR-0127: L0 first,
reseal, actual functional RED before implementation, GREEN and full validation.
No new public Rust field/signature/trait method, default mode or phase change.
Preserve other semantics and all received changes. Record source-bound builds,
full workspace tests/build/fmt/Clippy/Rustdoc, lineage and full lint zero; replay
A–H and R1–R3 without reclassifying their existing limitations as parity.
