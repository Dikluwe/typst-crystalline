# L0 — infra/export/render
Hash do Código: 549447a7

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/text-replacement.toml sha256:08c39c6b9cf6fdc571564646d12d71f9b12f2a5d2c029ca90f9253c39c28bb95
- 00_nucleo/prompts/_nuclei/layout/text-separators.toml sha256:79d5c48d0b9d0fb7d995207d18b1f9437209bf35c78a35cbfcedccea8b90abd2

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

## Separadores textuais — materialização autorizada em 2026-09-28

### Medição anterior à decisão

`03_infra/src/export/render.rs` é um consumer do percurso FrameItem. As sondas RED fonte→PDF de
`00_nucleo/diagnosticos/fatia-funcional-tekt-2026-09-28/` e a inspeção
dos handlers desse consumer precedem esta alteração. A aprovação do contrato
público em `00_nucleo/prompts/entities/layout_types.md` foi recebida em
2026-09-28 e registrada na proveniência da materialização.

### Responsabilidade deste owner

TextSeparator não pinta pixels; advance não tem tinta.

Conservar as responsabilidades existentes; a aceitação segue texto e geometria
observáveis, incluindo os controles de adjacência e os invariantes pinados.

## Identidade textual — materialização aprovada

### Medição anterior à decisão

As sondas bilaterais e o estado exato em
`00_nucleo/diagnosticos/identidade-hifen-discricionario-2026-09-28/initial.json`
e `before/` medem U+002D versus SHY com geometria coincidente em oito controles.
A inspeção do percurso FrameItem deste consumer está arquivada em approved-before/.
O dono confirmou o contrato em `00_nucleo/prompts/entities/layout_types.md`.

### Responsabilidade exclusiva

Tratar TextReplacement como envelope visual transparente nos percursos deste owner; descender nos filhos preservando text, ordem, coordenadas e o cálculo existente, sem novo algoritmo ou avanço.

O Núcleo text-replacement define apenas invariantes compartilhados; este prompt
possui somente o consumer declarado. Aceitação: texto/caixas, testes causais,
transporte recursivo e controle de hífen literal; não bytes PDF ou nova seleção.
