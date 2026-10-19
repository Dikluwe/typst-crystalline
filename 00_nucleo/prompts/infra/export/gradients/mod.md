# L0 — infra/export/gradients/mod
Hash do Código: 6b373abf

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:395a968c2fe49429b03b369cd5047df513132bfef6511ec21af466670ec2e147
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/pdf-conic-mesh.toml sha256:27b85cbce9a73d495cf7b2d9fe13c4ce3f8cba9e555970b86c9f919ae02db232
- 00_nucleo/prompts/_nuclei/pdf-conic-placement.toml sha256:3d2d44a53e9fb6a6c8d59ddee7b362412ae0ab05fac0e37e50ddf731d987c132
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/gradients/mod.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/gradients/mod.rs:54` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Gradient discovery façade

Own PatternRef, GradientObjectKind/GradientObject, RectKey/DedupKey, bbox/key helpers,
scan_all_gradients, pattern_resources_for_page and restricted child reexports.
Keep private static child modules; neither formatting functions nor sampling/
mesh algorithms are duplicated here.

Traverse page background, body and foreground recursively through Group/Link/
Semantic. Register Shape.fill and Shape.stroke Gradient paints in encounter order
with the same effective key; reuse matching fill/stroke identity once.
DedupKey combines gradient Arc pointer with effective bbox quantized by the existing
RectKey representation. Group passes its local bbox context; the shape's own
parent_bbox_at_emit wins over the inherited override.

Spot gradients additionally retain the painted shape rect for placement and lookup,
separate from a container's relative-parent bbox. The same spot gradient reused
at different shape locations must retain those placements; linear/radial process keys preserve
their existing policy; conic placement follows the refinement below. Keep sequential allocated pattern/shading/function IDs and
recursive page resource lookup synchronized with discovery, without collisions
or unresolved references. Do not include nonvisual alt as paint.

## Delegated owners

- `00_nucleo/prompts/infra/export/gradients/linear.md`: axial coordinates/RGB sampling.
- `00_nucleo/prompts/infra/export/gradients/radial.md`: focal circles/RGB sampling.
- `00_nucleo/prompts/infra/export/gradients/cmyk.md`: CMYK sampling/conversion.
- `00_nucleo/prompts/infra/export/gradients/relative.md`: relative tuple transforms.
- `00_nucleo/prompts/infra/export/gradients/adaptive.md`: PDF counts/SVG adaptive stops.
- `00_nucleo/prompts/infra/export/gradients/conic.md`: Type 6 mesh payloads.
- `00_nucleo/prompts/infra/export/gradients/function_dict.md`: interpolation functions.

`00_nucleo/prompts/infra/export/builder/resources.md` owns emitted resource objects
and shading placement; `00_nucleo/prompts/infra/export/stream/shape.md` owns paint
operators. This façade performs no I/O or entity interpolation.

## Verification

Check recursive three-layer discovery, fill/stroke reuse, distinct effective boxes,
spot placements and page-local reference closure. Pointer/key mechanics preserve
the current implementation; they do not define language equality.

## Carrier and export-discovery boundaries

Source→PDF RED/handler audit fatia-funcional-tekt-2026-09-28 preceded layout_types
public approval on2026-09-28. TextSeparator has no paint/gradients. Approved
identidade-hifen-discricionario-2026-09-28 initial/before eight U+002D/SHY equal-
bbox controls/audit preserve transparent TextReplacement traversal through children
with unchanged author text/order/coordinates/calculation; no new advance/algorithm.
Text/bbox/causal/recursive/literal-hyphen controls, not PDF bytes/new selection.

infra-imports-docs-ci-2026-10-13/before.json.gz retains sources/owners/spans/original
compiler output/HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/dirty tree/UTC/status/
diffstat at mod.rs:18,20,184,351. Remove only proven unused private imports, keep
public reexports/required productive and descendant-test imports. Compilation
revealed perceptual_distance_in_space/rgb_to_cmyk/bezier_control_points_for_arc/
compute_coons_patches_n_stops/compute_coons_patches_n_stops_extended/
multispace_sample_stops_conic reexports are test-only: cfg(test) only aliases,
productive definitions/internal calls stay. Conic uses cmyk::rgb_to_cmyk, not
facade alias. No signature/default/phase/formula/acceptance/suppression changes;
all-target compile/infra names+suite/build/workspace Clippy/Rustdoc/lineage/scoped
seals, no import/comment mirror tests.

## Conic occurrence geometry

### Evidence

mod.rs:224–264 loses conic RGB/CMYK position and dedups Arc/bbox; resources.rs:564–574
emits identity matrix. draw.rs:522–555,939–965 composes groups but Pattern refers
to initial stream space; page.rs:45–49 adds bleed. before/probe.json.gz red versus
conic(red,red) reproduces coverage loss entirely within Tekt. conic-posicionamento-
2026-10-14/placement-before.json.gz retains source/L0/HEAD/UTC/status/diffstat.
Later conic-geometria-amostragem before.json.gz/source probes show fixed center/
radius/ignored CMYK offsets/eight-bit unit Decode; current detailed mesh contract is
`00_nucleo/prompts/infra/export/gradients/conic.md`. Positioning alone left author
center/corners/CMYK-offset completion explicitly separate; later geometry contract
supersedes only that incomplete-stage wording, no universal conic certification.

### Current facade contract

Capture conic matrix using existing TransformMatrix. Conic key includes borrowed
FrameItem occurrence identity plus Arc without position quantization/page collision;
all builders traverse same borrowed PagedDocument without cloning between discovery/
paint. Export-local mechanical key, never persistent or language equality. Same
key for page resources. Other gradients keep old policies. Private context carries
matrix-to-PDF/local parent box separately from legacy bbox override.

Transport physical conic reference size and canvas bounds mapped through inverse
matrix into pattern unit coordinates, include unit box to cover stroke/path outside
nominal bbox without guessed miter/path expansion. Preserve canvas through Group/
bleed. Singular/nonfinite inverse keeps defensive unit domain without certifying
coverage. Reexport only required private types/helpers; obsolete wrappers test-only.
Document occurrence-conic versus legacy linear/radial dedup accurately.

Internal ADR-0127 no external entity/signature/flag/default/phase. L0 first/real
RED/code/scoped seals; position/dimensions/same Arc across occurrence+page/fill-
stroke/nested affine Group/bleed/explicit-local-page parent/resources/Compact-
Verbose. Repeat full raster without cropping/realignment.

## Approved paint-root transport

### Evidence and scope

Manifesto `d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d`
was approved for the native-tiling package; receipt
`lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Its bilateral Parent/Solid evidence cannot be replaced by structural inspection.
`lab/results/construction/diagnostics/paint-root-2026-10-14/before/results.json.gz`
retains sources/dirty tree/HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, UTC/
diffstat/binary hashes for containers/Place/Rotate with Solid/Gradient Parent.
This unit's before-source/syntax-nodes anchors: [668]. layout_types.rs:531–539
lacks technical-root distinction; helpers.rs:253–269 reconstructs Group. Box groups
background/body; Block stays flat except clipping. Reference-frame preservation is
language, envelope representation mechanics; prior general Rotate/Box geometry
divergences remain open.

### Contract and acceptance

Adapt Group construction/patterns to approved paint_root. New technical/legacy
fixture groups are false; reconstruction preserves received value, never inferring
root from dimensions/clipping/transforms. Keep other fields/drawing/resources.
Metadata migration alone does not certify exporter pattern integration or replace
Shape.fill/stroke with FramePaint. Independently authorized prepared-transport/
native-target/parity slices (historical T1c/T2/T3) stay incomplete until integrated
verification, no production/parity claim from a successful build. L0 first,
RED→GREEN/scoped resealing; preserve source→PDF/SVG text/geometry and revalidate
suite/build/doc/check/fmt/lineage.

paint-root-2026-10-14/first-render-delta.json/after-first/results.json.gz records
open block-auto-gradient-parent/PDF regression:3, 840 pixels differ>16 and oracle
distance increases; 35 other Tekt renders stable. Old Group parent_bbox override is
not affine root context; then-declarative Shape fields expose it. This is an open
refuter, not accepted reference/tolerance relaxation. Prepared paint/resource
integration must close cause before production certification. Retain both negative
sample and agreeing controls, including vanilla PDF quarter divergence.

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

Consume FramePaint when scanning Shape. Register gradients inside cells using
cell-local canvas/root/dimensions and borrowed-frame occurrence identity for conic.
Recursive resources include nested gradients without changing non-tiling rules.
PDF-cell gradients use local Y-down/to_pdf identity because PatternType1's matrix
performs outer inversion.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

## Local discovery collector

too-many-arguments-2026-10-15/before.json.gz/classification.json retains full
source/prior L0/signatures/hashes/UTC/status/diffstat/HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b/
dirty tree2026-10-15T16:39:18.873477+00:00; walk :320 has nine args/threshold7,
historical C045. Same map/two outputs/two counters are shared discovery state,
not per-branch geometry. clippy-aridade-interna-2026-10-16/before.json.gz retains
source/callers/current clauses/spans/28-warning inventory/global hashes/HEAD/tree/
UTC/diffstat. Named correction materializes earlier audit-only scope, no new parity/
performance claim.

scan_all_gradients local Collector owns ptr_to_idx/refs/grad_objs/next_id/counter;
recursive walk takes items, parent_bbox_override, page_height, placement, &mut Collector.
Initialize next_id=first_id/counter=1; retain per-Function/Shading/Pattern increments,
background/body/foreground/Shape fill-stroke/cell order and move same outputs to
return. Do not change pattern_resources_for_page second walk/dedup/position/conic
bounds/exhaustive match/IDs/geometry/Arc ownership/limits.

Continuous ADR-0127 no externally public/default/phase/dependency/semantics change;
new types/constructors private, six-case materialization only, conditional C047/
C066 residual exceptions only after _text removal. Other five correction arities
visible; no module/impl/crate suppression/higher threshold. L0 first/scoped seals/
public signature+field/value transport unchanged, PNG/SVG/PDF/preference/diagnostic/
resource/MCID controls, no global mutation, no relaxed fixtures/expectations/private-
carrier mirror tests. Full workspace suite/build/all-target Clippy/unfulfilled
expectations/Rustdoc/fmt/strict-reverse lineage under12GiB/zero swap/one job.
