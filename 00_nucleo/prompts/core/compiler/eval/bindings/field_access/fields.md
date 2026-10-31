# L0 — compiler/eval/bindings/field_access/fields
Hash do Código: 5c6d4c6e



Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:e79fb4e7f291d69f9108a4b17738b205861d818c6e9eeea873c337dc64bad2f8
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:3c1f911b856e019f09d75d07fd14052912e8f0fb6118fceaaac9a7f4a92bfd97
- 00_nucleo/prompts/_nuclei/math/matrix-morphology.toml sha256:dced4bcb1e45b09423bb3b9739af4da3cd440d500d6a620e08e13219236d1735
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c
- 00_nucleo/prompts/_nuclei/table/cell-presence.toml sha256:f7d3050154813e1ea46c26e235383ae337781a4b28d10ef3248f6eaec96ce221
- 00_nucleo/prompts/_nuclei/table/cell-projection.toml sha256:8388731d50379499612641a70a85695c0e7139ec605289c29161e12086abd78a
- 00_nucleo/prompts/_nuclei/table/cell-realization.toml sha256:2b9e828f5e0d958f4328d98a35320ba2ea321997e541e6ab90520d2e8811e7bf
- 00_nucleo/prompts/_nuclei/transform-semantics.toml sha256:6345ad9c825988872fb53b94c574f61e85a7c21074597400c33bd13ae6a3f68d


## Contextual text fields required by Figure counter callbacks

Measurement precedes this internal correction: the frozen `callback-context-none`
case in `lab/results/v16-corrections-2026-10-19/figure-kind-implementation/cases.json`
returns the unknown-context error on pinned vanilla, while candidate-build-1
incorrectly returns a Length and reports a counter result-type error. Direct
`text.size` and `text.lang` bilateral observations, exact sources, binary hashes,
UTC, HEAD and dirty-file stat are retained in `text-context-before/receipt.json`
under that implementation evidence directory. The cause is the style-field
lookup at `fields.rs:154–174`, which bypasses the existing context admission gate.

The focused `migration-tests-2-language` run exposes a second actual RED:
the outside-context diagnostic spans bytes 35..44 (the whole `text.lang`)
instead of the pinned oracle's 40..44 (the field identifier). Its frozen state,
log and the independent oracle fixture retain the source and provenance.
`fields.rs:157` currently uses `access.span()`; the existing AST field span
provides the proven identifier origin without guessing text offsets.

Reading `text.size` or `text.lang` requires `ctx.in_context`; without it, report
`can only be used when context is known` at `access.field().span()` with the
existing context-expression hint. Known-context reads keep the effective style
and default value. Do not change counter callback execution context or infer
styles from a callback's creation site. This is continuous ADR-0127 language
parity correction, with functional RED before the guard, no public API or phase
change and no linter suppression. All prior field/reflection contracts remain.

## Ownership and extraction evidence

Layer L1. Sole productive consumer:
`01_core/src/compiler/eval/bindings/field_access/fields.rs`. At clean HEAD
`ba5a4c7d305ed1444660d9eb80862f51b417efb6`, field_access.rs:557–1051 contains
extracted behavior; SHA/time/initial state in
`lab/results/construction/diagnostics/atomizacao-field-access-2026-09-30-before.json.gz`.

## Exclusive lookup and presence contract

Resolve value/Content fields preserving precedence/message/span/result and
parent field_access signatures/semantics. Static same-layer L1, no I/O/mutable
globals. Keep L1/field-access tests/crystalline-lint with valid ownership/hashes,
no inferred speedup.

field-access-before.json records HEAD/diff/binaries: Tekt smartquote().double
returns true, historical vanilla9f2b6e871 reports
`field "double" in smartquote is not known at this point`. Filter semantic getter
needs default, but direct field was not set. SmartQuote direct access uses
content_methods ContentField: Set returns value, Unset the measured message,
Undeclared keeps does-not-have-field error. Keep real span/present fields/
authoritative Some LocatedContent snapshot/transparent causal wrappers, other-
element semantics; invent no defaults. Same signature/phase, internal continuous
ADR-0127; precode RED.

## Matrix morphology and strict field access

Before deciding, matrix-morfologia-2026-10-06/before.json.gz records HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/diff/sources/
binaries/cases at historical `9f2b6e8715237cb086899a42873660fe744622e8`.
Raw/query mat prints{func:math.matrix} without rows; getter/methods lose fields,
func equality false. Pinned matrix.rs:91–221 declares fields/rows. Local
math_matrix.rs:45–52 retains rows/explicit, but Element.get_field default;
content_methods.rs:151–191 omits mat; cli.rs:1154–1348 falls back body.
Language morphology/semantics, not Rust/PDF bytes or inferred intent.

Direct Mat uses Set/Unset/Undeclared: Unset `field "f" in mat is not known at this point`
with real span, Undeclared `mat does not have field "f"`. Preserve authoritative
Some/other elements. RED→GREEN/bilateral fields-has-at-func/JSON-query/padding/
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

Direct TableCell access uses Set/Unset/Undeclared/real span, transparent causal
wrappers and authoritative Some snapshot.
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

Keep Set/Unset/Undeclared/spans, projecting presence through owning helper.
Some snapshot remains authoritative.

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

### Exclusive prepared presence access

Keep Set/Unset/Undeclared/spans/Some snapshot, no new global defaults.

### Ratified gate, acceptance, and explicit scope split

The ratified package adds five public fields, two public carriers, and one new public facade, and changes the logical phase before Cell show. The receipt above establishes ratification for this scope. Continue to require RED→GREEN, suites/build/lint/restricted preservation, and D08A/B2, D05/D06/D07/query collections. Acceptance is language, effect, and actual geometry behavior; byte equality or whitespace normalization cannot close render acceptance.

This first package D08B3-A is incomplete in the Cell domain: D08B3-B is to complete isolated Cell defaults (`raw-cell-no-placement`/`outside-fields`) and the separate decision on fresh generation by `show-cell-reconstruct`. Generic Table defaults/fields (`table-root-equality`), rich styles/functions/paint/strokes/insets, rich rows/gutters, multi-page spans, and PDF/UA conformance remain outside certification. Do not change additional legacy D08B2 casts, syntax, flags, or global non-locatable policy in this package.



## Ratified Source transparency

Before deciding, table-origens-d08b3c2-2026-10-07/baseline.json.gz retains HEAD/
working tree/UTC/sources/argv/binaries at historical pin9f2b6e871. Existing carrier
operations provide transparency; no observable Source element. Human confirmo
2026-10-07 final record
`lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`;
nested contract.md is preapproval history. Existing ShowRecipe.body delegation
also accepts Source.body without separate-origin execution/published carrier/
Location. Preserve this operation's classification/preparation/serialization.
Sole owner/static/no I/O/new signature/trait/flag/eval-layout phase;
C2/C/macro tests/recollections/build/lineage before certification.


## Approved typed-transform identity

transformacoes-2026-10-14/results.json.gz/supplemental controls measure lost
identity/presence in matrix/body. Premigration source/owner in
`lab/history/construction/records/00_nucleo/materialization/transformacoes-tipadas-2026-10-14/base-files.json.gz`,
hashes/lines consumers.json. Approved manifest
`960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d`
authorizes new representation/layout resolution. Language semantics/morphology
versus Rust/reconstruction mechanics. Direct Transform fields use lexical presence classifier distinguishing omitted/
explicit/undeclared, with existing diagnostics. RED→GREEN/dependent revalidation; build
alone closes no X1–X5.


## Contextual native-transform getters

transformacoes-2026-10-14/style-contract/results.json retains pin/commands/UTC:
contextless errors, aliases retain identity, With rejects set/getter/show, two
origin declarations fold per axis. layout/tests.rs:21648 constructs Matrix
explicitly. Approved manifest
`960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d`.

Four native functions' contextual getters return effective style/declared parameter
default by actual fn identity, aliases included. Outside context:
`can only be used when context is known`. With is no element/getter provider.
Origin reads axis-folded style; body not settable. Bake no result into lexical
instance. Validate corpus/causal tests; construction is not layout/release.

## Explicit domain guards

T006 fields.rs:234/T007:267 measured after source/L0 reading. SHA/HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/status/exact
diffstat in tekt-guards-2026-10-16/before.json.gz. Hash-reconfirmed28 guards/
21 owners; pinned CI asks split pattern or condition in body. Rust structure,
not functional-defect/AI-reading proof or upstream intent.

Merge two Value::Content and two Located snapshot-None arms, deciding declared
presence versus getter inside body. Keep short-circuit/early return/authoritative
Some/Set-Unset-Undeclared/messages-spans. Never query getter for absent Some key;
without snapshot ordinary Content keeps get_field, declared presence uses its
accessor without fallback.

Internal continuous ADR-0127, no new public/default/compatibility/phase. Sole
owner/no helper hiding conjunction/suppression/config change. Old functional
controls pass precode unchanged; real RED28 V17, GREEN zero. Check success/
fallback/error/short-circuit/effects/restricted reseal. Require build/full suite/
all-targets Clippy warnings-unfulfilled expectations as errors/fmt/Rustdoc warnings
as errors/strict V1/V5/V15/V26/reverse/SARIF no new warning,12GiB/zero swap/one
job. Separate measurement for memory/performance/global parity claim.

## HTML feature gates and module identity

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree.
`lab/results/construction/diagnostics/html-feature-identity-2026-10-17/before.json.gz`
retains UTC/status/exact diffstat/full sources-L0/hashes; all2123 prior hashes
checked before matrix reuse. before-probe.json.gz keeps argv/fixtures/output/
binary SHA. Historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`,
binary `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
fields.rs:39–52 gates format.html, not std.html; identity.rs:84–101 identifies
modules by names/bindings. Valid-import probes refute parity: std.html direct/
alias/closure/reexport accepts disabled Html; global.typ with type/format/document/
ordinary asset is wrongly gated; format.typ with svg/png/pdf/html functions gets
wrong html/bundle gates. Initial let m = import probe invalid (import returns none),
archived candidate-probe-syntax, no evidence credit. Pinned module.rs:138–154 reads
binding guard, code.rs:347–368 anchors field. Language semantics/diagnostics,
not required BindingGuard/upstream addresses. Parent L0 field_access.md:61–62
already forbids std identification by binding comparison.

### Exclusive feature lookup and acceptance

Html gate covers effective Std or Format module; asset/bundle retain respective
effective types. Keep priority/once-only receiver/pre-args-set short-circuit/
field span/messages/two hints. Pure eval_value_field_access receives no feature
policy. Precode tests use current APIs/real Module::new-stdlib/eight masks/aliases/
closure std shadowing/effects/spans, no suppression.

Consume pinned builtin-module-identity, private mechanics satisfying existing
gates/internal continuous ADR-0127. Public Module opaque, no exported field/
method; compare declarations/signatures before-after, no public/phase/default.

### Acceptance and limits

Real RED before code; GREEN gates/homonyms/aliases/closures/imports in eight
profiles, preserving message/hint/span/causal precedence. Disabled-feature
rejection and active-feature acceptance classified separately; no MATCH/Preserved
credit from active-feature cases for disabled cases. Keep prior cross-file std
module equality divergence; no Module Eq/global-parity claim. Revalidate prior
std.html-disabled matrix/active controls. No publication/commit/release. Require
build/full suite/strict all-targets Clippy-Rustdoc/fmt/V1/V5/V15/V26/reverse/global
differential without new error-warning. Reseal only six owners after valid
ownership/nuclei; heavy tasks sequential MemoryMax 12 GiB/High 10 GiB/swap0/jobs1.

## Feature-gate causal continuation

candidate-1/validation-green.log records four passing controls and real argument
test failure: std.html(panic(...)) panics before gate. before-set-probe.json.gz
retains UTC/hashes/sources/bilateral outputs at same HEAD/uncommitted tree:
called std.html/format.html/std.asset/format.bundle evaluate args before gate;
set std.html/std.format.html/alias hides target error/ignores args. Missing
set variable/field wrongly accepted; false condition short-circuits. Active-HTML
set alias ignores panic only in Tekt. Language order/diagnostic/effect.
collection_methods.rs:36 evaluates args before module authentication;
set.rs:44 uses ok() resolving target, :326 authenticates HTML by literal syntax.

Preserve prior identity responsibility. fields owner extracts three gates into
eval-internal check_module_field_feature(&Value, &str, Span, Features) ->
SourceResult<()> and calls it before lookup; other owners duplicate no diagnostics/
rules.

L0 precedes correction; internal/parity continuous ADR-0127, pinned
builtin-module-identity unchanged, no public field/fallback/suppression change.
Keep controls/eight-profile matrix; add bilateral effect/set checks with/without
feature. Full suite reruns after new source freeze; changed source cannot reuse
old gates.


## Single-evaluation evidence and boundary

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree;
`lab/results/construction/diagnostics/call-once-2026-10-17/before.json.gz`
records UTC/status/git diff HEAD --stat/sources/L0/config/hashes. Exactly2124
preceding hashes were checked before baseline. before-probe.json.gz and
before-math-probe.json.gz retain bilateral binaries/argv/fixtures/output.
Historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
calc.abs/array.len/str.from-unicode measure receiver3/args2 versus1/1;
module.measure/layout/within/before/after/func/has/body reach receiver4;
mutating fallback args3 versus1. Missing methods execute argument panic before
causal error. math.mat adds twice; prior array(1,) becomes(1,1,1) versus(1,1),
while empty array masks defect with truncate. call.rs:70–118 loses receiver;
collection_methods.rs:43 evaluates args before recognizing operation; contextual/
content/selector reevaluate AST on fallthrough; method_dispatch.rs:247 discards
args; invoke.rs:66–120 compensates by name. call_dispatch L0 already requires
receiver/args once. pinned-source.json.gz pins upstream call.rs/methods.rs:
interest is language order/effect/diagnostic, not copying Vm/storage/Rust equality.

First fixture declared reserved or as function: invalid, archived in
invalid-fixture-attempt with no call-effect credit. It revealed panic101 on
invalid import, still open for parsing/import owner's causal correction. Valid
baseline replaces only fixture.

### Exclusive once-resolved field lookup

Extract eval-internal eval_field_access_with_target(access, target: Value, scopes,
ctx, engine), preserving lookup body with resolved target. eval_field_access only
evaluates receiver/delegates. Never substitute pure eval_value_field_access:
keep features/style-context/a11y/spans/module warnings.

### Acceptance and limits

Consume pinned eval/call-once nucleus. Internal mechanics/parity ADR-0127
continuous flow keeps pub fn/pub use/public structs; new parameters/returns/
helpers remain crate/eval-visible, no public trait/field/default/phase. Supersede
only conflicting historical internal-signature/name-compensation restrictions;
keep Tekt extensions.

Co-located functional tests before code: receiver/args effects, mutating fallback,
error before args, where/with/selector/collections preserved, matrix without
truncating legitimate effect. Confirm real RED then GREEN. Compare valid bilateral
cases/controls, every collection method's hits/misses, errors/spans/occurrences/
Value complements. Residual diagnostic debt is not equality. Unhandled dispatch
must retain receiver without running args callback. Count observed effects, not
PDF bytes/Rust Eq/microbenchmark.

Ownership/V26 before restricted batch-owner reseal. Require build/full workspace,
all-targets Clippy -D warnings/-D unfulfilled_lint_expectations, strict Rustdoc,
fmt/V1/V5/V15/V26/reverse/differential SARIF without new error. Uniform V19 stays
informational with alternative matrix/behavior relation; no suppression/config
change. Heavy commands sequential under 12 GiB/High 10 GiB/swap0/jobs1. No publication/
commit/merge/release or global-objective closure claim.

### Measured Type::Array message and field anchor

before-probe.json.gz missing/array/green-first-attempt.log show same causal lookup,
wrong double quotes versus pinned backticks in
``type array does not contain field `__missing` ``. Change only Type::Array missing-
field literal in shared pure field-call lookup. Preserve content/span/hints/other
schemas/errors; new measured-pin test never relaxed. Other type/getter diagnostics
remain separately auditable, no global fix claim.

validation-observables.log/candidate-3 gives correct message but range0..17 in
(array).__missing(...) versus field8..17 pin. Final fields.rs span selector omits
Value::Type/falls to whole access.span. Pinned code.rs:347–368 uses field.span.
Add Type to access.field().span family; pure lookup preserves caller span/prior
special cases. Extended V19 is uniform field-only anchor, old controls cover other
alternatives/new Type::Array test uses Source::span_byte_range.

## Measured additional presence and text projection

HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, state/UTC/sources in
lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz. Pin
`bd5ebec285505889e8ff1d4f68804ab147a699f3`; before-pdf-probe.json.gz freezes
bilateral field/admission/callback/init. Prior paridade-novo-vanilla inventory
E02–E04 records carrier limits/discriminating sources, not dump/Rust structure
acceptance. Language observables.

Box/Block/H-V use presence path: missing width/height/weak is Unset, not nonexistent
field. Unknown names remain Undeclared. Styled keeps prior classification,
authoritative Some intact. Pure getter reaches Text.text. No extra receiver/args
evaluation/public API/phase. L0 first/functional RED/bilateral GREEN; no public/
product-default/phase/compatibility. Earlier State proposal independent, later
separately ratified reader contract grants no authority here.1:1/pins valid before
restricted reseal. Build/full suite/strict Clippy/Rustdoc/fmt/direct-reverse lineage,
MemoryMax 12 GiB/High 8 GiB/swap0/jobs1/heavy sequential. debug0/incremental0 only
validation environment, measured consumption/no inferred product gain.

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

### Exclusive strict marked-form access

Strict marked-form access uses content_field(Set/Unset/Undeclared), preserving
omitted-field distinction. Unmarked Styled remains opaque; no general transparency
for Text/Heading or morphology change.

Require H1–H8 in the ratified review, with bilateral RED before code. No public
contract beyond the Heading facade is authorized by H2; A1-S2 separately covers
five State helpers. No new public fields. Ordinary `Styled` and other callbacks
retain their boundaries. Require workspace/doctests/build/Clippy/Rustdoc, fmt,
V1/V5/V15/V26/reverse lineage and no V16 increase; use
12GiB/High 8 GiB/swap0/jobs1 with heavy commands sequential. RAM gain requires
measurement, not intention.

## Closed-domain decision and preservation

### Evidence before decision

At checkpoint `370d78355bd646d1abead5bcacef215288cc8834`,
`01_core/src/compiler/eval/bindings/field_access/fields.rs:759–769`
and the caller/producer evidence recorded in
`lab/results/v16-corrections-2026-10-19/batch-c/agents/structural/l0-audit.json`
show the current decision and its refusal path. That receipt records UTC,
working-tree diffstat and exact source/owner SHA-256 before this amendment.
The existing clauses above establish the behavior to preserve; an open Rust
match is structural evidence, not proof of additional language admission.

### Exclusive policy

The complement is not Matrix/Cell through the four causal carriers and returns false.
Styled is intentionally not generically peeled here; separate is_presence_content
classification and earlier authoritative snapshots still apply. False does not mean no
fields or allow a failed snapshot lookup to fall back.

Keep the strict field dispatch separate from the broader presence classifier. The
negative group says only that this receiver is outside the Matrix/Cell strict branch; it
does not deny semantic getters. Test Matrix and Cell through Source, ShowRecipe, Located
and Realized.visual, opposite hidden original/visual values, opaque Styled,
authoritative snapshots and ordinary getters.

The explicit nominal complement is: `Empty`, `Text`, `Space`, `SpaceFromNewline`, `Parbreak`, `Sequence`, `HtmlElem`, `HtmlFrame`, `Par`, `Heading`, `Title`, `Strong`, `Emph`, `Raw`, `ListItem`, `EnumItem`, `Link`, `Equation`, `MathSequence`, `MathIdent`, `MathText`, `MathFrac`, `MathAttach`, `MathPrimes`, `MathRoot`, `MathStretch`, `MathDelimited`, `MathLr`, `MathMid`, `MathOverbrace`, `MathUnderbrace`, `MathUnderbracket`, `MathUnderparen`, `MathUndershell`, `MathOverbracket`, `MathOverparen`, `MathOvershell`, `MathAlignPoint`, `Linebreak`, `MathCases`, `MathAccent`, `MathCancel`, `MathUnderline`, `MathOverline`, `MathVec`, `MathClassOverride`, `MathLimitsOverride`, `MathUnderover`, `MathOp`, `MathStyled`, `Label`, `Ref`, `CounterDisplay`, `CounterUpdate`, `Outline`, `OutlineEntry`, `Figure`, `FigureCaption`, `Image`, `Shape`, `Curve`, `Transform`, `Grid`, `GridHeader`, `GridFooter`, `GridCell`, `SetPage`, `PageRun`, `Align`, `Place`, `Flush`, `Styled`, `Divider`, `Terms`, `TermItem`, `Quote`, `Document`, `Asset`, `SmartQuote`, `PdfAttach`, `PdfArtifact`, `Underline`, `Strike`, `Overline`, `SmallCaps`, `Pad`, `Hide`, `HSpace`, `VSpace`, `Pagebreak`, `Colbreak`, `Stack`, `Boxed`, `Block`, `Bibliography`, `Cite`, `Footnote`, `FootnoteEntry`, `TableHeader`, `TableFooter`, `GridHLine`, `GridVLine`, `TableHLine`, `TableVLine`, `Table`, `Repeat`, `Columns`, `Metadata`, `State`, `StateUpdate`, `StateDisplay`, `CounterDisplayCallback`, `ContextBlock`, `Dynamic`.

### Acceptance and refutation

Preservation controls run before and after the repair. A real temporary enum
variant in a disposable complete workspace must fail with E0004 at this exact
decision after repair; failures at other decisions alone are insufficient.
Exercise nested closed domains separately where present. The structural RED is
the original V16/open evolution boundary, not a fabricated runtime defect.
Any changed result, message, span, callback count, argument evaluation or refusal
route refutes preservation. Retain informational V19 groups with their rationale;
add no suppression, hidden catchall, linter/configuration weakening or new policy.

Internal continuous ADR-0127: no public field/trait/signature, product default,
phase or compatibility change. L0 precedes code; reseal only valid owner pairs
after V15/V26. Require focused and full workspace tests, build/all-targets, fmt,
strict Clippy/Rustdoc, path checks, direct/reverse lineage and differential SARIF
without new errors or warnings outside the selected repairs. Heavy checks are
serialized under one job, 12 GiB maximum, 8 GiB high and zero swap. Historical
parity claims and unresolved scope exclusions above remain bounded.


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


## R4 — direct public field diagnostics

Evidence precedes correction: before-final/receipt.json under
lab/results/math-fields-r4-2026-10-23 records pinned bd5ebec2, UTC, source
map, binary SHA and working-tree stat. Direct omitted vec.gap/cases.reverse
reports does-not-have-field locally, whereas vanilla distinguishes unset with
`field ... is not known at this point`; these are observable language errors.

Extend the existing private strict field classifier is_matrix_or_cell_content to MathVec and MathCases, retaining transparent carrier recursion. Direct lookup must use content_field Set/Unset/Undeclared and preserve identifier span and authoritative LocatedContent snapshots. No generic Styled transparency.

This supersedes only conflicting lookup/name clauses above; continuous
ADR-0127 parity correction without public Rust signature/entity/trait, phase
or default changes. Tests first, RED→GREEN, existing suites and full lineage/lint.


## Issue 8894 — approved Stroke migration

Measurement: the source inventory and previous owner bytes in
`lab/results/construction/diagnostics/issue-8894-2026-10-31/` bind exact
HEAD/dirty diffstat/UTC and each affected source/owner before migration.
`stdlib/layout/stroke.rs:430–434` erases em; geometry.rs:81 carries only f64.
The explicit user approval in approval.json ratifies the public Length field
and occurrence-layout resolution, as specified by
`00_nucleo/prompts/core/entities/geometry.md`. This consumer retains its
exclusive responsibility and all other ratified clauses and controls.

Preserve full Length thickness through eval construction, field access and repr.
Resolve thickness with the effective occurrence font at existing layout consumers
and prepared FrameStroke boundary before scalar geometry/overhang is calculated.
Keep all other values, field-presence, validation/error priority, behavior and
previous approvals. Point-only fixtures retain exactly their numeric values.
The approved public/phase contract is bound in approval.json. RED→GREEN and
full proportional validation plus forward/reverse lineage remain required.
