# Passo 228 — A.2 `fill` Grid + Table inheritance (Fase 5 Layout candidata Categoria A 2/5; valida ADR-0080 N=8 → 9)

**Série**: 228 (décimo-quarto sub-passo Layout pós-M9c;
**segundo sub-passo materialização Fase 5 Layout candidata**
per ADR-0079 PROPOSTO; segundo sub-passo Categoria A
"cosméticos sem reabrir decisões"; **sub-passo paralelo
estructural a P227** A.1 stroke).
**Marco**: nenhum (décimo-sexto passo pós-M9c; **valida
pattern "refino aditivo paralelo entre variants irmãos"
N=1 → 2 cumulativo** P227+P228; **valida ADR-0080
PROPOSTO N=8 → 9** segunda aplicação real pós-formalização).
**Tipo**: refino aditivo a 2 variants existentes
(`Content::Grid` + `Content::Table`); 1 field novo a cada
variant + helper parsing trivial + renderização real
simplificada com Z-order correcto.
**Magnitude**: S+ a M (~1.5-2h; redução face a P227 por
ausência de Value variant novo + ausência de constructor
stdlib novo).
**Pré-condição**: P227 concluído (A.1 stroke Grid + Table
materializado; Value::Stroke variant novo; helper
extract_stroke + native_stroke constructor; renderização
Opção β simplificada; 2057 verdes; ADR-0080 N=7 → 8
validado real; ADR-0079 Categoria A 1/5); humano fixou
A.2 (decisão literal pós-P227 §8); `Color` baseline P25 em
`entities/layout_types.rs`; `Value::Color(Color)` existe
P25; `Content::Grid` baseline pós-P227 (9 fields); 
`Content::Table` baseline pós-P227 (4 fields); 
`native_grid` + `native_table` baseline pós-P227 com
`stroke:` named arg expansion.
**Output**: 1 ficheiro relatório curto + código alterado
em ~5-7 ficheiros L1 + L0 NÃO tocado (Opção γ literal
valida ADR-0080 N=8 → 9) + inventário 148 anotação
cumulativa Grid + Table (footnote ⁴⁸) + ADR-0079
anotação Categoria A 2/5.

---

## §1 Trabalho

`Content::Grid` pós-P227 tem 9 fields cumulativos
(columns/rows/cells/gutter/align/inset/header/footer/
stroke). **`fill` foi scope-out explícito em P224 + P226**
diagnóstico amplo (per ADR-0054 graded; cosméticos
não-estruturais). P227 fechou A.1 stroke; P228 fecha A.2
fill paralelo.

**P228 materializa A.2**:
- **Grid +1 field**: `fill: Option<Color>` (uniforme;
  paridade Smart→Option N=8 → 9 consolida).
- **Table +1 field**: `fill: Option<Color>` (paralelo Grid).
- **Helper trivial parsing** (extract_color OU inline
  match — audit C1 confirma se helper `extract_color` já
  existe).
- **Renderização real simplificada** em `layout_grid`:
  emite `FrameItem::Shape::Rect` per cell **antes** do
  conteúdo cell (Z-order correcto: fill → conteúdo →
  stroke).
- **Sem constructor stdlib novo** (anti-inflação;
  utilizador usa `fill: red` ou `fill: rgb(...)`).

**Decisão arquitectural central — 6 decisões fixadas**:

### Decisão 1 — Field tipo Opção α (`Option<Color>` uniforme)

Vanilla `fill` aceita `Paint` (enum `Color | Gradient |
Tiling`). Cristalino não tem `Paint` (per ADR-0029
enumeração tipos vanilla — gradients/tilings scope-out).

3 opções consideradas:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | `fill: Option<Color>` directo | Subset minimal; paridade simplificada cristalina; pattern Smart→Option N=8 → 9 |
| β | `fill: Option<Paint>` (criar Paint enum) | Inflação ADR-0029; gradients/tilings fora escopo |
| γ | `fill: Sides<Option<Color>>` per-side | Conceptualmente errado (fill é área; não tem "sides") |

**Decisão fixada — Opção α** porque:
- ADR-0029 enumera `Paint` como tipo vanilla scope-out
  cristalino.
- `Color` já existe baseline P25.
- Paridade observable preservada (fill cor sólida é forma
  predominante vanilla).
- API surface minimal.

### Decisão 2 — Stdlib parsing Opção α (Color directo)

Stroke (P227) aceitou 3 shorthands (Length/Color/Stroke).
Fill conceptualmente só faz sentido como Color (não há
Length-shorthand semântico para fill).

3 opções consideradas:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | Apenas `Value::Color` | Semantic puro; trivial |
| β | Color OR Stroke.paint (extract) | Inversão semantic-confusa |
| γ | Color OR "none" string OR Color | Já coberto por Opção α + default None |

**Decisão fixada — Opção α**:
- Parsing trivial inline ou helper `extract_color` (audit
  C1).
- Sem shorthand Length (não tem sentido semântico).
- Sem constructor stdlib novo (Color tem `native_rgb` +
  `native_luma` baseline P25).

### Decisão 3 — Constructor stdlib novo Opção γ (NÃO criar)

Anti-inflação aplicada:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | Criar `native_fill(color: ?)` constructor | Inflacionário; Color tem constructors P25 |
| β | Criar `native_paint(...)` paridade vanilla | Inflacionário; Paint scope-out |
| **γ** | NÃO criar constructor; utilizador usa `fill: red` ou `fill: rgb(...)` | Anti-inflação; paridade vanilla UX |

**Decisão fixada — Opção γ**: anti-inflação. Distinto de
P227 que criou `native_stroke` porque Stroke é tipo
composto (paint+thickness). Color é primitivo.

### Decisão 4 — Renderização Opção β Z-order correcto

Z-order paridade vanilla:
1. **Fill** emite primeiro (atrás do conteúdo).
2. **Conteúdo cell** emite a seguir.
3. **Stroke** emite por último (à frente do conteúdo).

3 opções consideradas para renderização Rect:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| α | Fill emitido completo per cell (Rect cobre bounds) | Visual correcto; magnitude trivial |
| **β** | Fill simplificado Rect per cell + audit P227 stroke Z-order | A.1 + A.2 combined em loop único; magnitude S+ |
| γ | Fill armazenado mas semantic adiada | Viola intent A.2 cosméticos visíveis |

**Decisão fixada — Opção β** (Z-order correcto):
- Fill emite `FrameItem::Shape::Rect` per cell antes do
  conteúdo cell.
- **Audit C1 confirma onde P227 stroke emitiu**: se foi
  antes do conteúdo (Z-order errado), refactor P227+P228
  combined para garantir fill → conteúdo → stroke.
- Decisão fixada: **se audit C1 revelar Z-order incorrecto
  em P227, registar `P228.div-N`** e adaptar P227 stroke
  emission ordem paralelamente.

### Decisão 5 — Tests E2E novos para Z-order

Stroke + fill simultâneos exigem teste de Z-order (fill
atrás; stroke à frente). **1-2 tests E2E adicionais Z-order**
para validar interacção:
- `p228_grid_fill_emitido_antes_de_conteudo` — Rect Z-order.
- `p228_grid_fill_e_stroke_simultaneos_z_order_correcto` —
  fill antes; stroke depois.

### Decisão 6 — L0 NÃO tocado (Opção γ literal valida ADR-0080 N=8 → 9)

**Fixada — Opção γ literal**: L0 prompts NÃO actualizados.
Pattern "L0 minimal para refactors" N=8 → **9** consolida.

P228 é **segunda aplicação real pós-formalização ADR-0080
PROPOSTO** (primeira foi P227 N=7 → 8). N=9 cumulativo
patamar empírico **fortemente sólido**.

**Promoção ADR-0080 PROPOSTO → EM VIGOR** continua a ser
candidato P229 administrativo XS — N=9 ultrapassa o N=8
de "via N=8+ sem decisão contrária" §"Promoção". Decisão
fixada aqui: NÃO promover em P228; preservar política
P158 minimalista; P229 candidato administrativo dedicado.

### Decisão 7 — ADR-0079 anotação Categoria A 2/5

Sem promoção PROPOSTO → IMPLEMENTADO (categoria A pendente
3/5 sub-passos restantes: A.3 per-cell + A.4 Block/Boxed
+ A.5 Place per-cell).

Reuso de dados (sem recolha nova):

- `Color` baseline P25 em `entities/layout_types.rs`.
- `Value::Color(Color)` existe P25.
- `FrameItem::Shape` + `ShapeKind::Rect` baseline P76
  geometry.
- `Content::Grid` baseline pós-P227 (9 fields incl. stroke).
- `Content::Table` baseline pós-P227 (4 fields incl. stroke).
- `native_grid` em `stdlib/structural.rs` pós-P227 com
  stroke named arg.
- `native_table` em `stdlib/structural.rs` pós-P227 com
  stroke named arg.
- `extract_color` helper (audit C1; provável existência
  como helper internal).
- Pattern "refino aditivo paralelo entre variants irmãos"
  N=1 P227 (Grid + Table; reusável directo).
- Pattern "L0 minimal para refactors" N=8 P227 (validar
  N=9 P228).
- ADR-0080 PROPOSTO Opção γ literal (validação cumulativa).
- ADR-0079 PROPOSTO Categoria A 1/5 P227.

---

## §2 Cláusulas (12 — atomização paridade P227)

### C1 — Inventário pré-P228: confirmar Color + helpers + Z-order P227

Auditoria empírica crítica:

```
grep -n "pub fn extract_color\|fn extract_color" 01_core/src/rules/stdlib/
grep -n "ShapeKind::Rect" 01_core/src/entities/geometry.rs
grep -A 10 "Value::Color" 01_core/src/entities/value.rs | head -15
grep -A 30 "if let Some(s) = stroke" 01_core/src/rules/layout/grid.rs
grep -n "fn layout_grid" 01_core/src/rules/layout/grid.rs
```

Hipótese:
- **`extract_color`** helper provavelmente existe (audit
  empírico crítico; usado por `native_rgb` parsing P25).
  Se NÃO existir: parsing inline trivial `Value::Color(c)
  => *c, _ => Err(...)`.
- **`ShapeKind::Rect`** existe baseline P76.
- **`Value::Color(Color)`** existe P25 ✓.
- **P227 stroke Z-order**: audit empírico onde emitido no
  `layout_grid` — **se antes do conteúdo cell**, Z-order
  errado (stroke deveria estar à frente). **Se depois do
  conteúdo**, Z-order correcto.

**Decisão crítica C1**: se audit confirma P227 stroke
emitido **antes do conteúdo** (Z-order errado):
- Registar `P228.div-N` formal.
- Adaptar P227 stroke renderização paralelamente em P228
  (mover emission após cell content loop).
- Pattern "fecho de divergência detectada pós-materialização
  via sub-passo subsequente" N=1 inaugurado.

Se Z-order P227 está correcto (stroke após conteúdo):
- Fill emite antes conteúdo (Opção β).
- Sem `P228.div-N`; sem adaptação P227.

### C2 — Adicionar `fill: Option<Color>` a `Content::Grid`

Editar `01_core/src/entities/content.rs` variant Grid:

```rust
Grid {
    columns, rows, cells,
    gutter, align, inset, header, footer,  // P224 baseline
    stroke,                                  // P227 (A.1)
    /// P228 — fill uniforme aplicado a todas cells (atrás
    /// do conteúdo; antes do stroke). Default `None` (sem
    /// fill). Per ADR-0079 PROPOSTO Categoria A.2 +
    /// ADR-0080 PROPOSTO Opção γ literal validation N=8 → 9.
    fill: Option<Color>,
},
```

Grid fields: **9 → 10** (+fill).

### C3 — Adicionar `fill: Option<Color>` a `Content::Table`

Editar `01_core/src/entities/content.rs` variant Table:

```rust
Table {
    columns, rows, children,                 // P157A baseline
    stroke,                                  // P227 (A.1)
    /// P228 — fill paridade Grid; Table herda renderização
    /// via delegate `layout_grid`.
    fill: Option<Color>,
},
```

Table fields: **4 → 5** (+fill).

### C4 — Arms cascata exhaustivos (compiler-driven)

Total arms refino Grid + Table P228:

**`entities/content.rs`** (5 arms × 2 variants = 10 arms):
- `is_empty` — proxy (preservado; fill não afecta).
- `plain_text` — concatena (preservado).
- `PartialEq::eq` — comparação +1 field cada (Grid 10
  fields; Table 5 fields).
- `map_content` — preserva `fill` Copy (Option<Color>
  é Copy via Color Copy P25).
- `map_text` — idem.

**`rules/introspect.rs`** (2 arms × 2 = 4 arms):
- `materialize_time` — preserva fill.
- `walk` — preserva.

**`rules/layout/mod.rs::layout_content`** (1 arm Grid +
1 arm Table — refino consume fill).

**`rules/layout/grid.rs::layout_grid`** (signature +1
param `fill: Option<&Color>`).

**`rules/introspect/locatable.rs`** (catch-all preserva).

**`rules/layout/mod.rs::measure_content_constrained`** (2
arms — preservam dimensions; fill não afecta layout
geometric pre-emit).

Total: **~12+ arms cumulativos** (menor que P227 por
ausência de Value variant novo). Compiler-driven; iterar
até zero errors (possíveis 6-12 errors E0027/E0063).

### C5 — Refino `native_grid` + `native_table` accept fill

Editar `stdlib/structural.rs::native_grid`:

```rust
// Accept named args expandido: [..., "stroke", "fill"].
let fill = match args.named.get("fill") {
    Some(Value::Color(c)) => Some(*c),
    Some(other) => return Err(/* "fill: espera Color, recebeu ..." */),
    None => None,
};
// ... existing ...
Ok(Value::Content(Content::Grid {
    columns, rows, cells, gutter, align, inset, header, footer,
    stroke, fill,  // P228 +1
}))
```

Editar `stdlib/structural.rs::native_table` paridade.

Magnitude C5: **XS (~15min)** — paridade P227 estructural
mas com parsing trivial (apenas Color match).

### C6 — Renderização Opção β em `layout_grid` (Z-order
correcto)

Editar `01_core/src/rules/layout/grid.rs::layout_grid`
adicionando fill param **+ refactor Z-order se C1 audit
revelar incorrecto em P227**:

```rust
pub(super) fn layout_grid(
    layouter: &mut Layouter,
    columns: &[TrackSizing],
    rows: &[TrackSizing],
    cells: &[Content],
    _gutter: Option<&Length>,
    _align: Option<&Align2D>,
    _inset: &Sides<Length>,
    _header: Option<&Content>,
    _footer: Option<&Content>,
    stroke: Option<&Stroke>,       // P227
    fill: Option<&Color>,           // P228 — NEW
) -> SourceResult<()> {
    // ... existing layout pre-cell processing ...

    for placed in &placed_cells {
        let (x0, y0, x1, y1) = placed.bounds();

        // P228 — Z-order step 1: fill emite primeiro (atrás
        // do conteúdo cell).
        if let Some(c) = fill {
            layouter.regions.current.current_items.push(
                FrameItem::Shape {
                    pos: Point { x: Pt(x0), y: Pt(y0) },
                    kind: ShapeKind::Rect,
                    width: x1 - x0,
                    height: y1 - y0,
                    fill: Some(*c),
                    stroke: None,
                }
            );
        }

        // Z-order step 2: conteúdo cell (lógica P82-84.6
        // existing preservada).
        // [...lógica existing emit cell content...]

        // P227 — Z-order step 3: stroke emite por último
        // (à frente do conteúdo).
        if let Some(s) = stroke {
            // [...4 lines per cell border existing P227...]
        }
    }
}
```

**Crítico**: se C1 audit revelar P227 stroke emitido
**antes do conteúdo** (Z-order errado em P227 original),
P228 inclui refactor P227 paralelo para Z-order correcto.

Magnitude C6: **S+ (~1h)** — maior parcela renderização +
audit Z-order.

### C7 — Layouter consumer

Editar arms `Content::Grid` e `Content::Table` em
`layout/mod.rs::layout_content` para passar `fill.as_ref()`:

```rust
Content::Grid { ..., stroke, fill, .. } => {
    layout_grid(layouter, ..., stroke.as_ref(), fill.as_ref())?;
}
Content::Table { ..., stroke, fill, .. } => {
    layout_grid(layouter, ..., stroke.as_ref(), fill.as_ref())?;
}
```

Magnitude C7: **XS (~10min)**.

### C8 — Sentinelas P228

Tests P228 (paridade P227 estrutura mas reduzido):

**Unit content** (~4 tests):
- `p228_grid_variant_aceita_fill` — instancia Grid com
  `fill: Some(Color::rgb(255, 0, 0))`.
- `p228_table_variant_aceita_fill` — idem Table.
- `p228_grid_partial_eq_inclui_fill` — eq compara 10
  fields.
- `p228_grid_map_content_preserva_fill`.

**Unit stdlib native_grid/table fill** (~5 tests):
- `p228_native_grid_fill_color_aceita`.
- `p228_native_grid_fill_none_default`.
- `p228_native_grid_fill_tipo_errado_rejeita` —
  `fill: 1pt` (Length) rejeitado com erro hard.
- `p228_native_table_fill_paridade_grid`.
- `p228_native_grid_fill_e_stroke_simultaneos_aceita`.

**Layout E2E** (~5 tests; Z-order foco):
- `p228_grid_fill_renderiza_rect_por_cell` — Grid 2x2 com
  fill emite 4 FrameItem::Shape::Rect.
- `p228_grid_sem_fill_zero_rects_extra` — Grid sem fill não
  emite Rect (regressão baseline).
- `p228_grid_fill_emitido_antes_de_conteudo_z_order` —
  Z-order: Rect index < text index.
- `p228_grid_fill_e_stroke_z_order_correcto` — fill antes;
  stroke depois.
- `p228_table_fill_delegate` — Table com fill produz Rect
  paridade Grid.

Total tests P228: **~14 tests** (4+5+5). Menor que P227
(~18) por ausência de Value variant + helper + constructor
+ menos shorthands parsing.

Esperado pós-P228: **2057 + 14 = 2071 verdes**.

### C9 — L0 NÃO tocado (Opção γ literal valida ADR-0080
N=8 → 9)

**Decisão fixada — Opção γ literal**: L0 prompts
`entities/content.md` + `entities/value.md` +
`entities/layout_types.md` (onde Color está documentada)
+ `rules/stdlib.md` **NÃO actualizados**.

Justificação:
- P228 é refino aditivo a variants Content existentes
  (Grid + Table) sem adição ao enum Value (Color já existe).
- Aplica-se ADR-0080 PROPOSTO Opção γ literal.
- **Pattern N=8 → 9 atingido** — segunda aplicação real
  pós-formalização ADR-0080. **Promoção EM VIGOR
  fortemente justificada** (N=9 ultrapassa critério N=8+).

Hash L0 prompts preservados.

### C10 — Verificação tests workspace + lint

```
cargo test --workspace 2>&1 | tail -3
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério:
- 2057 verdes pré-P228 + ~14 novos = **~2071 verdes**.
- 0 violations preservadas.
- Hashes propagados em ~5-7 ficheiros L1 (`content.rs`,
  `grid.rs`, `mod.rs`, `stdlib/structural.rs`, possíveis
  outros).
- L0 prompts não tocados — "Nothing to fix".

**Risco regressão Grid+Table baseline**: P82-84.6 + P157A
+ P224 + P227 tests pre-existentes podem precisar
adaptação se Grid/Table fields cresceram (+1 cada).
Hipótese N=2-5 adaptações intencionais (paridade P227
N=4 adaptações; possíveis tests com construtor directo
`Grid { ..., stroke, ... }` agora precisam +fill).

### C11 — Inventário 148 reclassificação P228

**§A.5 Layout linha 141 `grid(columns, ...)`**:
- Classificação: `implementado⁺ ⁵ ⁴⁵ ⁴⁶ ⁴⁷` → **`implementado⁺
  ⁵ ⁴⁵ ⁴⁶ ⁴⁷ ⁴⁸`**.
- Sem reclassificação categórica (já `implementado⁺`).
- Footnote ⁴⁸ adicionada documentando A.2 materializado;
  fill uniforme via Opção α + parsing Opção α (Color
  directo) + render Opção β Z-order correcto.

**Tabela B.2 Content variants**: Grid +1 field fill;
Table +1 field fill. Variants count Content: 59 preservado.

**Footnote ⁴⁸ P228 adicionada** (~70 linhas) documentando:
- A.2 materializado (segundo Categoria A Fase 5).
- 6 decisões fixadas (Opção α field + Opção α parsing +
  Opção γ sem constructor + Opção β Z-order + Opção γ L0
  + ADR-0079 sem promoção).
- Sem novos variants Value (Color já existe P25; distinto
  de P227).
- Sem novo constructor stdlib (anti-inflação; Color tem
  native_rgb/luma).
- Pattern Smart→Option N=8 → 9.
- Pattern "L0 minimal para refactors" N=8 → **9** —
  segunda aplicação real pós-ADR-0080 formalização.
- Pattern "refino aditivo paralelo entre variants irmãos"
  N=1 → 2 (P227 stroke + P228 fill).
- Sem promoção ADR-0080 PROPOSTO → EM VIGOR (P229
  candidato XS).
- Possível `P228.div-N` (Z-order P227 audit).

### C12 — ADR-0079 anotação Categoria A 2/5

Editar `00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`:

Bloco `### P228 anotação — Categoria A sub-passo 2 (fill
Grid + Table)` adicionado após bloco P227:

```markdown
**Categoria A**: 2/5 sub-passos materializados ✓.
- A.1 stroke (P227) ✓.
- **A.2 fill (P228) ✓**.
- A.3 stroke/fill GridCell per-cell — pendente.
- A.4 Block/Boxed outset/radius/clip — pendente.
- A.5 Place per-cell alignment override — pendente.

Trabalho P228:
- Grid +1 field fill: Option<Color> (9 → 10 fields).
- Table +1 field fill paralelo (4 → 5 fields).
- Sem novos Value variants (Color baseline P25 reusado).
- Sem novo constructor stdlib (anti-inflação).
- Renderização Opção β Z-order correcto (fill → conteúdo
  → stroke; possível adaptação P227 Z-order paralela se
  audit C1 revelou incorrecto).
- 14 tests novos verdes.
- Patterns N consolidados: L0 minimal N=9; Smart→Option
  N=9; refino paralelo variants irmãos N=2.

Status ADR-0079 mantido PROPOSTO (2/13-15 sub-passos
materializados).
```

**Status ADR-0079 mantido PROPOSTO**. Promoção a
IMPLEMENTADO continua diferida até todos sub-passos
materializados OU scope-out parcial formal humano.

**Status ADR-0080 mantido PROPOSTO** (N=9 atingido;
promoção EM VIGOR fortemente justificada P229 candidato
administrativo XS).

---

## §3 Output

1 ficheiro relatório:
`00_nucleo/materialization/typst-passo-228-relatorio.md`.

Estrutura (~5-7 KB; magnitude S+ a M justifica menor que
P227) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Inventário pré-P228 + audit Z-order P227 (C1).
- §3 Grid/Table refino +1 field cada (C2+C3).
- §4 `native_grid`/`native_table` accept fill (C5).
- §5 Renderização Opção β Z-order correcto (C6 + possível
  `P228.div-N` adaptação P227).
- §6 Decisões substantivas (7 decisões fixadas) +
  validação ADR-0080 N=8 → 9.
- §7 Resultados verificação + inventário 148 footnote ⁴⁸
  + ADR-0079 anotação Categoria A 2/5 (C8+C10+C11+C12).
- §8 Próximo sub-passo (P229 candidatos: promoção ADR-0080
  EM VIGOR OU A.3 per-cell OU pivot).

Código alterado:
- **Editado**: `01_core/src/entities/content.rs` (variants
  Grid + Table refino +1 field + arms cascata + ~4 unit
  tests).
- **Editado**: `01_core/src/rules/introspect.rs` (arms
  preservados; possível ajuste trivial).
- **Editado**: `01_core/src/rules/layout/grid.rs`
  (signature `layout_grid` +1 param + renderização Opção
  β Z-order + possível adaptação P227 stroke ordering).
- **Editado**: `01_core/src/rules/layout/mod.rs` (arms
  consume fill).
- **Editado**: `01_core/src/rules/stdlib/structural.rs`
  (`native_grid` + `native_table` accept fill; +~5 unit
  tests).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+~5
  E2E tests Z-order).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (footnote ⁴⁸ P228 + Tabela B.2 actualização cumulativa).
- **Editado**: `00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`
  (+ anotação Categoria A 2/5 P228).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- Per-cell fill em GridCell — A.3 candidato Fase 5
  separado.
- Gradients/patterns como fill — Paint cristalino
  scope-out per ADR-0029.
- `native_fill` constructor stdlib — anti-inflação.
- `Value::Fill` enum variant — Color baseline existe.
- Closure fill `(row, col) => color` — fora de escopo
  Fase 5.
- Promover ADR-0080 PROPOSTO → EM VIGOR — P229 candidato
  administrativo separado (N=9 atingido fortemente).
- Promover ADR-0079 PROPOSTO → IMPLEMENTADO — só pós
  Categoria A 5/5 + B + C + D completas.
- Tocar em L0 prompts — Opção γ literal valida ADR-0080
  N=8 → 9.
- Show rules `#show grid: ...` — fora de escopo Fase 5.
- Reabrir decisões arquiteturais — A.2 é Categoria A
  (sem reabrir).
- Alterar P227 stroke renderização SE Z-order já estiver
  correcto — paridade preservada literal.

---

## §5 Riscos a evitar

1. **`extract_color` helper ausente forçar inline parsing**:
   audit C1 confirma; alternativa inline trivial é
   aceitável.
2. **Z-order P227 stroke errado revelar via audit C1**:
   se P227 emitiu stroke antes do conteúdo, P228 precisa
   adaptar P227 paralelamente. Mitigação: `P228.div-N`
   formal + audit explícito + decisão fixada de adaptar.
3. **`Value::Color` parsing recusa Length/Stroke**:
   intent é só Color; rejeitar Length é correcto
   semantically (fill não é dimensão).
4. **Tests pre-existentes Grid/Table construtor**: hipótese
   N=2-5 testes pos-P227 com construtor directo precisam
   +1 field (fill: None). Adaptação intencional documentada.
5. **L0 tentação de actualizar**: violar ADR-0080 PROPOSTO
   Opção γ literal exactamente quando deveria validar
   (N=9 segunda aplicação real). Mitigação: §5 risco 5
   explícito + §C9 fixa Opção γ.
6. **Promoção prematura ADR-0080 EM VIGOR em P228**:
   tentação por N=9 atingido. Decisão 6 fixa NÃO promover
   em P228; P229 administrativo XS candidato.
7. **Magnitude exceder S+ a M (~1.5-2h)**: P227 chegou em
   ~1.5h; P228 mais simples (sem Value variant + sem
   constructor + parsing trivial). Hipótese real
   S+ (~1.2h).
8. **Z-order test E2E confundir índices**: `FrameItem`
   index na lista deve ser monotonic; fill emite antes;
   stroke depois. Test verifica ordem via index inspection.
9. **Table delegate produz Z-order errado**: Table delega
   `layout_grid`; mesmo Z-order automaticamente. Verificar
   E2E test específico.
10. **`Color::Copy` ausência forçar Clone**: audit C1
    confirma Copy; alternativa `*c` no map_content.
11. **`extract_color` se promovido a helper público
    contradiz política**: helper privado é suficiente
    P228; promoção pública candidato refino XS futuro
    (paridade `extract_length` N=10).
12. **Reabrir ADR-0029 Paint scope-out**: tentação por
    "fill: gradient" parecer útil. Rejeitada — gradients
    scope-out arquitectural; Fase 5 cosméticos é Color
    apenas.

---

## §6 Hipótese provável

C1 confirmará `Color::rgb` constructor baseline; possível
`extract_color` helper interno; `ShapeKind::Rect` baseline
P76; P227 stroke Z-order **provavelmente correcto** (após
conteúdo) — mas audit empírico é crítico.

C2+C3 adicionarão `fill: Option<Color>` a Grid e Table.

C4 cobrirá ~12 arms cumulativos (menor que P227).

C5 refinará `native_grid` + `native_table` accept fill
trivial.

C6 implementará renderização Opção β Z-order (fill Rect
antes; conteúdo; stroke depois). Possível `P228.div-N` se
P227 Z-order incorrecto.

C7 actualizará Layouter consumer arms.

C8 criará ~14 tests novos.

C9 fixará Opção γ literal (L0 NÃO tocado; valida ADR-0080
N=8 → 9).

C10 reportará ~2071 verdes; 0 violations; possíveis N=2-5
adaptações Grid/Table baseline.

C11 reclassificará footnote ⁴⁸; Tabela B.2 cumulativa.

C12 anotará ADR-0079 Categoria A 2/5.

Custo real: **S+ (~1.2-1.5h)** — abaixo limite por
ausência de Value variant + ausência de constructor + 
parsing trivial.

Mas é hipótese, não decisão. C1-C12 fixam-se
empíricamente.

---

## §7 Particularidade P228

P228 é estruturalmente distinto na trajectória pós-M9c:

- **Segundo sub-passo materialização Fase 5 Layout
  candidata** — sub-passo paralelo P227. Pattern emergente
  "refino aditivo paralelo entre variants irmãos" N=1 →
  **2 cumulativo** (P227 stroke; P228 fill).
- **Segunda aplicação real pós-formalização ADR-0080**
  — primeira foi P227 N=7 → 8; P228 valida N=8 → **9**.
  N=9 patamar empírico **muito sólido**.
- **Reduzida magnitude face a P227** — ausência de:
  - `Value::Stroke` equivalente (Color existe P25).
  - `extract_stroke` complexidade (parsing trivial).
  - `native_stroke` constructor (anti-inflação).
  - Length/Stroke shorthands (apenas Color directo).
- **Pattern "anti-inflação por aproveitamento de tipos
  existentes" N=1 inaugurado P228** — distinto de A.1
  P227 que precisou criar tipo composto.
- **Z-order considerations explícitas** — primeiro
  sub-passo Fase 5 que aborda Z-order multi-shape per
  cell (fill atrás; conteúdo; stroke à frente). Possível
  `P228.div-N` reactivo a audit P227 stroke.
- **Cobertura Layout per metodologia preservada 89%
  real** — A.2 é refino qualitativo a entrada já
  `implementado⁺` (cobertura categórica inalterada;
  qualidade refinada).
- **`extract_length` reuso N=10 estável** (helper público
  candidato preservado; P228 não usa).
- **Anti-inflação 20ª aplicação cumulativa** pós-P205D —
  Opção α field uniforme + Opção α parsing trivial +
  Opção γ sem constructor + Opção γ L0 não tocado + sem
  helper construtor Rust novo + ADR-0080 não promover em
  P228.
- **Decisão 3 Opção γ (NÃO criar constructor) inaugura
  precedente para refinos cosméticos simples** — Color é
  primitivo; constructors existem; anti-inflação aplicada.
  Distinto de P227 onde Stroke composto justificou
  `native_stroke`.

Por isso §5 risco 2 (Z-order P227 errado) é o mais
provável. Audit empírico C1 crítico. Defesa: pattern
"divergência factual detectada via sub-passo subsequente"
N=1 inaugurado se necessário; honestidade arquitectural
preservada.

**Critério de aceitação P228**:
- ~14 tests novos verdes.
- 2057 tests pre-existentes preservados (após N=2-5
  adaptações intencionais Grid/Table baseline).
- 0 violations.
- Grid +1 field fill; Table +1 field fill.
- Sem `Value::Fill` variant novo (Color reusado).
- Sem `native_fill` constructor (anti-inflação).
- Renderização Opção β Z-order funcional (fill antes;
  stroke depois).
- ADR-0080 N=8 → **9 validado**.
- ADR-0079 Categoria A 2/5 anotado.
- Cobertura Layout 89% preservada (refino qualitativo).
- Possível adaptação P227 Z-order via `P228.div-N` se
  necessário.

**Estado pós-P228 esperado**:
- Tests workspace: 2057 → **~2071 verdes** (+14).
- Stdlib funcs: 60 preservado (sem nova).
- Content variants: 59 preservado.
- Value variants: 55 preservado.
- Grid fields: 9 → **10** (+fill).
- Table fields: 4 → **5** (+fill).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada (refino
  qualitativo; cobertura Layout 89% preservada).
- Cobertura user-facing total: 67% preservada.
- ADR-0066 PROPOSTO; ADR-0061 IMPLEMENTADO; ADR-0078
  IMPLEMENTADO; ADR-0079 PROPOSTO; ADR-0080 PROPOSTO.
- Saldo DEBTs: 12 preservado.
- **20 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" N=8 → 9 validado**
  — segunda aplicação real pós-ADR-0080 formalização;
  promoção EM VIGOR P229 candidato muito sólido.
- **Pattern Smart→Option N=8 → 9 consolidado**.
- **Pattern "refino aditivo paralelo entre variants
  irmãos" N=1 → 2 consolidado** (Grid + Table cumulativos
  P227+P228; reusável A.3+).
- **Pattern "anti-inflação por aproveitamento de tipos
  existentes" N=1 inaugurado P228**.
- **Categoria A Fase 5 Layout**: 2/5 → próximos A.3
  per-cell, A.4 Block/Boxed, A.5 Place per-cell.
- **Fase 5 Layout candidata**: 2/13-15 sub-passos
  materializados (P227 A.1 ✓; P228 A.2 ✓; restantes
  pendentes).
