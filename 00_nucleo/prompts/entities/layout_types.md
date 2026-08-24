# Prompt L0 — layout_types
Hash do Código: 7a32b63c

## Módulo
`01_core/src/entities/layout_types.rs`

## Propósito
Tipos de dados de layout: coordenadas, frames, documento paginado.

## P1140.5-A — grupo semântico de fórmula

### Medição antes da decisão

O vanilla default gerou PDF `Tagged: yes`, `/StructElem /S /Formula` e
`/Alt(accessible equation)`; `--no-pdf-tags` removeu a structure tree. O PDF
cristalino medido gerou `Tagged: no`. `FrameItem` atual possui apenas desenho,
Group geométrico e Link (`layout_types.rs:303-442`), sem unidade semântica.

### Decisão

Adicionar variante fechada `FrameItem::Semantic` com `SemanticKind::Formula`,
`SemanticPlacement::{Inline, Block}`, `alt: Option<EcoString>` e `items` filhos.
Ela não possui desenho próprio: bounds/plain text/visitors/shaping/export visual
descem nos filhos. String vazia permanece `Some("")`; ausência e `none` são
`None`. A variante é contrato L1→L3 e força revisão exaustiva dos consumidores.

P1140.5 prova transporte até a fronteira de exportação. A geração de MCIDs,
ParentTree e StructTreeRoot pertence ao P1140.6; não fingir tagging por
`/ActualText` ou por um comentário no content stream.
Puramente declarativos — sem I/O, sem métricas de fonte.

### P1140.12 — barreira semântica de quebra explícita

Adicionar `SemanticKind::ExplicitLinebreakBoundary`. O envelope é visualmente
transparente, sem alt e contém apenas um filho Text vazio posicionado na
baseline da linha fechada. Sua única semântica é impedir que pós-processadores
fundam novamente linhas separadas por `linebreak`; não representa fórmula,
não produz texto e não desenha.

A variante é contrato público L1→L3. Posição y e espaço restante não
distinguem wrapping automático de quebra explícita depois que o Frame perdeu
a causa do flush. Não substituir o marcador por limiar empírico.

### P1140.13 — fronteira semântica de parágrafo

Medição prévia: `Content::Parbreak` altera apenas a geometria vertical em
`compiler/layout/mod.rs`; depois de fechado o `PagedDocument`, L3 não consegue
distinguir essa causa sem comparar distâncias entre baselines. A aproximação
vigente (`1.5 × altura`) não é semântica e pode confundir parágrafos com
wrapping automático.

Adicionar `SemanticKind::ParbreakBoundary`. O envelope é visualmente
transparente, tem `SemanticPlacement::Block`, contém um filho `Text` vazio na
baseline da linha que termina e preserva até L3 a causa estrutural da quebra.
Ele não representa texto, glifo, espaço, altura ou conteúdo acessível.

`ExplicitLinebreakBoundary` e `ParbreakBoundary` são variantes distintas: a
primeira fecha uma linha dentro do mesmo parágrafo; a segunda fecha o próprio
parágrafo. Consumers que só precisam impedir reflow podem tratá-las como a
mesma classe de barreira, sem apagar a distinção semântica do contrato.

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

> **Fonte de paridade (P1031)** — a fórmula é **citação literal**, e em duas camadas do
> vanilla ratificado (`e0e8ca4d`):
>
> - **Documentação da linguagem** — `crates/typst-library/src/layout/page.rs:127-131`, doc
>   comment do campo `margin`, publicado em
>   `typst.app/docs/reference/layout/page/#parameters-margin`:
>   *"The page's margins. - `{auto}`: The margins are set automatically to **2.5/21 times
>   the smaller dimension of the page**. This results in **2.5 cm margins for an A4 page**."*
>   Isto sustenta tanto a fórmula como o valor A4 de `≈ 2.5 cm` citados acima.
> - **Implementação** — `crates/typst-layout/src/pages/run.rs:121`:
>   `let default = Rel::<Length>::from((2.5 / 21.0) * min);`
>
> **Natureza**: literal. Note-se que a fórmula está **na documentação publicada**, não só no
> código — é regra de linguagem, não detalhe de implementação, o que reforça que
> `margin_is_auto` tem de reproduzir o recálculo e não fixar `70.87`.

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
— `StyleChain` não carrega contexto math) e `compiler/layout/text.rs` (herda de
`layouter.style.math`, merge de `#set text(...)` não é math-específico).

## P836 — campo `TextStyle::variations`

`TextStyle` ganha o campo
`pub variations: Option<crate::entities::font_variations::FontVariations>`
(default `None`), propagado do resolver `StyleChain::variations()` no
`From<&StyleChain> for TextStyle` e do merge de `layout/text.rs`
(canal custom `"text.variations"` + herdado do `layouter.style`).

Transporta as coordenadas de eixo explícitas de `#text(variations:)` /
`#set text(variations:)` até L3 (shaper, métricas, export), onde são
fundidas com os eixos derivados de `FontVariant` — ver
`infra/font_variant.md` (P836).

## P891 — campo `TextStyle::math_script: bool`

**Data:** 2026-07-24

Novo campo `pub math_script: bool` em `TextStyle` (default `false`, via
`#[derive(Default)]`, mesmo padrão de P784). `true` sse todo o conteúdo desta
chamada de layout está dentro de um script (sub/super-índice) de `MathAttach`
— definido **uma única vez**, em `attach.rs` ao construir `script_style`
(`TextStyle { math_script: true, size: style.size *
self.constants.script_percent_scale_down, ..style.clone() }`).

**Motivo**: `compiler/math/layout/spacing.rs::compute_gaps` precisa de saber se
a sequência inteira está em script size para suprimir `spacing_between`
(paridade vanilla `process.rs::spacing()`, condição "unless in script size" —
ver `math/layout/spacing.md` §P891). O cristalino processa uma sequência
inteira com um único `TextStyle` partilhado (ao contrário do `MathSize`
discreto por item do vanilla) — este campo é a adaptação mínima que carrega
esse sinal até `compute_gaps` sem introduzir um enum de tamanho discreto.

Dois sites de construção não-spread de `TextStyle` (fora do `..style.clone()`
normal) precisam de valor explícito, mesmos dois sites identificados em P784:
`entities/style_chain.rs::From<&StyleChain>` (`false` — `StyleChain` não
carrega contexto math/script) e `compiler/layout/text.rs` (herda de
`layouter.style.math_script`, merge de `#set text(...)` não é
script-específico).

## P915 — campo `TextStyle::cramped: bool`

**Data:** 2026-07-26

Novo campo `pub cramped: bool` em `TextStyle` (default `false`, via
`#[derive(Default)]`, mesmo padrão exacto de P891/`math_script`). `true` sse
o conteúdo desta chamada de layout está num contexto tipográfico "cramped"
(TeXbook/OpenType MATH — ver `scripts.rs:318-382` do vanilla, lido em
`typst-passo-915-relatorio.md` Fase A). Forçado a `true` em **4 pontos**
(mapeados por leitura do vanilla, não por inventário assumido):

1. **`attach.rs`** — estilo do subscrito (posições bottom-left/bottom-right,
   `bl`/`sub`), nunca o do superscrito (`tl`/`sup`, que herda `cramped` do
   estilo ambiente sem forçar). Ver `compiler/math/layout/attach.md` §P915.
2. **`frac.rs`** — estilo do denominador, nunca o do numerador (que herda).
   Ver `compiler/math/layout/frac.md` §P915.
3. **`root.rs`** — estilo do radicando e do índice (`root(n, x)`). Ver
   `compiler/math/layout/root.md` §P915.
4. **`accent.rs`** — estilo da base, incondicional (o cristalino só implementa
   accent "acima" — `MathAccentElem` não tem campo de posição — logo a
   condição do vanilla "só se accent.is_bottom()==false" é trivialmente
   sempre verdadeira aqui). Ver `compiler/math/layout/accent.md` §P915.

**Consumido** em `attach.rs::compute_script_shifts` — lê `cramped` do
`TextStyle` **ambiente** passado a `layout_attach` (não do `script_style`
interno), para decidir entre `superscript_shift_up`/`superscript_shift_up_
cramped` (`entities/math_constants.md` §P915) — só quando há superscrito
presente (`tl`/`sup`); nada mais na fórmula de `compute_script_shifts` muda.

**Achado de âmbito, não corrigido**: o vanilla também aplica cramped à base
de `overline()` (não `underline()`) — o cristalino não tem `overline()`/
`underline()` como construção matemática (esses nomes resolvem para
decoração de texto, `DecoKind::Overline`, via `stdlib/text.rs`, não para
`Content::MathUnderover`) — não há call site a tocar. Registado, não
inventado.

## P906 — `FrameItem::Glyph` ganha `style: TextStyle` + `base_char: char`

**Contexto**: mecanismo de esticamento horizontal de glifo (`compiler/layout.md`
§P906) — mas o achado abaixo é mais fundo, e partilhado com o eixo vertical
já existente (`layout_stretchy_delimiter`/`layout_assembly`).

**Achado, em cadeia, cada camada só visível depois de corrigir a anterior**:
`FrameItem::Glyph` (usado sempre que um glifo de variante/assembly
matemático não tem mapeamento Unicode de volta a char) só tinha
`{ pos, glyph_id, x_advance, size }` — nenhum campo de identidade de fonte
nem de carácter original. Consequência, confirmada por medição directa em
PDF real (`pdftotext -bbox`, `mutool`), não por inferência:

1. **Exportador não sabia que fonte usar** (`export/stream.md` §P906) —
   corrigido com `style` (usado para resolver via
   `FallbackFontMetrics::resolve_font_combo`).
2. **Selecção de QUAIS fontes embutir no PDF** (`infra/pipeline.md` §P906)
   é feita cedo, a partir de caracteres vistos em `Text`/`TextShaped` — cega
   a `Glyph` (que não carrega nenhum char). Corrigido com `base_char: char`
   (o carácter ORIGINAL pedido ao esticamento, ex. `⎵`/`⏟` — não o
   `glyph_id` resultante, que só é interpretável dentro da fonte de onde
   veio, informação perdida ao sair de L1, que trata `FontMetrics` como
   opaco).

**Não é redundante**: `style` sozinho (sem `base_char`) não chega para (2) —
`style.font` num `FrameItem::Glyph` nunca é reescrito para a fonte
efectivamente resolvida (ao contrário de `TextShaped.style.font`, reescrito
por `shaper.rs` durante o shaping) — só `base_char` permite re-resolver a
fonte candidata correcta no ponto de selecção. `base_char` sozinho (sem
`style`) não chega para (1) — a resolução de candidata depende de
`style.math`/`style.size`/variante, não só do char.

**Construção**: 4 sites em L1 (`layout_stretchy_delimiter`/
`layout_stretchy_glyph_horizontal` em `stretchy.rs`, `layout_assembly`/
`layout_assembly_horizontal` em `assembly.rs`) — todos já tinham `c: char`/
`style: &TextStyle` em scope, custo de fio zero. 6 sites de reconstrução
(`offset_item`/`translate_frame_item`-like em `equation.rs`, `cursor.rs`,
`slicing.rs`, `footnote_flush.rs`, `helpers.rs`, `math/layout/mod.rs`) só
propagam os campos inalterados.

## P945 — campo `TextStyle::math_size: MathSize` (nível discreto do vanilla)

**Data:** 2026-08-01

**Medição que motiva** (`typst-passo-945` Fase A): `$ mat(1,2,3;4,5,6;7,8,9) $`
em equação de **bloco** (`Display`) — o vanilla compõe as células a **11pt**
(denominator de `Display` = `Text`, factor 1.0 —
`lab/typst-original/crates/typst-library/src/math/style.rs:343-363`); o
cristalino compunha a `11pt × script_percent_scale_down = 7.7pt` (P923
implementou a descida de nível como ×0.7 incondicional — correcto só para
`Text→Script`, errado para `Display→Text`). Grelha ~30% mais curta → alvo do
delimitador curto → assembly com peças a menos (3 vs 4 glifos por lado,
`mutool trace`).

**Decisão**: novo enum e campo, mesmo padrão de P784/P891/P915:

```rust
pub enum MathSize { Display, Text, Script, ScriptScript }
// em TextStyle:
pub math_size: MathSize  // default: Text (via Default)
```

Semântica: o **nível MathSize discreto do vanilla** em vigor nesta chamada de
layout (equivalente a `EquationElem::size` na chain do vanilla). Os factores
de escala continuam a vir de `MathConstants`
(`script_percent_scale_down`/`script_script_percent_scale_down`, absolutos ao
tamanho base) — o campo só regista *em que nível* estamos, para que as
descidas de nível saibam o factor correcto:

| Transição (vanilla) | Factor sobre o tamanho corrente |
|---|---|
| `Display → Text` | ×1.0 |
| `Text → Script` | ×`script_percent_scale_down` |
| `Script → ScriptScript` | ×`sscript / script` (≈0.5/0.7) |
| `ScriptScript → ScriptScript` | ×1.0 |

**Ponto de entrada**: `compiler/layout/equation.rs::layout_equation` fixa
`math_size: Display` (bloco) / `Text` (inline) no `math_style` — paridade com
o vanilla (`EquationElem::size` = Display/Text conforme `block`,
`equation.rs:189-195`).

**Actualização nos pontos de descida** (sem mudar o factor de tamanho que
cada um já aplica hoje — só mantêm o campo honesto para consumidores
abaixo): `attach.rs` (scripts → desce um nível, Display/Text→Script,
Script/SScript→SScript), `frac.rs` (numerador/denominador → Display→Text,
Text→Script, …), `root.rs`, `underover.rs`, `matrix.rs`/`cases.rs` (estes
dois **também** corrigem o factor — ver `compiler/math/layout/matrix.md`
§P945).

## P968 — `faux_bold_stroke_pt`: limiar de intenção bold (weight ≥ 600), não "qualquer peso > 400"

**Data:** 2026-08-05

**Medição que motiva** (`typst-passo-968` Fase A; auditoria externa
2026-08-05 secção 8.3): no documento de 30 secções, **1541 de 2074 blocos
de texto (74.30%)** saem com `2 Tr` + `w` de 0.073pt/0.051pt; o vanilla usa
`0 Tr` em 100%. O documento tem **0% de negrito genuíno** (nenhum
`*...*`/`strong` — a única ocorrência de `*` no fonte é `x^*`, expoente).
Os dois `w` medidos batem exactamente com a fórmula de P139 aplicada a
**weight 450**: 50/300 × size × 0.04 = 0.073pt a 11pt e 0.051pt a 7.7pt
(script). A origem do 450: P944 fixa `weight: Some(450)` no `math_style` de
toda a equação (`compiler/layout/equation.rs`), replicando o show_set do
vanilla (`TextElem::weight = 450`,
`lab/typst-original/crates/typst-library/src/math/equation.rs:197`) —
**paridade de língua, correcta, fica**. A divergência é o gate de P139
disparar para ela: o vanilla **não tem** mecanismo de faux-bold em lado
nenhum (selecção de variante real via fontdb, sempre `0 Tr`).

**Decisão**: `faux_bold_stroke_pt` só produz stroke quando o weight
expressa **intenção de negrito** — limiar **weight ≥ 600** (fronteira
semibold/bold do OpenType). Abaixo de 600 o peso é um peso real da fonte
(450 Book do math, 500 medium, …), a satisfazer por selecção de variante,
nunca por contorno. A fórmula de P139 (`((w − 400)/300) × size × k`) fica
inalterada acima do limiar (700 → 0.44pt @ 11pt, como antes).

```rust
pub fn faux_bold_stroke_pt(&self, k: f64) -> f64 {
    let w = self.weight.unwrap_or(400);
    if w < 600 {
        return 0.0; // P968 — peso real da fonte não é intenção de negrito
    }
    ((w as f64 - 400.0) / 300.0) * self.size.val() * k
}
```

**O que não muda**: texto genuinamente negrito (weight 700) sem variante
bold real carregada continua a receber `2 Tr` + contorno — divergência de
mecânica consciente registada em P956 (a frente tipográfica de selecção de
variantes bold é pré-existente e fica como estava). Os pontos de emissão
(`stream.rs` Type1 e envelope verbose P956) não mudam — consomem o mesmo
helper.
