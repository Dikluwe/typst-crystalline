# Diagnóstico `Content::Table` minimal — Passo P157A

Inventário pré-materialização per **ADR-0034 + ADR-0065** —
primeiro sub-passo de Model Fase 2. **Décima primeira aplicação
consecutiva** do padrão diagnóstico-primeiro; **sétima** sob
critério estendido de ADR-0065 (P156C/D/G/H/J/L + P157 critério
#5 + agora P157A).

---

## 1. Assinatura vanilla `TableElem` minimal

Fonte: `lab/typst-original/crates/typst-library/src/model/table.rs`
(807 linhas; ~12 atributos top-level). Confirmado em P157
diagnóstico §2.4.

**Subset minimal P157A** (3 fields críticos):

| Field vanilla | Tipo vanilla | Tradução cristalina P157A |
|---------------|--------------|---------------------------|
| `columns: TrackSizings` | `TrackSizings` (vanilla) | `Vec<TrackSizing>` (paridade `Content::Grid`) |
| `rows: TrackSizings` | idem | `Vec<TrackSizing>` |
| `children: Vec<TableChild>` | variadic | `Vec<Content>` (subset minimal — sem TableChild estruturado) |

**Subset diferido para P157B/C/futuros** (per ADR-0054 graded):

| Field vanilla | Diferido para |
|---------------|---------------|
| `gutter`, `column_gutter`, `row_gutter` | futuro (refino XS) |
| `inset` (Celled<Sides<...>>) | futuro |
| `align`, `fill`, `stroke` (Celled<...>) | futuro |
| `summary` (acessibilidade) | futuro |
| TableChild estruturado (Cell/Header/Footer/HLine/VLine) | **P157B** (Cell + colspan/rowspan); **P157C** (Header/Footer); HLine/VLine scope-out cosmetic |

---

## 2. Comportamento observável (subset minimal)

Cells distribuídas via `idx % num_cols` (algoritmo Grid existente).
Alinhamento default à esquerda. Sem stroke/fill/inset visível.

**Paridade observável vs vanilla** (subset minimal):
- ✓ Cells dispostas em grelha de NxM.
- ✓ Cells preservam ordem.
- ✗ Sem header/footer especiais (P157C).
- ✗ Sem cell merging (colspan/rowspan diferidos para P157B).
- ✗ Sem stroke/fill/inset visível (scope-out per ADR-0054 graded).

**Divergência aceite** per ADR-0033 + ADR-0054:
- Output de P157A `table` é **visualmente idêntico** ao output
  de `grid` equivalente — diferença é semântica (variant
  dedicado vs primitiva de layout).
- Esta é divergência **estrutural** aceite per ADR-0033;
  paridade observável preservada.

---

## 3. ADR-0064 caso aplicável

### 3.1 Análise por field

| Field | Caso ADR-0064 |
|-------|---------------|
| `columns: Vec<TrackSizing>` | (não Smart) — type directo |
| `rows: Vec<TrackSizing>` | idem |
| `children: Vec<Content>` | idem (variadic) |

**Conclusão**: P157A NÃO aplica nenhum dos 4 casos ADR-0064
directamente — todos os fields são tipos directos sem default
contextual nem `Smart<T>` em vanilla (a serialização vanilla
trata `columns` omitido como `Vec::new()`, mas o cristalino
trata como `vec![Auto]` consistente com Grid).

**Caso A futuro** (P157B): `TableCell.x: Smart<usize>` → 
`Option<usize>`. Caso C/D futuros conforme P157B/C.

### 3.2 Reuso intra-Model

Construção do variant é simétrica a `Content::Grid`:
```rust
// Grid (existente)
Grid { columns, rows, cells }

// Table (P157A)
Table { columns, rows, children }
```

Apenas o **nome do field cells/children diverge**: cristalino
usa `cells` em Grid (legado) e adopta `children` em Table per
nomenclatura vanilla (`Vec<TableChild>`). Pequena divergência
intra-cristalino documentada — nomes podem unificar-se em
refactor escopo XS futuro se padrão emergir.

---

## 4. Variants Content existentes a estender

**Nenhuma**. `Content::Table` é variant novo — sem encaixe em
variants existentes.

`Content::Grid` é estruturalmente análogo mas **não é alias**:
- Vanilla distingue `model/table.rs` (semântica de dados) de
  `layout/grid.rs` (primitiva de layout).
- ADR-0060 §"Decisão 4" exige variant novo para Model
  structural.
- Reaproveitamento cristalino vive **só no algoritmo de
  layout** (delega a `layout_grid`); estrutura no enum é
  variant dedicado.

---

## 5. Helpers stdlib reusáveis

### 5.1 `extract_tracks` (privado em `stdlib/layout.rs`)

Assinatura actual:
```rust
fn extract_tracks(val: Option<&Value>) -> Vec<TrackSizing>
```

**Reuso N=2** em P157A (origem em `native_grid`; agora `native_table`).

**Decisão de visibilidade**: tornar `pub(super)` em
`stdlib/layout.rs` para sibling-module access em
`stdlib/structural.rs`. Mudança trivial sem impacto noutros
callers (helper continua privado fora de `stdlib::*`).

**Subpadrão emergente análogo a `extract_length` (N=7)**.
Promoção formal a helper público diferida até atingir N=3-4
(precedente do mesmo padrão).

### 5.2 `parse_track_sizing` (privado em `stdlib/layout.rs`)

Helper interno chamado por `extract_tracks` — não precisa de
mudança de visibilidade (acessível indirectamente via
`extract_tracks`).

### 5.3 Outros helpers

- `extract_length` (P156C+; N=7): irrelevante para P157A
  (futuro `gutter` em refino).
- `extract_alignment`, `extract_dir`, `extract_parity`,
  `extract_weak`: irrelevantes para P157A.

---

## 6. Limitações aceites (perfil ADR-0054 graded)

| Aspecto | Estado P157A | Refino futuro |
|---------|--------------|---------------|
| `gutter`/`column_gutter`/`row_gutter` | scope-out | refino XS futuro |
| `inset`/`align`/`fill`/`stroke` per-table | scope-out | refino M após Block/Box pattern |
| TableCell estruturado | scope-out | **P157B** |
| `colspan`/`rowspan` | scope-out (DEBT-34e) | **P157B** com armazenamento ignorado |
| TableHeader/Footer | scope-out | **P157C** |
| `repeat` em Header/Footer | scope-out (DEBT-56) | **P157C** com armazenamento ignorado |
| `summary` (acessibilidade) | scope-out | refino XS futuro |
| `TableHLine`/`TableVLine` | scope-out | cosmetic — não-foundational |
| Cells distribuídas via `idx % num_cols` | ✓ implementado (reusa Grid) | mesmo algoritmo cobre P157B/C |
| Variant Content + stdlib + layout E2E | ✓ implementado | — |

---

## 7. Tests planeados

### 7.1 Unit tests `Content::Table` (~5)

Em `entities/content.rs`:
1. Constructor default (`Content::table(vec![], vec![], vec![])` → variant vazio).
2. Constructor com tracks explícitas.
3. `is_empty()` proxy via `children.is_empty()`.
4. `plain_text()` recurse e concatena children (consistente com Sequence/Grid).
5. `PartialEq` cobertura (columns/rows/children).

### 7.2 Stdlib tests `native_table` (~5-7)

Em `stdlib/mod.rs`:
1. Defaults sem args (columns/rows vazios → caem em `[Auto]`).
2. `columns` como Int (e.g. `columns: 3` → 3 tracks Auto).
3. `columns` como Array de Length (e.g. `[10pt, 20pt]`).
4. Children variádicos (e.g. `#table(columns: 2)[a][b][c][d]`).
5. Named arg desconhecido rejeitado.
6. Child inválido rejeitado (e.g. `Value::Int` em items).

### 7.3 Layout E2E tests (~2)

Em `layout/tests.rs`:
1. `layout_table_renderiza_cells_em_grid` — content.text("a"/"b"/"c"/"d") com `columns: 2` produz layout 2x2.
2. `layout_table_paridade_com_grid_equivalente` — `Content::Table` e `Content::Grid` com mesmos campos produzem layout idêntico (FrameItems mesma posição/conteúdo).

**Δ esperado**: +12 a +18 tests (consistente com P156I Stack que
adicionou 25 e P156J Repeat que adicionou 19; P157A é mais
pequeno por reusar `layout_grid` directamente).

---

## 8. Decisão de módulo: `stdlib/structural.rs` continuação vs `stdlib/model.rs` novo

Inspecção de `01_core/src/rules/stdlib/`:

| Módulo existente | Funcs | Domínio |
|------------------|-------|---------|
| `assert.rs` | assert | foundations |
| `calc.rs` | calc module | foundations |
| `figure_image.rs` | figure, image | model + asset |
| `foundations.rs` | type, len, range, rgb, luma, str, int, float | foundations |
| `layout.rs` | align, place, grid, page, pad, hide, h, v, pagebreak, block, box, stack, repeat | layout (incl. grid) |
| `shapes.rs` | rect, ellipse, circle, line, polygon | visualize |
| `structural.rs` | **strong, emph, raw, heading, terms, divider, quote** | **Model structural** |
| `text.rs` | lower, replace, upper | text manipulation |
| `transforms.rs` | move, rotate, scale, skew | layout transforms |

**`structural.rs` é o módulo Model**. Contém todas as funcs
Model existentes (heading, terms, divider, quote — Fase 1
completa).

**Decisão**: adicionar `native_table` em
`stdlib/structural.rs`. Justificação:
- Vanilla `table` vive em `model/table.rs` — análogo a
  `model/heading.rs`, `model/quote.rs`, etc.
- `structural.rs` é o módulo natural para Model structural.
- Não cria módulo novo; preserva estrutura estabelecida em
  P96.5 (ADR-0037 coesão por domínio).
- Decisão menos disruptiva — sem alteração de re-exports em
  `stdlib/mod.rs` além de adicionar `native_table`.

**Decisão alternativa rejeitada**: criar `stdlib/model.rs` novo.
Rejeitada porque introduz redundância semântica com
`structural.rs` sem benefício observável. Promoção a `model.rs`
poderia ser refactor futuro se `structural.rs` exceder ~800
linhas (limite ADR-0037), mas actualmente 379 linhas — folga
abundante.

---

## 9. Estrutura de re-export em `stdlib/mod.rs`

`structural.rs` re-exports actuais (linha 42-44):
```rust
pub use crate::rules::stdlib::structural::{
    native_divider, native_emph, native_heading, native_quote,
    native_raw, native_strong, native_terms,
};
```

**Mudança P157A**: adicionar `native_table` à lista (alfabético):
```rust
pub use crate::rules::stdlib::structural::{
    native_divider, native_emph, native_heading, native_quote,
    native_raw, native_strong, native_table, native_terms,
};
```

**Registo em `eval/mod.rs::make_stdlib`**:
```rust
use crate::rules::stdlib::{
    ..., native_table, ...,
};
...
scope.define("table", Value::Func(Func::native("table", native_table)));
```

Sem mudança em prompts L0 além de hash auto-actualizado por
`crystalline-lint --fix-hashes`.

---

## Resumo executivo

P157A materializa subset minimal de `Content::Table`:
- Variant `Table { columns, rows, children: Vec<Content> }`.
- Stdlib `native_table` em `stdlib/structural.rs` (módulo Model
  existente; sem novo módulo).
- Layouter delega a `layout_grid` via pattern arm trivial em
  `layout_content`.
- Helper `extract_tracks` promovido a `pub(super)` para reuso
  cross-módulo (N=2; subpadrão emergente).

**Decisões arquitecturais P157A**:
- **Módulo stdlib**: `stdlib/structural.rs` (continuação Model
  existente; sem novo módulo `stdlib/model.rs`).
- **Field name `children`** (não `cells`): paridade vanilla;
  pequena divergência intra-cristalino documentada vs `Grid.cells`.
- **Layout delegação**: clone simples de `layout_grid`; sem
  modificação de `grid.rs`.

**Decisões diferidas para P157B/C/futuros**:
- TableCell estruturado (P157B).
- Header/Footer (P157C).
- gutter/inset/align/fill/stroke (refinos futuros).

**ADRs aplicadas**:
- ADR-0034: diagnóstico cumprido (este doc).
- ADR-0054: graded scope-out de 9+ atributos vanilla.
- ADR-0060: variant novo per Decisão 4.
- ADR-0064: NÃO aplicável directamente (subset sem Smart<T>);
  futuro em P157B/C.
- ADR-0065: critério #1/#5/#6 implícitos (naming children vs
  cells é subdecisão de naming; scope é critério #5; decisão
  de módulo é critério #1 de naming módulo).

**Tests planeados**: Δ +12-18.

**Risco**: baixo-médio. Mitigação: reuso significativo;
`layout_grid` não modificado.
