# L0 — stdlib/collections/bytes — Bytes methods
Hash do Código: 78e68100

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/call-once.toml sha256:3c1f911b856e019f09d75d07fd14052912e8f0fb6118fceaaac9a7f4a92bfd97

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/collections/bytes.rs`

## Evidence before specification

L1 exclusive consumer `01_core/src/compiler/stdlib/collections/bytes.rs`. `lab/results/construction/diagnostics/atomizacao-stdlib-collections-2026-09-30-before.json.gz` captures uncommitted collections.rs before behavior extraction; L1 suite 6401 pass/2 ignored is structural baseline, not bilateral parity/benchmark. Parent `00_nucleo/prompts/core/compiler/stdlib/collections.md` preserves historical criteria. Historical probes against `9f2b6e8715237cb086899a42873660fe744622e8` retain their original snapshot limits and do not certify current `bd5ebec285505889e8ff1d4f68804ab147a699f3` without remeasurement. Language semantics, syntax, morphology and observable diagnostics govern acceptance; enum/match/algorithm structure is mechanics. Source inspection is not inferred upstream intent.

`lab/results/construction/diagnostics/call-once-2026-10-17/before.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, dirty tree/UTC/status/diff HEAD --stat/sources/L0/config/hashes;2124 predecessor hashes were checked exactly before baseline. before-probe.json.gz/before-math-probe.json.gz retain bilateral binaries/argv/fixtures/outputs (historical oracle 9f2b6e871 binary `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`). Receiver/args repeated, missing methods evaluate panic args before causal error, math.mat duplicates accumulation and empty-array truncate hid it. Local call/collection/context/content/selector/method/invoke paths violate existing call_dispatch once-only contract; pinned-source.json.gz fixes upstream call/methods evidence. Invalid initial reserved-word `or` fixture is retained separately, no call-effect credit; its import panic 101 remains parsing/import debt.

## Exclusive contract

len counts octets; at accepts negative index/default only out of bounds; slice preserves limits/count, rejects invalid casts and retains CollectionCallSpans diagnostic anchors. Parent owns bound-method/type exposure and calls primitives without algorithm duplication, preserving identity/syntax/morphology/errors/spans/validation precedence. Keep try_dispatch_bytes_method cfg(test)-only for existing controls; productive len/at/slice catalog stays parent-owned. bytes_len/bytes_at/bytes_slice bodies unchanged, no duplicated validation.

## Acceptance and limits

Preserve L1 suite/lint/1:1 ownership and all language controls. Consume pinned call-once invariants. Internal ADR-0127 continuous-flow correction supersedes only conflicting internal-signature/name-compensation restrictions, retaining Tekt extensions, public functions/reexports/structs/traits/fields/default/phase. Functional co-located RED→GREEN for observed receiver/args effects, mutating fallback/error-before-args, where/With/selector/collections/matrix without truncating legitimate effects; hits/misses, spans/occurrences and Value complements. Unhandled preserves receiver and does not execute args callback; count effects, not PDF bytes/Rust Eq/benchmark. Preserve residual diagnostics explicitly. Ownership/V26 before restricted resealing; build/workspace/strict all-target Clippy including unfulfilled expectations/Rustdoc/fmt/strict-reverse V1/V5/V15/V26/global differential SARIF, no new error/config/suppression. Informative V19 retains alternatives matrix and behavioral relation. Sequential 12GiB/High 10GiB/swap0/jobs1. No publish/commit/merge/release/global-completion claim.
