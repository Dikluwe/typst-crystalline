# L0 — compiler/eval/bindings/value_methods/counter
Hash do Código: 667fd6c4

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/bindings/value_methods/counter.rs`. Internal extraction measured at HEAD `ce27af795` with a clean tree at 2026-09-30 23:12:44 UTC. The source is this unit's parent consumer; extraction is neither a bilateral parity measurement nor performance evidence.

The extracted logic was in `01_core/src/compiler/eval/bindings/value_methods.rs:256-627` at the indicated HEAD. This is an internal behavior extraction, not a new parity decision.

## Contract

Own Counter instance and static method adapters, display argument validation and label syntax preservation. Preserve argument order, spans and counter observations. Preserve the public contract, pipeline phase, default behavior and compatibility.

## Acceptance

L1 tests, `cargo build` and `crystalline-lint .` must pass. Do not infer performance gains from structural separation.
