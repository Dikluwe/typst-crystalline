# Prompt L0 — layout_types
Hash do Código: d8c50ed1

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
- `orientation`: valor EXIF `Orientation` (1-8) lido do cabeçalho. O exportador
  PDF usa este valor para compor a matriz `cm` de transformação (P776).

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

**P535** — headings para bookmarks PDF:
- `extracted_headings: Vec<(Label, Option<String>, Content, usize)>` — cópia
  de `Introspector::headings_for_toc()` no pipeline, usada pelo exportador para
  construir a árvore `/Outlines`. Cada tuplo é `(auto-label, número, body, level)`.

**P536** — metadados do documento (`/Info`):
- `document_info: DocumentInfo` — metadados definidos por `#set document(...)`.
  Copiado do `Module` pelo pipeline antes da exportação PDF.

Todos inicializados vazios em `new()` e populados por `Layouter::finish()` ou
pelo pipeline pós-layout — sem alterar a assinatura de `layout()`.

## `PageConfig`

Configuração da página activa durante o layout. Campos principais:
- `width`, `height`: dimensões em pt.
- `margin`: margem uniforme em pt.
- `numbering`: padrão de numeração automática (P532).
- `columns`: número de colunas activas (P537b).

### Margem automática por omissão

A margem por omissão **não é um valor fixo**. Segue o vanilla 0.15.0:

```text
margin = min(width, height) * (2.5 / 21)
```

Equivalentemente, `≈ 11.90476 %` da menor dimensão da página. Para uma
página A4 (`595.28 pt × 841.89 pt`), isto produz `70.87 pt` (`≈ 2.5 cm`).
Para uma página pequena (`height: 200 pt`, `width: 595.28 pt`), produz
`≈ 23.81 pt`.

`PageConfig` distingue duas situações através do campo `margin_is_auto`:

- `margin_is_auto: true` — margem calculada automaticamente. O default é
  `true`. Sempre que `SetPage` alterar `width` ou `height` sem fornecer um
  valor explícito de `margin`, a margem deve ser recalculada pela fórmula
  acima (`auto_margin()`).
- `margin_is_auto: false` — margem fixa definida pelo utilizador. Não é
  recalculada quando `width`/`height` mudam.

`PageConfig::default()` deve calcular a margem a partir de `width` e
`height`, não hard-codificar `70.87`. Quando `SetPage` recebe `margin`
explicitamente, `margin_is_auto` passa a `false`. Quando `margin` é
ausente, `margin_is_auto` não muda — o default é `true`, e uma margem
fixa pelo utilizador permanece fixa até novo `margin` explícito.

Sempre que `SetPage` altera `width` ou `height`, se `margin_is_auto` for
`true` a margem é recalculada pela fórmula; se for `false`, a margem
fixa é preservada.

## Critérios de verificação
- `Pt(10.0) + Pt(5.0) == Pt(15.0)`
- `Pt * f64` compila; `Pt + f64` não compila
- `Frame::plain_text()` junta texto dos FrameItem::Text com espaço
- `PagedDocument::plain_text()` concatena páginas com newline
- `PageConfig::default()` calcula margem como `min(width, height) * 2.5 / 21`

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

## P784 — `TextStyle.math: bool`

**Data:** 2026-07-17

Novo campo `pub math: bool` em `TextStyle` (default `false`, via `#[derive(Default)]`).
`true` sse este texto é conteúdo matemático. Definido **uma única vez**, em
`layout/equation.rs::layout_equation` (`TextStyle { math: true, ..self.style.clone() }`,
por cima do estilo herdado antes de chamar `MathLayouter::layout_equation`) — herdado
daí em diante por toda a árvore de layout math via `..style.clone()` (os sub-layouts
de math, `layout_text_node`/`attach.rs`/`frac.rs`/`root.rs`, nunca tocam `.font`
nem `.math`, só `.italic`/`.size`).

**Motivo**: `03_infra/src/shaper.rs` decide se engata a cadeia de fallback específica
de matemática (`fallback_fonts.rs::DEFAULT_FALLBACK_FONTS_MATH`) como primárias
adicionais. A condição original (P783, só `primary_has_math` — a fonte já resolvida
tem tabela MATH OpenType própria) nunca disparava no caso comum: a fonte de corpo
por omissão (`Libertinus Serif`) não tem tabela MATH. `style.math` dá ao shaper um
sinal directo e correcto de "isto é matemática", independente de qualquer propriedade
da fonte primária resolvida. Ver `infra/shaper.md` §P784 para a lógica de consumo.

Dois sites de construção não-spread de `TextStyle` (fora do `..style.clone()` normal)
precisaram de valor explícito: `entities/style_chain.rs::From<&StyleChain>` (`false`
— `StyleChain` não carrega contexto math) e `engine/layout/text.rs` (herda de
`layouter.style.math`, merge de `#set text(...)` não é math-específico).
