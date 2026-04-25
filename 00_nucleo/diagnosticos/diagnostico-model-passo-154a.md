# Diagnóstico Model (structural) — Passo 154A

**Data**: 2026-04-25
**Vanilla snapshot**: `lab/typst-original/` em commit
`ba61529986e0a5a916cbf937c3c65117cd450683`.
**Cristalino snapshot**: Passo 153; 59 ADRs; 12 DEBTs abertos;
inventário 148 actualizado por P149 (cobertura
arquitectural 72%).
**Output**: ADR-0060 (PROPOSTO) com roadmap Fase 1 / 2 / 3;
DEBT-55 (bibliography + cite XL).

---

## 1. Inventário detalhado (vanilla `model/`)

22 ficheiros em `lab/typst-original/crates/typst-library/src/model/`:

| Ficheiro | Elementos `#[elem]` | Atributos públicos principais |
|----------|---------------------|-------------------------------|
| `asset.rs` | `AssetElem` | path, alt, scaling |
| `bibliography.rs` | `BibliographyElem`, `BibliographyEntryElem` | path/sources, style, title, full |
| `cite.rs` | `CiteElem` | key, supplement, form, style |
| `divider.rs` | `DividerElem` | (no attrs; structural) |
| `document.rs` | `DocumentElem` | title, author, keywords, date |
| `emph.rs` | `EmphElem` | body |
| `enum.rs` | `EnumElem`, `EnumItemElem` | tight, numbering, start, full, body |
| `figure.rs` | `FigureElem`, `FigureCaptionElem` | body, kind, caption, supplement, numbering, gap, placement |
| `footnote.rs` | `FootnoteElem`, `FootnoteEntryElem` | body, numbering, separator |
| `heading.rs` | `HeadingElem` | level, depth, offset, body, numbering, supplement, outlined |
| `link.rs` | `LinkElem` | dest, body |
| `list.rs` | `ListElem`, `ListItemElem` | tight, marker, indent, body |
| `numbering.rs` | (function `numbering`) | pattern, args |
| `outline.rs` | `OutlineElem`, `OutlineEntryElem` | title, target, depth, indent, fill |
| `par.rs` | `ParElem`, `ParBreakElem` | leading, spacing, justify, linebreaks, first-line-indent, hanging-indent |
| `quote.rs` | `QuoteElem` | body, attribution, block, quotes |
| `reference.rs` | `RefElem` | target, supplement, form |
| `strong.rs` | `StrongElem` | body, delta |
| `table.rs` | `TableElem`, `TableCellElem`, `TableHeaderElem`, `TableFooterElem`, `TableHLineElem`, `TableVLineElem` | columns, rows, gutter, fill, stroke, align, body |
| `terms.rs` | `TermsElem`, `TermItemElem` | tight, separator, indent, hanging-indent, body |
| `title.rs` | `TitleElem` | (depende de DocumentElem) |

**Total elementos vanilla**: ~28 (incluindo sub-elementos
table, list, enum, terms).

---

## 2. Estado actual em cristalino

Probe empírico: `01_core/src/entities/content.rs` +
`01_core/src/rules/eval/mod.rs::make_stdlib`.

| Vanilla element | Cristalino estado | Variant `Content::*` | Stdlib func | Referência canónica |
|-----------------|-------------------|---------------------|-------------|---------------------|
| `HeadingElem` | `implementado⁺` | `Heading {level, body}` | `native_heading` | Passos 22, 99, 103 (ADR-0041) |
| `EmphElem` | `implementado` | desugar → `Content::Styled([Italic(true)])` | `native_emph` | Passo 101 (ADR-0026) |
| `StrongElem` | `implementado` | desugar → `Content::Styled([Bold(true)])` | `native_strong` | Passo 101 (ADR-0026) |
| `RefElem` | `implementado⁺` | `Ref {...}` + `Labelled` | (sintaxe `@`) | Passos 63–66 |
| `OutlineElem` | `implementado` | `Outline` | (no native_*; via heading sintaxe) | Passos 65–66 |
| `FigureElem` | `implementado⁺` | `Figure {body, caption}` | `native_figure` | Passos 75, DEBT-14/15 |
| `LinkElem` | `parcial` | `Link {url, body}` | (parse `[]()`) | Passo 23 |
| `ListElem` + `ListItemElem` | `parcial` | `ListItem(body)` | (sintaxe `-`) | Passo 23 |
| `EnumElem` + `EnumItemElem` | `parcial` | `EnumItem {number, body}` | (sintaxe `+`/`1.`) | Passo 23 |
| `ParElem` | `parcial` | sem variant; `leading` capturado em `StyleDelta` | (sintaxe `\n\n`) | Passo 138 (DEBT-52 fase B) |
| `BibliographyElem` | `ausente` | — | — | — |
| `CiteElem` | `ausente` | — | — | — |
| `FootnoteElem` | `ausente` | — | — | — |
| `QuoteElem` | `ausente` | — | — | — |
| `TermsElem` + `TermItemElem` | `ausente` | — | — | — |
| `TableElem` + sub-elementos | `ausente` | — (`Content::Grid` parcial cobre layout, não modelo de table) | — | — |
| `DocumentElem` | `ausente` | — (sem metadata wrapper) | — | — |
| `DividerElem` | `ausente` | — | — | — |
| `AssetElem` | `ausente` | — | — | — |
| `TitleElem` | `ausente` | — | — | — |
| `FigureCaptionElem` | `parcial` (inline em `Figure.caption`) | (sub-campo de `Figure`) | — | Passo 75 |
| `numbering` (func) | `implementado⁺` | (`SetHeadingNumbering`, `SetFigureNumbering`) | (numbering pattern em stdlib parcial) | Passos 75, 99 |

### 2.1 — Recálculo da contagem Model

Inventário 148 §A.6 listava 21 entradas com 4/4/5/8/0
(impl/impl⁺/parcial/ausente/scope-out). Diagnóstico empírico
ajusta:

| Estado | Contagem | Lista |
|--------|----------|-------|
| `implementado` | 3 | heading (era impl⁺), emph, strong, outline (4 se contar emph/strong ambos) |
| `implementado⁺` | 4 | heading (re-classificado), figure, ref, numbering |
| `parcial` | 5 | link, list, enum, par, caption (inline) |
| `ausente` | 10 | bibliography, cite, footnote, quote, terms, table, document, divider, asset, title |
| `scope-out` | 0 | — |
| **Total** | **22** | (ajustado de 21 — `caption` separada) |

Cobertura `(impl + impl⁺) / total` = **(3+4)/22 = 32%**
ou **(4+4)/22 = 36%** dependendo se classifica `heading`
como `implementado` ou `implementado⁺`. Inventário 148
indicava 38% — **revisão para baixo**.

---

## 3. Tipos arquitecturais bloqueantes

Para cada `parcial` ou `ausente` em §2:

| Bloqueante | Quem precisa | Custo estimado | Decisão arquitectural? |
|------------|--------------|----------------|------------------------|
| `Content::Table` | `table`, `figure(kind=table)` | M | sim — variant nova; ADR-0026 perfil aceita |
| Cell layouting | `table` (rendering) | M+ | DEBT-34d/e tocam grid cells; trabalho similar mas mais elaborado para table |
| `Content::Footnote` | `footnote` | M | sim — variant ou Styled? |
| Page model footnote area | `footnote` (rendering) | M-L | sim — exige Layout Fase X (categoria 38%) |
| `Content::Quote` | `quote` | S-M | sim — atributos `attribution`, `block`, `quotes` |
| `Content::Terms` + `Content::TermItem` | `terms` | S | sim — variant simples |
| `Content::Bibliography` + `Content::Cite` | `bibliography`, `cite` | XL | sim — exige ADR de autorização `hayagriva` (crate externa) |
| CSL parser | `cite` (style) | XL | sim — `hayagriva` provê |
| `Content::Document` | `document` (metadata wrapper) | S-M | provavelmente desnecessário se PDF metadata for emitida directamente em export |
| `Content::Divider` | `divider` | S | trivial; structural |
| `Content::Asset` | `asset` | S-M | imagens já cobertas via `Image`; `asset` adiciona alt-text + scaling |
| `Content::Title` | `title` | depende de `Document` | adiar |
| Numbering rules ricas | `figure` kinds, `heading` numbering | M | numbering pattern stdlib parcial — passo dedicado |
| `#show` selectors regex/where | `cite`, `bibliography`, refinements | M+ | depende de `regex` em L1 (gap 8 DEBT-52, ADR-0054bis condicional) |

---

## 4. Arqueologia das ausências

Para cada `ausente`, classificação per critério P149:

| Elemento | Razão | Classificação |
|----------|-------|---------------|
| `bibliography` | ADR-0017 (estratégia typst-library); exige `hayagriva` + CSL | adiamento priorizável (Fase 2; ADR de autorização) |
| `cite` | depende de `bibliography` | adiamento priorizável (mesma Fase 2) |
| `footnote` | bloqueado por Page model footnote area (Layout) | adiamento condicional; depende de Layout Fase X |
| `quote` | sem registo de razão; estrutural simples | sem priorização explícita; candidato Fase 1 |
| `terms` | sem registo; estrutural simples | sem priorização explícita; candidato Fase 1 |
| `table` | DEBT-34d/e abertos para grid cells; table requer mais (header/footer/spans) | adiamento priorizável (Fase 2) |
| `document` | metadata PDF emitida directamente; sem necessidade actual | divergência intencional (cristalino emite metadata em export PDF sem wrapper Content) |
| `divider` | sem registo; trivial | candidato Fase 1 |
| `asset` | imagens cobertas por `Image`; `asset` adiciona alt-text | candidato Fase 3 (acessibilidade) |
| `title` | depende de `document`; sem necessidade actual | divergência intencional |

---

## 5. Crates externas necessárias

| Elemento | Crates necessárias | Em cache? | Custo licença / autorização |
|----------|---------------------|-----------|-----------------------------|
| `bibliography` | `hayagriva` 0.9.1 | **sim** (per probe P152) | requer **ADR-0061** autorização (similar a ADR-0024 ecow, ADR-0023 indexmap, ADR-0057 hypher) |
| `cite` | (depende de hayagriva) | — | — |
| `footnote` | nenhuma específica | — | nenhuma autorização externa |
| `quote` | nenhuma | — | nenhuma |
| `terms` | nenhuma | — | nenhuma |
| `table` | nenhuma | — | nenhuma |
| `document` | nenhuma | — | nenhuma |
| `divider` | nenhuma | — | nenhuma |
| `asset` | nenhuma | — | nenhuma |
| `title` | nenhuma | — | nenhuma |

**Conclusão**: apenas `bibliography` + `cite` exigem crate
externa nova (`hayagriva`). Restantes podem ser
materializados com infra existente.

---

## 6. Priorização proposta (matriz custo × valor)

```
              Alto valor          Médio valor          Baixo valor
S       [F1: terms,                                    [F3: asset,
         divider]                                       title]
M       [F1: footnote*]    [F1: quote]                 [F2: document*]
M+      [F2: table]                                    [F3: stroke-obj]
L       [F2: figure-kinds]                             [—]
XL      [F2: bibliography                              [—]
         + cite]
```

`*` = tem dependência externa que pode reclassificar.

### 6.1 — Fase 1 proposta

Features de **alto valor + custo S/M** sem deps externas
nem dependências condicionais:

1. **`terms`** (S, alto valor — listas de definições em
   texto técnico).
2. **`quote`** (M, médio valor — citação como construção
   estrutural).
3. **`divider`** (S, médio valor — `---` markup syntactic).

**`footnote`** sai da Fase 1 porque depende de page model
(área reservada para notas no rodapé), trabalho de Layout
Fase X. Volta para Fase 2 condicional.

**Aspiração de cobertura post-Fase 1**: 8/22 → 11/22 = **50%**.

### 6.2 — Fase 2 (proposta)

Features de **valor alto** com complexidade ou deps:

4. **`table`** + `Content::Table` (M+, alto valor — bases
   reaproveitam `Content::Grid` parcial; sub-elementos
   cell/header/footer adicionam complexidade).
5. **`figure` kinds** (depende de table para figure-table).
6. **`bibliography` + `cite`** + **ADR-0061** autorização
   `hayagriva` (XL — passo dedicado).
7. **`footnote`** (depende de Layout Fase X — page model).

**Aspiração de cobertura post-Fase 2**: 11/22 → 15/22 = **68%**.

### 6.3 — Fase 3 (condicional)

Features de **baixa prioridade** ou divergências aceitáveis:

8. **`asset`** (acessibilidade — alt-text + scaling).
9. **`document`** + **`title`** (metadata wrapper —
   divergência intencional cristalino emite metadata
   directamente).
10. Outras se priorização humana mudar.

**Aspiração total**: 15/22 → 17-18/22 = **77-82%** com
restantes em scope-out.

---

## 7. Plano de materialização

### 7.1 — Sub-passos sugeridos

1. **Passo 154B** — `terms` + `divider` (S agregado;
   variants simples; sintaxe markup quando aplicável).
2. **Passo 155** — `quote` (M; structural; atributos
   `attribution`, `block`).
3. **Passo 156** — `Content::Table` foundations (M; variant
   nova + cell/header/footer; **não** depende de footnote).
4. **Passo 157** — `figure` kinds extension (depende de
   156).
5. **ADR-0061 + Passo 158** — `bibliography` + `cite`
   (XL; passo dedicado com ADR autorização hayagriva).
6. **Passo dedicado para `footnote`** — quando Layout
   page-model area for priorizado (independente de Model
   Fase 1).
7. Restantes (`asset`, `document`, `title`, etc.) — Fase 3
   condicional.

### 7.2 — Regra Content::Styled vs variant novo (ADR-0026)

Para cada feature da Fase 1/2:

| Feature | Recomendação | Razão |
|---------|--------------|-------|
| `terms` + `term_item` | variant novo (`Terms`, `TermItem`) | semântica distinta; estrutura aninhada |
| `divider` | variant novo (`Divider`) | sem body; sem styling associado |
| `quote` | variant novo (`Quote {body, attribution, block}`) | atributos não reduzíveis a `Style` |
| `table` + sub-elementos | variants novos (`Table`, `TableCell`, `TableHeader`, `TableFooter`) | estrutura complexa |
| `bibliography` | variant novo (`Bibliography`) | invoca CSL |
| `cite` | variant novo (`Cite`) | atributos `key`, `supplement`, `form` |
| `footnote` | variant novo (`Footnote {body}`) | semântica distinta; numbering implícito |
| `asset` | recurse para `Image` ou variant novo | depende da forma de alt-text desejada |

`Content::Styled` é **inadequado** para Model structural —
estes elementos têm semântica que excede styling.

### 7.3 — Relação com ADRs existentes

- **ADR-0026 + ADR-0026-R1**: `Content` enum fechado;
  novos variants exigem nova entrada. ADR-0060 propõe.
- **ADR-0036**: atomização — cada nova feature tem
  consumer explícito em layout/eval.
- **ADR-0037**: coesão por domínio — `Content` permanece
  em `entities/content.rs`; sub-modulos podem agrupar
  rendering por categoria.
- **ADR-0054**: perfil observacional graded — features
  Fase 1 cumprem com aproximações aceites.

---

## 8. Resumo executivo

Model (structural) tem cobertura empírica **32-36%** (revisão
para baixo do 38% declarado em inventário 148). Diagnóstico
revela **22 elementos** vanilla, dos quais cristalino tem 7-8
materializados, 5 parciais, e 10 ausentes.

**Ataque proposto** (ADR-0060 PROPOSTO):

- **Fase 1** (3 sub-passos: 154B + 155 + 156A): `terms` +
  `divider` + `quote`. Eleva cobertura para ~50%. Sem novas
  crates; sem dependências condicionais.
- **Fase 2** (3 sub-passos: 156B + 157 + ADR-0061+158):
  `table` + `figure` kinds + `bibliography`/`cite`. Inclui
  ADR-0061 para autorizar `hayagriva` (já em cache local
  per P152). Eleva cobertura para ~68%.
- **Fase 3** (condicional): `asset`, `document`, `title` —
  trabalho opcional / divergência intencional.

**Trabalho restante** (~5-7 entradas) é condicional ou XL;
registado no roadmap mas não obriga a executar. **DEBT-55
aberto** especificamente para `bibliography` + `cite`
(escopo XL; pré-condição ADR-0061).

**Footnote** sai da Fase 1 por depender de Layout Fase X
(page-model footnote area) — passo dedicado quando Layout
for priorizado, independente do roadmap Model.

`Content::Styled` é **inadequado** para Model structural;
todas as Fase 1/2 features exigem variants novos no
`Content` enum (per ADR-0026 perfil).

**Pós-roadmap completo**: cobertura Model ~75-80%, alinhada
com a ambição declarada do projecto sem comprometer
ADR-0017 (estratégia gradual).
