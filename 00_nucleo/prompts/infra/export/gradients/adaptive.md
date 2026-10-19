# L0 — infra/export/gradients/adaptive
Hash do Código: 462b00f9

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

## Internal numeric expressions

### Evidence

adaptive.rs:390 uses the selected math method. `lab/results/construction/diagnostics/gradientes-numericos-ci-2026-10-13/before.json.gz`
retains library/consumer/owner/manifest/lock/spans/HEAD/dirty tree/UTC/status/diffstat.
clippy.toml requires platform-independent math; libm =0.2.11 is already locked.
before-renders.json has 24 sources rendered PDF/SVG by Tekt and historical vanilla
9f2b6e8715237cb086899a42873660fe744622e8 with binary hashes/commands;
before-comparison.json observes full canvas without error-adjusted masks.
Pre-existing divergence is not complete parity. Rejected initial hsv-outside-color
fixture stays in initial-fixture-error.log; cases.json has valid inputs.


ci-simplificacoes-2026-10-14/before.json.gz retains source/owner/spans/HEAD/dirty
state/UTC/diffstat at :159; red.log records selected errors before code, types/
uses checked under pinned1.92.0. These are Rust expressions, no new parity.

### Contract and acceptance

Only the test error helper replaces powi(2) with local delta*delta, preserving
sum/sqrt/premultiplication/budgets. Productive sampling unchanged. Subdivision
next_t already f64 passes directly to Ratio without identity From; preserve exact
precision/operations/order/budgets/coincident offsets/alpha/space selection/
interpolation/subdivision cap. No public/default/phase/unsafe/dependency/suppression
beyond existing pinned numeric dependency. Selected-lint RED→GREEN/full suite/build/
all-targets/Rustdoc/fmt/lineage/full inventory, unchanged control expectations and
no simplification-mirror tests. Internal continuous ADR-0127 adds no public/default/phase/strategy; L0 first,
involved-owner seals. Direct L3 libm =0.2.11 reuses locked package, no upgrades/I/O.
Existing formula tests and same 24 before/after PDF/SVG sources, raster/structure
observations separate local preservation from historical oracle differences.
Analyze any new render difference; no larger tolerance/limit/fixture changes or
suppressions to hide failure. All-target compile/infra suite/build/Clippy/Rustdoc/
fmt/lineage; counts/file identity certify no language parity.
