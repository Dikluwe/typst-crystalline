# L0 — stdlib/layout/stroke — stroke
Hash do Código: 88dbd142

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/stroke-paint-inputs.toml sha256:b92ae8f26e4ebe8d5b7d0f1ee0e346e97210a9afb4c00ed1c9c184f690a263a8

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/layout/stroke.rs`

## Evidence before specification

L1 exclusive consumer `01_core/src/compiler/stdlib/layout/stroke.rs`; uncommitted layout.rs snapshot in `lab/results/construction/diagnostics/atomizacao-stdlib-layout-2026-09-30-before.json.gz` has L1 suite 6401 pass/2 ignored, structural evidence only, no benchmark/new bilateral proof. Parent `00_nucleo/prompts/core/compiler/stdlib/layout.md` retains full boundary criteria. Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

`lab/results/construction/diagnostics/value-compacto-memoria5-2026-10-08/` before.json.gz/inventory.json freezes each Value::Stroke file:line; sizes.json external model 272→144 bytes does not prove RSS. The public carrier is exclusively defined by `00_nucleo/prompts/core/entities/value.md`; human “confirmo”2026-10-08 ratifies manifest `6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237`, satisfying former proposed gate only for this package.

stroke-paint-2026-10-14/before/results.json.gz preserves HEAD/dirty tree/diffstat/UTC/sources/owners/commands/both binary hashes. Named paint is Color-only; old shape caster accepts Length/Color/Stroke and rect discards Dict. Uniform control draws 1pt black; Gradient/Tiling/Auto named and circle Gradient reject despite oracle acceptance. Empty Dict draws black versus no stroke in oracle. Pinned stroke.rs:236-243/sides.rs:238-258 separates smart Paint/empty/uniform Dict. square(size:...) fails dimensions, so proves nothing about stroke casting; those defaults/dimensions remain open.

## Exclusive contract

Construct stroke positional/named forms with paint/thickness/cap/join/dash/miter-limit/overhang; miter-limit finite positive. Preserve signature/native identity/syntax/morphology/errors/order; static hub forwarding without duplicate algorithms/public-default-phase changes. Build Value via existing From<Stroke>; receiving shared Value::Stroke produces owned Stroke to complete positional components without mutating payload. Keep parsed fields/specified/defaults/validation/error priority.

Named paint uses same private Paint cast as positional dictionary paint: Color/Gradient/Tiling retained; Auto and omission leave specified.paint=false and existing black fallback. Reject None/other types with exact measured "expected color, gradient, tiling, or auto, found <tipo vanilla>" and received span. Share helper here across both routes, no duplicate casting. Preserve thickness/cap/join/dash/miter-limit/overhang/positional-named mixing and their unresolved legacy differences.

## Acceptance and limits

Preserve existing tests/oracles, relevant isolation RED→GREEN and restricted owner reseal/revalidation for ratified compact carrier. No semantics/default/phase expansion or performance claim. Internal ADR-0127 continuous-flow parity correction: L0 first, meaningful RED→GREEN, restricted reseal/build/suite/lineage, PDF/raster/error recollection retaining uncorrected differences. Paint meaning/available form is acceptance, never binary equality. No public API/entity/default/phase expansion.


## Closed decision coverage

### Measurement before decision

At HEAD `91ea6ee91105b97b1aedf172e34780dddde14c94`, the source/owner hashes
and exact inspection state are in
`lab/results/v16-corrections-2026-10-19/batch-f/agents/encoding/read-only-audit.json`.
The baseline binary build and UTC-stamped bilateral commands/output are retained
in `lab/results/v16-corrections-2026-10-19/batch-f/agents/encoding/baseline-enabled-probes.json`;
the separate HTML textual projection receipt is `baseline-html-projection.json`
in that directory. The initial feature-disabled HTML run is setup evidence only.
The oracle is pinned to commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`,
tree `cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
These observations do not infer upstream intent or certify unrelated parity.

`stroke.rs:145–152` accepts Length only when its absolute component is
nonnegative, or the exact string dot; false guards and other Value variants
reject the entire dash array. `:154–157,196–204` rejects empty/invalid arrays.
Pinned upstream `visualize/stroke.rs:559–565,614–621` accepts Length and empty
arrays. Current probes confirm negative/empty rejection, em loss and different
wrong-type diagnostics; mixed length/dot and phase controls agree.

### Decision and acceptance

Close the named dash-item Value decision, explicitly including guard-false
Length and Str. Preserve accepted ordering, LineWidth, dictionary phase, supplied
span, error propagation and current abs/em/nonfinite conditions. A failed item
must reject the call, never be dropped or defaulted. Tests exercise native_stroke
with mixed values/phase and invalid negative/NaN/non-dot/types/empty, plus current
infinity/em behavior. These latter preservation observations do not certify parity:
negative/empty rejection, em loss and diagnostics remain unresolved obligations.
Future Value variants must produce local E0004. No change to positional stroke,
public signature, carrier, default or pipeline is authorized by this correction.


## Issue 8894 — approved Stroke migration

Measurement: the source inventory and previous owner bytes in
`lab/results/construction/diagnostics/issue-8894-2026-10-31/` bind exact
HEAD/dirty diffstat/UTC and each affected source/owner before migration.
`stdlib/layout/stroke.rs:430–434` erases em; geometry.rs:81 carries only f64.
The explicit user approval in approval.json ratifies the public Length field
and occurrence-layout resolution, as specified by
`00_nucleo/prompts/core/entities/geometry.md`. This consumer retains its
exclusive responsibility and all other ratified clauses and controls.

Migrate this consumer's declarative Stroke literal thickness from scalar
points to Length::pt without changing their value or other fields. FrameStroke
remains scalar. Keep all existing semantic assertions; update only their
representation as necessary. Producers preserve full Length; layout consumers
resolve against effective font size before scalar geometry. Repr preserves
absolute/em components. RED→GREEN, build/tests and scoped lineage are required.
