# L0 — image
Hash do Código: 26ee757d

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:1f0c020521a9654478f7ba3325005ba621fe5f26b307b56dc49e8c526fceb59f


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/image.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/image.rs:18`, `01_core/src/compiler/layout/image.rs:22`, `01_core/src/compiler/layout/image.rs:25`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Historical pins do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without a new measurement.

The equivalent-input pinned SVG probes in
`lab/results/construction/diagnostics/font-demand-geometry-study/results.json` show that an
initial standalone non-text block moves when only text top-edge changes in
Tekt, while the pinned vanilla position remains the region's physical top.
This is observable geometry, not a requirement to copy upstream algorithms.
Those earlier controls do not cover the root Place regressions measured below; legacy subframe conventions remain unchanged outside the bounded correction.

Inspection of the subframe path in the owned consumer shows that cap-height is
queried before a branch whose ordinary positioning uses cursor_y directly.
`lab/results/construction/diagnostics/font-demand-geometry-study/catalog-counts/results.json`
records remaining actual Place catalog access; it does not prove every accessor
call is a catalog scan. The no-typography descendant controls discriminate the
unused query from the shared subframe's actual origin acquisition.

The five `place_regression_*` refutators in `03_infra/src/integration_tests.rs:5690` reproduce loss of configured empty-page dimensions, image clipping, text top edge, and unwanted catalog acquisition. `01_core/src/compiler/layout/helpers.rs:185` discards clip_rect; `sub_frame.rs:143` and `placement.rs:327` acquire unconditional ascenders. `mod.rs:1978` skips the initial empty page. The raster test at `03_infra/src/export/line_cap_tests.rs:204` uses an A4 viewport at 16 pixels per point for a single isolated line. Provenance and RED logs are recorded in `lab/results/construction/diagnostics/correcao-falhas-2026-09-28/`. These observations justify internal parity and fixture corrections, not a new public contract or a general speedup claim.

`box-contexto-2026-10-14/probe.json.gz`preserves sources/binaries/HEAD/working tree/UTC: Box, inset2pt, prefix and nesting publish (0,0) despite displaced paint. The direct-context baseline difference is separate; do not copy vanilla merely because it differs. `mod.rs:1131-1148/1180-1200` writes local points to runtime.positions; Box groups its body in another coordinate system; `finish.rs:295-302` seals the map without recomposing those points. Realized writes through or_insert; labels have a parallel map. HTML already carries private markers through the frame at `html_frame.rs:30-199`. Preserve single-pass registration and per-sub-store sealing under ADR-0073/0074 position/hybrid-model sections; do not adopt vanilla global introspection. Inspection of coordinates and its owners found absolute uses in Grid and explicitly local uses in Place/Box; nucleus qualification preserves both and requires page transformation without doubled origin. Earlier snapshot: `posicoes-compostas-2026-10-14/before.json.gz`. Geometry/identity are observables; carrier/index are mechanics.

`block-auto-width-2026-10-14/before/results.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC/diffstat/status/sources/binaries/argv and historical pin 9f2b6e871. The initial extractor sought blue as #0000ff; blue-geometries-corrected.json reads actual #0074d9 from the same SVGs without rerun. For Shape, Empty, Place-only, Grid and Stack, Tekt backing width is 160 versus pin54,14,14,68,68 respectively. `block.rs:222,416,453` uses available width; `line_commit.rs:753` loses cursor on closure; `sub_frame.rs:354` saves terminal cursor only. Pinned `flow/block.rs:145–166,281–290,339–363` distinguishes Auto from expansion. Geometry is language; witness structure is internal mechanics. Place contaminating width, leaking scopes, lost literal advances or changed explicit-container dimensions refute the hypothesis.

## Ownership and contract

This prompt owns only the named consumer. State storage and general coordination remain in `00_nucleo/prompts/core/compiler/layout/mod.md`; the contract below defines this unit's operations, not ownership of that hub.

Own the pure image dimension calculator and block-layout adapter. Read intrinsic pixels, DPI and EXIF orientation through injected ImageSizer, not a decoder or filesystem. Use DPI fallback 72 and convert by 72/dpi; metadata precedence is the sizer owner's responsibility. Preserve unknown-size fallback 100pt square. Orientations five through eight swap dimensions used for aspect/layout but retain original intrinsic pixel dimensions for export.

Dimension overrides preserve the current Float/absolute-Length projection. With both axes, retain the target rectangle and fit: stretch uses it directly; cover/contain preserve aspect according to the current wide-image branch. Center the transformed image in the target, clip only cover, and use target height for cursor/fitting. One-axis overrides preserve aspect with no target clip; no overrides use converted intrinsic dimensions.

Close pending text and apply the existing main-flow block spacing/collapse protocol, preserving the source's image spacing constant of 1.2em, matching the canonical default in `00_nucleo/prompts/core/compiler/layout/vanilla_defaults.md`. Keep the distinct post-text baseline anchor versus first/post-block cap-height anchor; subframes bypass this main-flow protocol. Preserve ordinary pre/post overflow requests and emit original bytes, intrinsic dimensions, orientation and alt without synthesis, keeping None distinct from Some(empty). Geometry follows the shared coordinate convention, not the old prose's upward-Y assertion. No renderer or PDF matrix policy is owned here.

When main-flow layout still has its initial baseline pending and no pending line,
place the first block at the physical region origin without acquiring typography
merely to add and subtract an artificial baseline. Mark that initial state handled;
retain the existing post-block cursor/margin protocol so later text can request its
own metrics. Explicit text top-edge must not move this block. Preserve pending-text
and subframe paths, dimensions, clipping/paint and existing overflow behavior except
that first-block fitting uses its actual geometric extent. Tests in this consumer
must discriminate explicit top edges and prove this path needs no FontMetrics query.
This internal parity repair does not change public API or pipeline phases.

Ordinary subframe block positioning must not query cap-height when the result is
unused. Preserve the current subframe origin and cursor/extent protocol. For a
shape whose actual extent triggers the existing new-page branch, resolve the
needed cap-height before transitioning pages, preserving its original style and
fallback coordinates. Image subframe positioning/extent does not use cap-height.
Keep the main-flow typography-dependent paths unchanged. This removes unnecessary
queries, not the subframe owner's initial catalog demand; no speedup is asserted.

A first image with an initial baseline pending and an empty line consumes a geometric origin in the dedicated root Place sublayout as well as in main flow. It must not request typography for that origin. Existing baseline-origin sublayouts still clear the pending flag before dispatch and retain their positions.

For fresh-block admission, a pending line with only direct position carriers is equivalent to empty. Preserve meaning of every other item and all dimension/fit/pagination rules.

Publish causal width of the layout target, or natural dimension without a target, at logical position. Cover overflow does not enlarge causal width.

## Acceptance and limits

Preserve the unit's current rendered content, morphology, causal positions/pages, styles and diagnostics, including the explicitly documented limits. Follow the existing static descendant interface without altering public signatures, defaults, compatibility or pipeline phase. This documentation does not authorize implementation of unresolved proposals.

Composed-position correction follows ADR-0127 with no public API/field/flag/default/phase change. L0 precedes RED→GREEN for coordinates, no fictitious paint/height, identity/order/visual clones/rollback; revalidate HTML, geometry, context/queries and complete suite.

This internal parity correction under ADR-0127 adds no public contract/default/phase. L0 precedes RED→GREEN/restricted resealing. Causal width is only part of the ongoing block-region/width/transport correction: Auto height/backing, late alignment, fractions, fragmentation, outset and PDF Parent retain their own criteria. Testing a carrier alone does not approve render. Preserve full bilateral controls/refuters, suite/build/doc/fmt/lineage/inventories.


## Batch D V16 measurement (2026-10-19)

Evidence precedes the decisions below. The exact HEAD, uncommitted file list,
UTC, source bytes, argv, source/output/binary SHA-256 and pinned oracle recipe
are recorded in `lab/results/v16-corrections-2026-10-19/batch-d/agents/measurement/probes/png-before.json`.
The oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`.
SVG geometry is evidence of occupied layout space, not a byte-equality contract.
These observations establish behavior; they do not infer upstream intent.
`image.rs:139-144` accepts Float and the absolute component of Length;
`compiler/stdlib/figure_image.rs:431-432` forwards arbitrary axis Values without
typed validation. Thus unsupported values are reachable, not impossible. The
additional `axis-absolute-before.json` preserves source and diagnostics: boolean
and Float axes compile in product but are rejected by the oracle. Relative/em
axes have separate geometry debt; the current public calculator lacks region
and font-size inputs. This audit does not turn that debt into accepted language.
`calculate_dimensions` remains the owner of DPI, EXIF, intrinsic fallback and fit.

### Decision and refuters

At extract_pt, explicitly classify every current Value variant: Float and Length
retain existing projection; all other types retain None at this private bounded
projection. Auto means no explicit axis; other unsupported values are not a
proof of valid source syntax. Do not broaden this helper, silently coerce types,
or change a public signature to discharge V16. Axis source validation and
font/region-relative resolution remain open debts; a caller resolving them into
supported values would refute the claimed loss. Preserve target-vs-image bounds,
clip behavior and fit/aspect/DPI/orientation. Future Value variants must force a
new decision at this exact match. ADR-0127 continuous internal flow applies.
