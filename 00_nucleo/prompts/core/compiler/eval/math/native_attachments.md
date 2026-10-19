# L0 — compiler/eval/math/native_attachments
Hash do Código: 21bdfd54

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/math/native_attachments.rs`. The original behavior was in `01_core/src/compiler/eval/math/native_dispatch.rs:183-299` at HEAD `6fc546566`, clean tree, 2026-10-01 01:04:22 UTC. This is internal extraction, not a new parity or performance measurement.

## Contract

Dispatch direct mathematical scripts, limits and attach calls. Preserve named-argument validation order, Boolean syntax, the explicit inline bit, attachment morphology, spans and constructor delegation. Preserve the public contract, pipeline phase, defaults, compatibility and L1 purity.

## Acceptance

The L1 suite, `cargo build` and `crystalline-lint .` must pass. Do not infer performance gains.
