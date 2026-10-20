# L0 — core/compiler/stdlib/mod
Hash do Código: c83d648e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:3c1f911b856e019f09d75d07fd14052912e8f0fb6118fceaaac9a7f4a92bfd97
- 00_nucleo/prompts/_nuclei/math/matrix-morphology.toml sha256:dced4bcb1e45b09423bb3b9739af4da3cd440d500d6a620e08e13219236d1735
- 00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml sha256:a933254df6e90a505910759d443fe8b06f2646e2d4a9949bc179326fc9c5c329
- 00_nucleo/prompts/_nuclei/selectors/math-native-bridges.toml sha256:99bae094b60a12dbe90a88c5e32df53c0422584e53097bdad7892943337ca219

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/mod.rs`

## Internal selector conversion linkage

At checkpoint `370d78355bd646d1abead5bcacef215288cc8834`,
`01_core/src/compiler/stdlib/foundations/selector.rs` contains the native-kind
identity map used by construction, while the evaluator adapter has an incomplete
independent map. The measured admission cases and source receipts in
`lab/results/v16-corrections-2026-10-19/batch-c/agents/selectors/` support sharing
that existing pure operation. Add only a direct crate-visible reexport of
`foundations::selector::element_kind_of_native_func`. Keep foundations private,
all external visibility and public signatures unchanged, and put no mapping or
constructor logic in this hub. This internal continuous-flow linkage requires
L0 sealing first, consumer semantic RED→GREEN and existing workspace/path/lineage
gates. It does not resolve the separate native_flush visibility discrepancy.

## Evidence before specification


`01_core/src/compiler/stdlib/mod.rs:212` (pub super) fn errmsg: impl Into<String>) -> SourceResult<Value> {); `01_core/src/compiler/stdlib/mod.rs:221` (pub super) fn expect_no_named) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.



Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

Equation-where-2026-10-06/before.json.gz preserves dirty HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/stat/binary hashes/source/pin: Equation public fields/filter/alias/composition/show differ while internal size/unknown/partial/closure reject. Pinned func 433-462/equation 58-170 and existing block/body snapshots support internal bridge, not new public API. matrix-morfologia-2026-10-06/before.json.gz shows missing rows/getters/func identity despite existing carrier, not render proof. imports-ci-2026-10-13/before.json.gz distinguishes unused hub alias from used matrix-local implementation. Native-math candidate-1 strict-details/state receipts isolate items_after_test_module order after otherwise-passed focus matrix; changed final source needs final revalidation, old run is historical.

`lab/results/construction/diagnostics/native-math-catalogue-2026-10-17/` before.json.gz fixes dirty HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/diff HEAD --stat and protected consumer/L0/config hashes. before-measurement.json.gz is verified reuse, not fresh execution: baseline-reuse.json proves identical source/Tekt binary to 3956 function cases/eight feature profiles.26 math where/kind routes differ against historical 9f2b6e871; before-batch.json.gz has 227 field/exclusion/partial-alias/counter cases with hashes/commands/binaries/outputs. batch-schema.json preserves pinned source blocks/SHA/file:line/Tekt ABI; typst-macros elem.rs:410-421 excludes internal/external field_id, and 26 elem annotations are Mathy/non-Locatable. Public-language form/semantics, not Rust Func equality or intent inferred from rendering.

`lab/results/construction/diagnostics/native-format-catalogue-2026-10-17/` before.json.gz preserves dirty HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/status/stat/hashes; baseline-reuse.json proves same sources/config/binary as 3956-case/eight-profile prior matrix. before-batch.json.gz measures 144 detailed, before-compat.json.gz 456 name/alias/namespace controls with argv/binaries/outputs. Fifteen remaining where/kind routes differ; shared native_format pointer cannot distinguish five schemas. pinned-schema.json.gz preserves nine upstream declarations/blobs/lines/elem attributes. FuncRepr is pub(crate), as retificacao-func-repr.md refutes old public claim; public-contract-before.json fixes signatures/reexports/structs. This is internal representation/parity, no inferred intention from render.

`lab/results/construction/diagnostics/call-once-2026-10-17/before.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, dirty tree/UTC/status/diff HEAD --stat/sources/L0/config/hashes;2124 predecessor hashes were checked exactly before baseline. before-probe.json.gz/before-math-probe.json.gz retain bilateral binaries/argv/fixtures/outputs (historical oracle 9f2b6e871 binary `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`). Receiver/args repeated, missing methods evaluate panic args before causal error, math.mat duplicates accumulation and empty-array truncate hid it. Local call/collection/context/content/selector/method/invoke paths violate existing call_dispatch once-only contract; pinned-source.json.gz fixes upstream call/methods evidence. Invalid initial reserved-word `or` fixture is retained separately, no call-effect credit; its import panic 101 remains parsing/import debt.

## Exclusive façade contract


Own the stdlib façade: module declarations, explicit reexports, local err/expect_no_named helpers and cfg(test) test-module composition. Preserve established Rust public/internal paths and their specified visibility, with discrepancies explicitly retained below; do not create wrappers, duplicate namespace factories or move algorithms into the hub.

`err` produces one Error diagnostic with a detached span. `expect_no_named` rejects the first unknown named key in insertion order with the existing literal `argumento nomeado inesperado: 'KEY'` and a detached span. These helpers do not replace owner-specific origin-sensitive validation.

The actual native ABIs are the current Func native/Engine interfaces; the historical shortened signature is not an API specification. Per-native casts, diagnostics, return values and World use belong to their owners. Numeric helpers and promotion rules are not implemented here.

Registry composition belongs to `00_nucleo/prompts/core/compiler/stdlib/registry/mod.md`; eval keeps its own façade/call dispatch. Preserve the direct linkage of make_stdlib_with_features, calc_abs, format factory, collection call-span metadata, float/int/version dispatch, array-from-bytes, codec encoders, outline entry operations, spot construction and all existing native reexports. Do not globally register aliases from this façade.

Constructor hubs `00_nucleo/prompts/core/compiler/stdlib/foundations/mod.md`, `00_nucleo/prompts/core/compiler/stdlib/text/mod.md`, `00_nucleo/prompts/core/compiler/stdlib/structural/mod.md` and `00_nucleo/prompts/core/compiler/stdlib/primitives_constructors.md` own only their composition. Production behavior belongs to the respective descendants; the atomized test suite is owned by `00_nucleo/prompts/core/compiler/stdlib/tests/mod.md` and its test owners.

The contract requires the native_flush façade reexport to be pub(crate), not an externally public API. The current public reexport conflicts with that requirement. Keep this obligation explicit; the observed code does not supersede it. No Rust visibility change is made or authorized during sanitation, and the discrepancy remains for a separate contract/code resolution.



Provide direct native_math_equation and native_math_mat bridges with pub(in crate::compiler) to existing eval identity/getter tables; preserve all public exports/namespaces/signatures/factories. No string recognition/validation/schema/wrappers here. Remove only unused crate-internal math_matrix_augment façade alias (matrix child implementation stays, as do MathMatrixOptions/apply_math_matrix_option); its former path-preservation obligation is superseded only for this unused alias.

Propagate 23internal native bridges listed by `00_nucleo/prompts/core/compiler/stdlib/structural/math.md` directly structural→compiler::stdlib, pub(in crate::compiler), before inline test module alongside composition items. Do not resolve native_flush discrepancy or alter other preexisting items/ABI/visibility. Add direct pub(in crate::compiler) html::native_html_frame/pdf::native_pdf_attach/pdf::native_pdf_artifact exports before tests, no duplicate already-public native_html_elem. Crate-only CollectionDispatch/try_dispatch_collection_method_with direct exports preserve previous exports, no logic/wrapper.

## Acceptance and limits

Preserve façade architecture/detached local helper diagnostics/owner-specific origin-sensitive validators/public paths/pins/1:1 ownership. Meaningful consumer RED→GREEN and bilateral Equation public fields/rejections/aliases, matrix fields/has/at/func/JSON/query/padding/presence; retain bare math/render/other-unmeasured/generic Styled exclusions. Internal composition/unused alias/order cleanup changes no runtime/default/phase/public signature or performance; full core/build/Clippy/lineage, no suppression/test expectation deletion. Native_flush public-versus-required-crate discrepancy remains explicit pending contract/code resolution, observed code not supersession.

Apply pinned math-native-bridges invariants with exclusive owner boundaries. Internal table/composition correction follows ADR-0127 continuous flow without external field/trait/signature/compatibility/default/phase change. L0 precedes functional identity-missing RED then GREEN 26 cases/filters/exclusions/With/homonym/non-element helpers; compare complete matrix/aliases preserving unrelated observables (global Crystalline accent/cancel included, no parity credit when absent upstream). where/kind success does not certify getters/synthesis/index/show/layout/counter; counter controls/admission remain separate. Other fifteen format/HTML/PDF routes and callback collision require own investigation, not naming fixes. FG1/RM1/SY1/RG1 retain gates. Ownership/V26 before restricted five-pair reseal; build/full suite/fmt/strict Clippy/Rustdoc/lineage/global no new violation, sequential 12GiB/High10GiB/swap0/jobs1.

Apply pinned format-native-identity nucleus. Native/Format grouping is justified by same payload/ABI for projection/execution, but test every alternative for name/namespace/pointer/Eq/Hash/clone/With/dispatch; inventory/link V19 to tests without suppression, explicitly name new variant in exhaustive matches. No external definition/signature/field/trait/compatibility/default/phase changes, ADR-0127 continuous flow. Before code functional RED via current factories for fifteen cases and real observation against unmarked Native; GREEN full corpus/144 field-call/456 compatibility outside where/kind, public-contract and sizeof before/after. L0/ownership/V26 before restricted eleven-pair reseal; build/workspace/strict Clippy/Rustdoc/fmt/strict-reverse lineage/global no new errors/warnings. Sequential 12GiB/High10GiB/swap0/jobs1. Only where/kind admission closes; full selection/show/query/synthesis/carrier morphology/counter, FG1/RM1/SY1/RG1/V16 retain separate scopes/gates. No old-candidate copying or publication.

Consume pinned call-once invariants. Internal ADR-0127 continuous-flow correction supersedes only conflicting internal-signature/name-compensation restrictions, retaining Tekt extensions, public functions/reexports/structs/traits/fields/default/phase. Functional co-located RED→GREEN for observed receiver/args effects, mutating fallback/error-before-args, where/With/selector/collections/matrix without truncating legitimate effects; hits/misses, spans/occurrences and Value complements. Unhandled preserves receiver and does not execute args callback; count effects, not PDF bytes/Rust Eq/benchmark. Preserve residual diagnostics explicitly. Ownership/V26 before restricted resealing; build/workspace/strict all-target Clippy including unfulfilled expectations/Rustdoc/fmt/strict-reverse V1/V5/V15/V26/global differential SARIF, no new error/config/suppression. Informative V19 retains alternatives matrix and behavioral relation. Sequential 12GiB/High10GiB/swap0/jobs1. No publish/commit/merge/release/global-completion claim.

## Internal Figure cast bridges

The approved Figure/Grid integration at baseline
`731f0ffdf4c7c152f590551e72ff63f7bd85ef85` moves kind preparation into the
compiler realization owner. Its first workspace check records E0603 for access
to the private structural module in
`lab/results/v16-corrections-2026-10-19/figure-kind-implementation/integration-check.log`.
The existing pure `is_known_math_accent` and `is_callable_lr_symbol` classifiers
already own the measured Symbol callable distinction. Expose only these two
functions through crate-private reexports so realization preserves that exact
classification without copying tables or changing external APIs. Structural
modules and classifier algorithms remain unchanged; this hub owns no new domain
logic. Validate existing Symbol diagnostics, workspace compilation, topology,
canonical lineage and the complete Figure acceptance suite.
