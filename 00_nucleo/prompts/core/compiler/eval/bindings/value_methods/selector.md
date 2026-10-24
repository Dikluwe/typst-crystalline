# L0 — compiler/eval/bindings/value_methods/selector
Hash do Código: 5b7c7acf

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:3c1f911b856e019f09d75d07fd14052912e8f0fb6118fceaaac9a7f4a92bfd97

## Ownership and extraction evidence

Layer L1. Sole productive consumer:
`01_core/src/compiler/eval/bindings/value_methods/selector.rs`. Internal extraction
measured at clean HEAD ce27af795, 2026-09-30 23:12:44 UTC, from parent
value_methods.rs:1217–1335. Internal behavioral extraction, not a new parity
decision/bilateral measurement/performance proof.

## Exclusive adapter contract

Own Selector conversion/logical-contextual adapters. Preserve ordered composition,
argument evaluation/diagnostics/static dispatch/public contract/phase/default/
compatibility. eval_selector_or_and/within/before_after receive target: &Value
instead of Expr, removing only eval_expr(target_expr). Preserve conversions/
order/validation; evaluate args only after recognizing selector. Signatures remain
internal eval, not external Rust. Preserve L1 tests/cargo build/crystalline-lint;
structural split implies no speedup.

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

## Closed selector decisions and measured admission

### Evidence before decision

`lab/results/v16-corrections-2026-10-19/batch-c/agents/selectors/oracle-probes.json`
records the verified oracle commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
tree, binary SHA-256, recipe receipt, UTC, HEAD and exact working-tree diffstat.
`source-evidence.json` beside it freezes the consulted source. Upstream
`lab/typst-original/crates/typst-library/src/foundations/selector.rs:108-124`
defines literal/regex validation, :178-194 accepts Selector for logical operands,
:204-241 and :285-297 use LocatableSelector for contextual boundaries, :354-364
casts element functions by identity and :421-450 separately validates locatability.
These are language admission/equality/selection obligations, not required storage
or function-pointer mechanics. Successful eval of a show expression records
construction only, not realization; actual selection requires a compile probe.

The local adapter at `01_core/src/compiler/eval/bindings/value_methods/selector.rs:20-40`
only admits heading/figure; the constructor at
`01_core/src/compiler/stdlib/foundations/selector.rs:34-91` admits other element
identities and preserves a separate Tekt String-to-Kind extension. Current-pin
probes refute treating the adapter's refusal as intentional language behavior.
Native page equality is true against an alias and false against page.where().
The catalogue admits engine-native page/op; the old equality address accessor
excludes this ABI. The native matcher exits on engine ABI before causal descent.
Inference: existing representations suffice for these corrections; a case
requiring new public identity or phase would refute that bounded inference.

### Owned conversion and acceptance

Keep a closed nominal Value match at value_to_query_selector. Existing Selector
is retained. Native Func admission uses the existing identity-based element
catalogue, preserving legacy Kind representations wherever native_selector uses
them; other catalogue elements retain a bare NativeElement, never a fabricated
empty filter group. Reject closures, partial applications and homonymous functions.
Label and Location preserve their typed selectors. A nonempty String operand is
literal text via escaped Regex, never a legacy Kind-name reinterpretation;
nonempty Regex that cannot match empty text is accepted. This contextual operand
conversion does not change native_selector(String)'s existing Tekt extension.
All remaining Value variants explicitly refuse conversion. None means refusal,
not an empty valid selector. Invalid text/regex retain refusal without panics;
call-site diagnostics remain separately owned.

Do not expand method-receiver eligibility accidentally when admitting operands:
recognize existing selector/native-element receiver routes before evaluating
arguments. Arguments execute once only after receiver/method admission. Logical
composition preserves order. Contextual adapters validate the supplied boundary's
locatability using existing typed metadata, including nested compositions, before
constructing Within/Before/After; base-selector locatability remains query's gate.
No constructor/callback execution or function-name matching.

Tests first distinguish actual native alias/homonym, native versus converted
operands, literal regex escaping, empty-match refusal, Label/Location, scalar
refusal, argument effects and nonlocatable cell/page contextual boundaries.
Retain query/counter rejection of nonlocatable elements. Current-language
corrections use ADR-0127 continuous flow, with no public signature/entity change.

### Structural and integration gates

L0 first and restricted reseal precede Rust changes. Preservation controls pass
before/after; semantic corrections require real failing tests before code. A real
copied-workspace enum extension must produce attributable E0004 at every selected
closed match after repair, including both pair axes; use no stubs. Keep V19 groups
visible and review their semantic domains. No suppression, scan/severity/exclusion
change or equivalent open fallback hidden in another syntax. Validate full tests,
paths, direct/reverse lineage, V15/V26, build, all-target Clippy, fmt, Rustdoc and
exact SARIF delta. Preserve all earlier contracts and separately reported debts.

### Literal escaping within the L1 dependency boundary

`lab/results/v16-corrections-2026-10-19/batch-c/agents/selectors/literal-escape-provenance.json`
records the inspected working tree, UTC, lockfile and dependency source hashes.
Cargo.lock pins regex 1.12.3 and regex-syntax 0.8.10. Their cached crate archives
were verified against lockfile checksums and consulted source bytes against those
archives. The locked regex source `src/lib.rs:1351–1353` delegates escape to
regex-syntax. The latter's `src/lib.rs:213–221,262–268` prefixes a backslash before
each ASCII metacharacter in `\ . + * ? ( ) | [ ] { } ^ $ # & - ~`, preserving all
other Unicode scalar values. This establishes the current literal interpretation
and stored pattern identity; a direct crate dependency in this evaluator is not
required.

The candidate's direct external `regex::escape` call in this consumer violates
the L1 wrapper boundary (coordinator's full-lint V14 result). Replace it with a
private pure local String/char escape operation using precisely that metacharacter
set. Construct the regex only through the existing `entities::regex::Regex::new`
wrapper. Do not modify the wrapper API, dependency allowlist, lint configuration,
String-to-Kind constructor extension, admission rules or empty-string refusal.

This is an internal architecture correction under continuous ADR-0127 flow, with
L0 first and restricted resealing. The architectural RED is the actual V14; the
new functional controls are preservation controls, not claimed semantic RED.
Require them before and after: every metacharacter individually and in combined
syntax-looking literals; backslash escape-looking text; Unicode including a
combining sequence and emoji; whitespace/control literals; empty refusal; and
exact literal pattern identity used by the existing Regex language relation.
Substring selection must return the whole literal, without accidental captures
or empty matching; nonliteral alternatives must fail. The final full scan must
remove the direct-dependency V14 without suppressions or new violations.


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
