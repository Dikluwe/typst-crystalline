# Prompt L0 — layout_types
Hash do Código: c07a3c11

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
Variantes:
- `Text { pos, text, style }`
- `Line { start, end, thickness, color }`
- `Glyph { pos, glyph_id, x_advance, size }`
- `Image { pos, data, width, height, intrinsic_width, intrinsic_height }`
- `Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit }` (Passo 76)
- `Group { pos, matrix, clip_mask, inner_width, inner_height, items }` (Passo 78)
- `Link { target, items, pos, size }` (P422/P424/P463)
- `LinkTarget::Url(EcoString) | LinkTarget::Destination(Label)` (P463)

**`Line.color: Option<Color>`** (Passo 285) — `Some(c)` emite `r g b RG`
antes do stroke (`S`) no PDF; `None` preserva default preto bit-exact
(backward-compat para frac/sqrt overline/linhas geométricas pré-P285).
Activa o `stroke` parseado em `Content::Underline`/`Strike`/`Overline`
(P284 §5.4 — pendência registada e agora resolvida). Regra de herança
no consumer Layouter: `color = stroke.or(self.style.fill)` (utilizador
explícito > herança do texto corrente > default preto).

`Image`: representa uma imagem a renderizar na página.
- `pos`: canto superior esquerdo em coordenadas de página (pt).
- `data: Arc<Vec<u8>>`: bytes raw da imagem — zero-copy via Arc.
- `width`, `height`: dimensões físicas no documento (pt) — tamanho de layout.
- `intrinsic_width`, `intrinsic_height`: dimensões reais em píxeis — obrigatórias
  para o dicionário XObject PDF (/Width, /Height intrínsecos ≠ tamanho de layout).

`Link` (P422/P424/P463): hiperligação. O `body` é renderizado normalmente e os
itens resultantes ficam em `items`. O destino é preservado como metadado
(`LinkTarget`) para o exportador PDF:
- `LinkTarget::Url` → annotation `/Subtype /Link` com `/A << /S /URI >>`;
- `LinkTarget::Destination(Label)` → annotation `/Subtype /Link` com
  `/A << /S /GoTo /D /name >>` (navegação interna para `/Dests`).
`pos` e `size` definem a bounding box da área clicável. Cor/sublinhado
são scope-out (aguardam `FrameItem::Decoration`).

`plain_text()` ignora `Image`, `Line`, `Glyph`, `Shape`, `Group` e desce
recursivamente em `Link` — retorna apenas texto.

### `Frame`
Canvas de uma página. `plain_text()` para verificação em testes.

### `PagedDocument`
Resultado de `layout()`. `plain_text()` concatena páginas com `"\n"`.
Campos de labels (P460):
- `extracted_label_pages: HashMap<Label, usize>` — mapa label → página (1-based),
  gerado por `layout_labelled` durante a passagem.
- `extracted_label_positions: HashMap<Label, Point>` — mapa label → posição (x, y)
  no momento da inserção, usado pelo exportador PDF para `/Dests`.
**P488** — campos de página para LoF/LoT (fixpoint carry-forward):
- `extracted_figure_page_numbers: Vec<usize>` — páginas de figuras contadas, em ordem de documento.
- `extracted_table_page_numbers: Vec<usize>` — páginas de tabelas contadas, em ordem de documento.
Todos inicializados vazios em `new()` e populados por `Layouter::finish()` — sem
alterar a assinatura de `layout()`.

## Critérios de verificação
- `Pt(10.0) + Pt(5.0) == Pt(15.0)`
- `Pt * f64` compila; `Pt + f64` não compila
- `Frame::plain_text()` junta texto dos FrameItem::Text com espaço
- `PagedDocument::plain_text()` concatena páginas com newline

## P482 — `ShapedGlyph` e `FrameItem::TextShaped`

**P482** adiciona:

```rust
pub struct ShapedGlyph {
    pub glyph_id:  u16,
    pub x_advance: i32,
    pub x_offset:  i32,
    pub y_offset:  i32,
    pub cluster:   u32,
    pub char_code: char,
}
```

e variante:

```rust
FrameItem::TextShaped {
    pos:    Point,
    glyphs: Vec<ShapedGlyph>,
    style:  TextStyle,
    text:   EcoString,  // texto original (fallback + CMap)
}
```

`TextShaped` é produzido pelo shaper L3 (`03_infra/src/shaper.rs`) a
partir de `FrameItem::Text` com `style.font.is_some()`. `Text` é
preservado como fallback (Type1 / fonte não carregada).

`plain_text_items` trata `TextShaped` como `Text` — extrai `text` field.

Todos os match exaustivos de `FrameItem` em L1 devem incluir arm `TextShaped`.
Ver ADR-0120.

## P483 — FrameItem::Text deprecated

**Data:** 2026-06-28

`FrameItem::Text` marcado `#[deprecated(since = "P483")]`. Continua a existir
como fallback para fontes não carregadas ou Type1. Todos os sites de match
legítimos receberam `#[allow(deprecated)]` ou `#![allow(deprecated)]`.

`From<&StyleChain> for TextStyle` preenche agora `font` com pelo menos
`FontList("Helvetica")` quando nenhum `#set text(font:...)` está activo.
Garante cobertura ≥95% do shaping em produção.

Path primário em `export/stream.rs`: `TextShaped` antes de `Text`.

## P484 — FrameItem::Text como tipo pré-shaping (permanente)

**Data:** 2026-06-28

`FrameItem::Text` **não removido**. A remoção exigiria migrar os emit sites de
L1 (`cursor.rs`, `list_item.rs`, etc.) para emitir `FrameItem::TextShaped`
directamente — impossível sem bytes de fonte (ADR-0029). `FrameItem::Text` é
o tipo de **pré-shaping**: emitido por L1, convertido para `TextShaped` pelo
shaper L3. O `#[deprecated]` (P483) sinaliza que L3/export não deve usar
`Text` directamente; não indica remoção iminente.

## P485 — `units_per_em: u16` em `FrameItem::TextShaped`

**Data:** 2026-06-28

`FrameItem::TextShaped` recebe campo adicional `units_per_em: u16`:

```rust
TextShaped {
    pos:          Point,
    glyphs:       Vec<ShapedGlyph>,
    style:        TextStyle,
    text:         EcoString,
    units_per_em: u16,   // ← P485
}
```

Populado por `shaper.rs::try_shape` via `rb_face.units_per_em().max(1) as u16`.
Usado em `emit_shaped_pdf` para calcular números TJ: `-(x_advance / upm * 1000)`.
Sites de match que não precisam de `units_per_em` usam `..` (wildcard).
