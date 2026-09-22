# Prompt L0 — `infra/export/images` — Imagens PDF
Hash do Código: 13c64656

**Camada**: L3
**Ficheiro alvo**: `03_infra/src/export/images.rs`
**Criado em**: 2026-05-19 (P307c)
**Atualizado em**: 2026-07-23 (P876 — cache de `PdfImagePayload` por ponteiro `Arc`)
**ADRs**: ADR-0029 (image Arc dedup), ADR-0095 (dedup via Arc::as_ptr)

---

## Contexto

Cluster completo para emit de imagens em PDF:
- Detecção de formato (JPEG via `FFD8FF`, PNG via 8-byte magic, GIF via
  `GIF87a`/`GIF89a`, WebP via `RIFF`+`WEBP` — P833/#17; enum em L1, ver
  `entities/image-format.md`).
- Decoding PNG/GIF/WebP via `image` crate (features `png`/`gif`/`webp`; GIF
  fica estático no primeiro frame — paridade vanilla); emit como `/DeviceRGB`
  + opcional `/SMask` para alpha. `process_png_for_pdf` é formato-genérica
  (nome histórico).
- Compressão Zlib (`/FlateDecode`) para canais.
- JPEG passado raw (`/DCTDecode`) com ColorSpace detectado via SOF marker.
- JPEGs RGB usam `/ColorSpace [/ICCBased <id> 0 R]` com um perfil ICC sRGB compacto
  partilhado por todos os JPEGs RGB do documento (P777). O stream `/ICCBased` inclui
  `/Range [0 1 0 1 0 1]` e `Length` igual ao tamanho comprimido. JPEGs grayscale mantêm
  `/DeviceGray`; JPEGs CMYK mantêm `/DeviceCMYK`.
- Deduplicação por `Arc::as_ptr` (ADR-0095): mesma imagem usada N vezes → 1 XObject.
- **P876** — cache de `PdfImagePayload` por ponteiro `Arc` (`process_png_for_pdf`):
  a decodificação raster (PNG/GIF/WebP) só acontece uma vez por `Arc<Vec<u8>>` distinto;
  chamadas repetidas reutilizam o payload já processado. Combina com o cache de bytes por
  `FileId` em `SystemWorld` para que 50× `image("x.png")` produzam 1 XObject.
- Walkers recursivos em `FrameItem::Group` (P279 bug fix).
- **P833 (#18)** — `validate_document_images(doc) -> Result<(), String>`:
  valida TODAS as imagens por descodificação completa ANTES do export
  (chamada no pipeline), falhando a compilação com a mensagem do vanilla
  (`failed to decode image ({detalhe})`). Antes, imagem corrompida era
  omitida silenciosamente (exit 0, PDF sem a imagem). JPEG incluído na
  validação (o export embute JPEG cru sem descodificar). O `eprintln!` de
  `scan_all_images` fica apenas como fallback defensivo para callers da API
  pública de export que saltem a validação do pipeline.

## Restrições estruturais

- L3. Usa `image` crate (decoding), `flate2` (zlib). Não FS.
- `PdfImagePayload` e `process_png_for_pdf` são `pub` — API exposta via re-export em `export/mod.rs`.
- **P772p** — `ImageFormat`/`detect_format` **movidos para L1**
  (`entities/image-format.md`, `typst_core::entities::image_format::{ImageFormat, detect_image_format}`)
  — reutilizados aqui via `use`, não redefinidos. Motivo: `native_image`
  (L1, avaliação) precisa da mesma detecção para dar erro de compilação em
  vez de omissão silenciosa (P650/P772k); é pura detecção de assinatura,
  sem I/O, 100% legal em L1 — mover evita duplicar a lógica entre camadas.
- `ImageRef`, `ImageXObject`, `scan_all_images`, etc. são `pub(super)` para uso interno (`super::builder`, `super::stream`).
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
pub(crate) fn validate_document_images(doc: &PagedDocument) -> Result<(), String>;  // P833 (#18) — erro de compilação no formato vanilla

// ImageFormat/detect_format: ver entities/image-format.md (L1, P772p) — importados, não redefinidos aqui.
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
- **P876** — `process_png_for_pdf` memoiza o resultado por `Arc::as_ptr(raw_data) as usize`;
  o cache vive durante o thread de compilação e é consultado tanto por `validate_document_images`
  quanto por `scan_all_images`, evitando decodificações repetidas da mesma imagem.
- ID allocation: SMask sempre antes do RGB main (xref crescente).
- Imagens em Groups (Block clip / Transform) registadas pelo walker recursivo.
- PNG totalmente opaco → SMask omitido (optimização).
- JPEGs RGB partilham o mesmo objecto ICC profile; o ID é fornecido pelo builder.

## Critérios de verificação

- `process_png_for_pdf(png_bytes)` produz `PdfImagePayload` válido para PNGs RGB com/sem alpha.
- `detect_format(jpeg_bytes)` → `Jpeg`; `detect_format(png_bytes)` → `Png`; outros → `Unknown`.
- Imagens iguais em diferentes Groups → mesma XObject (verificado via `pipeline_dedup_jpeg_*` tests).

## P1477 — segundo perfil ICC sRGB (v2) selecionado por versão do PDF

Ver `typst-passo-1477.md` para a medição completa. Continuação do
scope-out de `typst-passo-1475.md` §2.5/§4.4 (A1a/A1b precisam de um
perfil ICC diferente do v4 já vendorizado em P777).

### Medição anterior à decisão

`krilla/src/configure/version.rs:54-61` (checkout local
`~/.cargo/git/checkouts/krilla-dfa083154a81f6b2/7772dbe/`): o vanilla
seleciona o perfil sRGB por **versão do PDF**, não por perfil archival —
`Pdf14|Pdf15|Pdf16 → sRGB-v2-magic.icc` (736 bytes),
`Pdf17|Pdf20 → sRGB-v4.icc` (480 bytes, já vendorizado como
`SRGB_ICC_PROFILE`, P777). Mesma fonte/licença CC0 1.0
(`icc/README.md`: Compact-ICC-Profiles), confirmado por
`sha256sum` do ficheiro local:
`af4efe28f6d311799865f325ba39184a2e978b113ca74124d60af7de22b105f4`.
Medido: usar o v4 quando a versão efetiva é 1.4-1.6 falha a regra
veraPDF 6.2.3.2 ("does not satisfy PDF 1.4 requirements").

### Contrato

1. Nova constante `SRGB_V2_ICC_PROFILE: &[u8]` (736 bytes, cópia literal
   dos bytes de `sRGB-v2-magic.icc`, mesmo padrão do array decimal já
   usado para `SRGB_ICC_PROFILE`), junto à constante existente
   (`images.rs:39-63`).
2. Novo getter `pub(super) fn srgb_v2_icc_profile_bytes() -> &'static
   [u8]`, espelhando `srgb_icc_profile_bytes()`.
3. Novo helper `pub(super) fn srgb_icc_profile_for_version(version:
   typst_core::entities::document_format_options::PdfVersion) ->
   &'static [u8]`: `match` exaustivo sobre as cinco variantes de
   `PdfVersion` (nunca wildcard, V16) — `V1_4 | V1_5 | V1_6` devolve o
   perfil v2, `V1_7 | V2_0` devolve o v4 (mesma regra §krilla acima).

### Scope-out explícito

`grey_icc` (perfil ICC para `/DeviceGray`) não tem equivalente
cristalino hoje — `images.rs` já documenta que JPEGs grayscale usam
ColorSpace direto, sem ICCBased. Não implementado, não medido se algum
perfil A1 com conteúdo grayscale precisaria disto.

### Aceitação e gate

`srgb_icc_profile_for_version(V1_4/V1_5/V1_6)` devolve os 736 bytes do
perfil v2 (byte-a-byte iguais ao ficheiro fonte, verificar via
`sha256`/tamanho no teste); `V1_7/V2_0` devolve os 480 bytes do v4
(regressão zero — `srgb_icc_profile_bytes()` original continua a
existir e a ser usado onde a versão não é conhecida/relevante, se
algum consumidor ainda a chamar diretamente). Nenhum consumidor deste
ficheiro é editado aqui — só `infra/export/builder.md` consome o novo
helper (ver lá).
