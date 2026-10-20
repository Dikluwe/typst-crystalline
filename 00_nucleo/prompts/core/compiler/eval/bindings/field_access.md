# L0 — compiler/eval/bindings/field_access
Hash do Código: bae2d93c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:e79fb4e7f291d69f9108a4b17738b205861d818c6e9eeea873c337dc64bad2f8
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:09b32666fb1b1fa7ba8f2ef7c979c60cbc64cc515b8d9c46818bec19d7471fb1
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:3c1f911b856e019f09d75d07fd14052912e8f0fb6118fceaaac9a7f4a92bfd97
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:d9b2ab50748dec1246d8a55c0b702fa2ee5e23dd737bedf3689e6c9f0297ac29

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/bindings/field_access.rs`
**Internal atomization:** facade preserves interface/tests. Individual owners
under `00_nucleo/prompts/core/compiler/eval/bindings/field_access/` own identity,
fields, callee errors and Content methods. Clauses below remain their normative
boundary criteria.
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/bindings/field_access.rs:434` — `fn native_public_name`.
- `01_core/src/compiler/eval/bindings/field_access.rs:451` — `fn is_user_defined_closure`.
- `01_core/src/compiler/eval/bindings/field_access.rs:474` — `fn is_format_pdf_namespace`.
- `01_core/src/compiler/eval/bindings/field_access.rs:479` — `fn is_format_pdf_function`.
- `01_core/src/compiler/eval/bindings/field_access.rs:497` — `fn is_a11y_pdf_target`.
- `01_core/src/compiler/eval/bindings/field_access.rs:504` — `fn is_effective_format_module`.
- `01_core/src/compiler/eval/bindings/field_access.rs:511` — `fn is_effective_std_module`.
- `01_core/src/compiler/eval/bindings/field_access.rs:519` — `fn is_effective_symbol_module`.
- `01_core/src/compiler/eval/bindings/field_access.rs:537` — `fn warn_deprecated_symbol_binding`.
- `01_core/src/compiler/eval/bindings/field_access.rs:557` — `fn eval_field_access`.
- `01_core/src/compiler/eval/bindings/field_access.rs:707` — `fn eval_value_field_access`.
- `01_core/src/compiler/eval/bindings/field_access.rs:1057` — `fn element_or_type_with_name`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/fields.rs:14–67`.
- `foundations/func.rs:280-308`.
- `foundations/func.rs:412-450`.
- `lab/typst-original/crates/typst-eval/src/code.rs:347-366`.
- `lab/typst-original/crates/typst-eval/src/code.rs:347–366`.
- `lab/typst-original/crates/typst-library/src/foundations/func.rs:280–308`.
- `lab/typst-original/crates/typst-library/src/foundations/func.rs:291–308`.
- `lab/typst-original/crates/typst-library/src/foundations/module.rs:139-150`.
- `lab/typst-original/crates/typst-library/src/foundations/value.rs:157–169`.
- `lab/typst-original/crates/typst-library/src/lib.rs:221-225,374`.
- `typst-eval/src/code.rs:347-367`.
- `typst-eval/src/code.rs:347–366`.
- `typst-library/src/symbols.rs:12-35`.

## Contract

Own eval_field_access, eval_value_field_access, field_callee_error,
content method projection, queried-content projection and deprecation adapters.
Keep the existing static catalogs and visibility; method implementation remains
delegated to its native owner. Resolve a receiver once and anchor pure lookup
errors at the supplied field span, without reconstructing syntax or origin.

Native function fields traverse the actual namespace (including With unwrapping).
Unknown native fields use the canonical function identity/name, not a lexical
alias. User closures, also through With, retain
`cannot access fields on user-defined functions`. Do not generalize that rule to
plugins or element functions. Module errors use the stored module name:
``module `<name>` does not contain `<field>` `` with no hint, at the field only.
Aliases of std refer to the object named global; an ordinary std.typ module stays
std. Never identify std by a local spelling or by comparing its bindings.

Dictionary, raw-content and float lookup preserve their distinct diagnostics and
field-only anchors. Other scalar misses use canonical long type names. Array
len/first/last keep their existing AST routing; unknown array fields fail as
array field access. Preserve the known distinction between isolated fields and
bound-call interception rather than making all methods first-class values.
In particular isolated Float is-nan/is-infinite/signum/to-bytes, Angle deg/rad,
Version at, Func where and SpotColorant tint do not become bound method values.
Type lookup exposes only its actual static methods; Type::Function is not thereby
a general constructor. Delegate static float/version/spot operations to stdlib.

Keep Duration days/hours/minutes/seconds, Version major/minor/patch,
Length abs/em, Relative length/ratio, Args positional/named, Stroke paint/thickness
and Alignment x/y projections (the corresponding single axis or None).
For contextual text.size, prefer the typed top style
size over a custom field and preserve derived script size; use the existing
default 11pt only when absent. text.lang keeps its existing contextual lookup.
Do not execute a style/font callback while projecting a field.

Content exposes func/has/at/fields/location through the closed ContentField
projection and existing helpers. Distinguish absent from a present None/Auto.
Consume self/default arguments coherently using the method_dispatch helpers.
OutlineEntry.body routing stays restricted to actual entry content; entry level,
element and fill projection remains distinct from body evaluation.

For LocatedContent Some, the pinned snapshot nucleus is authoritative: use its
complete ordered fields, not raw fallback, styles or reconstructed defaults.
Missing at uses the supplied default only when the field is absent. Preserve
Location independently. LocatedContent None keeps legacy projection, including
realized Equation. Raw projection uses causal presence masks. Equation exposes
body/block and explicit styles; alt is Str or explicit None and precedes body,
with top style precedence and omitted alt absent. MathPrimes exposes count;
MathLr exposes explicit size then body; MathStretch always exposes body and
exposes size only when explicit. Annotated math variants always expose body and
expose annotation only when present, including explicit None, in canonical order.

Feature gates use effective typed native/module identity, not homonymous
dictionaries or aliases by spelling. The three gated pdf members require
A11yExtras; absence produces the existing field diagnostic with its two hints.
HTML format fields require Html, bundle format fields and std.asset require
Bundle; features remain independent. Identity of the actual format namespace
is its existing surface, not a fabricated default projection. Active features
use ordinary lookup and the real constructor namespace, never a placeholder.

Symbol deprecation warnings are delegated to the sym owner and emitted on
successful root binding resolution with the existing causal anchor. Missing
roots do not warn; an invalid modifier of a deprecated root preserves the root
warning. Keep legacy join as a measured extension and the un-deprecated tack
bindings silent. Preserve the narrow tri fallback only for canonical gt/lt with
eq/not modifiers after normal modifier lookup fails; use sym lookup, preserve
the legacy routes and repr parents, and do not recognize arbitrary named Symbols.

Keep messages, severities, hint order, traces and spans unchanged. Public field
availability or generic method-value promotion requires a separate contract
decision; this owner does not authorize it.

## Test-only reexport evidence and boundary

After bindings-hub alias was cfg(test), all-targets build identifies corresponding
field_access.rs:451 alias as unused productively. External consumer is
stdlib/tests/introspection.rs:163; productive calls use content_methods directly.
HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree;
sources/UTC/diffstat in imports-ci-2026-10-13/before.json.gz and
first-check-state.json.gz. Keep eval_content_method_at reexport cfg(test), its
test visibility/productive definition/other reexports. Internal ADR-0127 composition,
no new public API/phase/default; revalidate old tests/build/lineage.

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

Reexport only fields owner's check_module_field_feature internally to eval; absorb no logic.

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

### Exclusive borrowed-field facade

Only reexport eval_field_access_with_target internally to eval, no absorbed logic.
Existing pure APIs remain.

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

## Type::Array diagnostic-control correction

candidate-5/workspace/workspace.log records one failure in old
p1338_preserve_other_categories_diagnostics: #array.nope still expects double
quotes/whole array.nope anchor. Historical pin9f2b6e871 missing/array emits
``type array does not contain field `__missing` `` with field-only span;
code.rs:347–368 uses field.span. Call RED/full bilateral comparison established
contract before code. Update only that #array.nope test line to backtick message/
nope anchor. Preserve length/boolean/none/auto/integer/string cases/severity/
spans/hints/traces/profiles, remove no assertion or ignore test.

## Measured historical text.text residual correction

workspace.log in
`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17`
records p1325_content_direct_alias_and_known_lookup_residual failing because
[x].text now returns text, matching before-pdf-probe.json.gz bilateral witness at
`bd5ebec285505889e8ff1d4f68804ab147a699f3`. Old error assertion was known
residual, not intended contract. Remove only that negative entry; add explicit
[x].text == "x" to existing positive test. Keep unknown-field/alias/Unicode/all-
profile negatives. Hide no presence regression or change productive facade
dispatch. L0 before test/reseal/full suite.

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

### Exclusive export and acceptance

Only reexport native_sequence directly from content_methods owner to bindings hub
with internal visibility, no construction/selection logic here. Preserve every
other export/test. Completes earlier R1-named R2 responsibility. Continuous
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


## Approved Figure/Grid kind repair (2026-10-19)

### Measurement and classification before decision

The frozen proposal at baseline `731f0ffdf4c7c152f590551e72ff63f7bd85ef85` in `lab/results/v16-corrections-2026-10-19/figure-kind/review.md`, its `agents/contracts/design.md`, `agents/contracts/semantic-partition.json`, and `agents/counter/design.md` bind measurements and full source/oracle provenance. Current-pin oracle is `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Constructor inference loses typed identity and lexical occurrence styles; semantic inspection and final supplement need separate phases. These are language semantics and causal effects, not a requirement for upstream Rust mechanics. The human explicitly approved the exact public carriers, phase changes and compatibility migration in the current session under ADR-0127.

### Exclusive responsibility

Preserve existing pure value/member lookup routing; Figure field behavior remains owned by field_access/content_methods with raw versus occurrence presence distinctions. Any test migration uses typed intent and preserves language assertions; no API change in this coordinator.

### Acceptance and boundaries

Require functional RED→GREEN, frozen kind/set/show-set/supplement/hidden-parent cases, alias/homonym and field snapshots, full build/tests, paths, strict forward/reverse lineage, V15/V26 and differential lint. No suppressions, opaque catchall, fake neutral child or weakened assertion. Caption omission/none, complete caption synthesis, supplement-dependent preview selection, complete function-number formatting, HTML implicit captions/references, Dynamic field enumeration, and mixed Table header/ordinary-cell supplemental target order remain explicit bounded debts.
