# L0 — compiler/eval/math/call_fallback
Hash do Código: a489b966

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/math/call_fallback.rs`. The original logic was in `01_core/src/compiler/eval/math/expression_call.rs:650-766` at HEAD `4e2a321ce`, clean tree, 2026-09-30 23:49:20 UTC. This is internal extraction; inspection is neither new parity evidence nor a performance measurement.

## Contract

Own unknown/global/symbol mathematical call fallback. Preserve lexical/global function lookup, symbol deprecation warning, delimited argument morphology and error priority. Preserve public contracts, pipeline phase, defaults, compatibility, static dispatch and L1 purity.

## Acceptance

L1 tests, `cargo build` and `crystalline-lint .` must pass. Do not infer performance gains.
