# Passo 227 — A.1 `stroke` Grid + Table inheritance (Fase 5 Layout candidata sub-passo 1; valida ADR-0080 N=7 → 8)

**Série**: 227 (décimo-terceiro sub-passo Layout pós-M9c;
**primeiro sub-passo materialização Fase 5 Layout candidata**
per ADR-0079 PROPOSTO; primeiro sub-passo Categoria A
"cosméticos sem reabrir decisões").
**Marco**: nenhum (décimo-quinto passo pós-M9c; primeiro
sub-passo Fase 5 — paridade estrutural P222 para Fase 4;
**valida ADR-0080 PROPOSTO N=7 → 8** primeira aplicação
real pós-formalização do pattern "L0 minimal para refactors").
**Tipo**: refino aditivo a 2 variants existentes
(`Content::Grid` + `Content::Table`); 1 field novo a cada
variant + helper extract_stroke novo + `native_stroke`
constructor stdlib + renderização real simplificada
(Opção β).
**Magnitude**: M (~2-3h).
**Pré-condição**: P226 concluído (ADR-0079 PROPOSTO Fase
5 Layout roadmap; ADR-0080 PROPOSTO L0 minimal N=7;
diagnóstico amplo 4 categorias A+B+C+D; 2039 tests verdes;
0 violations; saldo DEBTs 12); humano fixou A.1 (decisão
literal pós-P226 §8); `Stroke` baseline P76 em
`entities/geometry.rs` (`{ paint: Color, thickness: f64 }`);
`Content::Grid` baseline pós-P224 (8 fields: columns/rows/
cells/gutter/align/inset/header/footer); `Content::Table`
baseline P157A (3 fields: columns/rows/children) delegate
`layout_grid`.
**Output**: 1 ficheiro relatório curto + código alterado em
~6-8 ficheiros L1 + L0 NÃO tocado (paridade ADR-0080 PROPOSTO
Opção γ N=7 → 8) + inventário 148 reclassificação Grid +
Table (footnote ⁴⁷) + ADR-0079 anotação Categoria A 1/5.

---

## §1 Trabalho

`Content::Grid` pós-P224 tem 8 fields cumulativos mas
**stroke/fill foram scope-out explícito em P224** (per
ADR-0054 graded; cosméticos não-estruturais). Inventário
148 §A.5 linha 141 `grid(columns, ...)` classificada
`implementado⁺ ⁵ ⁴⁵` com nota "stroke/fill cosméticos
scope-out". P226 diagnóstico amplo identificou A.1 +
A.2 stroke + fill como sub-passos cumulativos cosméticos
não-reservados.

**P227 materializa A.1**:
- **Grid +1 field**: `stroke: Option<Stroke>` (uniforme;
  paridade Smart→Option N=7 → 8).
- **Table +1 field**: `stroke: Option<Stroke>` (paridade
  Grid; Table herda renderização via delegate `layout_grid`).
- **Helper `extract_stroke(val, fn, field)`** novo em
  `stdlib/layout.rs` (aceita Length / Color / Stroke
  shorthands).
- **`native_stroke(paint:?, thickness:?)`** stdlib func
  nova constructor (paridade `native_rgb`).
- **`Value::Stroke(Stroke)`** enum variant novo (se audit
  C1 confirma ausência).
- **Renderização Opção β simplificada** em `layout_grid`:
  emite `FrameItem::Shape::Line` per cell border (top +
  bottom + left + right) sem deduplicação de linhas
  adjacentes (refino futuro candidato A.3).

**Decisão arquitectural central — 6 decisões fixadas**:

### Decisão 1 — Field tipo Opção α (Option<Stroke> uniforme)

3 opções consideradas:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | `stroke: Option<Stroke>` uniforme | Subset minimal; paridade pattern Smart→Option N=7 → 8 |
| β | `stroke: Sides<Option<Stroke>>` per-side | A.3 candidato separado (per-cell GridCell) |
| γ | Novo tipo `GridStroke` rico | Over-engineering; L+ |

**Decisão fixada — Opção α** porque:
- Subset minimal per ADR-0054 graded Fase 5 sub-passo 1.
- Per-side é A.3 candidato P226 diagnóstico amplo
  (per-cell GridCell + Block/Boxed outset/radius/clip).
- Paridade pattern Smart→Option N=7 → 8 (consolida em
  ADR-0080 PROPOSTO Opção γ literal).
- API surface minimal.

### Decisão 2 — Stdlib parsing Opção β (Length/Color/Stroke shorthands)

3 opções consideradas:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | Apenas `Value::Stroke` (literal) | Verboso utilizador |
| **β** | Length OR Color OR Stroke shorthands | Paridade vanilla UX |
| γ | β + Sides parser | A.3 candidato |

**Decisão fixada — Opção β** porque:
- Paridade vanilla user-experience: `stroke: 1pt` e
  `stroke: red` ambos comuns + idiomáticos.
- Sides parser fica para A.3.
- Precedente `extract_alignment` P84.5 aceita múltiplas
  formas.

Helper novo `extract_stroke(val, fn, field) -> SourceResult<Stroke>`:
- `Value::Length(l) → Stroke { paint: Color::BLACK,
  thickness: l.to_pt() }`.
- `Value::Color(c) → Stroke { paint: c, thickness: 1.0 }`
  (default thickness 1pt paridade vanilla).
- `Value::Stroke(s) → s.clone()`.
- Outros tipos rejeitados com erro hard.

### Decisão 3 — `Value::Stroke` enum variant novo

Audit C1 confirmará. **Hipótese provável**: `Value::Stroke`
NÃO existe pré-P227 (tipo `Stroke` é interno a
`FrameItem`). P227 adiciona `Value::Stroke(Stroke)` ao
enum Value (paridade `Value::Color(Color)`, `Value::Length(Length)`).

**Decisão fixada (sujeita a audit C1)**:
- Se `Value::Stroke` existe: usar directamente.
- Se NÃO existe (hipótese provável): criar **`Value::Stroke(Stroke)`**
  novo (mudança aditiva ao enum Value paridade `Value::Color`).
  Magnitude trivial mas requer arms cascata em ~3-5 sítios
  L1 (`PartialEq`, `type_name`, etc.).

### Decisão 4 — `native_stroke` constructor

Stdlib func nova `stroke(paint: ?, thickness: ?) -> Value::Stroke`
(paridade `native_rgb` constructor):

```rust
pub fn native_stroke(_ctx, args, ...) -> SourceResult<Value> {
    // 1. Extract paint (named opcional; default Color::BLACK).
    let paint = match args.named.get("paint") {
        Some(Value::Color(c)) => *c,
        Some(other) => return Err(/* "paint: espera Color, recebeu ..." */),
        None => Color::BLACK,
    };

    // 2. Extract thickness (named opcional; default 1.0 pt).
    let thickness = match args.named.get("thickness") {
        Some(val) => extract_length(val, "stroke", "thickness")?.to_pt(),
        None => 1.0,
    };

    // 3. Validar thickness > 0.
    if thickness <= 0.0 {
        return Err(/* "thickness deve ser > 0" */);
    }

    // 4. Reject extra positionals + named desconhecidos.
    if !args.items.is_empty() { return Err(...) }
    for key in args.named.keys() {
        if !["paint", "thickness"].contains(&key.as_str()) {
            return Err(...)
        }
    }

    Ok(Value::Stroke(Stroke { paint, thickness }))
}
```

Re-export + scope register paridade P218 pattern.

Stdlib funcs count: **59 → 60** (+1 native_stroke).

### Decisão 5 — Renderização Opção β simplificada

3 opções consideradas:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | Renderização real completa (deduplicação linhas adjacentes) | M (~1.5h); refino A.3 |
| **β** | Renderização real simplificada (sem deduplicação) | S+ (~1h); A.1 visível |
| γ | Armazenado mas semantic adiada (pattern N=5) | Trivial; viola intent A.1 cosméticos visíveis |

**Decisão fixada — Opção β**:
- A.1 é refino cosmético; visibilidade real é o ponto.
- Deduplicação adiada para A.3 candidato.
- Linhas adjacentes ficam duplicadas (não-óptimo mas
  visualmente correcto).

Implementação em `layout_grid`:
```rust
if let Some(stroke) = grid.stroke {
    // Para cada cell (row, col), emitir 4 FrameItem::Shape::Line
    // (top + bottom + left + right da cell).
    for placed in &placed_cells {
        let x0 = column_x[placed.col];
        let y0 = row_y[placed.row];
        let x1 = column_x[placed.col + placed.colspan];
        let y1 = row_y[placed.row + placed.rowspan];

        // Top, Bottom, Left, Right.
        frame.push_at(FrameItem::Shape {
            kind: ShapeKind::Line { dx: x1 - x0, dy: 0.0 },
            stroke: Some(stroke),
            fill: None,
            pos: Point::new(Pt(x0), Pt(y0)),
        });
        // ... bottom + left + right ...
    }
}
```

**Optimização Opção β**: linhas adjacentes ficam
duplicadas (top de cell (1,0) sobrepõe bottom de cell
(0,0)). Visualmente correcto; refino futuro A.3 dedupica.

### Decisão 6 — Table refino paralelo

Table P157A delegate via `layout_grid`. Table precisa
**field próprio** porque é variant Content independente:

**3 opções:**
- α — Table herda via delegate sem refino próprio. Viola
  paridade variant-rico.
- **β** — Table refino paralelo `stroke: Option<Stroke>`;
  `native_table` aceita `stroke:` shorthand idem
  `native_grid`. Magnitude S+ (paridade Grid).
- γ — Table refino completo (também align/inset). Fora
  escopo P227.

**Decisão fixada — Opção β** (refino paralelo Table).

Total variants Content refinados P227: **2** (Grid + Table).

### Decisão 7 — ADR-0080 promoção PROPOSTO → EM VIGOR

P227 valida pattern N=7 → 8. ADR-0080 §"Promoção" diz
"via N=8+ sem decisão contrária OU passo administrativo
XS dedicado".

**Decisão fixada — NÃO promover em P227**: criar critério
fundamentar promoção mas deixar para passo administrativo
XS separado pós-P227 (paridade política P158; minimaliza
overhead administrativo no passo de materialização).
P228 candidato natural: promoção ADR-0080 PROPOSTO → EM
VIGOR.

Reuso de dados (sem recolha nova):

- `Stroke` baseline P76 em `entities/geometry.rs`.
- `Color::BLACK` baseline P76 ou P25 (audit C1).
- `Value::Color` baseline P25.
- `Value::Length` baseline P25.
- `extract_length` helper N=9 cumulativo (P227 N=10 via
  thickness em `native_stroke`).
- `Content::Grid` baseline pós-P224 (8 fields).
- `Content::Table` baseline P157A (3 fields).
- `native_grid` em `stdlib/structural.rs` baseline P224.
- `native_table` em `stdlib/structural.rs` baseline P157A.
- ADR-0080 PROPOSTO Opção γ literal (L0 NÃO tocado).
- ADR-0079 PROPOSTO Categoria A.1 identificada.
- Pattern P223 refino aditivo a variant existente N=1 → 2
  (Place P223 + Grid+Table P227).

---

## §2 Cláusulas (12 — atomização interna)

### C1 — Inventário pré-P227: confirmar Stroke + Value variants + Grid/Table baseline

Auditoria empírica:

```
grep -n "pub struct Stroke" 01_core/src/entities/geometry.rs
grep -n "Value::Stroke\|Value::Color" 01_core/src/entities/value.rs
grep -n "pub const BLACK\|Color::BLACK" 01_core/src/entities/layout_types.rs
grep -A 10 "pub enum Content" 01_core/src/entities/content.rs | head -15
grep -n "Content::Grid\|Content::Table" 01_core/src/rules/layout/
grep -n "fn native_grid\|fn native_table" 01_core/src/rules/stdlib/structural.rs
```

Hipótese (per project_knowledge_search):
- `Stroke { paint: Color, thickness: f64 }` em `geometry.rs:?`.
- `Value::Color(Color)` existe; `Value::Length(Length)` existe.
- `Value::Stroke` **NÃO existe** (hipótese provável; confirmar).
- `Color::BLACK` provavelmente existe; audit.
- `Content::Grid` pós-P224: 8 fields.
- `Content::Table` baseline: 3 fields.
- `native_grid` em `stdlib/structural.rs` (P224 confirmou).
- `native_table` em `stdlib/structural.rs` (P157A).

Se signature ou estado divergir: registar `P227.div-N`.

**Decisão crítica C1**: se `Color::BLACK` NÃO existe,
criar constante (paridade `Color::rgb(0, 0, 0)` direct).

### C2 — Adicionar `Value::Stroke(Stroke)` enum variant (se necessário)

Se C1 confirma `Value::Stroke` NÃO existe:

Editar `01_core/src/entities/value.rs` adicionando variant:

```rust
pub enum Value {
    // ... existing ...
    Color(Color),
    Length(Length),
    /// P227 — Stroke variant para parametrizar borders Grid/Table
    /// e refinos cosméticos Fase 5 candidata Layout per ADR-0079
    /// PROPOSTO + ADR-0080 PROPOSTO (Opção γ literal).
    Stroke(Stroke),
    // ... existing ...
}
```

Arms cascata em `value.rs`:
- `type_name()` — `Self::Stroke(_) => "stroke"`.
- `PartialEq::eq` — comparação `Stroke` struct.
- `Clone` — derive auto (Stroke é Clone).
- `Debug` — derive auto.

Possíveis outros sítios:
- `cast_*` helpers (audit C1; provável necessidade).

Magnitude isolada C2: **XS (~15min)**.

### C3 — Adicionar `stroke: Option<Stroke>` a `Content::Grid`

Editar `01_core/src/entities/content.rs` variant Grid:

```rust
Grid {
    columns: Vec<TrackSizing>,
    rows: Vec<TrackSizing>,
    cells: Vec<Content>,
    gutter: Option<Length>,
    align: Option<Align2D>,
    inset: Sides<Length>,
    header: Option<Box<Content>>,
    footer: Option<Box<Content>>,
    /// P227 — stroke uniforme aplicado a todas cell borders.
    /// Default `None` (sem borders). Renderização Opção β
    /// simplificada (sem deduplicação linhas adjacentes; refino
    /// candidato A.3). Per ADR-0079 PROPOSTO Categoria A.1 +
    /// ADR-0080 PROPOSTO Opção γ literal (L0 não tocado).
    stroke: Option<Stroke>,
},
```

Grid fields: **8 → 9** (+stroke).

### C4 — Adicionar `stroke: Option<Stroke>` a `Content::Table`

Editar `01_core/src/entities/content.rs` variant Table:

```rust
Table {
    columns: Vec<TrackSizing>,
    rows: Vec<TrackSizing>,
    children: Vec<Content>,
    /// P227 — stroke uniforme paridade Grid; Table herda
    /// renderização via delegate `layout_grid` (precedente
    /// P157A).
    stroke: Option<Stroke>,
},
```

Table fields: **3 → 4** (+stroke).

### C5 — Arms cascata exhaustivos (compiler-driven)

Total arms refino Grid + Table P227:

**`entities/content.rs`** (5 arms × 2 variants = 10 arms):
- `is_empty` — proxy children/cells (preservado; stroke
  não afecta).
- `plain_text` — concatena (preservado).
- `PartialEq::eq` — comparação +1 field para cada (Grid
  9 fields; Table 4 fields).
- `map_content` — preserva `stroke` Copy (Option<Stroke>
  é Clone).
- `map_text` — idem.

**`rules/introspect.rs`** (2 arms × 2 = 4 arms):
- `materialize_time` — preserva stroke.
- `walk` — preserva.

**`rules/layout/mod.rs::layout_content`** (1 arm Grid +
1 arm Table — refino layout consume stroke).

**`rules/layout/grid.rs::layout_grid`** (signature ou
arm consume stroke param; refino actual).

**`rules/layout/mod.rs::measure_content_constrained`**
(2 arms — preservam dimensions; stroke não afecta layout
geometric pre-emit).

**`rules/introspect/locatable.rs`** (catch-all preserva).

Total: **~15+ arms cumulativos em ~4 ficheiros L1**
(compiler-driven; iterar até zero errors; possíveis
10-20 errors E0027/E0063 paridade P224).

### C6 — Helper `extract_stroke` + `native_stroke` constructor

**Helper `extract_stroke`** em `stdlib/layout.rs` (paridade
`extract_length` localização):

```rust
pub(super) fn extract_stroke(
    val: &Value,
    fn_name: &str,
    field: &str,
) -> SourceResult<Stroke> {
    match val {
        Value::Length(l) => {
            let thickness = l.to_pt();
            if thickness <= 0.0 {
                return Err(/* "thickness deve ser > 0" */);
            }
            Ok(Stroke { paint: Color::BLACK, thickness })
        }
        Value::Color(c) => Ok(Stroke { paint: *c, thickness: 1.0 }),
        Value::Stroke(s) => Ok(s.clone()),
        other => Err(/* "espera Length/Color/Stroke, recebeu ..." */),
    }
}
```

Magnitude isolada: XS (~15min).

**`native_stroke`** stdlib constructor em `stdlib/layout.rs`
(per Decisão 4 §1; ~30 LOC).

Re-export em `stdlib/mod.rs` + scope register em `eval/mod.rs`
paridade P218 pattern.

### C7 — Refino `native_grid` + `native_table` accept stroke

Editar `stdlib/structural.rs::native_grid`:

```rust
// Accept named args expandido: ["columns", "rows", "gutter",
// "align", "inset", "header", "footer", "stroke"].
let stroke = match args.named.get("stroke") {
    Some(val) => Some(extract_stroke(val, "grid", "stroke")?),
    None => None,
};
// ... existing ...
Ok(Value::Content(Content::Grid {
    columns, rows, cells, gutter, align, inset, header, footer,
    stroke,  // P227 +1
}))
```

Editar `stdlib/structural.rs::native_table` paridade.

### C8 — Renderização Opção β em `layout_grid`

Editar `01_core/src/rules/layout/grid.rs::layout_grid`
adicionando stroke param:

```rust
pub(super) fn layout_grid(
    layouter: &mut Layouter,
    columns: &[TrackSizing],
    rows: &[TrackSizing],
    cells: &[Content],
    _gutter: Option<&Length>,  // P224.A
    _align: Option<&Align2D>,   // P224.A
    _inset: &Sides<Length>,     // P224.A
    _header: Option<&Content>,  // P224.B
    _footer: Option<&Content>,  // P224.B
    stroke: Option<&Stroke>,    // P227 — NEW
) -> SourceResult<()> {
    // ... existing layout_grid lógica P82-84.6 + P224 ...

    // P227 — Opção β simplificada: emite 4 FrameItem::Shape::Line
    // per cell border (top + bottom + left + right).
    if let Some(stroke) = stroke {
        for cell_info in &placed_cells {  // ou iterator equivalente
            let (x0, y0, x1, y1) = cell_info.bounds();

            // Top edge.
            frame.emit_shape_line(
                Point::new(Pt(x0), Pt(y0)),
                ShapeKind::Line { dx: x1 - x0, dy: 0.0 },
                stroke.clone(),
            );
            // Bottom edge.
            frame.emit_shape_line(
                Point::new(Pt(x0), Pt(y1)),
                ShapeKind::Line { dx: x1 - x0, dy: 0.0 },
                stroke.clone(),
            );
            // Left edge.
            frame.emit_shape_line(
                Point::new(Pt(x0), Pt(y0)),
                ShapeKind::Line { dx: 0.0, dy: y1 - y0 },
                stroke.clone(),
            );
            // Right edge.
            frame.emit_shape_line(
                Point::new(Pt(x1), Pt(y0)),
                ShapeKind::Line { dx: 0.0, dy: y1 - y0 },
                stroke.clone(),
            );
        }
    }
}
```

**Layouter consume signature**: `layout_grid(... +stroke
param)` chamado por arms `Content::Grid` e `Content::Table`
em `layout_content`.

Magnitude C8: **S+ (~1h)** — maior parcela renderização.

### C9 — Sentinelas P227

Tests P227 (paridade P223 + P224 scopes):

**Unit content** (~4 tests):
- `p227_grid_variant_aceita_stroke` — instancia Grid
  com `stroke: Some(...)`.
- `p227_table_variant_aceita_stroke` — idem Table.
- `p227_grid_partial_eq_inclui_stroke` — eq compara 9
  fields.
- `p227_grid_map_content_preserva_stroke`.

**Unit Value::Stroke** (~3 tests; se C2 criou variant):
- `p227_value_stroke_existe`.
- `p227_value_stroke_type_name`.
- `p227_value_stroke_partial_eq`.

**Unit stdlib extract_stroke** (~5 tests):
- `p227_extract_stroke_length_aceita` — Length → Stroke
  black/Length.
- `p227_extract_stroke_color_aceita` — Color → Stroke
  Color/1pt.
- `p227_extract_stroke_stroke_aceita` — Stroke → Stroke
  identidade.
- `p227_extract_stroke_tipo_errado_rejeita`.
- `p227_extract_stroke_thickness_negativo_rejeita`.

**Unit stdlib native_stroke** (~5 tests):
- `p227_native_stroke_defaults_aceita`.
- `p227_native_stroke_paint_thickness_aceita`.
- `p227_native_stroke_thickness_negativo_rejeita`.
- `p227_native_stroke_paint_tipo_errado_rejeita`.
- `p227_native_stroke_named_desconhecido_rejeita`.

**Unit stdlib native_grid/table stroke** (~4 tests):
- `p227_native_grid_stroke_length_aceita`.
- `p227_native_grid_stroke_color_aceita`.
- `p227_native_grid_stroke_negativo_rejeita`.
- `p227_native_table_stroke_paridade_grid`.

**Layout E2E** (~3 tests):
- `p227_grid_stroke_renderiza_4_linhas_por_cell` —
  Grid 2x2 com stroke emite 16 FrameItem::Line (4×4
  cells).
- `p227_grid_sem_stroke_zero_linhas_extra` — Grid sem
  stroke não emite Lines (regressão baseline).
- `p227_table_stroke_delegate` — Table com stroke produz
  Lines paridade Grid.

Total tests P227: **~24 tests** (4+3+5+5+4+3).
Esperado pós-P227: **2039 + 24 = 2063 verdes**.

### C10 — L0 NÃO tocado (ADR-0080 N=7 → 8 validação)

**Decisão fixada — Opção γ literal**: L0 prompts
`00_nucleo/prompts/entities/content.md` e
`00_nucleo/prompts/entities/geometry.md` **NÃO actualizados**.

Justificação:
- P227 é refino aditivo a variants Content existentes
  (Grid + Table) + adição aditiva ao enum Value
  (Stroke variant) + helper aditivo + stdlib aditivo.
- Aplica-se directamente ADR-0080 PROPOSTO Opção γ
  literal: "refactor aditivo pós-M9c NÃO actualiza L0
  prompts por defeito".
- **Pattern N=7 → 8 atingido** — primeira aplicação real
  pós-formalização ADR-0080. Validação empírica do ADR.

Hash `entities/content.md` + `entities/geometry.md` + `rules/stdlib.md`
preservados (não tocados).

### C11 — Verificação tests workspace + lint

```
cargo test --workspace 2>&1 | tail -3
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério:
- 2039 verdes pré-P227 + 24 novos = **2063 verdes**.
- 0 violations preservadas.
- Hashes propagados em ~5-7 ficheiros L1
  (`content.rs`, `value.rs`, `grid.rs`, `mod.rs`,
  `stdlib/layout.rs`, `stdlib/structural.rs`,
  `eval/mod.rs`).
- L0 prompts não tocados — "Nothing to fix" em L0 layer.

**Risco regressão Grid+Table baseline**: P82-84.6 + P157A
+ P224 tests pre-existentes podem precisar adaptação se
Grid/Table fields cresceram (+1 cada). Hipótese N=2-5
adaptações intencionais (tests com construtor directo).

### C12 — Inventário 148 reclassificação P227 + ADR-0079 anotação

**§A.5 Layout linha 141 `grid(columns, ...)`**:
- Classificação: `implementado⁺ ⁵ ⁴⁵ ⁴⁶` → **`implementado⁺
  ⁵ ⁴⁵ ⁴⁶ ⁴⁷`** (nota cumulativa P227).
- Sem reclassificação categórica (já `implementado⁺`
  pós-P224 + P225).
- Footnote ⁴⁷ adicionada documentando A.1 materializado;
  stroke uniforme via Opção α + parsing Opção β + render
  Opção β; ADR-0080 N=7 → 8 validado; reclassificação
  qualitativa: scope-out cosmético stroke fechado.

**Tabela B.2 Content variants**: actualização cumulativa
P227 — Grid +1 field stroke; Table +1 field stroke.
Variants count Content: 59 preservado (sem variants
novos; só refinos aditivos).

**`Value` enum**: variants count audit empírico — provável
54 → 55 (+Stroke).

**Footnote ⁴⁷ P227 adicionada** (~70 linhas) documentando:
- A.1 materializado (primeiro Categoria A Fase 5).
- 6 decisões fixadas (Opção α field + Opção β parsing +
  Value::Stroke novo + native_stroke constructor + Opção
  β render + Table refino paralelo).
- Helper `extract_stroke` novo.
- `extract_length` reuso N=9 → 10.
- Pattern Smart→Option N=7 → 8.
- Pattern "L0 minimal para refactors" N=7 → **8** —
  primeira aplicação real pós-ADR-0080 PROPOSTO.
- Pattern "refino aditivo paralelo entre variants
  irmãos" (Grid+Table) N=1 inaugurado P227.

**ADR-0079 Fase 5 Layout PROPOSTO anotação Categoria A
1/5 cumulativa**: bloco `### P227 anotação — Categoria A
sub-passo 1 (stroke Grid + Table)` adicionado.

**Status ADR-0079 mantido PROPOSTO** (sub-passo 1/13-15;
promoção a IMPLEMENTADO continua diferida).

**Status ADR-0080 mantido PROPOSTO** (N=8 atingido mas
promoção a EM VIGOR diferida a passo administrativo XS
P228 candidato).

---

## §3 Output

1 ficheiro relatório:
`00_nucleo/materialization/typst-passo-227-relatorio.md`.

Estrutura (~6-8 KB) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Inventário pré-P227 (C1 audit Stroke + Value variants
  + Grid/Table baseline).
- §3 `Value::Stroke` variant + Grid/Table refino +1 field
  cada (C2-C4).
- §4 Helper `extract_stroke` + `native_stroke` constructor
  + `native_grid`/`native_table` accept stroke (C6+C7).
- §5 Renderização Opção β simplificada (C8).
- §6 Decisões substantivas (7 decisões fixadas) +
  validação ADR-0080 N=7 → 8.
- §7 Resultados verificação + inventário 148 +
  ADR-0079 anotação Categoria A 1/5 (C9+C11+C12).
- §8 Próximo sub-passo (P228 candidato: A.2 fill OU
  promoção ADR-0080 EM VIGOR).

Código alterado:
- **Editado**: `01_core/src/entities/content.rs` (variants
  Grid + Table refino + arms cascata + ~4 unit tests).
- **Editado**: `01_core/src/entities/value.rs` (`Value::Stroke`
  variant novo + arms cascata + ~3 unit tests).
- **Editado**: `01_core/src/rules/introspect.rs` (arms novos).
- **Editado**: `01_core/src/rules/layout/grid.rs` (signature
  layout_grid +stroke param + renderização Opção β).
- **Editado**: `01_core/src/rules/layout/mod.rs` (arms
  consume stroke).
- **Editado**: `01_core/src/rules/stdlib/layout.rs`
  (helper `extract_stroke` + `native_stroke` constructor
  + ~13 unit tests).
- **Editado**: `01_core/src/rules/stdlib/structural.rs`
  (`native_grid` + `native_table` accept stroke; +~4 unit
  tests).
- **Editado**: `01_core/src/rules/stdlib/mod.rs` (re-export).
- **Editado**: `01_core/src/rules/eval/mod.rs` (scope register
  native_stroke).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+~3 E2E
  tests).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (footnote ⁴⁷ P227 + Tabela B.2 actualização cumulativa).
- **Editado**: `00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`
  (+ anotação Categoria A 1/5 P227).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- Sides<Option<Stroke>> per-side — A.3 candidato Fase 5
  separado.
- Per-cell stroke override em GridCell — A.3 candidato.
- Closure stroke `(row, col) => stroke(...)` — fora de
  escopo Fase 5 (refactor profundo).
- Stroke fill (preenchimento área) — confunde com `fill`
  (A.2 candidato).
- Deduplicação linhas adjacentes — A.3 candidato refino.
- Block/Boxed `outset`/`radius`/`clip` — A.4 candidato.
- Promover ADR-0080 PROPOSTO → EM VIGOR — P228
  candidato administrativo separado.
- Promover ADR-0079 PROPOSTO → IMPLEMENTADO — só pós
  série α/β/γ/δ completa.
- Tocar em L0 prompts — Opção γ literal per ADR-0080
  N=7 → 8 validação.
- Show rules `#show grid: ...` — fora de escopo Fase 5.
- Reabrir decisões arquiteturais — A.1 é Categoria A
  (sem reabrir).

---

## §5 Riscos a evitar

1. **`Value::Stroke` redundante se já existe**: audit C1
   crítico antes de adicionar variant ao enum.
2. **`Color::BLACK` ausente forçar criação**: audit C1
   confirmar; alternativa `Color::rgb(0, 0, 0)` directo.
3. **Renderização Opção β com linhas adjacentes duplas**:
   visualmente correcto mas não-óptimo; documentar como
   refino A.3.
4. **`layout_grid` signature inflar**: já tem 8 fields
   pós-P224; P227 adiciona 9º (+stroke). Mitigação:
   aceitar — paridade Grid struct.
5. **Table refino paralelo divergir Grid sutilmente**:
   Table delegate via `layout_grid`; preservar paridade
   estrutural (apenas signature dos 2 stdlib funcs varia
   trivialmente).
6. **Tests pre-existentes Grid/Table construtor**: hipótese
   N=2-5 testes com `Content::Grid { columns, rows, cells,
   gutter, align, inset, header, footer }` precisam +1
   field (stroke: None). Adaptação intencional documentada.
7. **`extract_stroke` shorthand parsing incorrecto**:
   Color shorthand → Stroke { c, 1.0 } usa default 1pt;
   verificar paridade vanilla literal.
8. **L0 tentação de actualizar**: violar ADR-0080 PROPOSTO
   Opção γ literal exactamente quando deveria validar.
   Mitigação: §5 risco 8 explícito + §C10 fixa Opção γ.
9. **`Color::BLACK` default debatible**: vanilla usa
   `BLACK` para stroke sem paint. Decisão fixada — paridade
   vanilla literal.
10. **Promoção prematura ADR-0080 EM VIGOR**: tentação de
    "fechar pattern N=8 atingido". Decisão 7 fixa NÃO
    promover em P227; passo administrativo XS separado
    (P228 candidato).
11. **Stroke thickness == 0 aceito**: vanilla rejeita
    thickness <= 0; cristalino paridade. Validação em
    `extract_stroke` + `native_stroke`.
12. **Magnitude exceder M (~2-3h)**: P224 atomização
    interna A/B/C mostrou-se mais leve que estimado;
    P227 simpler (sem 3 variants novos). Hipótese real
    S+ a M.

---

## §6 Hipótese provável

C1 confirmará `Stroke` baseline + ausência `Value::Stroke`
+ `Color::BLACK` ou alternativa + Grid 8 fields + Table 3
fields + native_grid/table em `stdlib/structural.rs`.

C2 adicionará `Value::Stroke(Stroke)` variant + ~3 arms
cascata. Magnitude XS.

C3+C4 adicionarão `stroke: Option<Stroke>` a Grid e Table.

C5 cobrirá ~15+ arms cumulativos (compiler-driven; iterar
até zero errors).

C6 criará `extract_stroke` helper + `native_stroke`
constructor; reuso `extract_length` N=9 → 10.

C7 refinará `native_grid` + `native_table` accept stroke.

C8 implementará renderização Opção β (4 lines per cell).

C9 criará ~24 tests novos.

C10 fixará Opção γ literal (L0 NÃO tocado; valida
ADR-0080 N=7 → 8).

C11 reportará 2063 verdes; 0 violations; possíveis N=2-5
adaptações Grid/Table baseline.

C12 reclassificará `grid(...)` footnote ⁴⁷ + Tabela B.2
cumulativa + ADR-0079 anotação Categoria A 1/5.

Custo real: **M (~1.5-2h)** — abaixo limite superior por
reuso massivo P224 + P157A patterns.

Mas é hipótese, não decisão. C1-C12 fixam-se
empíricamente.

---

## §7 Particularidade P227

P227 é estruturalmente distinto na trajectória pós-M9c:

- **Primeiro sub-passo materialização Fase 5 Layout
  candidata** — abertura formal pós-ADR-0079 PROPOSTO
  P226. Paridade estrutural P217 (primeiro Fase 3) e
  P222 (primeiro Fase 4).
- **Valida ADR-0080 PROPOSTO N=7 → 8** — primeira
  aplicação real pós-formalização do pattern "L0 minimal
  para refactors aditivos". **Pattern atingido a N=8**
  cumulativo; **promoção ADR-0080 PROPOSTO → EM VIGOR**
  fica como P228 candidato natural (passo administrativo
  XS).
- **Primeira validação de pattern emergente formalizado
  via materialização real pós-M9c** — distinto de outros
  patterns (Field semantic adiada N=5; div-N N=3;
  encerramento Fase N=2) que não têm ADR meta formalizada
  ainda.
- **`Value::Stroke` enum variant novo** — primeira adição
  ao enum Value em série Layout pós-M9c. Distinto de
  P217 (Content variant) + P218 (stdlib) + P220 (agregado)
  + P222 (stdlib + visibility) + P223 (refino aditivo a
  variant) + P224 (variants + módulo).
- **Pattern emergente "refino aditivo paralelo entre
  variants irmãos" N=1 inaugurado P227** (Grid + Table
  recebem mesmo field paralelo; precedente futuro
  candidato para Fase 5 sub-passos cosméticos).
- **Categoria A 1/5 sub-passos materializado** — primeiro
  cosmético Fase 5; precedente para A.2 fill + A.3 per-cell
  + A.4 Block/Boxed + A.5 Place per-cell.
- **Cobertura Layout per metodologia preservada 89%
  real** — A.1 é refino qualitativo a entrada já
  `implementado⁺` (cobertura categórica inalterada;
  qualidade refinada).
- **Pattern "L0 minimal para refactors" N=7 → 8
  consolidado em P227** — primeira validação empírica
  pós-formalização ADR-0080.
- **Pattern Smart→Option N=7 → 8 consolidado**
  (`stroke: Option<Stroke>` paridade Smart→Option default
  None).
- **`extract_length` reuso N=9 → 10** — atinge patamar
  N=10 (helper público candidato fortemente justificado
  refino futuro candidato sub-passo administrativo XS
  separado).
- **Anti-inflação 19ª aplicação cumulativa** pós-P205D —
  Opção α field (não Sides per-side) + Opção β parsing
  (não Sides parser) + Opção γ L0 não tocado + sem
  helper construtor Rust novo.

Por isso §5 risco 8 (L0 tentação) é o mais provável.
Tentação óbvia é "primeira materialização real Fase 5;
documentar formalmente em L0". Defesa: ADR-0080 PROPOSTO
Opção γ literal exactamente para esta situação; N=7 → 8
**valida** a regra.

**Critério de aceitação P227**:
- ~24 tests novos verdes.
- 2039 tests pre-existentes preservados (após N=2-5
  adaptações intencionais Grid/Table baseline).
- 0 violations.
- Grid +1 field stroke; Table +1 field stroke.
- `Value::Stroke` variant novo (se C1 confirma ausência).
- Renderização Opção β funcional (4 lines per cell).
- ADR-0080 N=7 → **8 validado**.
- ADR-0079 Categoria A 1/5 anotado.
- Cobertura Layout 89% preservada (refino qualitativo).

**Estado pós-P227 esperado**:
- Tests workspace: 2039 → **2063 verdes** (+24).
- Stdlib funcs: 59 → **60** (+native_stroke).
- Content variants: 59 preservado (Grid + Table refinados).
- Value variants: 54 → **55** (+Stroke; se C1 confirma).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**
  (refino qualitativo; nota Grid+Table stroke materializado).
- Cobertura user-facing total: 67% preservada.
- ADR-0066 PROPOSTO; ADR-0061 IMPLEMENTADO; ADR-0078
  IMPLEMENTADO; ADR-0079 PROPOSTO; ADR-0080 PROPOSTO.
- Saldo DEBTs: 12 preservado.
- **19 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" N=8** —
  primeira validação real pós-ADR-0080 formalização;
  promoção EM VIGOR P228 candidato.
- **Pattern Smart→Option N=8** consolidado.
- **Pattern "refino aditivo paralelo entre variants
  irmãos" N=1** inaugurado P227.
- **Categoria A Fase 5 Layout**: 1/5 → próximos A.2 fill,
  A.3 per-cell, A.4 Block/Boxed, A.5 Place per-cell.
