# L0 — infra/export/gradients/conic
Hash do Código: 4e21cba4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/pdf-conic-mesh.toml sha256:27b85cbce9a73d495cf7b2d9fe13c4ce3f8cba9e555970b86c9f919ae02db232
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/conic.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/conic.rs:17` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## PDF conic Coons mesh

Own conic RGB sampling, circular-arc control points, patch-count helpers and
binary Type 6 Coons mesh emitters for RGB, CMYK and spot. Preserve this mesh
strategy without claiming upstream tessellation identity.

The production mesh follows the geometry and sampling refinement below. Historical
patch-count helpers remain test-only evidence of the former policy; fixed counts,
8-bit coordinates and a circle inscribed in the shape are superseded. Preserve
Type 6 Coons patches, not the defective geometry or stop-to-patch association.
The builder owns resource dictionaries and physical placement; this owner emits
payload plus matching coordinate Decode bounds.

## Verification

Check geometric continuity, angular direction, process/CMYK/spot component widths
and valid Type 6 decoding. Binary encoding validity is a format contract; patch
counts and equality to an upstream algorithm are not language parity.

## Numeric and geometric evidence

conic.rs:58,60,61,169,170,278,279 contains selected numeric methods. `lab/results/construction/diagnostics/gradientes-numericos-ci-2026-10-13/before.json.gz`
retains library/consumer/owner/manifest/lock/spans/HEAD/dirty tree/UTC/status/diffstat.
clippy.toml requires platform-independent math; libm =0.2.11 is already locked.
before-renders.json has 24 sources rendered PDF/SVG by Tekt and historical vanilla
9f2b6e8715237cb086899a42873660fe744622e8 with binary hashes/commands;
before-comparison.json observes full canvas without error-adjusted masks.
Pre-existing divergence is not complete parity. Rejected initial hsv-outside-color
fixture stays in initial-fixture-error.log; cases.json has valid inputs.


conic.rs:142–143,241–242 fixes center/radius; :271–272 circularly assigns CMYK stops
by index ignoring offsets/final sample; resources.rs:508–552 uses eight-bit
coordinates/unit Decode. conic-posicionamento-2026-10-14 before/after/refuters
show white corners/ignored center/ineffective CMYK offsets despite correct sample().
conic-geometria-amostragem-2026-10-14/before.json.gz retains sources/L0/HEAD/tree/
diffstat/UTC/binaries. Bilateral corpus observes center/colors along axes/diagonal,
square/rectangle/rotation. Pinned PDF paint.rs:274–299 plus pixels start leftward
and proceed clockwise, unlike old Tekt; observed vanilla is no uniform fallback.
PDF Reference1.3 table4.31 PDF page250 permits sixteen-bit coordinates/independent
axis Decode, a format contract not upstream byte equality.

render-controls/results.json independent qpdf reconstruction preserves roundtrip
pixels; AntiAlias does not change cases. Center-exact Decode reduces angle-case
pixels differing >16 levels from107→15; center-quantization-before.json.gz retains
prior source/state. Symmetric bounds over65, 536 codes place center between codes,
rounding shifts physical center: avoidable quantization, no intended new center.
first-validation/followup-architecture.log observes new V16 at conic.rs:223:
`_ => 3` hides future color spaces.

## Current Type6 mesh contract

The earlier f32/libm correction used tanf arc offset/sincosf pairs with unchanged
angular difference/multiplication/center/radius/progression/sampling/quantization;
spot reused geometry. The later approved geometry refinement below supersedes
only those old precision/geometric mechanics requiring it, preserving Type6 strategy.
No obsolete productive emitter stays parallel.

Emit Type6 Coons with sixteen-bit coordinates and matching Decode bounds. Private
domain supplies physical reference size/normalized canvas bounds. Convert center
Y-down→Y-up; theta=pi-angle-2*pi*t for t in[0,1], normalize angle before trig.
Scale directions by width/height to preserve physical angle. Radius covers maximum
physical center→domain-corner distance plus0.1% quantization margin; Decode radius
expands1% to contain controls. Arc span≤1/128 turn, f64/libm coordinates, existing
cubic r*(4/3)*tan(delta/4). Final seam reuses initial direction exactly but final
color survives. No unit-box center/point clamp; encoded values stay within Decode.

Use effective_offsets, declared order/auto spacing/constant before-after extension/
duplicate-stop one-sided limits. Never interpolate discontinuities or force final
color back to first. Each interval calls Conic::sample on two reparameterized stops,
keeping L1 declared-space interpolator, no duplicate color conversion. Besides
angle cap subdivide when1/4, 1/2, 3/4 emitted-component samples differ>1/255 from
linear mix, max depth 8, a work cap not universal error proof. RGB/CMYK/spot share
geometry/intervals; native CMYK four components, spot one tint; colorants/Separation/
general-alpha handling unchanged. Zero stops→empty stream, one→constant; singular/
nonfinite domains uncertified. Old count/sample/unit-payload wrappers may be
test-only and delegate when applicable, never another productive policy.

Represent center at code32768: lower center-R, upper center+R*(32767/32768),
R=radius*1.01. Expansion still covers controls. Require decoded-center RED→GREEN/
full raster recollection. Keep seam/edge renderer differences recorded; never copy
vanilla's observed top white line. Explicit component dispatch retains Spot1/Cmyk4,
names Oklab/Oklch/Srgb/Luma/LinearRgb/Hsl/Hsv as RGB3, no wildcard/future-space
mask. Sampling/bytes/public behavior stays except measured geometry refinement.

## Acceptance and continuity

Internal formula/parity continuous ADR-0127 adds no public/external/default mode/
flag/phase. L0 before real RED/code. Preserve functional expectations; replace only
obsolete mechanical counts/eight-bit/inscribed-circle assertions with independent
geometry/sample/decoded-center evidence. Full corpus/qpdf/raster/regressions/
lineage+reverse/all-target checks. Deterministic snapshot regeneration requires
intentional-change proof and current corpus protocol, never green-by-blessing.
Internal continuous ADR-0127 adds no public/default/phase/strategy; L0 first,
involved-owner seals. Direct L3 libm =0.2.11 reuses locked package, no upgrades/I/O.
Existing formula tests and same 24 before/after PDF/SVG sources, raster/structure
observations separate local preservation from historical oracle differences.
Analyze any new render difference; no larger tolerance/limit/fixture changes or
suppressions to hide failure. All-target compile/infra suite/build/Clippy/Rustdoc/
fmt/lineage; counts/file identity certify no language parity.

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

Migrate FrameItem::Shape fixtures to FramePaint/FrameStroke with unchanged values/
semantic assertions. Tiling explicitly constructs a prepared cell; eval fixtures
remain declarative. No total productive conversion or weakened assertion.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.
