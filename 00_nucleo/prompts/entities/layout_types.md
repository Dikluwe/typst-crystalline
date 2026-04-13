# Prompt L0 — layout_types
Hash do Código: f8cb85d6

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
Variante inicial: `Text { pos, text, font_size }`. Futuras: Shape, Image.

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
