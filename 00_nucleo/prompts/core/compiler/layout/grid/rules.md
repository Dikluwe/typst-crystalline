# L0 — compiler/layout/grid/rules
Hash do Código: f2c94edf

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

## Evidence before specification

Original logic was at `01_core/src/compiler/layout/grid.rs:1175-1250`, HEAD `4e2a321ce`, clean tree,2026-09-30 23:49:20 UTC. This is internal extraction, not new parity or performance evidence.

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz` records HEAD, working tree, diffstat/UTC and sources/owners before migration. `layout_types.rs:510–511` still transports Paint/Stroke; `shape.rs:151–191` materializes only fill in groups; `tiling.rs:85` limits/clones the grid. `prepare_cell` already returns TilingFrame once per occurrence. Probes `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz` establish that dense-open and stroke-open still diverge in all three targets. The contract was approved in manifest `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Preparation/object counts are transport mechanics; render, contours, text, geometry and diagnostics are observable language obligations.

## Ownership and contract

Layer L1. Sole productive consumer: `01_core/src/compiler/layout/grid/rules.rs`. Emit explicit horizontal/vertical grid/table rules over rendered cells using actual emitted row starts and resolved tracks. Preserve stroke-none suppression, clipping, positions, dimensions, ordering and pagination geometry. The private helper receives Layouter and prepares each explicit-rule stroke on emission, preserving None suppression, geometry/order. H/V traits remain declarative. Preserve public signatures, defaults, pipeline phase, compatibility, static dispatch and L1 purity.

## Acceptance and limits

L1 tests, build and crystalline-lint must pass. Structural separation proves no performance gain.

Apply the pinned nucleus. Ratification supersedes older clauses awaiting approval or requiring declarative Paint/Stroke inside Shape only within the approved contract. Preserve all other existing behavior. L0 comes first, followed by RED→GREEN; explicitly migrate fixtures/consumers without a total From<Paint> conversion, PartialEq on carriers or a solid fallback merely to obtain a green build. Do not raise the old limit or move layout into L3. Validate fill/stroke transport, resources, clipping, affine Parent, native targets, Solid/Gradient controls, page semantics and the complete suite/build/doc/fmt/lineage. Prepared transport, native target integration and verification (historical T1c/T2/T3) are complete only together; a broken or partial intermediate state does not certify production. The remaining historical T4 scope and publication remain outside this package.
