# Diagnóstico Model Fase 2 — Passo P157

Inventário diagnóstico precedendo materialização de "table
foundations" per **ADR-0060 Decisão 1 sub-passo 3** (renumerado
de P156 para P157 em P156B). Aplicação directa de **ADR-0065
critério #5** (scope determinado por inventário) — **primeira
aplicação concreta do critério #5**, complementando P156L
(critério #3). **Décima primeira aplicação consecutiva** do
padrão diagnóstico-primeiro.

---

## §1 — ADR-0060 leitura e resumo

**Localização**: `00_nucleo/adr/typst-adr-0060-model-structural-roadmap.md`

**Título**: ADR-0060 — Model (structural) roadmap — Fase 1 +
Fase 2 + Fase 3.

**Status**: `IMPLEMENTADO` (Fase 1 fechada em P155 — terms +
divider em P154B; quote em P155). Fase 2 e Fase 3 prosseguem
como roadmap planeado — não exigem re-abertura da ADR.

### §1.1 Definição de fases

#### Fase 1 — S+M; sem novas crates

| Sub-passo | Features | Estado |
|-----------|----------|--------|
| P154B | `Content::Terms`, `Content::TermItem`, `Content::Divider` | ✓ implementado |
| P155 | `Content::Quote { body, attribution, block, quotes }` | ✓ implementado |
| **P157** (renumerado de P156 em P156B) | **Table foundations**: `Content::Table` variant nova + sub-elementos `TableCell`, `TableHeader`, `TableFooter` (M+; reaproveita `Content::Grid` parcial para layout) | **pendente — escopo de P157** |

**Cobertura post-Fase 1 declarada**: ~50% (8/22 → 11-12/22).

#### Fase 2 — com ADR de autorização

| Sub-passo | Features | ADR adicional | Estado |
|-----------|----------|---------------|--------|
| P158 | `figure` kinds extension (depende de P157 para figure-table) | — | pendente; depende de P157 |
| ADR-0062 + P159 | `Content::Bibliography` + `Content::Cite` com `hayagriva` | ADR-0062 (reservada) | DEBT-55 EM ABERTO |
| (sem identificador fixo) | `Content::Footnote` desbloqueado por Fase 1 Layout (ADR-0061) | — | scope-out per decisão humana 2026-04-25 |

**Cobertura post-Fase 2**: ~68% (11-12/22 → 15-16/22).

#### Fase 3 — condicional / divergência intencional

- `asset`: alt-text + scaling sobre `Image`; acessibilidade.
- `document` / `title`: divergência intencional cristalino
  (export PDF directo; sem wrapper Content).

### §1.2 Definição literal de "table foundations"

ADR-0060 §"Decisão 1 — Fase 1 sub-passo 3" e tabela §"Plano de
materialização" linha P157 declaram literalmente:

> **`Content::Table` foundations: variant nova + sub-elementos
> `TableCell`, `TableHeader`, `TableFooter` (M+; reaproveita
> `Content::Grid` parcial para layout).**

**Subset declarado** (literal):
- 4 variants Content novas: `Table`, `TableCell`, `TableHeader`,
  `TableFooter`.
- Reaproveitamento de `Content::Grid` para layout.
- Tamanho declarado: **M+**.
- Sem ADR adicional necessária.

**Subset NÃO declarado em ADR-0060** (decisão local de scope):
- `TableHLine` / `TableVLine` (cosmetic — linhas internas).
- Atributos `align`, `fill`, `stroke`, `inset` (per-cell ou
  per-table).
- `gutter` / `column_gutter` / `row_gutter`.
- `summary` (acessibilidade).
- `repeat` real para Header/Footer em page breaks (depende
  multi-region).

### §1.3 Decisão 4 ADR-0060 (variant novo vs Styled)

ADR-0060 declara: para Fase 1/2 — **variant novo**, não
`Content::Styled`. Razão: Model structural tem semântica que
excede styling (cells, headers, footers).

**Aplicável a P157**: 4 variants novos (Table/TableCell/
TableHeader/TableFooter) confirmado per Decisão 4.

---

## §2 — Estado de Model em código

### §2.1 Variants `Content::*` relacionadas com Model — inventário factual

Inspecção de `01_core/src/entities/content.rs`:

**Implementados** (Model structural):
- `Heading { level, body }` — implementado (Passos 22, 99, 103).
- `Figure { body, caption, kind, numbering }` — implementado⁺
  (Passos 75, ADR-0041). **`kind: "table"` slot existe** —
  preparado para `figure-table` em P158.
- `Outline` — implementado (Passos 65–66).
- `Ref { target }` — implementado⁺ (Passos 63–66).
- `Labelled { target, label }` — implementado.
- `Link { url, body }` — parcial (sem render visual).
- `Divider` — implementado (P154B).
- `Terms { items }`, `TermItem { term, description }` —
  implementado (P154B).
- `Quote { body, attribution, block, quotes }` — implementado
  (P155).

**Auxiliares**:
- `Grid { columns: Vec<TrackSizing>, rows: Vec<TrackSizing>,
  cells: Vec<Content> }` — parcial (Passos 80, 82–84.6).
  **Sem TableCell estruturado**; cells distribuídas linearmente
  via `idx % num_cols`.
- `SetFigureNumbering { pattern }` — implementado (Passo 75).
- `CounterDisplay`, `CounterUpdate`, `SetHeadingNumbering` —
  implementados (Passos 56–58).

**Ausentes** (Model alto-valor):
- `Content::Table` — **factualmente confirmado ausente**
  (zero código).
- `Content::TableCell` — factualmente confirmado ausente.
- `Content::TableHeader` — factualmente confirmado ausente.
- `Content::TableFooter` — factualmente confirmado ausente.
- `Content::Cite`, `Content::Bibliography`, `Content::Footnote` —
  ausentes (P159 + footnote scope-out).

**Pesquisa exaustiva** `grep -rn "Content::Table\|TableCell\|
TableHeader\|TableFooter" 01_core/src --include="*.rs"`:
- **Zero matches** em ficheiros de declaração de variants ou
  stdlib.
- "table" aparece apenas como **string literal `kind: "table"`**
  em `Figure` e `figure_numbers` para counters.

### §2.2 Stdlib funcs Model

Inspecção `01_core/src/rules/stdlib/mod.rs` re-exports:
- `native_heading`, `native_strong`, `native_emph`,
  `native_terms`, `native_divider`, `native_quote`, `native_raw`
  — Fase 1 cobertos.
- `native_figure`, `native_image` — figure cobertos.
- `native_grid` — grid coberto.

**Ausentes**: `native_table`, `native_table_cell`,
`native_table_header`, `native_table_footer`.

### §2.3 Estado de `grid` (dependência declarada)

Per tabela A.5 linha 141:

> `grid(columns, ...)` | `parcial` | Passos 82–84.6 | reclassificado
> em P156B (era `implementado⁺`); sem `gutter`, `align`, `stroke`,
> `fill`, `inset`, `header`, `footer`, `colspan`/`rowspan`.
> DEBT-34d/e abertos.

**Layout existente**: `01_core/src/rules/layout/grid.rs` (272
linhas) implementa algoritmo de tracks (TrackSizing::Auto/Fixed/
Fraction) com cells lineares distribuídas por `idx % num_cols`.

**Atributos vanilla cobertos**: `columns: Vec<TrackSizing>`,
`rows: Vec<TrackSizing>`, `cells: Vec<Content>`.

**Atributos vanilla scope-out**: gutter (3 variantes), align,
fill, stroke, inset, header, footer, colspan, rowspan. **9
atributos scope-out** — paralelos directos aos `Block` scope-out
em P156G.

### §2.4 Estado de `table` em vanilla (referência)

Inspecção `lab/typst-original/.../model/table.rs` (807 linhas):

**TableElem** (~12 atributos):
- `columns`, `rows` (TrackSizings).
- `gutter`, `column_gutter`, `row_gutter`.
- `inset` (Celled<Sides<...>> default 5pt).
- `align`, `fill`, `stroke` (Celled<...>).
- `summary` (internal — acessibilidade).
- `grid` (synthesized internal).
- `children: Vec<TableChild>` (variadic — TableHeader/TableFooter/
  TableHLine/TableVLine/TableCell).

**TableHeader** (3 atributos):
- `repeat: bool` (default true).
- `level: NonZeroU32` (default 1).
- `children: Vec<TableItem>`.

**TableFooter** (2 atributos):
- `repeat: bool` (default true).
- `children: Vec<TableItem>`.

**TableHLine / TableVLine** (cosmetic — linhas internas):
- 5+ fields cada (y/x, start, end, stroke, position).

**TableCell** (8 atributos):
- `body: Content` (required).
- `x`, `y` (Smart<usize>).
- `colspan`, `rowspan` (NonZeroUsize, default 1).
- `inset`, `align`, `fill`, `stroke` (Smart-wrapped).

**TableChild enum** (vanilla):
- `Header(TableHeader)`, `Footer(TableFooter)`, `Item(TableItem)`.

**TableItem enum** (vanilla):
- `HLine(TableHLine)`, `VLine(TableVLine)`, `Cell(TableCell)`.

### §2.5 Hashes actuais relevantes

- `01_core/src/entities/content.rs`: `ec58d849` (preservado
  em P156L).
- `01_core/src/rules/layout/grid.rs`: `a78b0adc`.
- `01_core/src/rules/stdlib/layout.rs`: `f6cc2443`.

---

## §3 — Scope de "table foundations"

### §3.1 Subset declarado per ADR-0060 (literal)

4 variants: `Table`, `TableCell`, `TableHeader`, `TableFooter`.
Tamanho declarado: M+.

### §3.2 Decisões Smart→Option/default per ADR-0064

Aplicação dos 4 casos canónicos a fields vanilla:

| Field vanilla | Caso | Tradução cristalina |
|---------------|------|---------------------|
| `TableElem.columns: TrackSizings` | (já em Grid) | `Vec<TrackSizing>` |
| `TableElem.rows: TrackSizings` | idem | `Vec<TrackSizing>` |
| `TableElem.gutter`/etc | scope-out | — (não materializado) |
| `TableElem.inset` (Celled) | scope-out | — |
| `TableElem.align`/`fill`/`stroke` | scope-out | — |
| `TableElem.children: Vec<TableChild>` | — | `Vec<Content>` (children podem ser TableCell/Header/Footer ou Content directo) |
| `TableHeader.repeat: bool default true` | D | `bool` directo, default `true` (paridade vanilla — N=4 do Caso D) |
| `TableHeader.level: NonZeroU32 default 1` | (constante) | `usize` directo, default `1` |
| `TableHeader.children` | — | `Vec<Content>` |
| `TableFooter.repeat: bool default true` | D | `bool` directo, default `true` (N=5 Caso D) |
| `TableFooter.children` | — | `Vec<Content>` |
| `TableCell.body: Content required` | — | `Box<Content>` |
| `TableCell.x: Smart<usize>` | A | `Option<usize>` (None ↔ auto-position no grid) |
| `TableCell.y: Smart<usize>` | A | `Option<usize>` (idem) |
| `TableCell.colspan: NonZeroUsize default 1` | (constante) | `usize` directo, default `1` |
| `TableCell.rowspan: NonZeroUsize default 1` | idem | `usize` directo, default `1` |
| `TableCell.inset/align/fill/stroke` | scope-out | — |

**N=4/5 do Caso D** atinge **N=8/9** total (após N=7 pós-P156L) —
patamar empírico continua a crescer; consolida ADR-0064 em
nova categoria de uso (vanilla `bool default true`).

### §3.3 Subset MÍNIMO (preserva granularidade 1-2 features)

**1 variant single**: `Content::Table { columns, rows, children:
Vec<Content> }` — sem TableCell estruturado.

- Suficiente para variant existir; cells passam Content directo.
- Layout reusa `layout_grid` directamente.
- **Insuficiente** para Fase 2 declarada (sem cells/header/footer).
- 1 feature → preserva N=10.

### §3.4 Subset MÁXIMO (cabe em passo M+)

**4 variants + 4 stdlib funcs num único passo**:
- `Content::Table { columns, rows, children: Vec<Content> }`.
- `Content::TableCell { body, x, y, colspan, rowspan }`.
- `Content::TableHeader { children, repeat, level }`.
- `Content::TableFooter { children, repeat }`.
- 4 native funcs em `stdlib/model.rs` (módulo novo) ou
  `stdlib/layout.rs`.
- Layouter trata children com pattern-match interno (Cell vs
  Header vs Footer vs Content directo).

**Quebra granularidade N=9** (1 passo cobre 4 features +
infraestrutura). Risco médio-alto: ~25-30 sítios pattern-match;
~30+ tests novos esperados.

### §3.5 Subset INTERMÉDIO (recomendação)

**3 sub-passos** preservando granularidade:

#### P157A — `Content::Table` minimal (1 feature; M)

- Variant `Content::Table { columns: Vec<TrackSizing>, rows:
  Vec<TrackSizing>, children: Vec<Content> }`.
- Stdlib `native_table` aceita named `columns`, `rows`; children
  variádicos posicionais.
- Layouter delega a `layout_grid` (cell layout linear).
- Sem TableCell; children são Content directo.
- **Granularidade preservada N=10**.
- Tests esperados: ~10-15 (variant + stdlib + layout E2E).

#### P157B — `Content::TableCell` + colspan/rowspan armazenados (1 feature; M)

- Variant `Content::TableCell { body: Box<Content>, x: Option<usize>,
  y: Option<usize>, colspan: usize, rowspan: usize }`.
- Stdlib `native_table_cell` (acessível via `table.cell` ou
  `cell` directo — decisão local em P157B).
- Layouter detecta TableCell em children de Table; usa body
  directamente. **colspan/rowspan armazenados mas ignorados**
  per **DEBT-34e** (scope-out per ADR-0054 graded — consistente
  com `Block.breakable` em P156G).
- **Granularidade preservada N=11**.
- Tests esperados: ~12-18.

#### P157C — `Content::TableHeader` + `Content::TableFooter` com `repeat` armazenado (1 feature combinada — paridade simétrica; S+/M)

- Variants `Content::TableHeader { children: Vec<Content>,
  repeat: bool, level: usize }` e `Content::TableFooter
  { children: Vec<Content>, repeat: bool }`.
- Stdlib `native_table_header` e `native_table_footer`.
- Layouter detecta Header/Footer em children de Table; renderiza
  como blocos lineares no início/fim. **`repeat` armazenado mas
  ignorado** (paridade vanilla exige multi-region — DEBT-56;
  scope-out per ADR-0054 graded).
- **Granularidade aceitável N=12** (2 variants estruturalmente
  análogos contam como 1 feature simétrica per precedente
  P156D HSpace+VSpace e P156C Pad+Hide).
- Tests esperados: ~10-15.

**Soma**: ~32-48 tests acumulados em 3 sub-passos. Cobertura
Model: 11/22 → ~14/22 (~64%) pós-P157C.

### §3.6 Recomendação para passo seguinte

**P157A** (subset mínimo de Tabela 3.5) é a recomendação:

- Preserva granularidade N=10 (estrita 1 feature/passo).
- Risco baixo (variant simples; reusa layout_grid directamente).
- Permite validar decisões arquitecturais (variant vs alias de
  Grid) antes de complicar com TableCell.
- Estabelece cadência cumulativa Model análoga a Layout
  (P156C-J/L mesa).
- Subset máximo M+ (Tabela 3.4) **rejeitado** porque viola
  granularidade N=9 sem benefício compensatório.

**Sub-passos posteriores P157B + P157C** materializam-se em
passos seguintes, cada um com diagnóstico próprio per ADR-0065.

---

## §4 — Dependências bloqueantes

### §4.1 Dependências de Layout

| Dependência | Estado | Bloqueia P157A? | Bloqueia P157B/C? |
|-------------|--------|:---------------:|:------------------:|
| `Content::Grid` + `layout_grid` | parcial mas funcional | Não | Não (P157B reusa) |
| Multi-region (DEBT-56 column flow) | EM ABERTO | Não | Sim para `repeat` real (P157C scope-out aceita) |
| `Content::Repeat` (P156J) | implementado mas single-render | Não (não aplicável) | Não (semântica diferente — repeat de body único vs cells) |

### §4.2 Dependências de Introspection

| Dependência | Estado | Bloqueia? |
|-------------|--------|:---------:|
| ADR-0017 Introspection runtime | adiada | Não (table não exige introspection runtime; counters resolvem em walk como em P155) |
| `measure(body)` | parcial | Não |

### §4.3 DEBTs abertos relevantes

| DEBT | Descrição | Impacto em P157A/B/C |
|------|-----------|----------------------|
| **DEBT-34d** | Auto não encolhe antes de matar fr (Grid) | Afecta qualidade output P157A se table usar Auto+fr; **não bloqueia** materialização |
| **DEBT-34e** | colspan/rowspan em Grid | **Limita** P157B (colspan/rowspan armazenados mas ignorados — scope-out graded) |
| DEBT-56 | Column flow multi-region | **Limita** P157C `repeat` real (armazenado mas ignorado) |
| DEBT-55 | Bibliography + Cite | Não bloqueia (fora de scope P157) |

### §4.4 ADRs em vigor relevantes

| ADR | Aplicação a P157A/B/C |
|-----|----------------------|
| ADR-0017 | Estratégia gradual — autoriza scope-out de runtime |
| ADR-0026/-R1 | `Content` enum fechado com `Arc<[T]>` para sequences — variants Table/TableCell/Header/Footer compatíveis |
| ADR-0033 | Paridade observável — exige cells visíveis na ordem correcta; layout pode divergir estruturalmente |
| ADR-0034 | Diagnóstico obrigatório para tipo vanilla — cumprido por P157 (este doc) e replicado por P157A/B/C |
| ADR-0054 | Perfil graded — autoriza scope-out de colspan/rowspan/repeat real |
| ADR-0060 | Roadmap Model — autoriza P157 como sub-passo Fase 1 |
| **ADR-0064** (P156K) | Smart→Option/default — aplicável (3.2) com 4 casos cobertos |
| **ADR-0065** (P156K) | Inventariar primeiro — aplicado por este passo (auto-validação critério #5 — primeira aplicação concreta) |

### §4.5 ADRs pendentes / candidatas

| ADR | Estado | Bloqueia? |
|-----|--------|:---------:|
| ADR-0061 (Layout roadmap) | PROPOSTO | Não (Layout não é dependência hard de Model) |
| ADR-0062 (hayagriva) | reservada (não criada) | Não (P159 future) |

### §4.6 Conclusão de dependências

**Zero bloqueios hard** para P157A. Limitações scope-out per
ADR-0054 graded em P157B (colspan/rowspan) e P157C (repeat
real). Sem novos DEBTs abertos por P157A/B/C — apenas
referências aos existentes (DEBT-34e em P157B; DEBT-56 em
P157C).

---

## §5 — Esboço de P157A (passo substantivo seguinte)

### §5.1 Identificador

**P157A** — segue precedente da série P156C-L (sufixo letra
após número base). Justificação: P157 é diagnóstico (este doc);
P157A inicia série granular Model Fase 2 análoga a Layout
P156C-J/L. Cadência espelhada cria simetria reconhecível
(ADR-0061 §"Aplicações cumulativas" Layout ↔ ADR-0060 análogo
para Model).

### §5.2 Tamanho

**M** (1 feature; M alinhado com P156G/H/I substantivos).

### §5.3 Subset concreto

```rust
Content::Table {
    columns:  Vec<TrackSizing>,
    rows:     Vec<TrackSizing>,
    children: Vec<Content>,
}
```

**Sem TableCell estruturado neste passo**. Children são Content
directo (texto, blocks, expressões). Layouter delega a algoritmo
de Grid existente.

**Stdlib**: `native_table(columns: ?, rows: ?, ..children) →
Content::Table`. Reusa helpers `extract_tracks` (já em
`stdlib/layout.rs` para Grid).

### §5.4 Sub-passos previstos (alto nível)

1. **Inventário** (mínimo per ADR-0034 + ADR-0065) em
   `diagnostico-table-passo-157a.md`.
2. **Variant** `Content::Table` em `entities/content.rs`.
   Cobertura exaustiva ~12 sítios pattern-match (paridade P156I
   Stack).
3. **Stdlib** `native_table` em `stdlib/model.rs` (**módulo
   novo**) ou `stdlib/layout.rs` (decisão arquitectural —
   **inventário .1 decide**).
4. **Layouter** delega a `layout_grid` clone simples.
5. **Tests**: ~10-15 (unit + stdlib + E2E).
6. **Hashes**: propagar com `crystalline-lint --fix-hashes`.

### §5.5 Granularidade

**Preservada N=10** (1 feature/passo). Cadência granular
mantida. Padrão #1 cresce de N=9 para N=10.

### §5.6 Padrões aplicáveis

- ADR-0064 Caso A (TableCell.x/y futuro em P157B).
- ADR-0064 Caso D (TableHeader.repeat futuro em P157C; default
  vanilla `true`).
- ADR-0065 critério #5 (scope determinado em §3 deste diagnóstico).
- Reuso template containers: P157A não acrescenta (variant
  simples reusa Grid). P157B/C podem reusar pattern variant
  rico de P156G/H/I/L.
- Helper `extract_tracks` reusado (subpadrão análogo a
  `extract_length` em ADR-0064 §Implicações).

### §5.7 Risco estimado

**Baixo-médio**:
- Reusos significativos (Grid layout + extract_tracks).
- Inventário .1 cobre divergências antes de execução.
- Cobertura exaustiva sistemática (precedente N=9 com zero
  reformulações).

---

## Resumo executivo

P157 confirma factualmente:

1. **ADR-0060 status**: `IMPLEMENTADO` (Fase 1 fechada). Fase 2
   prossegue como roadmap; **P157 é primeiro sub-passo Fase 2
   de Model**.
2. **Subset declarado** (literal ADR-0060): `Content::Table` +
   `TableCell` + `TableHeader` + `TableFooter` (4 variants;
   M+).
3. **Estado em código**: `table` factualmente ausente; `grid`
   parcial mas funcional; `Figure.kind: "table"` slot já existe
   (preparação P158 figure-table).
4. **Recomendação de scope**: dividir M+ em **3 sub-passos M
   cada** (P157A/B/C) preservando granularidade N=9 → 10/11/12.
5. **Dependências bloqueantes**: zero hard; limitações
   scope-out per ADR-0054 graded em P157B (colspan/rowspan
   per DEBT-34e) e P157C (repeat real per DEBT-56).
6. **P157A esboço**: variant `Content::Table` minimal reusando
   Grid; M; tests esperados ~10-15.

**Auto-validação ADR-0065 critério #5**: este diagnóstico
exemplifica "scope determinado por inventário" — subset
máximo (M+ vs 3xM) é decisão informada por estado factual,
não inferida da spec. Padrão N=6 → 7 (P156L critério #3 + agora
P157 critério #5 — primeira aplicação concreta de #5).
