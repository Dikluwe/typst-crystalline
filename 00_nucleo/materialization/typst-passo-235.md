# Passo 235 — B.3 `align`/`inset`/`breakable` per-cell em GridCell + TableCell (Fase 5 Layout candidata Categoria B 3/3; **fecha Categoria B**; valida pattern `.or()` N=2 → 3 atinge limiar formalização; sexta aplicação automática ADR-0080 EM VIGOR)

**Série**: 235 (vigésimo-primeiro sub-passo Layout pós-M9c;
**oitavo sub-passo materialização Fase 5 Layout candidata**
per ADR-0079 PROPOSTO; **terceiro e último sub-passo
Categoria B** "algorítmicos isolados"; **fecha Categoria
B 3/3 estructuralmente**; sexta aplicação automática
ADR-0080 EM VIGOR pós-P229).
**Marco**: nenhum status ADR (Categoria B fecha dentro de
ADR-0079 PROPOSTO sem transição; paridade pattern P232
fecho Categoria A); **valida pattern "precedência per-X
vs container-level via `.or()` resolution" N=2 → 3
cumulativo atingindo limiar formalização N=3-4** (promoção
formal candidato XS futuro paridade P229 ADR meta);
pattern "refino aditivo paralelo entre variants irmãos"
N=4 → 5 cumulativo (Grid+Table P227/P228; GridCell+
TableCell P230; Block+Boxed P231; **GridCell+TableCell
algorítmico P235**); pattern "Smart→Option" N=10 → 12
cumulativo (+inset Option +breakable Option); pattern
"aplicação automática ADR-0080 EM VIGOR" N=5 → 6.
**Tipo**: refino aditivo a 2 variants existentes
(`Content::GridCell` + `Content::TableCell`); 3 fields
novos a cada variant + renderização real align (via
Layouter cell_align save/restore per-cell paridade P232
estendido) + inset real (bounds reduction trivial) +
breakable armazenado semantic adiada graded (paridade
P156G Block.breakable + P224.B repeat).
**Magnitude**: M (~2.5-3h; paridade diagnóstico P226 B.3).
**Pré-condição**: P234 concluído (B.2 Consumer geometric
integração; 2122 verdes; 0 violations; saldo DEBTs 11;
ADR-0079 Categoria A 5/5 + Categoria B 2/3); humano fixou
B.3 (decisão literal pós-P234 §8); `Content::GridCell {
body, x, y, colspan, rowspan, stroke, fill }` baseline
P224.C + P230 (7 fields); `Content::TableCell { body, x,
y, colspan, rowspan, stroke, fill }` baseline P157B +
P230 (7 fields); `Sides<Length>` baseline P156C com
`Sides::ZERO`; `Align2D { h: Option<HAlign>, v:
Option<VAlign> }` baseline P84.5; Layouter `cell_align:
Option<Align2D>` baseline P232 (save/restore Grid-level);
pattern `.or()` resolution N=2 baseline P230 GridCell
stroke/fill + P232 Place per-axis; pattern "aplicação
automática ADR-0080 EM VIGOR" N=5 baseline P230+P231+
P232+P233+P234.
**Output**: 1 ficheiro relatório curto + código alterado em
~5-7 ficheiros L1 + **L0 NÃO tocado automaticamente** per
ADR-0080 EM VIGOR + inventário 148 anotação cumulativa
(footnote ⁵⁴) + ADR-0079 anotação **Categoria B 3/3 ✓
fechada estructuralmente** + saldo Fase 5: 8/13-15
sub-passos materializados.

---

## §1 Trabalho

P230 (A.3) cobriu apenas `stroke`/`fill` per-cell
(cosméticos). P226 diagnóstico Categoria B.3 marcou literal
**"Per-cell GridCell atributos `align`/`inset`/`breakable`
(M)"** — os 3 atributos algorítmicos distinguidos de
cosméticos P230.

P224 + P227 + P228 + P230 + P233 + P234 cumulativos
estabeleceram pattern `.or()` resolution per-cell vs
Grid-level (P230 stroke/fill; P232 Place per-axis). P235
estende a 3 atributos algorítmicos.

**P235 materializa B.3**:
- **GridCell +3 fields algorítmicos**: `align: Option<Align2D>`
  + `inset: Option<Sides<Length>>` + `breakable:
  Option<bool>`.
- **TableCell +3 fields paralelo** (pattern N=4 → 5
  cumulativo).
- **`native_grid_cell` + `native_table_cell` accept 3
  named args** (parsing align via helper baseline
  `extract_alignment`; inset via parsing Sides; breakable
  via Bool).
- **Renderização precedência `.or()`** per-cell:
  - `effective_align = cell.align.or(grid.align)` — reuso
    Layouter `cell_align` P232 baseline com save/restore
    per-cell.
  - `effective_inset = cell.inset.or(Some(grid_inset)).unwrap_or_default()`
    — reduz bounds antes layout body.
  - `effective_breakable` armazenado semantic adiada graded.

**Decisão arquitectural central — 8 decisões fixadas**:

### Decisão 1 — Escopo Opção α (align + inset + breakable apenas)

P226 diagnóstico Categoria B.3 literal. 3 opções:

| Opção | Atributos | Trade-off |
|-------|-----------|-----------|
| **α** | align + inset + breakable apenas | Subset minimal paridade P226 literal |
| β | + outros (colspan/rowspan validação) | Viola escopo P226 |
| γ | Apenas align (atomização interna) | Inflacionário |

**Decisão fixada — Opção α** (3 atributos).

### Decisão 2 — Tipos dos fields Opção β (todos Option uniformes)

3 opções consideradas:

| Opção | Tipos | Trade-off |
|-------|-------|-----------|
| α | `align: Option<Align2D>; inset: Sides<Length>; breakable: bool` | Inset/breakable sem Option; precedência inconsistente |
| **β** | Todos Option (`Option<Align2D>`; `Option<Sides<Length>>`; `Option<bool>`) | Coerente; precedência `.or()` uniforme nos 3 atributos |
| γ | Per-side individual Option Sides | Inflacionário; complexo |

**Decisão fixada — Opção β** (todos Option):
- Consistência com align Option baseline.
- Precedência `.or()` resolution uniforme.
- Pattern Smart→Option N=10 → **12 cumulativo** (+inset
  Option +breakable Option).

### Decisão 3 — Precedência Opção α (`.or()` uniforme 3 atributos)

Reuso pattern P230 + P232 `.or()` resolution.

```rust
let effective_align = cell.align.or(grid.align);
let effective_inset = cell.inset.or(Some(grid.inset)).unwrap_or_default();
let effective_breakable = cell.breakable.or(Some(grid_breakable_default)).unwrap_or(true);
```

**Decisão fixada — Opção α** (`.or()` uniforme).

**Pattern emergente "precedência per-X via `.or()`
resolution" N=2 → 3 cumulativo atingindo limiar formalização
N=3-4**:
- P230 GridCell stroke/fill over Grid.
- P232 Place per-axis over Grid.
- **P235 GridCell 3 algorítmicos over Grid** (align +
  inset + breakable).

**Promoção formal ADR meta candidato XS futuro paridade
P229**: pattern atingiu N=3; promoção formal pode acontecer
em passo administrativo XS dedicado.

### Decisão 4 — Refino paralelo TableCell Opção α

Pattern "refino aditivo paralelo entre variants irmãos"
N=4 cumulativo:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | TableCell +3 fields paralelo GridCell | Pattern N=4 → 5 cumulativo consolidado |
| β | Apenas GridCell | Quebra pattern paralelo |
| γ | TableCell sub-passo separado | Inflacionário |

**Decisão fixada — Opção α**:
- Pattern N=4 → **5 cumulativo** consolidado.
- TableCell estruturalmente paridade GridCell baseline
  P157B + P230.

### Decisão 5 — Renderização align Opção β (reuso Layouter cell_align P232 estendido)

3 opções:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | Cell.align aplica directamente ao body | Implementação trivial mas ignora pattern P232 |
| **β** | Reuso Layouter `cell_align` baseline P232 com save/restore per-cell (não apenas per-Grid) | Coerente cumulativo; estende P232 granularidade |
| γ | Helper extract_effective_alignment separado | Inflacionário |

**Decisão fixada — Opção β** (extensão P232):

P232 implementou Layouter `cell_align: Option<Align2D>`
com save/restore **per-Grid** (Grid-level scope; uniform
em todas cells). P235 estende:

```rust
// Pre-cell emit:
let saved_cell_align = self.cell_align;
let effective_align = cell.align.as_ref().copied().or(grid_align);
self.cell_align = effective_align;

// Emit cell body (Place dentro cell herda effective_align via P232).

// Post-cell emit:
self.cell_align = saved_cell_align;
```

**Pattern emergente "Layouter cell_align save/restore
granularidade per-cell (não apenas per-Grid)" N=1
inaugurado P235** — extensão P232 que só fez per-Grid
save/restore.

### Decisão 6 — Renderização inset Opção α (real)

Reuso pattern P234 `cell_origin_*` save/restore + bounds
reduction:

```rust
let effective_inset = cell.inset.or(Some(grid_inset)).unwrap_or_default();
// Bounds reduction:
let body_x = cell_x + effective_inset.left.to_pt();
let body_y = cell_y + effective_inset.top.to_pt();
let body_w = cell_w - effective_inset.left.to_pt() - effective_inset.right.to_pt();
let body_h = cell_h - effective_inset.top.to_pt() - effective_inset.bottom.to_pt();
// Set Layouter cell_origin_* ao body_x/y/w; layout body.
```

3 opções:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | Render real (bounds reduction) | Trivial pós-P234 cell_bounds; semantic correcto |
| β | Armazenado adiada graded | Viola intent algorítmico Categoria B |
| γ | Partial (top/bottom apenas) | Inflacionário; incompleto |

**Decisão fixada — Opção α** (render real). Pattern
"render real algorítmico per-cell" N=1 inaugurado P235.

### Decisão 7 — Renderização breakable Opção β (armazenado adiada graded)

3 opções:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | Render real (cell que excede region quebra ou não) | Requer refactor multi-region cell-level |
| **β** | Armazenado semantic adiada graded (paridade P156G Block.breakable + P224.B repeat) | Pattern "Field armazenado semantic adiada" N=7 → 8 cumulativo |
| γ | Partial (cells fail-graceful sem quebra) | Inflacionário; viola paridade vanilla |

**Decisão fixada — Opção β** (graded):
- Audit C1 confirma se `Block.breakable` P156G é real ou
  adiada baseline (provável adiada per scope-out P156G).
- Multi-region cell-level requer refactor estructural fora
  escopo P235.
- Pattern "Field armazenado semantic adiada" N=7 → **8
  cumulativo** (+breakable per-cell P235).

### Decisão 8 — L0 NÃO tocado (ADR-0080 EM VIGOR aplicação
automática N=6)

**Decisão fixada — aplicação automática sexta pós-P229**:

P235 é refactor aditivo a variants Content existentes
(GridCell + TableCell). ADR-0080 §"Decisão" aplica-se por
defeito. Pattern N=5 → **6 cumulativo** (P230+P231+P232+
P233+P234+**P235**). Pattern **extremamente sólido**
empíricamente; seis aplicações automáticas consecutivas
sem excepção.

L0 prompts NÃO tocados.

Reuso de dados (sem recolha nova):

- `Content::GridCell { body, x, y, colspan, rowspan,
  stroke, fill }` baseline P224.C + P230 (7 fields).
- `Content::TableCell { body, x, y, colspan, rowspan,
  stroke, fill }` baseline P157B + P230 (7 fields).
- `Sides<Length>` baseline P156C com `Sides::ZERO`.
- `Align2D { h: Option<HAlign>, v: Option<VAlign> }`
  baseline P84.5.
- Layouter `cell_align: Option<Align2D>` baseline P232
  (save/restore Grid-level estendido per-cell P235).
- `Layouter::cell_origin_*` baseline P84.6 (save/restore
  reuso P234 cell_bounds).
- `extract_alignment(args, default)` helper baseline P84.5.
- Pattern `.or()` resolution N=2 baseline P230 + P232.
- Pattern "refino aditivo paralelo entre variants irmãos"
  N=4 baseline P227+P228+P230+P231.
- Pattern "Smart→Option" N=10 baseline.
- Pattern "Field armazenado semantic adiada" N=7 baseline
  (weak P156D+E; breakable P156G; float P223; repeat
  P224.B; outset+radius+clip P231).
- Pattern "aplicação automática ADR-0080 EM VIGOR" N=5
  baseline P230+P231+P232+P233+P234.
- Helper `cell_bounds` privado baseline P234.
- ADR-0079 PROPOSTO Categoria A 5/5 + Categoria B 2/3
  baseline P234.

---

## §2 Cláusulas (12 — atomização paridade P230/P234)

### C1 — Auditoria pré-P235: confirmar GridCell/TableCell + Sides<Length> + extract_alignment + Block.breakable baseline

Audit empírico:

```
grep -A 10 "Content::GridCell {" 01_core/src/entities/content.rs
grep -A 10 "Content::TableCell {" 01_core/src/entities/content.rs
grep -n "extract_alignment\|extract_sides_lengths\|extract_sides" 01_core/src/rules/stdlib/
grep -B 2 -A 30 "Content::Block.*breakable\|breakable" 01_core/src/rules/layout/mod.rs
grep -n "cell_align" 01_core/src/rules/layout/mod.rs
grep -n "cell_origin_" 01_core/src/rules/layout/mod.rs
```

Hipótese:
- GridCell 7 fields baseline pós-P230 (body/x/y/colspan/
  rowspan/stroke/fill).
- TableCell 7 fields baseline pós-P230 (paralelo).
- `extract_alignment(args, default)` helper baseline P84.5
  em `stdlib/layout.rs`.
- `extract_sides_lengths` ou similar baseline (audit C1
  confirma).
- **Block.breakable real ou adiada?** — audit C1 crítico.
- Layouter `cell_align` baseline P232 save/restore
  Grid-level.

**Decisões críticas C1**:
1. **`Block.breakable` real ou adiada baseline**: se real,
   P235 breakable per-cell pode tentar real também; se
   adiada, P235 breakable adiada paridade.
2. **Helper `extract_sides_lengths` visibility**: audit
   C1; possível parsing inline se não disponível.
3. **Layouter `cell_align` extension granularidade
   per-cell**: pattern P232 só fez per-Grid; P235 estende
   per-cell save/restore.

Sem `P235.div-N` formal se hipótese converge.

### C2 — Adicionar `align` + `inset` + `breakable` a `Content::GridCell`

Editar `01_core/src/entities/content.rs` variant GridCell:

```rust
GridCell {
    body: Box<Content>,
    x: Option<usize>,
    y: Option<usize>,
    colspan: Option<usize>,
    rowspan: Option<usize>,
    stroke: Option<Stroke>,    // P230 (A.3)
    fill: Option<Color>,       // P230 (A.3)
    /// P235 — align per-cell (override Grid.align se Some;
    /// inherit se None). Reuso Layouter cell_align P232
    /// estendido per-cell save/restore.
    align: Option<Align2D>,
    /// P235 — inset per-cell (override Grid.inset se Some;
    /// inherit se None). Render real via bounds reduction.
    inset: Option<Sides<Length>>,
    /// P235 — breakable per-cell. Armazenado semantic
    /// adiada graded paridade P156G Block.breakable +
    /// P224.B repeat.
    breakable: Option<bool>,
},
```

GridCell fields: **7 → 10** (+align + inset + breakable).

### C3 — Adicionar `align` + `inset` + `breakable` a `Content::TableCell`

Editar TableCell paridade:

```rust
TableCell {
    body, x, y, colspan, rowspan,            // P157B baseline
    stroke, fill,                             // P230 (A.3)
    align: Option<Align2D>,                  // P235 paralelo
    inset: Option<Sides<Length>>,            // P235 paralelo
    breakable: Option<bool>,                  // P235 paralelo
},
```

TableCell fields: **7 → 10** (+align + inset + breakable).

### C4 — Arms cascata exhaustivos (compiler-driven)

Total arms refino GridCell + TableCell P235:

**`entities/content.rs`** (5 arms × 2 variants = 10 arms):
- `is_empty` — proxy body (preservado).
- `plain_text` — recurse body.
- `PartialEq::eq` — comparação +3 fields cada (10 fields).
- `map_content` — preserva 3 fields (Option Copy/Clone).
- `map_text` — idem.

**`rules/introspect.rs`** (2 arms × 2 = 4 arms).

**`rules/layout/grid.rs::layout_grid`** (match `placed.body`
extrai per-cell 5 fields agora: stroke + fill + align +
inset + breakable).

**`rules/layout/grid.rs` arm consumer** (Layouter
cell_align save/restore per-cell; bounds reduction
inset).

**`rules/introspect/locatable.rs`** (catch-all preserva).

Total: **~18-22 arms cumulativos**. Compiler-driven;
iterar até zero errors.

### C5 — Refino `native_grid_cell` + `native_table_cell` accept 3 named args

Editar `stdlib/structural.rs`:

```rust
// Em ambos native_grid_cell e native_table_cell:

// "align" name arg.
let align = match args.named.get("align") {
    Some(val) => Some(extract_alignment(val)?),
    None => None,
};
// "inset" name arg.
let inset = match args.named.get("inset") {
    Some(val) => Some(extract_sides_lengths_value(val)?),  // helper inline ou reuso
    None => None,
};
// "breakable" name arg.
let breakable = match args.named.get("breakable") {
    Some(Value::Bool(b)) => Some(*b),
    Some(other) => return Err(/* "breakable: espera Bool" */),
    None => None,
};
```

**Audit C1**: helper `extract_sides_lengths_value` para
parsing Value individual (não args completo P156G pad
pattern). Se ausente, parsing inline trivial paridade
P231 (Length uniforme → Sides::uniform OR Sides<Length>
explícito).

Magnitude C5: **S+ (~45min)** — 3 named args × 2 funcs.

### C6 — Renderização precedência effective_* via `.or()` (loop cells)

Editar `01_core/src/rules/layout/grid.rs::layout_grid`
loop de cells (pós-P234 baseline place_cells):

```rust
for placed in placed_cells.iter() {
    let (cell_x, cell_y, cell_w, cell_h) = cell_bounds(placed, ...);

    // Match placed.body extrai per-cell 5 fields (P230 + P235).
    let (cell_stroke, cell_fill, cell_align, cell_inset, cell_breakable) = match &placed.body {
        Content::GridCell { stroke, fill, align, inset, breakable, .. } |
        Content::TableCell { stroke, fill, align, inset, breakable, .. } => (
            stroke.as_ref(),
            fill.as_ref(),
            align.as_ref(),    // P235
            inset.as_ref(),    // P235
            breakable.as_ref() // P235
        ),
        _ => (None, None, None, None, None),
    };

    // Precedência .or() uniforme.
    let effective_stroke = cell_stroke.or(stroke);             // P230 baseline
    let effective_fill = cell_fill.or(fill);                   // P230 baseline
    let effective_align = cell_align.copied().or(grid_align);  // P235
    let effective_inset = cell_inset.copied().or(Some(grid_inset)).unwrap_or_default(); // P235
    let _effective_breakable = cell_breakable.copied().or(Some(true)).unwrap_or(true);  // P235 adiada

    // Z-order step 1: fill (P228 baseline).
    if let Some(c) = effective_fill { /* Rect emit */ }

    // P235 — Inset bounds reduction.
    let body_x = cell_x + effective_inset.left.to_pt();
    let body_y = cell_y + effective_inset.top.to_pt();
    let body_w = (cell_w - effective_inset.left.to_pt() - effective_inset.right.to_pt()).max(0.0);
    let body_h = (cell_h - effective_inset.top.to_pt() - effective_inset.bottom.to_pt()).max(0.0);

    // P235 — Cell-level cell_align save/restore (extensão P232 per-cell).
    let saved_cell_align = self.cell_align;
    self.cell_align = effective_align;

    // P234 — cell_origin_* save/restore com bounds reduction.
    let saved_origin_x = self.cell_origin_x;
    let saved_origin_y = self.cell_origin_y;
    let saved_origin_w = self.cell_origin_w;
    self.cell_origin_x = Some(body_x);
    self.cell_origin_y = Some(body_y);
    self.cell_origin_w = Some(body_w);

    // Z-order step 2: layout body com bounds reduzidos por inset.
    self.layout_sub_frame_with_width(&placed.body, body_w)?;

    // Restore.
    self.cell_origin_x = saved_origin_x;
    self.cell_origin_y = saved_origin_y;
    self.cell_origin_w = saved_origin_w;
    self.cell_align = saved_cell_align;

    // Z-order step 3: stroke (P227 baseline) com bounds cell_w/cell_h originais (não-reduzidos).
    if let Some(s) = effective_stroke { /* 4 lines emit */ }
}
```

**Decisão crítica**: layout body com bounds reduzidos
(inset aplicado); stroke/fill com bounds originais (inset
não afecta visual border/background).

Magnitude C6: **M (~1-1.5h)** — maior parcela renderização +
save/restore.

### C7 — Sentinelas P235

Tests P235 (~13-15 tests):

**Unit content** (~4 tests):
- `p235_gridcell_variant_aceita_align_inset_breakable`.
- `p235_tablecell_variant_aceita_align_inset_breakable`.
- `p235_gridcell_partial_eq_inclui_3_fields` — 10 fields.
- `p235_gridcell_map_content_preserva_3_fields`.

**Unit stdlib** (~6 tests):
- `p235_native_grid_cell_align_aceita`.
- `p235_native_grid_cell_inset_sides_aceita`.
- `p235_native_grid_cell_inset_length_uniforme_aceita`.
- `p235_native_grid_cell_breakable_bool_aceita`.
- `p235_native_grid_cell_breakable_tipo_errado_rejeita`.
- `p235_native_table_cell_paridade_gridcell`.

**Layout E2E precedence** (~5 tests crítico):
- `p235_per_cell_align_override_grid` — cell align center;
  grid top → cell renderiza center.
- `p235_per_cell_align_partial_override_per_axis` — cell
  align top; grid center → V override; H herda center.
- `p235_per_cell_inset_override_grid` — cell inset 10pt;
  grid inset 0 → body bounds reduzidos 10pt.
- `p235_per_cell_inset_none_inherits_grid` — cell inset
  None; grid inset 5pt → body bounds reduzidos 5pt.
- `p235_per_cell_breakable_armazenado_layout_preservado`
  — semantic adiada graded; field armazenado mas não
  afecta rendering (paridade P156G).

Total tests P235: **~13-15 tests** (4+6+5). Esperado
pós-P235: **2122 + 15 = ~2137 verdes** (paridade hipótese;
ajuste pós-implementação).

### C8 — L0 NÃO tocado (ADR-0080 EM VIGOR aplicação
automática N=6)

**Decisão fixada — aplicação automática**: sexta aplicação
automática pós-promoção P229. Pattern N=5 → 6 cumulativo
muito sólido empíricamente.

L0 prompts NÃO tocados.

### C9 — Verificação tests workspace + lint

```
cargo test --workspace 2>&1 | tail -3
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério:
- 2122 verdes pré-P235 + ~13-15 novos = **~2137 verdes**.
- 0 violations preservadas.
- Hashes propagados em ~5-7 ficheiros L1 (`content.rs`,
  `grid.rs`, `mod.rs`, `stdlib/structural.rs`, possíveis
  outros).
- L0 prompts não tocados — "Nothing to fix".

**Risco regressão**: tests baseline P224 + P227 + P228 +
P230 + P233 + P234 com construtores GridCell/TableCell.
Hipótese N=3-7 adaptações (cells agora têm 10 fields cada;
defaults None para 3 novos preservam comportamento).

### C10 — Inventário 148 footnote ⁵⁴

**§A.5 Layout entrada `grid(...)`**: footnote ⁵³ → ⁵³ ⁵⁴.

Footnote ⁵⁴ adicionada (~120 linhas) documentando:
- B.3 materializado (terceiro Categoria B Fase 5 fecha
  3/3).
- 8 decisões fixadas.
- Renderização 3 atributos diferenciada (align via
  Layouter cell_align P232 estendido; inset render real
  bounds reduction; breakable semantic adiada graded).
- 5 patterns emergentes consolidados/inaugurados:
  - "Precedência per-X via `.or()` resolution" N=2 → **3
    cumulativo atingindo limiar formalização N=3-4**
    (promoção formal candidato XS futuro).
  - "Refino aditivo paralelo entre variants irmãos" N=4
    → **5 cumulativo**.
  - "Smart→Option" N=10 → **12 cumulativo** (+inset
    Option +breakable Option).
  - "Field armazenado semantic adiada" N=7 → **8
    cumulativo** (+breakable per-cell).
  - "Aplicação automática ADR-0080 EM VIGOR" N=5 → **6
    cumulativo** (pattern extremamente sólido).
  - **"Layouter cell_align save/restore granularidade
    per-cell (não apenas per-Grid)" N=1 inaugurado P235**
    — extensão P232.
  - **"Render real algorítmico per-cell" N=1 inaugurado
    P235** — distinto cumulativo de cosméticos P230.
- **Categoria B Fase 5 FECHADA ESTRUCTURALMENTE 3/3**.

### C11 — ADR-0079 anotação Categoria B 3/3 ✓ fechada

Editar ADR-0079 com bloco P235:

```markdown
### P235 anotação — Categoria B sub-passo 3 (GridCell + TableCell
align/inset/breakable per-cell); **Categoria B 3/3 ✓ FECHADA
ESTRUCTURALMENTE**

**Categoria B**: 3/3 sub-passos materializados ✓ **FECHADA**.
- B.1 DEBT-34d Auto track sizing fix (P233) ✓.
- B.2 Consumer geometric place_cells → Layouter (P234) ✓.
- **B.3 GridCell + TableCell algorítmico (P235) ✓**.

Trabalho P235:
- GridCell +3 fields align/inset/breakable (7 → 10 fields).
- TableCell +3 fields paralelo (7 → 10 fields).
- Renderização diferenciada por atributo:
  - align: real via Layouter cell_align P232 estendido
    per-cell save/restore.
  - inset: render real bounds reduction.
  - breakable: armazenado semantic adiada graded paridade
    P156G + P224.B.
- ~13-15 tests novos.
- **Sexta aplicação automática ADR-0080 EM VIGOR**.

Patterns consolidados/inaugurados:
- **"Precedência per-X via `.or()` resolution" N=2 → 3
  cumulativo atingindo limiar formalização N=3-4** —
  **promoção formal ADR meta candidato XS futuro
  paridade P229**.
- "Refino aditivo paralelo entre variants irmãos" N=4 →
  5 cumulativo.
- Smart→Option N=10 → 12 cumulativo.
- "Field armazenado semantic adiada" N=7 → 8 cumulativo.
- "Aplicação automática ADR-0080 EM VIGOR" N=5 → 6
  cumulativo.
- **"Layouter cell_align save/restore granularidade
  per-cell" N=1 inaugurado P235**.
- **"Render real algorítmico per-cell" N=1 inaugurado
  P235**.

Status ADR-0079 mantido PROPOSTO (8/13-15 sub-passos
cumulativos; **Categoria A 5/5 ✓ + Categoria B 3/3 ✓ +
C/D pendentes**).

**Marco interno implícito Categoria B fechada
estructuralmente** — próximo sub-passo pode pivot
Categoria C (estruturais reabrindo) ou Categoria D
(runtime queries) ou outro módulo.
```

### C12 — Critério aceitação P235

- ~13-15 tests novos verdes.
- 2122 tests pre-existentes preservados (após N=3-7
  adaptações intencionais).
- 0 violations.
- GridCell +3 fields (7 → 10); TableCell +3 fields (7 →
  10; paralelo).
- Renderização diferenciada: align real via cell_align
  estendido; inset real bounds reduction; breakable adiada
  graded.
- **Categoria B 3/3 ✓ fechada estructuralmente**.
- ADR-0079 anotado Categoria B fechada sem transição
  status.
- ADR-0080 EM VIGOR aplicação automática N=5 → 6.
- Cobertura Layout 89% preservada (refino qualitativo).

---

## §3 Output

1 ficheiro relatório curto:
`00_nucleo/materialization/typst-passo-235-relatorio.md`.

Estrutura (~6-8 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Inventário pré-P235 + audit Block.breakable real/adiada
  + cell_align granularidade (C1).
- §3 GridCell/TableCell refino +3 fields cada (C2+C3).
- §4 `native_grid_cell`/`native_table_cell` accept 3 named
  args (C5).
- §5 Renderização precedência effective_* via `.or()` +
  diferenciação por atributo (C6).
- §6 Decisões substantivas (8 decisões fixadas) + sexta
  aplicação automática ADR-0080 EM VIGOR.
- §7 Resultados verificação + inventário 148 footnote ⁵⁴
  + ADR-0079 anotação **Categoria B 3/3 ✓ fechada
  estructuralmente** (C9+C10+C11).
- §8 Próximo sub-passo (P236 candidatos: D.1 state; C.1/
  C.2; pivot outro módulo; ADR meta admin batch).

Código alterado:
- **Editado**: `01_core/src/entities/content.rs` (GridCell
  + TableCell refino +3 fields cada + arms cascata + ~4
  unit tests).
- **Editado**: `01_core/src/rules/introspect.rs` (arms
  preservados).
- **Editado**: `01_core/src/rules/layout/grid.rs`
  (renderização precedência effective_* + bounds reduction
  inset + cell_align save/restore per-cell).
- **Editado**: `01_core/src/rules/stdlib/structural.rs`
  (`native_grid_cell` + `native_table_cell` accept 3 named
  args; +~6 unit tests).
- **Possivelmente editado**: `01_core/src/rules/stdlib/layout.rs`
  (helper `extract_sides_lengths_value` ou similar se
  necessário).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+~5
  E2E precedence tests).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (footnote ⁵⁴ P235).
- **Editado**: `00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`
  (+ anotação Categoria B 3/3 ✓ fechada estructuralmente).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- Outros atributos per-cell (e.g., `stroke-overhang` cells)
  — fora escopo P226 Categoria B.3.
- Render real `breakable` per-cell multi-region — refactor
  estructural fora escopo; Categoria C.2 candidato.
- Promoção formal pattern `.or()` resolution a ADR meta —
  P236 administrativo XS candidato (paridade P229 mas
  para pattern `.or()`); fora escopo P235.
- Promoção formal pattern "refino aditivo paralelo variants
  irmãos" — XS candidato futuro.
- Promover ADR-0079 PROPOSTO → IMPLEMENTADO — só pós
  Categorias A + B + C + D completas (B.3 ✓; C + D
  pendentes).
- Promover ADR-0066 PROPOSTO → IMPLEMENTADO — só pós-D.1
  state materializa.
- Tocar em L0 prompts — ADR-0080 EM VIGOR aplicação
  automática N=6.
- Reabrir decisões arquiteturais — B.3 é Categoria B
  algorítmico isolado.
- Show rules `#show grid.cell: ...` — fora escopo Fase 5.
- Marco cirúrgico blueprint pelo fecho Categoria B —
  anti-inflação (pattern §3.0... para fechos/aberturas de
  Fase, não categorias internas).
- Helper `extract_sides_lengths_value` público — usar
  inline trivial ou helper privado.

---

## §5 Riscos a evitar

1. **`Block.breakable` real baseline forçar P235 também
   real**: audit C1 crítico. Hipótese provável: Block.breakable
   adiada per P156G scope-out; P235 breakable adiada paridade.
2. **Helper `extract_sides_lengths_value` ausente**: parsing
   inline trivial paridade P231 (Length uniforme → Sides::
   uniform; Sides<Length> direct).
3. **Tests pre-existentes GridCell/TableCell**: hipótese
   N=3-7 testes precisam +3 fields None defaults.
   Adaptação intencional documentada.
4. **Layouter cell_align extensão per-cell quebra P232
   tests baseline**: P232 testou per-Grid save/restore.
   Mitigação: extensão estritamente aditiva; save/restore
   per-cell aninhado dentro de save/restore per-Grid
   preserva semantic.
5. **L0 tocado por engano**: sexta aplicação automática.
   Mitigação: §5 risco 5 explícito + §C8 fixa não tocar.
6. **Bounds reduction inset off-by-one**: cells com inset
   maior que cell_w/cell_h. Mitigação: `.max(0.0)` clamp
   no body_w/body_h.
7. **Magnitude exceder M (~2.5-3h)**: P234 chegou em
   ~1h30m. P235 mais complexo (3 fields × 2 variants +
   render diferenciado + extensão cell_align). Hipótese
   real M (~2h).
8. **Pattern `.or()` N=3 promoção prematura**: limiar
   formalização N=3-4 atingido; mas P235 não-promove
   (P236 candidato XS administrativo). Mitigação: §"Não-
   objectivos" explícito.
9. **Marco cirúrgico blueprint pelo fecho Categoria B**:
   tentação por simetria com fecho Categoria A (P232 sem
   marca). Rejeitada — anti-inflação preservada literal.
10. **Render real align quebra Place baseline P232**:
    P235 cell_align extension afecta scope per-cell;
    Place dentro cell continua usar `self.cell_align`
    (que agora vem de cell.align.or(grid)). Pattern
    preservado.
11. **Inset Option semantic confuso**: `Sides::ZERO`
    semantic "sem override" (cell inset None) vs "explicit
    zero inset" (cell inset Some(Sides::ZERO)). Mitigação:
    Decisão 2 Opção β `Option<Sides<Length>>` distingue
    explicitamente.
12. **Documentar pattern `.or()` em L0**: tentação por
    "limiar formalização atingido N=3". Rejeitada —
    ADR-0080 EM VIGOR aplicação automática; promoção ADR
    meta é passo administrativo XS separado.

---

## §6 Hipótese provável

C1 confirmará GridCell + TableCell baseline 7 fields cada;
`extract_alignment` baseline P84.5; `extract_sides_lengths_value`
provável ausente (parsing inline acceptable); **Block.breakable
adiada baseline** (P156G scope-out confirmado); Layouter
cell_align baseline P232 save/restore per-Grid.

C2+C3 adicionarão 3 fields paralelo a GridCell e TableCell.

C4 cobrirá ~18-22 arms cumulativos (compiler-driven).

C5 refinará `native_grid_cell` + `native_table_cell`
accept 3 named args (helper align + parsing inline inset
+ Bool breakable).

C6 implementará renderização diferenciada (align via
cell_align extension per-cell; inset real bounds reduction;
breakable adiada graded).

C7 criará ~13-15 tests novos.

C8 NÃO tocará L0 (aplicação automática ADR-0080 EM VIGOR
N=5 → 6).

C9 reportará ~2137 verdes; 0 violations; possíveis N=3-7
adaptações.

C10 reclassificará footnote ⁵⁴.

C11 anotará ADR-0079 **Categoria B 3/3 ✓ fechada
estructuralmente**.

C12 verifica critério aceitação.

Custo real: **M (~2-2.5h)** — 3 fields × 2 variants +
render diferenciado + extensão cell_align per-cell.

Mas é hipótese, não decisão. C1-C12 fixam-se empíricamente.

---

## §7 Particularidade P235

P235 é estruturalmente distinto na trajectória pós-M9c:

- **Terceiro e último sub-passo Categoria B Fase 5 Layout
  candidata** — **fecha Categoria B 3/3 estructuralmente**.
  Pós-P235, Categoria B não tem mais sub-passos
  identificados em ADR-0079.
- **Pattern "fecho categoria completa dentro de ADR
  PROPOSTO sem transição" N=1 → 2 cumulativo** (P232
  Categoria A; **P235 Categoria B**).
- **Pattern "precedência per-X via `.or()` resolution"
  N=2 → 3 cumulativo atingindo limiar formalização N=3-4**
  — **promoção formal ADR meta candidato XS futuro
  paridade P229**. Decisão pivot pós-P235 inclui ADR meta
  admin para pattern `.or()`.
- **Pattern "refino aditivo paralelo entre variants
  irmãos" N=4 → 5 cumulativo** — pattern extremamente
  sólido empíricamente; consolidação cumulativa.
- **Pattern "Smart→Option" N=10 → 12 cumulativo**
  (+inset Option +breakable Option) — pattern muito sólido;
  promoção formal candidato XS futuro paridade P229.
- **Pattern "Field armazenado semantic adiada" N=7 → 8
  cumulativo** (+breakable per-cell) — pattern muito
  sólido; promoção formal candidato.
- **Sexta aplicação automática ADR-0080 EM VIGOR
  pós-promoção P229** — pattern N=5 → 6 cumulativo.
  Pattern empíricamente **extremamente sólido**; seis
  aplicações automáticas consecutivas sem excepção.
- **Pattern emergente "Layouter cell_align save/restore
  granularidade per-cell" N=1 inaugurado P235** — extensão
  pattern P232 que só fez per-Grid save/restore.
- **Pattern emergente "render real algorítmico per-cell"
  N=1 inaugurado P235** — distinto cumulativo de
  cosméticos P230 (que foram graded por Z-order; não-real
  algorítmico).
- **Pattern emergente "renderização diferenciada por
  atributo dentro do mesmo sub-passo" N=1 inaugurado P235**
  — align (real); inset (real); breakable (adiada graded).
  Distinto de sub-passos uniformes.
- **Cobertura Layout per metodologia preservada 89% real**
  — B.3 é refino qualitativo algorítmico per-cell.
- **Anti-inflação 27ª aplicação cumulativa** pós-P205D —
  Opção α escopo restrito + Opção β tipos Option uniformes
  + Opção α `.or()` precedência + Opção α refino paralelo
  + Opção β graded breakable + Opção γ L0 automático +
  sem promoção formal patterns + sem marco blueprint +
  ADR-0079 sem promoção.

Por isso §5 risco 8 (pattern `.or()` N=3 promoção
prematura) é o mais provável simbolicamente. **Defesa**:
P235 não-promove; P236 candidato XS administrativo
dedicado paridade P229. Pattern `.or()` atinge limiar mas
promoção formal é decisão arquitectural separada.

**Critério de aceitação P235**:
- ~13-15 tests novos verdes.
- 2122 tests pre-existentes preservados (após N=3-7
  adaptações).
- 0 violations.
- GridCell +3 fields (7 → 10); TableCell +3 fields
  (paralelo).
- Render diferenciado por atributo funcional.
- **Categoria B 3/3 ✓ fechada estructuralmente**.
- ADR-0080 EM VIGOR aplicação automática N=5 → 6.
- Cobertura Layout 89% preservada.

**Estado pós-P235 esperado**:
- Tests workspace: 2122 → **~2137 verdes** (+13-15).
- Stdlib funcs: 60 preservado.
- Content variants: 59 preservado.
- Value variants: 55 preservado.
- **GridCell fields: 7 → 10** (+align + inset + breakable).
- **TableCell fields: 7 → 10** (+align + inset + breakable
  paralelo).
- Layouter fields: preservados (n+1 pós-P232).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- ADR-0066 PROPOSTO; ADR-0061 IMPLEMENTADO; ADR-0078
  IMPLEMENTADO; ADR-0079 PROPOSTO (8/13-15; Categoria A
  5/5 ✓ + Categoria B 3/3 ✓ + C/D pendentes); ADR-0080
  EM VIGOR.
- Saldo DEBTs: 11 preservado.
- **27 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" aplicação
  automática N=5 → 6 cumulativo** (P230+P231+P232+P233+
  P234+P235) — pattern extremamente sólido empíricamente.
- **Pattern "precedência per-X via `.or()` resolution"
  N=2 → 3 cumulativo atingindo limiar formalização N=3-4**
  — promoção formal ADR meta candidato XS futuro paridade
  P229.
- **Pattern "refino aditivo paralelo entre variants
  irmãos" N=4 → 5 cumulativo** consolidado.
- **Pattern "Smart→Option" N=10 → 12 cumulativo**.
- **Pattern "Field armazenado semantic adiada" N=7 → 8
  cumulativo**.
- **Pattern "fecho categoria completa dentro de ADR
  PROPOSTO sem transição" N=1 → 2 cumulativo** (P232
  Categoria A; P235 Categoria B).
- **Pattern "Layouter cell_align save/restore granularidade
  per-cell" N=1 inaugurado P235**.
- **Pattern "render real algorítmico per-cell" N=1
  inaugurado P235**.
- **Pattern "renderização diferenciada por atributo dentro
  do mesmo sub-passo" N=1 inaugurado P235**.
- **Categoria B Fase 5 Layout: 3/3 ✓ FECHADA
  estructuralmente** — próximo sub-passo pivot Categoria
  C/D ou outro módulo ou ADR meta admin.
- **Fase 5 Layout candidata: 8/13-15 sub-passos
  materializados** (~53-62% cumulativo; **Categoria A
  100% interna; Categoria B 100% interna; C + D
  pendentes**).
