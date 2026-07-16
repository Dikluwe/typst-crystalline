# Prompt L0 — `infra/export/images` — Imagens PDF
Hash do Código: 85429e58

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/images.rs`
**Criado em**: 2026-05-19 (P307c)
**ADRs**: ADR-0029 (image Arc dedup), ADR-0095 (dedup via Arc::as_ptr)

---

## Contexto

Cluster completo para emit de imagens em PDF:
- Detecção de formato (JPEG via `FFD8FF`, PNG via 8-byte magic).
- Decoding PNG via `image` crate; emit como `/DeviceRGB` + opcional `/SMask` para alpha.
- Compressão Zlib (`/FlateDecode`) para canais PNG.
- JPEG passado raw (`/DCTDecode`) com ColorSpace detectado via SOF marker.
- JPEGs RGB usam `/ColorSpace [/ICCBased <id> 0 R]` com um perfil ICC sRGB compacto
  partilhado por todos os JPEGs RGB do documento (P777). O stream `/ICCBased` inclui
  `/Range [0 1 0 1 0 1]` e `Length` igual ao tamanho comprimido. JPEGs grayscale mantêm
  `/DeviceGray`; JPEGs CMYK mantêm `/DeviceCMYK`.
- Deduplicação por `Arc::as_ptr` (ADR-0095): mesma imagem usada N vezes → 1 XObject.
- Walkers recursivos em `FrameItem::Group` (P279 bug fix).

## Restrições estruturais

- L3. Usa `image` crate (decoding), `flate2` (zlib). Não FS.
- `PdfImagePayload` e `process_png_for_pdf` são `pub` — API exposta via re-export em `export/mod.rs`.
- `ImageFormat`, `ImageRef`, `ImageXObject`, `scan_all_images`, etc. são `pub(super)` para uso interno (`super::builder`, `super::stream`).
- Builders de XObject (`build_jpeg_xobject`, `build_png_smask_xobject`, `build_png_rgb_xobject`) são `pub(super)`.
- `compress_zlib` é `pub(super)` — partilhável com `gradients/conic.rs` se Pattern shading streams forem comprimidos.

## Interface

```rust
pub struct PdfImagePayload {
    pub width: u32, pub height: u32,
    pub color_space: &'static str,
    pub rgb_data_compressed: Vec<u8>,
    pub alpha_data_compressed: Option<Vec<u8>>,
}
pub fn process_png_for_pdf(raw_data: &[u8]) -> Result<PdfImagePayload, String>;

pub(super) enum ImageFormat { Jpeg, Png, Unknown }
pub(super) fn detect_format(data: &[u8]) -> ImageFormat;
pub(super) fn jpeg_color_space(data: &[u8]) -> &'static str;
pub(super) fn jpeg_is_rgb(data: &[u8]) -> bool;
pub(super) fn compress_zlib(data: &[u8]) -> Result<Vec<u8>, String>;
pub(super) fn scan_all_images(doc, first_id, icc_profile_id) -> (Vec<ImageRef>, HashMap<usize, usize>, Vec<ImageXObject>);
pub(super) fn xobject_resources_for_page(page, ptr_to_idx, refs) -> String;
pub(super) fn build_icc_profile_stream(icc_data) -> Vec<u8>;
pub(super) fn build_jpeg_xobject(data, iw, ih, color_space, icc_profile_id) -> Vec<u8>;
pub(super) fn build_png_smask_xobject(w, h, alpha) -> Vec<u8>;
pub(super) fn build_png_rgb_xobject(payload, smask_id) -> Vec<u8>;
```

## Invariantes

- Dedup por `Arc::as_ptr(data) as usize` — seguro porque doc mantém Arcs vivos durante export.
- ID allocation: SMask sempre antes do RGB main (xref crescente).
- Imagens em Groups (Block clip / Transform) registadas pelo walker recursivo.
- PNG totalmente opaco → SMask omitido (optimização).
- JPEGs RGB partilham o mesmo objecto ICC profile; o ID é fornecido pelo builder.

## Critérios de verificação

- `process_png_for_pdf(png_bytes)` produz `PdfImagePayload` válido para PNGs RGB com/sem alpha.
- `detect_format(jpeg_bytes)` → `Jpeg`; `detect_format(png_bytes)` → `Png`; outros → `Unknown`.
- Imagens iguais em diferentes Groups → mesma XObject (verificado via `pipeline_dedup_jpeg_*` tests).
