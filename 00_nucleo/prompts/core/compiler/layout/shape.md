# L0 — shape block behaviour
Hash do Código: 248e09fa


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/layout/shape.rs`
**Architecture:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/flow-frame-width.toml sha256:0684f5fcab7844d2ac81dd302d9afc7c12595cf5651a9f4c7c91daea8166b3f7
- 00_nucleo/prompts/_nuclei/layout/position-frame-carriers.toml sha256:1f0c020521a9654478f7ba3325005ba621fe5f26b307b56dc49e8c526fceb59f
- 00_nucleo/prompts/_nuclei/layout/rectangle-decoration.toml sha256:dbc5dd8b047bdebcd6adf912663a2d5e50b50ad46d4af6c6cf7beff23097ded8
- 00_nucleo/prompts/_nuclei/layout/relative-line.toml sha256:4d97941169e8f16d10287bb1cb938752181434dab2c76b72cf90f355073c59b9
- 00_nucleo/prompts/_nuclei/layout/subframe-overflow.toml sha256:06c0f9a3e9b81ea332d57174938fda46c0865535787945094cea27ee5afbd562
- 00_nucleo/prompts/_nuclei/place-flow-boundary.toml sha256:2e0595153eaf379004fd35fe6a3887a7d62532bec78700833e8b8569e672137e
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

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


## Private first-flow baseline witness

### Measurement before correction

`lab/results/nested-fractions-repeat-2026-10-21/prototype-1/zero-source-after/measurement.json.gz`
binds actual source, candidate/oracle SHA-256, fonts, HEAD, uncommitted diffstat
and UTC. Zero-copy Repeat loses its prepared baseline, moving adjacent BEFORE/TAIL
while its body height remains real. Source inspection at pinned
`lab/typst-original/crates/typst-layout/src/repeat.rs:19–29`,
`inline/box.rs:32–47,79–89`, and `flow/distribute.rs:689–699` shows that
a prepared frame retains baseline even without copies, and the FIRST actual
in-flow frame decides baseline presence. The local Box paint scan cannot see it.
Exact preamendment owners/sources and state are archived in
`lab/results/nested-fractions-repeat-2026-10-21/baseline-witness-before.json`.
These are language geometry obligations; private carrier representation is mechanics.

### Exclusive private obligation

At actual Shape frame emission publish authoritative no-baseline as a first-flow decision. Do not infer a font baseline for a shape or let a later Text override its absence. Existing Shape geometry, purity, origin and extent remain unchanged.

Require genuine RED→GREEN for empty Repeat baseline, positive copies, earlier Text,
earlier empty frame, nested insets/shift, Shape/empty controls, LIFO and checkpoint
restoration, then the same pinned bilateral SVG/PDF observations without relaxing
1e-8 geometry or semantic membership. Unknown legacy producers retain their
previous fallback unless a measured owning frame explicitly supplies a decision.
This is an ADR-0127 continuous private layout correction, with no public contract,
new product mode/default, compatibility break or eval/layout phase change.
L0 precedes implementation and scoped resealing; full tests/Clippy/rustdoc/fmt,
zero Tekt errors/warnings, paths and forward/reverse lineage remain mandatory.



## R13 — approved materialization

Owner approved the saved proposal SHA-256 d8c9a01413232bac7113d1148af3c1b3bcd671bb37eeca68a94d5749d671db00, including the public carrier and eval-to-layout boundary. Approval and measured baseline are retained in lab/results/construction/diagnostics/relative-lines-r13-2026-10-25/. The previously pending section is now ratified within that exact scope. Apply the pinned nucleus before code; RED→GREEN and complete validation remain mandatory.
Resolve the carrier before existing shape flow placement/reservation and use its resolved path/dimensions there. Preserve all existing block spacing, paint preparation and region behavior. Local tests must distinguish independent axes, em and causal negative/nonzero-origin dimensions.


## Issue 8780 — constrained relative shape measurement (2026-10-31)

### Measurement before decision

`lab/results/construction/diagnostics/issue-8780-fix-2026-10-31/before.json`
freezes UTC, HEAD, exact uncommitted diffstat/status and original source/L0 hashes.
The original report and receipt are in
`lab/results/construction/diagnostics/typst-open-bugs-2026-10-30/`:
its literal 8780 SVG emits a red 0×50pt rectangle in a 100×100pt page.
`01_core/src/compiler/layout/measurement.rs:366-395` uses absolute-only
resolve_pt for Shape dimensions; its unsupported complement at :671 includes
GridCell. `01_core/src/compiler/layout/grid/tracks.rs:218-254` passes that cell
through constrained measurement. `01_core/src/compiler/layout/sub_frame.rs:403-415`
already retains the parent's relative height independently of unconstrained
layout height, both for row measurement and cell emission. Therefore zero width
is independently explained without attributing vanilla's infinite-height row
algorithm to Tekt. Geometry is language behavior; dispatcher strategy is mechanics.
Upstream issue 8780 describes a separate zero-height mechanism and warns about
applying a height percentage twice. Its report is observed behavior, not proof of intent.
The separate `vanilla-width-controls/receipt.json` and
`historical-width-controls/receipt.json` in the same 8780 diagnostic directory
measure width50% in an Auto column: pinned vanilla emits25pt, historical Tekt
bare emits50pt and wrapped emits0pt. Width measurement and final column-relative
width are separate stages; that control does not define a new vertical base.

### Exclusive correction and refuters

Expose the existing pure resolve_dimension helper only to sibling layout units
through pub(super), so constrained measurement uses the same ratio/mixed/em
arithmetic and nonfinite relative-term policy as emission. Preserve its body and
all existing geometry/flow/paint/default behavior; introduce no public signature.
This is continuous-flow internal formula/parity repair under ADR-0127. L0 and
restricted seals precede RED tests and code. Require RED→GREEN, core layout and
workspace validation, build, full crystalline-lint and source PDF/SVG controls.
Any second application of the height ratio to the Auto row result, changed
parent state, missing body, zero finite geometry
or unrelated control regression refutes the correction. Unbounded percentages
retain the existing zero-relative-term policy; broader cyclic sizing, fractional
shape sizes and vanilla row semantics are not certified by this repair.


## Issue 8894 — approved Stroke migration

Measurement: the source inventory and previous owner bytes in
`lab/results/construction/diagnostics/issue-8894-2026-10-31/` bind exact
HEAD/dirty diffstat/UTC and each affected source/owner before migration.
`stdlib/layout/stroke.rs:430–434` erases em; geometry.rs:81 carries only f64.
The explicit user approval in approval.json ratifies the public Length field
and occurrence-layout resolution, as specified by
`00_nucleo/prompts/core/entities/geometry.md`. This consumer retains its
exclusive responsibility and all other ratified clauses and controls.

Migrate this consumer's declarative Stroke literal thickness from scalar
points to Length::pt without changing their value or other fields. FrameStroke
remains scalar. Keep all existing semantic assertions; update only their
representation as necessary. Producers preserve full Length; layout consumers
resolve against effective font size before scalar geometry. Repr preserves
absolute/em components. RED→GREEN, build/tests and scoped lineage are required.


## Bodyless rectangle intrinsic geometry — issue 8976 (2026-10-31)

### Measurement before decision

The historical independent report/fixtures under
lab/results/construction/diagnostics/typst-open-bugs-2026-10-30/agent-text-svg/
separate Tekt's zero-height literal rectangle from vanilla's Edge mask defect.
Current source shape.rs:233-248 and measurement.rs:385-407 substitute available
width and zero height for omitted dimensions. native_rect at
stdlib/shapes/rect.rs:33-34 correctly retains omitted fields for layout; baking
resolved dimensions there would change source semantics. Source inspection is
not a claim about upstream intent.

The pinned upstream shapes.rs:542-558 explicitly defines a 45x30pt default,
capped by region size and replaced independently on forced axes.
issue-8976-geometry-2026-10-31/oracle-receipt.json under
lab/results/construction/diagnostics/ binds actual oracle SHA-256, source,
UTC, dirty HEAD/stat and 45x30 omitted/Auto/24pt-font observations, 20x30 narrow
and 45x15 short controls, plus explicit/zero/relative cases. before.json.gz
freezes the combined productive/L0 inputs. The reflection probe is rejected
by the oracle and is a separate unsupported observation, not a sizing criterion.
These are physical geometry/language obligations, not mandatory PDF bytes,
soft-mask implementation or copied Rust layout. Changed explicit geometry,
paint-dependent size, lost text/flow, font scaling of the default or duplicated
height ratios refute this correction.

### Classification and exclusive correction

Internal geometry/parity repair, continuous flow ADR-0127. No public contract,
constructor field, mode/flag/default path, compatibility or phase change.
This supersedes the earlier blanket automatic-dimension debt only for bodyless
Rect/RoundedRect intrinsic defaults; all other unresolved scopes and approvals
remain intact. L0 and restricted seal precede functional RED and code.

Own a private pure intrinsic_defaults helper for bodyless Rect/RoundedRect only:
choose 45pt horizontally and 30pt vertically, each capped at the corresponding
nonnegative available regional axis; infinite constraints retain those finite
sizes. Feed those fallbacks to the existing resolve_dimension calls. Omitted
and Auto axes use these defaults; explicit lengths (including zero), relative
and mixed values continue through the existing resolver without forcing aspect
ratio or shrinking an explicit size. Keep Ellipse/Path/Line legacy defaults and
all Circle/Square constructor and body support debts outside this change.
Export nothing publicly, add no entity field or constructor-origin value, and
preserve paint preparation, source identity, shape emission/flow/baseline and
fitting. Cite pinned shapes.rs:542-558 for the intrinsic constants. Tests cover
omitted/Auto, one explicit axis, zero, small finite and infinite regions, font
independence and the complete emitted physical frame.

### Acceptance

Require fresh combined-state source reproduction, assertional module RED then
GREEN, original literal plus explicit-size and solid-paint controls in PDF,
SVG and independent raster. Preserve finite/infinite regional constraints,
one/zero/relative/em axes, reuse at different fonts, Box/Block, following text
and existing 8780/8525/8894 behavior. Gate on geometry and visible content;
compare alpha differences separately. Do not introduce, copy or use /SMask or
Edge-specific behavior as a target. Revalidate core tests, cargo build, full
crystalline-lint, ownership and both seals with artifacts exclusively in
/dev/shm/typst-crystalline-8976-target. Retain exact HEAD, dirty stat, UTC,
source/binary hashes and failed attempts. No commit or branch/worktree change.


Only an absent axis or explicit Value::Auto receives the intrinsic fallback.
The private helper may inspect each optional axis value to retain the old
available-width/zero-height fallback for other unsupported Value classes;
this repair does not widen dimension casts or change their diagnostic debt.
All valid explicit values continue through the same existing resolver.


## Issue 8583 — approved consumer obligation

Measurement and explicit public/phase approval are bound by before/combined
source archives, baseline.json and approval.json under
`lab/results/construction/diagnostics/issue-8583-2026-10-31/`.
The radius caster at casting.rs:31–39 erases ratios and rect.rs:44–56 drops
side dictionaries. These are language geometry/paint losses.

Resolve rect_radius after real dimensions and font are known; preserve flow
and emit its decoration through a child render-layer module. rect_stroke Some
may produce fill plus distinct resolved border paths, superseding one-Shape
emission only in that new case. None preserves every existing route.
