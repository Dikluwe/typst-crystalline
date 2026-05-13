# Relatório do passo P228 — A.2 `fill` Grid + Table inheritance (Fase 5 Categoria A 2/5; valida ADR-0080 N=8 → 9)

**Data**: 2026-05-13.
**Spec**: `00_nucleo/materialization/typst-passo-228.md`.
**Tipo**: refino aditivo a 2 variants existentes (Grid + Table) +
parsing trivial inline + renderização Opção β Z-order correcto.
**Magnitude planeada**: S+ a M (~1.5-2h). **Magnitude real**: S+
(~1h — abaixo do limite por ausência de Value variant + ausência
de constructor stdlib + audit P227 confirmou Z-order correcto;
sem `P228.div-N`).
**Marco**: **segundo sub-passo Fase 5 Layout candidata —
paralelo estructural P227**; **valida ADR-0080 PROPOSTO N=8 →
9** (segunda aplicação real pós-formalização do pattern).

---

## §1 O que foi feito

P228 materializa A.2 fill Grid + Table:
- **Grid +1 field** `fill: Option<Color>` (9 → 10 fields).
- **Table +1 field** `fill: Option<Color>` (4 → 5 fields).
- **Sem `Value::Fill` variant novo** (Color reusado P25).
- **Sem `native_fill` constructor stdlib** (anti-inflação;
  Color tem `native_rgb`/`native_luma`).
- **`native_grid`/`native_table` accept `fill:` named** via
  inline match (rejeita Length).
- **Renderização Opção β Z-order correcto** (fill Rect
  antes do conteúdo; stroke Line depois).
- **L0 NÃO tocado** validando ADR-0080 Opção γ N=8 → **9**.
- 14 tests novos (4 unit content + 5 unit stdlib + 5 E2E
  layout Z-order); workspace **2057 → 2071 verdes** (+14);
  6 adaptações intencionais; 0 regressões reais; 0
  violations.

---

## §2 Inventário pré-P228 + audit Z-order P227 (C1)

**Audit empírico**:
- `Color::rgb(r, g, b)` constructor baseline P25 ✓.
- **`extract_color` helper NÃO existe** — usar inline match.
- `ShapeKind::Rect` baseline P76 ✓.
- `Value::Color(Color)` baseline P25 ✓.
- **P227 stroke Z-order CORRECTO** — emitido em
  `grid.rs:277` após `for item in cell_items` (linha 262)
  que transfere conteúdo cell. Z-order já correcto (stroke
  à frente do conteúdo). **Sem `P228.div-N`**.

Decisão crítica C1: P228 só precisa **adicionar fill
emission antes do conteúdo cell loop** (Z-order step 1);
P227 stroke preserva-se sem alteração.

Sem `P228.div-N` empírico (audit confirma hipótese provável
spec).

---

## §3 Grid/Table refino +fill field cada (C2+C3)

**Grid +1 field** em `entities/content.rs:307`:
```rust
Grid {
    columns, rows, cells,
    gutter, align, inset, header, footer,  // P224 baseline
    stroke,                                  // P227 (A.1)
    fill: Option<Color>,                     // P228 (A.2)
}
```

**Table +1 field** em `entities/content.rs:768`:
```rust
Table {
    columns, rows, children,                 // P157A baseline
    stroke,                                  // P227 (A.1)
    fill: Option<Color>,                     // P228 (A.2)
}
```

Grid fields: 9 → **10**. Table fields: 4 → **5**.

---

## §4 `native_grid`/`native_table` accept fill (C5)

**`native_grid` refinada** em `stdlib/layout.rs`:
- Accept named args expandido: `["columns", ..., "stroke",
  "fill"]`.
- Parsing fill trivial inline (Opção α):
```rust
let fill = match args.named.get("fill") {
    Some(Value::Color(c)) => Some(*c),
    Some(other) => return Err(/* "fill: espera Color, recebeu ..." */),
    None => None,
};
```

**`native_table` refinada** em `stdlib/structural.rs`:
- Accept `["columns", "rows", "stroke", "fill"]`.
- Parsing fill paridade Grid (inline match).

---

## §5 Renderização Opção β Z-order correcto (C6 + C7)

**`layout_grid` signature +1 param** `fill: Option<&Color>`:
```rust
pub(super) fn layout_grid(
    &mut self, ...,
    stroke: Option<&Stroke>,  // P227
    fill:    Option<&Color>,   // P228 — NEW
)
```

**Renderização Opção β em loop de cells**:
```rust
// P228 — Z-order step 1: fill emite primeiro (antes do conteúdo).
if let Some(c) = fill {
    self.regions.current.current_items.push(FrameItem::Shape {
        pos:    Point { x: Pt(cell_x), y: Pt(row_start_y) },
        kind:   ShapeKind::Rect,
        width:  cell_w,
        height: row_h,
        fill:   Some(*c),
        stroke: None,
    });
}

// Z-order step 2: conteúdo cell (existing P82-84.6 lógica).
for item in cell_items { ... }

// Z-order step 3 (P227): stroke emite depois (à frente).
if let Some(s) = stroke { ... }
```

**Layouter consumer**: arms `Content::Grid` e `Content::Table`
passam `fill.as_ref()` ao `layout_grid`.

---

## §6 Decisões substantivas + validação ADR-0080 N=8 → 9

**7 decisões fixadas**:
- **Decisão 1** — Opção α `Option<Color>` uniforme (vs Sides
  per-side A.3; vs Paint enum — Paint scope-out ADR-0029).
- **Decisão 2** — Opção α parsing trivial Color directo (vs
  Stroke shorthand inversão; semantic puro).
- **Decisão 3** — Opção γ **NÃO criar constructor stdlib**
  (anti-inflação; Color tem `native_rgb`/`native_luma`;
  distinto de P227 onde Stroke composto justificou
  `native_stroke`).
- **Decisão 4** — Opção β Z-order correcto (fill antes;
  conteúdo; stroke depois).
- **Decisão 5** — Tests E2E Z-order para validar interacção
  P227+P228.
- **Decisão 6** — Opção γ L0 NÃO tocado validação ADR-0080
  N=8 → 9.
- **Decisão 7** — ADR-0079 anotação Categoria A 2/5 (sem
  promoção).

**ADR-0080 PROPOSTO Opção γ N=8 → 9 validado**:
- L0 prompts NÃO tocados em P228.
- `crystalline-lint --fix-hashes`: "Nothing to fix" em L0
  layer.
- **Segunda aplicação real pós-formalização** do pattern
  "L0 minimal para refactors aditivos pós-M9c". **Pattern
  atingido N=9 cumulativo**; promoção ADR-0080 PROPOSTO →
  EM VIGOR candidato P229 administrativo XS muito sólido.

**Anti-inflação 20ª aplicação cumulativa** pós-P205D
(Opção α field uniforme + Opção α parsing trivial + Opção
γ sem constructor + Opção β Z-order + Opção γ L0 + sem
helper Rust + ADR-0080 não promover em P228).

---

## §7 Resultados verificação + inventário 148 + ADR-0079 (C8+C10+C11+C12)

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde (~10 errors E0027/E0063 cascade) |
| `cargo test --workspace` | ~2071 verdes | **2071 verdes** (1782+242+24+2+21) ✓ |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | **"Nothing to fix"** ✓ (L0 não tocado validação ADR-0080) |
| Adaptações pre-existentes | N=2-5 | **N=6** (3 P224+P227 unit tests `Grid {...}` + 1 partial_eq closure + 2 layout E2E tests) |
| Content variants count | 59 preservado | ✓ 59 (Grid+Table refinados) |
| Stdlib funcs count | 60 preservado | ✓ 60 (sem nova; anti-inflação) |
| Value variants count | 55 preservado | ✓ 55 (Color reusado P25) |
| Regressões reais | 0 | **0** |

**Inventário 148**:
- §A.5 Layout linha 141 `grid(...)`: footnote `⁵ ⁴⁵ ⁴⁶ ⁴⁷`
  → `⁵ ⁴⁵ ⁴⁶ ⁴⁷ ⁴⁸`. Sem reclassificação categórica.
- Footnote ⁴⁸ adicionada (~95 linhas) documentando A.2
  materializado + 7 decisões + ADR-0080 N=8 → 9 validado +
  patterns cumulativos N=9 + N=2 paralelo irmãos + N=1
  anti-inflação tipos existentes.

**ADR-0079**:
- §"Aplicações cumulativas" anotado com bloco
  `### P228 anotação — Categoria A sub-passo 2 (fill Grid
  + Table)`.
- Status mantido PROPOSTO (2/13-15 sub-passos).
- Categoria A: 2/5 materializados (A.1 ✓; A.2 ✓; A.3-A.5
  pendentes).

---

## §8 Próximo sub-passo

P228 fecha segundo sub-passo Fase 5 Layout candidata
(Categoria A 2/5). Decisão humana sobre próxima sessão:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **P229 administrativo** | Promoção ADR-0080 PROPOSTO → EM VIGOR (XS dedicado) | XS (~30min) | **alta** (N=9 atingido fortemente; reset administrativo limpo permite sub-passos seguintes usar pattern formalmente `EM VIGOR`) |
| **A.3 per-cell stroke/fill** | GridCell +stroke/+fill per-cell + precedence rules | M (~2-3h) | média (consolida Categoria A; precedência Grid-level vs per-cell) |
| **A.4 Block/Boxed cosméticos** | outset/radius/clip refinos P156G+H scope-outs | M (~2-3h cada) | baixa-média (refinos ortogonais Grid/Table) |
| **B.1 DEBT-34d fix** | Auto track sizing algorítmico (fecha DEBT-34d preservado) | M (~2-3h) | média |
| **D.1 state runtime** | desbloqueia ADR-0066 IMPLEMENTADO + Introspection +33pp | M (~2-3h) | média |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |

**Recomendação subjectiva**: **P229 administrativo** —
N=9 atingido fortemente; reset administrativo limpo
permite sub-passos seguintes usarem pattern formalmente
documentado como `EM VIGOR`. Momentum natural P227 → P228
→ P229 administrativo consolida metodologia antes de
expandir materialização Fase 5.

Alternativa imediata: **A.3 per-cell** — consolida
Categoria A continuação (3/5 sub-passos); valida pattern
"refino aditivo paralelo entre variants irmãos" N=2 → 3
(GridCell estende paralelismo Grid+Table de P227+P228).

**Decisão humana fica em aberto literal** pós-P228.

**Estado pós-P228**:
- Tests workspace: 2057 → **2071 verdes** (+14 P228).
- Content variants: 59 preservado.
- Value variants: 55 preservado.
- Stdlib funcs: 60 preservado (sem nova; anti-inflação).
- Grid fields: 9 → **10** (+fill).
- Table fields: 4 → **5** (+fill).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada
  (refino qualitativo; cobertura Layout 89% preservada).
- ADR-0066 PROPOSTO; ADR-0061 IMPLEMENTADO; ADR-0078
  IMPLEMENTADO; ADR-0079 PROPOSTO; ADR-0080 PROPOSTO.
- Saldo DEBTs: 12 preservado.
- **20 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" N=8 → 9 validado
  real** — segunda aplicação pós-ADR-0080 formalização;
  **promoção EM VIGOR P229 candidato muito sólido**.
- **Pattern Smart→Option N=8 → 9 consolidado**.
- **Pattern "refino aditivo paralelo entre variants
  irmãos" N=1 → 2 consolidado** (P227 stroke + P228 fill).
- **Pattern "anti-inflação por aproveitamento de tipos
  existentes" N=1 inaugurado P228** — Color primitivo
  dispensa constructor (distinto P227 Stroke composto).
- **Categoria A Fase 5 Layout**: 2/5 → próximos A.3
  per-cell, A.4 Block/Boxed, A.5 Place per-cell.
- **Fase 5 Layout candidata**: 2/13-15 sub-passos
  materializados (P227 A.1 ✓; P228 A.2 ✓; restantes
  pendentes).
