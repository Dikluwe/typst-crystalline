# L0 — shape block behaviour
Hash do Código: 4c5e184c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:1f0c020521a9654478f7ba3325005ba621fe5f26b307b56dc49e8c526fceb59f
- 00_nucleo/prompts/_nuclei/layout/subframe-overflow.toml sha256:06c0f9a3e9b81ea332d57174938fda46c0865535787945094cea27ee5afbd562
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:2e0595153eaf379004fd35fe6a3887a7d62532bec78700833e8b8569e672137e
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/shape.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Evidence before specification

The current consumer implements the operations specified below at `01_core/src/compiler/layout/shape.rs:25`, `01_core/src/compiler/layout/shape.rs:35`.

Source inspection establishes current behavior and mechanical ownership; it is not a new bilateral parity measurement or evidence of upstream intent. Observable text, morphology, geometry and errors remain language-level obligations; buffer layout and traversal structure are implementation mechanics. Historical probes and their snapshot-specific qualifications are recorded in `lab/results/construction/diagnostics/sanitization-layout-history.md`. Historical pins do not certify the current ratified reference `bd5ebec285505889e8ff1d4f68804ab147a699f3` without new measurement.

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

containers-clip-2026-10-13/before-renders.json records oversized declared-height Block Shape producing two blank Tekt pages versus one clipped region; full source/provenance there. shape.rs:119/cursor.rs:125/line_commit.rs:438/region_progress.rs:115,131 allow external overflow transition while is_sub_frame; sub_frame.rs:109-121,317-340 saves local buffers/cursor but cannot undo descendant committed page. `lab/results/construction/diagnostics/subframe-isolamento-2026-10-13/before.json.gz` preserves source/L0/HEAD/dirty tree/status/diffstat/UTC. Test hypothesis directly before condition changes, not by raising test height/dropping frame items.

`box-contexto-2026-10-14/probe.json.gz`preserves sources/binaries/HEAD/working tree/UTC: Box, inset2pt, prefix and nesting publish (0,0) despite displaced paint. The direct-context baseline difference is separate; do not copy vanilla merely because it differs. `mod.rs:1131-1148/1180-1200` writes local points to runtime.positions; Box groups its body in another coordinate system; `finish.rs:295-302` seals the map without recomposing those points. Realized writes through or_insert; labels have a parallel map. HTML already carries private markers through the frame at `html_frame.rs:30-199`. Preserve single-pass registration and per-sub-store sealing under ADR-0073/0074 position/hybrid-model sections; do not adopt vanilla global introspection. Inspection of coordinates and its owners found absolute uses in Grid and explicitly local uses in Place/Box; nucleus qualification preserves both and requires page transformation without doubled origin. Earlier snapshot: `posicoes-compostas-2026-10-14/before.json.gz`. Geometry/identity are observables; carrier/index are mechanics.

`lab/results/construction/diagnostics/place-fluxo-2026-10-14/before/results.json.gz`records the same HEAD, uncommitted tree/diffstat/UTC/sources/binaries/argv and historical pin. All 24 root/Box pairs after 12 prefixes differ: Linebreak/Parbreak require y22.25pt, Shape y27pt, Block/Grid/VSpace20pt y35pt, Tekt 15pt. Rotate also overlaps independently. `line_commit.rs:721–733` prepares next baseline; `shape.rs:196–206` adds below/cap; `block.rs:465` separates boundary/margin; `place.rs:91–99` recognizes still-pending line only. Pinned flow/distribute.rs:703–715 uses physical flow offset for axis-unspecified Place. Weak margins/overlap/new page/rollback/subframe leakage can refute the causal-end hypothesis.

`block-auto-width-2026-10-14/before/results.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, UTC/diffstat/status/sources/binaries/argv and historical pin 9f2b6e871. The initial extractor sought blue as #0000ff; blue-geometries-corrected.json reads actual #0074d9 from the same SVGs without rerun. For Shape, Empty, Place-only, Grid and Stack, Tekt backing width is 160 versus pin54,14,14,68,68 respectively. `block.rs:222,416,453` uses available width; `line_commit.rs:753` loses cursor on closure; `sub_frame.rs:354` saves terminal cursor only. Pinned `flow/block.rs:145–166,281–290,339–363` distinguishes Auto from expansion. Geometry is language; witness structure is internal mechanics. Place contaminating width, leaking scopes, lost literal advances or changed explicit-container dimensions refute the hypothesis.

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz` records HEAD, working tree, diffstat/UTC and sources/owners before migration. `layout_types.rs:510–511` still transports Paint/Stroke; `shape.rs:151–191` materializes only fill in groups; `tiling.rs:85` limits/clones the grid. `prepare_cell` already returns TilingFrame once per occurrence. Probes `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz` establish that dense-open and stroke-open still diverge in all three targets. The contract was approved in manifest `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`; receipt: `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`. Preparation/object counts are transport mechanics; render, contours, text, geometry and diagnostics are observable language obligations.

`lab/results/construction/diagnostics/transformacoes-2026-10-14/shape-dim-before-complete/results.json.gz` preserves HEAD/dirty tree/diffstat/UTC/sources/binaries. Root 50%/25% in 160 region measures 80×40 pin; Block80/inset5 requires 35 width and 75 auto-height or 25 with height 60; Box80×60 gives 40×30, auto-height Box 40×80. Unbounded 50%+10pt axes must give 10×10, not invalid candidate SVG. helpers.rs:427-438 resolves absolute only; shape.rs:150-151 uses it universally; sub_frame.rs:130-141 inherits auto sentinel instead of real region. Pinned flow/block.rs:255-290,layout/ratio.rs:108-110,rel.rs:112-113 support relative geometry, not bytes/Rust layout. First GREEN shape-dim-green.log still gives 37.5 versus 35 under Block inset5 because root available_width subtracts physical margins rather than local line_start_x; active block_place_region inner width must dominate.

`lab/results/construction/diagnostics/transformacoes-2026-10-14/shape-terminal-complete/results.json.gz` preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, dirty tree/status/diffstat/UTC/source/binaries/historical pin 9f2b6e871. Shape-only height 43.2 versus 30, second Shape y25.962 versus 33.2, following Text baseline20.438 versus 40.438, Shape→Rotate top13.2 versus 33.2 and Rotate→Shape height 30 versus 53.2. shape.rs:196-266 subtracts cap-height from geometric top,`:289-304` omits confirmed baseline/bottom consumed by cursor.rs:90-110/transform.rs:352-356. Pinned flow/collect.rs:253-285/distribute.rs:302-318 distinguish frame from terminal weak margin. Geometry, not Rust/buffer strategy, is language obligation. shape-flow-before/results.json.gz also measures lost strong VSpace 7pt/30pt after Shape/Rotate; weak-between-item collapse remains a separate gap.

## Exclusive ownership and current contract

Own only `01_core/src/compiler/layout/shape.rs`, shape rendering rather than ShapeElem data/entity methods. Resolve Rect/RoundedRect/Ellipse/Path dimensions or Line dx/dy in existing adapter, radius against active effective size, preserving kind/fill_rule/parent_bbox_at_emit, cursor/bbox state and ordinary block-spacing/collapse boundaries. No rasterization/paint-to-solid sampling/global Place scan/entity render coupling.

A first Shape with pending initial baseline and no material line uses physical geometric region origin in root main flow and dedicated root Place sublayout, without querying typography to add/subtract fictitious baseline. Mark initial state handled; preserve later postblock cursor/margin so subsequent text requests its own metrics. Direct-position-carrier-only line is empty for this fresh-block condition; other items including separators retain meaning. A neutral carrier never causes font ascent demand. Legacy baseline-origin sublayouts keep their established cleared-pending route/positions.

Current main-flow vertical refinement supersedes earlier first/postblock cap-height and posttext formulas only for physical Shape placement. Close pending text through existing hook and place at preceding physical end plus collapsed gap, without subtracting cap-height from geometric top. First Shape retains physical origin; page transition uses new region's geometric top. On completion publish Shape bottom to prev_line_baseline/last_block_descent_y, clear equation descent/parbreak marker, keep pending below separate, and register shape_base+resolved_h after cursor update excluding preparatory cap-height/below. Do not query fonts for a shape-only sequence. Preserve local subframe route/dimensions/paints/transport and following strong VSpace contribution to height.

Ordinary fitting uses actual candidate bottom, not unrelated typographic leading. Before requesting overflow region distinguish is_sub_frame: oversized local Shape keeps current position/size and never performs external-transition cap-height compensation. Main-flow ordinary progress remains; no frame discard/test-height workaround. Removing unused subframe cap-height queries does not itself prove every font accessor was a catalogue scan or a speedup.

Dimension subset: Length resolves effective font size, Ratio against regional axis, Relative sums both; nonfinite relative product contributes 0 retaining absolute portion. Width first uses active block_place_region inner width, otherwise existing local available width. Height uses explicit inner Block height, else saved real relative region, else root available height; never subframe sentinel. Other Values/None/Auto/Line dx-dy keep legacy helper. Automatic identities/dimensions, circle/square and casts remain separately incomplete under shape-dimensoes-automaticas; no public API change or correct-control claim is implied. No extra speculative execution.

Prepare fill/stroke through `00_nucleo/prompts/core/compiler/layout/tiling.md` prepare_paint/prepare_stroke before emission, recording layout errors. Emit one Shape with both prepared paints, no repetition Group. This approved transport supersedes former repeated content-fill Group/separate-stroke fallback only within approved scope; preserve declarative ShapeElem, pos/kind/dimensions/fill_rule/parent bbox/block protocol/flow witnesses and paint order/semantics. Publish resolved logical horizontal dimension independently of paint/stroke/tiling; stroke/outset is never flow advance.

## Acceptance and incomplete scope

Preserve language content/morphology/causal positions-pages/styles/diagnostics and static interfaces, no unapproved public signature/default/compatibility/phase. Internal ADR-0127 corrections require L0 before RED→GREEN/restricted reseal and full corpus/suites/validators.

Initial nontext controls discriminate explicit text top-edge and prove no FontMetrics demand for geometric origin; preserve pending-text/subframe/clip/paint/dimension/ordinary overflow controls with actual-extent fitting. Oversized local tests first fail then pass with no external pages, unchanged external pagination/core/build/fmt/lineage; container collection/dimensions/clipping remain their owners' incomplete scope.

Relative-size controls keep origin/paint/stroke/resources/effects and no extra evaluation; None/Auto/circle/square/casts remain explicit debt. Final vertical refinement cannot discard strong VSpace; Shape defaults/RTL alignment/Block Auto/general weak-VSpace collapse retain own pending work and cannot be closed here.

Flow-end private state authorization is limited to measured causal contract with LIFO save/restore. Certified line/Shape/Block/Align/Grid/Raw/VSpace does not complete Block/inset/fragmentation/late inline Box-Transform or other Heading/List/Stack/Equation/Columns producers. Invalid witness remains legacy fallback/pending, not parity credit. Revalidate full suite/HTML-context/PDF-SVG/build-doc-fmt-check/lineage/warnings.

This internal parity correction under ADR-0127 adds no public contract/default/phase. L0 precedes RED→GREEN/restricted resealing. Causal width is only part of the ongoing block-region/width/transport correction: Auto height/backing, late alignment, fractions, fragmentation, outset and PDF Parent retain their own criteria. Testing a carrier alone does not approve render. Preserve full bilateral controls/refuters, suite/build/doc/fmt/lineage/inventories.

Prepared fill/stroke tests use counted Image body once per occurrence, no grid and target-area-independent preparation cost. Preserve true render controls rather than carrier-only proof.

Apply the pinned nucleus.Ratification supersedes older clauses awaiting approval or requiring declarative Paint/Stroke inside Shape only within the approved contract. Preserve all other existing behavior. L0 comes first, followed by RED→GREEN; explicitly migrate fixtures/consumers without a total From<Paint> conversion, PartialEq on carriers or a solid fallback merely to obtain a green build. Do not raise the old limit or move layout into L3. Validate fill/stroke transport, resources, clipping, affine Parent, native targets, Solid/Gradient controls, page semantics and the complete suite/build/doc/fmt/lineage. Prepared transport, native target integration and verification (historical T1c/T2/T3) are complete only together; a broken or partial intermediate state does not certify production. The remaining historical T4 scope and publication remain outside this package.


## Ratified Circle constructor origin

### Evidence and approval before implementation

At HEAD `954d13185e26a1ed90f6d7c60f6ba27760ca9889`, the source/owner audit in
`lab/results/v16-corrections-2026-10-19/circle-origin/l0-before.json` freezes
UTC, exact diffstat and hashes. The approved proposal
`lab/results/v16-corrections-2026-10-19/batch-f/agents/geometry/circle-deferred/proposal/review.md`
(SHA-256 `5b7136bc18f419004113d8d83c9c54adb816e3b70389039aeeed35e1c6f772e8`)
binds the baseline equality and rejected full-Length candidate receipts.
`stdlib/shapes/ellipse.rs:92-104` erased the em component;
`entities/elements/shape.rs:21-29` used one geometry carrier for both constructors.
Those observations concern language identity and geometry, not mandatory upstream
Rust structure. The explicit user approval is recorded in
`lab/results/v16-corrections-2026-10-19/circle-origin/approval-receipt.json`;
it satisfies ADR-0127 section 2 for exactly this public type/field and migration.

### Exclusive approved responsibility

Preserve existing full-Length dimension resolution against each occurrence's
effective font and existing ShapeKind dispatch/flow/paint behavior. Origin is
semantic data, never a new render dispatch or geometry parameter. Migrate generic
ShapeElem fixtures to Geometry. The rejected candidate recorded actual 2em
geometry at two font sizes; repeat logical/paint bounds for explicit2pt/2em/mixed
radii, reused aliases and Block/Box nesting through the actual layout path.
Only explicit Circle radius conversion/constructor identity moves out of prior
blanket Circle debt. Auto/default and unrelated Circle surfaces remain open.
