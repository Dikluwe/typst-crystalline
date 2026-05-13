# Relatório do passo P227 — A.1 `stroke` Grid + Table inheritance (Fase 5 Categoria A 1/5; valida ADR-0080 N=7 → 8)

**Data**: 2026-05-13.
**Spec**: `00_nucleo/materialization/typst-passo-227.md`.
**Tipo**: refino aditivo a 2 variants existentes (Grid + Table) +
1 enum variant novo (Value::Stroke) + helper + stdlib constructor +
renderização real Opção β.
**Magnitude planeada**: M (~2-3h). **Magnitude real**: M (~1.5h
— abaixo do limite por reuso massivo P224/P157A patterns).
**Marco**: **primeiro sub-passo materialização Fase 5 Layout
candidata**; **valida ADR-0080 PROPOSTO N=7 → 8** (primeira
aplicação real pós-formalização do pattern "L0 minimal para
refactors").

---

## §1 O que foi feito

P227 materializa A.1 stroke Grid + Table:
- **Grid +1 field** `stroke: Option<Stroke>` (8 → 9 fields).
- **Table +1 field** `stroke: Option<Stroke>` (3 → 4 fields).
- **`Value::Stroke(Stroke)` variant novo** (Value variants 54 → 55).
- **Helper `extract_stroke`** + **`native_stroke` constructor**
  + **`native_grid`/`native_table` accept `stroke:` shorthand**.
- **Renderização Opção β simplificada** (4 lines per cell border).
- **L0 NÃO tocado** validando ADR-0080 Opção γ N=7 → **8**.
- 18 tests novos (4 unit content + 11 unit stdlib + 3 E2E
  layout); workspace **2039 → 2057 verdes** (+18); 4
  adaptações intencionais Grid/Table constructors
  pre-existentes; 0 regressões reais; 0 violations.

---

## §2 Inventário pré-P227 (C1)

Audit empírico:
- `Stroke { paint: Color, thickness: f64 }` em
  `entities/geometry.rs:24` ✓.
- `Value::Color(Color)`, `Value::Length(Length)` existem;
  **`Value::Stroke` NÃO existe** (confirmado).
- `Color::BLACK` constant **ausente**; alternativa
  `Color::rgb(0, 0, 0)` directo (decisão minimal change).
- `Content::Grid` 8 fields pós-P224.
- `Content::Table` 3 fields baseline P157A.
- `native_grid` em `stdlib/layout.rs:196` ✓.
- `native_table` em `stdlib/structural.rs:222` ✓.

**Audit pattern N=7 confirmado**:
- P217+P218+P219+P220+P222+P223+P224 todos Opção γ L0 não
  tocado. P227 estende para N=8 validação real
  pós-formalização ADR-0080 PROPOSTO.

Sem `P227.div-N` empírico.

---

## §3 `Value::Stroke` + Grid/Table +stroke (C2-C4)

**`Value::Stroke(Stroke)`** adicionado em
`entities/value.rs:60` após `Color(Color)`:
- `type_name() => "stroke"`.
- `From<Stroke> for Value` impl.
- Arms Clone/PartialEq derive auto.

**Grid +1 field** em `entities/content.rs:303`:
```rust
Grid {
    columns, rows, cells,
    gutter, align, inset, header, footer,  // P224 baseline
    stroke: Option<Stroke>,                  // P227
}
```

**Table +1 field** em `entities/content.rs:753`:
```rust
Table {
    columns, rows, children,                 // P157A baseline
    stroke: Option<Stroke>,                  // P227 (refino paralelo Grid)
}
```

---

## §4 Helper + native_stroke + native_grid/table accept (C6-C7)

**Helper `extract_stroke(val, fn, field)`** em
`stdlib/layout.rs` após `extract_length` (Opção β shorthand
parsing paridade vanilla UX):
```rust
match val {
    Value::Length(l) => Stroke { paint: Color::rgb(0,0,0),
                                  thickness: l.abs.to_pt() },
    Value::Color(c)  => Stroke { paint: *c, thickness: 1.0 },
    Value::Stroke(s) => s.clone(),
    other            => Err(/* tipo errado */),
}
// + validação thickness > 0
```

**`native_stroke(paint:?, thickness:?)` constructor** em
`stdlib/layout.rs` após `native_measure` (~70 LOC):
- Defaults paint=`Color::rgb(0,0,0)`, thickness=1.0.
- Validações: sem posicionais, thickness > 0, named
  restritos a paint+thickness.
- Stdlib funcs: 59 → **60** (+native_stroke).
- Re-export + scope register paridade P218.

**`native_grid` + `native_table` refinados** para aceitar
`stroke:` named via `extract_stroke` shorthand. Lista
named args expandida (grid: ["columns", ..., "stroke"];
table: ["columns", "rows", "stroke"]).

---

## §5 Renderização Opção β (C8)

`layout_grid` signature +1 param `stroke: Option<&Stroke>`.
Imports `ShapeKind` + `Stroke` em `geometry`.

Renderização inline no loop de cells:
```rust
if let Some(s) = stroke {
    let x0 = cell_x;
    let y0 = row_start_y;
    let cell_h = row_h;
    // Top edge.
    self.regions.current.current_items.push(FrameItem::Shape {
        pos:    Point { x: Pt(x0), y: Pt(y0) },
        kind:   ShapeKind::Line { dx: cell_w, dy: 0.0 },
        width:  0.0, height: 0.0,  // f64 directos
        fill:   None,
        stroke: Some(s.clone()),
    });
    // Bottom, Left, Right análogos.
}
```

**Limitação documentada**: linhas adjacentes ficam duplicadas
(top de cell (1,0) sobrepõe bottom de cell (0,0)).
Visualmente correcto mas não-óptimo; refino A.3 candidato.

**Layouter consumer** em `layout/mod.rs`: arms `Content::Grid`
e `Content::Table` passam `stroke.as_ref()` ao `layout_grid`.

---

## §6 Decisões substantivas + validação ADR-0080 N=7 → 8

**7 decisões fixadas**:
- **Decisão 1** — Opção α `Option<Stroke>` uniforme (vs
  Sides per-side A.3; vs novo tipo GridStroke).
- **Decisão 2** — Opção β parsing Length/Color/Stroke
  shorthands paridade vanilla UX.
- **Decisão 3** — `Value::Stroke` variant novo (audit C1
  confirmou ausência; criado paridade `Value::Color`).
- **Decisão 4** — `native_stroke` constructor paridade
  `native_rgb` (não named-only field; constructor primário).
- **Decisão 5** — Opção β render simplificada (vs Opção α
  deduplicação A.3; vs Opção γ semantic adiada N=5).
- **Decisão 6** — Table refino paralelo Grid (variant-rico
  paridade).
- **Decisão 7** — ADR-0080 **NÃO promover EM VIGOR em P227**
  (P228 administrativo XS candidato; preserva política
  minimalista P158).

**ADR-0080 PROPOSTO Opção γ N=7 → 8 validado**:
- L0 prompts `entities/content.md` + `entities/value.md` +
  `entities/geometry.md` + `rules/stdlib.md` **NÃO
  tocados** em P227.
- `crystalline-lint --fix-hashes`: "Nothing to fix" em
  L0 layer.
- **Primeira aplicação real pós-formalização** do pattern
  "L0 minimal para refactors aditivos pós-M9c".
- **Pattern atingido N=8 cumulativo**; promoção ADR-0080
  PROPOSTO → EM VIGOR candidato sólido P228 administrativo
  XS.

**Anti-inflação 19ª aplicação cumulativa** pós-P205D
(Opção α field uniforme + Opção β parsing graded + Opção
γ L0 + Opção β render simplificada + sem helper construtor
Rust novo + ADR-0080 não promover em P227).

---

## §7 Resultados verificação + inventário 148 + ADR-0079 anotação (C9 + C11 + C12)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde (após ~20 errors E0027/E0063 cascade) |
| `cargo test --workspace` | ~2063 verdes | **2057 verdes** (1768+242+24+2+21) ✓ (18 novos vs ~24 spec; subset pragmático) |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | **"Nothing to fix"** ✓ (L0 não tocado validação ADR-0080) |
| Adaptações pre-existentes | N=2-5 | **N=4** (3 P224 unit tests `Grid {...}` + 1 P224 unit test `Value::Content(Content::Table {...})` pattern) |
| Content variants count | 59 preservado | ✓ 59 (Grid + Table refinados, não-novos) |
| Stdlib funcs count | 59 → 60 | ✓ 60 (+native_stroke) |
| Value variants count | 54 → 55 | ✓ 55 (+Stroke) |
| Regressões reais | 0 | **0** |

**Inventário 148** (`typst-cobertura-vanilla-vs-cristalino.md`):
- §A.5 Layout linha 141 `grid(columns, ...)`: footnote
  `⁵ ⁴⁵ ⁴⁶` → `⁵ ⁴⁵ ⁴⁶ ⁴⁷`. Sem reclassificação categórica
  (já `implementado⁺`); refino qualitativo cosmético
  stroke fechado.
- Footnote ⁴⁷ adicionada (~115 linhas) documentando A.1
  materializado + 7 decisões + ADR-0080 N=7 → 8 validado +
  patterns N=8 cumulativos + reuso N=10 extract_length.

**ADR-0079** (`typst-adr-0079-layout-fase-5-roadmap.md`):
- §"Aplicações cumulativas" criada com bloco
  `### P227 anotação — Categoria A sub-passo 1 (stroke
  Grid + Table)`.
- Status ADR-0079 mantido PROPOSTO (1/13-15 sub-passos).
- Categoria A: 1/5 materializados (A.1 ✓; A.2-A.5 pendentes).

---

## §8 Próximo sub-passo

P227 fecha primeiro sub-passo Fase 5 Layout candidata
(Categoria A 1/5). Decisão humana sobre próxima sessão:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **A.2 fill** | Grid + Table `fill: Option<Color>` cosmético (paridade A.1 structure) | S+ a M (~1.5-2h) | alta (sub-passo paralelo A.1; momentum natural P227; valida ADR-0080 N=8 → 9) |
| **P228 administrativo** | Promoção ADR-0080 PROPOSTO → EM VIGOR (XS dedicado) | XS (~30min) | média (consolida pattern N=8 atingido; reset administrativo limpo) |
| **B.1 DEBT-34d fix** | Auto track sizing algorítmico (fecha DEBT-34d preservado) | M (~2-3h) | média (algorítmico isolado; fecha DEBT) |
| **D.1 state** | `state(key, init)` runtime mutable (desbloqueia ADR-0066 IMPLEMENTADO) | M (~2-3h) | média (+33pp Introspection) |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |

**Recomendação subjectiva**: **A.2 fill** — sub-passo
paralelo A.1 structure (Opção α field uniforme + Opção β
parsing shorthand + extend native_grid/table); momentum
cumulativo P227 → P228 natural. Valida pattern
"refino aditivo paralelo entre variants irmãos" N=1 → 2.

Alternativa: **P228 administrativo** (promoção ADR-0080 EM
VIGOR) — reset limpo após N=8 atingido; permite
sub-passos seguintes A.2+ usar pattern formalmente
documentado como `EM VIGOR`.

**Decisão humana fica em aberto literal** pós-P227.

**Estado pós-P227**:
- Tests workspace: 2039 → **2057 verdes** (+18 P227).
- Content variants: 59 preservado (Grid + Table refinados).
- Value variants: 54 → **55** (+Stroke).
- Stdlib funcs: 59 → **60** (+native_stroke).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada
  (refino qualitativo P227; cobertura Layout 89% preservada).
- ADR-0066 PROPOSTO; ADR-0061 IMPLEMENTADO; ADR-0078
  IMPLEMENTADO; ADR-0079 PROPOSTO; ADR-0080 PROPOSTO
  (todos preservados).
- Saldo DEBTs: 12 preservado.
- **19 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" N=7 → 8 validado
  real** — primeira aplicação pós-ADR-0080 formalização;
  promoção EM VIGOR P228 candidato.
- **Pattern Smart→Option N=7 → 8 consolidado**.
- **Pattern "refino aditivo paralelo entre variants
  irmãos" N=1 inaugurado P227** (Grid+Table; reusável
  A.2+A.3).
- **`extract_length` reuso N=9 → 10** (patamar atingido;
  helper público candidato refino XS futuro).
- **Categoria A Fase 5 Layout**: 1/5 → próximos A.2 fill,
  A.3 per-cell, A.4 Block/Boxed, A.5 Place per-cell.
- **Fase 5 Layout candidata**: 1/13-15 sub-passos
  materializados (P227 A.1 ✓; restantes A.2-A.5 + B.1-B.3
  + C.1-C.2 + D.1-D.6 pendentes).
