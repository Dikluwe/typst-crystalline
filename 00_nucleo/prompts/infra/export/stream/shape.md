# L0 — stream/shape
Hash do Código: efd113ae

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/pdf-conic-placement.toml sha256:3d2d44a53e9fb6a6c8d59ddee7b362412ae0ab05fac0e37e50ddf731d987c132
- 00_nucleo/prompts/_nuclei/pdf-group-local-space.toml sha256:94c2f27db514ce909100e929a459524e08ebeb09583f4a39173f53f457c770cd
- 00_nucleo/prompts/_nuclei/pdf-solid-opacity.toml sha256:afd5a809b6e3f2ac1f9a752fb1f6c6d704139eb1f71b085d430d9417f826b04e
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff


**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/stream/shape.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/stream/shape.rs:18` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Paint and geometry primitives

Own process/spot paint operators, paths, rounded rectangles, fill/stroke and line
caps together. Consume ShapeKind<Pt>/Corners<Pt> resolved in L1; clamp radii to the
available geometry without resolving Length or discarding relative context.
Preserve path segments, curves, ellipse and fill rules. Butt omits 0 J, Round emits
1 J and Square 2 J; a missing line color retains the empty prefix. Isolate state
with the existing graphics envelopes.

Process colors preserve rg/RG. Spot references use complete colorant identity,
deterministic resource names and the existing u8-quantized tint. Gradient paint
finds the prepared Pattern using its DedupKey/bounding box; spot gradients use the
shape box consistently with resource collection. Preserve linear/radial process-gradient keys,
tiling and missing-resource fallbacks. Do not create resources, choose subsetting
or compress streams.

## Verification

Check four resolved corners, paths/fill rules, cap variants, spot tint/resource
closure and gradient bounding-box lookup in top and local painting.

## Local Y-down clip paths

emit_shape_path_local at shape.rs:215–259 writes Rect y=-height, flips PathItem y and ignores Ellipse/Line although both callers' group cm already establishes Y-down. nested_group_clip_has_transformed_bounds_and_does_not_leak got page y177..169 rather than169..161. `lab/results/construction/diagnostics/grupos-aninhados-2026-10-13/clip-before.json.gz` records candidate/failure/source/L0, with base HEAD/status/diffstat/UTC in before.json.gz. This is an unwanted second semantic reflection, not PDF coding freedom.

Emit local Y-down paths: Rect[0,width]×[0,height]; Path preserves author point/control y. RoundedRect reuses existing numeric formatter at(0,0) with top/bottom radii permuted because its own y grows upward; physical corner identities stay. Ellipse fills the local box using four cubic arcs/existing kappa; Line preserves dx/dy signs. Match known geometry exhaustively, no empty fallback. Caller owns W n/graphics envelope; helper neither changes CTM nor paints. Do not change emit_rounded_rect_ops or other painted shapes. Internal continuous flow, no public API/phase/default/strategy change. RED tests cover local Path coordinates, RoundedRect corner identity/Ellipse extrema; rerun unchanged nested clip and infra suite, raster source→PDF with failure evidence, distinguish still-open gradient defects. Shared coordinates/graphics scope belong to pinned pdf-group-local-space.

## Conic occurrence lookup

mod.rs:224–264 loses Conic RGB/CMYK position/deduplicates Arc+bbox; builder/resources.rs:564–574 emits identity, draw.rs:522–555,939–965 composes groups but Pattern is initial-stream-relative, page.rs:45–49 adds bleed. Constant-red versus conic(red,red) before/probe reproduces coverage loss entirely within Tekt. `lab/results/construction/diagnostics/conic-posicionamento-2026-10-14/placement-before.json.gz` preserves source/L0/HEAD/UTC/status/diffstat.

Pass dispatcher Shape occurrence to private fill/stroke helpers and resolve the collector key; legacy bbox remains for other gradients. Helpers receive existing PageContext instead of separate map/reference parameters, transporting identical borrows without changing fields/constructor. No resource creation/path/solid/tiling/fallback change. L0 first/scoped lineage/RED→GREEN tests for position/size/same Arc across occurrences/pages/fill/stroke/nested affine groups/bleed/explicit parent/local/page/resources/Compact/Verbose; reraster without cropping/realignment. No external public API/flag/default/phase. Positioning alone does not certify full conic functionality: author center/corner coverage/CMYK sampling-offset completion belongs to conic-geometria-amostragem; retaining the mesh here isolates cause and never legitimizes an inscribed circle as final output.

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

Consume FramePaint; emit Pattern cs/scn or CS/SCN for registered Tiling fill/stroke, never a solid fallback. Keep Solid/Gradient, geometry and thickness. Missing registered prepared cells are explicit internal errors, never representative colors.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.

## Solid opacity roles

`lab/results/construction/diagnostics/tiling-overlap-2026-10-14/before.json.gz` preserves source/owner/HEAD/UTC/tree. tiling-nativo-2026-10-14/final-renders and remaining-analysis.md show cell8×8/pitch4×4/alpha50%: PDF opaque red, SVG one contribution, PNG four(255,15,15). pdf_tiling.rs:184 uses BBox=size, svg/paints.rs:148 overflow visible, shape.rs:155/text.rs:275 drop alpha. Upstream also clips overlap and is not a requirement for that defect. The explicit-period source-over hypothesis must preserve no-overlap controls and analytical four-contribution composition.

Own private per-FrameItem-leaf solid opacity helpers returning(fill,stroke), with deterministic f32-bit names, no u8 quantization. Shape considers Solid only per role; text/glyph/separator use color alpha for both including faux-bold; Line stroke only; other leaves/containers opaque. Gradient/Tiling never inherit representative-color alpha. Emit ExtGState only when some alpha differs from1. Helpers create no objects/resources. This is internal within the approved tiling package, no new public API/phase/flag/default. L0 first/RED→GREEN, analytical composition including distinct-color order, simple/nested/rotated/text/image and extraction/page-selection controls; workspace/build/fmt/Rustdoc/lineage. Variable-gradient alpha/PNG quality remain incomplete, so no production certification with these open.
