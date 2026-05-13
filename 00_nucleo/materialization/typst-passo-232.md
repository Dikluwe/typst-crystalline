# Passo 232 — A.5 `Place` per-cell alignment override (Fase 5 Layout candidata Categoria A 5/5; **fecha Categoria A**; terceira aplicação automática ADR-0080 EM VIGOR)

**Série**: 232 (décimo-oitavo sub-passo Layout pós-M9c;
**quinto e último sub-passo Categoria A Fase 5 Layout
candidata** per ADR-0079 PROPOSTO; **fecha Categoria A
5/5 estructuralmente**; terceira aplicação automática
ADR-0080 EM VIGOR pós-P229).
**Marco**: nenhum status ADR (Categoria A fecha dentro de
ADR-0079 PROPOSTO sem transição); **pattern emergente
"fecho categoria completa dentro de ADR PROPOSTO sem
transição" N=1 inaugurado P232**; **pattern emergente
"sub-passo sem novos fields; só lógica precedence" N=1
inaugurado P232** — distinto cumulativo de A.1-A.4 que
adicionaram fields.
**Tipo**: refino algorítmico puro a arm existente
`Content::Place` em `layout/mod.rs`; **zero fields novos**;
**zero novos variants**; **zero novas stdlib funcs**;
apenas lógica resolução precedência Place vs Grid/Table
align via `.or()` por eixo.
**Magnitude**: S+ (~1h; paridade diagnóstico P226 S+).
**Pré-condição**: P231 concluído (A.4 outset/radius/clip
Block+Boxed; 2101 verdes; 0 violations; saldo DEBTs 12;
ADR-0079 Categoria A 4/5; segunda aplicação automática
ADR-0080 EM VIGOR); humano fixou A.5 (decisão literal
pós-P231 §8); `Content::Place { alignment, dx, dy, scope,
float, clearance, body }` baseline P84.5/P84.6 + P223 (7
fields); `Align2D` baseline P84.5 (struct + Option permite
vazio); `Content::Grid.align: Option<Align2D>` baseline
P224.A; `Content::Table.align: Option<Align2D>` se existir
(audit C1; provável também presente P224 ou paralelo);
`extract_alignment(args, default)` helper baseline P84.5;
Pattern "precedência per-X via `.or()` resolution" N=1
baseline P230 (GridCell over Grid).
**Output**: 1 ficheiro relatório curto + código alterado em
~2-3 ficheiros L1 (refactor mínimo) + L0 NÃO tocado
(terceira aplicação automática ADR-0080 EM VIGOR) +
inventário 148 footnote ⁵¹ + ADR-0079 anotação **Categoria
A 5/5 ✓ fechada estructuralmente** + saldo Fase 5: 5/13-15
sub-passos materializados.

---

## §1 Trabalho

P226 diagnóstico §"Categoria A" linha A.5 marcou literal:
**"Place per-cell alignment override (S+)"**. Releitura
honesta do diagnóstico revela que A.5 não adiciona fields
a Place (já tem `alignment: Align2D` baseline P84.5) nem
adiciona fields a GridCell (align per-cell é Categoria
B.3 distinta per P230 escopo). 

**A.5 = lógica de precedência**:
- Place baseline tem `alignment: Align2D` field obrigatório.
- `Align2D` baseline é struct `{ h: Option<HAlign>, v:
  Option<VAlign> }` que **permite vazio** (`(None, None)`).
- Grid baseline P224.A tem `align: Option<Align2D>` field.
- **Place dentro Grid context**: precedência por eixo
  independente — Place explícito por eixo override Grid;
  Place vazio por eixo herda Grid.

**P232 materializa A.5**:
- **Zero fields novos** em Place/Grid/Table/Cell.
- **Lógica resolução effective alignment** dentro do arm
  `Content::Place` em `layout/mod.rs`: 
  - `effective_h = place.alignment.h.or(grid_align.and_then(|a| a.h))`.
  - `effective_v = place.alignment.v.or(grid_align.and_then(|a| a.v))`.
- **Renderização paridade P84.5/P84.6 + P223 preservada**
  literal; apenas alignment efectivo usado em vez de
  `place.alignment` directo.
- **Place fora Grid context** (cell_origin_* todos None)
  preserva comportamento baseline (Place.alignment usado
  directamente; sem Grid para herdar).

**Decisão arquitectural central — 8 decisões fixadas**:

### Decisão 1 — Interpretação A.5 Opção α (lógica precedence)

3 opções consideradas:

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | Place precedence override Grid align (zero fields novos; só lógica resolução) | Subset minimal; paridade pattern P230 GridCell precedence; magnitude S+ |
| β | Place sintaxe nova `grid.cell(align: ...)` | Refactor estructural; fora escopo S+ |
| γ | Reinterpretar como GridCell +align field | Cruza Categoria B.3; viola atomização P226 |

**Decisão fixada — Opção α** (paridade diagnóstico P226
literal S+).

### Decisão 2 — Default Place alignment "vazio" significa "herdar Grid"

`Align2D` baseline P84.5 permite vazio `(None, None)`.
P232 estabelece convenção semântica:

| Sintaxe | `Align2D` valor | Comportamento P232 |
|---------|-----------------|---------------------|
| `place(body)` | `{ h: None, v: None }` | Ambos eixos herdam Grid |
| `place(top, body)` | `{ h: None, v: Some(Top) }` | V explícito; H herda Grid.h |
| `place(center, body)` | `{ h: Some(Center), v: None }` | H explícito; V herda Grid.v |
| `place(center + top, body)` | `{ h: Some(Center), v: Some(Top) }` | Ambos override Grid |

**Decisão fixada — Opção α** (precedência por eixo
independente via `.or()`):

```rust
let effective_h = place.alignment.h.or(grid_align.and_then(|a| a.h));
let effective_v = place.alignment.v.or(grid_align.and_then(|a| a.v));
```

Pattern "precedência per-X vs container-level via `.or()`
resolution" N=1 → **2 cumulativo** (P230 GridCell over
Grid; **P232 Place per-axis over Grid**).

### Decisão 3 — Renderização Opção α inline no arm Place existente

`Content::Place` arm em `layout/mod.rs` baseline P84.6
+ P223 implementa renderização completa. **Resolução
effective alignment** acontece dentro deste arm quando
está em Grid context (cell_origin_* fields Some).

| Opção | Mecânica | Trade-off |
|-------|----------|-----------|
| **α** | Resolução inline dentro arm Place existente; renderização paridade baseline preservada | Refactor mínimo; coerente |
| β | Helper extract_effective_alignment separado | Inflacionário para uso único |
| γ | Resolver no momento da construção (eval/stdlib) | Quebra separation eval vs layout |

**Decisão fixada — Opção α** (inline; minimal change).

Substituir `place.alignment.h.unwrap_or(default)` por
`effective_h.unwrap_or(default)` (e idem v). Audit C1
determina expressões exactas pre-existentes.

### Decisão 4 — Stdlib parsing preservado

`native_place` baseline P84.5/P84.6 + P223 NÃO precisa
mudança. `extract_alignment(args, Align2D::default())`
já retorna `Align2D::default() = (None, None)` se ausente.

**Decisão fixada — Opção α stdlib NÃO modificado**:
sintaxe utilizador preservada literal (paridade vanilla
preservada baseline).

### Decisão 5 — Tests E2E precedência

Crítico testar precedência por eixo:
- Grid sem align; Place sem alignment → comportamento
  baseline preservado (top-left default implícito).
- Grid align=center; Place sem alignment → cell center
  (herda Grid).
- Grid align=center; Place top → vertical top override;
  horizontal center herda.
- Grid align=center; Place top+right → both override.
- Place fora de Grid (cell_origin_* None) → comportamento
  baseline preservado.

**Decisão fixada — 5 tests E2E precedência explícitos**.

### Decisão 6 — `Content::Table` align field paralelo

Audit C1 crítico: P224.A adicionou `align: Option<Align2D>`
a Grid. **`Content::Table` recebeu mesmo field paralelo?**

3 opções:
- α — Audit confirma Table tem align (paridade Grid P224.A);
  P232 lógica precedence aplica-se a Place dentro Grid OU
  Table context.
- β — Audit revela Table NÃO tem align field (P224.A
  apenas Grid).
- γ — Não importa para P232 — Place dentro Table delegate
  `layout_grid` (P157A); Table.align se existir é passado
  via delegate.

**Decisão fixada (sujeita a audit C1)**: se Table tem
align, lógica precedence cobre Table context automaticamente
via delegate. Se NÃO, P232 escopo limitado a Grid context;
Table refino paralelo align field candidato refino XS
separado.

### Decisão 7 — Fecho Categoria A 5/5

P232 é último sub-passo Categoria A Fase 5. Pós-P232,
Categoria A 5/5 completa.

**Marco interno implícito**: Categoria A Fase 5 Layout
candidata fechada estructuralmente. ADR-0079 §"Categoria
A" anotada como **5/5 ✓ fechada estructuralmente** sem
transição ADR-0079 status (sub-categorias B/C/D ainda
pendentes per roadmap completo).

**Decisão fixada — anotação ADR-0079 explícita "Categoria
A 5/5 ✓"** sem promoção ADR status. **Pattern emergente
"fecho categoria completa dentro de ADR PROPOSTO sem
transição" N=1 inaugurado P232** — paridade conceitual
mas distinto de "encerramento Fase" §3.0duodecies/§3.0terdecies
P221/P225 que envolveram transições ADR.

### Decisão 8 — L0 NÃO tocado (ADR-0080 EM VIGOR
aplicação automática N=3)

**Decisão fixada — aplicação automática terceira pós-P229**:

P232 é refactor algorítmico puro (zero fields; zero
variants; zero stdlib). ADR-0080 EM VIGOR §"Decisão"
aplica-se automaticamente. Pattern "aplicação automática
ADR EM VIGOR sem decisão explícita por sub-passo" N=2 →
**3 cumulativo**.

L0 prompts NÃO tocados.

Reuso de dados (sem recolha nova):

- `Content::Place { alignment, dx, dy, scope, float,
  clearance, body }` baseline P84.5/P84.6 + P223 (7
  fields).
- `Align2D { h: Option<HAlign>, v: Option<VAlign> }`
  baseline P84.5 (permite vazio).
- `Content::Grid.align: Option<Align2D>` baseline P224.A.
- `Content::Table.align: Option<Align2D>` (audit C1;
  provável baseline P224 ou ortogonal).
- `extract_alignment(args, default)` helper baseline P84.5.
- `cell_origin_x/y/w` fields Layouter baseline P84.6
  (sinal de Grid context).
- Pattern "precedência per-X via `.or()` resolution" N=1
  baseline P230.
- ADR-0080 EM VIGOR aplicação automática N=2 baseline
  P230+P231.

---

## §2 Cláusulas (10 — atomização paridade P230 reduzida)

### C1 — Inventário pré-P232: confirmar Place + Grid.align + Table.align + arm Place layouter

Auditoria empírica:

```
grep -n "Place {" 01_core/src/entities/content.rs
grep -n "Align2D" 01_core/src/entities/layout_types.rs
grep -n "align:" 01_core/src/entities/content.rs | grep -i "Grid\|Table"
grep -B 2 -A 30 "Content::Place" 01_core/src/rules/layout/mod.rs
grep -n "cell_origin" 01_core/src/rules/layout/mod.rs
grep -n "extract_alignment" 01_core/src/rules/stdlib/
```

Hipótese:
- `Content::Place` 7 fields baseline P84.5/P84.6 + P223.
- `Align2D` struct + Option baseline P84.5.
- `Grid.align: Option<Align2D>` baseline P224.A ✓.
- `Table.align: Option<Align2D>` — **audit C1 crítico**
  (paridade P224.A ou ausente).
- Arm `Content::Place` em `layout/mod.rs` baseline P84.6
  + P223 com `cell_origin_*` save/restore pattern.
- `extract_alignment(args, default)` helper baseline P84.5.

**Decisão crítica C1**: 
1. Se Table tem align → P232 cobre Grid + Table contexts.
2. Se Table NÃO tem align → P232 escopo limitado Grid;
   Table align paralelo refino XS futuro candidato.
3. Identificar onde `cell_align` (Grid-level) está
   disponível no arm Place. Possível precisar saving
   adicional similar a `cell_origin_*` P84.6.

Sem `P232.div-N` formal se hipótese converge.

### C2 — Refactor arm `Content::Place` lógica precedence

Editar `01_core/src/rules/layout/mod.rs` arm
`Content::Place`:

```rust
Content::Place { alignment, dx, dy, scope, float: _, clearance: _, body } => {
    // ... existing P84.6+P223 logic ...

    // P232 — Resolver effective alignment via .or() per
    // axis. Place explícito por eixo override Grid; Place
    // vazio por eixo herda Grid (cell_align se Some).
    let cell_align = layouter.cell_align;  // P84.6+P224.A; saved/restored per cell
    let effective_h = alignment.h
        .or(cell_align.and_then(|a| a.h));
    let effective_v = alignment.v
        .or(cell_align.and_then(|a| a.v));
    let effective_alignment = Align2D { h: effective_h, v: effective_v };

    // Substituir uso de `alignment` por `effective_alignment` no
    // resto do arm (renderização paridade P84.5/P84.6+P223).
    // ... existing logic with `effective_alignment` ...
}
```

**Audit C1 crítico**: confirmar onde `cell_align` é salved/
restored no Layouter (paridade `cell_origin_*` P84.6).
Se não existe, precisa saving paralelo:

```rust
// P232 — Layouter ganha campo cell_align: Option<Align2D>
// (paridade cell_origin_*).
// Save/restore por cell no braço Grid (existing P84.6
// pattern; +1 field).
```

Magnitude C2: **S (~30min)** — refactor mínimo paridade
P84.6 cell_origin_* pattern.

### C3 — Possível +1 field no `Layouter` struct

Se audit C1 revelar `cell_align` não existe no Layouter:

Editar `01_core/src/rules/layout/mod.rs` (Layouter struct):

```rust
pub struct Layouter<'a> {
    // ... existing ...
    pub cell_origin_x: Option<f64>,    // P84.6
    pub cell_origin_y: Option<f64>,    // P84.6
    pub cell_origin_w: Option<f64>,    // P84.6
    pub cell_available_h: Option<f64>, // P83
    /// P232 — Grid-level align disponível em Grid context
    /// para Place herdar via `.or()`. Save/restore por cell
    /// paridade cell_origin_*.
    pub cell_align: Option<Align2D>,
}
```

Save/restore por cell no arm `Content::Grid` baseline
P84.6:

```rust
Content::Grid { ..., align, ... } => {
    let saved_cell_align = layouter.cell_align;
    layouter.cell_align = align.clone();
    // ... iter cells; arm Place herda via cell_align ...
    layouter.cell_align = saved_cell_align;  // restore
}
```

Magnitude C3: **XS (~15min)**.

### C4 — Possível paridade Table align field

Se audit C1 revelar Table NÃO tem align field:

**Decisão pragmática C4**: P232 escopo limitado a Grid
context. Table.align refino paralelo é **refino XS futuro
candidato** (não-reservado per política P158).

Se Table tem align (audit C1 OK): P232 cobre Table context
automaticamente via delegate `layout_grid` P157A.

Magnitude C4: **XS (~5min audit)**.

### C5 — Sentinelas P232

Tests P232 (paridade P230 estrutura mas reduzido):

**Unit content** (~1 test trivial):
- `p232_place_alignment_vazio_default` — `Align2D::default()
  == { h: None, v: None }`.

**Unit layouter** (~3 tests):
- `p232_place_dentro_grid_herda_align_full` — Grid align
  center; Place sem alignment → Place renderiza centered.
- `p232_place_dentro_grid_override_per_axis` — Grid align
  center; Place top → V top override; H center herda.
- `p232_place_fora_grid_baseline_preservado` — Place
  baseline P84.5 preservado quando cell_origin_* None.

**Layout E2E precedence** (~4-5 tests crítico):
- `p232_grid_center_place_sem_alignment_herda` — visual
  cell center.
- `p232_grid_center_place_top_v_override` — visual cell
  top + center horizontal.
- `p232_grid_top_place_center_full_override` — visual cell
  center.
- `p232_grid_sem_align_place_sem_alignment_default` —
  visual cell top-left default.
- `p232_table_align_place_herda_via_delegate` — se Table
  tem align field; senão skip.

Total tests P232: **~9 tests** (1+3+5; ou 8 se Table.align
ausente). Esperado pós-P232: **2101 + 9 = ~2110 verdes**.

### C6 — L0 NÃO tocado (ADR-0080 EM VIGOR aplicação
automática N=3)

**Decisão fixada — aplicação automática**: terceira
aplicação automática pós-promoção P229. Pattern "aplicação
automática ADR EM VIGOR sem decisão explícita por sub-passo"
N=2 → **3 cumulativo**.

L0 prompts NÃO tocados.

### C7 — Verificação tests workspace + lint

```
cargo test --workspace 2>&1 | tail -3
crystalline-lint .
crystalline-lint --fix-hashes .
```

Critério:
- 2101 verdes pré-P232 + ~9 novos = **~2110 verdes**.
- 0 violations preservadas.
- Hashes propagados em ~2-3 ficheiros L1 (`layout/mod.rs`
  possível +1 field Layouter; possível ajuste arm Grid).
- L0 prompts não tocados — "Nothing to fix".

**Risco regressão**: Place tests pre-existentes P84.5/
P84.6 + P223. Hipótese N=0-2 adaptações (Place baseline
preservado quando fora Grid context; mudança aplica-se só
em Grid context com align Some).

### C8 — Inventário 148 footnote ⁵¹

**§A.5 Layout entradas `place(...)` + `grid(...)`**:
footnotes existing. Adicionar **footnote ⁵¹** documentando:
- A.5 materializado (último Categoria A Fase 5 fecha
  categoria 5/5).
- 8 decisões fixadas.
- Lógica precedência Place per-axis vs Grid via `.or()`.
- Zero fields novos (paridade pattern "sub-passo só
  lógica" N=1 inaugurado P232).
- Pattern "precedência per-X via `.or()` resolution" N=1
  → 2 cumulativo (P230 GridCell; P232 Place per-axis).
- Pattern "aplicação automática ADR EM VIGOR" N=2 → 3
  cumulativo.
- **Categoria A Fase 5 fechada estructuralmente** (5/5
  sub-passos).
- ADR-0079 §"Categoria A" anotada 5/5 ✓ sem transição
  status.

### C9 — ADR-0079 anotação Categoria A 5/5 ✓ fechada

Editar ADR-0079 com bloco P232:

```markdown
### P232 anotação — Categoria A sub-passo 5 (Place per-cell
alignment override); **Categoria A 5/5 ✓ fechada
estructuralmente**

**Categoria A**: 5/5 sub-passos materializados ✓ **FECHADA**.
- A.1 stroke (P227) ✓.
- A.2 fill (P228) ✓.
- A.3 per-cell GridCell+TableCell (P230) ✓.
- A.4 outset/radius/clip Block+Boxed (P231) ✓.
- **A.5 Place per-cell alignment override (P232) ✓**.

Trabalho P232:
- **Zero fields novos** em Place/Grid/Table/Cell.
- Lógica precedência per-axis via `.or()` no arm
  `Content::Place` (Grid context).
- Possível +1 field `cell_align` no Layouter (audit C1).
- ~9 tests novos.
- **Terceira aplicação automática ADR-0080 EM VIGOR**.

Patterns consolidados:
- "Precedência per-X vs container-level via `.or()`
  resolution" N=1 → **2 cumulativo** (P230 GridCell; P232
  Place per-axis).
- "Aplicação automática ADR EM VIGOR" N=2 → 3 cumulativo.
- **"Fecho categoria completa dentro de ADR PROPOSTO sem
  transição" N=1 inaugurado P232**.
- **"Sub-passo sem novos fields; só lógica precedence"
  N=1 inaugurado P232**.

Status ADR-0079 mantido PROPOSTO (5/13-15 sub-passos
cumulativos; Categoria A 5/5 ✓; B/C/D pendentes).

**Marco interno implícito Categoria A fechada
estructuralmente** — próximo sub-passo pode ser:
- Pivot Categoria B (algorítmicos: B.1 DEBT-34d; B.2
  consumer geometric; B.3 per-cell align/inset/breakable).
- Pivot Categoria C (estruturais reabrindo: C.1 Place
  float real; C.2 multi-region completa).
- Pivot Categoria D (runtime queries: D.1 state desbloqueia
  ADR-0066 IMPLEMENTADO).
- Pivot outro módulo (Visualize/Text/Model).
```

### C10 — Critério aceitação P232

- ~9 tests novos verdes.
- 2101 tests pre-existentes preservados (após N=0-2
  adaptações se necessárias).
- 0 violations.
- **Zero fields novos** em Place/Grid/Table/Cell.
- Possível +1 field `cell_align` no Layouter (paridade
  cell_origin_*).
- Lógica precedência per-axis funcional.
- ADR-0079 Categoria A 5/5 ✓ anotado **fechada
  estructuralmente**.
- ADR-0080 EM VIGOR aplicação automática N=2 → 3.
- Cobertura Layout 89% preservada (refino qualitativo).

---

## §3 Output

1 ficheiro relatório curto:
`00_nucleo/materialization/typst-passo-232-relatorio.md`.

Estrutura (~5-7 KB; magnitude S+ justifica menor que
P231) com 8 §s:

- §1 O que foi feito (sumário 3-5 linhas).
- §2 Inventário pré-P232 + audit Table.align + arm Place
  (C1).
- §3 Refactor arm Place + possível Layouter +1 field (C2+C3).
- §4 Resolução effective alignment via `.or()` per axis
  + decisão Opção α (C2 detalhe).
- §5 Decisões substantivas (8 decisões fixadas) + terceira
  aplicação automática ADR-0080 EM VIGOR.
- §6 Resultados verificação (C5+C7).
- §7 Inventário 148 footnote ⁵¹ + ADR-0079 anotação
  **Categoria A 5/5 ✓ fechada estructuralmente** (C8+C9).
- §8 Próximo sub-passo (P233 candidatos: pivot B/C/D ou
  pivot outro módulo; **Categoria A fechada permite reset
  decisão**).

Código alterado:
- **Editado**: `01_core/src/rules/layout/mod.rs` (arm
  Place refactor + possível Layouter +1 field cell_align
  + arm Grid save/restore paralelo cell_origin_*).
- **Editado**: `01_core/src/rules/layout/tests.rs` (+~5
  E2E precedence + ~3 unit).
- **Possivelmente editado**: `01_core/src/entities/content.rs`
  (+1 unit test se applicável).
- **Editado**: `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md`
  (footnote ⁵¹ P232).
- **Editado**: `00_nucleo/adr/typst-adr-0079-layout-fase-5-roadmap.md`
  (+ anotação Categoria A 5/5 ✓ fechada estructuralmente).

**Sem novos ficheiros**.

---

## §4 Não-objectivos

- Adicionar fields a Place — Place já tem `alignment`
  baseline P84.5; A.5 é lógica não-field.
- Adicionar `align` a GridCell — Categoria B.3 separada
  (per-cell algorítmico).
- Add `align` field a Table se ausente — refino XS
  separado (paridade Grid P224.A).
- Implementar precedência multi-nível (Grid + cell +
  Place) — Categoria B.3 candidato cumulativo.
- Promover ADR-0079 PROPOSTO → IMPLEMENTADO — só pós
  Categorias A + B + C + D completas ou scope-out parcial
  formal.
- Promover ADR-0066 PROPOSTO → IMPLEMENTADO — só pós-D.1
  state materializa.
- Tocar em L0 prompts — ADR-0080 EM VIGOR aplicação
  automática.
- Show rules `#show place: ...` — fora escopo Fase 5.
- Reabrir decisões arquiteturais — A.5 é Categoria A
  (sem reabrir).
- Marco cirúrgico blueprint §3.0quinquadecies para
  fecho Categoria A — anti-inflação (pattern §3.0... para
  fechos/aberturas de Fase, não categorias internas).
- Promover patterns emergentes consolidados (refino paralelo
  N=4; semantic adiada N=7; bool simples N=3; Smart→Option
  N=10) a ADRs meta — passos administrativos XS separados
  futuros se humano priorizar (paridade P229 pattern).

---

## §5 Riscos a evitar

1. **`cell_align` não saved no Layouter** — audit C1
   crítico. Mitigação: criar +1 field paralelo
   `cell_origin_*` save/restore pattern.
2. **`Table.align` ausente** — audit C1. Mitigação:
   limitar escopo P232 a Grid context; Table align refino
   XS futuro candidato.
3. **Place fora Grid baseline quebrado** — Place sem
   alignment dentro Grid muda; Place sem alignment fora
   Grid preserva. Mitigação: lógica `.or()` só aplica se
   `cell_align.is_some()`; senão usa baseline directo.
4. **Cantos casos `Align2D::default()`** — `(None, None)`
   semanticamente "vazio". Renderização baseline precisa
   defaults explícitos (top-left implícito). Mitigação:
   testar caso "Grid sem align; Place sem alignment" =
   default top-left preservado.
5. **Tests pre-existentes Place P84.5/P84.6+P223**:
   hipótese N=0-2 adaptações. Place fora Grid preserva
   baseline; Place dentro Grid sem align Grid preserva
   baseline. Adaptação só se test usa Place dentro Grid
   com align Grid (caso novo).
6. **L0 tocado por engano** — terceira aplicação
   automática ADR-0080 EM VIGOR. Mitigação: §5 risco 6
   explícito + §C6 fixa não tocar.
7. **Refino paralelo Table align infiltrar P232** —
   tentação por consistência. Decisão 6 fixa
   audit-dependente; refino XS separado se necessário.
8. **Marco cirúrgico blueprint pelo fecho Categoria A**:
   tentação por simetria com §3.0duodecies/§3.0terdecies.
   Rejeitada — pattern §3.0... para fechos/aberturas de
   Fase, não categorias internas dentro de Fase.
9. **Promoção ADR-0079 PROPOSTO → IMPLEMENTADO por "fecho
   Categoria A"**: tentação. Rejeitada — ADR-0079
   §"Critério" requer **todos** sub-passos OU scope-out
   parcial formal; Categoria A 5/5 ≠ ADR-0079 IMPLEMENTADO.
10. **Magnitude exceder S+ (~1h)**: P230/P231 chegaram
    ≤ estimado. P232 mais simples (zero fields novos).
    Hipótese real S (~45min) provável.
11. **Pattern "sub-passo sem novos fields" N=1 promoção
    prematura**: tentação por inaugurar pattern + promover
    formal. Rejeitada — N=1 não atinge limiar formalização
    (N=3-4); pattern registado sem promoção.
12. **Reescrever §"Aplicações cumulativas" ADR-0080
    com P232**: P232 valida ADR-0080 EM VIGOR pela terceira
    vez automática. Tentação por anotar formal a cada
    aplicação. Rejeitada — anti-inflação; aplicações
    cumulativas em footnote ⁵¹ inventário 148 + ADR-0079
    suficiente; ADR-0080 §"Aplicações" preserva 9 entradas
    P217-P228 históricas.

---

## §6 Hipótese provável

C1 confirmará Place 7 fields baseline; Align2D struct +
Option vazio; Grid.align baseline P224.A; **Table.align
provavelmente também tem field** (paridade P224 implícita
ou ortogonal); arm Place layouter destrutura 7 fields
existing.

C2 refactor arm Place: `effective_h/v` via `.or()`;
substituir uso `alignment` por `effective_alignment` no
resto do arm.

C3 (se necessário) Layouter +1 field `cell_align: Option<Align2D>`
paralelo cell_origin_* save/restore.

C4 audit Table.align: provavelmente OK; senão refino XS
futuro.

C5 criará ~9 tests novos (1+3+5).

C6 NÃO tocará L0.

C7 reportará ~2110 verdes; 0 violations; possíveis N=0-2
adaptações.

C8+C9 reclassificará footnote ⁵¹ + ADR-0079 anotação
**Categoria A 5/5 ✓ fechada estructuralmente**.

C10 verifica critério aceitação.

Custo real: **S (~45min)** — zero fields novos +
refactor mínimo arm Place.

Mas é hipótese, não decisão. C1-C10 fixam-se
empíricamente.

---

## §7 Particularidade P232

P232 é estruturalmente distinto na trajectória pós-M9c:

- **Quinto e último sub-passo Categoria A Fase 5 Layout
  candidata** — **fecha Categoria A 5/5 estructuralmente**.
  Pós-P232, Categoria A não tem mais sub-passos
  identificados em ADR-0079 §"Próximos passos".
- **Pattern emergente "fecho categoria completa dentro
  de ADR PROPOSTO sem transição" N=1 inaugurado P232** —
  distinto de §3.0duodecies P221 + §3.0terdecies P225 que
  envolveram transições ADR PROPOSTO → IMPLEMENTADO.
- **Pattern emergente "sub-passo sem novos fields; só
  lógica precedence" N=1 inaugurado P232** — distinto
  cumulativo de A.1-A.4 que adicionaram fields cumulativos
  (stroke/fill/outset/radius/clip/per-cell). P232 é
  **trabalho algorítmico puro**.
- **Terceira aplicação automática ADR-0080 EM VIGOR
  pós-promoção P229** — pattern N=2 → 3 cumulativo.
  Validação contínua de regra metodológica em prática.
- **Pattern "precedência per-X via `.or()` resolution"
  N=1 → 2 cumulativo** (P230 GridCell over Grid; **P232
  Place per-axis over Grid**). Pattern atinge limiar
  formalização N=3-4 candidato; promoção formal candidato
  XS futuro.
- **Cobertura Layout per metodologia preservada 89% real**
  — A.5 é refino qualitativo algorítmico.
- **Anti-inflação 24ª aplicação cumulativa** pós-P205D —
  Opção α lógica não-field + Opção α inline arm + Opção α
  stdlib preservada + Opção γ L0 automático + sem helper
  novo + sem marco blueprint + sem promoção ADR-0079 +
  sem promoção patterns emergentes.

Por isso §5 risco 8 (marco cirúrgico blueprint inflacionário)
é o mais provável simbolicamente. Tentação: "Categoria A
fechada paralelo a fecho Fase; marca §3.0quinquadecies".
**Defesa**: pattern §3.0... para fechos/aberturas de
**Fase**, não categorias internas dentro de Fase.
Categoria A fecha **dentro** de Fase 5 PROPOSTO; não fecha
Fase 5 inteira.

**Critério de aceitação P232**:
- ~9 tests novos verdes.
- 2101 tests pre-existentes preservados (após N=0-2
  adaptações).
- 0 violations.
- Zero fields novos Place/Grid/Table/Cell.
- Possível +1 field Layouter `cell_align`.
- Lógica precedência per-axis funcional.
- **Categoria A 5/5 ✓ fechada estructuralmente**.
- ADR-0079 anotado Categoria A fechada sem transição
  status.
- ADR-0080 EM VIGOR aplicação automática N=2 → 3.
- Cobertura Layout 89% preservada.

**Estado pós-P232 esperado**:
- Tests workspace: 2101 → **~2110 verdes** (+9).
- Stdlib funcs: 60 preservado.
- Content variants: 59 preservado.
- Value variants: 55 preservado.
- Place fields: 7 preservado (zero novos).
- Grid fields: 10 preservado.
- Table fields: 5 preservado.
- GridCell fields: 7 preservado.
- TableCell fields: 7 preservado.
- Block fields: 8 preservado.
- Boxed fields: 8 preservado.
- Layouter fields: +1 possível (`cell_align`).
- §A.5 distribuição: `12/4/2/0/0 = 18` preservada.
- Cobertura Layout per metodologia: **89% preservada**.
- Cobertura user-facing total: 67% preservada.
- ADR-0066 PROPOSTO; ADR-0061 IMPLEMENTADO; ADR-0078
  IMPLEMENTADO; ADR-0079 PROPOSTO (5/13-15; Categoria A
  5/5 ✓ fechada); ADR-0080 EM VIGOR.
- Saldo DEBTs: 12 preservado.
- **24 aplicações cumulativas anti-inflação** pós-P205D.
- **Pattern "L0 minimal para refactors" aplicação
  automática N=3 cumulativo** (P230+P231+P232).
- **Pattern "precedência per-X via `.or()`" N=2 cumulativo**
  (P230 GridCell over Grid; P232 Place per-axis over Grid).
- **Pattern "fecho categoria completa dentro de ADR
  PROPOSTO sem transição" N=1 inaugurado P232**.
- **Pattern "sub-passo sem novos fields; só lógica
  precedence" N=1 inaugurado P232**.
- **Categoria A Fase 5 Layout: 5/5 ✓ fechada
  estructuralmente** — próximo sub-passo pode pivot
  Categoria B/C/D ou outro módulo.
- **Fase 5 Layout candidata: 5/13-15 sub-passos
  materializados** (~33%-38% cumulativo; Categoria A
  100% interna).
