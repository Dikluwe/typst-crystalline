# L0 — compiler/eval/math/expression_call
Hash do Código: 80cba1a0

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/math/expression_call.rs`. Internal extraction measured at HEAD `ce27af795` with a clean tree at 2026-09-30 23:12:44 UTC. The source is this unit's parent consumer; extraction is neither a bilateral parity measurement nor performance evidence.

The extracted logic was in `01_core/src/compiler/eval/math/expression.rs:138-913` at the indicated HEAD. This is an internal behavior extraction, not a new parity decision.

## Contract

Own lexical shadowing and qualified callees in the `Expr::FuncCall` arm. Delegate ordered native math function dispatch to `00_nucleo/prompts/core/compiler/eval/math/native_dispatch.md` and unknown/global/symbol fallback to `00_nucleo/prompts/core/compiler/eval/math/call_fallback.md`. Preserve argument evaluation order, spans, content morphology and the static dispatch order defined by the parent math prompt. Preserve the public contract, pipeline phase, default behavior and compatibility.

## Acceptance

L1 tests, `cargo build` and `crystalline-lint .` must pass. Do not infer performance gains from structural separation.
