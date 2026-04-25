# Diagnóstico Layout (Fase X) — Passo 156B

**Data**: 2026-04-25.
**Passo**: P156B (oitava aplicação do padrão diagnóstico-primeiro;
**primeira aplicação a categoria Layout**).
**Spec**: `00_nucleo/materialization/typst-passo-156b.md`.
**Outputs derivados**:
- ADR-0061 PROPOSTO (Layout roadmap; reocupação — era hayagriva).
- DEBT-56 (column flow Fase 3).
- ADR-0060 anotada (renumeração Model Fase 2: P156→P157, P157→P158, P158→P159).
- DEBT-55 actualizada (P159 + ADR-0062).
- Reserva ADR-0062 para hayagriva.
- Inventário 148 actualizado (Tabela A linha Layout reclassificada).

**Padrão**: análogo a P154A (Model). Inventário factual + arqueologia
+ ADR de roadmap PROPOSTO + DEBTs derivados conforme critério explícito.

---

## §1 — Inventário detalhado vanilla

Fonte: `lab/typst-original/crates/typst-library/src/layout/`
(~32 ficheiros + `grid/` subdir). Lógica algorítmica em
`lab/typst-original/crates/typst-layout/` (out-of-scope para este
diagnóstico estrutural).

### §1.1 Elementos visuais user-facing

| Elemento | Ficheiro | Atributos principais | Notas |
|----------|----------|----------------------|-------|
| `PageElem` | `page.rs` | paper, width, height, flipped, margin, binding, columns, fill, numbering, supplement, number_align, header, header_ascent, footer, footer_descent, background, foreground, body | 18 atributos `#[ghost]` (set-rule). Não pode ser `#show`-targeted. |
| `PagebreakElem` | `page.rs` | weak, to: Parity, boundary | Trivial declarativo. `weak` evita break em página vazia; `to` força par/ímpar. |
| `BoxElem` | `container.rs` | width: Sizing, height, baseline, fill, stroke, radius, inset, outset, clip, body | Container inline. 10 atributos. |
| `BlockElem` | `container.rs` | width, height: Sizing, breakable, fill, stroke, radius, inset, outset, spacing, above, below, clip, sticky, body: Option<BlockBody> | Container block. 14 atributos. **Suporta `breakable` + `sticky` (page-break-aware)**. |
| `InlineElem` | `container.rs` | (ghost; usado por equation) | Construct custom; layouter callback dinâmico. |
| `StackElem` | `stack.rs` | dir: Dir, spacing: Option<Spacing>, children: Vec<StackChild> | Itera children por `dir`. `StackChild::{Block(Content), Spacing(Spacing)}`. |
| `ColumnsElem` | `columns.rs` | count: NonZeroUsize, gutter: Rel<Length>, body: Content | **Apenas declarativo (3 campos, 103 linhas)**. Algoritmo column flow vive em `typst-layout/src/flow/`. **Vanilla actual NÃO faz balanceamento de colunas.** |
| `ColbreakElem` | `columns.rs` | weak: bool | Trivial. |
| `PadElem` | `pad.rs` | left, top, right, bottom, x, y, rest, body | 7 atributos padding + body. Resolve para `Sides<Rel<Length>>`. Trivial. |
| `HideElem` | `hide.rs` | body: Content, hidden: bool (synthesized) | Mantém o espaço, oculta visualmente. Tagged. Trivial. |
| `RepeatElem` | `repeat.rs` | body: Content, gap: Length (default 1em), justify: bool | **Lazy fill horizontal** (TOC dot leaders). 45 linhas. Lógica em `typst-layout/src/repeat.rs`. |
| `PlaceElem` | `place.rs` | alignment, scope: PlacementScope, float: bool, clearance: Length, dx, dy, body | Locatable + Tagged + Unqueriable. Suporta floats. |
| `FlushElem` | `place.rs` | (vazio) | Força layout de floats pendentes. |
| `AlignElem` | `align.rs` | alignment: Alignment, body: Content | Trivial. |
| `MoveElem` | `transform.rs` | dx, dy, body | Translação. |
| `RotateElem` | `transform.rs` | angle, origin: Alignment, reflow: bool, body | |
| `ScaleElem` | `transform.rs` | factor, x, y, origin, reflow, body | |
| `SkewElem` | `transform.rs` | ax: Angle, ay: Angle, origin, reflow, body | Cisalhamento. |
| `HElem` | `spacing.rs` | amount: Spacing, weak: bool | H-spacing (`h()` stdlib). |
| `VElem` | `spacing.rs` | amount: Spacing, weak: bool, attach: bool | V-spacing (`v()` stdlib). |
| `GridElem` | `grid/mod.rs` | columns, rows, gutter, column_gutter, row_gutter, fill: Celled, align: Celled, stroke, inset, children: Vec<GridChild> | Sintetizado. Tagged. **Muito complexo** (~1028 + 2421 linhas). |
| `GridCell` / `GridHLine` / `GridVLine` / `GridHeader` / `GridFooter` | `grid/mod.rs` | x, y, colspan, rowspan, breakable, repeat | Sub-elementos com spans + repeating headers. |
| `LayoutElem` | `layout.rs` | (Locatable; ghost para `layout()` func) | Acesso ao tamanho disponível via callback. |

### §1.2 Funções stdlib relacionadas

| Função | Ficheiro | Notas |
|--------|----------|-------|
| `layout(callback)` | `layout.rs` | Callback recebe `Size`. Locatable. Depende de Introspection. |
| `measure(content, container?)` | `measure.rs` | Calcula tamanho sem renderizar. **Contextual**. |
| `pagebreak(weak: bool, to: ?)` | `page.rs` | Construtor de `PagebreakElem`. |

### §1.3 Tipos primitivos (geometria + units)

| Tipo | Ficheiro | Notas |
|------|----------|-------|
| `Frame` | `frame.rs` | size, baseline, items: `Arc<LazyHash<Vec<(Point,FrameItem)>>>`, kind: FrameKind |
| `FrameItem` | `frame.rs` | enum: Group, Text, Shape, Image, Link, Tag |
| `FrameKind` | `frame.rs` | Soft, Hard (boundary para gradientes) |
| `Fragment` | `fragment.rs` | `Vec<Frame>` (resultado layout multi-página) |
| `Region` / `Regions<'a>` | `regions.rs` | `size, expand, full, backlog, last`. **Sem campo footnote** — footnote area é state separado em `flow/`. |
| `Sides<T>` / `Side` | `sides.rs` | left/top/right/bottom; usados para margens, padding |
| `Corners<T>` / `Corner` | `corners.rs` | top_left/top_right/bottom_right/bottom_left |
| `Axes<T>` / `Axis` | `axes.rs` | x/y; base de Size, Point |
| `Dir` | `dir.rs` | LTR, RTL, TTB, BTT |
| `Abs` / `AbsUnit` | `abs.rs` | Scalar (f64) em pt; Mm/Cm/In/Pt |
| `Em` | `em.rs` | Relativa a font-size; `at(font_size) -> Abs` |
| `Length` | `length.rs` | abs + em |
| `Ratio` | `ratio.rs` | percentage (0..1) |
| `Rel<T>` | `rel.rs` | rel + abs |
| `Fr` | `fr.rs` | Fractional unit (preenche restante proporcional) |
| `Point` | `point.rs` | x, y: Abs |
| `Rect` | `rect.rs` | min, max: Point |
| `Size` | `size.rs` | alias para `Axes<Abs>` |
| `Angle` / `AngleUnit` | `angle.rs` | radianos; Rad/Deg |
| `Transform` | `transform.rs` | sx, ky, kx, sy: Ratio; tx, ty: Abs (matriz 2D affine) |
| `ScaleAmount` | `transform.rs` | Ratio ou Length |
| `Sizing` | `container.rs` | Auto, Rel, Fr |
| `Spacing` | `spacing.rs` | Rel, Fr |
| `Margin` / `Binding` / `Parity` / `Paper` / `PageRanges` | `page.rs` | Set-rule helpers |
| `Alignment` / `HAlignment` / `VAlignment` / `OuterHAlignment` / `OuterVAlignment` / `FixedAlignment` | `align.rs` | Hierarquia rica para resolução via `Dir` |
| `PlacementScope` | `place.rs` | `Column`, `Parent` |

### §1.4 Page runtime real

A `PageElem` em `typst-library` é apenas set-rule descriptor.
A `Page` runtime real está em
`lab/typst-original/crates/typst-layout/src/document.rs`:

```text
pub struct PagedDocument {
    pages: EcoVec<Page>,
    info: DocumentInfo,
    introspector: Arc<PagedIntrospector>,
}

pub struct Page {
    pub frame: Frame,
    pub fill: Smart<Option<Paint>>,
    pub numbering: Option<Numbering>,
    pub supplement: Content,
    pub number: u64,
}
```

A `Page` final é **frame + metadata mínima**. Header/footer/
background/foreground são fundidos no `Frame` durante layout
(não há campos separados). Footnote area é sub-frame composto
durante o flow (não campo de `Page`).

### §1.5 Mecanismo de page break vanilla

Dois mecanismos coexistem:

1. **Manual**: `PagebreakElem` com atributos `weak`, `to`,
   `boundary`.
2. **Automático**: durante o flow quando `Regions::next()` é
   invocado. Lógica em `typst-layout/src/pages/run.rs` (~233
   linhas) + `flow/compose.rs` (~946 linhas).

Set rules de `PageElem` aplicadas no meio do documento causam
page break implícito se mudarem propriedades.

### §1.6 Footnote: como é gerida no vanilla

**Não existe campo `footnote_area`** em `Page` nem em `Regions`.
Em vez disso, footnotes são geridas pelo `FlowState` em
`typst-layout/src/flow/mod.rs`:

```text
footnotes: EcoVec<Packed<FootnoteElem>>,        // queue
footnote_spill: Option<std::vec::IntoIter<Frame>>, // overflow
footnote: FootnoteConfig { separator, clearance, gap }
```

Apenas o "root flow" tem footnotes (`is_root: bool`). A struct
`Regions` reserva espaço implicitamente via `backlog`/`last`,
sem campo dedicado.

### §1.7 Column flow: complexidade

Vanilla:
- Declaração: 103 linhas (`columns.rs`).
- Algoritmo: vive em `typst-layout/src/flow/` (~3000 linhas).
- Implementação: `Regions` com width reduzida (`width / count -
  gutter`), iterando colunas como páginas sequenciais.
- `ColbreakElem` é tratado como pagebreak interno à coluna.
- **Não faz balanceamento** de altura de colunas.

Não requer biblioteca externa. É extensão natural de
`Regions` + `Frame`. **Mas exige refactor profundo do
Layouter** (single-region → multi-region iteration).

### §1.8 Crates externas usadas pelo módulo Layout vanilla

Em `typst-library/src/layout/`:

- `comemo` (Track) — tracked queries em `grid/mod.rs`.
- `smallvec` — `TrackSizings`, sub-elements grid.
- `ecow` (transitivamente) — EcoString, EcoVec.
- `typst_utils` — Scalar, LazyHash, NonZeroExt, Numeric, singleton.
- `typst_syntax` — Span (em `Frame`).

**Nenhuma dependência de algoritmo de layout externo** (cosmic-text,
taffy, harfbuzz). Tudo in-tree.

---

## §2 — Estado actual em cristalino

Fonte: `01_core/src/entities/{layout_types.rs,content.rs}` +
`01_core/src/rules/layout/{mod.rs,cursor.rs,grid.rs,placement.rs,
figure.rs,helpers.rs,metrics.rs,hyphenation.rs}` +
`01_core/src/rules/eval/mod.rs` (`make_stdlib`).

### §2.1 Tabela: vanilla esperado vs cristalino actual

| Elemento vanilla | Cristalino tipo / função | Status empírico | Notas |
|------------------|--------------------------|-----------------|-------|
| `align(...)` | `Content::Align { alignment, body }` + constantes `left/center/...` | **implementado** | Passos 84.5/84.6 (DEBT-36, 37) |
| `place(...)` | `Content::Place { alignment, dx, dy, scope, body }` + `native_place` | **parcial** | Sem `float`, `clearance`. Diverge no `PlaceScope::Parent` (vanilla exige `float:true`) |
| `move(...)` / `rotate(...)` / `scale(...)` | `Content::Transform { matrix, body }` + `native_move/rotate/scale` | **implementado** | Unificado num único variant; 3 funções stdlib |
| `skew(...)` | — | **ausente** | Sem `Content::*`; sem stdlib |
| `grid(...)` | `Content::Grid { columns, rows, cells }` + `native_grid` | **parcial** (era declarado `implementado⁺`) | Sem `gutter`, `align`, `stroke`, `fill`, `inset`, `header`, `footer`, `colspan`/`rowspan`. DEBT-34d/e abertos. |
| `pad(...)` | — | **ausente** (era declarado `parcial`) | Sem `Content::Pad`; sem stdlib `pad()`. **Reclassificação P156B**. |
| `box(...)` | — | **ausente** | Sem `Content::Box`; sem stdlib `box()` |
| `block(...)` | — | **ausente** | Sem `Content::Block`; sem stdlib `block()` |
| `stack(...)` | — | **ausente** | Sem `Content::Stack`; sem stdlib `stack()` |
| `hide(...)` | — | **ausente** | Sem `Content::Hide`; sem stdlib `hide()` |
| `repeat(...)` | — | **ausente** | Sem `Content::Repeat`; sem stdlib `repeat()` |
| `columns(n, ...)` | — | **ausente** | Sem `Content::Columns`; sem column-flow no Layouter |
| `colbreak()` | — | **ausente** | Depende de columns |
| `pagebreak()` | (apenas implícito via overflow + `Content::SetPage`) | **ausente** (era declarado `parcial`) | Sem `Content::Pagebreak` nem `pagebreak()` stdlib. **Reclassificação P156B**: parcial → ausente (manual) |
| `h()` / `v()` (spacing) | — | **ausente** | Sem `Content::H`/`Content::V`; sem stdlib `h()`/`v()`. **Adição P156B** (não estava em §A.5 do inventário 148) |
| `measure(body)` | `measure_content` (helper privado) | **parcial** | Heurística simples; sem `measure()` exposto à stdlib |
| `layout(callback)` | — | **ausente** | Depende de Introspection runtime (ADR-0017 adiada) |
| (footnote area no Page) | — | **ausente** | Bloqueante crítico para `footnote()` (Model) |

### §2.2 Stdlib Layout — funções registadas

Funções Layout no `make_stdlib` (`eval/mod.rs:513-571`):

- **Implementadas**: `page` (gera `Content::SetPage`), `grid`,
  `align`, `place`, `move`, `rotate`, `scale`, `rect`,
  `ellipse`, `circle`, `line`, `polygon`, `image`, `figure`.
- **Ausentes**: `block`, `box`, `pad`, `hide`, `stack`,
  `columns`, `repeat`, `pagebreak`, `colbreak`, `h`, `v`,
  `skew`, `measure`, `layout`.

### §2.3 Estrutura crítica: Page actual

```text
01_core/src/entities/layout_types.rs:359
pub struct Page {
    pub width:  f64,
    pub height: f64,
    pub items:  Vec<FrameItem>,
}
```

**Confirmado**: NÃO tem `footnote_area`, nem `header`,
`footer`, `background`, `foreground`, `number`, `numbering`,
`fill`. Snapshot mínimo de dimensão + items planos.

### §2.4 Estrutura crítica: PagedDocument actual

```text
01_core/src/entities/layout_types.rs:424
pub struct PagedDocument {
    pub pages: Vec<Page>,
    pub extracted_label_pages: HashMap<Label, usize>,
}
```

Páginas criadas em três sítios:
- `Layouter::new_page()` (`cursor.rs:128`) — overflow ou pagebreak interno.
- `Layouter::finish()` (`mod.rs:677`) — página final.
- `Content::SetPage` (`mod.rs:500`) — força `new_page()` se a actual não vazia.

### §2.5 Estrutura crítica: Frame + FrameItem

```text
pub struct Frame {
    pub size:  Size,
    pub items: Vec<FrameItem>,
}

pub enum FrameItem {
    Text { pos, text: EcoString, style: TextStyle },
    Line { start, end, thickness },
    Glyph { pos, glyph_id, x_advance, size },
    Image { pos, data: Arc<Vec<u8>>, ... },
    Shape { pos, kind: ShapeKind, ... },
    Group { pos, matrix, clip_mask, inner_*, items: Vec<FrameItem> },
}
```

**Frame é vestigial**: o Layouter escreve directo em
`current_items: Vec<FrameItem>` da página actual (não usa
Frame intermediário). Frame só é construído em testes.

### §2.6 PageConfig — divergência arquitectural

```text
pub struct PageConfig {
    pub width:  f64,
    pub height: f64,
    pub margin: f64,  // <-- escalar (vanilla é Sides<Length>)
}
```

Vanilla suporta `Margin { sides: Sides<Option<Smart<Rel<Length>>>>,
two_sided: Option<bool> }`. Cristalino: margem uniforme.

### §2.7 Re-classificação empírica vs inventário 148 §A.5

| Feature | Inventário 148 declarado | P156B empírico | Movimento |
|---------|---------------------------|----------------|-----------|
| pad | parcial | ausente | ↓ |
| pagebreak | parcial | ausente (manual) | ↓ |
| grid | implementado⁺ | parcial | ↓ |
| place | implementado | parcial (sem float/clearance) | ↓ |
| (h/v spacing) | (não listado) | ausente (entrada nova) | + |
| (skew) | (não listado em A.5) | ausente (entrada nova) | + |

Padrão: análogo a Model 154A (38% declarado → 32-36% empírico).

**Recálculo Layout** (com 18 entradas após adição de `h/v`):

| Estado | Contagem | Entradas |
|--------|---------:|----------|
| implementado | 5 | align, move, rotate, scale, (transform agregado conta como 1; soma com align+place=parcial+grid=parcial fica 3 implementado puros) |

Ajuste preciso (cada feature como entrada):

- **implementado** (4): align, move, rotate, scale.
- **implementado⁺** (0).
- **parcial** (3): place (sem float/clearance), grid (sem gutter/etc), measure (heurística privada).
- **ausente** (11): pad, box, block, stack, hide, repeat, columns, colbreak, pagebreak (manual), h/v (1 entrada combinada), skew.
- **scope-out** (0).
- **Adição implícita**: footnote area (bloqueante; faz parte de Page model, não user-facing isolado).

**Cobertura empírica Layout**: (4 + 0) / 18 = **22%**
considerando apenas `implementado`. Se incluir `parcial` como
"existe mas incompleto": (4 + 3) / 18 = **39%** (próximo do
38% declarado mas com entradas redistribuídas).

**Convenção P154A para reportar**: cobertura conta `implementado`
+ `implementado⁺` apenas. Logo: **22% empírico** (vs 38%
declarado).

---

## §3 — Tipos arquitecturais bloqueantes

Para cada `parcial`/`ausente` em §2.1, listar tipos faltantes:

| Bloqueante | Quem precisa | Custo estimado | Decisão arquitectural? |
|------------|--------------|---------------:|------------------------|
| `Page::footnote_area: Vec<FrameItem>` (ou similar) | `footnote()` (Model) | M | **sim** — extensão minimalista de `Page` |
| Reservar espaço de footnote durante layout | idem | M | **sim** — Layouter altera `cursor_y` reserve |
| `Content::Pad` ou `Style::Pad` | `pad()` | S | sim — variant ou Style |
| `Content::Hide` | `hide()` | S | sim — variant trivial |
| `Content::Pagebreak` ou `Style::PageBreak` | `pagebreak()` | S | sim — variant ou Style |
| `Content::H` + `Content::V` | `h()` + `v()` | S | sim — variants triviais |
| `Content::Block` ou `Content::Styled` | `block()` | M+ | sim — decisão |
| `Content::Box` ou `Content::Styled` | `box()` | S-M | sim — decisão |
| `Content::Stack` + `dir: Dir` | `stack()` | S-M | sim — variant novo |
| Multi-region Layouter (column flow) | `columns()` + `colbreak()` | L+ | **sim — ADR dedicada futura**; **DEBT-56** |
| `Content::Repeat` + lazy semantic | `repeat()` | M | sim — variant + consumer |
| Introspection runtime | `measure()`, `layout()` | XL | depende ADR-0017 (adiada) |
| `Content::Skew` | `skew()` | S | sim — variant trivial |
| Sides<Length> para margens | PageConfig completo | S | refactor de PageConfig |
| Header/footer/background/foreground em Page | Page model rico | M+ | sim — extensão estrutural |

**Bloqueante crítico identificado**: footnote area é a chave
para desbloquear `footnote()` em Model Fase 2 (já registado
em ADR-0060 Fase 2 Decisão 2).

---

## §4 — Arqueologia das ausências

Para cada feature `ausente`, classificação:

| Elemento | Razão da ausência | Classificação |
|----------|-------------------|---------------|
| footnote area | bloqueado por footnote (Model 154A); nunca materializado; pré-condição estrutural | **candidato Fase 1 sub-fase A** (alto valor: desbloqueia footnote) |
| pad | sem registo formal; granularidade trivial; nunca priorizado | candidato Fase 1 (trivial) |
| hide | sem registo formal; granularidade trivial | candidato Fase 1 (trivial) |
| pagebreak (manual) | implícito via overflow tem coberto até agora; manual nunca priorizado | candidato Fase 1 (trivial) |
| h/v spacing | sem registo formal (não no §A.5 inventário 148); nunca priorizado | candidato Fase 1 (trivial; comum em uso vanilla) |
| block | sem registo formal; granularidade fina | candidato Fase 2 (médio custo) |
| box | sem registo formal; granularidade fina | candidato Fase 2 (médio custo) |
| stack | sem registo formal; estrutural simples | candidato Fase 2 (médio custo) |
| columns | trabalho L+; column flow algorithm exige refactor profundo do Layouter | **DEBT-56** (Fase 3 condicional) |
| colbreak | depende de columns | DEBT-56 (com columns) |
| repeat | semantic lazy; menos usado fora de TOC dot leaders | candidato Fase 3 (baixo valor) |
| skew | feature visual menos comum; trivial | candidato Fase 3 (baixo valor) |
| measure / layout(callback) | depende de Introspection runtime; ADR-0017 (Passo 17) adiou | candidato post-ADR-0017 (não imediato) |

**Padrão**: a maioria das ausências (pad/hide/pagebreak/h/v/block/box/stack)
são **simples e nunca foram priorizadas** porque o trabalho
inicial focou-se em pipeline básico (paragraph + heading + figure
+ image + math). Não há ADRs de adiamento explícito; é dívida
implícita.

A excepção é `measure`/`layout` (depende de Introspection
adiada por ADR-0017) e `columns` (custo L+ com refactor do
Layouter).

---

## §5 — Crates externas necessárias

**Confirmado empiricamente**: o módulo Layout vanilla não
usa nenhuma crate externa específica de layout (cosmic-text,
taffy, harfbuzz). Apenas:
- `comemo` (já em L1).
- `smallvec` (já transitivamente).
- `ecow` (já em L1 — ADR-0024 + ADR-0035).
- `typst_utils` (in-tree).

Para o cristalino:
- **Page break heurísticas**: nenhuma crate. Algoritmo simples
  (cursor + reserve + new_page).
- **Column flow**: trabalho próprio. Sem dependência. Refactor
  do Layouter para multi-region.
- **Bidi** (já coberto): `unicode-bidi` já em deps cristalino.
- **Footnote area**: nenhuma crate. Trabalho próprio.

**Conclusão**: **nenhuma ADR de autorização de crate é
necessária para Layout Fase X**. Toda a Fase 1 e Fase 2 são
trabalho L1 puro com tipos existentes.

Se Fase 3 (columns) for materializada: continua sem crates
novas.

---

## §6 — Priorização proposta (matriz custo × valor)

```
              Alto valor              Médio valor            Baixo valor
S       [F1: pad, hide,           [F2: box, stack]       [F3: skew]
         pagebreak, h, v]
M       [F1: footnote area]      [F3: repeat]
M+      [F2: block]
L+      [F3: columns]
                                                          → DEBT-56
XL      [—]
```

### §6.1 Fase 1 — Sub-fase mínima que desbloqueia footnote (M+ agregado)

**Objectivo declarado**: desbloquear `footnote()` (Model Fase 2)
sem requerer todo o roadmap Layout.

5 features:

1. **Page model com footnote area** (M, alto valor — **crítico**):
   estende `Page` com `footnote_area: Vec<FrameItem>` (ou
   `Option<Frame>`). Layouter reserva espaço; footnote em Model
   Fase 2 popula via consumer dedicado.
2. **Pad** (S, alto valor — trivial): `Content::Pad` + stdlib
   `pad()` + consumer no Layouter (margem interna).
3. **Hide** (S, médio valor — trivial): `Content::Hide` + stdlib
   `hide()` + consumer (skip emit, manter cursor advance).
4. **Page break manual** (S, alto valor): `Content::Pagebreak`
   + stdlib `pagebreak()` + consumer (`Layouter::new_page()`).
5. **H/V spacing primitives** (S, alto valor): `Content::HSpace`
   + `Content::VSpace` + stdlib `h()`/`v()` + consumer (avance
   cursor sem emit).

**Aspiração de cobertura post-Fase 1**: 4/18 → 9/18 = **50%**
(implementado puro). Considerando `parcial`: ~55%.

**Footnote desbloqueado** após esta sub-fase (Model Fase 2 pode
abrir passo de footnote sem aguardar mais Layout).

### §6.2 Fase 2 — Containers (M+/L agregado)

3 features:

6. **Block** (M+, alto valor): `Content::Block` com atributos
   width, height, breakable, inset. Decisão: variant novo
   (semantic excede `Content::Styled`).
7. **Box** (S-M, médio valor): `Content::Box` (inline). Decisão
   por feature.
8. **Stack** (S-M, médio valor): `Content::Stack { dir, spacing,
   children }`. Variant novo (composição lateral).

**Aspiração post-Fase 2**: 9/18 → 12/18 = **67%**.

### §6.3 Fase 3 — Condicional (com DEBT)

5 features:

9. **Columns** (L+, alto valor mas complexo): `Content::Columns`
   + multi-region Layouter. **DEBT-56 aberto** (column flow
   Fase 3 Layout).
10. **Colbreak** (S, depende de columns): incluído com columns.
11. **Repeat** (M, baixo valor): `Content::Repeat` + lazy
    semantic.
12. **Skew** (S, baixo valor): `Content::Skew` ou via Transform.
13. **Refino** PageConfig (Sides<Length> margens; header/footer/
    background/foreground): M+, médio valor.

**Aspiração post-Fase 3**: 12/18 → 17-18/18 = **94-100%**.

`measure`/`layout(callback)`: condicional ao desbloqueio de
ADR-0017 (Introspection runtime).

---

## §7 — Plano de materialização

### §7.1 Sub-passos sugeridos

Numeração após renumeração da ADR-0060 (P156→P157,
P157→P158, P158→P159):

| Passo | Escopo | Features | ADR adicional? |
|-------|--------|----------|----------------|
| **P156C** | M+ agregado | Fase 1 Layout (page model footnote area + pad + hide + pagebreak + h + v) | — (aplica ADR-0061) |
| **P157** | M+ | Model Fase 2 table foundations (renumerado de P156) | — |
| **P158** | M | Model figure-kinds (renumerado de P157) | — |
| **P159** | XL | Model bibliography + cite | **ADR-0062** (autorização hayagriva; renumerada de ADR-0061) |
| **passo dedicado footnote** | M | Model Fase 2 footnote (desbloqueado por P156C) | — |
| **passo Fase 2 Layout** | M+ | block + box + stack | — |
| **passo Fase 3 Layout** | L+ | columns (com colbreak); fecha DEBT-56 | **ADR dedicada column flow** (futura) |
| **refino Page rico** | M+ | Sides<Length> margens; header/footer/background/foreground | — (aplica ADR-0061) |
| **passo skew + repeat** | S+M | features visuais menos comuns | — |

Numeração final dos passos pós-P156C fica para a **decisão
humana** + ADR-0061 + ordem de materialização real.

### §7.2 Regra `Content::Styled` vs variant novo (per ADR-0026)

| Feature | Recomendação | Razão |
|---------|--------------|-------|
| footnote area | **`Page::footnote_area`** (extensão de Page) | extensão estrutural; não Content variant |
| pad | **variant novo** (`Content::Pad { left, top, right, bottom, body }`) | atributos não reduzíveis a Style |
| hide | **variant novo** (`Content::Hide { body }`) | semantic distinta (display: none) |
| pagebreak | **variant novo** ou **`Style::PageBreak`** | decisão default: variant `Content::Pagebreak { weak: bool }` (semantic clara) |
| h / v | **variants novos** (`Content::HSpace { amount, weak }`, `Content::VSpace { amount, weak, attach }`) | atributos não reduzíveis a Style |
| block | **variant novo** | atributos múltiplos (width, height, breakable, inset, fill, stroke); excede Styled |
| box | **variant novo** | inline container com baseline; excede Styled |
| stack | **variant novo** | atributos `dir` + composição |
| columns | **variant novo** + **algoritmo dedicado** | column flow exige consumer especializado |
| repeat | **variant novo** | lazy semantic exige consumer dedicado |
| skew | **variant novo** ou via Transform existente | decisão por simplicidade |

Decisões finais ficam para cada passo de materialização;
ADR-0061 fornece guia.

### §7.3 Relação com ADRs existentes

- **ADR-0026 + ADR-0026-R1**: `Content` enum fechado;
  novos variants exigem nova entrada. ADR-0061 propõe
  ~10 variants novos no total.
- **ADR-0036**: atomização — cada feature tem consumer
  explícito.
- **ADR-0037**: coesão por domínio — Layout permanece em
  `01_core/src/rules/layout/` (módulo dedicado por
  sub-fase se >800 linhas).
- **ADR-0054**: perfil observacional graded — features
  Fase 1 cumprem com aproximações aceites (footnote area
  mínima sem column flow; pagebreak básico sem `to: Parity`).

---

## §8 — Sumário executivo

Layout (categoria 38% declarado per inventário 148 §A.5)
tem 17 entradas vanilla; após **reclassificação empírica P156B**
e **adição de h/v + skew** (não estavam no §A.5), são **18
entradas**. Cobertura empírica recalculada: **22% implementado
puro** (vs 38% declarado), com 3 entradas adicionais
`parcial` (place, grid, measure).

Diagnóstico revela:

- **5 entradas viáveis para Fase 1** (page model footnote area
  + pad + hide + pagebreak + h/v): trivial-a-médio em escopo
  agregado M+. **Desbloqueia footnote** em Model Fase 2 sem
  requerer todo o roadmap Layout.
- **3 entradas para Fase 2** (block + box + stack): médio
  custo. Cobertura post-Fase 2: ~67%.
- **5 entradas para Fase 3** (columns + colbreak + repeat
  + skew + refino Page rico): inclui `columns` que é trabalho
  L+ com refactor do Layouter — **DEBT-56 aberto**.

Ataque proposto: 1 passo grande (P156C, M+ agregado) eleva
cobertura para ~50% e desbloqueia footnote. Trabalho
restante distribui-se em 3-5 passos posteriores conforme
prioridade humana.

**Sem novas crates externas** — Layout é trabalho L1 puro
em todas as fases, incluindo column flow.

ADR-0061 (Layout roadmap, status `PROPOSTO`) é criada neste
passo. Reocupa o número (era reservado para `hayagriva`);
**hayagriva passa para ADR-0062**. ADR-0060 anotada com
renumeração Model Fase 2 (P156→P157, P157→P158, P158→P159).
DEBT-55 actualizada.

---

## §9 — Cross-references

- `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`
  (ADR criada por este passo).
- `00_nucleo/adr/typst-adr-0060-model-structural-roadmap.md`
  (anotada com renumeração).
- `00_nucleo/adr/README.md` (índice ADRs actualizado).
- `00_nucleo/DEBT.md` — DEBT-55 actualizada; **DEBT-56 aberto**.
- `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (Tabela A linha "Layout" reclassificada).
- `00_nucleo/diagnosticos/diagnostico-model-passo-154a.md`
  (precedente diagnóstico-primeiro a categoria).
- `00_nucleo/diagnosticos/historiograma-passos.md` (§4.1
  evidência 6/6 do padrão diagnóstico-primeiro).
- `00_nucleo/materialization/typst-passo-156b.md` (spec).
- `00_nucleo/materialization/typst-passo-156b-relatorio.md`
  (relatório).
- Vanilla source:
  `lab/typst-original/crates/typst-library/src/layout/`.
- Cristalino source: `01_core/src/entities/layout_types.rs`,
  `01_core/src/entities/content.rs`,
  `01_core/src/rules/layout/`,
  `01_core/src/rules/eval/mod.rs`.
