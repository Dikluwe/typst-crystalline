# L0 — compiler/eval/modules
Hash do Código: 30064f50

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:e79fb4e7f291d69f9108a4b17738b205861d818c6e9eeea873c337dc64bad2f8
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:9a193bdaa08a4eb5b41e9d4124388d0de24bd66776fedf04ef9273c1390462ec
- 00_nucleo/prompts/_nuclei/eval/source-validation.toml sha256:727a433beef7710d70729176c3b5a5f851b7e98be320c7594b86e5b269d8d49d

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/modules.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/modules.rs:44` — `fn eval_imported_file`.
- `01_core/src/compiler/eval/modules.rs:117` — `fn eval_module_import`.
- `01_core/src/compiler/eval/modules.rs:251` — `fn eval_module_include`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `lab/typst-original/crates/typst-eval/src/import.rs:82-103`.
- `lab/typst-original/crates/typst-eval/src/import.rs:82-105`.
- `typst-library/src/lib.rs:374`.

## Contract

Own imported-file evaluation, module import and include. Obtain sources and
package resolution only through World; preserve current FileId, Route cycle
detection, isolated Engine/scopes, exported bindings, sink and diagnostic traces.
Construct the imported global stdlib object with public name global under lexical
binding std. Ordinary imported modules retain their own stored names.

A bare Module expression uses the syntactic import binding name (identifier,
field or literal file-derived name), not Module::name(). Dynamic source without
as/items fails with `dynamic import requires an explicit name` and
``you can name the import with `as` ``. Dynamic as/items/wildcard imports remain
valid. Wildcards copy exported bindings; item aliases preserve missing-item
diagnostics at the original-name span. Include keeps string/path routes and
causal traces without changing caller bindings into imported exports.

After successful source validation, a bare identifier import with neither as
nor items emits exactly one `this import has no effect` at the source identifier,
without hints or trace. Preserve it even if a later expression fails. Field,
file, alias, items, wildcard, dynamic and failed imports do not acquire this
warning by generalization. Keep existing rename-warning limitations explicit
rather than broadening the warning contract. Existing Portuguese diagnostic
literals in the consumer are observables, not prose to translate.


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

### Exclusive imported-stdlib identity

Construct isolated imported-file stdlib via Module::stdlib(stdlib.clone()).
File-exported modules remain Module::new with file name and no std/format
authority. Keep World resolution/warnings/content/import-include semantics.

Consume pinned builtin-module-identity nucleus. Marker is private mechanics
implementing existing gates, no new public contract; continuous ADR-0127.
Module stays opaque, no exported fields/methods; compare public declarations/
signatures before/after. No public/phase/default authorization.

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


## Invalid imported sources and inherited context

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree.
`lab/results/construction/diagnostics/import-syntax-2026-10-17/before.json.gz`
retains UTC/status/exact diffstat/sources/L0/hashes;2125 prior hashes checked.
before-probe.json.gz keeps bilateral fixtures/argv/output/binaries;
before-context-probe measures eight profiles/real Bundle. Historical vanilla pin
`9f2b6e8715237cb086899a42873660fe744622e8`, binary SHA
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
modules.rs:108 enters eval_markup without root validation; include repeats at324.
mod.rs:649/778 validates main entries with distinct warning handling. Reserved/
mixed/nested panic101 at ast/mod.rs:94 while pin gives syntax error at real
source. Missing strings/expressions may be accepted, prefix panic precedes syntax
error, module warnings lost. Upstream lib.rs:71–82/128–137 validates
errors_and_warnings before eval; import.rs:36–47 records file trace. Infer no
intent from mechanics: acceptance is no crash/improper effect and causal
spans/hints/traces.

modules.rs:104–106 inherits only full_error: enabled Html/A11yExtras/Bundle
fails imports as disabled; module document in Bundle target fails as Paged.
before-context-probe's bundle exports child.html in pin, fails Tekt: propagation
error, not new target. Prior format:format.html attempt archived in
context-format-function-attempt has separate argument-contract divergence, not
target evidence; valid fixture uses .html/auto inference.

### Exclusive validation and context propagation

For local/package import, after Source/cycle resolution validate root through
helper before bare_name/stdlib composition/eval_imported_file; send warnings to
engine.sink.warn_diagnostic. Execute no invalid-file prefix or publish bindings.
eval_imported_file is private already-validated-source boundary. Valid file's
local_ctx inherits caller full_error/features/target, keeping apply_show_rules
false, local scopes/styles/show-rules/guards/current budgets. Add
Tracepoint::Import(path) at import.span to file validation/eval errors in include's
causal order; not object imports/bare-identifier warnings/local item-selection
errors. String/path include validates root after source/cycle and before child
Engine/effect; keep Include map_err, also on validation failures. Private static
validate+eval block may share map_err without API change. Preserve World/package/
cycle/lexical binding/other Portuguese diagnostic literals. Co-located current-
API tests cover errors/hints/origins/traces/no effects/warnings/package-path/
nested include/profiles/Bundle target.

Internal continuous ADR-0127, L0 before code. No public Rust/default/flag/
SourceDiagnostic-Tracepoint/World/Sink/eval-layout change. Private static helper/
closure; pinned source-validation nucleus shares policy across owners.

### Acceptance and open boundaries

Functional RED before implementation, no productive catch; tests may use
catch_unwind only to convert old crash to readable failure. GREEN requires errors
before effects, nonfatal warnings/stable dedup, imported-source provenance/nested
traces. In-memory World package, RootedPath include, all feature profiles and
Bundle document are mandatory controls. Keep main/expression/valid-file controls/
frozen corpus; no active-feature credit for expected disabled-profile rejection.
Probes retain complete stderr/HTML, no byte normalization to declare parity.
Existing markup/diagnostic differences are not silently equivalent. Parser/
include morphology/document.format/imports of unsupported categories remain
separate work unless corrected/measured in this scope.

Valid ownership/V26 before restricted reseal. Build/workspace/all-targets Clippy
-D warnings/-D unfulfilled_lint_expectations/strict Rustdoc/fmt/V1/V5/V15/V26/
reverse/differential SARIF without new errors/warnings. Extraction may move
uniform three-error-type V19 to new owner, keeping exhaustive match/justification.
Heavy tasks sequential, 12 GiB/High 10 GiB/swap0/jobs1. No commit/push/merge/release/
lint suppression.

## Closed-domain decision and preservation

### Evidence before decision

At checkpoint `370d78355bd646d1abead5bcacef215288cc8834`,
`01_core/src/compiler/eval/modules.rs:283–366`
and the caller/producer evidence recorded in
`lab/results/v16-corrections-2026-10-19/batch-c/agents/structural/l0-audit.json`
show the current decision and its refusal path. That receipt records UTC,
working-tree diffstat and exact source/owner SHA-256 before this amendment.
The existing clauses above establish the behavior to preserve; an open Rust
match is structural evidence, not proof of additional language admission.

### Exclusive policy

Only Str and Path are accepted by the following include dispatch. The listed complement
receives <invalid path> temporarily, then errors with include: caminho deve ser path ou
string, recebeu {} at path_span before trace_path is used. It never authorizes a file
read or include under that synthetic path.

The synthetic trace path is never passed to World: all non-Str/non-Path values
immediately fail in the following source dispatch before source lookup or Include trace
creation. Keep the existing Portuguese diagnostic, actual expression span and error
precedence. Test rejected None/Bool/Int/Content with a World whose include operations
panic, plus string and rooted-path inclusion and traces using the existing in-memory
World.

The explicit nominal complement is: `None`, `Bool`, `Int`, `Float`, `Array`, `Dict`, `Module`, `Datetime`, `Func`, `Content`, `LocatedContent`, `Auto`, `Length`, `Relative`, `Ratio`, `Angle`, `Color`, `Stroke`, `Fraction`, `Align`, `Location`, `Gradient`, `Regex`, `Tiling`, `Bytes`, `Decimal`, `Duration`, `Version`, `Selector`, `Symbol`, `Args`, `State`, `Counter`, `Label`, `Dir`, `Type`, `SpotColorant`.

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
