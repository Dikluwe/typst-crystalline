# Relatório de auditoria — P768

**Passo:** 768  
**Data:** 2026-07-15  
**Base:** P767c (linha de trabalho `cetz`/shapes fechada)  
**Foco:** Auditoria sistemática das classificações comportamentais de `rules.rs` do vanilla vs cristalino, priorizando por risco de repetir o padrão Shape (inline no cristalino / block no vanilla).  
**L0:** Não aplica — passo de sonda/auditoria; L0s afectados serão actualizados nos passos de correcção dedicados.  

---

## 1. Metodologia

1. Ler `lab/typst-original/crates/typst-layout/src/engine.rs` e classificar cada `_RULE` pelo tipo de `Content` que produz (`BlockElem`, `InlineElem`, `Sequence`/directo, condicional).
2. Verificar, em `01_core/src/engine/layout/mod.rs` e nos arquivos de feature correspondentes, como o cristalino trata cada equivalente.
3. Preencher a coluna "confirmado por sonda antes de implementar" com base nos prompts/relatórios dos passos originais (quando disponíveis).
4. Priorizar por risco: elementos que combinam frequentemente com texto no mesmo fluxo e cujo posicionamento vertical depende da classificação.
5. Medir uma amostra de 2–3 itens de maior risco com `mutool trace` (coordenadas, não só AE).

Nota: a versão do vanilla em `lab/typst-original/` não expõe um enum `Behaviour` no layout; a classificação comportamental é inferida do que a `ShowFn` devolve. A coluna "Behaviour" do material P768 é portanto mapeada para o tipo de layout resultante.

---

## 2. Tabela completa — `rules.rs` do vanilla

| Regra vanilla | Elemento | Classificação vanilla | Cristalino (actual) | Confirmado por sonda antes de implementar? |
|---|---|---|---|---|
| STRONG_RULE | StrongElem | inline (Styled/Text) | `Content::Styled` via eval → inline | Sim — P101 converteu explicitamente Strong/Emph para Styled. |
| EMPH_RULE | EmphElem | inline (Styled/Text) | `Content::Styled` via eval → inline | Sim — P101. |
| LIST_RULE | ListElem | block (`BlockElem::multi_layouter`) | `Content::ListItem` → `list_item::layout` (block) | Sim — vários passos de listas; flushing explícito. |
| ENUM_RULE | EnumElem | block (`BlockElem::multi_layouter`) | `Content::EnumItem` → `enum_item::layout` (block) | Sim — vários passos de enumeração. |
| TERMS_RULE | TermsElem | block (`StackElem` + padding) | `Content::Terms` → `terms::layout` (block) | Sim — P381 atomização. |
| LINK_MARKER_RULE | LinkMarker | inline (body) | Não existe como elemento separado; link é inline | N/A — cristalino trata `Content::Link` directamente. |
| LINK_RULE | LinkElem | inline (LinkMarker wrapper) | `Content::Link` → `link::layout` (inline) | Sim — P157/P381. |
| DIRECT_LINK_RULE | DirectLinkElem | inline (body linked) | Não existe como elemento separado | N/A — integrado em Link. |
| DIVIDER_RULE | DividerElem | block (`LineElem`) | `Content::Divider` → `divider::layout` (block) | Sim — P381; flushing explícito. |
| TITLE_RULE | TitleElem | block (`BlockElem`) | `Content::Title` → `title::layout` (block) | Sim — heading/title. |
| HEADING_RULE | HeadingElem | block (`BlockElem`) | `Content::Heading` → `heading::layout` (block) | Sim — vários passos de heading. |
| FIGURE_RULE | FigureElem | block (`BlockElem`; float opcional) | `Content::Figure` → `figure::layout` (block; float via Place) | Parcial — implementado em vários passos; paridade visual testada pontualmente. |
| FIGURE_CAPTION_RULE | FigureCaption | block (`BlockElem`) | Integrado em `figure::layout` | N/A — não é Content separado. |
| QUOTE_RULE | QuoteElem | condicional (`block=true` → block; else inline) | `Content::Quote` → `quote::layout` (condicional) | Sim — `quote.rs` trata `e.block` explicitamente. |
| FOOTNOTE_RULE | FootnoteElem | inline (superscript marker) | `Content::Footnote` → `footnote::layout` (inline) | Sim — P326. |
| FOOTNOTE_ENTRY_RULE | FootnoteEntry | block/sequence | Não existe como Content separado no cristalino | N/A — gestão de rodapé diferente. |
| OUTLINE_RULE | OutlineElem | sequence de entries | `Content::Outline` → `outline::layout_outline` | Sim — P323/P381. |
| OUTLINE_ENTRY_RULE | OutlineEntry | block (`BlockElem`) | Gerado internamente em outline | N/A — não é Content separado. |
| REF_RULE | RefElem | inline (realize) | `Content::Ref` → `ref::layout` (inline) | Sim — P462. |
| CITE_GROUP_RULE | CiteGroup | inline (realize) | `Content::Cite` → `cite::layout` (inline) | Sim — P468. |
| BIBLIOGRAPHY_RULE | BibliographyElem | block (`BlockElem` ou grid) | `Content::Bibliography` → `bibliography::layout` (block) | Sim — bibliografia. |
| CSL_LIGHT_RULE | CslLightElem | inline (Text delta) | Não implementado como Content separado | N/A — CSL não migrado. |
| CSL_INDENT_RULE | CslIndentElem | inline/wrapper (`PadElem`) | Não implementado como Content separado | N/A — CSL não migrado. |
| TABLE_RULE | TableElem | block (`BlockElem::multi_layouter`) | `Content::Table` → `table::layout` (block) | Sim — P224/P381. |
| TABLE_CELL_RULE | TableCell | inline/wrapper (`show_cell`) | `Content::TableCell` → `table_cell::layout` (inline/wrapper) | Sim — células não quebram parágrafo. |
| SUB_RULE | SubElem | inline (Text shift) | `Content::Styled` via eval → inline | Sim — P101/P381. |
| SUPER_RULE | SuperElem | inline (Text shift) | `Content::Styled` via eval → inline | Sim — P101/P381. |
| UNDERLINE_RULE | UnderlineElem | inline (Text deco) | `Content::Underline` → trata como Styled/Text deco inline | Sim — decorações. |
| OVERLINE_RULE | OverlineElem | inline (Text deco) | `Content::Overline` → inline | Sim — decorações. |
| STRIKE_RULE | StrikeElem | inline (Text deco) | `Content::Strike` → inline | Sim — decorações. |
| HIGHLIGHT_RULE | HighlightElem | inline (Text deco) | `Content::Styled` com highlight → inline | Sim — P101. |
| SMALLCAPS_RULE | SmallcapsElem | inline (Text feature) | `Content::SmallCaps` → inline | Sim — P381. |
| RAW_RULE | RawElem | condicional (`block=true` → block; else inline) | `Content::Raw` → `raw::layout` (condicional) | Sim — `raw.rs` trata `e.block` explicitamente. |
| RAW_LINE_RULE | RawLine | inline (body) | Não existe como Content separado | N/A — integrado em Raw. |
| ALIGN_RULE | AlignElem | wrapper (aligned body) | `Content::Align` → `align::layout` (wrapper) | Sim — P156/P381. |
| PAD_RULE | PadElem | block (`BlockElem::multi_layouter`) | `Content::Pad` → `pad::layout` (block) | Sim — P156C. |
| COLUMNS_RULE | ColumnsElem | block (`BlockElem::multi_layouter`) | `Content::Columns` → `columns::layout` (block) | Sim — P217/P537b. |
| STACK_RULE | StackElem | block (`BlockElem::multi_layouter`) | `Content::Stack` → `stack::layout` (block) | Sim — P381. |
| GRID_RULE | GridElem | block (`BlockElem::multi_layouter`) | `Content::Grid` → `grid::layout` (block) | Sim — P224/P381. |
| GRID_CELL_RULE | GridCell | inline/wrapper (`show_cell`) | `Content::GridCell` → `grid_cell::layout` (inline/wrapper) | Sim — P224. |
| MOVE_RULE | MoveElem | block (`BlockElem::single_layouter`) | `Content::Transform`? / não mapeado directo | Move/Scale/Rotate/Skew/Repet mapeados para `Content::Transform` ou não implementados isoladamente. |
| SCALE_RULE | ScaleElem | block (`BlockElem::single_layouter`) | `Content::Transform` | Ver Move. |
| ROTATE_RULE | RotateElem | block (`BlockElem::single_layouter`) | `Content::Transform` | Ver Move. |
| SKEW_RULE | SkewElem | block (`BlockElem::single_layouter`) | `Content::Transform` | Ver Move. |
| REPEAT_RULE | RepeatElem | block (`BlockElem::single_layouter`) | `Content::Repeat` → `repeat::layout` (block) | Sim — P381. |
| HIDE_RULE | HideElem | inline/wrapper (body hidden) | `Content::Hide` → `hide::layout` (wrapper) | Sim — P381. |
| LAYOUT_RULE | LayoutElem | block (`BlockElem::multi_layouter`) | Não implementado (`#layout`) | N/A — não migrado. |
| IMAGE_RULE | ImageElem | block (`BlockElem::single_layouter`) | `Content::Image` → `image::layout` (block) | **Não** — implementado em passos antigos sem medição de ancoramento vertical com texto; **achado de alto risco** (ver §4). |
| LINE_RULE | LineElem | block (`BlockElem::single_layouter`) | `Content::Shape` (ShapeKind::Line) → `shape::layout` (block) | **Não** — corrigido indirectamente por P767c; antes estava inline. |
| RECT_RULE | RectElem | block (`BlockElem::single_layouter`) | `Content::Shape` → `shape::layout` (block) | **Não** — corrigido em P767a/P767c. |
| SQUARE_RULE | SquareElem | block (`BlockElem::single_layouter`) | `Content::Shape` → `shape::layout` (block) | **Não** — corrigido em P767c. |
| ELLIPSE_RULE | EllipseElem | block (`BlockElem::single_layouter`) | `Content::Shape` → `shape::layout` (block) | **Não** — corrigido em P767c. |
| CIRCLE_RULE | CircleElem | block (`BlockElem::single_layouter`) | `Content::Shape` → `shape::layout` (block) | **Não** — corrigido em P767c. |
| POLYGON_RULE | PolygonElem | block (`BlockElem::single_layouter`) | `Content::Shape` → `shape::layout` (block) | **Não** — corrigido em P767c. |
| CURVE_RULE | CurveElem | block (`BlockElem::single_layouter`) | `Content::Curve` → `curve::layout` (block) | Parcial — `curve.rs` faz `flush_line` e avança cursor, mas não aplica `above`/`below` nem trata ancoramento depois de texto. |
| EQUATION_RULE | EquationElem | condicional (`block=true` → block; else inline) | `Content::Equation` → `layout_equation_arm` (condicional) | Sim — equações inline vs block são tratadas. |
| ATTACH_RULE | AttachElem | empty | Não implementado | N/A — PDF attachment. |
| ARTIFACT_RULE | ArtifactElem | inline (body) | Não existe como Content separado | N/A — tags de acessibilidade PDF. |
| PDF_MARKER_TAG_RULE | PdfMarkerTag | inline (body) | Não existe como Content separado | N/A — tags de acessibilidade PDF. |

**Resumo da classificação:**
- **Block no vanilla e no cristalino:** List, Enum, Terms, Divider, Title, Heading, Figure, Table, Pad, Columns, Stack, Grid, Repeat, Image, Line, Rect, Square, Ellipse, Circle, Polygon, Curve, Equation(block).
- **Inline no vanilla e no cristalino:** Strong, Emph, Link, Footnote, Ref, Cite, Sub, Super, Underline, Overline, Strike, Highlight, SmallCaps, Raw(inline), Align, Hide.
- **Condicional (block/inline) no vanilla e no cristalino:** Quote, Raw, Equation.
- **Não implementados / não aplicáveis:** CSL_LIGHT/INDENT, LAYOUT_RULE, Attach/Artifact/PdfMarkerTag.

---

## 3. Priorização por risco

Critérios de risco:
1. Combina frequentemente com texto no mesmo fluxo.
2. A classificação afecta o posicionamento vertical (block vs inline).
3. Não foi confirmado por sonda antes da implementação.

| Elemento | Risco | Justificação |
|---|---|---|
| `Image` | **Alto** | Block no vanilla; cristalino trata como block, mas sem aplicar `above`/`below` nem ancorar na grelha de linhas. Mesmo padrão de P767c. |
| `Curve` | **Médio-Alto** | Block no vanilla; cristalino faz `flush_line` e avança cursor, mas sem `above`/`below` nem ancoramento correto depois de texto. |
| `Figure` | **Médio** | Block/float no vanilla; cristalino block/float. A classificação está correcta, mas float dentro de parágrafo pode precisar de revisão. |
| `Table`/`Grid` | **Baixo-Médio** | Block no vanilla e no cristalino; divergências conhecidas são em sub-layouts (células), não no fluxo principal. |
| Quote/Raw/Equation | **Baixo** | Classificação condicional já é tratada explicitamente no cristalino; medição inline confirmou alinhamento. |

---

## 4. Amostra medida — itens de maior risco

Documentos gerados em `temp_p768/`.

### 4.1 `A #image("tiny.png", width: 2cm, height: 1.5cm) B` — achado de alto risco

| Elemento | Vanilla Y (pt) | Cristalino Y (pt) | ΔY (pt) |
|---|---|---|---|
| "A" baseline | 78,104 | 78,105 | +0,001 |
| `image` inferior | 44,533 | 49,974 | +5,441 |
| `image` superior | 89,887 | 92,493 | +2,606 |
| "B" baseline | 154,262 | 135,012 | −19,250 |

**Interpretação:** o cristalino coloca a imagem logo após o `flush_line` (na baseline seguinte a "A") e avança apenas pela altura da imagem. O vanilla aplica o espaçamento de bloco (`above` + `below`) e ancora a imagem na grelha de linhas. O desvio de ~19 pt em "B" é exactamente o mesmo padrão que `Content::Shape` tinha antes de P767c. Nota: a escala da imagem também difere ligeiramente (vanilla 45,35 pt vs cristalino 42,52 pt para `height: 1.5cm`), mas isso é um problema separado de resolução de imagem; o foco do achado é o ancoramento vertical no fluxo de texto.

**Decisão:** corrigir em passo dedicado (P769 ou numerção atribuída), replicando a lógica P767c para `Content::Image`.

### 4.2 `A #quote(block: false)[x] B` — inline quote

| Elemento | Vanilla Y (pt) | Cristalino Y (pt) | ΔY (pt) |
|---|---|---|---|
| "A" baseline | 78,104 | 78,105 | +0,001 |
| "B" baseline | 78,104 | 78,105 | +0,001 |

**Interpretação:** quote inline fica na mesma linha de texto; alinhamento vertical correcto.

### 4.3 `A #raw("x") B` — inline raw

| Elemento | Vanilla Y (pt) | Cristalino Y (pt) | ΔY (pt) |
|---|---|---|---|
| "A" baseline | 78,104 | 78,105 | +0,001 |
| "B" baseline | 78,104 | 78,105 | +0,001 |

**Interpretação:** raw inline fica na mesma linha; alinhamento vertical correcto.

### 4.4 `A $x$ B` — inline equation

| Elemento | Vanilla Y (pt) | Cristalino Y (pt) | ΔY (pt) |
|---|---|---|---|
| "A" baseline | 78,104 | 78,378 | −0,274 |
| "B" baseline | 78,104 | 78,378 | −0,274 |

**Interpretação:** a linha toda está deslocada ~0,27 pt, provavelmente por diferença na métrica matemática/fonte, não por classificação errada. A e B mantêm-se alinhados entre si.

---

## 5. Decisões registadas (regra 1)

| Achado | Classificação | Decisão |
|---|---|---|
| `Image` trata-se como block mas sem `above`/`below` nem ancoramento na grelha de linhas. | Divergência confirmada (mesmo padrão P767c) | **Corrigir em passo dedicado** — aplicar lógica P767c a `Content::Image`. |
| `Curve` faz `flush_line` mas sem `above`/`below` nem ancoramento depois de texto. | Provável divergência (não medida com texto neste passo) | **Backlog priorizado** — medir `A #curve(...) B` no passo de Image; se confirmado, corrigir conjuntamente. |
| `Quote`/`Raw`/`Equation` condicionais estão correctos. | Já correcto | **Não corrigir**. |
| `Figure`/`Table`/`Grid` block estão correctos no fluxo principal. | Sem divergência visível | **Backlog baixa prioridade** — só reabrir se medição futura indicar desvio. |
| `Move`/`Scale`/`Rotate`/`Skew` mapeados para `Content::Transform` no cristalino. | Implementação diferente; classificação block preservada | **Não corrigir** agora — `Transform` já quebra parágrafo. |
| `CSL_LIGHT`/`CSL_INDENT`/`LAYOUT_RULE`/`ATTACH`/`ARTIFACT`/`PDF_MARKER_TAG` não implementados. | N/A | **Scope-out** — CSL e PDF markers não são prioridade. |

---

## 6. Validação automática

```bash
cargo test --workspace
```

- **Resultado:** todas as suites passaram (sem alterações de código neste passo, apenas auditoria).

```bash
crystalline-lint .
```

- **Resultado:** zero violações excepto `V7` (prompt órfão pré-existente).

---

## 7. Próximo passo

- Abrir passo dedicado para corrigir `Content::Image` (ancoramento vertical depois de texto, replicando P767c).
- No mesmo passo, medir `Content::Curve` com texto; se divergir, corrigir conjuntamente.
- Actualizar o L0 correspondente (`00_nucleo/prompts/engine/layout/image.md` ou criar novo) antes de escrever código, conforme Protocolo de Nucleação.
