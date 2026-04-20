# Prompt L0 — layout_types
Hash do Código: 0e2ebf9a

## Módulo
`01_core/src/entities/layout_types.rs`

## Propósito
Tipos de dados de layout: coordenadas, frames, documento paginado.
Puramente declarativos — sem I/O, sem métricas de fonte.

## Divergência do original
- **Abs** original: `Abs(Scalar)` com unidades raw internas e conversões pt/mm/cm.
  Cristalino usa `Pt(f64)` — mais simples, suficiente para o Passo 19.
- **Frame** original: `items: Arc<LazyHash<Vec<(Point, FrameItem)>>>` — posição na tupla.
  Cristalino embute `pos: Point` em `FrameItem::Text` — mais simples.
- **PagedDocument** original: `EcoVec<Page>` + `DocumentInfo` + `Arc<PagedIntrospector>`.
  Cristalino usa `Vec<Frame>` — stub até Passo 20+.

## Tipos

### `Pt`
Newtype f64 para pontos tipográficos. `Pt + Pt` OK; `Pt + f64` NÃO implementado.

### `Point`, `Size`
Coordenada 2D e tamanho 2D em `Pt`.

### `FrameItem`
Variantes: `Text { pos, text, style }`, `Line { start, end, thickness }`,
`Glyph { pos, glyph_id, x_advance, size }`, `Image { pos, data, width, height, intrinsic_width, intrinsic_height }`.

`Image`: representa uma imagem a renderizar na página.
- `pos`: canto superior esquerdo em coordenadas de página (pt).
- `data: Arc<Vec<u8>>`: bytes raw da imagem — zero-copy via Arc.
- `width`, `height`: dimensões físicas no documento (pt) — tamanho de layout.
- `intrinsic_width`, `intrinsic_height`: dimensões reais em píxeis — obrigatórias
  para o dicionário XObject PDF (/Width, /Height intrínsecos ≠ tamanho de layout).

`plain_text()` ignora `Image`, `Line`, `Glyph` — retorna apenas texto.

### `Frame`
Canvas de uma página. `plain_text()` para verificação em testes.

### `PagedDocument`
Resultado de `layout()`. `plain_text()` concatena páginas com `"\n"`.
Campo `extracted_label_pages: HashMap<Label, usize>` expõe o mapa de páginas
gerado por `layout_labelled` durante a passagem. Inicializado vazio em `new()`.
Populado por `Layouter::finish()` — sem alterar a assinatura de `layout()`.

## Critérios de verificação
- `Pt(10.0) + Pt(5.0) == Pt(15.0)`
- `Pt * f64` compila; `Pt + f64` não compila
- `Frame::plain_text()` junta texto dos FrameItem::Text com espaço
- `PagedDocument::plain_text()` concatena páginas com newline
