# L0 — infra/export/gradients/adaptive
Hash do Código: d92b5ba8

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

## Exhaustive precise-sampler space decisions

### Source evidence before decision

At HEAD `841c9855f8ad6695f4e140d1ee4a33e4f4a46689`,
`03_infra/src/export/gradients/adaptive.rs:213–307` contains
`sample_gradient_precise_for_svg`. The early gate at `:219–232` admits only
Oklab, LinearRgb, Oklch, Hsl and Hsv; other spaces use the existing domain
sampler. The V16-087 decision at `:272–276` selects hue index 2 for Oklch,
index 0 for Hsl/Hsv and no index otherwise. Native component extraction is at
`:253–266` and reconstruction at `:291–305`.
`01_core/src/entities/color.rs:305–315` defines nine ColorSpace variants.
This evidence is static reachability/arithmetic classification, not a new
sampler-accuracy or performance result; the row's source/L0 digests are in
`lab/results/v16-cause-analysis-2026-10-19/entities-boundaries-inventory.json`.

### Decision and preserved complement

Name the no-hue alternatives Oklab, LinearRgb, Srgb, Luma, Cmyk and Spot
explicitly at the hue-index match. Only the first two reach that decision under
the existing gate; the latter four retain domain-sample fallback and gain no
native precise-sampler support. Preserve Oklch index 2 and Hsl/Hsv index 0.
Give the dominating gate an explicit exhaustive ColorSpace decision as well,
keeping the same five precise and four delegated alternatives. For the downstream
ColorSpace reconstruction, classify the same four excluded alternatives explicitly
with its existing unreachable invariant; do not expand native reconstruction,
return a fabricated color, or introduce a public sampler type. The component-pair
mismatch remains an invariant diagnostic, not a new accepted conversion. A future
ColorSpace must force a local decision instead of inheriting the gate or hue default.

Preserve Conic's early domain sampling, t/offset/local f64 precision, separate f32
weights, operation order, shortest hue routing, normalized result hue, alpha,
coincident-stop side selection, empty/single-stop behavior, midpoint clipping,
premultiplication, budgets and subdivision cap. No supported SVG paint pair or
PDF strategy is promoted by this classification.

### Acceptance and exact-site evolution proof

Keep `p1257_tests::p1278_sampler_polar_preserva_precisao_e_pesos_vanilla`,
`p1257_pre_pos_serializacao_oklab_below_cap`,
`p1257_pre_pos_serializacao_linear_rgb`,
`p1262_oklab_saturado_recorta_antes_de_premultiplicar`,
`p1262_oito_stopsets_saturados_sao_materializados` and
`p1262_p1237_full_dump_cobre_vinte_e_quatro_fixtures` unchanged.
Propose focused controls for the three hue families crossing the 360-degree
boundary, the two reachable nonangular spaces, and Srgb/Luma/Cmyk/Spot early
fallback, including native alpha, endpoints and coincident stops. Compare fallback
to the existing domain sample contract without weakening numeric budgets or
substituting byte equality for language/visible color. Retain same/different spot
colorant behavior; no new gamut policy is authorized.

In a disposable copy add a sentinel only to copied ColorSpace.
Require attributable E0004 at the hue-index match after repair, independently of
errors at the gate, component match or reconstruction. Baseline lack of E0004 at
that site and normal no-sentinel library/test compilation are separate evidence.
An early dependency error or an error at another match is inconclusive. Tests
must preserve the selected arithmetic, not mirror an enum-name list.

This is an internal classification correction under ADR-0127 continuous flow:
no public field, trait, signature, product default, compatibility rule, dependency
version or pipeline phase changes. Preserve the owning layer and static dispatch.
No suppression, severity change, `if let`/`matches!` replacement, or helper that
relocates the unchecked complement is an acceptable repair. Validate V15/V26
before restricted forward/reverse resealing. Freeze source/dependency identity during structural and functional validation;
record exact targets, compiler diagnostics, resource settings and outcomes before
claiming acceptance. A structural proof is not a functional test result.

### Attributable actual-domain omission proof

For V16-087, an equally strict structural alternative avoids a new enum variant
preventing core or external-dependency metadata emission before the real infra
consumer is type-checked. Use two disposable copies with the same unchanged real
dependencies: one with the original wildcard decision and one with its repaired
exhaustive classification. At the hue-index match at adaptive.rs:272–276, remove only
the complete unguarded Oklch => Some(2) arm in each copy. This arm is unique and unguarded; no other arm
in that match handles Oklch. With the omission, the wildcard baseline
still covers Oklch and must lack E0004 at this site; the repaired copy must
emit E0004 at this exact match. Retain function/target identity, mutation diff,
source/dependency hashes, compiler JSON and no-omission controls. An unrelated
error or dependency failure remains inconclusive. No replacement stub, new public
API, linter or target-scope change is allowed.

This paired omission experiment proves that the actual-domain classification no
longer has an implicit complement. The normal repaired match must remain fully
nominal and exhaustive, so future enum growth still forces an explicit owner
decision. This alternative changes only how the structural obligation is measured;
it weakens neither the future-variant obligation nor the independent functional
controls, and no probe result is claimed by the specification.
