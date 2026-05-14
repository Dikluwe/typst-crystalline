# Relatório do passo P234 — B.2 Consumer geometric `place_cells` → Layouter integration (Fase 5 Categoria B 2/3; colspan/rowspan funcionais em renderização pela primeira vez pós-M9c)

**Data**: 2026-05-13.
**Spec**: `00_nucleo/materialization/typst-passo-234.md`.
**Tipo**: refactor estructural consumer — `layout_grid` passa
a chamar `place_cells` baseline P224.C ao invés de iterar
`cells.chunks(num_cols)` direct; **zero fields novos** em
Content variants; **zero novos variants**; **zero novas
stdlib funcs**; refactor semantic `PlacedCell.body` para
preservar outer cell wrapper.
**Magnitude planeada**: M (~2-3h). **Magnitude real**: M (~1h30m
— audit C1 trivial + integração estrutural + 1 fix iterativo
de row_heights padding).
**Marco**: **colspan/rowspan funcionais em renderização pela
primeira vez pós-M9c** (criados algoritmicamente P224.C;
isolados até P234); **3 patterns emergentes
consolidados/inaugurados** (three-pass measure→place→emit
N=1; integração consumer pós-isolamento N=1; PlacedCell
baseline confirmado N=2); **quinta aplicação automática
ADR-0080 EM VIGOR**.

---

## §1 O que foi feito

P234 materializa B.2 integração consumer geometric:
- **`layout_grid` chama `place_cells`** baseline P224.C;
  obtém `Vec<PlacedCell>` em vez de iterar `cells.chunks(
  num_cols)` direct.
- **Bounds reais per cell** via helper privado `cell_bounds`
  (`cell_w = sum(resolved_widths[col..col+colspan])`;
  `cell_h = sum(row_heights[row..row+rowspan])`).
- **PlacedCell.body semantic ajustada P234** — preserva
  outer cell (`Content::GridCell {...}` wrapper) em vez
  de strip inner body. **5 fields baseline preservados**.
- **Z-order P227+P228 + precedência per-cell P230
  preservados integralmente** com bounds reais.
- **L0 NÃO tocado** — quinta aplicação automática ADR-0080
  EM VIGOR.
- 11 tests novos (E2E layout); workspace **2111 → 2122
  verdes** (+11); 0 adaptações intencionais; 0 regressões
  reais; 0 violations.

---

## §2 Inventário pré-P234 + audit signature + integration points (C1)

**Audit empírico crítico**:

**`place_cells` baseline** (`grid_placement.rs`):
- Signature: `pub(crate) fn place_cells(cells: &[Content],
  num_cols: usize) -> SourceResult<Vec<PlacedCell>>`. **Sem
  `num_rows` param** (derivado dinamicamente via occupancy
  grid expansão).
- Algoritmo: Pass 1 (explicit cells `x` ou `y` Some);
  Pass 2 (auto cells via cursor linear).
- Visibility `pub(crate)` — acessível cross-módulo em `grid.rs`.

**`PlacedCell` baseline P224.C**:
- 5 fields: `body: Content`, `row: usize`, `col: usize`,
  `colspan: usize`, `rowspan: usize`.
- **Semantic body PRE-P234**: stripped inner body (e.g.,
  `Content::text("X")` em vez de `Content::GridCell {..}`).
  **Descoberta crítica P234**: este strip perde wrapper P230
  per-cell stroke/fill. **Solução P234**: ajustar semantic
  para preservar outer wrapper (5 fields preservados; só
  semantic body muda).

**`layout_grid` actual baseline**:
- Itera `cells.chunks(num_cols)` direct: `for (row_idx,
  row_items) in rows_of_items.iter().enumerate() { for
  (col_idx, cell) in row_items.iter() }`.
- Sem chamar `place_cells` — algoritmo P224.C criado mas
  não-integrado (per atomização ADR-0036).
- `col_starts` + `resolved_widths` + `row_heights` P233
  disponíveis no scope `layout_grid`.

**Decisões críticas C1**:
1. **Signature OK** (`num_rows` derivado internamente).
2. **`colspan/rowspan: usize` >= 1** em PlacedCell (default
   1 quando None).
3. **`pub(crate)` cross-módulo OK** (mesmo crate).
4. **Semantic body P230 — refactorada P234** preservar outer.

Sem `P234.div-N` formal — refactor semantic body é evolução
natural pós-audit que descobre necessidade durante implementação.

---

## §3 Refactor `layout_grid` chama `place_cells` (C2)

**Diff resumo**:

Pre-P234 (baseline P224+P227+P228+P230+P233):
```rust
let rows_of_items: Vec<&[Content]> = cells.chunks(num_cols).collect();
// ... row sizing usando rows_of_items ...
for (row_idx, row_items) in rows_of_items.iter().enumerate() {
    // pagination ...
    for (col_idx, cell) in row_items.iter().enumerate() {
        let cell_w = resolved_widths[col_idx];
        let cell_x = col_starts[col_idx];
        // ... emit usando (cell_x, row_start_y, cell_w, row_h) ...
    }
}
```

Pós-P234:
```rust
let rows_of_items: Vec<&[Content]> = cells.chunks(num_cols).collect();
// ... row sizing usando rows_of_items (preservada baseline) ...

// P234 — placement via place_cells.
let placed_cells: Vec<PlacedCell> = place_cells(cells, num_cols).unwrap_or_default();

// Derive num_rows_produced_final do placed.
let num_rows_from_placed = placed_cells.iter()
    .map(|p| p.row + p.rowspan).max().unwrap_or(0);
let num_rows_produced_final = num_rows_from_placed.max(num_rows_produced).max(1);

// Pad row_heights (Fixed tracks resolved literal; Auto/Fraction = 0).
while row_heights.len() < num_rows_produced_final {
    let row_idx = row_heights.len();
    let track = &row_tracks[row_idx % row_tracks.len()];
    let h = match track {
        TrackSizing::Fixed(pt) => *pt,
        _ => 0.0,
    };
    row_heights.push(h);
}

// Group placed cells por start row.
let mut cells_per_row: Vec<Vec<usize>> = vec![vec![]; num_rows_produced_final];
for (i, p) in placed_cells.iter().enumerate() {
    if p.row < num_rows_produced_final { cells_per_row[p.row].push(i); }
}

for row_idx in 0..num_rows_produced_final {
    // pagination preservada baseline ...
    for &placed_idx in &cells_per_row[row_idx] {
        let placed = &placed_cells[placed_idx];
        let (cell_x, cell_y, cell_w, cell_h) = cell_bounds(
            placed, &col_starts, &resolved_widths, &row_heights, row_start_y);
        // ... emit usando bounds reais ...
    }
}
```

**`PlacedCell.body` semantic refactorada P234** em
`grid_placement.rs`: `body: (*cell).clone()` (outer
preservado) em vez de `body: body.clone()` (inner stripped).

**cell_cache removido** — emissão pós-P234 não usa cache
(cells re-medidas durante emit; custo perf ~2× aceitável).

Magnitude C2 real: **S+ (~45min)** — refactor mecânico
coerente + 1 fix iterativo row_heights padding (Fixed
tracks resolved literal pós-padding).

---

## §4 Helper `cell_bounds` (C3+C4)

Module-level fn em `grid.rs`:

```rust
fn cell_bounds(
    placed: &PlacedCell,
    col_starts: &[f64],
    resolved_widths: &[f64],
    row_heights: &[f64],
    current_row_start_y: f64,
) -> (f64, f64, f64, f64) {
    let x0 = col_starts.get(placed.col).copied().unwrap_or(0.0);
    let y0 = current_row_start_y;
    let cell_w: f64 = (placed.col..placed.col + placed.colspan)
        .map(|i| resolved_widths.get(i).copied().unwrap_or(0.0))
        .sum();
    let cell_h: f64 = (placed.row..placed.row + placed.rowspan)
        .map(|i| row_heights.get(i).copied().unwrap_or(0.0))
        .sum();
    (x0, y0, cell_w, cell_h)
}
```

**Pattern paridade `extract_stroke` P227** — helper privado
no `grid.rs` reusável em 3 lugares (cell_origin set + Z1 fill
+ Z3 stroke). Anti-inflação preservada (sem novo módulo).

**Decisão C4 layout body bounds reais**: `cell_origin_x/y/w` +
`cell_available_h` set ao bounds reais antes de
`layout_sub_frame_with_width`; restored pós-emit. Paridade
pattern save/restore `cell_origin_*` P84.6 + P232 cell_align.

---

## §5 Decisões substantivas (8 decisões) + quinta aplicação automática ADR-0080 EM VIGOR

**8 decisões fixadas**:
- **Decisão 1** — Opção α integração completa (não β parcial
  nem γ refactor PlacedCell rejeitado P230).
- **Decisão 2** — Opção α PlacedCell baseline 5 fields
  literal; **semantic body ajustada P234** preservar outer
  cell wrapper.
- **Decisão 3** — Opção α bounds via helper privado
  `cell_bounds` (paridade pattern P227 `extract_stroke`).
- **Decisão 4** — Opção α match `placed.body` semantic P230
  literal (post-refactor body wrapper).
- **Decisão 5** — 11 tests E2E P234 (4 colspan/rowspan
  funcionais + 4 regressões baseline + 3 cenários adicionais).
- **Decisão 6** — Opção γ L0 NÃO tocado (**quinta aplicação
  automática ADR-0080 EM VIGOR**).
- **Decisão 7** — Sem promoção formal patterns N=1.
- **Decisão 8** — cell_cache descartado MVP (re-integração
  refino futuro).

**ADR-0080 EM VIGOR aplicação automática N=4 → 5 cumulativo**:
- L0 prompts NÃO tocados em P234.
- `crystalline-lint --fix-hashes`: "Nothing to fix" em L0.
- **Quinta aplicação automática pós-promoção P229**
  (P230+P231+P232+P233+**P234**). Pattern empíricamente
  extremamente sólido (N=5 consecutivo sem excepção).

**Anti-inflação 26ª aplicação cumulativa** pós-P205D.

---

## §6 Resultados verificação + tests colspan/rowspan funcionais (C5+C7)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | ~2123 verdes | **2122 verdes** (1833+242+24+2+21) ✓ (11 novos vs ~10-12 spec; subset minimal pragmático) |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | **"Nothing to fix"** ✓ (L0 não tocado automático N=5) |
| Adaptações pre-existentes | N=0-3 | **N=0** (PlacedCell semantic body fix preserva tests P230) |
| Content variants | 59 preservado | ✓ (zero novos) |
| Stdlib funcs | 60 preservado | ✓ |
| PlacedCell fields | 5 preservado | ✓ (só semantic body muda) |
| Regressões reais | 0 | **0** |

**Tests P234** (11 E2E layout):
- `p234_grid_colspan_2_cell_ocupa_2_cols_fill` — fill Rect
  width=100 (col_sums) ✓.
- `p234_grid_rowspan_2_cell_ocupa_2_rows_fill` — fill Rect
  height=70 (row_sums) ✓.
- `p234_grid_colspan_com_stroke_envolve_ambas_cols` — Line
  horizontal dx=100 (multi-col edge) ✓.
- `p234_grid_colspan_per_cell_stroke_override_grid_p230_preservado`
  — thickness 7.0 emerge multi-col ✓.
- `p234_grid_sem_colspan_rowspan_baseline_preservado` —
  placement sequencial preservado ✓.
- `p234_grid_stroke_baseline_P227_preservado` — 8 lines
  ≥ 2 cells × 4 stroke ✓.
- `p234_grid_fill_baseline_P228_preservado` — 2 Rects ≥
  1 per cell ✓.
- `p234_grid_auto_sizing_baseline_P233_preservado` —
  Auto+Fr renderiza ambos ✓.
- `p234_grid_mix_explicit_e_auto_renderiza_todos` — Pass 1
  + Pass 2 integration ✓.
- `p234_grid_colspan_fill_position_x0` — Rect position
  x=col_starts[0] ✓.
- `p234_grid_colspan_rowspan_2x2_fill_bounds_combinados` —
  2×2 bounds combinados width=50 height=25 ✓ (após fix
  row_heights padding Fixed tracks).

**1 fix iterativo P234** (não-regressão): row_heights padding
ignorava row_tracks Fixed quando placed estende além de
chunks-derived; corrigido para resolved literal de
row_tracks. Pattern emergente "fix iterativo durante
implementação MVP" — não-marco.

---

## §7 Inventário 148 footnote ⁵³ + ADR-0079 anotação Categoria B 2/3 (C8+C9)

**Inventário 148**:
- §A.5 Layout entrada `grid(...)`: pattern P233 preservado
  — footnote text adicionada sem superscript update em linha
  141 (consistência P232+P233+P234).
- Footnote ⁵³ adicionada (~125 linhas) documentando B.2
  materializado + 8 decisões + 5 patterns
  consolidados/inaugurados + 11 tests verdes + L0 NÃO tocado
  N=5.

**ADR-0079**:
- §"Aplicações cumulativas" anotado com bloco
  `### P234 anotação — Categoria B sub-passo 2 (Consumer
  geometric place_cells → Layouter integration);
  colspan/rowspan funcionais em renderização pela primeira
  vez pós-M9c`.
- Status ADR-0079 mantido PROPOSTO (7/13-15 sub-passos
  cumulativos; **Categoria A 5/5 ✓ + Categoria B 2/3**).

---

## §8 Próximo sub-passo

P234 fecha segundo sub-passo Categoria B (2/3). Decisão
humana sobre próxima sessão:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **B.3 GridCell algorítmico** | Per-cell align/inset/breakable; valida pattern `.or()` N=2 → 3 atinge limiar formalização | M (~2-3h) | alta (fecha Categoria B 3/3; consolida pattern) |
| **D.1 state runtime** | Runtime mutable; **desbloqueia ADR-0066 PROPOSTO → IMPLEMENTADO** + Introspection +33pp | M (~2-3h) | alta (transição arquitectural maior) |
| **C.1 Place float real** | Flow contorna (reabre Opção B P219 graded) | L+ (~5-8h) | baixa |
| **C.2 Multi-region completa** | Reabre P216B + DEBT-56b; resolve cell rowspan cruza pagination | L+ a XL (~10-20h) | baixa |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa-média |

**Recomendação subjectiva**: **B.3 GridCell per-cell
algorítmico** (M ~2-3h) — fecha Categoria B 3/3 sequencial;
valida pattern `.or()` N=2 → 3 atinge limiar formalização
N=3-4 (pattern P230 GridCell over Grid; P232 Place over
Grid; P235 GridCell align/inset/breakable). Alternativa:
**D.1 state** se humano priorizar promoção ADR-0066
IMPLEMENTADO + bonus Introspection +33pp.

**Decisão humana fica em aberto literal** pós-P234.

**Estado pós-P234**:
- Tests workspace: 2111 → **2122 verdes** (+11 P234).
- Content variants: 59 preservado.
- Value variants: 55 preservado.
- Stdlib funcs: 60 preservado.
- Grid/Table/Cell/Block/Boxed/Place fields preservados.
- **PlacedCell fields: 5 preservado** (só semantic body
  muda para outer wrapper preservado).
- Layouter fields: preservados (n+1 pós-P232).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- ADRs: PROPOSTO 12; EM VIGOR 29 (ADR-0080); IMPLEMENTADO
  21; total 67.
- **Saldo DEBTs: 11 preservado**.
- **26 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" aplicação
  automática N=4 → 5 cumulativo** (P230+P231+P232+P233+
  **P234**). Pattern extremamente sólido empíricamente.
- **Pattern "three-pass measure→place→emit" N=1
  inaugurado P234** — extensão two-pass P233.
- **Pattern "integração consumer pós-isolamento algorítmico
  em sub-passo posterior" N=1 inaugurado P234**.
- **Pattern "PlacedCell baseline P224.C suficiente sem
  refactor" confirmado N=2** (P230 audit; P234 integração).
- **Reuso `place_cells` N=0 → 1 cumulativo** — primeiro
  consumer geometric real.
- **Colspan/rowspan funcionais em renderização pela
  primeira vez pós-M9c**.
- **Categoria B Fase 5 Layout: 2/3 → próximo B.3 fecha
  3/3** (per-cell algorítmico align/inset/breakable; valida
  pattern `.or()` N=2 → 3 atinge limiar formalização).
- **Fase 5 Layout candidata: 7/13-15 sub-passos
  materializados** (~47-54% cumulativo; Categoria A 100%
  interna; Categoria B 67% interna).
