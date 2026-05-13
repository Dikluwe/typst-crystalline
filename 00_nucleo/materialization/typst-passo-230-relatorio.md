# Relatório do passo P230 — A.3 stroke/fill per-cell em GridCell + TableCell (Fase 5 Categoria A 3/5; primeira aplicação pós-ADR-0080 EM VIGOR)

**Data**: 2026-05-13.
**Spec**: `00_nucleo/materialization/typst-passo-230.md`.
**Tipo**: refino aditivo a 2 variants existentes (GridCell +
TableCell) + reuso helper P227 + parsing inline P228 +
renderização precedência override via `.or()` resolution.
**Magnitude planeada**: M (~2-2.5h). **Magnitude real**: M
(~1.5h — abaixo do limite por reuso massivo P227+P228 patterns
+ sem PlacedCell refactor).
**Marco**: **primeira aplicação automática ADR-0080 EM VIGOR
pós-promoção P229**; pattern "refino aditivo paralelo entre
variants irmãos" N=2 → **3 cumulativo** (Grid+Table P227/P228;
GridCell+TableCell P230).

---

## §1 O que foi feito

P230 materializa A.3 stroke/fill per-cell:
- **GridCell +2 fields** stroke + fill (5 → 7 fields).
- **TableCell +2 fields** stroke + fill paralelo (5 → 7
  fields; refino paralelo).
- **`native_grid_cell`/`native_table_cell` accept stroke +
  fill** (reuso `extract_stroke` P227 N=1 → 2; inline Color
  P228 paridade).
- **Renderização precedência override** em `layout_grid`:
  `effective_X = cell.X.or(grid.X)`; per-cell `Some` override;
  `None` inherit.
- **Z-order P227+P228 preservado**: fill efectivo →
  conteúdo → stroke efectivo.
- **L0 NÃO tocado automaticamente** (ADR-0080 EM VIGOR
  aplicação automática; primeira pós-promoção P229).
- 15 tests novos (4 unit + 6 stdlib + 5 E2E precedência);
  workspace **2071 → 2086 verdes** (+15); ~10 adaptações
  intencionais Cell constructors pre-existentes; 0
  regressões reais; 0 violations.

---

## §2 Inventário pré-P230 + audit cross-módulo (C1)

**Audit empírico**:
- `Stroke { paint: Color, thickness: f64 }` baseline ✓.
- `Value::Stroke` baseline P227 ✓.
- `Value::Color` baseline P25 ✓.
- `extract_stroke` `pub(super)` em `stdlib/layout.rs` ✓ —
  **cross-módulo a `stdlib/structural.rs` funciona** (P227
  `native_table` já importa via `crate::rules::stdlib::layout::extract_stroke`).
- GridCell 5 fields baseline P224.C; TableCell 5 fields
  baseline P157B ✓.
- `layout_grid` Z-order baseline P227+P228 correcto
  (fill antes; stroke depois) ✓.

**Decisão crítica C1**: cross-módulo OK sem promoção
visibility (`pub(super)` em `stdlib::layout` é acessível a
`stdlib::structural` via path completo `crate::rules::stdlib::layout::extract_stroke`).

Sem `P230.div-N`.

---

## §3 GridCell/TableCell refino +2 fields (C2+C3)

```rust
GridCell {
    body, x, y, colspan, rowspan,            // P224.C baseline
    stroke: Option<Stroke>,                   // P230 (A.3)
    fill:   Option<Color>,                    // P230 (A.3)
}

TableCell {
    body, x, y, colspan, rowspan,            // P157B baseline
    stroke: Option<Stroke>,                   // P230 paralelo
    fill:   Option<Color>,                    // P230 paralelo
}
```

GridCell: 5 → **7 fields**. TableCell: 5 → **7 fields**.

Arms cascata (compiler-driven):
- `entities/content.rs` (5 sítios refino: PartialEq,
  map_content, map_text + constructor `Content::table_cell`
  fix).
- `rules/introspect.rs` (2 arms: materialize_time + walk).
- `rules/layout/mod.rs` (2 arms: GridCell/TableCell
  layout_content isolados — ignoram stroke/fill fora de
  Grid context).
- `rules/layout/grid.rs` (extract per-cell + effective_*
  resolution).
- `rules/layout/grid_placement.rs` (destructure `..` para
  ignorar fields novos no `extract_cell_fields`).

---

## §4 `native_grid_cell`/`native_table_cell` accept (C5)

```rust
// Em ambos native_grid_cell e native_table_cell:
"stroke" => stroke = Some(extract_stroke(value, "grid_cell", "stroke")?),
"fill" => match value {
    Value::Color(c) => fill = Some(*c),
    other => return Err(/* "fill: espera Color, recebeu ..." */),
},
```

Reuso helper `extract_stroke` P227 **N=1 → 2 cumulativo**
(primeiro reuso pós-criação). Parsing inline Color
paridade P228 (rejeita Length explicitamente).

`native_grid_cell` em `stdlib/structural.rs`; `native_table_cell`
em `stdlib/structural.rs`. Cross-módulo import
`crate::rules::stdlib::layout::extract_stroke` já
disponível (P227 baseline).

---

## §5 Renderização precedência effective_* + Z-order (C6)

**Decisão pragmática**: sem refactor `PlacedCell` (spec
hipótese C6 sugeriu mas `layout_grid` actual itera `cells:
&[Content]` direct, sem usar `place_cells` algorítmico —
consumer geometric integration P224.C é B.2 candidato
distinto Fase 5).

**Implementação inline no loop**:
```rust
// P230 — extrair per-cell stroke/fill via match no Content.
let (cell_stroke, cell_fill) = match cell {
    Content::GridCell { stroke, fill, .. } => (stroke.as_ref(), fill.as_ref()),
    Content::TableCell { stroke, fill, .. } => (stroke.as_ref(), fill.as_ref()),
    _ => (None, None),
};
let effective_stroke: Option<&Stroke> = cell_stroke.or(stroke);
let effective_fill: Option<&Color> = cell_fill.or(fill);

// Z-order step 1: fill efectivo antes do conteúdo.
if let Some(c) = effective_fill { /* Rect emit */ }

// Z-order step 2: conteúdo cell (existing).

// Z-order step 3: stroke efectivo depois.
if let Some(s) = effective_stroke { /* 4 Lines emit */ }
```

**Precedência paridade vanilla literal**: cell `Some(...)`
override Grid-level; cell `None` inherit Grid-level via
`.or()` resolution.

---

## §6 Decisões substantivas + primeira aplicação automática ADR-0080 EM VIGOR

**8 decisões fixadas**:
- **Decisão 1** — Opção α fields restritos (stroke + fill;
  align/inset/breakable per-cell são Categoria B.3 separado).
- **Decisão 2** — Opção α precedência override completo
  (paridade vanilla literal via `.or()`).
- **Decisão 3** — Opção α Z-order limpo cada cell uma vez.
- **Decisão 4** — Reuso helper `extract_stroke` N=1 → 2.
- **Decisão 5** — Tests E2E precedência 5 explícitos.
- **Decisão 6** — **Opção γ aplicação automática ADR-0080
  EM VIGOR** sem decisão explícita Opção γ por sub-passo.
- **Decisão 7** — Opção α refino paralelo TableCell
  (pattern N=2 → 3 cumulativo).
- **Decisão 8** — `extract_stroke` reuso N=1 → 2 (patamar
  trivial; sem promoção pública).

**ADR-0080 EM VIGOR aplicação automática**:
- L0 prompts `entities/content.md` + `rules/stdlib.md`
  **NÃO tocados** em P230.
- `crystalline-lint --fix-hashes`: "Nothing to fix" em L0
  layer.
- **Primeira aplicação automática pós-promoção P229**:
  regra metodológica formal aplicada por defeito (sem
  decisão explícita por sub-passo em spec individual).
- **Pattern emergente "aplicação automática ADR EM VIGOR
  sem decisão explícita por sub-passo" N=1 inaugurado P230**.

**Anti-inflação 22ª aplicação cumulativa** pós-P205D
(Opção α fields restritos + Opção α precedência + Opção γ
L0 automático + helper reuso + refino paralelo variants
irmãos + sem refactor PlacedCell + sem promoção helper
público + ADR-0079 sem promoção).

---

## §7 Resultados verificação + inventário 148 + ADR-0079 (C7+C9+C10+C11)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | ~2086 verdes | **2086 verdes** (1797+242+24+2+21) ✓ |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | **"Nothing to fix"** ✓ (L0 não tocado automático) |
| Adaptações pre-existentes | N=3-7 | **N=~10** (P224 GridCell + P157B TableCell + P224.C grid_placement.rs constructors) |
| GridCell fields | 5 → 7 | ✓ |
| TableCell fields | 5 → 7 | ✓ |
| Regressões reais | 0 | **0** |

**Inventário 148**:
- §A.5 Layout linha 141 `grid(...)`: footnote `⁵ ⁴⁵ ⁴⁶ ⁴⁷
  ⁴⁸` → `⁵ ⁴⁵ ⁴⁶ ⁴⁷ ⁴⁸ ⁴⁹`. Sem reclassificação
  categórica.
- Footnote ⁴⁹ adicionada (~110 linhas) documentando A.3
  materializado + 8 decisões + primeira aplicação automática
  ADR-0080 EM VIGOR + patterns cumulativos.

**ADR-0079**:
- §"Aplicações cumulativas" anotado com bloco
  `### P230 anotação — Categoria A sub-passo 3 (stroke/fill
  per-cell GridCell + TableCell; precedência override)`.
- Status ADR-0079 mantido PROPOSTO (3/13-15 sub-passos).
- Categoria A: 3/5 materializados (A.1 ✓; A.2 ✓; A.3 ✓;
  A.4-A.5 pendentes).

---

## §8 Próximo sub-passo

P230 fecha terceiro sub-passo Fase 5 Layout candidata
(Categoria A 3/5). Decisão humana sobre próxima sessão:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **A.4 Block/Boxed** | outset/radius/clip P156G+H scope-outs | M (~2-3h cada) | média (refinos ortogonais Grid/Table) |
| **A.5 Place per-cell** | Place dentro Grid com align: ? per-cell | S+ (~1h) | média (fecha Categoria A 5/5) |
| **B.1 DEBT-34d** | Auto track sizing algorítmico (fecha DEBT-34d preservado) | M (~2-3h) | média (fecha DEBT preservado per `P224.div-1`) |
| **B.2 Consumer geometric** | `place_cells` algorítmico → Layouter geometric | M (~2-3h) | média (integra P224.C com layout_grid) |
| **D.1 state runtime** | runtime mutable; desbloqueia ADR-0066 IMPLEMENTADO | M (~2-3h) | alta (+33pp Introspection; primeira feature runtime) |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |

**Recomendação subjectiva**: **A.5 Place per-cell**
(S+ ~1h) — fecha Categoria A próxima (4/5); momentum
cumulativo P227 → P228 → P230 → A.5 → A.4 natural.
Alternativa: **A.4 Block/Boxed** se prefere refinos
P156G+H scope-outs sequencialmente; ou **D.1 state** se
humano priorizar runtime queries desbloqueio.

**Decisão humana fica em aberto literal** pós-P230.

**Estado pós-P230**:
- Tests workspace: 2071 → **2086 verdes** (+15 P230).
- Content variants: 59 preservado.
- Value variants: 55 preservado.
- Stdlib funcs: 60 preservado (reuso helper; sem nova).
- GridCell fields: 5 → **7** (+stroke + fill).
- TableCell fields: 5 → **7** (+stroke + fill paralelo).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- ADR-0066 PROPOSTO; ADR-0061 IMPLEMENTADO; ADR-0078
  IMPLEMENTADO; ADR-0079 PROPOSTO; **ADR-0080 EM VIGOR**.
- Saldo DEBTs: 12 preservado.
- **22 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" — primeira
  aplicação automática N=1** pós-EM VIGOR (precedente
  para todos refinos aditivos seguintes).
- **Pattern "refino aditivo paralelo entre variants
  irmãos" N=3 cumulativo** consolidado.
- **Pattern "aplicação automática ADR EM VIGOR sem
  decisão explícita por sub-passo" N=1 inaugurado P230**.
- **Pattern "precedência per-cell vs container-level via
  `.or()` resolution" N=1 inaugurado P230** (reusável
  A.4+B.3+).
- **Helper `extract_stroke` reuso N=2** (patamar trivial).
- **Categoria A Fase 5 Layout**: 3/5 → próximos A.4
  Block/Boxed (cosméticos refinos P156G+H scope-outs);
  A.5 Place per-cell.
- **Fase 5 Layout candidata**: 3/13-15 sub-passos
  materializados (P227 A.1 ✓; P228 A.2 ✓; **P230 A.3 ✓**;
  restantes pendentes).
