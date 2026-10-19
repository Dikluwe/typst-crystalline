# L0 — compiler/eval/math/native_dispatch
Hash do Código: fd92036c

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/math/native_dispatch.rs`. The original logic was in `01_core/src/compiler/eval/math/expression_call.rs:181-766` at HEAD `4e2a321ce`, clean tree, 2026-09-30 23:49:20 UTC. This is internal extraction; inspection is neither new parity evidence nor a performance measurement.

## Contract

Own exhaustive static dispatch for recognized bare math function names. Delegate scripts/limits/attach syntax to `00_nucleo/prompts/core/compiler/eval/math/native_attachments.md`. Delegate over/under decoration and vector constructors to `00_nucleo/prompts/core/compiler/eval/math/native_decorations.md`. Preserve constructor validation, argument order, span and morphology. Delegate unrecognized/global/symbol behavior to call_fallback without changing precedence. Preserve public contracts, pipeline phase, defaults, compatibility, static dispatch and L1 purity.

## Acceptance

L1 tests, `cargo build` and `crystalline-lint .` must pass. Do not infer performance gains.
