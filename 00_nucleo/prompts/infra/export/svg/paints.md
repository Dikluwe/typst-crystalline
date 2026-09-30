# L0 — svg/paints
Hash do Código: 836c88f5

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

Tiling supports only resolved size plus Color body, retaining size/spacing/relative
semantics. Opaque/unresolved tiling keeps tiling-content-or-size fallback. Unsupported
paint retains its existing first-color geometry-preserving fallback with the correct
data-crystalline-fill-fallback or stroke marker and Unknown classification. Do not
promote unsupported pairs by wildcard or add ICC/downloads/decoders here.

## Verification

Exercise each supported pair/role, negative spaces, references, focal/aspect geometry,
stop discontinuities/alpha, adaptive representation, conic local error budgets and
tiling support. Do not replace the existing validated numerical envelopes with
stop counts, IDs or XML bytes as parity criteria.
