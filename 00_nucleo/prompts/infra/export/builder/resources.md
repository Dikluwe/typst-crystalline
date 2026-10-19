# L0 — builder/resources
Hash do Código: 0f11047f

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:88391e330334a2221170109c82b5dbf1fb60c4d2f6e2660e732df24b67554d9f
- 00_nucleo/prompts/_nuclei/pdf-conic-mesh.toml sha256:27b85cbce9a73d495cf7b2d9fe13c4ce3f8cba9e555970b86c9f919ae02db232
- 00_nucleo/prompts/_nuclei/pdf-conic-placement.toml sha256:3d2d44a53e9fb6a6c8d59ddee7b362412ae0ab05fac0e37e50ddf731d987c132
- 00_nucleo/prompts/_nuclei/pdf-solid-opacity.toml sha256:afd5a809b6e3f2ac1f9a752fb1f6c6d704139eb1f71b085d430d9417f826b04e
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder/resources.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/builder/resources.rs:46` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Resource aggregation

Own page ColorSpace entries, spot-colorant collection and Separation dictionaries,
gradient-object emission and image-XObject aggregation. Consume image and gradient
payloads from their specialized owners; do not redefine decoding, ICC profile
selection, gradient sampling or conic mathematics here.

Verbose resources include /c0 referencing the shared sRGB ICC profile, even when
there is no RGB JPEG. Compact retains its existing resource policy. An RGB JPEG
requires the shared profile; grayscale and CMYK keep their own image paths.
Collect selected-page spot colorants in encounter order across background, body,
foreground and nested content, including text, line, shape fill/stroke and gradient
stops. Identity is the complete colorant, not only its name. Emit deterministic
resource names by delegating complete-colorant naming to
`00_nucleo/prompts/infra/export/stream/shape.md`, and correctly escaped PDF names,
including None and All. Separation
alternate spaces and Type2 tint functions preserve the declared fallback: RGB uses
the shared ICC when available, and CMYK starts at zero ink. Do not replace /c0.

Emit supplied gradient and image objects at their allocated IDs with resolvable
references. Preserve spot-gradient Separation resources and the existing axial
Extend and stop-function routing without taking ownership of the gradient model.
Merge prepared bitmap /ImN entries into page /XObject dictionaries alongside normal
images; preserve both when the page uses both. Excluded pages must not introduce
page-local spot inks into an included page's resource dictionary.

## Verification

Check RGB JPEG, grayscale/CMYK, verbose/compact, repeated and same-name distinct
spot colorants, gradient roles and mixed bitmap/image resource closure.

## Separator resource collection

Source→PDF RED/handler audit in fatia-funcional-tekt-2026-09-28 precedes received
layout_types public approval on2026-09-28 with recorded provenance. Collect font
style from each separator, including Group/Link. Existing language text/geometry/
adjacency/pinned invariants stay; no shared productive ownership.

## Conic placement and mesh resources

conic-posicionamento-2026-10-14/placement-before.json.gz retains source/L0/HEAD/
UTC/status/diffstat; mod.rs:224–264 drops occurrence position, resources.rs:564–574
uses identity, draw Group transforms/page bleed reveal initial-stream coordinates.
Tekt-only red versus conic(red,red) controls refute correct placement without
copying vanilla. Emit six prepared matrix components in every conic color space,
keeping IDs/Type6/Decode/payload/colors and prior decimal spelling for diagonal
spot patterns. No first-page position inference/geometry resampling.

conic-geometria-amostragem-2026-10-14/before.json.gz retains source/L0/HEAD/tree/
diffstat/UTC/binaries; fixed centers/radii, circular CMYK index stops and eight-bit
unit Decode fail measured corners/center/offset controls. Current mesh emitter
`00_nucleo/prompts/infra/export/gradients/conic.md` owns geometry/sampling. Consume
one prepared-domain emitter, write BitsPerCoordinate16 and Decode four geometric
bounds plus correct component pairs (RGB3/CMYK4/spot1). Keep Type6/IDs/native
colors/Separation/dummy Function/validated physical matrix, never assume unit Decode
or resample. Internal continuous ADR-0127: no external/public/default/phase change;
L0 first/RED→GREEN/scoped seals, occurrence/page/affine/bleed/parent/fill-stroke/
Compact-Verbose/full unmasked raster controls and conic full corpus/qpdf/regressions/
lineage/reverse. Positioning alone does not certify author center/CMYK/coverage;
current geometry contract completes only its measured slice, not universal domains.

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

Collect spot colorants in prepared paints/nested cells while preserving declarative
PageFill and current gradient emission. RGB JPEG detection also visits prepared
cells/background/foreground to reserve Compact ICC. Semantic visitor still drives
spot collection with explicit paint descent, no duplicate traversal.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

## Solid opacity aggregation

`lab/results/construction/diagnostics/tiling-overlap-2026-10-14/before.json.gz`
retains sources/owners/HEAD/UTC/tree. Native tiling final-renders/remaining-analysis
observes8×8cell/4×4pitch/50%alpha: opaque-red PDF/one SVG/four PNG(255,15,15).
BBox-size/visible-overflow/ignored-alpha sites reveal observable defects; vanilla
also clips overlap, no requirement to copy. Explicit period/analytic source-over
must preserve nonoverlap controls, including distinct-color order.

Collect leaf opacity pairs via the single stream helper and visual-resource walker
including cells; include Solid canvas. Emit direct ExtGState /ca /CA, deterministic
ordered dedup, return through same private aggregate for all three builders/cell
Forms. External API/ICC/spot unchanged. Internal ADR-0127 within approved native
package, no API/phase/flag/default; L0 first/RED→GREEN/analytic/simple-nested-rotated-
text-image/page-selection controls, build/workspace suite/fmt/Rustdoc/lineage.
Variable-gradient alpha/PNG quality stay incomplete; no production certificate
while open.
