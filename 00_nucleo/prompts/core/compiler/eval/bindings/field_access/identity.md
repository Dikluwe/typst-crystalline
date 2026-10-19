# L0 — compiler/eval/bindings/field_access/identity
Hash do Código: d1985474

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:e79fb4e7f291d69f9108a4b17738b205861d818c6e9eeea873c337dc64bad2f8
- 00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml sha256:a933254df6e90a505910759d443fe8b06f2646e2d4a9949bc179326fc9c5c329

## Ownership and extraction evidence

Layer L1. Sole productive consumer:
`01_core/src/compiler/eval/bindings/field_access/identity.rs`. At clean HEAD
`ba5a4c7d305ed1444660d9eb80862f51b417efb6`, field_access.rs:434–556 contains
extracted behavior. SHA/time/initial state in
`lab/results/construction/diagnostics/atomizacao-field-access-2026-09-30-before.json.gz`.

## Exclusive identity boundary

Identify native functions/effective modules; emit deprecated-symbol warnings
from resolved binding. Preserve field_access parent's signatures/semantics:
`00_nucleo/prompts/core/compiler/eval/bindings/field_access.md`. Static same-layer
L1, no I/O/mutable globals. Preserve L1/field-access tests/crystalline-lint with
valid ownership/hashes; no inferred performance gain.

## Native-format identity evidence and application boundary

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree.
`lab/results/construction/diagnostics/native-format-catalogue-2026-10-17/before.json.gz`
retains UTC/status/exact diffstat/hashes; baseline-reuse.json proves source/
configuration/binary equality with prior3956-case/eight-profile final matrix.
before-batch.json.gz measures144 detailed cases; before-compat.json.gz456 name/
alias/namespace controls. Probes keep argv/binaries/output. Historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`, executable SHA
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Fifteen remaining routes reject where/kind only in Tekt. format.rs:17–27/96–146
shares native_format across five formats, so pointer alone cannot distinguish
schemas. pinned-schema.json.gz retains nine upstream declarations/blob/hash/
lines/elem attributes. FuncRepr is pub(crate), entities/func.md:21/func.rs:24;
retificacao-func-repr.md refutes prior claim L0 made it public.
public-contract-before.json fixes public signatures/reexports/structs. Internal
representation/parity classification, not new public contract or render-inferred
intent.

native_public_name includes Format payload with Native, including With unwrapping.
is_user_defined_closure explicitly classifies Format false. is_format_pdf_function's
Native arm also accepts Format, preserving exact current name/namespace validation,
gates/aliases. Never use it to authenticate element capability. Keep std/format
module policies, symbol warnings/a11y diagnostics.

### Acceptance and scope

Consume pinned format-native-identity nucleus. Native/Format may share body
because payload/ABI/projection/execution are same; independently exercise both
in name/namespace/pointer/Eq-Hash/clone-With/dispatch tests. Inventory new V19
unions/link tests, no suppression. Explicitly name new variant in closed matches.
Continuous ADR-0127: no new external definition/signature/field/trait,
compatibility/default/phase. Before code, real functional RED for15 routes via
current factories plus real observation against unmarked Native. GREEN requires
full corpus/144 field-call cases/456 compatibility controls unchanged outside
where/kind. Compare public-contract-before/after and sizeof before/after.

L0 before code, ownership/V26 before restricted11-pair reseal. Build/full suite/
strict Clippy-Rustdoc/fmt/V1/V5/V15/V26/reverse/global differential without new
error-warning. Heavy tasks sequential MemoryMax 12 GiB/High 10 GiB/swap0/jobs1.
Track legitimate V19 with coverage. Closes where/kind admission only, not all
selection/show/query/synthesis/carrier morphology/counter. FG1/RM1/SY1/RG1/prior
V16 retain scope/gates; copy no old candidate or publish.

## Direct nominal-alternative coverage

candidate-1/state.json.gz/differential-review.json in native-format-catalogue-
2026-10-17 record eight new V19/four updated. Updates only include Format/increase
count, not removed violations. First comparator wrongly demanded identical
message. Add direct tests without productive changes/suppression/single-path
group proof: native_public_name/is_user_defined_closure with Native/Format/Engine/
Closure/Element/Plugin/With, including closure/format With. is_format_pdf_function
covers both compatible native arms, non-PDF formats and remaining types. Name/
namespace remains field/gate policy, no element-capability authentication.

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

### Exclusive effective-module recognition

is_effective_std_module/is_effective_format_module delegate to Module's internal
predicates. Remove authentication via name/type/document/svg/png/pdf/html.
Keep deprecated-symbol/PDF-a11y/function-identity policies. Test Module::new native
scope copies and real clones, not empty-name-only cases.

Consume pinned builtin-module-identity nucleus. Private marker implements existing
gates, not public contract; continuous ADR-0127 internal/parity. Module remains
publicly opaque without new exported field/method; compare public declarations/
signatures before/after. No public/phase/default change.

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
