# L0 — compiler/math/layout/line_decorations
Hash do Código: e5ed28df

## Evidence before decision

At HEAD ea7ae25ab (clean tree, 2026-09-30), the implementation was in `01_core/src/compiler/math/layout/mod.rs:1312-1401`. This measurement identifies the extraction boundary; it neither measures performance nor changes language parity.

## Owner and contract

Layer L1. Exclusive productive consumer: `01_core/src/compiler/math/layout/line_decorations.rs`. Lay out mathematical overline and underline using the existing font constants, baseline-relative coordinates and styles. Extraction preserves the public contract, defaults, pipeline phase, compatibility and L1 purity.

## Acceptance

Dispatch remains static. L1 tests, build and crystalline-lint must pass. Do not infer performance gains from separation.
