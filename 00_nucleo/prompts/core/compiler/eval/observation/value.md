# L0 — compiler/eval/observation/value
Hash do Código: 389ebb49

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/observation-relation.toml sha256:6f6d86b0ee72d314a84d86b12f9524b2959e075b29f99fcf6d2de9f30a4b338a
- 00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml sha256:a933254df6e90a505910759d443fe8b06f2646e2d4a9949bc179326fc9c5c329
- 00_nucleo/prompts/_nuclei/selectors/native-bare-identity.toml sha256:7e7373acce2b00247d18b9c5ea1b9d53833275997cd2c304e9d919d2cce10648

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/observation/value.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/observation/value.rs:11` — `impl ObservationEq for crate::entities::value::Value {`.
- `01_core/src/compiler/eval/observation/value.rs:249` — `impl ObservationEq for crate::entities::path::VirtualRoot {`.
- `01_core/src/compiler/eval/observation/value.rs:265` — `impl ObservationEq for crate::entities::package_spec::PackageSpec {`.
- `01_core/src/compiler/eval/observation/value.rs:274` — `impl ObservationEq for crate::entities::package_spec::PackageVersion {`.

## Contract

Own the exhaustive Value match and runtime, selection, argument, diagnostic,
function and module carriers outside the other observation families. Compare
variant, presence, order and payload. Scope retains binding order and values.
Native functions compare call identity, name and namespace; closures are Same
only by Arc identity. Plugins and services lacking proof remain Unproven.
Diagnostics distinguish success, errors, spans and causal metadata. Never
execute a compared value. Signed zero and NaN retain their bit distinctions.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

## Additional native-selector observation consumer

Before deciding, table-cell-selector-2026-10-06/check.log records E0004 at
observation/value.rs:470 after ratified public variant. Match uses Self::Element,
so Selector::Element inventory missed it in manifest. Existing L0 requires
exhaustive variant/payload relation; no silent missing branch.

Internal continuous ADR-0127: NativeElement compares only NativeElement through
existing Func ObservationEq (call/name/namespace), without execution; other
variants Different. This conservative observation relation does not replace
language equality's real-address contract in equality owner. Names do not prove
element identity. No public/phase change beyond ratified variant. Test/validate
with D08A set, resealing only this additional owner.

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

Compare Format only with Format by kind and NativeFunc call/name/namespace,
without execution. Kind uses private Eq, callback fn_addr_eq, name/namespace
existing relation. Explicitly name Format on exhaustive unlike-pair axis.
Native/Format cross pairs are Different even if public Eq is true: this observes
causal element capability without replacing language equality with mechanics.
Keep other Value/Func/Module branches/priorities.

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

## Direct coverage of nominal alternatives

candidate-1/state.json.gz and differential-review.json in native-format-catalogue-
2026-10-17 record eight new V19/four updated. Updated groups merely include Format
and raise count, not removed violations. Initial comparator incorrectly required
identical message and rejected update.

Add direct complementary tests only; change no productive behavior/suppress no
note or treat one path as group proof. Matrix of five kinds with fixed name/
callback/namespace isolates witness; separately vary callback/name/namespace to
show each causal axis matters. Execute no function. Public Eq may stay true while
observation is Different.
