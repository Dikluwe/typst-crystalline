# L0 — shape block behaviour
Hash do Código: e3f1d69e

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/shape.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/shape.rs:25`, `01_core/src/compiler/layout/shape.rs:35`.

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

Own Shape rendering in `shape.rs`. Resolve the current Rect/RoundedRect/Ellipse/Path dimensions or Line dx/dy, establish the initial typography reference, close pending text and retain the main-flow block-spacing/collapse protocol. Shape position follows the source's distinct first/post-block cap-height versus post-text baseline-plus-above branches; subframes use direct positioning without main-flow above/below. Request ordinary progress only from the candidate shape's actual bottom, so a fitting shape is not rejected by unrelated typographic leading. Resolve radius against effective size, preserve Paint, stroke, fill rule and parent bbox. Delegate supported content-backed tiling fills to `00_nucleo/prompts/compiler/layout/tiling.md`; when materialized, preserve the separate unfilled stroke shape and paint order; otherwise retain the declarative fill. Advance using the current shape extent/below protocol and leave parent state coherent. Do not rasterize, sample all paints into a single solid color, shift Place through a global scan or move this render logic into ShapeElem.

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

A first shape with an initial baseline pending and an empty line consumes a geometric origin in the dedicated root Place sublayout as well as in main flow. It must not request typography for that origin. Existing baseline-origin sublayouts still clear the pending flag before dispatch and retain their positions.
