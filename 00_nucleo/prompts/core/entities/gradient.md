# L0 — Gradient facade and shared semantic contract
Hash do Código: 1482e2f0

**Layer:** L1. **Exclusive consumer:** `01_core/src/entities/gradient.rs`.
The facade owns GradientStop, RelativeTo, public reexports and historical local
tests. Separate owners define offsets, variants, kind and interpolation:
`00_nucleo/prompts/core/entities/gradient/offsets.md`,
`00_nucleo/prompts/core/entities/gradient/linear.md`,
`00_nucleo/prompts/core/entities/gradient/radial.md`,
`00_nucleo/prompts/core/entities/gradient/conic.md`,
`00_nucleo/prompts/core/entities/gradient/kind.md` and
`00_nucleo/prompts/core/entities/gradient/interpolation.md`.
The common semantic obligations below are their boundary acceptance criteria,
not permission for a second implementation in this facade. Color and Paint
remain owned by `00_nucleo/prompts/core/entities/color.md` and
`00_nucleo/prompts/core/entities/paint.md`. No L1 I/O or mutable global state.

## Evidence before the current contract

The initial linear-only subset grew to Linear/Radial/Conic, then focal fields,
eight process spaces, relative coordinates and anti-alias state. Its former
reservation comments were not unit placeholders. Those bounded omissions are
superseded only by the explicit later additions below; the old labels
“3/3 complete” and “24/24 complete” never close unrelated limits or establish
current-pin parity. Shared Arc storage is the local representation, not an
upstream-mechanics parity obligation. Historical ADR-0086/0087/0088/0089,
ADR-0091/0092 and their cumulative records preserve that design provenance;
apply current governing ADRs rather than reviving revoked guidance.

At historical `e055a12f`, repeat(2) on four automatically spaced Linear RGB
stops exposed `0, 0.1666666716337204, 0.3333333432674408, 0.5, 0.5,
0.6666666865348816, 0.8333333730697632, 1` versus upstream
`0, 1/6, 1/3, 1/2, 1/2, 2/3, 5/6, 1`. Local code first resolved through
Vec<f32> and reconstructed Ratio(f64); upstream gradient.rs:664–688 consumes
resolved Ratio(f64). Removing that narrowing alone still failed RED: the
native_gradient_stops accessor independently narrowed again. Both first-loss
points require their own functional controls; changing only repeat is
insufficient evidence of public precision.

After the earlier color/offset corrections, Radial sampling at
`t = 37.123456789%` still narrowed its public argument to f32. Oklab black/white
lightness was `0.3712349534034729` versus `0.37123456597328186`; a/b already
agreed. Linear RGB red/blue first channel was `0.6287654638290405` versus
`0.6287654042243958`. Upstream gradient.rs:846–860 forwards t:f64 and
:1461–1484 creates f32 weights only at mixing. The hypothesis that argument
narrowing causes this discrepancy is refuted if removing it fails to reproduce
those public components. This is component-level acceptance, not a render,
quantization or budget change.

Historical ratified upstream `a51e02804`, visualize/gradient.rs:545–716,
sets anti_alias=false for sharp, true in constructors and preserves it in
repeat. This affects visible sharp transitions and cannot be discarded as
unobservable mechanics. The owner approved the public field gate with
“Continuar” before the L1/L3 changes; the original owner explicitly records
this package as approved and materialized.

For spot sampling, upstream visualize/color.rs:1147–1245,2471–2521 chooses the
shared complete colorant when all stops have it, otherwise Oklab.
`gradient.linear(s.tint(50%), s.tint(25%)).space() == s` and sample(50%) returns
s.tint(37.5%). Adding red changes automatic space to oklab; explicit space:s
with a process stop rejects conversion to spot. Local gradient.rs:204–273,
369–385,478–500 previously carried only process ColorSpace and prenormalized
stops. The owner confirmed the spot public package on 2026-09-23 under ADR-0127.
That sampling gate does not itself certify spot-gradient PDF/SVG/PNG export.

The 2026-10-13 color evidence records HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree, sources, hashes,
UTC, status and diffstat in
`lab/results/construction/diagnostics/cores-ci-2026-10-13/before.json.gz`
and constructors-before.json.gz, with probes/numerical proof in evidence.json.gz.
Its historical vanilla pin is `9f2b6e8715237cb086899a42873660fe744622e8`,
verified executable SHA-256
`7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
All eighteen proposed interpolation coefficient spellings compile to exactly
the same f32 bits (literals.rs/literals.json). Equivalent spelling is distinct
from precision loss; only these demonstrated alternatives are permitted.

The same HEAD/tree/pin/executable and source provenance appear in
`lab/results/construction/diagnostics/limites-cor-2026-10-13/before.json.gz`,
parents-before.json.gz and evidence.json.gz. red.rotate(45deg) exposed negative
blue locally versus upstream zero; rgb(10%,20%,30%,40%) exposed 10.2%/30.2%
instead of 10%/30%; Linear RGB accepted 256 and alpha−1% rejected upstream;
negative Oklch chroma became chromatic locally versus gray. Preserve complete
outputs and extra public digits. Upstream color.rs:1825–1895 distinguishes
bounded FromColor, identity and from_linear; palette 0.7.6
convert/from_into_color.rs:53–59 clamps after conversion.
encoding/srgb.rs:100–120 uses reciprocal multiplication in linear decode and
mul_sub in encode; num.rs:718–722 implements mul_sub without fusion. The older
fmaf-encode assertion is therefore refuted for this source. oklab.rs:355–368
clamps chroma to zero before polar conversion; hues.rs:155–157 uses sincosf
through libm. None of these old measurements certifies the current upstream pin.

## Measured CMYK endpoint expectation correction — 2026-10-21

`lab/results/cmyk-box-corrections-2026-10-21/agents/cmyk/gradient-endpoint-audit/receipt.json`
(SHA-256 `36c300812ac93c727d260d76a783427b70b847f92c860a5d4c223f215b439013`)
records exact fixture bytes, commands, complete stdout/stderr, frozen productive
input hashes, uncommitted HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, dirty-file stat and
UTC `2026-10-22T00:22:44.584114+00:00` through `2026-10-22T00:22:45.146508+00:00`. Both evaluations exited zero with empty stderr and
unchanged inputs. Ratified vanilla commit
`bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, binary
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`,
and identified candidate `90d348465b4b23e7f8ff3a808ec8d5d2e15ea2e7652ba7616cc37e3fbd66d8a4`
returned identical language values for
`gradient.linear(rgb(255, 0, 0), rgb(0, 0, 255), space: cmyk)` endpoints.
At 0%, CMYK components are `(0, 1, 1, 0)`, hex is `#ed1d24`, and
normalized RGBA is `(237/255, 29/255, 36/255, 1)` in f32. At 100%, components
are `(1, 1, 0, 0)`, hex is `#2e3092`, and normalized RGBA is
`(46/255, 48/255, 146/255, 1)` in f32. The latter blue channel is
`0.572549045085907`, refuting the historical test's `b1 > 0.7` assumption.

Pinned `visualize/gradient.rs:1369–1401` converts stop colors to the selected
mixing space; `:1461–1484` samples those prepared stops. Existing local
`entities/gradient/interpolation.rs:271–289,353–358` preserves CMYK endpoint
components, and the canonical Color conversion now projects them through the
measured fixed profile. This is language-value evidence, not a requirement to
copy upstream preprocessing mechanics. It indicates a refuted old naive RGB
expectation, not a gradient algorithm defect; differing endpoint components or
RGBA in a fresh source-bound comparison would refute that classification.

The actual full-suite RED is retained in
`lab/results/cmyk-box-corrections-2026-10-21/final/validation/tests-domain-infra.log`
(SHA-256 `3550c8231b7950b7d78a73f20bad3a62497881bf681a23eb6a5a7c07f218bdf9`): the unchanged
`entities::gradient::tests::p270_linear_sample_cmyk_endpoints` failed at
`gradient.rs:1074` with `assertion failed: b1 > 0.7`. The source and measured
oracle values above refute that expectation before its replacement.

Replace only the facade's `p270_linear_sample_cmyk_endpoints` heuristic channel
thresholds and their naive-conversion comment with exact assertions for both
CMYK targets, raw CMYK components with and without alpha, both measured hex
strings and all normalized f32 RGBA channels.
Expected values come from the ratified oracle above. Preserve all other
sampling, interpolation, alpha, space, offset, tolerance and fixture contracts.
Do not relax the 0.7 threshold, modify interpolation, remove/ignore the test or
claim complete gradient/ICC parity. This is ADR-0127 continuous internal test
expectation correction: L0 first, the recorded full-suite RED, exact replacement
and focused/full revalidation, with no public signature/default/phase change.

## Facade data and public composition

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct GradientStop {
    pub color: Color,
    pub offset: Option<Ratio>,
}
impl GradientStop {
    pub fn new(color: Color, offset: Ratio) -> Self {
        Self { color, offset: Some(offset) }
    }
    pub fn unspaced(color: Color) -> Self {
        Self { color, offset: None }
    }
}
```

GradientStop loses Copy because Color now carries SpotColorant; its constructor,
offset and semantics do not change. Necessary clones are Rust mechanics.
Retain `RelativeTo { Self_, Parent }`, Default=Self_, with per-variant
`relative: Option<RelativeTo>`: None means Auto, resolved to Self_. Public
reexports include Gradient, GradientStop, Linear, Radial and Conic; the public
color_to_oklab_with_alpha helper remains accessible to L3. Keep the facade's
module/public reexport paths and historical tests. Reexport implementation
belongs to the entities/mod owner, not another productive consumer here.

The following is the shared data schema of the child-owned types, retaining
all fields as additions to their earlier forms:

```rust
pub struct Linear {
    pub stops: Arc<[GradientStop]>,
    pub angle: Angle,
    pub space: ColorSpace,
    pub relative: Option<RelativeTo>,
    pub anti_alias: bool,
}
pub struct Radial {
    pub stops: Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub radius: Ratio,
    pub focal_center: Axes<Ratio>,
    pub focal_radius: Ratio,
    pub space: ColorSpace,
    pub relative: Option<RelativeTo>,
    pub anti_alias: bool,
}
pub struct Conic {
    pub stops: Arc<[GradientStop]>,
    pub center: Axes<Ratio>,
    pub angle: Angle,
    pub space: ColorSpace,
    pub relative: Option<RelativeTo>,
    pub anti_alias: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    Linear(Arc<Linear>),
    Radial(Arc<Radial>),
    Conic(Arc<Conic>),
}
```

Preserve Debug/Clone/PartialEq for variant data and derived field equality,
including anti_alias. Arc clones are O(1). Constructors retain the following
API families and their existing signatures; abbreviated argument lists record
names, not permission to change concrete Rust types:

```text
Gradient::linear(stops: impl Into<Arc<[GradientStop]>>, angle: Angle) -> Self
Gradient::radial(stops, center, radius) -> Self
Gradient::conic(stops, center, angle) -> Self
Gradient::radial_with_focal(stops, center, radius, focal_center, focal_radius) -> Self
Gradient::linear_with_space(stops, angle, space) -> Self
Gradient::radial_with_space(stops, center, radius, space) -> Self
Gradient::conic_with_space(stops, center, angle, space) -> Self
Gradient::first_stop_color(&self) -> Color
Linear/Radial/Conic::effective_offsets(&self) -> Vec<f32>
Linear/Radial/Conic::sample(&self, t: f32) -> Color
impl From<Gradient> for Paint
```

first_stop_color supplies the first-stop fallback for Paint::to_color().
Paint::Gradient is active and Paint is not Copy. This package preserves
`TextStyle.fill: Option<Color>`; it does not migrate that field to Option<Paint>.
Linear-only, radial-only and reserved-conic enum excerpts are superseded by the
three actual variants; do not restore placeholders or discard later fields.

## Variant defaults, spaces and surface boundaries

Radial's ordinary constructor defaults focal_center=center and focal_radius=0.
Explicit focal constructors/stdlib arguments preserve both. The corresponding
stdlib accepts focal_center as Array[Ratio,Ratio] and focal_radius:Ratio and
rejects focal_radius>radius and
`dist(focal_center, center)² >= (radius - focal_radius)²`: the focal circle must
be inside the outer circle. Existing no-focal construction remains equivalent,
including `/Coords [cx cy 0 cx cy r]` at the export boundary. Preserve the sixteen
historical radial/focal regression assertions; old struct-literal call sites
receive explicit default fields. Radial sample(t) remains pure one-dimensional
stop interpolation, not two-dimensional/focal evaluation. Focal changes affect
native Type 3 PDF coordinates, not sampling. Conic has no focal_* fields; its
old absence was not a deferred radial feature.

All variants carry selected ColorSpace. The eight process spaces are
Oklab, Oklch, Srgb, Luma, LinearRgb, Hsl, Hsv and Cmyk. Luma is the historical
local name for D65Gray. Default is Oklab, preserving earlier Oklab behavior.
The stdlib space strings remain `"oklab" | "oklch" | "srgb" | "luma" |
"linear-rgb" | "hsl" | "hsv" | "cmyk"`; selected-space sampling dispatches through
interpolate_in_space. Polar HSL/Oklch/HSV interpolation takes the shorter hue
path, wrapping when |h1−h0|>180°, as measured from upstream mix_iter:1126–1136.
The older fixed-Oklab scope-out is superseded by these runtime spaces, not by
unmeasured color conversion or export promises.

Each variant also accepts ColorSpace::Spot(SpotColorant) under its color owner.
Automatic selection chooses spot only when every complete colorant agrees;
otherwise choose Oklab. In chosen spot space, interpolate tint only and retain
Color::Spot identity. Endpoints preserve the original color. Explicit invalid
spot conversion is a language constructor error, never silent process fallback.
sample_precise retains percentage precision. The eight process interpolation
contracts and controls remain unchanged. Spot-gradient rendering is a separate
package, historically completed by the export record P1483 under
`00_nucleo/prompts/infra/export/stream.md`; presence of Color::Spot alone is not
proof of PDF/SVG/PNG support.

Relative stdlib strings are `"self"`, `"parent"`, `"auto"` for all variants.
Default None/Some(Self_) keeps the established pipeline. Parent coordinate
semantics and their deliberately limited container scope are specified below.
The original deferred gradient.sample/stops language surfaces are superseded
only by their separately owned accessors in
`00_nucleo/prompts/core/compiler/stdlib/gradients.md`.

## Precise offsets, interpolation and anti-alias state

Effective offsets preserve explicit Ratios and distribute unspecified stops
uniformly between explicit neighbors. Resolve effective offsets in f64 as one
internal operation shared by precise sampling, repeat and public-boundary
identity normalization. Linear/Radial/Conic may expose it crate-internally;
keep public historical effective_offsets()->Vec<f32> and sample(f32) unchanged
for existing Rust callers. gradient.stops exposes precise effective offsets;
precise Linear sampling accepts both that canonical boundary and historical
f32 identities, including coincident non-dyadic sharp boundaries. repeat and
the language stops accessor must both avoid the two serial f32 losses measured
above. Their focused acceptance compares public Ratios; it does not close sharp,
sampling, quantization, budgets or adaptive render merely by fixing offsets.

Oklab conversion uses the complete palette 0.7.6::oklab::linear_srgb_to_oklab
constants pinned by the Color owner, identical across private duplicates.
Preserve f32 values, each direction's actual gamma operation order and the
cube root bit-compatible with libm::cbrtf. Shortened constants that change bits
are prohibited. The eighteen separately proven equivalent spellings are an
explicit exception to spelling identity, not to value/order identity.
Keep exponential decode fused, encode mul_sub unfused and linear decode's
reciprocal multiplication, superseding the incorrect generic gamma/fmaf
prohibition only where measured. Color conversion corrections belong to the
conversion/interpolation owners; this facade only receives the corresponding
lineage update and does not change sampling/offset/public contracts.

Oklab mixing materializes separate f32 weights and uses
`(w0*c0 + w1*c1)/(w0+w1)` per channel; `c0 + (c1-c0)*t` is not numerically
equivalent. The precise Linear language path retains t and Ratio offsets as
f64 until local weights are calculated, then converts each weight separately.
Precise Radial Oklab and LinearRgb sampling likewise retains f64 t/offsets,
then sets w0=(1−local) as f32 and w1=local as f32 and uses the weighted expression
for every channel, including alpha. That bounded correction leaves Conic and
the other six spaces outside its measured cluster; it does not remove their
existing behavior. Endpoints and public intermediate components must agree
with the measured oracle before tuning adaptive L3 helpers. Preserve render
budget, quantization and cap; no generic parity certificate follows from
remaining probe differences.

Rust/Typst constructors and historical struct literals initialize anti_alias
true. sharp preserves the variant with transformed stops and sets false;
repeat retains its source value. Pure 1D sampling ignores anti_alias. Render
consumers honor it where their strategy supports anti-aliasing, preserving
ordinary-gradient defaults. PartialEq includes it. The approved package also
converted Typst stops to the ratified mixing space, so affected PDF references
were regenerated; that historical receipt is not permission to relax expected
outputs during future changes.

## Export boundary and superseded strategy evidence

Export algorithms remain under the infrastructure export owners, principally
`00_nucleo/prompts/infra/export/stream.md`; this pure facade does not emit PDF.
The inherited common specification records the following retained boundary
criteria and explicit supersessions, rather than authorizing export code here.

Linear/Radial RGB-family and perceptual export consumes the selected-space
sample dispatcher and emits DeviceRGB. Cmyk Linear/Radial takes direct
DeviceCMYK shading with four-component functions and Range
`[0 1 0 1 0 1 0 1]`; do not restore the older CMYK→sRGB fallback. The original
seven-space helper rename oklab_sample_stops_*→multispace_sample_stops_* changed
no body, because sample(t) already dispatched by space. Historical claims about
unchanged baseline bytes/counts describe that source snapshot, not present
validation or a universal language byte-parity gate.

Conic's current retained strategy is unified Type 6 Coons for all eight process
spaces: RGB emit_conic_coons_stream_rgb with N=stops*4 angular patches, CMYK
emit_conic_coons_stream_cmyk with its retained one-patch-per-stop path. Decode
has five RGB pairs or six CMYK pairs, and an N=1 identity Type 2 function with
three or four components. Corner colors use Conic::sample(t) and the selected
space. The CMYK stream uses 41 bytes/patch versus 37 RGB: flag, twelve control
points with two coordinates, and four corner colors. Preserve the cubic-circle
arc offset `r*(4/3)*tan(angle/4)` and the infrastructure helpers
bezier_control_points_for_arc/compute_coons_patches_n_stops. Adaptive N does not
apply to the CMYK Coons one-patch-per-stop strategy. The reserved multispace
factor_delta recalibration remains a separate candidate, not an implemented
gain inferred from the current path.

The earlier Type 4 Gouraud strategy is superseded, not coactive: its historical
N=32 disk slices used ShadingType 4, DeviceRGB, 8-bit coordinates/components/flags,
Decode `[0 1 0 1 0 1 0 1 0 1]`, 96 vertices and 576 bytes at 32 triangles; center
was first-stop color and edge colors sample(i/N). It subsequently used adaptive
hybrid stops/contrast with factor_delta 256.0. That implementation's helper
emit_conic_gouraud_stream, compute_adaptive_n_conic and oklab_delta_e were removed
by the unified Coons package; twenty old tests were replaced by approximately
eighteen strategy tests. ADR-0090 Type4 guidance is revoked by that package.
The intermediate dual Type4-RGB/Type6-CMYK dispatcher, formerly default-OFF
Coons opt-in and its temporary allow(dead_code), are construction history,
not current flags, fallback policy or suppression authority. Earlier Type6
scope-out ended there; Type7 remains outside this slice. The original industry
research/family comparison and claims about upstream bug#4422 are retained as
historical evidence, not new measured oracle equivalence or ICC/PDF-A closure.

For Linear/Radial RGB/perceptual sampling, adaptive_n_for_stops replaces fixed
N 16 with threshold-based 16/32/64 from maximum pair Oklab delta-E, cap N_max 64
(4*N_base). Keep the private perceptual_distance_in_space helper and public
color_to_oklab_with_alpha conversion, without duplicating different constants.
Pastel low contrast retains 16; red→blue high contrast selects 64 deterministically.
Preserve same-input PDF reproducibility controls. Conic Coons and direct CMYK
remain separate strategies; the old adaptive Gouraud helper does not revive.

## Parent-relative coordinate and resource boundary

Parent transform is native Rust apply_parent_transform(local,parent_bbox),
following the measured correct_transform behavior; do not switch to PDF Matrix
as a surrogate. Self_ retains the established coordinate path. Parent without a
local box falls back to page_bbox at L3; None input previously produced identity.
The later real page fallback supersedes the earlier structurally unwired
helper/dead-code stage. Default Self_/None must add no parent-measurement work.

Layouter parent_bbox:Option<Rect> saves/restores parent context, and captured
FrameItem::Shape.parent_bbox_at_emit:Option<Rect> propagates it to GradientObject.
Only explicit width AND height populate Block/Boxed boxes; absent dimensions
retain page fallback, rather than eager measurement of every block. Boxed y
remains current regions.cursor_y, baseline-relative; exact inline-top y is an
open refinement. The container set subsequently extends from Block/Boxed to
Grid cell/Stack/Pad: Grid uses exact
`Rect { x: body_x, y: body_y, w: body_w, h: body_h }` after tracks/insets; Stack
measures vertical max_w*sum_h or horizontal sum_w*max_h with
measure_content_constrained before layout, locating bbox after flush_line.
Pad uses its INNER body region without insets, measured with available_inner.
Retain body_w>0 && body_h>0 guards and caller save/restore. The historically
accepted extra measurement is limited to relative=parent pipelines; its old
~1.5–2x cost is snapshot-specific, not a new benchmark or default overhead claim.

Post-layout FrameItem::Group parent boxes are L3-only. scan_all_gradients has
an internal recursive parent_bbox_override:Option<Rect>; the Group bbox is
Rect { pos, inner_width, inner_height } in native Y-down coordinates, without
Y inversion. Shape chooses inner-wins
`parent_bbox_at_emit.or(parent_bbox_override)`: an existing captured box from
Block/Boxed/Grid/Stack/Pad wins over Group override. Recursively register groups
in pattern_resources_for_page as well; otherwise a traversed gradient might
lack its actual resource. draw_item_local also recurses nested Groups, rather
than silently dropping them, and consumes the registered pattern instead of
s.paint.to_color() fallback. Thread its three corresponding inputs
parent_bbox_override,pat_ptr_to_idx,pat_refs; construct equivalent Group boxes
in scan/resource/draw so dedup_key_for finds the identical resource. Preserve
solid Group and top-level Shape behavior. Extracting the three repeated Group
bbox constructors is an unreserved helper candidate, not authorization for
another owner or new public API.

Resource deduplication is document-global and uses
`DedupKey { arc_ptr: usize, bbox: Option<RectKey> }`, where RectKey has four i32
millipoint coordinates. Compute effective bbox identically during scan and
resource registration, and pass it into emit_stroke_paint lookup. Self_/None
uses bbox=None and one key per Arc. A shared Arc in N distinct effective parent
boxes legitimately yields N patterns; one-context/shared-default cases keep
one resource. This supersedes historical first-Arc-context-wins behavior;
quantization avoids f64 HashMap keys while preserving subtypographic precision.
Increased worst-case PDF size is the accepted semantic tradeoff, not a gain.

## Explicitly open scope and retained negative decisions

CMYK ICC and PDF/A compliance remain open. The original NO-GO investigation
found three blockers: an external PDF crate requires a new architectural
exception to export's no-external-PDF-crate rule; hardcoded profile bytes lacked
an identified royalty-free recognized redistributable CMYK profile under the
then-investigated Adobe/ECI/IDEAlliance/ICC Tech Note 7 evidence; preserving
DeviceCMYK without profile remained the bounded path. This is a retained
historical investigation, not a fresh legal/licensing claim. The three external
prerequisites remain in archived typst-passo-273-14-trabalho-previo-externo.md.
No automatic profile synthesis, crate adoption or PDF/A completion is authorized.

Measured post-layout boxes for dimensionless containers remain open. The NO-GO
record found no empirical demand over its eight reviewed substeps; eager
measure_content_constrained on every undimensioned Block risked O(N²) instead
of O(N), while a new lazy walker of roughly 60–100 lines was disproportionate to
that evidence. Preserve explicit-dimension/page fallback. The two prerequisites
are a concrete empirical case and an executive performance-cost decision, in
archived typst-passo-273-15-trabalho-previo-externo.md. Lack of historical demand
is not proof that a future use case cannot exist.

Exact top y for inline Boxed also remains open. Its NO-GO record found no demand
over nine reviewed substeps; a broad inline line_height refactor was outside
that gradient slice and the ADR-0078 multiregion phase, and ad hoc font_metrics
would create unmeasured debt. Preserve the accepted baseline-relative y and
its conscious inline-height limitation (historical P156H); later exact-top work
needs its three recorded prerequisites. DEBT-56 was already closed by P221 on
2026-05-12, confirmed by DEBT.md:535, so it is not the blocker. Updating the stale
Content reference was an unreserved candidate. The actual blockers were that
inline-height restriction and the deferred multiregion phase, not the old debt.

Historical completion of the gradient series and its meta-ADRs does not close
these three limits. ADR-0095/0096/0097 formalized resource dedup, optional parent
context with pending consumers, and evidence-based NO-GO scope maintenance;
other emerging helper/duplication/layout-cost patterns were deliberately not
formalized. Methodology counters, maximal-completion rhetoric, old test totals,
unchanged-byte receipts and administrative-only milestones remain in the
archive and authorize no new productive code here. Normal/no-focal/Oklab/Self_
controls retain their expected behavior through the respective later changes.

## Acceptance and change limits

GradientStop::new retains Some(offset), unspaced retains None, linear constructs
the correct Arc variant, and first_stop_color returns its first color. Verify
uniform effective spacing; first/last sampling endpoints after the documented
Oklab roundtrip; Arc clone behavior and derived PartialEq. Preserve radial focal
validation/default tests, all three variant/space controls, hue shorter-path,
precise canonical/f32 boundary compatibility, repeat/accessor Ratios, Radial
precision components/alpha, spot identity/tint/errors and anti_alias state.
Do not substitute mechanics tests for observable public components.

Color bounds/precision maintenance follows ADR-0127 continuous flow: no public
field/signature/binding/default-mode/compatibility or phase change. L0 precedes
functional RED, implementation and revalidation; remaining differences stay
visible. The facade receives only its prescribed lineage update for child
conversion work, preserving sampling/offset APIs and 1:1 ownership. Builds,
local/L1 regression suites, existing integrations and crystalline-lint must
validate the actual materialization; source inspection or an old completion
claim is not a fresh measurement. Export tests, parent scopes, resource lookup
and outstanding ICC/box decisions remain acceptance boundaries of their owners,
not extra productive responsibilities of this facade.

## Historical record

The complete original, including all measurements, explicit intermediate
strategies, administrative revisions, methodology counts and unresolved
prerequisite references, is preserved at
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/gradient.md`.
Construction records remain under
`lab/history/construction/records/00_nucleo/materialization/`; identifiers
P262–P274/P273.5–P273.17, P1145/P1253/P1269/P1271/P1482/P1483 identify archived
provenance and bounded supersessions, not an active execution-step hierarchy.
