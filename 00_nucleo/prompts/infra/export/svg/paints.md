# L0 — svg/paints
Hash do Código: e496e1b6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/paints.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/svg/paints.rs:19` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Paint servers and support boundary

Own PaintDefs/SvgPaintDef/PaintReference, support/fallback predicates, local definition
deduplication and linear/radial/conic/tiling/stops writers. References resolve within
the SVG graph and preserve their fill/stroke role. Definition reuse includes the
observable paint geometry; conic uses the finite positive item box rather than
discarding aspect ratio. Keep alpha separate from color and preserve stop order,
multiplicity, coincident offsets, discontinuities and a unique terminal endpoint.

Linear/Radial support sRGB, Oklab, LinearRgb, Oklch, Hsl and Hsv individually.
Use objectBoundingBox geometry, including the independent radial focal point.
sRGB stops retain the native numeric writer. Other supported spaces consume
svg_adaptive_stops from the adaptive-gradient owner without changing its 0.001
threshold or cap 64; serialize every adaptive offset directly from f64 using
Ratio::repr's two-decimal percentage with offset * 100 followed by percentage
rounding away from zero. Do not narrow to f32 or merge arithmetic operations.
Luma/CMYK and unvalidated spaces remain explicit gradient-color-space fallback.

Conic support is limited to sRGB, Oklab, LinearRgb and Hsv. Emit a vector pattern
of angular wedges referencing local two-stop linear gradients, preserving center,
initial angle, clockwise progression and reciprocal box-aspect correction. Do not
replace the graph by raster or disguised solid. Wedge count is not a parity rule.
For promoted combinations, interior premultiplied encoded-sRGB error is max <=4/255,
p95 <=2/255 and alpha max <=2/255; the complete local mask requires premultiplied
RGBA p95 <=4/255 and at most 0.5% of pixels above 16/255. Derive the mask from the
declared graph/geometry before comparison, including seams/boundaries/stroke; no
error-based crop, erosion, fitting or realignment. Preserve the measured side at
coincident boundaries and the 0/1 seam. Local paint failures are not external layout
Unknown; integration beyond that local geometry remains outside this owner.
Other conic spaces, including CMYK, keep conic-gradient fallback.

The earlier size-plus-Color-only tiling boundary is superseded by the approved
prepared vector-cell contract below. Opaque/unresolved unprepared carriers retain
their explicit tiling-content-or-size failure boundary. Unsupported
paint retains its existing first-color geometry-preserving fallback with the correct
data-crystalline-fill-fallback or stroke marker and Unknown classification. Do not
promote unsupported pairs by wildcard or add ICC/downloads/decoders here.

## Verification

Exercise each supported pair/role, negative spaces, references, focal/aspect geometry,
stop discontinuities/alpha, adaptive representation, conic local error budgets and
tiling support. Do not replace the existing validated numerical envelopes with
stop counts, IDs or XML bytes as parity criteria.

## Numeric evidence and prepared vector patterns

`lab/results/construction/diagnostics/gradientes-numericos-ci-2026-10-13/before.json.gz`
retains library/consumer/owner/manifest/lock/spans/HEAD/tree/UTC/status/diffstat at
paints.rs:165,166,259,260,299–302,327–330. clippy.toml requires platform-independent
math, libm =0.2.11 already locked. Full24-source before-render/binary/command/
whole-canvas comparison evidence retains pre-existing differences and invalid
initial hsv fixture in that package, not a global parity claim.
Use f64 libm::sin/cos/atan2 preserving y/x argument order, post-rem_euclid negation,
aspect/center/wedges/stops/fallback, promoted spaces/thresholds/masks/envelopes.
Pin direct L3 =0.2.11 without upgrades/I/O. Continuous ADR-0127 no public/default/
phase/strategy; L0 first/scoped seals. Existing formulas/same24 documents PDF-SVG
before/after/raster/structure, separately compare preservation and historical
vanilla divergences; analyze new render changes, no changed tolerance/fixtures/
suppression. All-target compile/infra suite/build/Clippy/Rustdoc/fmt/lineage;
counts/files alone prove no parity.

## Prepared Shape fixture transport

### Evidence and approval

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz`
retains HEAD/tree/diffstat/UTC/sources/owners before migration. layout_types.rs:510–511
carried Paint/Stroke; shape.rs:151–191 materialized only group fills; tiling.rs:85
limited/cloned its grid. prepare_cell already returns TilingFrame once per
occurrence. `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
shows dense-open/stroke-open divergences across all three targets. Manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` was approved;
receipt `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Preparation/object counts are transport mechanics; rendering/outlines/text/geometry/
diagnostics are language observables.

### Contract and acceptance

Store FramePaint and placement matrix in private definitions. Dedup Solid/Gradient
by their data, Tiling by private cell identity+matrix, never serialized address or
language equality inference. Emit prepared tiling as userSpaceOnUse vector pattern
with clipped frame.size cell, pitch width/height and affine-helper patternTransform.
Queue nested definitions without breaking references; share document fonts/glyphs/
clip IDs, local root/decorative mode. Preserve current gradient support/fallback;
prepared tiling cannot silently fall back to solid. Test density/clipping/Self-
Parent/pivot/large-period/overlap/nested resources.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

## Explicit period composition supersedes visible overflow

`tiling-overlap-2026-10-14/before.json.gz` retains sources/owners/HEAD/UTC/tree;
native tiling final-renders/remaining-analysis.md observes8×8cell/4×4pitch/50%alpha:
opaque-red PDF/one SVG contribution/four PNG(255,15,15). pdf_tiling.rs:184 BBox=size,
paints.rs:148 overflow visible and text/shape lost alpha expose composition defects;
vanilla clipping overlap is no requirement to copy. Analytic source-over and
nonoverlap controls refute incorrect composition.

Store clipped vector cell body once as identified g in defs. Pattern viewport=pitch,
overflow hidden, translated use per helper contribution in preserved order; never
rely on overflow visible. This specifically supersedes previous overflow-visible
clause. Keep nested queue/unique glyph-font-clip IDs/local root/decorative-link
exclusion. No isolated semantic links from cell. Continuous ADR-0127 within approved
native package adds no API/phase/flag/default. L0 first/RED→GREEN/analytic distinct-
color ordering/simple-nested-rotated-text-image/page selection-extraction controls,
build/workspace suite/fmt/Rustdoc/lineage. Variable-gradient alpha/PNG quality remain
open; no production certification with them uncompleted.

## Cache exhaustivity and borrowed cell session

verified-v3/followup-clippy-inventory.json/followup-architecture.log retains
candidate-final/workspace-pre structural findings. Private comparator names all
six distinct-variant pairs false, no wildcard hiding future alternatives. Cache
identity/equality is not language equality; no limit increase/allow. Preserve suite/
renders before-after/fmt/doc/lineage.

clippy-aridade-final-2026-10-16/before.json.gz/callers.txt retains checked source/
L0/HEAD/tree/UTC/status/diffstat/hashes/workspace+fixtures; MathLayouter homonyms
stay separate. Original22 warnings/five arities are representation, no parity claim.
Migrate tiling-cell dispatcher to private SvgRenderSession from
`00_nucleo/prompts/infra/export/svg.md`: reborrow same fonts/glyph_defs/self/
clip_next_id/glyph_fonts, destinations empty/html_frame false. No cache copy/new
counter. Keep temporary identity/decorative true, restore old pair after borrow;
nested queue/order/clip/contributions/period/PaintDefs.write signature unchanged.
This materializes only named private scope, superseding its earlier pending status.
No external/default/phase/compatibility change; L0 first/scoped seals, real arity
RED→GREEN/no suppression/threshold change, old expectations/body/value transport,
Grid/Table/outline/SVG-tiling controls before-after/full workspace suite/build/
all-target Clippy/Rustdoc/fmt/strict-reverse lineage/Tekt no new warnings under12GiB/
zero swap/one job; no carrier-mirror/global-parity/PDF-UA claims.
