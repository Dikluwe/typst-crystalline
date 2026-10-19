# L0 — compiler/eval/bindings/value_methods/color
Hash do Código: 5788126d

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/bindings/value_methods/color.rs`. Internal extraction measured at HEAD `ce27af795` with a clean tree at 2026-09-30 23:12:44 UTC. The source is this unit's parent consumer; extraction is neither a bilateral parity measurement nor performance evidence.

The extracted logic was in `01_core/src/compiler/eval/bindings/value_methods.rs:178-254` at the indicated HEAD. This is an internal behavior extraction, not a new parity decision.

## Contract

Own Color instance method adapter. Preserve receiver pre00000000, argument evaluation and delegation to the stdlib color owner. Preserve the public contract, pipeline phase, default behavior and compatibility.

## Acceptance

L1 tests, `cargo build` and `crystalline-lint .` must pass. Do not infer performance gains from structural separation.

## Pending clarification

The archived contract contains the corrupt phrase `receiver pre00000000`. Its intended receiver constraint is unresolved; preserve the existing receiver behavior and do not infer a new contract from that phrase.
