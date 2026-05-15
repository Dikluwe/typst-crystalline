# Spec do passo P252 — A.4 Boxed stroke-overhang via refactor `Stroke { paint, thickness, overhang }` cross-cutting (M; default `false` cristalino divergente conscientemente; default `true` vanilla aplicado em stdlib parse via `extract_stroke`; terceira aplicação cumulativa citante ADR-0082 PROPOSTO N=2 → 3 → triggera promoção EM VIGOR humana possível; fecha Boxed A.4 completo 6/6)

**Data**: 2026-05-14.
**Tipo**: refactor cross-cutting entity `Stroke` + activação
semantic real em Layouter (bounds Shape expandidos por
`thickness/2.0` quando `overhang=true`). Promove último
scope-out P156H Boxed (stroke-overhang); aproveita refactor
para suportar overhang em todos os variants com stroke (Block,
Boxed, Grid, GridCell, Table, TableCell — 6 variants
cumulativos). Default `false` no construtor Rust (cristalino
divergência consciente per ADR-0054 graded); default `true`
vanilla aplicado em stdlib parse via `extract_stroke`.
**Magnitude planeada**: **M (~2-4h)** — paridade audit
empírico §1.1 (~34 sítios construtores literais + 1 struct
declaration + 1 L0 prompt + 1 helper extract_stroke + 6
arms Shape emit em Layouter).
**Marco**: **terceira aplicação cumulativa citante ADR-0082
PROPOSTO** N=2 → 3 → **triggera promoção EM VIGOR humana
possível** (paridade ADR-0065 P156K validado via P156J/P157A/
P157B sequente atinge N=3 → EM VIGOR pós-humano); **fecha
Boxed A.4 completo 6/6** (último scope-out P156H stroke-overhang);
**Boxed A.4 COMPLETO**; **primeira aplicação cumulativa do
padrão "refactor cross-cutting entity primitivo"** N=1
inaugurado; décima quinta aplicação cumulativa pattern "spec
C1 audit obrigatório bloqueante pós-P236.div-1" N=14 → 15
cumulativo (lição refinada P252: "refactor cross-cutting de
entity primitivo exige mapa empírico exhaustive de todos os
construtores literais antes de modificar struct").

---

## §1 O que será feito

### §1.1 Estado pré-P252 (factual; confirmado audit empírico 2026-05-14)

**`Stroke` struct** (`01_core/src/entities/geometry.rs:24`):

```rust
pub struct Stroke {
    pub paint:     Color,
    pub thickness: f64,
}
```

**Construtores literais cumulativos identificados**: **~34 sítios**.

| Categoria | Ficheiro | Sítios |
|-----------|----------|--------|
| Struct declaration + test | `entities/geometry.rs` | 1 (linha 73) |
| Tests em content.rs | `entities/content.rs` | 10 (linhas 4047-4591) |
| Layouter literal | `rules/layout/mod.rs` | 1 (linha 1142) |
| Stdlib shapes | `rules/stdlib/shapes.rs` | 8 (linhas 63-260) |
| Layouter tests | `rules/layout/tests.rs` | 14 (linhas 3154-6440) |

**Stroke em Content variants**: **8 declarações** `stroke:
Option<Stroke>` (mais do que as 6 hipotetizadas; auditoria
revelou):
- linha 255 — provavelmente Grid (P227).
- linha 309 — provavelmente GridCell.
- linha 363 — provavelmente Table.
- linha 637 — Block (P247).
- linha 700 — Boxed (P247).
- linha 754 — provavelmente TableCell.
- linha 893 — variant adicional (a confirmar §2).
- (uma das linhas pode ser arm duplicado de PartialEq/map_*).

**Consumer Layouter Shape emit**: 6 arms identificados:
- Block (P247 mod.rs ~linha 1330) — Shape emit com stroke.
- Boxed (P247 mod.rs ~linha) — Shape emit com stroke.
- Grid (grid.rs P227) — outer border + cell borders.
- GridCell (grid.rs P228+P230+P234) — cell border.
- Table (grid.rs P227 via layout_grid delegação) — paridade
  Grid.
- TableCell (grid.rs P157B + P248 + P251) — cell border per
  P227.

**`extract_stroke` helper** em `stdlib/layout.rs:351`:

```rust
pub(super) fn extract_stroke(val: &Value, fn_name: &str, field: &str)
    -> SourceResult<Stroke>;
```

Reusado 6× (linhas 282 grid, 622 grid_cell, 257-259 table,
414 table_cell, 815 native_block, mais 1 native_box pós-P247).

**PDF exporter `03_infra/src/export.rs`**: 4 caminhos `FrameItem::
Shape { ..., stroke }` (linhas 845, 1117, 1361, 1543 confirmadas
P247 audit). Usam `stroke.paint` + `stroke.thickness`. **NÃO
usam overhang** — bounds recebem-se finais já calculados
em Layouter.

### §1.2 Trabalho a fazer P252

**1. Refactor `Stroke` struct** (`entities/geometry.rs:24`):

```rust
pub struct Stroke {
    pub paint:     Color,
    pub thickness: f64,
    pub overhang:  bool,   // P252 — default false em construtor Rust
}
```

**2. Adaptação ~34 construtores literais**:

Adicionar `overhang: false` mecânicamente em todos os ~34
sítios identificados §1.1. Cascade replace_all guiado:

```rust
// Antes (P227-P251):
Stroke { paint: Color::rgb(0, 0, 0), thickness: 1.0 }

// Depois (P252):
Stroke { paint: Color::rgb(0, 0, 0), thickness: 1.0, overhang: false }
```

**3. Helper `extract_stroke` actualizado**:

```rust
pub(super) fn extract_stroke(val: &Value, fn_name: &str, field: &str)
    -> SourceResult<Stroke>
{
    match val {
        Value::Length(l) => Ok(Stroke {
            paint:    Color::rgb(0, 0, 0),
            thickness: l.to_pt(),
            overhang: true,  // P252 — vanilla default para inputs stdlib
        }),
        Value::Color(c) => Ok(Stroke {
            paint:    *c,
            thickness: 1.0,
            overhang: true,  // vanilla default
        }),
        Value::Dict(d) => {
            // Parse paint + thickness existente
            let paint     = d.get("paint")?;
            let thickness = d.get("thickness")?;
            let overhang  = d.get("overhang")
                            .map(|v| v.as_bool().ok())
                            .flatten()
                            .unwrap_or(true);  // vanilla default
            Ok(Stroke { paint, thickness, overhang })
        }
        _ => Err(...),
    }
}
```

**4. Activação semantic real Layouter** (6 arms Shape emit):

```rust
// Pattern aplicado em Block + Boxed + Grid + GridCell +
// Table + TableCell arms Shape emit:
if let Some(stroke) = &effective_stroke {
    let overhang_pt = if stroke.overhang {
        stroke.thickness / 2.0
    } else {
        0.0
    };
    let shape_bounds = Rect {
        x:      pos.x - overhang_pt,
        y:      pos.y - overhang_pt,
        width:  width + 2.0 * overhang_pt,
        height: height + 2.0 * overhang_pt,
    };
    self.regions.current.current_items.push(FrameItem::Shape {
        pos:    Point { x: Pt(shape_bounds.x), y: Pt(shape_bounds.y) },
        kind:   shape_kind,
        width:  shape_bounds.width,
        height: shape_bounds.height,
        fill:   None,
        stroke: Some(stroke.clone()),
    });
}
```

**5. L0 prompt `geometry.md` refino**:

Documentar field `overhang: bool` + default cristalino `false`
+ paridade vanilla via stdlib + activação Layouter bounds
expansão. Hash propagado automaticamente.

**6. Boxed A.4 COMPLETO** — último scope-out P156H fechado:
6/6 (outset+radius+clip+fill+stroke+stroke-overhang).

### §1.3 Limitações conscientes P252

**Limitação 1**: construtor Rust `Stroke { paint, thickness,
overhang }` default `false` divergente da vanilla (`true`).
Paridade vanilla preservada via stdlib parse `extract_stroke`.
Documentado em ADR-0054 §"Promoções reais cumulativas" tabela
P252 + L0 prompt `geometry.md` §"Default cristalino divergente
P252".

**Limitação 2**: PDF exporter não consume `stroke.overhang`
directamente — bounds finais já recebidos com overhang
aplicado em Layouter. Single source of truth.

**Limitação 3**: round corners (P242 ShapeKind::RoundedRect)
+ overhang: bounds Shape rounded com radius preservado;
overhang expande bounds incluindo radius. Paridade vanilla
graded.

**Limitação 4**: stroke-overhang em variants Grid/Table cells:
overhang aplicado em borders cell-level — divergência possível
face a vanilla onde overhang é só BoxElem. **Decisão**: P252
aplica uniformemente a todos os 6 variants para consistência
arquitectural; documentado em §"Distinção vanilla" do relatório.

### §1.4 Tests esperados

Tests P252 novos estimados: **8-15** (range M magnitude;
refactor cross-cutting + activação semantic + paridade):

- 2-3 unit Stroke struct (overhang field; PartialEq adapted;
  Clone adapted; default false em construtor).
- 3-5 unit `extract_stroke` (Length input → overhang true;
  Color input → overhang true; Dict com `overhang: false` →
  false; Dict sem overhang → true default vanilla).
- 2-3 unit Layouter bounds Shape (overhang true expande
  bounds por thickness/2; overhang false preserva bounds
  literais).
- 1-2 E2E Boxed stroke-overhang (BoxElem com `stroke:
  {overhang: true}` → bounds visualmente expandidos; com
  `overhang: false` → bounds literais).
- 1-2 regression P247 stroke literal preservado (defaults
  pré-P252 = overhang false → output PDF bit-equivalente).

**Workspace pós-P252**: **2294 → ~2302-2309 verdes** (range
+8-15 paridade M magnitude).

### §1.5 Adaptações pre-existentes

Estimativa **N=30-40** adaptações tests pré-existentes
(range alto devido a refactor struct cross-cutting):

- ~24 construtores literais Stroke em tests precisam de
  `overhang: false` mecânicamente.
- ~10 construtores em prod (geometry + content + layout +
  shapes) precisam de `overhang: false` mecânicamente.
- PartialEq tests Stroke ganham field na comparação.
- Cascade replace_all guiado provavelmente cobre 90% via
  search-replace.

**Cenário `P252.div-N` se >40 adaptações** → reconciliação
prévia.

---

## §2 Verificação empírica pré-P252 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=14 → 15 cumulativo

Audit C1 obrigatório bloqueante pós-P236.div-1. Lição refinada
N=14 P251 ("audit C1 deve confirmar localidade pos.y antes de
fixar abordagem γ-Items vs γ-Content para slicing") expande
para **N=15 cumulativo**: "refactor cross-cutting de entity
primitivo exige mapa empírico exhaustive de todos os
construtores literais antes de modificar struct".

### §2.1 Stroke struct + construtores literais (confirmado 2026-05-14)

Auditoria empírica concluída pré-spec: ~34 sítios construtores
literais identificados §1.1. Refino procedural P252:
**audit pre-spec preceding spec writing** — primeira aplicação
cumulativa onde audit C1 é realizado antes do spec ser escrito
(em vez de durante audit-during-spec).

### §2.2 Stroke em Content variants completos

```bash
grep -n "stroke:.*Option<Stroke" 01_core/src/entities/content.rs
```

Resultado audit anterior: **8 declarações**. Identificar
exactamente quais (linha 893 incerta). Confirmar:
1. Grid (P227 linha 255).
2. GridCell (P228+P230 linha 309).
3. Table (linha 363).
4. Block (P247 linha 637).
5. Boxed (P247 linha 700).
6. TableCell (P157B+P227 linha 754).
7. **Linha 893** — confirmar variant.
8. Possível duplicação arm cascade.

### §2.3 `extract_stroke` helper exhaustive

```bash
grep -B2 -A 20 "pub(super) fn extract_stroke" 01_core/src/rules/stdlib/layout.rs
```

Confirmar match arms (Length, Color, Dict, outros).

### §2.4 PDF exporter Shape emit (confirmado 2026-05-14)

4 sítios `FrameItem::Shape { ..., stroke }` confirmados.
**Não consome overhang** — bounds finais.

### §2.5 Algoritmo vanilla stroke-overhang — REFERÊNCIA EMPÍRICA

```bash
grep -B2 -A 10 "overhang\|stroke" \
  lab/typst-original/crates/typst-library/src/visualize/stroke.rs \
  2>/dev/null | head -50
```

Identificar:
- Vanilla default `overhang: true` confirmado.
- Como vanilla aplica overhang em bounds? (Layout consumer?
  Exporter?)
- Round corners interaction.
- BoxElem-only ou todos os variants?

**Confronto referência empírica obrigatório em C2** antes
de cristalizar Decisões 1-3.

### §2.6 Tests pré-P252 baseline

```bash
cargo test --workspace
```

Esperado: **2294 verdes** (estado pós-P251).

### §2.7 Decisão arquitectural pós-audit

Após §2.1-§2.6 completos, fixar empíricamente:
- **Decisão 1** struct `Stroke` final.
- **Decisão 2** default construtor Rust (`false` per decisão
  humana pré-spec).
- **Decisão 3** default stdlib parse (`true` vanilla via
  `extract_stroke`).
- **Decisão 4** activação semantic real Shape emit (6 arms
  uniformemente vs Boxed-only).
- **Decisão 5** PDF exporter intocado confirmado.

### `P252.div-N` antecipadas — possíveis

- **`P252.div-1`** se §2.2 revelar variants Stroke adicionais
  não-listados.
- **`P252.div-2`** se §2.5 vanilla é overhang aplicado per-
  shape-kind diferente (ex: Rect vs RoundedRect têm fórmulas
  diferentes).
- **`P252.div-3`** se §2.5 revelar vanilla BoxElem-only
  → re-decisão §1.3 Limitação 4 (uniform 6 vs Boxed-only 1).
- **`P252.div-4`** se §2.6 baseline ≠ 2294 → reconciliação
  prévia.

---

## §3 Decisões fixadas P252 — 12 decisões

### Decisão 0 — Audit C1 lição N=14 → 15 cumulativo

Pattern "spec C1 audit obrigatório bloqueante pós-P236.div-1"
N=14 → **15 cumulativo**. Refino procedural P252: "refactor
cross-cutting de entity primitivo exige mapa empírico
exhaustive de todos os construtores literais antes de modificar
struct". Anotação em ADR-0080 §"Lição refinada P252".

### Decisão 1 — `Stroke` struct +1 field `overhang: bool`

```rust
pub struct Stroke {
    pub paint:     Color,
    pub thickness: f64,
    pub overhang:  bool,
}
```

**Justificação**: paridade vanilla literal (struct match
exacto); minimal cross-cutting; Stroke já é entity primitivo
usado em 6 variants Content.

### Decisão 2 — Default construtor Rust `overhang: false` (cristalino divergente conscientemente)

Construtor Rust `Stroke { paint, thickness, overhang: false }`
é divergência consciente face a vanilla default `true`.
**Justificações cumulativas**:
1. Backward compat literal estrita: bounds Shape preservados
   bit-equivalente em todos os ~34 construtores existentes.
2. Anti-inflação 44ª: defaults zero-impact preservados em
   construtor Rust low-level.
3. Tests pré-P252 não regridem (paridade pattern P247 fill
   default `None` zero-impact).
4. Documentado em ADR-0054 §"Promoções reais cumulativas"
   como divergência graded P252.

### Decisão 3 — Default stdlib parse `overhang: true` (paridade vanilla via `extract_stroke`)

`extract_stroke` aplica default vanilla `true` quando:
- Input é Length atalho.
- Input é Color atalho.
- Input é Dict sem chave `overhang` explícita.

Apenas `Dict { ..., overhang: false }` produz `false`
explícito. Paridade vanilla user-facing preservada.

### Decisão 4 — Activação semantic real uniformemente em 6 variants Shape emit

Block + Boxed + Grid + GridCell + Table + TableCell todos
recebem activação `overhang` em Shape emit. **Não Boxed-only**
(divergência consciente §1.3 Limitação 4). Justificação:
consistência arquitectural cristalina; refactor cross-cutting
benefit-maximization.

Decisão final §2.7 pode reverter para Boxed-only se §2.5
vanilla revelar limitação real BoxElem-only.

### Decisão 5 — PDF exporter intocado

Single source of truth: bounds Shape finais já calculados
em Layouter. PDF exporter recebe `FrameItem::Shape { pos,
width, height, ..., stroke }` e usa directamente. **Zero
trabalho L3**.

### Decisão 6 — `extract_stroke` helper expandido (não duplicado)

Adicionar parse de `overhang` ao helper existente (sem criar
helper novo). Paridade subpadrão "helper privado reuso" N+1
cumulativo (precedentes: extract_color P247 inline; extract_length
N=7+; extract_stroke P227 N=6 reusos).

### Decisão 7 — Sem novo Content variant; sem nova ADR; sem novo entity type

P252 refactor cross-cutting de entity existente (`Stroke`)
+ activação consumer Layouter. **Anti-inflação 44ª aplicação
cumulativa** preservar.

### Decisão 8 — Cita ADR-0082 PROPOSTO N=2 → 3 (terceira citante)

P252 é **terceira aplicação concreta citante** ADR-0082
PROPOSTO (P250 N=1; P251 N=2; **P252 N=3**). 4 critérios
operacionais ADR-0082 verificados explicitamente:

1. **Storage prévio**: stroke-overhang scope-out P156H
   declarado "rejeitado em `native_box` com erro hard" (graded).
2. **Consumer Layouter pre-promoção é graded**: scope-out
   actualmente erro hard — não armazenado nem consumido.
3. **Paridade vanilla referência empírica**: audit C1 §2.5
   obrigatório.
4. **Backward compat literal**: defaults `overhang: false`
   construtor Rust preservam output bit-equivalente; só
   inputs stdlib com `overhang: true` ganham semantic nova
   (paridade vanilla user-facing).

**Validação ADR-0082 N=3 citante atingida** — **triggera
promoção EM VIGOR humana possível** (paridade ADR-0065 P156K
validada via P156J/P157A/P157B sequente).

### Decisão 9 — Sub-padrão "Refactor cross-cutting entity primitivo" N=1 inaugurado

P252 inaugura sub-padrão **N=1**: "Refactor cross-cutting
entity primitivo com cascade replace_all guiado". Pattern
emergente candidato a formalização N=3-4 futuro (hipóteses:
`Color` extensão; `Length` extensão; `Sides<T>` refactor).

### Decisão 10 — Marco Boxed A.4 COMPLETO 6/6 scope-outs P156H

P252 fecha último scope-out P156H Boxed (stroke-overhang).
**Boxed A.4 COMPLETO 6/6**: outset + radius + clip + fill +
stroke + **stroke-overhang**. **Segundo variant Content com
100% scope-outs originais fechados cumulativamente** (Block
P250 foi primeiro 10/10; Boxed P252 segundo 6/6).

### Decisão 11 — Promoções reais scope-outs ADR-0054 graded cumulativas N=13 → 14

P252 promove **1 sub-activação granular** nova: stroke-overhang
semantic real. **Cumulativo granular**: N=13 (pós-P251) + 1 =
**N=14 cumulativo pós-P252**. ADR-0054 §"Promoções reais
cumulativas" anotada P252.

### Decisão 12 — Anti-inflação 44ª aplicação cumulativa

- Opção β L0 minimal: refino `geometry.md` documentando field
  `overhang` + default cristalino `false` + paridade vanilla
  via stdlib; hash propagado.
- Opção α extensão field-by-field (+1 field em struct
  primitivo).
- Opção α activação consumer real (6 arms Shape emit).
- Opção α reuso `extract_stroke` helper expandido (não
  duplicado).
- Opção α default zero-impact construtor Rust (backward
  compat literal estrita; primeira aplicação cumulativa
  pós-P251 sub-padrão "backward compat literal estrita"
  N=1 → 2).
- Opção α anotação cumulativa minimal ADRs (0061 + 0079 +
  0080 + 0054 + **0082 citação terceira**).
- Opção α sub-padrão N=1 inaugurado "refactor cross-cutting
  entity primitivo".
- Opção α Boxed A.4 COMPLETO marco interno.

---

## §4 Ficheiros a editar (C2+C3+C4+C5)

| Categoria | Ficheiro | Trabalho |
|-----------|----------|----------|
| L1 entity | `01_core/src/entities/geometry.rs` | `Stroke` +1 field `overhang: bool`; PartialEq/Clone derivados estendidos automaticamente |
| L0 prompt | `00_nucleo/prompts/entities/geometry.md` | Documentar field `overhang` + default cristalino `false` + paridade vanilla via stdlib; secção nova "§Default cristalino divergente P252" |
| L1 stdlib | `01_core/src/rules/stdlib/layout.rs` | `extract_stroke` helper expandido com parse `overhang` (default vanilla `true` quando ausente do dict) |
| L1 stdlib | `01_core/src/rules/stdlib/shapes.rs` | 8 construtores Stroke em shapes ganham `overhang: false` mecânicamente |
| L1 Layouter | `01_core/src/rules/layout/mod.rs` | Arm Block + Boxed Shape emit: bounds expansão por `thickness/2.0` quando overhang=true; 1 construtor literal Stroke ganha `overhang: false` (linha 1142) |
| L1 Layouter | `01_core/src/rules/layout/grid.rs` | Arms Grid + GridCell + Table + TableCell Shape emit: bounds expansão análoga |
| Tests adaptações content | `01_core/src/entities/content.rs` (test module) | ~10 construtores Stroke ganham `overhang: false` mecânicamente |
| Tests adaptações geometry | `01_core/src/entities/geometry.rs` (test module) | 1 construtor + 2-3 unit tests novos para field overhang |
| Tests adaptações layout | `01_core/src/rules/layout/tests.rs` | ~14 construtores Stroke ganham `overhang: false` mecânicamente; 1-3 tests novos Layouter bounds; 1-2 regression P247 stroke literal preservado |
| Tests stdlib | `01_core/src/rules/stdlib/mod.rs` (test module) | 3-5 unit `extract_stroke` overhang parse |
| Tests E2E | `01_core/src/rules/layout/tests.rs` ou local | 1-2 E2E Boxed stroke-overhang |
| Inventário 148 | `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` | §A.5 `box(...)` reclassificada (footnote ⁶⁹ P252 — Boxed A.4 COMPLETO); cobertura Layout per metodologia recalculada |
| ADR-0061 | `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md` | §"Refino futuro" anotação P252 — Boxed A.4 COMPLETO |
| ADR-0079 | `00_nucleo/adr/typst-adr-0079-fase-5-layout-roadmap.md` | Categoria A.4 §"Sub-categorias materializadas": Boxed.stroke-overhang P252; **Boxed A.4 COMPLETO 6/6**; segundo variant Content com 100% scope-outs fechados |
| ADR-0080 | `00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md` | §"Lição refinada P252" N=15 cumulativo; sub-categoria nova "Refactor cross-cutting entity primitivo" N=1 inaugurada |
| ADR-0054 | `00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md` | §"Promoções reais cumulativas" extensão: P252 ×1 = cumulativo N=13 → **N=14** (P242 ×2 + P247 ×3 + P248 ×3 + P250 ×4 + P251 ×1 + **P252 ×1**); divergência consciente default `overhang: false` documentada |
| **ADR-0082** | `00_nucleo/adr/typst-adr-0082-promocoes-reais-scope-outs-graded.md` | **§"Aplicações citantes" sub-secção: P252 terceira aplicação citante explícita** (N=2 → 3); **N=3 limiar atingido** — **triggera promoção EM VIGOR humana possível** documentada |
| DEBT.md | `00_nucleo/DEBT.md` | DEBT-30/34c/34e/56 sentinelas preservadas; sem reabertura; sem novo DEBT |
| Relatório P252 | `00_nucleo/materialization/typst-passo-252-relatorio.md` | Estrutura canónica passos materialização M magnitude |

---

## §5 Critério aceitação P252 (C6+C7)

| Critério | Esperado |
|----------|----------|
| `cargo build --workspace` | **verde** |
| `cargo test --workspace` | **2294 → ~2302-2309 verdes** (+8-15 paridade M) |
| `crystalline-lint .` | **0 violations** |
| `crystalline-lint --fix-hashes` | **1 hash propagado** (`geometry.md`) |
| Content variants | **62 preservado** |
| ShapeKind variants | **5 preservado** |
| Block / Boxed / TableCell fields | preservados |
| `Stroke` fields | **2 → 3** (+overhang) |
| Layouter fields | preservado |
| Regions fields | **4 preservado** |
| Stdlib funcs | **64 preservado** |
| §A.5 `box(...)` | reclassificação implementado⁺ + footnote ⁶⁹ P252 (Boxed A.4 COMPLETO) |
| Cobertura Layout per metodologia | **~97-98% → ~98-99%** (+1pp refino qualitativo) |
| Cobertura user-facing total | **~75-76% preservado** |
| Scope-outs Block originais P156G fechados | **10/10 preservado** (Block A.4 COMPLETO) |
| Scope-outs Boxed originais P156H fechados | **5/6 → 6/6** (Boxed A.4 COMPLETO; segundo variant com 100% scope-outs fechados) |
| Promoções reais scope-outs ADR-0054 cumulativas granular | **13 → 14** (P252 ×1) |
| ADR-0079 Categoria A.4 | anotação cumulativa P242+P246+P247+P248+P250+P251+P252; **Boxed A.4 COMPLETO** |
| ADR-0080 sub-categoria | "Refactor cross-cutting entity primitivo" N=1 inaugurada |
| ADR-0061 §"Refino futuro" | anotação P252 |
| ADR-0054 §"Promoções reais" | cumulativo granular N=14 |
| **ADR-0082** | **§"Aplicações citantes" N=2 → 3** (terceira citante; **N=3 limiar atingido** — promoção EM VIGOR humana possível) |
| DEBT-30/34c/34e/56 | sentinelas preservadas |
| L0 hashes propagados | 1 (`geometry.md`) |
| Adaptações pre-existentes | **N=30-40** estimadas (cascade replace_all guiado); `P252.div-N` se >40 |
| Regressões reais | **0** mandatório |
| Patterns emergentes | "Refactor cross-cutting entity primitivo" N=1 inaugurado; "Aplicação citante ADR-0082" N=2 → 3 (limiar atingido); "Spec C1 audit obrigatório bloqueante" N=14 → 15 cumulativo; "Backward compat literal estrita" N=1 → 2 cumulativo |

**3 pré-condições obrigatórias verificadas**:

1. **Tests baseline preservados**: 2294 verdes pré-P252 →
   ~2302-2309 pós-P252 (+8-15 novos; N=30-40 adaptações
   construtores mecânicos documentadas).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   P252 toca entity primitivo + Layouter consumer + stdlib
   apenas; Introspector trait intocada.
3. **Backward compat literal**: construtores literais Stroke
   com `overhang: false` (default Rust) + stdlib inputs sem
   `overhang` explícito → vanilla `true` aplicado pelo helper.
   Output PDF bit-equivalente para tests pré-P252 (construtores
   literais usam `false`; só novos tests testam overhang=true).

**Promoções ADR esperadas**:

- ADR-0079 Categoria A.4 **Boxed A.4 COMPLETO 6/6** documentado;
  segundo variant Content com 100% scope-outs fechados (após
  Block P250).
- ADR-0080 sub-categoria nova "Refactor cross-cutting entity
  primitivo" N=1 inaugurada + lição refinada N=15 cumulativo.
- ADR-0061 §"Refino futuro" anotação P252.
- ADR-0054 §"Promoções reais" cumulativo granular N=14 +
  divergência consciente default `overhang: false`.
- **ADR-0082 §"Aplicações citantes" N=2 → 3** (terceira citante;
  **N=3 limiar interno atingido** — **triggera promoção EM VIGOR
  humana possível**).
- **Sem novas ADRs criadas**.

---

## §6 Próximo sub-passo pós-P252

P252 fecha **Boxed A.4 COMPLETO 6/6** + **N=3 citantes ADR-0082
atingido**. Restantes pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **ADR-0082 → EM VIGOR humana** | Passo administrativo XS promoção (paridade ADR-0065 P156K validado pós-N=3 citantes) | XS | **alta** (limiar atingido P252; decisão humana directa) |
| **ADR-0079 → IMPLEMENTADO graded** | Categoria A.4 muito reforçada (Block 10/10 + Boxed 6/6 + TableCell row break + C.2 parcial) | XS-S | **alta** (Fase 5 Layout candidata fechamento administrativo) |
| **DEBT-34e abrir P-passo** | Refactor placement Grid completo (colspan/rowspan real) | L+ | baixa (não-reservado P158) |
| **Pivot outro módulo** | Visualize 54%; Text 52%; Model 50% | varia | média (Layout muito reforçado pós-P252; pivot razoável) |
| **A.4 TableCell row break refino** | γ-Content via re-layout (refino P251 γ-Items) | L+ | baixa (P251 graded suficiente) |
| **Pausa marco** | A.4 Block COMPLETO + A.4 Boxed COMPLETO + C.2 parcial + 14 promoções reais + ADR-0082 N=3 limiar | XS | baixa |

**Recomendação subjectiva pós-P252**: **ADR-0082 → EM VIGOR
humana** (passo administrativo XS) — primeira aplicação cumulativa
do padrão "ADR meta PROPOSTO → EM VIGOR pós-N=3 citantes"
(paridade ADR-0065). Magnitude XS pura administrativa; valida
ADR-0082 empíricamente como pattern sólido cumulativo.

Alternativa: **ADR-0079 → IMPLEMENTADO graded** (XS-S) —
fechamento administrativo Fase 5 Layout agora que A.4 Block
COMPLETO + A.4 Boxed COMPLETO + A.4 TableCell row break +
C.2 parcial. **Patamar conceptual máximo** para fechamento
administrativo Fase 5.

**Decisão humana fica em aberto literal** pós-P252.

**Estado esperado pós-P252**:
- Tests workspace: **~2302-2309 verdes** (+8-15 P252).
- Content variants: **62 preservado**.
- Block / Boxed / TableCell fields: preservados.
- ShapeKind variants: **5 preservado**.
- **`Stroke` fields: 2 → 3** (+overhang).
- Layouter fields: preservado.
- Layouter methods: preservado.
- Regions fields: **4 preservado**.
- Stdlib funcs: **64 preservado**.
- §A.5 distribuição: refino qualitativo (footnote ⁶⁹ P252 —
  Boxed A.4 COMPLETO).
- Cobertura Layout per metodologia: **~97-98% → ~98-99%**.
- Cobertura user-facing total: **~75-76% preservado**.
- **ADRs distribuição preservada literal**: PROPOSTO 13; EM VIGOR
  29; IMPLEMENTADO 23; total **69 preservado**. Anotações
  cumulativas 0061+0079+0080+0054+**0082 §"Aplicações citantes"
  N=3 — limiar atingido**.
- **Saldo DEBTs: 11 preservado** (DEBT-30/34c/34e/56 sentinelas
  preservadas; sem reabertura; sem novo DEBT).
- **44 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P252** (4):
  - "Refactor cross-cutting entity primitivo" N=1 inaugurado.
  - "Aplicação citante ADR-0082 PROPOSTO" N=2 → **3 cumulativo
    (limiar atingido)**.
  - "Spec C1 audit obrigatório bloqueante" N=14 → **15
    cumulativo**.
  - "Backward compat literal estrita" N=1 → **2 cumulativo**
    (P251 cell tails + P252 stroke overhang).
- "Promoção real scope-out ADR-0054 graded" granular N=13 →
  **14 cumulativo** (P252 ×1).
- **Scope-outs originais Block fechados**: 10/10 preservado
  (Block A.4 COMPLETO).
- **Scope-outs originais Boxed fechados**: 5/6 → **6/6**
  (**Boxed A.4 COMPLETO**; segundo variant Content com 100%
  scope-outs fechados).
- **Categoria A.4 Fase 5 Layout**: Block COMPLETO + Boxed
  COMPLETO; **TableCell parcial (row break real)**; multi-
  region completo C.2 continua diferido.
- **Marco interno**: Boxed A.4 COMPLETO 6/6 — segundo variant
  Content com 100% scope-outs originais fechados (após Block
  P250); refactor cross-cutting de entity primitivo `Stroke`
  primeira aplicação cumulativa; **ADR-0082 N=3 limiar interno
  atingido** — promoção EM VIGOR humana possível; padrão
  "Backward compat literal estrita" N=2 cumulativo consolida;
  lição C1 audit N=15 cumulativa refinada procedimentalmente;
  primeiro passo cumulativo onde audit empírico pré-spec é
  formalizado como **lição procedural**.

---

## §7 Notas operacionais para o executor

1. **Audit C1 BLOQUEANTE prioridade absoluta**. Não materializar
   antes de §2.1-§2.7 completos. **Lição N=15 cumulativa**:
   refino procedural "refactor cross-cutting de entity primitivo
   exige mapa empírico exhaustive de todos os construtores
   literais antes de modificar struct". Audit pre-spec **já
   feito** (§1.1 + decisão humana §3.2 default); §2 audit
   confirma + estende.

2. **Cascade replace_all guiado**: `sed` ou IDE search-replace
   per construtor pattern:

   ```bash
   # Pattern 1: Stroke { paint: ..., thickness: ... }
   # Target:   Stroke { paint: ..., thickness: ..., overhang: false }
   ```

   Cuidado com edge cases (formatting multilinha; comentários
   embutidos). Verificação manual final via `cargo build`.

3. **Decisão 4 final fixa pós-audit §2.7** — uniformemente em
   6 variants vs Boxed-only. Audit vanilla §2.5 decide. Se
   `P252.div-3` activar (vanilla é BoxElem-only), restringir
   activação a arm Boxed apenas; outros variants preservam
   `overhang: false` zero-impact mas estrutura field
   preservada.

4. **Ordem de implementação recomendada**:
   1. Audit C1 §2 completo (~30-45 min — inclui leitura
      vanilla §2.5).
   2. Decisões finais §3 (~10-15 min documentação).
   3. `Stroke` struct +1 field (~5-10 min).
   4. L0 prompt `geometry.md` refino (~15-20 min).
   5. Cascade replace_all construtores (~30-45 min — N=30-40
      sítios).
   6. `extract_stroke` helper expandido (~15-20 min).
   7. Layouter Shape emit activação 6 arms (~45-60 min).
   8. Tests novos + adaptações (~45-60 min).
   9. Anotações ADRs + inventário 148 + relatório (~30-45
      min).

   **Total ~3-5h** paridade M magnitude.

5. **Backward compat para construtores literais Rust**: defaults
   `overhang: false` preservam bounds Shape literais. Test
   sentinela: `p252_stroke_construtor_rust_default_overhang_false_preserva_bounds`.

6. **Paridade vanilla para stdlib inputs**: `extract_stroke`
   aplica default `true`. Test sentinela:
   `p252_extract_stroke_dict_sem_overhang_default_true_paridade_vanilla`.

7. **Custo real esperado**: ~3-5h (paridade M magnitude). Maior
   parcela: cascade replace_all + adaptações tests (~40%);
   Layouter Shape emit activação (~25%); helper expandido +
   testes (~15%); audit C1 + anotações (~20%).

8. **`P252.div-N` cenários antecipados em §2.7**. Activar se:
   - Variants Stroke adicionais não-listados (`P252.div-1`).
   - Vanilla overhang per-shape-kind diferente (`P252.div-2`).
   - Vanilla BoxElem-only (`P252.div-3` — recommendation:
     manter struct field uniforme mas restringir activação
     a Boxed arm).
   - Baseline ≠ 2294 (`P252.div-4`).

9. **Cita ADR-0082 PROPOSTO explícitamente — TERCEIRA citante
   N=3 limiar atingido**. Relatório P252 §"Citação ADR-0082"
   lista 4 critérios verificados (paridade P250+P251 relatório
   §"Citação ADR-0082"):
   1. Storage prévio ✓ (scope-out P156H "rejeitados em
      `native_box` com erro hard" → P252 promoção real graded).
   2. Consumer Layouter pre-promoção graded ✓ (scope-out
      rejeitado = graded).
   3. Paridade vanilla referência empírica ✓ (audit §2.5).
   4. Backward compat literal ✓ (default `false` Rust;
      sentinelas P252).

   **Validação ADR-0082 N=3 citante atingida** — **limiar
   interno N=3 atingido** (paridade ADR-0065 P156K via
   P156J/P157A/P157B EM VIGOR). **Promoção ADR-0082 → EM
   VIGOR humana possível pós-P252**.

10. **Marco "Boxed A.4 COMPLETO 6/6"**. P252 fecha categoria
    inteira de scope-outs Boxed per P156H original. Documentar
    em relatório §"Marco P252" como milestone conceptual:
    **segundo variant Content com 100% dos scope-outs originais
    fechados cumulativamente** (após Block P250). Patamar
    interno "Category A.4 Block + Boxed COMPLETOS" reforçado.

11. **Anti-inflação 44ª aplicação cumulativa** pós-P205D
    preservar: Opção β L0 minimal (`geometry.md` hash
    propagado) + Opção α extensão field-by-field (+1 field
    em struct primitivo) + Opção α activação consumer real
    (6 arms Shape emit) + Opção α reuso `extract_stroke`
    expandido (não duplicado) + Opção α default zero-impact
    construtor Rust (backward compat literal estrita N=2
    cumulativo) + Opção α anotação cumulativa minimal ADRs
    (0061+0079+0080+0054+**0082 citação terceira**) + Opção
    α sub-padrão N=1 inaugurado "refactor cross-cutting
    entity primitivo" + Opção α Boxed A.4 COMPLETO marco
    interno.

12. **Divergência consciente default `overhang: false`
    documentada explícitamente** em ADR-0054 §"Promoções
    reais cumulativas" tabela P252 + L0 prompt `geometry.md`
    §"Default cristalino divergente P252". Pattern emergente
    "construtor Rust low-level zero-impact + stdlib parse
    paridade vanilla" candidato a sub-padrão futuro se ocorrer
    de novo (N=1 inaugurado de facto P252; precedentes
    conceptuais Block.breakable P156G + Boxed.baseline P156H
    são análogos mas distintos — defaults zero-impact construtor
    Rust **paridade** com vanilla, não divergentes).
