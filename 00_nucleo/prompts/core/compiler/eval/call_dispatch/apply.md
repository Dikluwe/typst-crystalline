# L0 — compiler/eval/call_dispatch/apply
Hash do Código: e392d5bf

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/selectors/format-native-identity.toml sha256:a933254df6e90a505910759d443fe8b06f2646e2d4a9949bc179326fc9c5c329

## Ownership and extraction evidence

Layer L1. Sole productive consumer:
`01_core/src/compiler/eval/call_dispatch/apply.rs`. At clean HEAD
`d5c563dfab39f789e8e2f72c9a9364f5a761e3ca`, call_dispatch.rs:503–603 contains
extracted behavior. SHA-256/time/initial state are in
`lab/results/construction/diagnostics/atomizacao-call-dispatch-2026-09-30-before.json.gz`.

## Exclusive contract

Apply closed FuncRepr to closures/elements/plugins/native functions, including native span selection.

Preserve public signatures/diagnostics/interception priorities/traces/argument
semantics in `00_nucleo/prompts/core/compiler/eval/call_dispatch.md`. Static L1,
no I/O/mutable globals. Preserve L1/call_dispatch tests and crystalline-lint with
valid ownership/hashes. Infer no performance gain.

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

Apply Format in same static Native arm using exact existing NativeFunc.call/
ctx/args/World/current_file. No recursion/wrapper/new phase/context installation
or restoration. native_fn_addr span identity keeps same pointer.

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
