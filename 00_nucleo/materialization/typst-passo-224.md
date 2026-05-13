# Passo 224 — `Content::Grid` refino substantivo completo (Opção δ; fecha DEBT-34d/e)

**Série**: 224 (décimo sub-passo Layout pós-M9c; **terceiro
e último sub-passo Fase 4 Layout candidata**; série α
"terminar Layout" Opção α P221 §8 — fecha série α
estructuralmente).
**Marco**: **fecho série α "terminar Layout"** —
encerramento estrutural Fase 4 Layout candidata; segundo
marco interno pós-M9c após P221 (Fase 3 Layout fechada);
pattern emergente "encerramento Fase pós-M9c" N=1 → 2
preparado (P225 documental será encerramento formal).
**Tipo**: refino substantivo composto a variant existente
`Content::Grid` (P82+P83+P84.6 baseline); 5 fields aditivos
(`gutter`/`align`/`inset`/`header`/`footer`) + 2 variants
Content novos (`GridHeader`/`GridFooter`/`GridCell`) +
placement algorítmico real (fecha DEBT-34d e DEBT-34e).
**Magnitude**: L (~5-8h cumulativo; atomização interna em
3 sub-fases A/B/C).
**Pré-condição**: P223 concluído (`Content::Place` refino
float + clearance graded; 2012 tests verdes; §A.5 `place`
impl⁺; cobertura Layout 83% real; DEBT-37 §"Divergência"
fechada); humano fixou **Opção δ — substantivo completo**
em resposta directa à proposta P223 §8 caminhos;
`Content::Grid { columns, rows, cells }` baseline P82-84.6;
`extract_tracks` helper `pub(super)` P157A; `TableCell` /
`TableHeader` / `TableFooter` precedente estrutural
P157B/C; DEBT-34d (placement algorítmico Grid completo) +
DEBT-34e (placement TableCell ignorado) ambos em aberto
desde 2026-04-26.
**Output**: 1 ficheiro relatório longo (~10-15 KB; magnitude
L justifica) + código alterado em ~6-8 ficheiros L1 + L0
`entities/content.md` decisão pattern N=6→? (Opção α
secção dedicada vs Opção γ continuação) + DEBT.md (DEBT-34d
+ DEBT-34e fechados) + ADR-0061 anotação Fase 4 candidata
3/3 (fecha série α) + inventário 148 reclassificação.

---

## §1 Trabalho

P82 + P83 + P84.6 materializaram `Content::Grid` baseline
com 3 fields (`columns: Vec<TrackSizing>`, `rows: Vec<TrackSizing>`,
`cells: Vec<Content>`). Algoritmo de layout 3-passagens
(Fixed → Auto → Fraction) operacional. **8 atributos
vanilla scope-out** per inventário 148 §A.5 linha 141:
`gutter`, `align`, `stroke`, `fill`, `inset`, `header`,
`footer`, `colspan`/`rowspan`.

**P224 materializa subset Opção δ — substantivo completo**:
- ✓ `gutter` — Length entre cells (atributo aditivo).
- ✓ `align` — Align2D uniforme (atributo aditivo).
- ✓ `inset` — Sides<Length> margem interna (atributo aditivo).
- ✓ `header` — variant Content novo `GridHeader { children,
  repeat }`.
- ✓ `footer` — variant Content novo `GridFooter { children,
  repeat }`.
- ✓ `colspan`/`rowspan` — variant Content novo `GridCell {
  body, x, y, colspan, rowspan }` + **placement algorítmico
  real** (fecha DEBT-34d + DEBT-34e estructuralmente).

**Scope-out preservado** (paridade pattern P156G+H+I e
P157A/B/C scope-outs):
- ✗ `stroke` — atributo cosmético (renderização visual).
- ✗ `fill` — idem.
- Justificação: stroke/fill são **cosméticos** (rendering
  visual), não estruturais. Refinos futuros candidatos
  Fase 5 NÃO-reservada per política P158.

**Bonus: `Content::Table` herda refinos via delegate
`layout_grid`** (precedente P157A `Content::Table` layouter
delega a `layout_grid` clone simples). Audit em C1
empírico se Table precisa adaptações.

**Decisão arquitectural central — 4 decisões fixadas**:

### Decisão 1 — Variants novas vs atributos para Header/Footer

Vanilla: `GridHeader { children, repeat: bool }` + `GridFooter
{ children, repeat: bool }` são **elementos estruturados
separados** (P157C precedente: `TableHeader`/`TableFooter`
adicionados ao enum como variants Content).

**3 opções consideradas**:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | Variants Content novos `GridHeader`/`GridFooter` | Paridade vanilla literal; coerente P157C; +2 variants |
| β | Fields `header: Option<Box<Content>>` / `footer: Option<Box<Content>>` | Menos variants; sem `repeat` field |
| γ | Fields `header: Option<Vec<Content>>` / `footer: Option<Vec<Content>>` | Inferior a α (sem repeat semantic) |

**Decisão fixada — Opção α**: paridade literal P157C; +2
variants Content (`GridHeader`, `GridFooter`); cada um
com `children: Vec<Content>` + `repeat: bool` paridade
vanilla.

### Decisão 2 — Variant novo vs atributos para Cell

Vanilla: `GridCell { body, x, y, colspan, rowspan, align,
fill, stroke, inset, breakable }` é elemento separado.
Subset escolhido: 5 fields paridade P157B `TableCell`.

**Decisão fixada — Opção α**: variant Content novo
`GridCell { body, x: Option<usize>, y: Option<usize>,
colspan: Option<usize>, rowspan: Option<usize> }` (paridade
P157B literal; 5 fields).

`align`/`fill`/`stroke`/`inset`/`breakable` per-cell:
**scope-out** P224. Refinos futuros candidatos.

### Decisão 3 — Placement algorítmico real ou armazenado?

P157B `TableCell` armazenou fields mas DEBT-34e ficou
aberto (algoritmo placement ignorado). **Opção δ humana
fixou "fecha DEBT-34d/e"** — significa implementar placement
real.

**Algoritmo vanilla placement** (per audit C1 vanilla
`layout/grid/cells.rs`):
- `x: None` + `y: None` → próxima posição livre
  (left-to-right, top-to-bottom; auto-placement).
- `x: Some(n)` + `y: None` → coluna explícita; próxima
  linha livre nessa coluna.
- `x: None` + `y: Some(n)` → linha explícita; próxima
  coluna livre nessa linha.
- `x: Some(n)` + `y: Some(n)` → posição totalmente
  explícita.
- `colspan: Some(n)` → célula ocupa N colunas adjacentes
  (validação: cabe).
- `rowspan: Some(n)` → célula ocupa N linhas adjacentes
  (validação: cabe; pode estender rows).
- Conflito (2 cells na mesma posição) → erro hard.

**Decisão fixada — implementar placement real em L1 puro**:
- Função `place_cells(cells: Vec<GridCell>, num_cols:
  usize) -> SourceResult<Vec<PlacedCell>>` em
  `01_core/src/rules/layout/grid_placement.rs` (módulo
  novo).
- Trabalho L1 puro (algorítmico; não toca layout
  geometric).
- Layouter consome `Vec<PlacedCell>` para iteração.
- DEBT-34d + DEBT-34e fecham simultaneamente.

### Decisão 4 — Atomização interna A/B/C ou flat?

Magnitude L (~5-8h) justifica atomização interna explícita
em 3 sub-fases:

- **P224.A** — `gutter` + `align` + `inset` (atributos
  aditivos simples; refino field-by-field paridade P156G+H+I).
- **P224.B** — `header` + `footer` (variants novos paridade
  P157C; +2 variants Content).
- **P224.C** — `GridCell` + placement algorítmico (variant
  novo paridade P157B + fecha DEBT-34d/e).

**Decisão fixada — Opção atomização interna A/B/C**: cada
sub-fase é cláusula dedicada (C2.A, C2.B, C2.C); critério
de aceitação por sub-fase. Magnitude planeada acumulada:
S+ + M + M+ = L.

Atomização externa em P224.A, P224.B, P224.C separados
**rejeitada** — humano fixou Opção δ no passo único; pattern
P156G+H+I precedente para subset Fase agregado.

Reuso de dados (sem recolha nova):

- `Content::Grid { columns, rows, cells }` baseline
  P82-84.6.
- `Content::Table { columns, rows, children }` baseline
  P157A (herda refinos via delegate).
- `Content::TableCell { body, x, y, colspan, rowspan }`
  baseline P157B (precedente literal para GridCell).
- `Content::TableHeader/Footer` baseline P157C (precedente
  para GridHeader/Footer).
- `Sides<Length>` infraestrutura P156C.
- `Align2D` baseline P84.5.
- `extract_tracks` helper `pub(super)` P157A.
- `extract_length` helper privado P156C+ (N=9 cumulativo
  pós-P223).
- `extract_alignment` helper P84.5/P84.6.
- `extract_usize_or_none_min` helper P157B.
- DEBT-34d (placement algorítmico Grid completo) — aberto.
- DEBT-34e (placement TableCell ignorado) — aberto.

---

## §2 Cláusulas (12 — atomização interna)

### C1 — Inventário pré-P224 baseline Grid

Auditoria empírica:

```
grep -n "Grid {" 01_core/src/entities/content.rs
grep -n "Content::Grid\|Content::Table" 01_core/src/
grep -n "fn layout_grid" 01_core/src/rules/layout/
grep -rn "DEBT-34" 00_nucleo/DEBT.md
```

Hipótese:
- `Content::Grid` em `entities/content.rs` com 3 fields
  baseline (`columns: Vec<TrackSizing>`, `rows: Vec<TrackSizing>`,
  `cells: Vec<Content>`).
- Arms cascata em ~7-8 sítios (paridade P217+P223).
- `layout_grid` function em `rules/layout/grid.rs` ou
  `rules/layout/mod.rs`.
- `native_grid` em `stdlib/layout.rs` ou `stdlib/structural.rs`.
- DEBT-34d aberto (Grid placement); DEBT-34e aberto
  (TableCell placement).

Empíricamente verificar:
1. Localização exacta + visibility de cada peça.
2. Field naming (`cells` vs `children` — diferença ortográfica
   Grid/Table).
3. Tests existentes Grid+Table que possam quebrar.

Se signature ou estrutura divergir significativamente:
registar `P224.div-1`.

### C2.A — Sub-fase A: gutter + align + inset (atributos
aditivos)

Editar `01_core/src/entities/content.rs` variant `Grid`
adicionando 3 fields:

```rust
Grid {
    columns: Vec<TrackSizing>,
    rows: Vec<TrackSizing>,
    cells: Vec<Content>,
    /// P224.A — gutter entre cells (default zero;
    /// uniforme — paridade vanilla simplificada).
    gutter: Option<Length>,
    /// P224.A — alignment uniforme aplicado a todas as
    /// cells; default `Align2D::default()` (top-left).
    align: Option<Align2D>,
    /// P224.A — inset uniforme (margem interna em cada
    /// cell); default zero (paridade pattern P156G inset).
    inset: Sides<Length>,
    /// P224.B — header opcional.
    header: Option<Box<Content>>,
    /// P224.B — footer opcional.
    footer: Option<Box<Content>>,
},
```

**Atributos aditivos** (subset Opção δ per ADR-0054 graded):
- `gutter: Option<Length>` — espaço uniforme entre cells
  (paridade pattern Smart→Option N=7 cumulativo); `None`
  == zero.
- `align: Option<Align2D>` — alignment uniforme; default
  `None` (top-left implícito).
- `inset: Sides<Length>` — margem interna; default zero;
  paridade P156G+H+I.

**Notas em Decisão 1 C2.A**:
- Vanilla `gutter` aceita `Length | Auto`; subset escolhido
  `Option<Length>` (paridade pattern P156I `Stack.spacing`).
- Vanilla `inset` aceita `Length | Sides<Length>` (uniforme
  ou per-side); subset escolhido `Sides<Length>` directo
  (paridade P156G).
- Vanilla `align` aceita `Alignment | Array<Alignment>`
  (uniforme ou per-cell); subset escolhido `Option<Align2D>`
  uniforme (paridade ADR-0054 graded; per-cell adiado a
  refino futuro).

Magnitude isolada: **S+ (~1.5h)**.

### C2.B — Sub-fase B: header + footer (variants Content)

Editar `01_core/src/entities/content.rs` adicionando 2
variants Content novos:

```rust
/// P224.B — Grid header (paridade P157C TableHeader).
GridHeader {
    children: Vec<Content>,
    /// `repeat: bool` — armazenado mas semantic adiada
    /// per ADR-0054 graded (paridade P156D weak + P156E
    /// weak + P156G breakable + P223 float; pattern N=5
    /// cumulativo).
    repeat: bool,
},

/// P224.B — Grid footer (paridade P157C TableFooter).
GridFooter {
    children: Vec<Content>,
    /// `repeat: bool` — paridade GridHeader (N=5).
    repeat: bool,
},
```

Total variants Content: **56 → 58** (+2: GridHeader,
GridFooter).

**Decisões fixadas em C2.B**:
- `children: Vec<Content>` paridade P157C literal (não
  `Arc<[Content]>` — refino futuro candidato; paridade
  P157C immediate baseline).
- `repeat: bool` armazenado mas semantic adiada (paridade
  pattern N=5 cumulativo `weak`/`breakable`/`float`/
  `breakable` Block).

Arms cascata para GridHeader + GridFooter (paridade Stack
N children):
- `is_empty` — `children.iter().all(|c| c.is_empty())`.
- `plain_text` — concatena com `\n`.
- `PartialEq::eq` — children + repeat.
- `map_content` / `map_text` — recurse em cada child.
- `materialize_time` (introspect) — recurse.
- `walk` (introspect) — walk em cada child.
- `layout_content` — preservar children no contexto Grid
  (semantic real adiada — header/footer renderizam como
  rows normais sem destaque visual; refino futuro
  candidato).
- `locatable.rs` — não-locatable (paridade Pagebreak/Stack).

Magnitude isolada: **M (~2-3h)**.

### C2.C — Sub-fase C: GridCell + placement algorítmico
(fecha DEBT-34d/e)

Editar `01_core/src/entities/content.rs` adicionando 1
variant Content novo:

```rust
/// P224.C — Grid cell estruturado (paridade P157B
/// TableCell literal; 5 fields).
GridCell {
    body: Box<Content>,
    x: Option<usize>,
    y: Option<usize>,
    colspan: Option<usize>,
    rowspan: Option<usize>,
},
```

Total variants Content: **58 → 59** (+1: GridCell).

Arms cascata para GridCell (paridade P157B TableCell
literal):
- `is_empty` — proxy `body.is_empty()`.
- `plain_text` — recurse body.
- `PartialEq::eq` — 5 fields.
- `map_content` / `map_text` — recurse body; preservar
  x/y/colspan/rowspan (Copy).
- `materialize_time` — recurse body.
- `walk` — walk body.
- `layout_content` — consume placement results.

**Módulo novo `01_core/src/rules/layout/grid_placement.rs`**
(L1 puro; algorítmico):

```rust
//! P224.C — Placement algorítmico Grid completo.
//! Fecha DEBT-34d (Grid placement) + DEBT-34e (TableCell
//! placement ignorado).
//!
//! Algoritmo paridade vanilla `layout/grid/cells.rs`:
//! - Cells com `x`/`y` explícitos → posição fixada.
//! - Cells com `x: None` + `y: None` → auto-placement
//!   left-to-right, top-to-bottom.
//! - `colspan`/`rowspan` ocupam N colunas/linhas
//!   adjacentes.
//! - Conflito (2 cells na mesma posição) → erro hard.

pub struct PlacedCell {
    pub body: Content,
    pub row: usize,
    pub col: usize,
    pub colspan: usize,  // >= 1 (resolvido per default 1)
    pub rowspan: usize,  // >= 1
}

pub fn place_cells(
    cells: Vec<Content>,  // cells originais (possivelmente
                          // wrappadas em GridCell ou raw Content)
    num_cols: usize,
) -> SourceResult<Vec<PlacedCell>> {
    let mut grid_occupancy: Vec<Vec<bool>> = Vec::new();
    let mut placed: Vec<PlacedCell> = Vec::new();
    let mut auto_cursor: usize = 0;  // index linear next free

    // 2 passagens:
    // Pass 1: place explicit cells (x or y or both Some).
    // Pass 2: place auto cells (x None + y None).

    // [...algoritmo detalhado em §3 do relatório...]

    Ok(placed)
}
```

**Layouter consumer** em `rules/layout/grid.rs`:
- Antes de iterar cells, chamar `place_cells(cells, num_cols)`.
- Iterar `Vec<PlacedCell>` em ordem; renderizar cada body
  na célula `(row, col)` ocupando `colspan × rowspan`.
- Save/restore `cell_origin_*` paridade P84.6.

**DEBT-34d + DEBT-34e fecham**: critério de fecho cumprido
(placement algorítmico real materializado).

Magnitude isolada: **M+ (~3-5h)**.

### C3 — Arms cascata exhaustivos (compiler-driven)

Total arms refino/aditivos P224 (paridade P217 estratégia):

**`entities/content.rs`** (5 arms refino Grid + 5 arms
GridHeader + 5 arms GridFooter + 5 arms GridCell = 20 arms):
- `is_empty` — Grid (proxy children/cells); 3 novas
  variants per spec.
- `plain_text` — Grid (concatena); 3 novas variants per
  spec.
- `PartialEq::eq` — Grid (8 fields agora); 3 novas
  variants 5/2/5 fields.
- `map_content` — Grid (recurse cells + header/footer
  bodies); 3 novas variants recurse.
- `map_text` — idem map_content.

**`rules/introspect.rs`** (2 arms × 4 variants = 8 arms):
- `materialize_time` — Grid + 3 novas.
- `walk` — Grid + 3 novas.

**`rules/layout/grid.rs`** (1 arm refino Grid + 0 directo
para new variants — consumidos por placement):
- `layout_grid` — refino accept gutter + align + inset +
  header/footer + placement results.

**`rules/layout/mod.rs::layout_content`** (1 arm Grid
preservado; +3 arms aditivos para GridHeader/GridFooter/
GridCell):
- `Content::Grid { ... }` — preservado; consume placement.
- `Content::GridHeader { children, repeat: _ }` — semantic
  adiada; layout children sequencial (paridade P157C).
- `Content::GridFooter { children, repeat: _ }` — idem.
- `Content::GridCell { body, x: _, y: _, colspan: _,
  rowspan: _ }` — semantic adiada para `layout_content`
  fora de Grid context; arm consumido por placement em
  Grid arm.

**`rules/introspect/locatable.rs`** (catch-all ou explicit):
- 3 novas variants não-locatable.

**`rules/layout/mod.rs::measure_content_constrained`** (1
arm Grid + 3 aditivos):
- Grid refino measurement com gutter/inset.
- 3 novas variants: recurse body/children sum.

Total: **~30+ arms cumulativos em ~5 ficheiros L1**
(compiler-driven; iterar até zero errors).

### C4 — Refino `native_grid` + 2 stdlib novas + paridade

Editar `stdlib/layout.rs` ou `stdlib/structural.rs`
(localização per audit C1) função `native_grid`:

```rust
pub fn native_grid(...) -> SourceResult<Value> {
    // ... existing columns/rows extraction ...

    // P224.A — extract gutter (default None).
    let gutter = match args.named.get("gutter") {
        Some(val) => Some(extract_length(val, "grid", "gutter")?),
        None => None,
    };

    // P224.A — extract align (default None).
    let align = match args.named.get("align") {
        Some(val) => Some(extract_alignment_value(val, "grid")?),
        None => None,
    };

    // P224.A — extract inset (default zero Sides).
    let inset = match args.named.get("inset") {
        Some(val) => extract_sides_lengths(val, "grid", "inset")?,
        None => Sides::default(),
    };

    // P224.B — extract header/footer from children Vec.
    let (header, footer, cells) =
        split_header_footer(children_resolved)?;

    // Reject scope-out atributos (stroke/fill explicitamente).
    for key in args.named.keys() {
        if !["gutter", "align", "inset", "header", "footer"]
            .contains(&key.as_str())
        {
            return Err(eco_format!(
                "grid: named arg `{}` não suportado (paridade graded;
                 stroke/fill são cosméticos scope-out; refinos
                 futuros candidatos)",
                key
            ));
        }
    }

    Ok(Value::Content(Content::Grid {
        columns, rows, cells, gutter, align, inset, header, footer,
    }))
}
```

**`native_grid_header`** e **`native_grid_footer`** stdlib
novas:
```rust
pub fn native_grid_header(...) -> SourceResult<Value> {
    let children = collect_content_children(args)?;
    let repeat = extract_bool_with_default(args, "grid.header", "repeat", false)?;
    Ok(Value::Content(Content::GridHeader { children, repeat }))
}

pub fn native_grid_footer(...) -> SourceResult<Value> {
    // paridade native_grid_header com repeat default false.
}
```

**`native_grid_cell`** stdlib nova:
```rust
pub fn native_grid_cell(...) -> SourceResult<Value> {
    // paridade native_table_cell P157B literal (mesma
    // signature 5 fields).
}
```

Scope register em `eval/mod.rs` (3 novas registos):
```rust
scope.define("grid_header", ...);
scope.define("grid_footer", ...);
scope.define("grid_cell", ...);
```

**Naming flat** paridade P157B `table_cell` (não vanilla
`grid.cell` namespaced; documentado per ADR-0033).

Stdlib funcs count: **56 → 59** (+3: grid_header,
grid_footer, grid_cell).

### C5 — Sentinelas P224 (atomização interna A/B/C)

Tests P224 (paridade P157B + P157C scopes):

**P224.A tests** (~12 tests):
- 3 unit content: Grid variant aceita gutter/align/inset;
  partial_eq 8-fields; defaults preservados.
- 6 unit stdlib: gutter Length/negativo; align Align2D/string
  invalid; inset Sides/Length single.
- 3 E2E layout: gutter visível entre cells; align top-left
  vs center; inset top/bottom aplicado.

**P224.B tests** (~10 tests):
- 2 unit content GridHeader: variant + partial_eq + recurse.
- 2 unit content GridFooter: idem.
- 4 unit stdlib: grid_header sem args / com children /
  com repeat; grid_footer idem.
- 2 E2E layout: header renderiza primeiro; footer renderiza
  por último.

**P224.C tests** (~15 tests; placement algorítmico foco
principal):
- 4 unit content GridCell: variant + partial_eq + recurse.
- 4 unit stdlib: grid_cell sem args / com body / com
  x/y/colspan/rowspan / scope_out rejeitados.
- 7 unit placement: auto placement linear; explicit x/y;
  colspan/rowspan ocupação adjacente; conflito 2-cells
  rejeitado; colspan excede num_cols rejeitado;
  auto-cursor advance após explicit; mistura auto+explicit.

Total tests P224: **~37 tests cumulativos** (12+10+15).
Esperado pós-P224: **2012 + 37 = 2049 verdes** (margem 2-5
para ajustes empíricos).

### C6 — L0 `entities/content.md` decisão Opção α vs γ

Decisão sobre L0:
- **Opção α** — secção dedicada `## Variant Content::Grid
  refino — Passo 224` + 3 sub-secções para variants novas
  (GridHeader/GridFooter/GridCell). Paridade P157C
  (TableHeader/Footer documentados separadamente).
- **Opção γ** — sem extensão L0 (consolida pattern N=6
  → 7).

**Decisão fixada — Opção α**: P224 introduz **2 variants
Content novos substantivos** + placement algorítmico real
+ fecha 2 DEBTs. Magnitude L justifica documentação
formal L0. Distinto de P217 (variant aditivo simples)
+ P218 (stdlib aditivo) + P219 (refactor arm) + P220
(variant + arm + stdlib agregado) + P222 (stdlib expose)
+ P223 (refino aditivo a variant existente).

**Reabertura do pattern "L0 minimal"** justificada por
escala material P224. Pattern emergente "L0 formal para
variants Content novos + fecho DEBT" N=1 inaugurado.

Hash propagado em `entities/content.md` (mudança
substantiva).

### C7 — DEBT.md fechamento DEBT-34d + DEBT-34e

Editar `00_nucleo/DEBT.md`:

**DEBT-34d — Grid placement algorítmico completo**:
- Título: "Grid placement algorítmico completo — **ENCERRADO
  (Passo 224)** ✓".
- Fechado em: 2026-05-13 (P224).
- Resolvido por: módulo novo
  `01_core/src/rules/layout/grid_placement.rs` com função
  `place_cells(cells, num_cols)` que implementa algoritmo
  vanilla paridade (auto + explicit + colspan/rowspan +
  conflito detection). Layouter consome resultado.
- Critério fecho 5/5 satisfeito:
  - ✅ Algoritmo placement materializado.
  - ✅ Tests E2E + unit cobrem casos.
  - ✅ Conflito 2-cells rejeitado com erro hard.
  - ✅ Auto-placement linear funcional.
  - ✅ Colspan/rowspan adjacente correto.

**DEBT-34e — TableCell placement ignorado**:
- Título: "TableCell placement ignorado — **ENCERRADO
  (Passo 224)** ✓".
- Fechado em: 2026-05-13 (P224).
- Resolvido por: Grid placement algorítmico (DEBT-34d
  ENCERRADO) **herda** para `Content::Table` via delegate
  `layout_grid` (precedente P157A "Layouter delega a
  `layout_grid` clone simples"). TableCell x/y/colspan/
  rowspan agora resolvidos via `place_cells` no caminho
  Grid → Table delegate.

**Saldo DEBTs**: 13 → **11 abertos** (DEBT-34d + DEBT-34e
fecham).

### C8 — Verificação tests workspace

Critério: 2012 verdes pré-P224 + ~37 novos = **2049 verdes**.

**Risco regressão Table baseline**: P157A/B/C tests
pre-existentes podem precisar adaptação se `place_cells`
fecha DEBT-34e e altera placement observable.

**Risco regressão Grid baseline**: P82/P83/P84.6 tests
pre-existentes (linhas/colunas/cell_origin) — placement
algorítmico real pode mudar observable em casos com
colspan/rowspan (que antes eram ignorados).

**Hipótese provável N tests afectados**: 2-5 testes
P157A/B/C Table com cells x/y/colspan/rowspan que agora
respondem ao placement. Adaptação: tests verificam novo
behaviour correcto (sentinela de regressão DEBT-34d/e
fecho).

**Critério ajustado**: 2012 verdes preservados (após
adaptação de N=2-5 tests pre-existentes) + ~37 novos =
~2049 verdes.

### C9 — Verificação lint

```
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério: 0 violations. Hashes propagados em:
- `entities/content.rs` (variant Grid refino + 3 variants
  novos).
- `entities/content.md` L0 (Opção α — secção dedicada +
  3 sub-secções).
- `rules/layout/grid.rs` (refino layout_grid).
- `rules/layout/grid_placement.rs` (módulo novo).
- `rules/layout/mod.rs` (arms novos).
- `rules/introspect.rs` (arms novos).
- `rules/stdlib/layout.rs` ou `stdlib/structural.rs`
  (refino native_grid + 3 stdlib novas).
- `rules/stdlib/mod.rs` (re-exports).
- `rules/eval/mod.rs` (scope registers).

L0 `entities/content.md` ganha **secção dedicada** —
reabertura justificada (Opção α). Hash propagado.

### C10 — Inventário 148 reclassificação P224 cumulativa

Editar `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`:

**§A.5 Layout linha 141 `grid(columns, ...)`**:
- Classificação: `parcial ⁵` → **`implementado⁺` ⁵ ⁴⁵**.
- Referência: "Passos 82–84.6" → "Passos 82 + 83 + 84.6
  + 224".
- Nota: refino substantivo aditivo +5 fields + placement
  algorítmico (fecha DEBT-34d/e); 2 variants novos
  GridHeader/Footer + 1 GridCell; stroke/fill cosméticos
  scope-out.

**Tabela A.5 Layout**: distribuição actualizada:
- Pré-P224 (pós-P223): `12/3/3/0/0 = 18`.
- Pós-P224: **`12/4/2/0/0 = 18`** (1 parcial → impl⁺;
  zero ausentes preservado).

**Cobertura Layout per metodologia §A.9**:
- Pré: `(12+3)/18 = 83%`.
- Pós: `(12+4)/18 = **89%**` (+6pp real).

**Tabela A user-facing total**: re-distribuição:
- Pré: `68/26/25/20/2 = 141`.
- Pós: `68/27/24/20/2 = 141`.
- Cobertura total: `(68+27)/141 ≈ **67%**` (preservada;
  Layout não é maioria do user-facing).

**Tabela B.2 Content variants**: **3 entradas novas**:
- `GridHeader { children, repeat }` ↔ vanilla GridHeader
  (`implementado⁺` — repeat semantic adiada).
- `GridFooter { children, repeat }` ↔ vanilla GridFooter
  (idem).
- `GridCell { body, x, y, colspan, rowspan }` ↔ vanilla
  GridCell (subset minimal; `align`/`fill`/`stroke`/`inset`/
  `breakable` per-cell scope-out).

**Content variants count**: **56 → 59** (+3 cumulativos
P224).

**Footnote ⁴⁵ P224** adicionada documentando:
- Refino substantivo Opção δ cumulativa.
- +5 fields Grid + 3 variants novos.
- Placement algorítmico real fecha DEBT-34d/e.
- 4 decisões fixadas + scope-out cosméticos.
- Pattern emergente "fecho cumulativo de DEBTs via refino
  composto" N=1.
- Pattern emergente "subset Fase agregado L cumulativo
  pós-M9c" N=2 (P218+P220 agregado vs P224 substantivo).
- Reclassificação `grid` parcial → impl⁺.
- Δ Layout cobertura: 83% → 89% real (+6pp).
- Cumulativo Fase 4: +17pp (P222 +6pp + P223 +5pp + P224
  +6pp).

### C11 — ADR-0061 anotação Fase 4 candidata 3/3 (fecha
série α)

Editar `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`:

Bloco `### P224 anotação — Fase 4 Layout candidata
sub-passo 3 (Grid refino substantivo Opção δ; fecha série
α "terminar Layout")` adicionado após `### P223 anotação`:

```markdown
**Fase 4 Layout candidata 3/3 sub-passos materializada**
**série α "terminar Layout" fechada estructuralmente**:
- P222 measure ✓
- P223 place ✓
- P224 grid substantivo Opção δ ✓

Trabalho cumulativo Fase 4:
- 3 variants Content novos cumulativos (GridHeader/Footer
  + GridCell).
- +5 fields refino a variant existente (Grid).
- +2 fields refino a variant existente (Place).
- 4 stdlib funcs novas (native_measure + native_grid_header
  + native_grid_footer + native_grid_cell).
- 1 stdlib refino (native_place + 2 named args; native_grid
  + 5 named args).
- 1 helper visibility promotion (measure_content).
- 1 módulo novo L1 (grid_placement.rs).
- 2 DEBTs fechados estructuralmente (DEBT-34d + DEBT-34e
  via P224.C placement algorítmico).
- DEBT-37 §"Divergência" fechada (P223).
- 0 ADR PROPOSTA→IMPLEMENTADA na série α (ADR-0066
  PROPOSTA mantida; ADR-0061 IMPLEMENTADA mantida desde
  P221).
- ~62 tests adicionados Fase 4 (P222 11 + P223 14 + P224
  ~37); 1998 → ~2049 verdes.
- Reclassificações: 3 entradas parcial → impl⁺ (measure +
  place + grid).
- Cobertura Layout: 78% Fase 3 → 83% pós-P223 → **89%
  pós-P224** (+11pp cumulativo Fase 4 real; +17pp visíveis
  per metodologia).

**ADR-0061 status mantido IMPLEMENTADO** (Fase 3 fechada
P221; Fase 4 candidata 100% materializada per Opção α
P221 §8 fechada P224). Fase 5 candidata futura (refinos
stroke/fill + per-cell align Grid + flow real Place float)
identificada mas **NÃO reservada** per política P158.

**P225 será encerramento documental** (paridade P221 para
Fase 3): ADR-0061 anotação Fase 4 candidata completa;
inventário 148 footnote consolidada; blueprint marca
§3.0terdecies.
```

**Status ADR-0061**: IMPLEMENTADO mantido. Anotação Fase 4
candidata 3/3 sub-passo fecha série α.

### C12 — Critério de aceitação P224

Critério (compositivo per atomização interna):

**P224.A**:
- ✓ Grid variant +3 fields (gutter/align/inset).
- ✓ ~12 tests P224.A verdes.
- ✓ Tests Grid pré-existentes preservados.

**P224.B**:
- ✓ +2 variants Content (GridHeader/GridFooter) com 5+5
  arms cascata.
- ✓ +2 stdlib funcs (native_grid_header/native_grid_footer).
- ✓ ~10 tests P224.B verdes.

**P224.C**:
- ✓ +1 variant Content (GridCell) com 5 arms cascata.
- ✓ +1 stdlib func (native_grid_cell).
- ✓ Módulo novo `grid_placement.rs` com `place_cells`.
- ✓ ~15 tests P224.C verdes.
- ✓ **DEBT-34d fechado** (placement algorítmico real).
- ✓ **DEBT-34e fechado** (Table herda via delegate).

**Cumulativo P224**:
- ✓ 2012 verdes + ~37 novos = **~2049 verdes**.
- ✓ 0 violations preservadas.
- ✓ Content variants 56 → **59** (+3).
- ✓ Stdlib funcs 56 → **59** (+3).
- ✓ §A.5 `grid(...)` reclassificada parcial → impl⁺.
- ✓ Cobertura Layout: 83% → **89%** real (+6pp).
- ✓ DEBTs abertos: 13 → **11** (DEBT-34d/e fecham).
- ✓ **Série α "terminar Layout" fechada estructuralmente**
  (3/3 sub-passos Fase 4 candidata).

---

## §3 Output

1 ficheiro relatório longo:
`00_nucleo/materialization/typst-passo-224-relatorio.md`.

Estrutura (~10-15 KB devido à magnitude L) com 10 §s:

- §1 O que foi feito (sumário 5-8 linhas).
- §2 Inventário pré-P224 baseline Grid + DEBT-34d/e (C1).
- §3 P224.A refino Grid +3 fields (C2.A + arms cascata).
- §4 P224.B GridHeader/Footer variants novos (C2.B + arms).
- §5 P224.C GridCell + placement algorítmico (C2.C +
  módulo grid_placement.rs + Layouter consumer).
- §6 Stdlib refino + 3 novas + scope register (C4).
- §7 Decisões substantivas (4 decisões fixadas + atomização
  interna A/B/C + scope-out cosméticos).
- §8 Resultados verificação (~37 tests + 2012 pre-existentes
  + N=2-5 adaptações Table).
- §9 Inventário 148 + DEBT.md fechamentos + ADR-0061
  anotação cumulativa.
- §10 Próximo sub-passo (**P225 encerramento Fase 4 Layout
  documental**; paridade P221 para Fase 3).

Código alterado:
- **Editado**: `01_core/src/entities/content.rs` (variant
  Grid refino + 3 variants novos + arms cascata + ~20
  unit tests).
- **Editado**: `01_core/src/rules/introspect.rs` (arms
  novos).
- **Editado**: `01_core/src/rules/layout/grid.rs` (refino
  layout_grid + consume placement).
- **Editado**: `01_core/src/rules/layout/mod.rs` (arms
  novos).
- **Editado**: `01_core/src/rules/introspect/locatable.rs`
  (arms novos).
- **Editado**: `01_core/src/rules/stdlib/layout.rs` ou
  `stdlib/structural.rs` (refino + 3 stdlib novas).
- **Editado**: `01_core/src/rules/stdlib/mod.rs` (re-exports).
- **Editado**: `01_core/src/rules/eval/mod.rs` (scope
  registers).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+E2E).
- **Novo**: `01_core/src/rules/layout/grid_placement.rs`
  (módulo placement algorítmico).
- **Editado**: `00_nucleo/prompts/entities/content.md`
  (secção dedicada P224 + 3 sub-secções).
- **Editado**: `00_nucleo/DEBT.md` (DEBT-34d + DEBT-34e
  ENCERRADOS).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (Tabela A.5 + Tabela B.2 cumulativa + §A.5 reclassificação
  + footnote ⁴⁵ P224).
- **Editado**: `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md`
  (+ anotação Fase 4 candidata 3/3 fecha série α).

**1 ficheiro novo** (`grid_placement.rs`); resto editado.

---

## §4 Não-objectivos

- Implementar `stroke` / `fill` Grid — cosméticos scope-out
  formal P224; Fase 5 candidata futura NÃO-reservada per
  política P158.
- Per-cell `align` / `inset` / `fill` / `stroke` /
  `breakable` em GridCell — subset paridade P157B literal
  (5 fields); refino futuro candidato.
- Show rules `#show grid: ...` — fora de escopo Fase 4
  Layout.
- Multi-region flow real para Grid (cells que excedem page) —
  diferido a Fase 5 candidata (paridade decisão Opção B
  P219 columns).
- Promover ADR-0066 → IMPLEMENTADO — paridade decisão P222.
- Reabrir decisão P216B (`Regions` minimal) — preservada.
- Reabrir DEBT-37 ou DEBT-56 — preservadas em estado
  encerrado.
- L0 extensão para todos variants — apenas Grid + 3 novos
  (Opção α justificada).
- Helper construtor `Content::grid_with_header(...)` —
  overhead inflacionário.
- ADR meta documental "L0 minimal para refactors" — Caminho
  4 candidato P221 §8 ainda diferido (Opção α P224 reabre
  pattern; promoção formal continua em aberto se humano
  priorizar pós-P225).

---

## §5 Riscos a evitar (cumulativo magnitude L)

1. **Magnitude real exceder planeada significativamente**:
   L (~5-8h) é estimativa generosa. Risco real >10h se
   placement algorítmico exigir 2-3 iterações de refactor.
   Mitigação: atomização interna A/B/C; cada sub-fase é
   "checkpoint" verde antes de prosseguir.
2. **Placement algorítmico bugs subtis**: auto-placement
   com colspan/rowspan tem casos bordo (cell que excede
   num_cols, conflito com cell explicit já posicionada).
   Mitigação: ~7 unit tests dedicados a placement em C5
   P224.C.
3. **Table baseline regressões**: P157A/B/C Table tests
   podem mudar observable quando placement real fecha
   DEBT-34e. Mitigação: hipótese N=2-5 adaptações
   intencionais documentadas em C8.
4. **Refactor pattern explosão**: variant Grid com 8 fields
   é mais complexo que P156G+H+I (4-5 fields). Compiler-driven
   pode produzir >20 errors. Mitigação: paciência iterativa;
   pattern P217 estratégia.
5. **`stroke`/`fill` scope-out questionado**: tentação de
   "fazer também" porque "está no caminho". Rejeitado —
   atributos cosméticos genuinamente fora do escopo Fase
   4 (paridade P156G+H+I scope-outs).
6. **L0 secção dedicada inflada**: tentação de detalhar
   todos os algorítmos em L0. Rejeitada — L0 documenta
   semantic + interface; algoritmo placement fica em código
   + footnote ⁴⁵.
7. **Esquecer DEBT-34e ao fechar DEBT-34d**: ambos têm de
   fechar simultaneamente (Table herda via delegate).
   Mitigação: C7 explicita ambos.
8. **Variant naming conflito**: `Content::GridCell` vs
   `Content::TableCell` (ambos com 5 fields idênticos).
   Aceitar dupla — paridade vanilla literal (vanilla também
   tem ambos separados). Refactor para unificar fica como
   refino futuro candidato.
9. **Helper `place_cells` API divergente face a vanilla**:
   vanilla pode ter signature diferente. Mitigação:
   paridade observable ADR-0033 (interno pode divergir).
10. **Tests P224.A passam mas P224.B falha mid-passo**:
    risco de regressão a meio do passo. Mitigação:
    atomização interna A/B/C como checkpoint formal verde;
    pode interromper e retomar se necessário (paridade
    pattern P156E + P220 agregação mas com checkpoints).
11. **Cobertura Layout 89% vs blueprint Opção γ 78%
    divergência amplifica**: pós-P224 a divergência
    metodológica vs visual é 89% real vs 78% visual. P225
    documental resolverá (decisão Opção γ refresh para
    "89% (12 impl + 4 impl⁺ + 2 parcial)").
12. **DEBT-34d + DEBT-34e fechamento dual incorrecto**:
    se placement algorítmico cobre Grid mas não Table via
    delegate, DEBT-34e não pode fechar. Mitigação: audit
    empírico em C1 + verificação Table delegate em
    P224.C.

---

## §6 Hipótese provável

C1 confirmará `Content::Grid { columns, rows, cells }`
baseline P82-84.6; Table via delegate `layout_grid`;
DEBT-34d/e abertos.

C2.A refino Grid +3 fields (gutter/align/inset);
magnitude S+ real.

C2.B GridHeader/Footer +2 variants paridade P157C;
magnitude M real.

C2.C GridCell +1 variant paridade P157B + módulo
`grid_placement.rs` novo + Layouter consumer; magnitude
M+ real.

C3 cobrirá ~30+ arms exhaustivos (compiler-driven; iterar
até zero errors; possíveis 20-40 errors E0027/E0063).

C4 refino native_grid + 3 stdlib novas; magnitude S+
real.

C5 criará ~37 tests novos cumulativos (~12+10+15).

C6 fixará Opção α (secção dedicada L0; reabertura
justificada).

C7 fechará DEBT-34d + DEBT-34e simultaneamente; saldo 13
→ 11.

C8 reportará ~2049 tests verdes (2012 + 37; possíveis
N=2-5 adaptações Table).

C9 reportará 0 violations; hashes propagados em ~10
ficheiros.

C10 reclassificará `grid(...)` parcial → impl⁺; cobertura
Layout 83% → 89%; user-facing 67% preservada.

C11 anotará ADR-0061 Fase 4 candidata 3/3 fecha série α;
P225 encerramento documental natural.

Custo real: L (~5-8h). Maior parcela em C2.C placement
algorítmico (M+) + C3 arms cumulativos + C5 ~37 tests.

Mas é hipótese, não decisão. C1-C12 fixam-se empíricamente.

---

## §7 Particularidade P224

P224 é estruturalmente distinto na trajectória pós-M9c:

- **Terceiro sub-passo Fase 4 Layout candidata** — paridade
  estrutural P220 (terceiro Fase 3 sub-fase b).
- **Fecha série α "terminar Layout" estructuralmente** —
  segundo marco interno pós-M9c após P221 (Fase 3). P225
  documental completa marco formal.
- **Refino substantivo composto** — distinto de P217-P223
  pelos seguintes critérios cumulativos:
  - +3 variants Content novos (GridHeader + GridFooter +
    GridCell).
  - +5 fields refino a variant existente (Grid).
  - +3 stdlib funcs novas + 1 stdlib refinada (native_grid).
  - +1 módulo L1 novo (`grid_placement.rs`).
  - 2 DEBTs fechados simultaneamente (DEBT-34d + DEBT-34e).
  - Atomização interna A/B/C explícita.
- **Pattern emergente "fecho cumulativo de DEBTs via
  refino composto" N=1 inaugurado** — distinto de DEBT-37
  (fechada via refino aditivo single P223) + DEBT-56
  (fechada via série composta P217-P220 encerrada P221).
- **Pattern emergente "subset Fase agregado L cumulativo
  pós-M9c" N=2** — primeiro foi P218+P220 agregados
  triviais; P224 é primeiro agregado substantivo (L) com
  atomização interna A/B/C explícita.
- **Pattern "L0 minimal para refactors" suspenso N=6 → 6
  estável** — P224 reabre Opção α (secção dedicada) para
  variants substantivos. Distinto de continuação Opção γ.
  Padrão refina-se: "Opção γ para refactors aditivos
  simples; Opção α para variants Content novos + DEBT
  fecho composto".
- **Cobertura Layout 83% → 89% real** per metodologia.
  **Terceiro aumento cumulativo Fase 4** (+17pp cumulativo:
  P222 +6pp + P223 +5pp + P224 +6pp).
- **Pattern emergente "Field armazenado semantic adiada"
  N=4 → 5** (P156D + P156E + P156G + P223 + **P224 repeat
  Header/Footer**). N=5 patamar empírico forte.
- **Anti-inflação 18ª aplicação cumulativa** pós-P205D —
  scope-out cosméticos stroke/fill explícito + atomização
  interna em vez de externa.
- **Sintoma de "tudo o que faz sentido em Layout pós-M9c"** —
  pós-P224, Layout cobertura 89% real com 4 parciais
  remanescentes (`columns`/`colbreak` parciais Fase 3
  scope-out + 2 parciais Fase 5 candidata stroke/fill).
  Layout em "estado terminal estructural" — Fase 4 esgota
  refinos materializáveis sem reabrir decisões
  arquitecturais maiores (Opção A multi-region).

Por isso §5 risco 1 (magnitude exceder) é o mais provável.
Tentação óbvia é "tudo cabe num passo porque está coerente".
Defesa: atomização interna A/B/C como checkpoint formal
verde; interrupção autorizada per spec se A ou B vermelho.

**Critério de aceitação P224**:
- ~37 tests novos verdes (12+10+15).
- 2012 tests pre-existentes preservados (após N=2-5
  adaptações intencionais Table; conta zero como regressão
  real).
- 0 violations.
- §A.5 `grid(...)` reclassificada parcial → impl⁺.
- Cobertura Layout: 83% → **89%** real (+6pp).
- Cobertura user-facing total: 67% preservada.
- DEBT-34d ENCERRADO ✓.
- DEBT-34e ENCERRADO ✓.
- Saldo DEBTs: 13 → 11.
- Fase 4 candidata Layout: 2/3 → **3/3 sub-passos** (P222
  ✓; P223 ✓; **P224 ✓**).
- Série α "terminar Layout" fechada estructuralmente.

**Estado pós-P224 esperado**:
- Tests workspace: 2012 → ~**2049** verdes.
- Stdlib funcs: 56 → **59** (+3).
- Content variants: 56 → **59** (+3).
- §A.5 distribuição: `12/4/2/0/0 = 18` (1 parcial →
  impl⁺; zero ausentes preservado).
- Cobertura Layout per metodologia: 83% → **89%**.
- Cobertura user-facing total: 67% preservada.
- ADR-0066 PROPOSTO; ADR-0061 IMPLEMENTADO; ADR-0078
  IMPLEMENTADO.
- DEBT-34d + DEBT-34e ambos ENCERRADOS.
- Saldo DEBTs: 13 → 11 abertos.
- 18 aplicações cumulativas anti-inflação.
- Pattern "L0 minimal" suspenso N=6 estável (Opção α
  reabertura justificada P224).
- Pattern "Field armazenado semantic adiada" N=4 → 5.
- Pattern "fecho cumulativo de DEBTs via refino composto"
  N=1 inaugurado.
- Pattern "subset Fase agregado L cumulativo pós-M9c"
  N=1 → 2.
- **Layout em estado terminal estructural** — refinos
  remanescentes são cosméticos (stroke/fill) ou exigem
  reabertura arquitectural maior (Opção A multi-region).
- **P225 será encerramento documental** Fase 4 Layout
  candidata (paridade P221 estrutura formal).
