# L0 — infra/export/render
Hash do Código: 609c8b98

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/render.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/render.rs:40` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Raster contract

Own RenderOptions, FontKey, raster entrypoints, rendering state, recursive item
dispatch, glyph/image/shape rasterization and PNG encoding. Inputs are resolved Page
or PagedDocument values and caller-supplied fonts; outputs are in-memory PNG bytes.
Do not read files, fetch fonts, interpret CLI/PPI or run layout.

RenderOptions defaults to pixel_per_pt=2.0 and render_bleed=false. The scale is
pixels per typographic point, not PPI; the caller resolves physical units.
Use max(pixel_per_pt, 0.01), round each scaled dimension and require at least one
pixel. Without bleed use the logical page; with bleed use the physical canvas and
translate by bleed.left/top once. Auto fill is white, None retains transparency,
and Paint uses the existing to_color projection. Paint background, body and foreground
in order. The current allocation path may panic with "tamanho validado acima";
no independent maximum allocation bound is implemented here.

TextShaped renders only with a matching FontList/FontVariant/FontVariations and a
valid supplied face. Preserve glyph advances, offsets, position, scale and alpha;
the current pixglyph path uses its existing raster/outline support. Plain Text and
direct Glyph are omitted by dispatch, even with fonts; do not promise tofu or
complete mathematical glyph coverage.

Line maps Butt/Round/Square to the corresponding tiny-skia cap without changing
thickness, color, transforms or coordinates. Shape consumes resolved ShapeKind<Pt>
and all four Corners<Pt> directly, never resolving Length in a neutral context.
Preserve paths, ellipse/rounded geometry and current solid Paint projection.
The shape path uses Winding fill and its existing Butt/Miter stroke settings;
line-cap support does not imply equivalent Shape stroke or fill-rule support.

Images use existing decoding/texture preparation and EXIF composition on the
resolved box. Groups compose position and matrix and recurse, but currently do
not apply clip_mask. Image clip_rect is not consumed by this dispatcher.
Link and Semantic are visually transparent containers: recurse without painting
target, kind, placement or alt. No PDF tagged structure or accessible text layer
is produced by raster.

render_document_to_png uses the existing fontless per-page route, stacks successfully
decoded pages vertically with the rounded gap and white composite canvas, and keeps
its 1x1 allocation fallback for an empty/invalid composite. Preserve PNG encoding
and failure behavior; do not claim compressed PNG bytes as language parity.

## Verification

Check IHDR dimensions, bleed/fill/layers, supported shaped text, image orientation,
resolved radii, and deterministic line endpoint coverage. Compare visible geometry
and color/alpha, not PNG compression bytes. Unsupported clips/direct glyphs/complex
paint remain recorded limitations, not validated equivalence.
