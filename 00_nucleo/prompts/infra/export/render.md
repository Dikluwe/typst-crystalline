# Prompt L0 — `infra/export/render` — Rasterização PNG
Hash do Código: f464d3b6

**Camada**: L3  
**Ficheiro alvo**: `03_infra/src/export/render.rs`  
**Origem**: P870  
**ADRs**: ADR-0033 (paridade observable)

---

## Contexto

Rasterizador de páginas Typst para imagens PNG. Recebe uma `Page` cristalina e devolve bytes PNG. Baseia-se na estrutura do `typst-render` do vanilla (`lab/typst-original/crates/typst-render/`), mas adaptado aos tipos cristalinos (`FrameItem`, `TextStyle`, `Color`, `ShapeKind`, etc.).

## Interface pública

```rust
pub struct RenderOptions {
    /// Pixels por ponto tipográfico. Default 2.0 (paridade vanilla).
    pub pixel_per_pt: f32,
    /// Se true, expande a área renderizada para incluir bleed.
    pub render_bleed: bool,
}

impl Default for RenderOptions { ... }

/// Renderiza uma página para uma imagem PNG (texto sem fontes resolvidas → scope-out/tofu).
pub fn render_page_to_png(page: &Page, opts: &RenderOptions) -> Vec<u8>;

/// Renderiza uma página para PNG com fontes resolvidas (paridade `export_pdf_with_font`).
pub fn render_page_to_png_with_fonts(
    page: &Page,
    opts: &RenderOptions,
    fonts: &[((FontList, FontVariant, FontVariations), Vec<u8>)],
) -> Vec<u8>;

/// Renderiza um documento completo, empilhando páginas verticalmente.
pub fn render_document_to_png(doc: &PagedDocument, opts: &RenderOptions, gap_pt: f64) -> Vec<u8>;
```

## Dependências

- `tiny-skia` — canvas 2D e rasterização.
- `resvg` — renderização de paths/SVG internos quando necessário.
- `pixglyph` — rasterização de glifos (texto).
- `bytemuck` — cast seguro de bytes para `tiny-skia`.
- `image` — já em `03_infra/Cargo.toml`; usado para encoding PNG final.

## Estratégia de porte

- Copiar a estrutura de `typst-render/src/lib.rs` (State, render_frame recursivo).
- Mapear `FrameItem` cristalino para primitivas `tiny-skia`:
  - `TextShaped` → iterar glifos e renderizar via `pixglyph` ou fallback `text`.
  - `Shape` → path preenchido/contornado.
  - `Image` → decodificar e desenhar.
  - `Group` → aplicar transformação e recursar.
  - `Line` → linha.
  - `Link` → renderizar filhos.
- Manter o eixo Y cristalino (Y-down) ou converter conforme `tiny-skia` espera.

## Restrições

- L3 — sem I/O directo a ficheiros; recebe `Page` e devolve `Vec<u8>`.
- Usar os tipos cristalinos de `typst_core::entities::layout_types`.
- Não duplicar lógica de PDF; reutilizar helpers de `export/images.rs` para decodificação de imagens quando possível.

## Critérios de verificação

- Documento "Hello" renderiza PNG com texto legível.
- Documento com `circle`/`rect` renderiza formas.
- Documento com imagem renderiza imagem.
- Comparação visual (RMSE) contra vanilla para casos simples.
