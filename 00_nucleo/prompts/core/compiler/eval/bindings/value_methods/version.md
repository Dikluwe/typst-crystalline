# L0 — compiler/eval/bindings/value_methods/version
Hash do Código: c879ec5b

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/bindings/value_methods/version.rs`. Internal extraction measured at HEAD `ce27af795` with a clean tree at 2026-09-30 23:12:44 UTC. The source is this unit's parent consumer; extraction is neither a bilateral parity measurement nor performance evidence.

The extracted logic was in `01_core/src/compiler/eval/bindings/value_methods.rs:629-653` at the indicated HEAD. This is an internal behavior extraction, not a new parity decision.

## Contract

Own Version method adapter. Preserve argument spans, evaluation and stdlib delegation. Preserve the public contract, pipeline phase, default behavior and compatibility.

## Acceptance

L1 tests, `cargo build` and `crystalline-lint .` must pass. Do not infer performance gains from structural separation.
