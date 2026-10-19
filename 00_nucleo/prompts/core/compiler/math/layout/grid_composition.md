# L0 — compiler/math/layout/grid_composition
Hash do Código: fc4a512b

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/math/layout/grid_composition.rs`. Original behavior in `01_core/src/compiler/math/layout/mod.rs:1835-2153` at HEAD `51a027f99`, clean tree, 2026-10-01 00:44:23 UTC. This is internal extraction, not new bilateral parity or performance evidence.

## Contract

Own two-pass math-grid measurement and placement, row and column gaps, alignment-boundary spacing, baseline sequence, synthetic-parenthesis floor and item order. Preserve matrix/cases/multiline callers and static dispatch. Preserve the public contract, pipeline phase, defaults, compatibility and L1 purity.

## Acceptance

The L1 suite, `cargo build` and `crystalline-lint .` must pass; extraction does not imply performance gains.
