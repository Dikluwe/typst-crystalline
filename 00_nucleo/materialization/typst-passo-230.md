# Passo 230 — A.3 `stroke`/`fill` per-cell em GridCell + TableCell (Fase 5 Layout candidata Categoria A 3/5; primeira aplicação pós-ADR-0080 EM VIGOR)

**Série**: 230 (décimo-sexto sub-passo Layout pós-M9c;
**terceiro sub-passo materialização Fase 5 Layout candidata**
per ADR-0079 PROPOSTO; terceiro sub-passo Categoria A
"cosméticos sem reabrir decisões"; **primeira aplicação
real ADR-0080 EM VIGOR** pós-promoção P229).
**Marco**: nenhum (décimo-oitavo passo pós-M9c; **valida
pattern "refino aditivo paralelo entre variants irmãos"
N=2 → 3 cumulativo** P227+P228+P230 estendido a cells
estructurados; **primeira aplicação automática ADR-0080
EM VIGOR sem decisão explícita Opção γ por sub-passo** —
regra metodológica em prática).
**Tipo**: refino aditivo a 2 variants existentes
(`Content::GridCell` + `Content::TableCell`); 2 fields
novos a cada variant + helper reuso `extract_stroke` P227 +
renderização com precedência per-cell vs Grid-level.
**Magnitude**: M (~2-2.5h).
**Pré-condição**: P229 concluído (ADR-0080 EM VIGOR pós
N=9 cumulativo validado; 2071 verdes; 0 violations; saldo
DEBTs 12; ADR-0079 Categoria A 2/5); humano fixou A.3
(decisão literal pós-P229 §6); `Content::GridCell { body,
x, y, colspan, rowspan }` baseline P224.C; `Content::TableCell
{ body, x, y, colspan, rowspan }` baseline P157B; `Stroke`
+ `Color` baseline P25+P76; `Value::Stroke` baseline P227;
helper `extract_stroke` baseline P227; `native_grid_cell`
+ `native_table_cell` em `stdlib/structural.rs` baseline;
`layout_grid` consume `stroke`/`fill` Grid-level baseline
P227+P228 com Z-order correcto.
**Output**: 1 ficheiro relatório curto + código alterado em
~5-7 ficheiros L1 + **L0 NÃO tocado automaticamente** per
ADR-0080 EM VIGOR (sem necessidade decisão explícita por
sub-passo) + inventário 148 anotação cumulativa (footnote
⁴⁹) + ADR-0079 anotação Categoria A 3/5.

---

## §1 Trabalho

`Content::GridCell` baseline P224.C tem 5 fields (subset
paridade P157B TableCell literal). Diagnóstico P226
Categoria A.3 marcou "stroke/fill GridCell per-cell" como
sub-passo Fase 5 cosmético cumulativo a A.1+A.2.
`Content::TableCell` baseline P157B paridade P224.C.

**P230 materializa A.3**:
- **GridCell +2 fields**: `stroke: Option<Stroke>` + `fill:
  Option<Color>` (per-cell override Grid-level).
- **TableCell +2 fields**: `stroke: Option<Stroke>` + `fill:
  Option<Color>` (paralelo GridCell; pattern "refino aditivo
  paralelo entre variants irmãos" N=2 → 3 estendido).
- **`native_grid_cell` + `native_table_cell` accept `stroke:`
  + `fill:` named args** (reuso helper `extract_stroke`
  P227 N=1 → 2; parsing inline Color trivial P228).
- **Renderização precedência** em `layout_grid`: resolver
  `effective_stroke = cell.stroke.or(grid.stroke)` +
  `effective_fill = cell.fill.or(grid.fill)` per-cell;
  emit no Z-order P227+P228 (fill atrás; conteúdo; stroke
  à frente).

**Decisão arquitectural central — 8 decisões fixadas**:

### Decisão 1 — Fields adicionados Opção α (stroke + fill apenas)

P226 diagnóstico separou Categoria A.3 (stroke/fill
per-cell; cosméticos) de Categoria B.3 (align/inset/breakable
per-cell; algorítmicos). P230 segue diagnóstico literal:

| Opção | Fields | Trade-off |
|-------|--------|-----------|
| **α** | `stroke` + `fill` apenas (paridade A.1+A.2 per-cell) | Subset minimal; coerente diagnóstico P226 |
| β | + align/inset/breakable (B.3 incluído) | Viola escopo Categoria A.3; B.3 separado |
| γ | + outset/radius/clip | A.4 candidato (não GridCell; Block/Boxed) |

**Decisão fixada — Opção α**: stroke + fill apenas
(cosméticos paralelos A.1+A.2). B.3 (align/inset/breakable)
fica para Categoria B distinta.

### Decisão 2 — Precedência Opção α (override completo paridade vanilla)

Vanilla literal: per-cell `Some(...)` override Grid-level;
per-cell `None` inherit Grid-level.

3 opções consideradas:

| Opção | Lógica | Trade-off |
|-------|--------|-----------|
| **α** | `cell.stroke.or(grid.stroke)` (override) | Paridade vanilla literal; lógica trivial |
| β | Merge per-side | Inflacionário; não há per-side |
| γ | Per-cell ADICIONAL (somar) | Semanticamente errado |

**Decisão fixada — Opção α** (paridade ADR-0033
observable literal).

Lógica em `layout_grid`:
```rust
let effective_stroke = cell.stroke.as_ref().or(grid_stroke);
let effective_fill = cell.fill.as_ref().or(grid_fill);
```

### Decisão 3 — Renderização Z-order com precedência

Z-order P227+P228 preservado integralmente:
1. **Fill efectivo** (atrás do conteúdo) — `cell.fill.or(grid.fill)`.
2. **Conteúdo cell**.
3. **Stroke efectivo** (à frente do conteúdo) —
   `cell.stroke.or(grid.stroke)`.

3 opções considerados:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | Cada cell emite uma vez (fill + stroke resolvidos per-cell) | Z-order limpo; semantic correcto |
| β | Grid emite default + per-cell sobrepõe | Duplicação visual; wrong |
| γ | Per-cell EM VEZ DE Grid (Opção α reformulado) | Equivalente α |

**Decisão fixada — Opção α**: refactor mínimo `layout_grid`
para resolver effective_* dentro do loop de cells.

### Decisão 4 — Stdlib parsing GridCell/TableCell

`native_grid_cell` baseline P224.C accept 5 named args.
P230 adiciona 2 named args paridade P227+P228:

- `stroke:` aceita Length/Color/Stroke shorthands (reuso
  `extract_stroke` helper P227 N=1 → 2).
- `fill:` aceita Color directo (parsing inline trivial
  paridade P228; rejeita Length).

`native_table_cell` paralelo P157B baseline; P230 paridade
literal.

Stdlib funcs count: 60 preservado (refinos a funcs
existentes; sem novas).

### Decisão 5 — Tests E2E precedência

Crítico testar precedência:
- Grid-level apenas → todas cells inherit (Grid-level
  emit; cells sem override).
- Per-cell apenas → cells com override emit; cells sem
  override skip.
- Ambos → per-cell override Grid-level (visual: per-cell
  vence).
- Grid stroke + per-cell fill → cell tem ambos (sem
  conflito; ortogonais).

**Decisão fixada — 4-5 tests E2E precedência explícitos**.

### Decisão 6 — L0 NÃO tocado automaticamente (ADR-0080 EM
VIGOR aplicação automática)

**Decisão fixada — aplicação automática ADR-0080 EM VIGOR**:

ADR-0080 status EM VIGOR pós-P229. §"Decisão" determina
"refactors aditivos pós-M9c NÃO actualizam L0 prompts por
defeito". P230 é refactor aditivo a variants Content
existentes (GridCell + TableCell). **Aplicação automática
sem necessidade fixar Opção γ explícita em spec individual**.

Pattern emergente "aplicação automática ADR EM VIGOR sem
decisão explícita por sub-passo" N=1 inaugurado P230 —
**primeiro sub-passo pós-ADR-0080 EM VIGOR a herdar Opção
γ por defeito sem decisão explícita**.

L0 prompts `entities/content.md` + `rules/stdlib.md`
preservados (não tocados; hashes preservados).

### Decisão 7 — Refino paralelo TableCell Opção α

3 opções consideradas:

| Opção | Acção | Trade-off |
|-------|-------|-----------|
| **α** | Refino paralelo TableCell (+2 fields paridade GridCell) | Pattern "refino aditivo paralelo entre variants irmãos" N=2 → 3 cumulativo |
| β | Apenas GridCell (escopo Categoria A.3 literal) | Quebra pattern P227+P228 paralelo Grid+Table |
| γ | Adiar TableCell para sub-passo separado A.3-bis | Inflacionário; ortogonal a momentum |

**Decisão fixada — Opção α**:
- Consistência cumulativa: P227 stroke Grid+Table paralelo;
  P228 fill Grid+Table paralelo; **P230 stroke+fill
  GridCell+TableCell paralelo** (pattern N=2 → 3
  cumulativo).
- Magnitude adicional mínima (TableCell baseline P157B
  paridade GridCell; refino paralelo trivial).
- Pattern "refino aditivo paralelo entre variants irmãos"
  consolida-se a 3 aplicações cumulativas.

### Decisão 8 — `extract_stroke` reuso N=1 → 2

P227 criou `extract_stroke` helper privado `pub(super)`
em `stdlib/layout.rs`. P230 reusa helper para parsing
`stroke:` em `native_grid_cell` (que está em `stdlib/structural.rs`).

**Decisão fixada**: helper já é `pub(super)` per P227;
audit C1 confirma. Se não for, promover para `pub(super)`
mínimo OU `pub(crate)` (paridade `measure_content`
promovido P222).

Reuso `extract_stroke` N=1 → **2 cumulativo** (P227 cria;
P230 reusa primeiro). Patamar N=2 trivial — sem promoção
helper público ainda (paridade `extract_length` que só
atingiu candidato promoção pública pós-N=10).

Reuso de dados (sem recolha nova):

- `Content::GridCell { body, x, y, colspan, rowspan }`
  baseline P224.C.
- `Content::TableCell { body, x, y, colspan, rowspan }`
  baseline P157B.
- `Stroke` baseline P76 em `entities/geometry.rs`.
- `Value::Stroke(Stroke)` baseline P227.
- `Color` baseline P25.
- Helper `extract_stroke` baseline P227 (N=1 → 2).
- Pattern parsing inline Color baseline P228.
- `native_grid_cell` em `stdlib/structural.rs` baseline
  P224.C.
- `native_table_cell` em `stdlib/structural.rs` baseline
  P157B.
- `layout_grid` consume Grid-level stroke/fill baseline
  P227+P228 com Z-order correcto.
- Pattern "refino aditivo paralelo entre variants irmãos"
  N=2 baseline P227+P228.
- ADR-0080 EM VIGOR baseline P229 (aplicação automática
  Opção γ).
- ADR-0079 PROPOSTO Categoria A 2/5 baseline P228.

---

## §2 Cláusulas (12 — atomização paridade P228)

### C1 — Inventário pré-P230: confirmar GridCell + TableCell + helpers + Z-order resolvido

Auditoria empírica:

```
grep -n "GridCell {" 01_core/src/entities/content.rs
grep -n "TableCell {" 01_core/src/entities/content.rs
grep -n "pub(super) fn extract_stroke" 01_core/src/rules/stdlib/layout.rs
grep -n "fn native_grid_cell\|fn native_table_cell" 01_core/src/rules/stdlib/structural.rs
grep -B 2 -A 20 "if let Some(c) = fill" 01_core/src/rules/layout/grid.rs
```

Hipótese:
- `GridCell { body, x, y, colspan, rowspan }` 5 fields
  baseline P224.C.
- `TableCell { body, x, y, colspan, rowspan }` 5 fields
  baseline P157B.
- `extract_stroke` `pub(super)` em `stdlib/layout.rs`.
  Visibilidade cross-módulo: se `native_grid_cell` em
  `structural.rs` chamar `extract_stroke` em `layout.rs`,
  precisa visibilidade ≥ `pub(super)` no scope partilhado
  ou re-export.
- `native_grid_cell` + `native_table_cell` em
  `stdlib/structural.rs` aceita 5 named args (paridade
  baseline).
- `layout_grid` consume Grid-level stroke/fill com Z-order
  fill→content→stroke (P227+P228 baseline).

**Decisão crítica C1**: verificar visibilidade
`extract_stroke` cross-módulo. Se `pub(super)` for
insuficiente, promover para `pub(crate)` (paridade
`measure_content` P222 promoção visibility).

Se signature ou estado divergir: registar `P230.div-N`.

### C2 — Adicionar `stroke` + `fill` a `Content::GridCell`

Editar `01_core/src/entities/content.rs` variant GridCell:

```rust
GridCell {
    body: Box<Content>,
    x: Option<usize>,
    y: Option<usize>,
    colspan: Option<usize>,
    rowspan: Option<usize>,
    /// P230 — stroke per-cell (override Grid-level se
    /// Some; inherit se None). Per ADR-0079 PROPOSTO
    /// Categoria A.3 + ADR-0080 EM VIGOR aplicação
    /// automática.
    stroke: Option<Stroke>,
    /// P230 — fill per-cell (override Grid-level se Some;
    /// inherit se None).
    fill: Option<Color>,
},
```

GridCell fields: **5 → 7** (+stroke + fill).

### C3 — Adicionar `stroke` + `fill` a `Content::TableCell`

Editar variant TableCell paridade:

```rust
TableCell {
    body, x, y, colspan, rowspan,            // P157B baseline
    /// P230 — stroke per-cell paralelo GridCell.
    stroke: Option<Stroke>,
    /// P230 — fill per-cell paralelo GridCell.
    fill: Option<Color>,
},
```

TableCell fields: **5 → 7** (+stroke + fill).

### C4 — Arms cascata exhaustivos (compiler-driven)

Total arms refino GridCell + TableCell P230:

**`entities/content.rs`** (5 arms × 2 variants = 10 arms
total):
- `is_empty` — proxy body (preservado; stroke/fill não
  afectam).
- `plain_text` — recurse body (preservado).
- `PartialEq::eq` — comparação +2 fields cada (GridCell
  7 fields; TableCell 7 fields).
- `map_content` — preserva `stroke` + `fill` Copy
  (Option<Stroke> via Clone; Option<Color> via Copy).
- `map_text` — idem.

**`rules/introspect.rs`** (2 arms × 2 = 4 arms):
- `materialize_time` — preserva stroke + fill.
- `walk` — preserva.

**`rules/layout/grid.rs::layout_grid`** (refino do loop
de cells com precedência effective_stroke + effective_fill).

**`rules/introspect/locatable.rs`** (catch-all preserva).

Total: **~16 arms cumulativos** (~paridade P228 12 +
metade refino layout). Compiler-driven; iterar até zero
errors.

### C5 — Refino `native_grid_cell` + `native_table_cell`
accept stroke + fill

Editar `stdlib/structural.rs::native_grid_cell`:

```rust
// Accept named args expandido: ["body", "x", "y",
// "colspan", "rowspan", "stroke", "fill"].
let stroke = match args.named.get("stroke") {
    Some(val) => Some(extract_stroke(val, "grid_cell", "stroke")?),
    None => None,
};
let fill = match args.named.get("fill") {
    Some(Value::Color(c)) => Some(*c),
    Some(other) => return Err(/* "fill: espera Color, recebeu ..." */),
    None => None,
};
// ... existing ...
Ok(Value::Content(Content::GridCell {
    body, x, y, colspan, rowspan,
    stroke, fill,  // P230 +2
}))
```

Editar `native_table_cell` paridade literal.

**Audit cross-módulo**: se `extract_stroke` em
`stdlib/layout.rs` for `pub(super)` apenas, promover
visibilidade ou re-exportar para `stdlib/structural.rs`
acesso. Audit C1 determinará.

Magnitude C5: **S (~30min)**.

### C6 — Renderização precedência em `layout_grid`

Editar `01_core/src/rules/layout/grid.rs::layout_grid`
dentro do loop de cells:

```rust
for placed in &placed_cells {
    let (x0, y0, x1, y1) = placed.bounds();

    // P230 — resolver effective stroke/fill per-cell
    // (override Grid-level se Some; inherit se None).
    let cell_stroke = placed.cell_stroke.as_ref();  // GridCell.stroke
    let cell_fill = placed.cell_fill.as_ref();      // GridCell.fill
    let effective_stroke = cell_stroke.or(grid_stroke);
    let effective_fill = cell_fill.or(grid_fill);

    // Z-order step 1: fill efectivo atrás do conteúdo.
    if let Some(c) = effective_fill {
        // ... emit Rect com fill ...
    }

    // Z-order step 2: conteúdo cell.
    // ... existing ...

    // Z-order step 3: stroke efectivo à frente.
    if let Some(s) = effective_stroke {
        // ... emit 4 lines per cell border ...
    }
}
```

**Crítical**: `placed_cells` precisa carregar `cell_stroke`
e `cell_fill` per cell (extraídos do `GridCell` variant
no momento do placement P224.C). Refactor `PlacedCell`
struct +2 fields:

```rust
pub struct PlacedCell {
    pub body: Content,
    pub row: usize,
    pub col: usize,
    pub colspan: usize,
    pub rowspan: usize,
    /// P230 — preserved from GridCell variant.
    pub cell_stroke: Option<Stroke>,
    /// P230 — preserved from GridCell variant.
    pub cell_fill: Option<Color>,
}
```

Magnitude C6: **S+ (~1h)** — maior parcela é refactor
`PlacedCell` + propagação no `place_cells` P224.C.

### C7 — Sentinelas P230

Tests P230 (paridade P228 estrutura + precedência foco):

**Unit content** (~4 tests):
- `p230_gridcell_variant_aceita_stroke_fill`.
- `p230_tablecell_variant_aceita_stroke_fill`.
- `p230_gridcell_partial_eq_inclui_stroke_fill` —
  comparação 7 fields.
- `p230_gridcell_map_content_preserva_stroke_fill`.

**Unit stdlib** (~6 tests):
- `p230_native_grid_cell_stroke_aceita_length_shorthand`.
- `p230_native_grid_cell_stroke_aceita_color_shorthand`.
- `p230_native_grid_cell_fill_color_aceita`.
- `p230_native_grid_cell_fill_length_rejeita`.
- `p230_native_table_cell_paridade_gridcell`.
- `p230_native_grid_cell_stroke_e_fill_simultaneos`.

**Layout E2E precedence** (~5 tests **crítico**):
- `p230_per_cell_stroke_override_grid_level` — Grid
  stroke + cell stroke → cell stroke prevalece.
- `p230_per_cell_fill_override_grid_level` — idem fill.
- `p230_per_cell_none_inherits_grid_level` — cell sem
  stroke usa Grid-level.
- `p230_per_cell_some_grid_none_emite_apenas_cell` —
  só cell tem stroke; Grid sem; cell emite.
- `p230_per_cell_stroke_e_grid_fill_simultaneos_z_order`
  — Z-order preservado mesmo com mix per-cell vs Grid-level.

Total tests P230: **~15 tests** (4+6+5). Esperado pós-P230:
**2071 + 15 = 2086 verdes**.

### C8 — L0 NÃO tocado (ADR-0080 EM VIGOR aplicação
automática)

**Decisão fixada — aplicação automática per ADR-0080 EM
VIGOR**: L0 prompts `entities/content.md` + `rules/stdlib.md`
NÃO actualizados.

Justificação:
- P230 é refactor aditivo a variants Content existentes
  (GridCell + TableCell).
- ADR-0080 §"Decisão" EM VIGOR (P229) determina "L0 não
  tocado por defeito".
- **Primeira aplicação automática** sem decisão explícita
  por sub-passo. Pattern emergente "aplicação automática
  ADR EM VIGOR" N=1 inaugurado P230.

Hash L0 prompts preservados (não tocados).

### C9 — Verificação tests workspace + lint

```
cargo test --workspace 2>&1 | tail -3
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério:
- 2071 verdes pré-P230 + ~15 novos = **~2086 verdes**.
- 0 violations preservadas.
- Hashes propagados em ~5-7 ficheiros L1 (`content.rs`,
  `grid.rs`, `grid_placement.rs` se PlacedCell expandido,
  `stdlib/structural.rs`, `stdlib/layout.rs` se
  visibilidade `extract_stroke` ajustada, `mod.rs`).
- L0 prompts não tocados — "Nothing to fix".

**Risco regressão**: P224.C + P157B tests pre-existentes
com construtor directo `GridCell {...}` ou `TableCell {...}`
podem precisar adaptação (+2 fields stroke/fill: None
defaults). Hipótese N=3-7 adaptações intencionais (paridade
P228 N=6 mas escala maior por cells terem mais tests
baseline).

### C10 — Inventário 148 reclassificação P230 (footnote ⁴⁹)

**§A.5 Layout linha 141 `grid(columns, ...)`**:
- Footnote `⁵ ⁴⁵ ⁴⁶ ⁴⁷ ⁴⁸` → `⁵ ⁴⁵ ⁴⁶ ⁴⁷ ⁴⁸ ⁴⁹`.
- Sem reclassificação categórica.

**Tabela B.2 Content variants**: actualização cumulativa
P230 — GridCell +2 fields; TableCell +2 fields. Variants
count: 59 preservado (refinos a variants existentes).

**Footnote ⁴⁹ P230 adicionada** (~80 linhas) documentando:
- A.3 materializado (terceiro Categoria A Fase 5).
- 8 decisões fixadas (Opção α fields + Opção α precedência
  + Opção α Z-order + parsing reuso extract_stroke + tests
  precedência + aplicação automática ADR-0080 EM VIGOR +
  refino paralelo TableCell + reuso extract_stroke N=2).
- Pattern "L0 minimal para refactors" **aplicação
  automática pós-EM VIGOR** N=1 inaugurado.
- Pattern "refino aditivo paralelo entre variants irmãos"
  N=2 → **3 cumulativo** (Grid+Table P227+P228;
  **GridCell+TableCell P230**).
- Reuso helper `extract_stroke` N=1 → 2.
- Pattern emergente "precedência per-cell vs container-level
  via `.or()` resolution" N=1 inaugurado P230.

### C11 — ADR-0079 anotação Categoria A 3/5

Editar ADR-0079:

```markdown
### P230 anotação — Categoria A sub-passo 3 (stroke/fill
per-cell GridCell + TableCell; precedência override)

**Categoria A**: 3/5 sub-passos materializados ✓.
- A.1 stroke (P227) ✓.
- A.2 fill (P228) ✓.
- **A.3 per-cell GridCell+TableCell (P230) ✓**.
- A.4 Block/Boxed outset/radius/clip — pendente.
- A.5 Place per-cell alignment override — pendente.

Trabalho P230:
- GridCell +2 fields stroke/fill (5 → 7 fields).
- TableCell +2 fields paralelo (5 → 7 fields).
- Precedência override: cell `Some` → override; cell
  `None` → inherit Grid-level.
- Reuso helper extract_stroke N=1 → 2.
- ~15 tests novos (4 unit content + 6 unit stdlib + 5 E2E
  precedência).
- **Primeira aplicação automática ADR-0080 EM VIGOR** —
  L0 não tocado sem decisão explícita por sub-passo
  (ADR-0080 §"Decisão" aplicada por defeito).

Patterns consolidados:
- "Refino aditivo paralelo entre variants irmãos" N=2 →
  **3 cumulativo** (Grid+Table P227/P228;
  GridCell+TableCell P230).
- "Aplicação automática ADR EM VIGOR sem decisão explícita
  por sub-passo" N=1 inaugurado P230.
- "Precedência per-cell vs container-level via `.or()`
  resolution" N=1 inaugurado P230.

Status ADR-0079 mantido PROPOSTO (3/13-15 sub-passos
materializados).
```

### C12 — Critério aceitação P230

- ~15 tests novos verdes.
- 2071 tests pre-existentes preservados (após N=3-7
  adaptações intencionais).
- 0 violations.
- GridCell +2 fields (5 → 7); TableCell +2 fields (5 → 7).
- PlacedCell expandido +2 fields (cell_stroke + cell_fill).
- Precedência per-cell vs Grid-level funcional.
- Helper `extract_stroke` reuso N=2.
- ADR-0079 Categoria A 3/5 anotado.
- Cobertura Layout 89% preservada (refino qualitativo).
- **Primeira aplicação automática ADR-0080 EM VIGOR**
  (L0 não tocado sem decisão explícita).

---

## §3 Output

1 ficheiro relatório curto:
`00_nucleo/materialization/typst-passo-230-relatorio.md`.

Estrutura (~6-8 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Inventário pré-P230 + audit `extract_stroke`
  visibilidade (C1).
- §3 GridCell/TableCell refino +2 fields cada (C2+C3).
- §4 `native_grid_cell`/`native_table_cell` accept (C5).
- §5 Renderização precedência + PlacedCell expandido
  (C6).
- §6 Decisões substantivas (8 decisões fixadas) +
  primeira aplicação automática ADR-0080 EM VIGOR.
- §7 Resultados verificação + inventário 148 footnote ⁴⁹
  + ADR-0079 anotação Categoria A 3/5 (C7+C9+C10+C11).
- §8 Próximo sub-passo (P231 candidatos: A.4 Block/Boxed;
  A.5 Place per-cell; B.1 DEBT-34d; D.1 state; pivot).

Código alterado:
- **Editado**: `01_core/src/entities/content.rs` (GridCell
  + TableCell refino +2 fields cada + arms cascata +
  ~4 unit tests).
- **Editado**: `01_core/src/rules/introspect.rs` (arms
  preservados ou ajuste trivial).
- **Editado**: `01_core/src/rules/layout/grid.rs`
  (renderização precedência effective_stroke + effective_fill).
- **Editado**: `01_core/src/rules/layout/grid_placement.rs`
  (PlacedCell expandido +2 fields preservados de GridCell;
  refactor `place_cells`).
- **Editado**: `01_core/src/rules/stdlib/structural.rs`
  (`native_grid_cell` + `native_table_cell` accept
  stroke/fill; +~6 unit tests).
- **Possivelmente editado**: `01_core/src/rules/stdlib/layout.rs`
  (visibilidade `extract_stroke` ajuste se necessário).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+~5
  E2E precedence tests).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (footnote ⁴⁹ P230 + Tabela B.2 actualização cumulativa).
- **Editado**: `00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`
  (+ anotação Categoria A 3/5 P230).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- Per-cell `align`/`inset`/`breakable` — **Categoria B.3**
  separado (algorítmico per-cell; não cosmético).
- Per-cell `outset`/`radius`/`clip` — Block/Boxed Categoria
  A.4 (não GridCell).
- Closure per-cell `(row, col) => stroke(...)` — fora de
  escopo Fase 5.
- Promoção helper `extract_stroke` para público (N=2
  insuficiente; patamar N=10 paridade `extract_length`).
- Promover ADR-0079 PROPOSTO → IMPLEMENTADO — só pós
  Categoria A 5/5 + B + C + D completas (ou scope-out
  parcial formal).
- Promover ADR-0066 PROPOSTO → IMPLEMENTADO — só pós-D.1
  (state) materializa.
- Tocar em L0 prompts — ADR-0080 EM VIGOR aplicação
  automática.
- Show rules `#show grid.cell: ...` — fora de escopo
  Fase 5.
- Reabrir decisões arquiteturais — A.3 é Categoria A
  (sem reabrir).
- Refactor estrutural GridCell ↔ TableCell unificação —
  preservados separados literal (paridade vanilla).

---

## §5 Riscos a evitar

1. **`extract_stroke` visibilidade insuficiente cross-módulo**:
   audit C1 crítico. Se `pub(super)` for limitado,
   promover para `pub(crate)` (paridade `measure_content`
   P222).
2. **PlacedCell refactor invasivo**: +2 fields
   (cell_stroke + cell_fill) precisa propagação em
   `place_cells` P224.C + arms cascade no `layout_grid`
   consumer. Mitigação: refactor mínimo focal; tests
   isolados em `grid_placement::tests` preservam P224.C
   baseline.
3. **Precedência confusa entre Grid-level e per-cell**:
   semantic crítico (cell Some → override; cell None →
   inherit). Mitigação: 5 tests E2E explícitos C7 +
   helper `effective_*` claramente nomeado.
4. **Tests pre-existentes GridCell/TableCell**: hipótese
   N=3-7 testes com construtor directo precisam +2 fields
   defaults None. Adaptação intencional documentada.
5. **L0 tocado por engano**: violar ADR-0080 EM VIGOR
   exactamente quando deveria validar aplicação automática.
   Mitigação: §5 risco 5 explícito + §C8 fixa não tocar.
6. **Pattern N=2 helper extract_stroke não justifica
   promoção pública**: tentação por N=2. Rejeitada —
   patamar N=10 paridade `extract_length` precedente.
7. **`Content::TableCell` refino paralelo questionado**:
   tentação de "manter escopo literal A.3 GridCell apenas".
   Rejeitada — Decisão 7 Opção α (pattern paralelo N=2 →
   3 consolidação).
8. **Magnitude exceder M (~2-2.5h)**: P227 chegou em
   ~1.5h; P228 em ~1h; P230 mais complexo (precedência +
   PlacedCell refactor). Hipótese real S+ a M (~1.5-2h).
9. **Z-order precedência teste manualmente errado**:
   E2E tests verificam ordem por index inspection no
   frame items. Mitigação: tests específicos C7.
10. **Refactor `place_cells` quebrar P224.C tests
    existentes**: PlacedCell expandido +2 fields. Mitigação:
    Default values None preservam comportamento P224.C
    baseline; 7 unit tests `grid_placement::tests`
    preservam-se.
11. **Precedência merge inadvertida**: tentação de "fundir"
    Grid-level + per-cell quando ambos Some. Rejeitada —
    Decisão 2 Opção α override completo literal vanilla.
12. **Documentar precedência em L0**: tentação por "regra
    nova; documentar". Rejeitada — ADR-0080 EM VIGOR
    aplicação automática. Documentação fica em inline-doc
    + footnote inventário 148 + ADR-0079 anotação.

---

## §6 Hipótese provável

C1 confirmará GridCell + TableCell baseline 5 fields cada;
`extract_stroke` visibilidade auditada (provável `pub(super)`
em `layout.rs`; cross-módulo a `structural.rs` precisa
verificação).

C2+C3 adicionarão stroke + fill a GridCell e TableCell.

C4 cobrirá ~16 arms cumulativos (paridade P228 estructura
mas com cells).

C5 refinará `native_grid_cell` + `native_table_cell`
accept stroke/fill via `extract_stroke` reuso + parsing
inline Color.

C6 implementará renderização precedência (`effective_*`
via `.or()`) + PlacedCell expandido +2 fields. Magnitude
maior que P228 por refactor `place_cells` P224.C.

C7 criará ~15 tests novos (incl. 5 E2E precedência
explícitos).

C8 NÃO tocará L0 (aplicação automática ADR-0080 EM VIGOR).

C9 reportará ~2086 verdes; 0 violations; possíveis N=3-7
adaptações cells baseline.

C10 reclassificará footnote ⁴⁹; Tabela B.2 cumulativa.

C11 anotará ADR-0079 Categoria A 3/5.

C12 verifica critério aceitação.

Custo real: **M (~1.5-2h)** — paridade P228 mas com
refactor PlacedCell + precedência.

Mas é hipótese, não decisão. C1-C12 fixam-se empíricamente.

---

## §7 Particularidade P230

P230 é estruturalmente distinto na trajectória pós-M9c:

- **Terceiro sub-passo materialização Fase 5 Layout
  candidata** — Categoria A 3/5 sub-passos. Pattern
  emergente "Categoria A progresso cumulativo Fase 5
  pós-M9c" N=2 → 3.
- **Primeira aplicação automática ADR-0080 EM VIGOR
  pós-promoção P229** — ADR-0080 §"Decisão" aplicada por
  defeito sem decisão explícita Opção γ por sub-passo.
  Pattern emergente "aplicação automática ADR EM VIGOR
  sem decisão explícita por sub-passo" N=1 inaugurado P230.
- **Pattern "refino aditivo paralelo entre variants
  irmãos" N=2 → 3 cumulativo** (Grid+Table P227/P228;
  **GridCell+TableCell P230**). Padrão consolida-se para
  cells estructurados.
- **Pattern emergente "precedência per-cell vs
  container-level via `.or()` resolution" N=1 inaugurado
  P230** — semantic override paridade vanilla literal;
  reusável para refinos A.4 Block/Boxed (per-element vs
  ancestor) e B.3 (align/inset/breakable per-cell).
- **Helper `extract_stroke` reuso N=1 → 2** — primeiro
  reuso pós-criação P227. Patamar trivial; promoção
  pública continua diferida (paridade `extract_length`
  N=10 patamar).
- **Refactor `PlacedCell` +2 fields** — primeiro refactor
  estrutural a tipo L1 cristalino pós-Fase 4 (P224.C
  criou `PlacedCell`). Magnitude mínima mas estruturalmente
  significativa.
- **Cobertura Layout per metodologia preservada 89% real**
  — A.3 é refino qualitativo cosmético per-cell.
- **Anti-inflação 22ª aplicação cumulativa** pós-P205D —
  Opção α fields restritos + Opção α precedência override
  + Opção γ L0 automático + helper reuso + refino paralelo
  variants irmãos + ADR-0079 sem promoção.
- **Decisão 6 Opção γ aplicação automática inaugura
  precedente para sub-passos Fase 5 seguintes** —
  refactors aditivos pós-P229 herdam Opção γ por defeito;
  divergências exigem decisão explícita humana fixada em
  spec individual.

Por isso §5 risco 5 (L0 tocado por engano) é o mais
provável simbolicamente. Tentação: "precedência semantic
nova; documentar formal em L0 GridCell". Defesa: ADR-0080
EM VIGOR aplica-se automaticamente; precedência fica em
inline-doc + footnote ⁴⁹ inventário 148 + ADR-0079
anotação.

**Critério de aceitação P230**:
- ~15 tests novos verdes.
- 2071 tests pre-existentes preservados (após N=3-7
  adaptações intencionais).
- 0 violations.
- GridCell +2 fields (5 → 7).
- TableCell +2 fields (5 → 7; paralelo).
- PlacedCell expandido +2 fields.
- Precedência per-cell vs Grid-level funcional via `.or()`.
- Helper `extract_stroke` reuso N=2.
- ADR-0080 EM VIGOR aplicação automática (L0 não tocado).
- ADR-0079 Categoria A 3/5 anotado.
- Cobertura Layout 89% preservada.

**Estado pós-P230 esperado**:
- Tests workspace: 2071 → **~2086 verdes** (+15).
- Stdlib funcs: 60 preservado.
- Content variants: 59 preservado.
- Value variants: 55 preservado.
- GridCell fields: 5 → **7** (+stroke + fill).
- TableCell fields: 5 → **7** (+stroke + fill paralelo).
- PlacedCell fields: 5 → **7** (+cell_stroke + cell_fill
  preserved).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada (refino
  qualitativo; cobertura Layout 89% preservada).
- Cobertura user-facing total: 67% preservada.
- ADR-0066 PROPOSTO; ADR-0061 IMPLEMENTADO; ADR-0078
  IMPLEMENTADO; ADR-0079 PROPOSTO; **ADR-0080 EM VIGOR**.
- Saldo DEBTs: 12 preservado.
- **22 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" — primeira
  aplicação automática N=1** pós-EM VIGOR (precedente
  para todos refinos aditivos seguintes).
- **Pattern "refino aditivo paralelo entre variants
  irmãos" N=3 cumulativo** (consolidação).
- **Pattern "aplicação automática ADR EM VIGOR sem
  decisão explícita por sub-passo" N=1 inaugurado P230**.
- **Pattern "precedência per-cell vs container-level via
  `.or()` resolution" N=1 inaugurado P230** (reusável
  A.4+B.3+).
- **Helper `extract_stroke` reuso N=2** (patamar trivial).
- **Categoria A Fase 5 Layout**: 3/5 → próximos A.4
  Block/Boxed, A.5 Place per-cell. **Pendentes 2 sub-passos
  Categoria A para fechar 5/5**.
- **Fase 5 Layout candidata**: 3/13-15 sub-passos
  materializados (P227 A.1 ✓; P228 A.2 ✓; **P230 A.3 ✓**;
  restantes pendentes).
