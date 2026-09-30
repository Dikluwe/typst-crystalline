# L0 — image
Hash do Código: 6fd638b4

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/image.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/image.rs:18`, `01_core/src/compiler/layout/image.rs:22`, `01_core/src/compiler/layout/image.rs:25`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `00_nucleo/diagnosticos/sanitization-layout-history.md`. Older vanilla pins must not be presented as proof against the current pinned reference `9f2b6e8715237cb086899a42873660fe744622e8`.

The equivalent-input pinned SVG probes in
`00_nucleo/diagnosticos/font-demand-geometry-study/results.json` show that an
initial standalone non-text block moves when only text top-edge changes in
Tekt, while the pinned vanilla position remains the region's physical top.
This is observable geometry, not a requirement to copy upstream algorithms.
Those earlier controls do not cover the root Place regressions measured below; legacy subframe conventions remain unchanged outside the bounded correction.

Inspection of the subframe path in the owned consumer shows that cap-height is
queried before a branch whose ordinary positioning uses cursor_y directly.
`00_nucleo/diagnosticos/font-demand-geometry-study/catalog-counts/results.json`
records remaining actual Place catalog access; it does not prove every accessor
call is a catalog scan. The no-typography descendant controls discriminate the
unused query from the shared subframe's actual origin acquisition.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/compiler/layout.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the pure image dimension calculator and block-layout adapter. Read intrinsic pixels, DPI and EXIF orientation through injected ImageSizer, not a decoder or filesystem. Use DPI fallback 72 and convert by 72/dpi; metadata precedence is the sizer owner's responsibility. Preserve unknown-size fallback 100pt square. Orientations five through eight swap dimensions used for aspect/layout but retain original intrinsic pixel dimensions for export.

Dimension overrides preserve the current Float/absolute-Length projection. With both axes, retain the target rectangle and fit: stretch uses it directly; cover/contain preserve aspect according to the current wide-image branch. Center the transformed image in the target, clip only cover, and use target height for cursor/fitting. One-axis overrides preserve aspect with no target clip; no overrides use converted intrinsic dimensions.

Close pending text and apply the existing main-flow block spacing/collapse protocol, preserving the source's image spacing constant of 1.2em, matching the canonical default in `00_nucleo/prompts/compiler/layout/vanilla_defaults.md`. Keep the distinct post-text baseline anchor versus first/post-block cap-height anchor; subframes bypass this main-flow protocol. Preserve ordinary pre/post overflow requests and emit original bytes, intrinsic dimensions, orientation and alt without synthesis, keeping None distinct from Some(empty). Geometry follows the shared coordinate convention, not the old prose's upward-Y assertion. No renderer or PDF matrix policy is owned here.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

## Initial non-text block correction

When main-flow layout still has its initial baseline pending and no pending line,
place the first block at the physical region origin without acquiring typography
merely to add and subtract an artificial baseline. Mark that initial state handled;
retain the existing post-block cursor/margin protocol so later text can request its
own metrics. Explicit text top-edge must not move this block. Preserve pending-text
and subframe paths, dimensions, clipping/paint and existing overflow behavior except
that first-block fitting uses its actual geometric extent. Tests in this consumer
must discriminate explicit top edges and prove this path needs no FontMetrics query.
This internal parity repair does not change public API or pipeline phases.

## Subframe metric demand

Ordinary subframe block positioning must not query cap-height when the result is
unused. Preserve the current subframe origin and cursor/extent protocol. For a
shape whose actual extent triggers the existing new-page branch, resolve the
needed cap-height before transitioning pages, preserving its original style and
fallback coordinates. Image subframe positioning/extent does not use cap-height.
Keep the main-flow typography-dependent paths unchanged. This removes unnecessary
queries, not the subframe owner's initial catalog demand; no speedup is asserted.

## Correction — 2026-09-28

### Measurement before decision

The five `place_regression_*` refutators in `03_infra/src/integration_tests.rs:5690` reproduce loss of configured empty-page dimensions, image clipping, text top edge, and unwanted catalog acquisition. `01_core/src/compiler/layout/helpers.rs:185` discards clip_rect; `sub_frame.rs:143` and `placement.rs:327` acquire unconditional ascenders. `mod.rs:1978` skips the initial empty page. The raster test at `03_infra/src/export/p1431_line_cap_tests.rs:204` uses an A4 viewport at 16 pixels per point for a single isolated line. Provenance and RED logs are recorded in `00_nucleo/diagnosticos/correcao-falhas-2026-09-28/`. These observations justify internal parity and fixture corrections, not a new public contract or a general speedup claim.

### Owner responsibility

A first image with an initial baseline pending and an empty line consumes a geometric origin in the dedicated root Place sublayout as well as in main flow. It must not request typography for that origin. Existing baseline-origin sublayouts still clear the pending flag before dispatch and retain their positions.
