# L0 — compiler/eval/rules/set — Set rule evaluation
Hash do Código: 64f5c22a

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/paragraph-hanging-indent.toml sha256:12bf17ff10c443c56512c508693160e37e80f6ee925c506912718231b39ced12
- 00_nucleo/prompts/_nuclei/paragraph-hanging-validation.toml sha256:00e900edd4f7f3d6cd9aef545f6631615d612b377b17e9dd37f0254c4fd7cf51

- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:e79fb4e7f291d69f9108a4b17738b205861d818c6e9eeea873c337dc64bad2f8
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:09b32666fb1b1fa7ba8f2ef7c979c60cbc64cc515b8d9c46818bec19d7471fb1
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:9a193bdaa08a4eb5b41e9d4124388d0de24bd66776fedf04ef9273c1390462ec
- 00_nucleo/prompts/_nuclei/layout/raw-block-boundary.toml sha256:a3772778f9e542de87823509c8eac57904fc73fea9526cebe6052833a69ef806
- 00_nucleo/prompts/_nuclei/math/bare-insertion.toml sha256:c54d66589196680f0db26d0b6328d20c5d28d474458f0661275e871139d07d9b
- 00_nucleo/prompts/_nuclei/paragraph-break-policy.toml sha256:41e38095ab33ab1710ba11f43d6a5ca50def5a703f77a0001a066b72964c4165
- 00_nucleo/prompts/_nuclei/paragraph-composition.toml sha256:802ff8f08c2cc2e7a81987f63f980ddc1254d4ff4f8c972804d6bb3e84b4c484
- 00_nucleo/prompts/_nuclei/paragraph-costs.toml sha256:d6c54bdd10c4ee5884a9a46bbdabaad3ffdea3aa0e4ad905411f6b43d1b6286b
- 00_nucleo/prompts/_nuclei/paragraph-selection.toml sha256:dc3d1eff8516cab7b246430e867fd906289164509c4533e071be543f33b210b6
- 00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml sha256:a933254df6e90a505910759d443fe8b06f2646e2d4a9949bc179326fc9c5c329
- 00_nucleo/prompts/_nuclei/smartquote-semantics.toml sha256:d92a80d952b2cb79d90b96a75a0387cfde2da24f2fd41327de8ab0e647a97f8c
- 00_nucleo/prompts/_nuclei/table/cell-style-defaults.toml sha256:041abef9e300ff0412d9631dced12480f8e0e6e65fb9cc3733f290e6664f649b

Layer: L1
Exclusive consumer: `01_core/src/compiler/eval/rules/set.rs`

## Extraction evidence and contract

The uncommitted tree captured in
`lab/results/construction/diagnostics/atomizacao-eval-rules-2026-09-30-before.json.gz`
contains this family's routines in rules.rs. L1 passed6401 tests/two ignored
before extraction. Structural measurement, no benchmark/fresh bilateral vanilla
proof. Historical measurements/pin caveats remain in
`00_nucleo/prompts/core/compiler/eval/rules.md`.

Validate targets/args and build Styles deltas without partial mutation, preserving
evaluation order/current defaults. Keep page/heading/par/text/format/other parent
schemas. Disabled HTML gate precedes args only after common condition reaches
target, as refined below.

Preserve observed signatures/identities/casts/defaults/error precedence/eval
phase. Parent keeps shared diagnostics/static bindings; selector matching,
termination and textual realization retain exclusive owners, no duplicate
algorithms. L1 suite/crystalline-lint without regression; language semantics/
syntax/morphology under ADR-0107/0108, sole ownership/valid pins ADR-0129/0130.

Static target order stays. page/text arms have exclusive owners
`00_nucleo/prompts/core/compiler/eval/rules/set/page.md` and
`00_nucleo/prompts/core/compiler/eval/rules/set/text.md`; this owner retains other
targets/static forwarding, no public/default/phase change from extraction.

## Ratified SmartQuote set schema

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

Add smartquote.double Bool to existing set table with current type_mismatch/span
and push_custom smartquote.double, matching compile-set-double-native positive.
Keep enabled/alternative/quotes validation. Populate no leaf field from chain or
change other targets.

## Inherited automatic-Equation block

math-bare before.json.gz block-style/local-block/number-style measures inherited
block in vanilla. set.rs:583–630 handles numbering/other fields but omits block.
For existing math.equation target evaluate block once, Bool-validate and transport
equation.block lexically; omitted keeps chain. Same signature/default/enum/type
diagnostic. Prepared automatic-Equation owner resolves it. Test local block not
leaking/block numbering/invalid value.

## Raw block margin evidence and delegated physical boundaries

These measured refinements constrain set transport and its dependent layout
owners; no rendering algorithm transfers to this dispatcher.

after.json.gz/after-additional.json.gz (complete binary/snapshot/UTC) refute initial
formula: consecutive extra gap2.64pt; Cell prefix5.49796pt high, standalone2.59598pt
high; explicit margins absent because set.rs:976–1006 misses Native block.
refuters-red.json.gz Strong-cell loses height, After83.27018 versus89.95610,
plain-cellAfter87.36012; preexisting sizes-up/down paragraph negatives delta10.8pt.

Raw weak default uses80% size:10.56pt, not13.2pt. Outer 11pt paragraph contributes
13.2pt, consecutive Raw10.56pt. Existing prev_margin_is_parbreak handles weak
paragraph margin including inherited Raw; explicit below remains strong/outside
that branch. Cursor collapses weak margin with actual par.spacing, leaving strong/
regional/fractional/checkpoint paths. Prior closed subframe paragraph contributes
above. First Raw in subframe uses initial cursor minus outer top-edge because
cell_body.rs:253–260 recomposes legacy subframe with ascender-top-edge. Terminal
boundary classifier crosses Strong/Emph, with explicit complement/no dropped
data; predicate, not enum projection.

Set accepts typed Native block/aliases only for spacing/above/below Length|Auto,
evaluated once/delta published after whole validation. Other fields/types retain
previous fallback/warning, uncertified. Apply transports properties/size as Length.
Render margin uses effective Raw size default/show-set; prior paragraph contributes
separate outer size. Styles does not measure glyphs. Remaining negative controls
refute hypothesis; preexisting sizes-down not claimed fixed. Expand no prior-
paragraph-size rule without own measurement.

edges-red.json.gz: below0 overlaps After74.95612 versus82.19410; negative
After72.95612 versus77.19410; auto+spacing20 Raw98.10418 versus91.30420;
first below0 must beat next Raw auto above (def97.99011, not108.55011).
Explicit edge wins in either direction; two explicit/two weak max-collapse. Auto
edge interrupts spacing shorthand/resumes par.spacing/weak default. Explicit
below≤0 leaves cursor real end+below with pending geometric baseline; weak below,
even zero, enters Cursor weak path adding next text top-edge.

suites-third.log refutes unconditional FontMetrics init: two nontext place image/
shape controls started acquiring font book. Initialize regional ascent only in
legacy subframe with terminal Raw block product. Geometric/nontext retains route
without new font query; nontext data gains no font dependency from repair.

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

### Exclusive declarative Cell set

Recognize Native table.cell by typed identity through pure Ident/FieldAccess and
aliases. Reuse eval_args ordered evaluation/spreads; false condition evaluates
no args. Named schema x/y/colspan/rowspan/fill/stroke/align/inset/breakable rejects
body/unknown/positionals at causal spans. Reuse owned constructor casts without
double declaration evaluation/second rich schema. Publish only declarative
cell.<field> values after complete validation. StyleChain/custom stores declarations,
never resolved plan/carrier. Keep other target arms/rich-uncertified warnings.

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

Borrow inherited stroke into fold_stroke as Option<&Stroke>; publish result through
existing From<Stroke>. Keep capture/fold order, mutate no other alias payload.
Keep tests/oracles, pertinent RED→GREEN if new isolation behavior exists,
owner-only reseal/revalidation. No semantic/default/phase expansion.


## Common condition before target

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree;
`lab/results/construction/diagnostics/set-condicoes-2026-10-14/{before,extra-before}/results.json.gz`
retains UTC/sources/owners/status/diffstat/commands/binary hashes at historical
pin9f2b6e871. set text/page/par/block if false executes argument panic; condition
mutation N=0 false/N=2 true versus1/12, also show-set. Disabled format.html accessed
though false skips target in oracle; invalid condition precedes access.
set.rs:44–54 checks only Cell condition after target probe; integer error says int.
Pinned rules.rs:15–34 evaluates/casts condition before target/args: language
order/effect/diagnostic. extra-before heading true/no-condition swallows argument
panic; set.rs:193–194/1119–1120 turns Err into None for heading/registered element.
Not short-circuit. Registry target is Tekt extension: keep open schema but propagate
SourceResult causal eval errors; current-registration pure test gives own RED.

### Current condition and error propagation

Evaluate optional condition exactly once at eval_set_rule entry before target
probe/resolution/features/args. False returns Value::None with no delta/carrier/
target warning/argument execution. True/absence proceeds; other type
`expected boolean, found <long name>` at condition span via vanilla_type_name;
eval errors intact. Remove Cell-specific duplicate evaluation. HTML pre-args gate
applies only reached target: false touches no binding/feature. Active path keeps
gate/hints; false control proves short-circuit, not disabled-HTML parity.
capture_set_styles inherits condition with same scratch/style restore; markup
lexical false-set boundary remains own owner.

At heading/registered-element argument-error→None sites propagate SourceResult
message/span/trace. Change no valid casts/defaults/schema/target admission.
Only this consumer/owned tests, internal continuous ADR-0127/no public type/
signature/flag/phase. RED→GREEN false/no-target-effect, true condition-before-args,
once-only condition/types/ranges/failure order/Cell-alias/show-set/previous styles/
active heading-registry errors. Bilateral CLI feature/numbering/condition-arg
failures/label-after-false; build/suite/strict-reverse lineage/warning inventory.
No all-set-schema/target/special-show-selector parity claim.

## Approved native-transform set routing

transformacoes-2026-10-14/style-contract/results.json retains pin/commands/UTC:
contextless error/alias identity/With rejects set-getter-show/two origin sets
axis-fold. layout/tests.rs:21648 directly builds Matrix. Approved manifest
`960f20a8593874da3f3588281dbefa535e4e0ade8109ae117acbd4f5a038281d`.
After common condition resolve aliases by executable identity of four natives.
Validate through same typed construction without layout/body execution; set only
declared parameters in existing custom styles. Origin axis-folds inherited.
Reject With as pinned; artificial empty body never becomes future lexical field.
Validate corpus/causal tests; construction is not layout/release.

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

After existing false/true/type condition, propagate pure_path target-resolution
error instead of ok() erasure, including feature/missing variable/field refuted
by probes. Do not generalize non-pure_path resolution. Keep target_function
available through as_ref while extracting native_target. Existing HTML set arm
uses actual FuncRepr::Format(FormatElementKind::Html, _) identity, never spelling.
Move only it before remaining FieldAccess block; remove duplicated literal gate
inside, since ordinary resolution authenticates/features. Real alias/std/native
root uses existing pretty validator; ordinary homonym never enters by name.
Keep pretty body/spans/false condition/other formats-targets/priorities, no public
API/new phase.

L0 precedes correction; internal/parity continuous ADR-0127, pinned
builtin-module-identity unchanged, no public field/fallback/suppression change.
Keep controls/eight-profile matrix; add bilateral effect/set checks with/without
feature. Full suite reruns after new source freeze; changed source cannot reuse
old gates.


## Heading callback evidence and responsibility

The 2026-10-17 evidence records HEAD
`b1b486f3d42431f8ea360bc5712637ee1e3d4f6c` and working-tree/UTC/hash provenance
in `lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/before.json.gz`.
`before-pdf-probe.json.gz` records `heading-supplement-panic` succeeding in Tekt
and producing `A1_HEADING` in vanilla
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Both accept
`heading-supplement-valid`, but only vanilla uses `Custom` in the reference.
Upstream `model/heading.rs:249–289` synthesizes supplement per occurrence;
`model/reference.rs:393–415` applies the function and display.
`Set.rs:286–320` ignores supplement and clears numbering;
`snapshot_fields` at `heading.rs:55–68` fixes translation. These are observable
callbacks/values, not required upstream mechanics.

Heading target evaluates arguments once through existing route, transporting
supplement auto/none/Content/Str/Func as custom heading.supplement. No callback
in set. Supplement-only set leaves prior numbering; change numbering only when
provided. Keep other heading scope-outs, no all-field support claim.

Internal parity continuous ADR-0127, no new Rust API/public field/eval-layout
logic transfer. Existing post-walk Engine owns callback application; L0 first/
RED/bilateral controls, no encoder/walk/layout callbacks/universal-field parity.

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

### Exclusive Sequence set

Recognize only actual native_sequence callback. Empty set no-op; any extras fail
in causal order. Required constructor children is not settable. Evaluate args once
before validation, value panic first. False condition precedes target/args.
Reject With of this identity before args with existing transform-With diagnostic.
Classify no local names/change other target/default/delta. Completes earlier R1-named R2 responsibility. Continuous
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

## Bibliography block boundary and hanging validation

### Evidence before decision

`lab/results/construction/diagnostics/bibliography-indent-2026-10-18/before.json.gz`
contains60 probe pairs at HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`,
uncommitted tree/UTC/status/exact diffstat/sources/argv/binaries/text/boxes;
before-sources.json.gz preserves full L0/source. Oracle commit
`bd5ebec285505889e8ff1d4f68804ab147a699f3`,tree
`cac836c07e4dec2a89219370428292393a3216b5`,binary
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`;
Tekt baseline `97f6fa3078bdd6c40aabb80d6f93f3fd019b405ce70fbf2266686d8fb42271cb`.
Untitled MLA starts at citation Silva x41.84/y5.168 versus8/27.463995; titled
Tekt mixes title/citation y6.7568/8.108/interleaves custom-CSL extraction, pin
separates. Eight positive Par tests lack indent; ten none/auto/int/ratio/string
set/constructor negatives wrongly accepted versus expected length.
bibliography.rs:106–123 dispatches title/cache inline without closing prior line;
set.rs:1180–1192 unknown hanging-indent, structural/par.rs:173 ignores it.

Pinned rules.rs:463–531 realizes title/entry blocks; no-prefix indent uses inset+
compensating HElem, prefixed uses two columns. par.rs:431 Length, collect.rs:
146–148 keeps initial absolute advance. Language position/type/error, no required
Rust graph/bytes/storage algorithm. Inferred missing block boundary mixes title/
citation/body and missing validation admits invalid types, refutable by positions/
types-hints-spans/other controls. No one-offset cure or all-extraction-mechanics claim.

### Scope split and initial validation

Internal continuous ADR-0127/no public field-trait-signature/default-flag/Rust
compatibility/phase. L0 first. Historical validation step incomplete:
A2-Structure owns CSL hanging_indent/prefix/body/margins/columns;
A2-HangingTransport per-occurrence overrides/priorities; A2-ParIndent ordinary
Par set/constructor/strategies/directions; A2-Multi attribution/cache per bibliography.
Later physical contract below completes only its measured subset. Keep incomplete-
support warnings until consumers exist; rejecting negatives does not fix positives.

Resolve Par by existing effective native_par typed fn_addr_eq after dispatcher,
direct/alias/std.par, no constructor/name-only identity. argument-before.json.gz
first type GREEN still accepts invalid alias, retained as bypass refutation.
Other target routes unchanged; general nonelement rejection separate/unclosed.
Validate hanging-indent via pinned paragraph-hanging-validation before fallback.
Initial valid Length retained warning/behavior until later transport below; invalid
uses expected_length_error at expression span. Other keys/scopes/casts/arg count/
show-set unchanged by initial gate.

Precode real RED/GREEN title-citation-body-fallback/negative controls including
zero/negative/affine/empty. Re-measure PDF boxes/extraction, retain indent/multiple-
bibliography REDs, no unsourced Tekt advantage. Build/workspace/doctests/strict
Clippy-Rustdoc/fmt/V1/V5/V15/V26/reverse/SARIF no new error-warning.
12GiB/High 8 GiB/swap0/jobs1/debug0/incremental0/heavy sequential; no commit/push/
release.

## Measured physical hanging-indent transport

### Evidence before decision

bibliography-hanging-2026-10-18/before-sources.json.gz retains same HEAD/uncommitted
tree/status/diffstat/UTC/2099 sources-L0. before.json.gz92 pairs, extra-before26,
boundary-before10, repeat-before4. Same oracle pin/SHA above; Tekt initial
`2cc69a555e9738dc659c06ed6d933853ab981cbb73e93c388a5b43c55afad095`.
160pt/margin8/font12 hanging10: first x8, continuation8→18; negative−5 x3.
RTL first right152, continuation142. Tekt ignores Length. MLA default 18pt(1.5em),
ambient 3em does not override, show-set 3em requires 36pt; IEEE/Plain gain none.
bibliography.where(style:"mla") does not match vanilla probe: preserve selection,
force no override. Inline-only Block/Box ignores hanging10; explicit Par inside/
Block with Parbreak applies. Simple Heading/Raw/List/Quote does not. Partial/
zero/repeated/empty scope declarations close boundaries: FIRST/MIDDLE/LAST
5.168/27.464/49.76 versus all5.168 Tekt. Initial literal-bracket probe refuted,
corrected to #[...] and archived extra-before-initial, not Styled-scope evidence.
Extreme values clip ink in extractor: absent pdftotext is not erased Content.
partial-font 24pt does not change shared 12pt em.

Pinned realize lib.rs:1061–1085,1190–1219 distinguishes fragment/Par/interruption;
flow/collect.rs:72–134 selects flow; inline/mod.rs:174,216–221/collect.rs:141–147
resolves shape; line.rs:496–503 positions; rules.rs:510–523 CSL inset/compensation;
bibliography.rs:112–127,367 defaults/precedence. Language geometry, not required
Rust structure/historical intent. Lost boundary/run-em/double indent/page reset/
forced greedy fallback/lost copy/repeated callback/control regression refutes closure.

### Current exclusive transport and acceptance

Internal continuous ADR-0127, no public field/signature/new external default-flag/
phase/realization order. Initial/continuation shape supersedes corresponding
constant-width limitation only. Certified Latin RTL follows pinned nucleus,
no active BidiCursor/tracking/invented BidiSpan; other limits stay. Offpage values/
copy require frames/SVG inspection, not equal clipped extraction. Earlier physical
scope did not itself authorize later textual-causal package.

Par arm validates/evaluates hanging-indent once and transports Length/local
occurrence. Alternate private eval.par.hanging-boundary Bool per chain frame so
diff_styles retains repeated set; consumer tests key presence, never bool value.
Remove only hanging unsupported-property warning, preserving other properties/
warnings. This supersedes earlier valid-Length warning-only limitation.

Precode RED→GREEN/source-layout/bilateral sign/boundary/variant/restoration
controls. Grouped-arm execution proves no all-alternative coverage; closed inline/
flow classifications require class/variant controls. Valid V15/V26 before reseal,
tests/build/strict Clippy-Rustdoc/fmt/reverse/differential Tekt at pin1279707,
12GiB/High 8 GiB/swap0/jobs1. No publication.
