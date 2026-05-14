# Passo 234 — B.2 Consumer geometric `place_cells` → Layouter integration (Fase 5 Layout candidata Categoria B 2/3; quinta aplicação automática ADR-0080 EM VIGOR)

**Série**: 234 (vigésimo sub-passo Layout pós-M9c; **sétimo
sub-passo materialização Fase 5 Layout candidata** per
ADR-0079 PROPOSTO; **segundo sub-passo Categoria B**
"algorítmicos isolados"; quinta aplicação automática
ADR-0080 EM VIGOR pós-P229).
**Marco**: nenhum status ADR; **integração consumer
geometric pós-P224.C placement algorítmico criado mas
não-integrado** (~time P224 → P234; consolidação
arquitectural natural); pattern emergente "integração
consumer pós-isolamento algorítmico em sub-passo
posterior" N=1 inaugurado P234; pattern "three-pass
measure→place→emit" N=1 inaugurado P234 (extensão
two-pass P233); pattern "aplicação automática ADR-0080
EM VIGOR" N=4 → 5 cumulativo.
**Tipo**: refactor estructural consumer — `layout_grid`
passa a chamar `place_cells` baseline P224.C ao invés de
iterar `cells: &[Content]` direct; **zero fields novos**
em Content variants; **zero novos variants**; **zero
novas stdlib funcs**.
**Magnitude**: M (~2-3h; paridade diagnóstico P226 B.2).
**Pré-condição**: P233 concluído (B.1 DEBT-34d FECHADO;
2111 verdes; 0 violations; saldo DEBTs 11; P224.div-1
RESOLVIDA P233; ADR-0079 Categoria A 5/5 + Categoria B
1/3); humano fixou B.2 (decisão literal pós-P233 §8);
`grid_placement.rs::place_cells` baseline P224.C
(algorítmico isolado retornando `Vec<PlacedCell>`);
`PlacedCell { body, row, col, colspan, rowspan }`
baseline P224.C (5 fields preservados pós-P230 audit que
rejeitou refactor); `layout_grid` baseline P224+P227+P228+
P230+P233 que itera `cells: &[Content]` direct; `col_sizes`
+ `row_sizes` calculados P233 baseline two-pass; pattern
"aplicação automática ADR-0080 EM VIGOR" N=4 baseline P230+
P231+P232+P233; pattern "algoritmo two-pass measure→place"
N=1 baseline P233.
**Output**: 1 ficheiro relatório curto + código alterado em
~3-4 ficheiros L1 + L0 NÃO tocado (quinta aplicação
automática ADR-0080 EM VIGOR) + inventário 148 anotação
cumulativa (footnote ⁵³) + ADR-0079 anotação **Categoria
B 2/3** (B.1 ✓; B.2 ✓; B.3 pendente).

---

## §1 Trabalho

P224.C criou `grid_placement.rs::place_cells` algorítmico
isolado (módulo novo P224 que resolve placement
algoritmicamente incluindo colspan/rowspan). **P230 audit
revelou empíricamente** que `layout_grid` itera `cells:
&[Content]` direct **sem usar `place_cells`** —
algorítmico foi criado mas **não-integrado** com consumer
geometric conforme atomização ADR-0036 (P224.C escopo
algorítmico isolado; integração consumer separada P226
Categoria B.2).

P226 diagnóstico Categoria B.2 marcou literal: "Consumer
geometric `place_cells` → Layouter (M)".

**P234 materializa B.2**:
- **`layout_grid` passa a chamar `place_cells`** baseline
  P224.C; obtém `Vec<PlacedCell>` em vez de iterar `cells:
  &[Content]` direct.
- **Bounds calculation per-cell** usa `placed.row/col/
  colspan/rowspan` × `col_sizes/row_sizes` P233 baseline.
- **Match no `placed.body`** preserva extracção per-cell
  stroke/fill paridade P230 (apenas mudança de iteração
  source).
- **Colspan/rowspan funcionam realmente** em renderização
  (não apenas isolado em `place_cells`).
- **Renderização Z-order P227+P228 preservado integralmente**
  (fill → conteúdo → stroke per cell).
- **Precedência per-cell P230 preservada** (cell stroke/
  fill override Grid via `.or()`).

**Decisão arquitectural central — 8 decisões fixadas**:

### Decisão 1 — Escopo Opção α (integração completa)

3 opções consideradas:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | `layout_grid` chama `place_cells` → Vec<PlacedCell>; substitui iteração `cells: &[Content]` direct | Integração arquitectural natural; consolida P224.C |
| β | Integração parcial: apenas certos casos (e.g., colspan/rowspan); resto preserva iteração direct | Inflacionário; código split; viola coerência |
| γ | Integração + refactor PlacedCell expandido +2 fields | Reabre rejeição P230 audit que descartou refactor PlacedCell |

**Decisão fixada — Opção α** (integração completa). **Não
reabrir** rejeição PlacedCell refactor P230 (audit
empírico revelou inline match suficiente).

### Decisão 2 — `PlacedCell` baseline P224.C suficiente literal

P224.C `PlacedCell` baseline: `{ body, row, col, colspan,
rowspan }` (5 fields preservados P230 pós-audit).
`layout_grid` actual extrai stroke/fill/per-cell via match
no `Content::GridCell { stroke, fill, .. }` directly.

**Pós-P234**: match no `placed.body` em vez de `cell` direct.
`placed.body: Content` preserva GridCell variant inteiro
incluindo stroke/fill P230 fields.

3 opções consideradas:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | `PlacedCell` baseline 5 fields literal; match em `placed.body` extrai per-cell | Refactor mínimo; coerente P230 audit |
| β | `PlacedCell` expandido +2 fields (cell_stroke, cell_fill) | P230 audit rejeitou; reabrir |
| γ | `PlacedCell` expandido +alguns fields (cobertura parcial) | Inflacionário; semantic mixed |

**Decisão fixada — Opção α** (paridade P230 audit).

### Decisão 3 — Bounds calculation Opção α (`placed.row/col/
colspan/rowspan` × sizes)

Pós-integração, `layout_grid` recebe `Vec<PlacedCell>` mas
precisa calcular **bounds visuais reais** (x0, y0, x1, y1)
per cell usando `col_sizes` + `row_sizes` P233.

```rust
fn cell_bounds(placed: &PlacedCell, col_sizes: &[f64], row_sizes: &[f64]) -> (f64, f64, f64, f64) {
    let x0 = col_sizes[..placed.col].iter().sum::<f64>();
    let y0 = row_sizes[..placed.row].iter().sum::<f64>();
    let w = col_sizes[placed.col..placed.col + placed.colspan].iter().sum::<f64>();
    let h = row_sizes[placed.row..placed.row + placed.rowspan].iter().sum::<f64>();
    (x0, y0, x0 + w, y0 + h)
}
```

**Decisão fixada — Opção α** (cálculo inline; sem helper
externalizado a menos que audit C1 revele complexidade
maior).

### Decisão 4 — Match per-cell preservado paridade P230

Pós-integração:

```rust
// Pré-P234 (P230 baseline):
for (idx, cell) in cells.iter().enumerate() {
    let (cell_stroke, cell_fill) = match cell {
        Content::GridCell { stroke, fill, .. } => (stroke.as_ref(), fill.as_ref()),
        Content::TableCell { stroke, fill, .. } => (stroke.as_ref(), fill.as_ref()),
        _ => (None, None),
    };
    // ... emit usando idx para posição sequencial simples ...
}

// Pós-P234:
let placed_cells = place_cells(cells, num_cols, num_rows)?;
for placed in placed_cells.iter() {
    let (cell_stroke, cell_fill) = match &placed.body {
        Content::GridCell { stroke, fill, .. } => (stroke.as_ref(), fill.as_ref()),
        Content::TableCell { stroke, fill, .. } => (stroke.as_ref(), fill.as_ref()),
        _ => (None, None),
    };
    let (x0, y0, x1, y1) = cell_bounds(placed, &col_sizes, &row_sizes);
    // ... emit usando bounds reais (incl. colspan/rowspan) ...
}
```

**Decisão fixada — Opção α**: mudança source de iteração
(de `cells` direct para `place_cells` result); match no
body preserva semantic P230 literal.

### Decisão 5 — Tests E2E colspan/rowspan funcionais

Crítico testar pós-integração:
- Grid 3 cols; cell com `colspan: 2` → cell visualmente
  ocupa 2 colunas (largura = col_sizes[col] + col_sizes[col+1]).
- Grid 3 rows; cell com `rowspan: 2` → cell visualmente
  ocupa 2 linhas (altura = row_sizes[row] + row_sizes[row+1]).
- Grid mix: várias cells com colspan/rowspan diferentes;
  placement correcto.
- Cell colspan=2 + stroke → stroke renderiza ao redor de
  ambas as colunas (não cada coluna separada).
- Cell colspan=2 + fill → fill cobre ambas as colunas (Rect
  com largura combinada).
- Cell colspan + per-cell stroke override Grid → precedência
  P230 preservada em multi-col.

**Decisão fixada — 5-7 tests E2E colspan/rowspan**.

### Decisão 6 — Tests regressão baseline preservada

Tests pre-existentes P224 + P227 + P228 + P230 + P233 assumem
placement sequencial sem colspan/rowspan. Pós-P234:
- Tests sem colspan/rowspan: `colspan: None, rowspan: None`
  → resolved como `colspan: 1, rowspan: 1` em `place_cells`
  → posição sequencial paridade literal.
- **Zero regressões esperadas** (algoritmo correcto preserva
  comportamento simples).

**Decisão fixada — 3-5 tests regressão explícitos**:
- Grid sem colspan/rowspan: placement preservado paridade
  P224+P227+P228+P230+P233.
- Stroke baseline P227 preservado.
- Fill baseline P228 preservado.
- Per-cell precedência P230 preservada.
- Auto sizing P233 fix preservado.

### Decisão 7 — L0 NÃO tocado (ADR-0080 EM VIGOR aplicação
automática N=5)

**Decisão fixada — aplicação automática quinta pós-P229**:

P234 é refactor algorítmico interno:
- Zero fields novos.
- Zero novos variants.
- Zero novas stdlib funcs.
- Refactor consumer `layout_grid` chama `place_cells`
  baseline.

ADR-0080 EM VIGOR §"Decisão" aplica-se por defeito.
Pattern N=4 → **5 cumulativo** (P230+P231+P232+P233+
**P234**). Pattern muito sólido.

L0 prompts NÃO tocados.

### Decisão 8 — Pattern emergente "three-pass" N=1 inaugurado P234

P233 inaugurou pattern "two-pass measure→place" N=1
cumulativo. **Pós-P234**:

3 patterns relacionados:
- **"two-pass measure→place"** N=1 baseline P233 (preservado).
- **"three-pass measure→place→emit"** N=1 inaugurado P234 —
  P233 measure pre-pass + P224.C `place_cells` place pre-pass
  agora integrado + P234 emit final pass com bounds reais.
- Pattern **"integração consumer pós-isolamento
  algorítmico em sub-passo posterior"** N=1 inaugurado P234
  — paridade conceitual ao pattern "fecho de DEBT preservado
  conscientemente em sub-passo posterior" P233.

**Decisão fixada — patterns documentados sem promoção
formal** (N=1 cada; limiar formalização N=3-4 não atingido).

Reuso de dados (sem recolha nova):

- `grid_placement.rs::place_cells` baseline P224.C
  (`pub fn place_cells(cells: &[Content], num_cols: usize,
  num_rows: usize) -> SourceResult<Vec<PlacedCell>>`).
- `PlacedCell { body, row, col, colspan, rowspan }` baseline
  P224.C.
- `layout_grid` baseline P224+P227+P228+P230+P233 com
  `col_sizes` + `row_sizes` resolvidos P233 two-pass.
- Pattern Z-order baseline P227+P228.
- Pattern precedência per-cell baseline P230.
- Pattern "aplicação automática ADR-0080 EM VIGOR" N=4
  baseline P230+P231+P232+P233.
- Pattern "two-pass measure→place" N=1 baseline P233.

---

## §2 Cláusulas (10 — atomização paridade P233)

### C1 — Auditoria pré-P234: confirmar `place_cells` baseline + `layout_grid` integration points

Audit empírico crítico:

```
grep -A 20 "pub fn place_cells\|pub fn place_cells_with" 01_core/src/rules/layout/grid_placement.rs
grep -B 2 -A 30 "PlacedCell" 01_core/src/rules/layout/grid_placement.rs
grep -B 2 -A 60 "for.*cell.*in.*cells\|fn layout_grid" 01_core/src/rules/layout/grid.rs
grep -n "col_sizes\|row_sizes" 01_core/src/rules/layout/grid.rs
grep -n "colspan\|rowspan" 01_core/src/rules/layout/grid.rs
```

Hipótese:
- `place_cells` signature `pub fn place_cells(cells:
  &[Content], num_cols: usize, num_rows: usize) ->
  SourceResult<Vec<PlacedCell>>` ou similar — audit C1
  confirma signature exacta.
- `PlacedCell` baseline 5 fields preservados pós-P230 audit
  rejeitou refactor.
- `layout_grid` actual itera `cells: &[Content]` em loop
  sequencial; ignora colspan/rowspan (cells em posição
  `(row, col)` resolvida via `idx % num_cols`).
- `col_sizes` + `row_sizes` calculados P233 disponíveis no
  scope `layout_grid`.

**Decisões críticas C1**:
1. **Signature `place_cells` precisa adicionar param
   `num_rows`?** Audit C1 confirma — possível baseline
   apenas `num_cols` e `num_rows` calculado dinamicamente
   pelos cells.
2. **`PlacedCell.colspan/rowspan: usize` ou `Option<usize>`?**
   Audit C1.
3. **Visibility `place_cells` cross-módulo OK?** Provável
   `pub` baseline P224.C.

Se signature ou estado divergir significativamente: registar
`P234.div-N`.

### C2 — Refactor `layout_grid` para chamar `place_cells`

Editar `01_core/src/rules/layout/grid.rs::layout_grid`:

**Substituir iteração cells direct por place_cells call**:

```rust
// Pré-P234 (baseline P224+P227+P228+P230+P233):
// for (idx, cell) in cells.iter().enumerate() {
//     let row = idx / num_cols;
//     let col = idx % num_cols;
//     ... emit ...
// }

// Pós-P234:
let placed_cells = place_cells(cells, num_cols, num_rows)?;

for placed in placed_cells.iter() {
    // Bounds reais usando placed.row/col/colspan/rowspan
    // × col_sizes/row_sizes P233.
    let (x0, y0, x1, y1) = cell_bounds(placed, &col_sizes, &row_sizes);
    let cell_w = x1 - x0;
    let cell_h = y1 - y0;

    // Match no placed.body preserva per-cell P230 semantic.
    let (cell_stroke, cell_fill) = match &placed.body {
        Content::GridCell { stroke, fill, .. } => (stroke.as_ref(), fill.as_ref()),
        Content::TableCell { stroke, fill, .. } => (stroke.as_ref(), fill.as_ref()),
        _ => (None, None),
    };
    let effective_stroke = cell_stroke.or(stroke);
    let effective_fill = cell_fill.or(fill);

    // Z-order step 1: fill antes do conteúdo (P228).
    if let Some(c) = effective_fill {
        self.regions.current.current_items.push(FrameItem::Shape {
            pos: Point { x: Pt(x0), y: Pt(y0) },
            kind: ShapeKind::Rect,
            width: cell_w,
            height: cell_h,
            fill: Some(*c),
            stroke: None,
        });
    }

    // Z-order step 2: conteúdo (existing layout body).
    // ... layout placed.body ...

    // Z-order step 3: stroke depois (P227).
    if let Some(s) = effective_stroke {
        // ... 4 lines per cell border usando bounds reais ...
    }
}
```

Magnitude C2: **S+ (~1h)** — refactor mínimo coerente.

### C3 — Adicionar helper `cell_bounds` ou inline

Decisão arquitectural:
- α — Helper `cell_bounds` privado no `grid.rs` (reusável
  ~3 lugares: fill emit; stroke emit; layout body).
- β — Inline cálculo (~5 linhas duplicadas).

**Decisão fixada — Opção α** (helper privado `fn cell_bounds`)
se cálculo aparece ≥2 lugares; senão inline.

Magnitude C3: **XS (~10min)**.

### C4 — Layout body usar bounds reais

Pós-P234, layout do body `placed.body` precisa receber
bounds reais (incluindo colspan/rowspan). Audit C1
determina exactamente como `layout_content` consume body
bounds.

**Decisão fixada — Opção α**: `layout_content` recebe
bounds reais via region constraints (Layouter scope save/
restore paridade `cell_origin_*` P84.6).

Magnitude C4: **S (~30min)** — paridade pattern P84.6 +
P232 cell_align save/restore.

### C5 — Sentinelas P234

Tests P234 (~10-12 tests):

**Layout E2E colspan/rowspan funcional** (~7 tests
crítico):
- `p234_grid_colspan_2_cell_ocupa_2_cols` — visual width
  = col_sizes[0] + col_sizes[1].
- `p234_grid_rowspan_2_cell_ocupa_2_rows` — visual height
  similar.
- `p234_grid_colspan_rowspan_mix` — cell 2x2.
- `p234_grid_colspan_com_stroke_envolve_ambas_cols` —
  stroke render correcto multi-col.
- `p234_grid_colspan_com_fill_cobre_ambas_cols` — fill
  Rect width combinada.
- `p234_grid_colspan_com_per_cell_stroke_override_grid` —
  P230 precedência preservada multi-col.
- `p234_grid_varios_cells_colspan_rowspan_diferentes` —
  layout completo complexo.

**Tests regressão baseline** (~3-5 tests):
- `p234_grid_sem_colspan_rowspan_baseline_preservado` —
  placement sequencial preservado.
- `p234_grid_stroke_baseline_P227_preservado`.
- `p234_grid_fill_baseline_P228_preservado`.
- `p234_grid_per_cell_baseline_P230_preservado`.
- `p234_grid_auto_sizing_baseline_P233_preservado`.

Total tests P234: **~10-12 tests** (7-8 + 3-5). Esperado
pós-P234: **2111 + 12 = ~2123 verdes** (paridade hipótese;
ajuste pós-implementação).

### C6 — L0 NÃO tocado (ADR-0080 EM VIGOR aplicação
automática N=5)

**Decisão fixada — aplicação automática**: quinta
aplicação automática pós-promoção P229. Pattern N=4 → 5
cumulativo.

L0 prompts NÃO tocados.

### C7 — Verificação tests workspace + lint

```
cargo test --workspace 2>&1 | tail -3
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério:
- 2111 verdes pré-P234 + ~10-12 novos = **~2123 verdes**.
- 0 violations preservadas.
- Hashes propagados em ~3-4 ficheiros L1 (`grid.rs`,
  possível `grid_placement.rs` se signature ajusta).
- L0 prompts não tocados — "Nothing to fix".

**Risco regressão**: tests P224 + P227 + P228 + P230 + P233
baseline. Hipótese N=0-3 adaptações intencionais (cells
sem colspan/rowspan = `colspan: 1, rowspan: 1` em
`PlacedCell` resolved; comportamento sequencial preservado).

### C8 — Inventário 148 footnote ⁵³

**§A.5 Layout entrada `grid(...)`**: footnote `⁵² → ⁵² ⁵³`.

Footnote ⁵³ adicionada (~85 linhas) documentando:
- B.2 materializado (segundo Categoria B Fase 5).
- 8 decisões fixadas.
- Integração consumer geometric pós-P224.C placement
  algorítmico isolado.
- Colspan/rowspan funcionais em renderização pela primeira
  vez pós-M9c.
- 4 patterns emergentes consolidados/inaugurados:
  - "Aplicação automática ADR-0080 EM VIGOR" N=4 → 5
    cumulativo.
  - "Three-pass measure→place→emit" N=1 inaugurado P234.
  - "Integração consumer pós-isolamento algorítmico em
    sub-passo posterior" N=1 inaugurado P234.
  - "PlacedCell baseline P224.C suficiente sem refactor"
    confirmado empíricamente segunda vez (P230 + P234).
- Reuso `place_cells` baseline P224.C N=0 → 1 cumulativo
  (primeiro consumer geometric real).

### C9 — ADR-0079 anotação Categoria B 2/3

Editar ADR-0079:

```markdown
### P234 anotação — Categoria B sub-passo 2 (Consumer
geometric place_cells → Layouter integration)

**Categoria B**: 2/3 sub-passos materializados ✓.
- B.1 DEBT-34d Auto track sizing fix (P233) ✓.
- **B.2 Consumer geometric (P234) ✓**.
- B.3 GridCell per-cell algorítmico (align/inset/breakable)
  — pendente.

Trabalho P234:
- `layout_grid` passa a chamar `place_cells` baseline
  P224.C; obtém `Vec<PlacedCell>` em vez de iterar `cells`
  direct.
- Bounds reais per `placed.row/col/colspan/rowspan` ×
  `col_sizes/row_sizes` P233.
- **Colspan/rowspan funcionam realmente em renderização**
  pela primeira vez pós-M9c.
- Z-order P227+P228 + precedência P230 preservados
  integralmente.
- ~10-12 tests novos.
- **Quinta aplicação automática ADR-0080 EM VIGOR**.

Patterns consolidados/inaugurados:
- "L0 minimal para refactors" aplicação automática N=4 → 5
  cumulativo.
- **"Three-pass measure→place→emit" N=1 inaugurado P234**
  (extensão two-pass P233).
- **"Integração consumer pós-isolamento algorítmico em
  sub-passo posterior" N=1 inaugurado P234**.

Status ADR-0079 mantido PROPOSTO (7/13-15 sub-passos
cumulativos; Categoria A 5/5 ✓; Categoria B 2/3; C+D
pendentes).
```

### C10 — Critério aceitação P234

- ~10-12 tests novos verdes.
- 2111 tests pre-existentes preservados (após N=0-3
  adaptações).
- 0 violations.
- Zero fields novos em Content variants.
- Zero novos variants.
- Zero novas stdlib funcs.
- `layout_grid` chama `place_cells` baseline P224.C.
- Colspan/rowspan funcionais em renderização.
- Z-order P227+P228 + precedência P230 preservados.
- ADR-0079 Categoria B 2/3 anotado.
- ADR-0080 EM VIGOR aplicação automática N=4 → 5.
- Cobertura Layout 89% preservada (refino qualitativo).

---

## §3 Output

1 ficheiro relatório curto:
`00_nucleo/materialization/typst-passo-234-relatorio.md`.

Estrutura (~6-8 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Inventário pré-P234 + audit `place_cells` signature
  + integration points (C1).
- §3 Refactor `layout_grid` chama `place_cells` (C2).
- §4 Bounds calculation per placed.row/col/colspan/rowspan
  × sizes P233 (C3+C4).
- §5 Decisões substantivas (8 decisões fixadas) + quinta
  aplicação automática ADR-0080 EM VIGOR.
- §6 Resultados verificação + tests colspan/rowspan
  funcionais (C5+C7).
- §7 Inventário 148 footnote ⁵³ + ADR-0079 anotação
  Categoria B 2/3 (C8+C9).
- §8 Próximo sub-passo (P235 candidatos: B.3 GridCell
  per-cell algorítmico; D.1 state; pivot).

Código alterado:
- **Editado**: `01_core/src/rules/layout/grid.rs`
  (`layout_grid` refactor chama `place_cells`; bounds
  calculation per-cell; possível helper `cell_bounds`
  privado).
- **Possivelmente editado**: `01_core/src/rules/layout/grid_placement.rs`
  (signature `place_cells` ajuste se necessário; audit C1
  determina).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+~10-12
  tests novos colspan/rowspan + regressões).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (footnote ⁵³ P234).
- **Editado**: `00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`
  (+ anotação Categoria B 2/3 P234).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- Refactor `PlacedCell` baseline +fields — P230 audit
  rejeitou; reabrir contradiz consenso empírico.
- Per-cell algorítmico align/inset/breakable — Categoria
  B.3 separada (P235 candidato; valida pattern `.or()`
  N=2 → 3).
- Multi-region span (cell colspan/rowspan que cruza
  regions) — Categoria C.2 separada (reabre P216B).
- Optimização performance `place_cells` (algoritmo já
  baseline P224.C; refactor algorítmico fora escopo).
- Promover ADR-0079 PROPOSTO → IMPLEMENTADO — só pós
  Categorias A + B + C + D completas (B.2 ✓; B.3 pendente).
- Tocar em L0 prompts — ADR-0080 EM VIGOR aplicação
  automática N=5.
- Reabrir decisões arquiteturais — B.2 é Categoria B
  algorítmico isolado.
- Show rules `#show grid: ...` — fora escopo Fase 5.
- Promoção pattern "three-pass" a ADR meta — N=1 insuficiente.
- Promoção pattern "integração consumer pós-isolamento"
  a ADR meta — N=1 insuficiente.
- Closure cells `(row, col) => content` — fora escopo
  Fase 5.

---

## §5 Riscos a evitar

1. **`place_cells` signature divergente da hipótese**:
   audit C1 crítico. Mitigação: refactor signature mínimo
   ou path completo cross-módulo.
2. **`PlacedCell` baseline insuficiente**: P230 audit
   rejeitou expansão. Mitigação: confirmar match em
   `placed.body` cobre todos casos extracção per-cell.
3. **Bounds calculation off-by-one**: colspan/rowspan
   semantic (inclusive vs exclusive ranges). Mitigação:
   tests específicos C5 com bounds calculados manualmente
   no test.
4. **Tests baseline P224+P227+P228+P230+P233 quebrados**:
   hipótese N=0-3 adaptações. Cells sem colspan/rowspan
   = `colspan: 1, rowspan: 1` em PlacedCell preservam
   comportamento sequencial. Mitigação: 5 tests regressão
   C5 explícitos.
5. **Z-order P227+P228 quebrado em colspan**: stroke +
   fill multi-col precisam Rect/Lines com bounds reais.
   Mitigação: tests específicos C5 stroke/fill em colspan.
6. **Precedência per-cell P230 quebrada em colspan**:
   `cell.stroke.or(grid.stroke)` preservado literal;
   bounds reais não afectam lógica precedência. Mitigação:
   test específico C5.
7. **L0 tocado por engano**: quinta aplicação automática.
   Mitigação: §5 risco 7 explícito + §C6 fixa não tocar.
8. **`place_cells` retorna `Result<Vec<PlacedCell>>`**:
   tratar errors (cells overflow grid; colspan exceeds
   num_cols). Mitigação: error handling preservado
   propagar `?`.
9. **Magnitude exceder M (~2-3h)**: P233 chegou em ~45min.
   P234 maior complexidade (integração estructural).
   Hipótese real M (~2h).
10. **Pattern N=1 promoção prematura**: N=1 não atinge
    limiar formalização. Mitigação: documentar patterns
    sem promoção.
11. **Refactor invasivo cross-módulo**: `place_cells` em
    `grid_placement.rs`; consumer em `grid.rs`. Cross-módulo
    OK baseline (audit C1).
12. **Documentar three-pass em L0**: tentação por "algoritmo
    importante". Rejeitada — ADR-0080 EM VIGOR aplicação
    automática.

---

## §6 Hipótese provável

C1 confirmará `place_cells` baseline P224.C signature OK
cross-módulo; `PlacedCell` 5 fields baseline preservados;
`layout_grid` itera `cells` direct sem chamar `place_cells`;
`col_sizes` + `row_sizes` P233 disponíveis no scope.

C2 refactor `layout_grid` para chamar `place_cells` →
`Vec<PlacedCell>`; substituir iteração direct.

C3 helper `cell_bounds` privado (paridade pattern helpers
privados P227 `extract_stroke`).

C4 bounds reais propagados via region constraints (paridade
P84.6 `cell_origin_*`).

C5 criará ~10-12 tests novos (7-8 colspan/rowspan + 3-5
regressões).

C6 NÃO tocará L0 (aplicação automática N=5).

C7 reportará ~2123 verdes; 0 violations; possíveis N=0-3
adaptações.

C8 reclassificará footnote ⁵³.

C9 anotará ADR-0079 Categoria B 2/3.

C10 verifica critério aceitação.

Custo real: **M (~2h)** — integração estructural mais
trabalhosa que P233 fix subset minimal.

Mas é hipótese, não decisão. C1-C10 fixam-se empíricamente.

---

## §7 Particularidade P234

P234 é estruturalmente distinto na trajectória pós-M9c:

- **Sétimo sub-passo materialização Fase 5 Layout
  candidata** — segundo Categoria B algorítmico cumulativo.
- **Primeira integração consumer pós-isolamento algorítmico
  em sub-passo posterior pós-M9c** — P224.C `place_cells`
  criado mas não-integrado conscientemente (atomização
  ADR-0036); P234 integra. Pattern emergente N=1 inaugurado.
- **Primeira renderização real de colspan/rowspan pós-M9c**
  — funcionalidade prevista P224.C agora visível em
  output.
- **Pattern "three-pass measure→place→emit" N=1 inaugurado
  P234** — extensão pattern P233 "two-pass measure→place"
  N=1. Padrão mais geral: P233 measure pre-pass + P224.C
  place pre-pass integrado + P234 emit final pass.
- **Quinta aplicação automática ADR-0080 EM VIGOR
  pós-promoção P229** — pattern N=4 → 5 cumulativo
  (P230+P231+P232+P233+P234). Pattern extremamente sólido
  empíricamente.
- **`PlacedCell` baseline P224.C confirmado suficiente
  empíricamente segunda vez** (P230 audit rejeitou refactor;
  P234 validate empíricamente que baseline serve).
- **Reuso `place_cells` N=0 → 1 cumulativo** — primeiro
  consumer geometric real (criado P224.C; isolado até
  P234).
- **Cobertura Layout per metodologia preservada 89% real**
  — B.2 é refino algorítmico de funcionalidade existente.
- **Anti-inflação 26ª aplicação cumulativa** pós-P205D —
  Opção α integração completa + Opção α PlacedCell baseline
  literal + Opção α bounds inline ou helper privado +
  Opção γ L0 automático + sem refactor PlacedCell + sem
  promoções patterns emergentes + ADR-0079 sem promoção.

Por isso §5 risco 3 (bounds off-by-one) é o mais provável
empíricamente. Mitigação: tests específicos C5 com bounds
calculados manualmente para validar (colspan=2 → width =
col_sizes[col] + col_sizes[col+1] inclusivo).

**Critério de aceitação P234**:
- ~10-12 tests novos verdes.
- 2111 tests pre-existentes preservados (após N=0-3
  adaptações).
- 0 violations.
- Zero fields novos.
- `layout_grid` chama `place_cells` baseline P224.C.
- Colspan/rowspan funcionais em renderização.
- Z-order P227+P228 + precedência P230 preservados.
- ADR-0079 Categoria B 2/3 anotado.
- ADR-0080 EM VIGOR aplicação automática N=4 → 5.
- Cobertura Layout 89% preservada.

**Estado pós-P234 esperado**:
- Tests workspace: 2111 → **~2123 verdes** (+10-12).
- Stdlib funcs: 60 preservado.
- Content variants: 59 preservado.
- Value variants: 55 preservado.
- Grid/Table/Cell/Block/Boxed/Place fields preservados.
- PlacedCell fields: 5 preservado (não-refactorado).
- Layouter fields: preservados (n+1 pós-P232).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- ADR-0066 PROPOSTO; ADR-0061 IMPLEMENTADO; ADR-0078
  IMPLEMENTADO; ADR-0079 PROPOSTO (7/13-15; Categoria A
  5/5 ✓; Categoria B 2/3); ADR-0080 EM VIGOR.
- Saldo DEBTs: 11 preservado.
- **26 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" aplicação
  automática N=5 cumulativo** (P230+P231+P232+P233+P234)
  — pattern extremamente sólido.
- **Pattern "three-pass measure→place→emit" N=1
  inaugurado P234**.
- **Pattern "integração consumer pós-isolamento algorítmico
  em sub-passo posterior" N=1 inaugurado P234**.
- **Pattern "PlacedCell baseline P224.C suficiente sem
  refactor" confirmado N=2** (P230 audit; P234 integração).
- **Reuso `place_cells` N=1 cumulativo** — primeiro
  consumer geometric real.
- **Categoria B Fase 5 Layout: 2/3 → próximo B.3 fecha
  3/3** (per-cell algorítmico align/inset/breakable;
  valida pattern `.or()` N=2 → 3 atinge limiar
  formalização N=3-4).
- **Fase 5 Layout candidata: 7/13-15 sub-passos
  materializados** (~47-54% cumulativo; Categoria A 100%
  interna; Categoria B 67% interna).
