# L0 — entities/gradient/radial
Hash do Código: 330593e5

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/gradient/radial.rs`

## Evidence before specification

At clean commit `b7c6bbe0a0bfe3bebf0bb071b33fa0da51574229`,
`01_core/src/entities/gradient.rs:621-795` contained the extracted behavior.
Source SHA-256, timestamp, and initial state are recorded in
`lab/results/construction/diagnostics/atomizacao-gradient-entity-2026-09-30-before.json.gz`.

At reviewed HEAD `841c9855f8ad6695f4e140d1ee4a33e4f4a46689`,
`01_core/src/entities/gradient/radial.rs:125-181` implements sample_precise;
its ColorSpace decision is at lines 142-173. The frozen V16-079 inventory
and causal ledger in `lab/results/v16-cause-analysis-2026-10-19/` agree with
the consumer and this owner's pre-edit hashes. The closed enum at
`01_core/src/entities/color.rs:305-315` has nine variants.
`01_core/src/entities/gradient/interpolation.rs:98-139` already dispatches
all nine spaces, including the complete-colorant Spot behavior. These are
source/contract observations, not executed tests, a new precision measurement
or evidence that every space has the same numerical implementation.

## Contract

Define Radial and its sampling methods. Preserve public contracts, defaults, sampling semantics, and the parity
criteria in `00_nucleo/prompts/core/entities/gradient.md`. Remain in L1,
without I/O or mutable global state.

Keep sample_precise's two specialized branches distinct: Oklab and LinearRgb
retain f64 parameter/offset/local calculation until separately materializing
`w0 = (1.0 - local) as f32` and `w1 = local as f32`; retain the existing
weighted expression and total for every channel, including alpha. Do not
replace those expressions with a lerp or reuse the historical sample(f32)
path. The parent owner's bounded precision correction remains in force.

The seven other spaces, Srgb, Luma, Oklch, Hsl, Hsv, Cmyk and Spot(_), form
the explicit delegation class. Name these alternatives in the existing
ColorSpace match and keep the same interpolate_in_space call with the same
two stop colors, `local as f32` and selected space. Delegation performs actual
interpolation; it is not a neutral fallback. Keep the SpotColorant payload and
its identity. Do not convert all output to sRGB or copy the interpolation
algorithm into Radial. A future ColorSpace variant must require a decision
at this sampling match rather than silently enter the delegation class.

Preserve clamping, empty/single-stop results, effective-offset and interval
selection, coincident-offset behavior, endpoint returns, focal independence
and anti_alias independence of pure one-dimensional sampling. The general
interpolator retains shorter-hue behavior and same-complete-colorant tint
interpolation; its existing mixed/different-colorant process conversion is
not broadened here. Explicit constructor conversion errors remain the
constructor owner's responsibility. This nominal correction adds no precision,
color-space, focal, render-budget, export or parity promise.

## Acceptance

Gradient tests and the L1 suite must pass; `crystalline-lint .` must validate
ownership and hashes. Do not infer a performance improvement.

Preserve the existing local `preserves_variant_behavior` test and parent
gradient tests `p264_radial_effective_offsets_auto_spacing`,
`p264_radial_sample_extremos`, `p264_radial_sample_clamp_above_1`,
`p269_radial_sample_inalterado_por_focal_default`,
`p269_radial_sample_inalterado_com_focal_explicit`, and
`p270_radial_sample_oklab_preserva_p264`. Retain the existing
`p270_radial_sample_srgb`, `p270_radial_sample_linear_rgb`,
`p270_radial_sample_luma`, `p270_radial_sample_oklch`,
`p270_radial_sample_hsl`, `p270_radial_sample_hsv` and
`p270_radial_sample_cmyk` controls. Those mostly call sample(f32), and some
assert only successful execution: they are not sufficient proof of the
sample_precise decision.

Local regression controls must directly exercise sample_precise in both
specialized spaces and every delegated space with explicit expected color
components/alpha or spot identity/tint. Include shorter-hue wrap, coincident
offsets, zero/one stops, clamped endpoints and matching/different Spot
colorants. Freeze expectations independently of the edited match; do not
make a tautological test whose sole expected value is the same delegated
call. Preserve existing tolerances and exact previously measured precision
criteria. A changed component, lost Spot identity or changed interval/weight
boundary refutes this behavior-preserving grouping. New bilateral probes,
if needed to settle an expected value, must use the current ratified pin
and complete provenance rather than historical certificates.

Structural RED/GREEN uses a sentinel added only to ColorSpace in a disposable
complete workspace. Require E0004 attributable to sample_precise's selected
match at `01_core/src/entities/gradient/radial.rs:142-173` after correction,
rather than accepting errors elsewhere in its function or other color matches.
No sentinel enters productive code. Do not evade V16 through if-let/matches!,
an auxiliary wildcard, a suppression or changed linter/configuration. Keep
informative V19/V20 alternatives visible and review their actual contracts.
This is ADR-0127 continuous internal work with L0 first and unchanged public
fields/signatures/defaults/phase/compatibility. Keep pure L1, static dispatch,
1:1 ownership and valid V15/V26 before restricted resealing and
full regression/build/lint gates. Nominal coverage does not establish new
parity or performance.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/gradient/radial.md`. Historical observations are not a new parity certification.
