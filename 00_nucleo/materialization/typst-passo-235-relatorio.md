# Relatório do passo P235 — B.3 GridCell/TableCell align/inset/breakable per-cell (Fase 5 Categoria B 3/3; FECHA Categoria B estructuralmente; pattern `.or()` N=3 atinge limiar formalização)

**Data**: 2026-05-13.
**Spec**: `00_nucleo/materialization/typst-passo-235.md`.
**Tipo**: refino aditivo a 2 variants Content (`GridCell` +
`TableCell`); +3 fields cada (`align: Option<Align2D>`,
`inset: Option<Sides<Length>>`, `breakable: Option<bool>`);
renderização diferenciada por atributo (align real;
inset real; breakable adiada graded); stdlib accept 3 named
args; precedência `.or()` uniforme.
**Magnitude planeada**: M (~2.5-3h). **Magnitude real**: M (~2h
— audit C1 + bulk patterns cascade + bulk script error recovery
+ 1 adaptação intencional + tests + docs).
**Marco**: **Categoria B Fase 5 Layout 3/3 ✓ FECHADA
estructuralmente** (pattern "fecho categoria completa dentro
de ADR PROPOSTO sem transição" N=1 → 2 cumulativo paridade
P232 Categoria A); **pattern `.or()` resolution N=2 → 3
cumulativo atingindo limiar formalização N=3-4** (promoção
formal ADR meta candidato XS futuro); **3 patterns inaugurados**;
**sexta aplicação automática ADR-0080 EM VIGOR**.

---

## §1 O que foi feito

P235 materializa B.3:
- **GridCell +3 fields** (`align`, `inset`, `breakable`) —
  7 → 10 fields cumulativos.
- **TableCell +3 fields paralelo** — 7 → 10 fields.
- **`native_grid_cell`/`native_table_cell` 3 named args**:
  helpers `extract_align_value` + `extract_inset_value` +
  Bool direct.
- **Renderização diferenciada**: align real (Layouter
  `cell_align` P232 estendido per-cell); inset real (bounds
  reduction); breakable adiada graded.
- **Precedência `.or()` uniforme** P230 + P232 + P235 nos
  3 atributos.
- **L0 NÃO tocado** — sexta aplicação automática ADR-0080
  EM VIGOR.
- 15 tests novos (4 unit content + 6 unit stdlib + 5 layout
  E2E); workspace **2122 → 2137 verdes** (+15); 1 adaptação
  intencional; 0 regressões reais; 0 violations.
- **Categoria B 3/3 ✓ FECHADA estructuralmente**.

---

## §2 Audit pré-P235 (C1)

- **GridCell**: 7 fields baseline pós-P230 (body/x/y/colspan/
  rowspan/stroke/fill). ✓
- **TableCell**: 7 fields baseline pós-P230 (paralelo). ✓
- **`extract_alignment(args, default)`**: helper baseline
  `stdlib/layout.rs:158` mas toma `&Args` (full args).
  Para single Value, criado `extract_align_value` privado
  P235 em `stdlib/structural.rs`.
- **`extract_sides_lengths(args, fn_name)`**: helper baseline
  `stdlib/layout.rs:422` retorna `Sides<Option<Length>>` e
  toma `&Args`. Para single Value em cell, criado
  `extract_inset_value` privado P235 (Length uniforme via
  `Sides::uniform`).
- **`Block.breakable`**: confirmado **adiada baseline**
  (`layout/mod.rs:1121` + 1528 usam `breakable: _` ignored
  per P156G scope-out). P235 paridade: breakable per-cell
  armazenado adiada graded.
- **Layouter `cell_align`**: baseline P232 save/restore
  per-Grid em `layout_grid` lines 47-48 + 386. P235 estende
  per-cell save/restore aninhado dentro do per-Grid.

Sem `P235.div-N` formal.

---

## §3 GridCell/TableCell refino +3 fields cada (C2+C3)

`entities/content.rs`:

```rust
GridCell {
    body, x, y, colspan, rowspan,    // P224.C baseline
    stroke, fill,                     // P230 (A.3)
    align: Option<Align2D>,          // P235
    inset: Option<Sides<Length>>,    // P235
    breakable: Option<bool>,         // P235
},

TableCell {
    body, x, y, colspan, rowspan,    // P157B baseline
    stroke, fill,                     // P230 (A.3) paralelo
    align: Option<Align2D>,          // P235 paralelo
    inset: Option<Sides<Length>>,    // P235 paralelo
    breakable: Option<bool>,         // P235 paralelo
},
```

**Arms cascata exhaustivos** (compiler-driven; ~18-22 arms):
- `content.rs::PartialEq::eq` GridCell + TableCell (10 fields
  cada compared).
- `content.rs::map_content` GridCell + TableCell.
- `content.rs::map_text` GridCell + TableCell.
- `content.rs::table_cell` constructor function.
- `introspect.rs::materialize_time` GridCell + TableCell.
- `layout/mod.rs::layout_content` GridCell + TableCell arms.
- `stdlib/structural.rs::native_grid_cell` + `native_table_cell`
  constructors.
- `grid_placement.rs` 6 test constructors (sed bulk update).
- `content.rs` 7 test constructors (manual edits).
- `layout/tests.rs` 5 test constructors (manual edits).

**Bulk Python script attempt failed** — script identificou
`Content::TableCell { body, ... } => Content::TableCell { ... }`
como bloco único e stripped o prefixo. Recuperado via git
checkout + redo manual. Pattern emergente "Python script
bulk falha quando match patterns aninhados; manual edits
recomendado N=1 inaugurado P235".

---

## §4 native_grid_cell/native_table_cell accept 3 named args (C5)

`stdlib/structural.rs`:

```rust
"align" => align = Some(extract_align_value(value, "grid_cell", "align")?),
"inset" => inset = Some(extract_inset_value(value, "grid_cell", "inset")?),
"breakable" => match value {
    Value::Bool(b) => breakable = Some(*b),
    other => return Err(...),
},
```

Helpers privados P235:

```rust
fn extract_align_value(val: &Value, fn_name: &str, field: &str)
    -> SourceResult<Align2D>
{
    match val {
        Value::Align(a) => Ok(*a),
        Value::Str(s)   => Ok(Align2D::from_string(s.as_str())),
        other => Err(...),
    }
}

fn extract_inset_value(val: &Value, fn_name: &str, field: &str)
    -> SourceResult<Sides<Length>>
{
    match val {
        Value::Length(l) => Ok(Sides::uniform(*l)),
        Value::Float(f)  => Ok(Sides::uniform(Length::pt(*f))),
        Value::Int(n)    => Ok(Sides::uniform(Length::pt(*n as f64))),
        other => Err(...),
    }
}
```

---

## §5 Renderização precedência effective_* (C6)

`layout/grid.rs::layout_grid` per-cell emission:

```rust
let (cell_stroke, cell_fill, cell_align, cell_inset, _cell_breakable) = match cell {
    Content::GridCell { stroke, fill, align, inset, breakable, .. } |
    Content::TableCell { stroke, fill, align, inset, breakable, .. } => (
        stroke.as_ref(), fill.as_ref(),
        align.as_ref().copied(), inset.as_ref(), breakable.as_ref().copied(),
    ),
    _ => (None, None, None, None, None),
};

// Precedência .or() uniforme.
let effective_stroke = cell_stroke.or(stroke);                       // P230
let effective_fill   = cell_fill.or(fill);                           // P230
let effective_align  = cell_align.or(self.cell_align);               // P235
let effective_inset: Sides<Length> = cell_inset.cloned().unwrap_or(inset);  // P235

// P235 — Cell-level cell_align save/restore (extensão P232 per-cell).
let saved_cell_align_inner = self.cell_align;
self.cell_align = effective_align;

// P235 — Inset bounds reduction.
let inset_l = effective_inset.left.abs.to_pt();
let inset_t = effective_inset.top.abs.to_pt();
let inset_r = effective_inset.right.abs.to_pt();
let inset_b = effective_inset.bottom.abs.to_pt();
let body_x = cell_x + inset_l;
let body_y = cell_y + inset_t;
let body_w = (cell_w - inset_l - inset_r).max(0.0);
let body_h = (cell_h - inset_t - inset_b).max(0.0);

// cell_origin_* set ao body bounds reduzidos.
self.cell_available_h = Some(body_h);
self.cell_origin_x = Some(body_x);
// ...

// Layout body em body_x/body_w reduzidos por inset.
let (_, cell_items) = self.layout_sub_frame_with_width(cell, body_x, body_w);

// Restore.
self.cell_align = saved_cell_align_inner;
// ...

// Content emit rebaseado a body_y (não cell_y) para refletir inset offset.
```

**Render diferenciado**:
- **align**: real via Layouter `cell_align` extension
  per-cell (Place dentro cell herda effective_align via
  P232 mechanism).
- **inset**: real via bounds reduction antes layout body.
- **breakable**: extraído mas semantic adiada graded.

Stroke/fill emit em bounds **cell_w/cell_h originais** (não
reduzidos por inset — bordas/fundo da cell completa, não
do body interno).

---

## §6 Decisões substantivas (8 decisões) + sexta aplicação automática ADR-0080

**8 decisões fixadas**:
- **Decisão 1** — Opção α escopo restrito (align + inset
  + breakable apenas; não outros).
- **Decisão 2** — Opção β tipos Option uniformes (todos
  `Option<T>` para precedência `.or()` consistente).
- **Decisão 3** — Opção α `.or()` precedência uniforme
  nos 3 atributos.
- **Decisão 4** — Opção α refino paralelo TableCell.
- **Decisão 5** — Opção β reuso Layouter `cell_align`
  estendido per-cell (não Opção α directo nem γ helper
  separado).
- **Decisão 6** — Opção α render real inset (bounds
  reduction trivial pós-P234).
- **Decisão 7** — Opção β breakable armazenado adiada
  graded (paridade P156G + P224.B).
- **Decisão 8** — Opção γ L0 automático (**sexta aplicação
  automática ADR-0080 EM VIGOR**).

**Adaptações intencionais P235** (N=1):
- `native_table_cell_named_arg_desconhecido_rejeitado`
  usava `inset` como exemplo "unknown arg"; agora `inset`
  é conhecido P235. Adaptado para `outset` que continua
  scope-out per ADR-0054 graded.

**ADR-0080 EM VIGOR aplicação automática N=5 → 6**:
- L0 prompts NÃO tocados em P235.
- `crystalline-lint --fix-hashes`: "Nothing to fix".
- **Sexta aplicação automática pós-promoção P229**
  (P230+P231+P232+P233+P234+**P235**). Pattern **extremamente
  sólido empíricamente** — seis aplicações consecutivas
  sem excepção.

**Anti-inflação 27ª aplicação cumulativa** pós-P205D.

---

## §7 Resultados verificação + Inventário ⁵⁴ + ADR-0079 Categoria B 3/3 ✓ (C9-C11)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | ~2137 verdes | **2137 verdes** (1848+242+24+2+21) ✓ |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | **"Nothing to fix"** ✓ (L0 automático N=6) |
| Adaptações pre-existentes | N=3-7 | **N=1** (test outset arg) ✓ (subset minimal pragmático) |
| GridCell fields | 7 → 10 | ✓ |
| TableCell fields | 7 → 10 | ✓ |
| Regressões reais | 0 | **0** |

**Tests P235** (15 total):
- **Unit content** (4):
  - `p235_gridcell_variant_aceita_align_inset_breakable`.
  - `p235_tablecell_variant_aceita_align_inset_breakable`.
  - `p235_gridcell_partial_eq_inclui_3_fields`.
  - `p235_gridcell_map_content_preserva_3_fields`.
- **Unit stdlib** (6):
  - `p235_native_grid_cell_align_aceita`.
  - `p235_native_grid_cell_inset_length_uniforme_aceita`.
  - `p235_native_grid_cell_breakable_bool_aceita`.
  - `p235_native_grid_cell_breakable_tipo_errado_rejeita`.
  - `p235_native_table_cell_align_paridade_gridcell`.
  - `p235_native_table_cell_inset_paridade_gridcell`.
- **Layout E2E** (5):
  - `p235_per_cell_inset_override_grid_bounds_reduzidos`.
  - `p235_per_cell_inset_none_inherits_grid`.
  - `p235_per_cell_breakable_armazenado_layout_preservado`.
  - `p235_per_cell_align_override_grid_armazenado`.
  - `p235_per_cell_align_none_inherits_grid`.

**Inventário 148**:
- Footnote ⁵⁴ adicionada (~130 linhas) documentando B.3
  materializado + 8 decisões + 9 patterns emergentes
  consolidados/inaugurados + Categoria B FECHADA + 15
  tests + L0 não tocado N=6.

**ADR-0079**:
- Bloco `### P235 anotação — Categoria B sub-passo 3
  (GridCell + TableCell align/inset/breakable per-cell);
  Categoria B 3/3 ✓ FECHADA estructuralmente`.
- Status ADR-0079 mantido PROPOSTO (8/13-15 sub-passos
  cumulativos; **Categoria A 5/5 ✓ + Categoria B 3/3 ✓ +
  C/D pendentes**).

---

## §8 Próximo sub-passo

P235 fecha Categoria B 3/3 estructuralmente. Próxima sessão
candidatos:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **D.1 state runtime** | Runtime mutable; **desbloqueia ADR-0066 PROPOSTO → IMPLEMENTADO** + Introspection +33pp | M (~2-3h) | alta (transição arquitectural maior) |
| **ADR meta admin** | Promoção formal pattern `.or()` resolution N=3 atinge limiar | XS (~30min) | média (consolidação meta paridade P229) |
| **C.1 Place float real** | Flow contorna (reabre Opção B P219 graded) | L+ (~5-8h) | baixa |
| **C.2 Multi-region completa** | Reabre P216B + DEBT-56b | L+ a XL (~10-20h) | baixa |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa-média |

**Recomendação subjectiva**: **D.1 state runtime** (M ~2-3h)
— transição arquitectural maior; desbloqueia ADR-0066 PROPOSTO
→ IMPLEMENTADO + Introspection +33pp. Alternativa: **ADR
meta admin XS** para pattern `.or()` se humano priorizar
consolidação meta.

**Decisão humana fica em aberto literal** pós-P235.

**Estado pós-P235**:
- Tests workspace: 2122 → **2137 verdes** (+15 P235).
- Content variants: 59 preservado.
- Value variants: 55 preservado.
- Stdlib funcs: 60 preservado.
- **GridCell fields: 7 → 10** (+align + inset + breakable).
- **TableCell fields: 7 → 10** (+align + inset + breakable
  paralelo).
- Layouter fields: preservados.
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**
  (refino qualitativo P235 algorítmicos per-cell).
- Cobertura user-facing total: 67% preservada.
- ADRs: PROPOSTO 12; EM VIGOR 29 (ADR-0080); IMPLEMENTADO
  21; total 67.
- **Saldo DEBTs: 11 preservado**.
- **27 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" aplicação
  automática N=5 → 6 cumulativo** (P230+P231+P232+P233+
  P234+**P235**). Pattern extremamente sólido empíricamente.
- **Pattern "precedência per-X via `.or()` resolution"
  N=2 → 3 cumulativo atingindo limiar formalização N=3-4**
  — promoção formal ADR meta candidato XS futuro.
- **Pattern "refino aditivo paralelo entre variants
  irmãos" N=4 → 5 cumulativo**.
- **Pattern "Smart→Option" N=10 → 12 cumulativo**.
- **Pattern "Field armazenado semantic adiada" N=7 → 8
  cumulativo**.
- **Pattern "fecho categoria completa dentro de ADR
  PROPOSTO sem transição" N=1 → 2 cumulativo** (P232
  Categoria A; **P235 Categoria B**).
- **Pattern "Layouter cell_align save/restore granularidade
  per-cell" N=1 inaugurado P235**.
- **Pattern "render real algorítmico per-cell" N=1
  inaugurado P235**.
- **Pattern "renderização diferenciada por atributo dentro
  do mesmo sub-passo" N=1 inaugurado P235**.
- **Categoria B Fase 5 Layout: 3/3 ✓ FECHADA
  estructuralmente** — próximo sub-passo pivot.
- **Fase 5 Layout candidata: 8/13-15 sub-passos
  materializados** (~53-62% cumulativo; **Categoria A
  100% interna; Categoria B 100% interna; C + D pendentes**).
