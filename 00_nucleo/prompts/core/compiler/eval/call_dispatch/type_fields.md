# L0 — compiler/eval/call_dispatch/type_fields
Hash do Código: 87cda106

## Ownership and extraction evidence

Layer L1. Sole productive consumer:
`01_core/src/compiler/eval/call_dispatch/type_fields.rs`. At clean HEAD
`d5c563dfab39f789e8e2f72c9a9364f5a761e3ca`, call_dispatch.rs:604–721 contains
extracted behavior. SHA-256/time/initial state are in
`lab/results/construction/diagnostics/atomizacao-call-dispatch-2026-09-30-before.json.gz`.

## Exclusive contract

Resolve static callable type fields, preserving native identities/exposed functions.

Preserve public signatures/diagnostics/interception priorities/traces/argument
semantics in `00_nucleo/prompts/core/compiler/eval/call_dispatch.md`. Static L1,
no I/O/mutable globals. Preserve L1/call_dispatch tests and crystalline-lint with
valid ownership/hashes. Infer no performance gain.

## Measured exhaustive type catalogue

### Evidence before decision

`lab/results/construction/diagnostics/tekt-v16-catalogos-2026-10-17/before.json.gz`
and before-measurement.json.gz fix HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/status/
diffstat/sources/L0/config/binaries. Inventory118 fields:115 CLI-exercised and
three Location module tests.159 pairs/318 invocations:141 match exit/stdout/main
error+hints,18 retain existing diagnostic divergence. Exercised inventoried
functions/constants exist on both backends; omitted-field absence upstream is
not proved. Historical pin `9f2b6e8715237cb086899a42873660fe744622e8`,
binary SHA `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Snapshot value.rs has38 Type/39 Value variants; flagged wildcards mix closed
families with open field/method names. Prior state/locations remain inventory.

### Current classification and acceptance

Combine old constant/function queries into exhaustive Type match. Nine families
Direction/Alignment/Angle/Function/Duration/Length/Selector/State/Location use
internal open-str name lookup. Keep constants/names/native identity/Native versus
NativeWithEngine/signatures; same-named fields on different types stay distinct.
Unknown name returns None only within classified family. Other29 Type variants
share V19: this owner supplies no catalogue for those families and delegates to
other owners/caller, not a claim they have no language fields. Explicit enumeration
requires local classification of future Type. No stringified type dispatch or
V16/V19 suppression.

Internal structural continuous ADR-0127, no new API/default/compatibility/phase;
keep runtime intent without universal parity certification. Structural RED:
old productive patterns accept future variant, after change current control
compiles/future fails E0004. Probe RHS erased only to isolate patterns, payload
stubs not execution equivalence tests. Do not mutate workspace/Cargo target to
probe future variants. Functional module controls pass before/after, no functional
RED claim.

Four V16 become three closed-family selections (first owner combines two matches).
Check159 pairs before/after preserving full stdout/stderr within each backend,
including18 divergent negatives for separate repair. No suppression/global lint
relaxation. Test same-name function identities, constants versus functions,
cross-family misses/Location. L0 first, only two owners resealed after V15/V26;
build/full suite/Clippy -D warnings/-D unfulfilled_lint_expectations, Rustdoc/four
indexes/fmt/diff/strict-reverse lineage/differential SARIF under 12 GiB/High 10 GiB/
swap0/jobs1. No commit/push/merge/release.
