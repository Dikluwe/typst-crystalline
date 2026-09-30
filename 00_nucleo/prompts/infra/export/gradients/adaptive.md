# L0 — infra/export/gradients/adaptive
Hash do Código: fc0b870f

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/adaptive.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/adaptive.rs:42` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## PDF sample-count selection

Own perceptual_distance_in_space, adaptive_n_for_stops and the separate SVG adaptive
adapter. PDF distance is Euclidean Oklab in f32; the space parameter is reserved
and does not select a native metric. For fewer than two stops return 16.
Otherwise use max adjacent-pair distance: below 0.05 → 16, below 0.30 → 32,
otherwise 64. Keep the existing PDF applicability selected by the resource caller;
this helper does not choose a shading strategy.

## SVG adaptive stops

Own svg_adaptive_stops, effective f64 offsets and private precise sampling/error
helpers. Linear/Radial preserve original stop order, alpha and coincident offsets.
Subdivide only positive-length intervals when anti_alias is enabled, comparing
the exact midpoint to encoded-sRGB interpolation between endpoints after
premultiplication. Threshold is 0.001 and maximum subdivision factor per interval
64; terminate deterministically. The Conic path returns original colors/effective
offsets without applying this Linear/Radial bisection.

For Oklab/Oklch, clamp encoded-sRGB float channels before endpoint midpoint mixing,
and clamp exact/approximate channels before alpha premultiplication/error comparison.
Do not clamp native-space sampling prematurely or defer this boundary to u8.
LinearRgb and other paths retain their current error conversion; no global Color
gamut policy is introduced.

The precise sampler covers Oklab, LinearRgb, Oklch, Hsl and Hsv: keep t/offsets
and local parameter in f64 until materializing separate f32 weights 1-local/local.
Each component uses (w0*c0+w1*c1)/(w0+w1). Apply short hue routing at Oklch index 2
and Hsl/Hsv index 0, retaining native-space alpha; normalize the resulting hue.
Other spaces keep the existing domain sample path. Preserve duplicate-stop side
selection, endpoints and interpolation of unspecified offsets. Do not reshape
the domain Gradient::sample API.

## Verification and scope

Preserve the adjudicated local budget: maximum encoded-sRGB premultiplied error
<= max(0.001, measured vanilla error)+1e-6; p95 <= measured vanilla p95+1e-6;
alpha maximum/p95 <= the measured vanilla envelope+1e-6. Coincident points are
adjudicated by declared order; continuous comparison starts at epsilon to the right.
Use a pinned, independently measured oracle, not automatic acceptance after u8
saturation, larger budgets/caps, stop counts or byte equality. Historical upstream
evidence and fixture provenance live in the export sanitization diagnostic; those
measurements do not establish parity against a later pin without remeasurement.

Supported SVG paint pairs are owned by
`00_nucleo/prompts/infra/export/svg/paints.md`; generating samples does not promote
Luma/CMYK or an unvalidated pair. PDF count selection remains independent.
