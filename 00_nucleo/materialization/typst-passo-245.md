# Spec do passo P245 — M7+4 Place float real (promoção graded P223 → semantic real; algoritmo defer ao topo/fundo da página + clearance vertical; fecha ADR-0081 IMPLEMENTADO total 5/5 + desbloqueia Categoria C.1 Fase 5 Layout)

**Data**: 2026-05-14.
**Tipo**: refino consumer Layouter — promoção graded → real
semantic; **algoritmo defer ao topo/fundo da página + clearance
vertical**. Activa fields `float: bool` + `clearance: Option<Length>`
que P223 (2026-05-13) armazenou em `Content::Place` mas
Layouter consumer ignorou literal (`float: _, clearance: _` em
`mod.rs:916`).
**Magnitude planeada**: **L (~5-8h)** — paridade estimativa
ADR-0081 §"Escopo" linha 71. Algoritmo float vanilla é
trabalho contido (Layouter consumer + buffer floats pendentes
+ flush antes de page break + clearance Y), não exige tipo
entity novo nem ADR nova.
**Marco**: **último sub-passo M7+ pendente**; fecha **ADR-0081
IMPLEMENTADO total 5/5** (M7+1 ✓ + M7+2 ✓ + M7+3 ✓ via cumulativo
P243 fase (a) + Linha A pré-existente + M7+4 ✓ + M7+5 ✓);
**desbloqueia Categoria C.1 Fase 5 Layout** (ADR-0079 PROPOSTO);
**reclassifica `place(...)` §A.5 Layout `implementado⁺ ⁵ ⁴⁴`
→ `implementado⁺` literal** (remove notas de rodapé "float
armazenado mas ignorado").

---

## §1 O que será feito

P245 promove a estrutura graded P223 a semantic real no Layouter.

### Estado pré-P245 (factual, audit empírico §2)

`Content::Place` tem **7 fields** (linha 414 em
`01_core/src/entities/content.rs`):

```rust
Place {
    alignment: Align2D,
    dx:        f64,
    dy:        f64,
    scope:     PlaceScope,
    float:     bool,            // P223; armazenado; ignorado em Layouter
    clearance: Option<Length>,  // P223; armazenado; ignorado em Layouter
    body:      Box<Content>,
}
```

Layouter consumer em `01_core/src/rules/layout/mod.rs:916`:

```rust
Content::Place { alignment, dx, dy, scope,
                 float: _, clearance: _, body } => {
    // P84.5+P84.6 lógica de scope+ancoragem;
    // float+clearance ignorados literal
    ...
}
```

`native_place` aceita `float: bool` (default `false`) + `clearance:
Length` (default `None`). DEBT-37 §"Divergência" fechada P223:
`scope: Parent + float: true` exigido (erro hard caso contrário).

### Estado pós-P245 (objectivo)

Layouter consumer activa semantic real:

1. **Floats são deferred ao topo ou fundo da página** (não
   renderizados in-place).
2. **`alignment.y` selecciona destino**: `Top` → topo da página;
   `Bottom` → fundo da página; `Horizon` → fundo (paridade vanilla
   default `bottom`).
3. **`alignment.x` aplica-se na linha do float** (left/center/
   right horizontal na linha vertical destino).
4. **`clearance: Some(Length)` adiciona espaço vertical** entre
   conteúdo de fluxo regular e área de float (anti-colisão).
5. **Conteúdo regular respeita reserva de espaço**: cursor.y
   máximo da página fica reduzido pela altura do float
   (top-aligned float reserva espaço inicial; bottom-aligned
   reserva espaço final).
6. **Floats pendentes flush antes de page break**: ao chamar
   `Layouter::new_page()`, floats da página actual emitidos
   antes de iniciar nova página. Floats acumulados na nova
   página resetam ao zero.

### Tests esperados

Tests P245 novos estimados: **10-15** (range L magnitude).

- 4-5 unit Layouter (placement float top/bottom; clearance Y;
  scope Parent+float real; horizontal alignment dentro do
  destino).
- 3-4 unit content (cascata fields preservada).
- 2-3 E2E layout (float top + body; float bottom + body; mistura
  com pagebreak + float pendente).
- 1-2 E2E regressão (Place sem float continua exactamente como
  P84.6+P223 — body in-place via cursor; tests baseline 2198
  preservados).

**Workspace pós-P245**: **2198 → ~2208-2213 verdes** (range
+10-15 paridade L magnitude).

### Adaptações pre-existentes

Estimativa **N=0-3** adaptações tests pré-existentes
(provavelmente N=0; P223 já restaurou DEBT-37 sentinela
`scope: Parent + float: true`; tests baseline P84.6 já
adaptados em P223 a chamarem com `float: true` onde
necessário).

**Se audit C1 detectar tests baseline que assumem float ignorado**
(ex: test que faz `place(..., float: true, ...)` e verifica
posicionamento in-place), criar `P245.div-N` formal antes de
adaptar.

---

## §2 Verificação empírica pré-P245 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=7 → 8 cumulativo

Audit C1 obrigatório bloqueante pós-P236.div-1 (lição refinada
P244 N=7 → 8 cumulativo). Refinamento procedural P244 aplicável
literal: grep variants `Content::*` candidatas antes de assumir
ausência. Aqui não há variants novos (P245 é refino consumer),
mas a lição expande-se a "grep fields/arms já implementados
antes de assumir trabalho original":

### §2.1 Inventário Content::Place pré-P245 (factual; já confirmado P244)

`grep -n "Place {\|Self::Place" 01_core/src/entities/content.rs`:

```
414:    Place {
1431:    Self::Place { body, .. } => body.plain_text(),
1604:    (Self::Place { alignment: aa, dx: dxa, dy: dya, scope: sa,
1606:     Self::Place { alignment: ab, dx: dxb, dy: dyb, scope: sb,
2030:    Content::Place { alignment, dx, dy, scope, float, clearance, body }
2308:    Content::Place { alignment, dx, dy, scope, float, clearance, body }
3732:    let p = Content::Place {     // unit test float storage
3753:    let p = Content::Place {     // unit test clearance storage
3774:    let mk = |float, clearance|  // unit test cascata
3794:    let p = Content::Place {     // unit test map preservation
```

**7 fields confirmados** (alignment, dx, dy, scope, float,
clearance, body). Arms cascata em PartialEq (1604/1606),
map_content (2030), map_text (2308), plain_text (1431) já
incluem `float` + `clearance` desde P223.

### §2.2 Inventário Layouter consumer pré-P245 (factual)

`grep -n "Content::Place" 01_core/src/rules/layout/mod.rs`:

```
156:    /// `Content::Place { scope: Column, .. }` ancora à célula.
162:    /// disponível para `Content::Place` herdar via `.or()` per eixo
916:    Content::Place { alignment, dx, dy, scope,
                         float: _, clearance: _, body } => {
```

**Consumer ignora `float` + `clearance` literal** (`_`).
**Confirma trabalho real pendente** — não é cenário tipo
P243→P244 onde trabalho já está feito.

### §2.3 Layouter fields pré-P245

`grep -n "cell_origin\|cell_available" 01_core/src/rules/layout/mod.rs | head -10`:

Confirmar presença de `cell_origin_x`, `cell_origin_y`,
`cell_origin_w`, `cell_available_h` (P83+P84.6). **Hipótese
inicial**: presentes; usados pelo scope Column dentro de Grid.

**Adicional necessário P245**: novo field
`floats_pending: Vec<DeferredFloat>` em Layouter (buffer dos
floats por página; flush em `new_page()`).

### §2.4 Verificação adicional crítica — algoritmo float vanilla

`ls lab/typst-original/crates/typst-layout/src/flow/`:

Procurar `place.rs` ou `placed.rs` para referência vanilla
exacta. Vanilla layout-flow float é implementado em
`PlacedChild` (referenciado em P244 audit "Reabertura 1 Opção
B P219 graded"). Hipótese semantic vanilla:

1. Float colectado durante walk-time em `FlowState.floats`.
2. Layout pass deferre emissão de float até final page
   layout (top floats antes do flow body; bottom floats
   após).
3. Clearance Y entre flow body e área float aplicado per
   `clearance: Length`.

Cristalino single-pass simplifica: floats colectados em
`Layouter::floats_pending` durante layout sequencial; flush
em `new_page()` antes de transição.

### §2.5 Tests pré-P245 baseline

```
cargo test --workspace
```

Esperado: **2198 verdes** (estado pós-P244 administrativo).

### §2.6 Sub-tests P223 baseline

`grep -n "fn .*place.*float\|fn .*place.*clearance" 01_core/src/entities/content.rs 01_core/src/rules/stdlib/*.rs 01_core/src/rules/layout/*.rs | head -20`:

Identificar tests P223 que validam storage de `float` +
`clearance`. **Devem preservar** pós-P245 (storage continua
a ser cascata; só semantic activa).

### §2.7 DEBT-37 sentinela ainda activa

`grep -n "scope: Parent.*float: true\|DEBT-37" 01_core/src/`:

DEBT-37 sentinela "`scope: Parent + float: true` exigido"
restaurada P223 — preservar literal pós-P245.

### `P245.div-N` antecipadas — nenhuma esperada

- Se grep §2.1 revelar fields adicionais não documentados →
  `P245.div-1` formal investigação humana.
- Se grep §2.2 revelar consumer já parcialmente activado →
  `P245.div-2` (re-escopo do trabalho real).
- Se tests baseline §2.5 não estão a 2198 → `P245.div-3`
  reconciliação prévia.

---

## §3 Decisões fixadas P245 — 9 decisões

### Decisão 1 — Buffer `floats_pending: Vec<DeferredFloat>` no Layouter

Novo field `floats_pending: Vec<DeferredFloat>` em `Layouter`,
onde:

```rust
struct DeferredFloat {
    alignment: Align2D,      // Top/Horizon/Bottom em y selecciona destino
    body_items: Vec<FrameItem>,  // body já layouted, pronto para emit
    body_height: f64,        // altura ocupada (para reservar)
    body_width:  f64,        // largura ocupada (para alignment x dentro da página)
    clearance:   f64,        // 0.0 se None; else resolvido
    scope:       PlaceScope, // Column ou Parent
}
```

Buffer reset em `Layouter::new_page()` após flush dos floats
pendentes na página actual.

**Justificação**: single-pass cristalino exige que floats sejam
acumulados durante layout sequencial e emitidos no fim da
página (antes do page break ou no `Layouter::finish`).

### Decisão 2 — Layouter arm `Content::Place { float: true, .. }` → buffer

```rust
Content::Place { alignment, dx, dy, scope,
                 float: true, clearance, body } => {
    // Validação DEBT-37 sentinela já feita em native_place
    // (scope: Parent + float: true).
    let resolved_clearance = clearance.map(|l| l.to_pt()).unwrap_or(0.0);

    // Layout body em sub-frame para medir + capturar items
    let (body_items, body_height, body_width) =
        self.layout_sub_frame_capture(body, /* width disponível */)?;

    // Push ao buffer
    self.floats_pending.push(DeferredFloat {
        alignment, body_items, body_height, body_width,
        clearance: resolved_clearance, scope,
    });

    // Cursor.y NÃO avança (paridade vanilla — float não consume flow space directo)
    // MAS: reservar espaço se top-aligned (decisão 5)
    // body Y aplicado em flush, não in-place
}
```

**Distinção crítica face a Place não-float**: Place sem float
emite items in-place via `cursor.y` actual + `dy`; Place float
emite items na fase de flush.

### Decisão 3 — Place sem float (float: false) — preservado literal

Arm `Content::Place { float: false, .. }` mantém comportamento
P84.5+P84.6 literal (anchoring scope Column/Parent + dx/dy
in-place). **Tests baseline P84.6 preservados** sem adaptação.

### Decisão 4 — Flush floats em `Layouter::new_page()` + `Layouter::finish()`

Antes de cada transição de página:

```rust
fn new_page(&mut self) {
    self.flush_pending_floats();  // novo método
    // ... lógica P156E existente
}

fn finish(&mut self) -> ... {
    self.flush_pending_floats();  // novo método; última página
    // ... lógica existente
}
```

`flush_pending_floats`:

1. Separa `floats_pending` em top-floats e bottom-floats
   conforme `alignment.y`.
2. Top-floats emitidos a `Y = margem_top + soma_clearance_cumulativa`
   (do topo da área útil).
3. Bottom-floats emitidos a `Y = page_height - margem_bottom -
   body_height - soma_clearance_cumulativa` (do fundo).
4. `alignment.x` aplica-se dentro da largura da página para
   posicionamento horizontal de cada float.
5. `floats_pending.clear()` após flush.

### Decisão 5 — Reserva de espaço para floats top-aligned

Top floats consomem espaço inicial da área útil da página. Decisão
crítica: **reserva é feita no momento de bufferização**
(`floats_pending.push`) ajustando `cursor_y_min` da página
actual, não no flush. Isto garante que flow regular não invade
a área de float.

```rust
if matches!(alignment.y, Some(VAlign::Top)) {
    self.cursor_y_top_reserve += body_height + resolved_clearance;
    // body regular começa abaixo da reserva
}
```

Bottom floats: análogo — `cursor_y_bottom_reserve` reduz altura
útil; flow regular evita zona reservada quando verifica overflow.

### Decisão 6 — `scope: Parent + float: true` real

Quando dentro de Grid, anchor à página inteira (não célula);
emit fora do sub-frame da célula. Paridade vanilla "spans columns
+ float: true". Requer flag `is_inside_grid_cell` no Layouter
para diferenciar; `cell_origin_*` fields P84.6 são proxy
suficiente.

`scope: Column + float: true`: paridade vanilla rejeitada como
erro hard em `native_place`. **Esta sentinela já está activa P223**
— DEBT-37 §"Divergência" fechada.

### Decisão 7 — Sem tipo entity novo; sem ADR nova

P245 é refino consumer. Buffer struct `DeferredFloat` é local ao
Layouter (`01_core/src/rules/layout/`), não é `entities/` L1 type.
Não há novo `Content::*` variant; não há nova trait. ADR-0081
preserva-se literal — P245 materializa M7+4 que já está descrito
literal na ADR.

### Decisão 8 — Anti-inflação 37ª aplicação cumulativa

Opção β para L0 minimal (ADR-0080) — Layouter refactor interno,
não toca abstracções L1 (Content, geometry, Sides, etc.). Hashes
L0 preservados (paridade P243 Layouter internal refactor).

Opção α para A.4 outset/fill/stroke refino de Block+Boxed —
**diferido a passo futuro NÃO reservado** (per política P158
"sem novas reservas").

Opção α para promoção scope-outs adicionais — **nenhum
promovido em P245** (foco em Place float; outros scope-outs
diferidos).

Opção α para anotação cumulativa minimal nas ADRs — ADR-0081
transita 4.5/5 → **5/5 IMPLEMENTADO total**; ADR-0079 marca
Categoria C.1 cumprido; ADR-0080 sub-categoria nova "Layouter
internal refactor (semantic activation)" N=2 cumulativo.

### Decisão 9 — Padrão emergente "Promoção graded → real semantic"

P245 inaugura sub-padrão **N=1**: "Promoção graded → real
semantic activação consumer" (storage P223 → semantic P245
~24h depois cross-passo). Candidato a formalização N=3-4 se
outras aplicações ocorrerem (analogia hipotética: M7+3 graded
P243 → M7+3 real futuro se Linha A não tivesse antecipado;
Block.width graded P156G → real futuro).

---

## §4 Ficheiros a editar (C2+C3+C4+C5)

| Categoria | Ficheiro | Trabalho |
|-----------|----------|----------|
| L1 Layouter | `01_core/src/rules/layout/mod.rs` | Arm `Content::Place { float: true, .. }` activa; método `flush_pending_floats`; novos fields `floats_pending` + `cursor_y_top_reserve` + `cursor_y_bottom_reserve` |
| L1 Layouter | `01_core/src/rules/layout/mod.rs` | `new_page()` + `finish()` chamam flush |
| L1 helpers | `01_core/src/rules/layout/mod.rs` (ou módulo) | Helper `layout_sub_frame_capture` para layout body sem emitir (capture items + dimensões) |
| L1 Layouter | `01_core/src/rules/layout/mod.rs` | Tracking de `is_inside_grid_cell` (ou reuso de `cell_origin_*`) |
| Tests Layouter | `01_core/src/rules/layout/mod.rs` (test module) | 4-5 unit tests + 2-3 E2E |
| Tests content | `01_core/src/entities/content.rs` (test module) | 3-4 unit tests reforço cascata fields (provavelmente preservados; reforço apenas) |
| Inventário 148 | `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` | §A.5 `place(...)` `implementado⁺ ⁵ ⁴⁴` → `implementado⁺` literal (remove footnotes 5 e 44 que apontavam "float armazenado mas ignorado") |
| ADR-0081 | `00_nucleo/adr/typst-adr-0081-m7-plus-pipeline-restructuring-scope.md` | Status `IMPLEMENTADO parcial 4.5/5` → **`IMPLEMENTADO total 5/5`** + bloco P245 anotação |
| ADR-0079 | `00_nucleo/adr/typst-adr-0079-fase-5-layout-roadmap.md` (nome a confirmar) | Categoria C.1 `pendente` → `cumprido P245` |
| ADR-0080 | `00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md` | Sub-categoria "Layouter internal refactor (semantic activation)" N=1 → 2 cumulativo |
| DEBT.md | `00_nucleo/DEBT.md` | DEBT-37 nota cumulativa "P245 activa semantic real; sentinela `scope: Parent + float: true` mantém-se" — não reabertura, anotação |
| Relatório P245 | `00_nucleo/materialization/typst-passo-245-relatorio.md` | Estrutura canónica passos materialização L magnitude |

---

## §5 Critério aceitação P245 (C6+C7)

| Critério | Esperado |
|----------|----------|
| `cargo build --workspace` | **verde** |
| `cargo test --workspace` | **2198 → ~2208-2213 verdes** (+10-15 paridade L magnitude) |
| `crystalline-lint .` | **0 violations preservado** |
| `crystalline-lint --fix-hashes` | **"Nothing to fix"** (paridade P243 Layouter internal refactor — L0 não tocado) |
| Content variants | **62 preservado** (zero alterações L1 entities) |
| ShapeKind variants | **5 preservado** |
| Layouter fields | **+3** (`floats_pending`, `cursor_y_top_reserve`, `cursor_y_bottom_reserve`) |
| Regions fields | **3 preservado** (não tocado) |
| Stdlib funcs | **64 preservado** (`native_place` inalterado; só consumer activado) |
| §A.5 `place(...)` reclassificada | `implementado⁺ ⁵ ⁴⁴` → **`implementado⁺`** literal |
| Cobertura Layout per metodologia | **~91-94% → ~93-96%** (+2pp; remove notas graded de place) |
| ADR-0081 status | **`IMPLEMENTADO parcial 4.5/5` → `IMPLEMENTADO total 5/5`** |
| ADR-0079 Categoria C.1 | **`pendente` → `cumprido P245`** |
| ADR-0080 sub-categorias | Layouter internal refactor (semantic activation) N=1 → 2 |
| DEBT-37 sentinela | **preservada P223** (`scope: Parent + float: true` exigido) |
| L0 hashes propagados | **0** (paridade P243 Layouter internal) |
| Adaptações pre-existentes | **N=0-3** estimadas; `P245.div-N` se >3 |
| Regressões reais | **0** mandatório |
| Patterns emergentes | "Promoção graded → real semantic activação consumer" N=1 inaugurado |

**3 pré-condições obrigatórias verificadas** (per ADR-0081
§"Pré-condições obrigatórias"):

1. **Tests baseline preservados**: 2198 verdes pré-P245 →
   ~2208-2213 pós-P245 (+10-15 novos; adaptações N=0-3
   documentadas).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   P245 toca Layouter consumer apenas. `Introspector` trait
   intocada; sub-stores trackable F3 intocados. Verificação:
   tests memoization existentes preservados.
3. **Backward compat eval-time**: Place sem float (default
   `float: false`) preservado P84.6+P223 literal. Place com
   float real é semantic nova mas não-breaking (default era
   ignorar; agora activa).

**Promoções ADR esperadas**:

- ADR-0081 IMPLEMENTADO parcial 4.5/5 → **IMPLEMENTADO total 5/5**
  — fecha M7+ completo.
- ADR-0079 Categoria C.1 desbloqueada; pode antecipar promoção
  ADR-0079 → IMPLEMENTADO se Categoria C.2 não-bloqueante humana
  (decisão diferida P245).
- ADR-0080 sub-categoria "Layouter internal refactor (semantic
  activation)" N=2 cumulativo.

---

## §6 Próximo sub-passo pós-P245

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **ADR-0079 → IMPLEMENTADO total** | Promoção Fase 5 Layout completa (depende de Categoria C.2 multi-region completion ou scope-out humano) | XS-S | alta se humano decide scope-out C.2 |
| **Cell layout migration → `regions.current.height`** | Decisão 7 P243 diferida; activa A.4 breakable per-cell | M (~2-4h) | média |
| **Refino A.4** — `outset`+`fill`+`stroke` Block+Boxed | 3 de 4 scope-outs restantes pós-P242 | S-M por attr | baixa-média |
| **ADR meta admin XS** — formalizar "passo administrativo XS" N=6 | Promoção formal pattern N≥4 (limiar sólido atingido P244) | XS | média |
| **Pivot outro módulo** | Visualize 54%; Text 52%; Model 50% | varia | baixa |
| **Pausa M-fase** | M7+ completo; Fase 5 graded preservado | XS | baixa |

**Recomendação subjectiva pós-P245**: decisão humana entre
fechamento administrativo Fase 5 (promoção ADR-0079) ou
continuação materialização (cell layout migration; A.4 refinos).
M-fase M9d completa estrutural (5/5) — patamar conceptual claro.

**Decisão humana fica em aberto literal** pós-P245.

**Estado esperado pós-P245**:
- Tests workspace: **~2208-2213 verdes** (+10-15 vs P244).
- Content variants: **62 preservado**.
- ShapeKind variants: **5 preservado**.
- Layouter fields: **+3** (`floats_pending`, `cursor_y_top_reserve`,
  `cursor_y_bottom_reserve`).
- Regions fields: **3 preservado**.
- Stdlib funcs: **64 preservado**.
- §A.5 distribuição: **12/4/2/0/0 → 12/5/1/0/0** (place sai de
  parcial para implementado⁺ literal; pode também eliminar
  uma entrada parcial ou ausente conforme reclassificação
  empírica).
- Cobertura Layout per metodologia: **~91-94% → ~93-96%**.
- Cobertura user-facing total: **~74-75% → ~75-76%**.
- **ADRs distribuição**: PROPOSTO 12; EM VIGOR 29; IMPLEMENTADO
  **22 → 23** (ADR-0081 transita parcial → total); total 68
  preservado.
- **Saldo DEBTs: 11 preservado** (DEBT-37 já encerrado P84.6
  + sentinela P223 mantida; nada novo).
- **37 aplicações cumulativas anti-inflação** pós-P205D (+1 P245
  preserva sem inflar).
- **Patterns emergentes pós-P245**:
  - "Promoção graded → real semantic activação consumer" N=1
    inaugurado P245. Candidato a formalização N=3-4.
  - "Spec C1 audit obrigatório bloqueante" N=7 → **8 cumulativo**.
  - "Passo administrativo XS" N=6 preservado (P245 não-administrativo).
- **Categoria D Fase 5 Layout: 3/? sub-passos materializados**
  preservado.
- **Categoria A Fase 5 Layout**: 5/5 + parcial A.4 P242 preservado.
- **Categoria B Fase 5 Layout**: 3/3 preservado.
- **Categoria C.1 Fase 5 Layout**: **PENDENTE → CUMPRIDO P245** ✓.
- **Categoria C.2 Fase 5 Layout**: pendente (cell-level multi-region;
  scope-out humano candidato pós-P245 para fechar ADR-0079).
- **Fase 5 Layout candidata**: 14/13-15 → **15/13-15 sub-passos
  materializados** (Categoria C.1 cumprida).
- **M9d / M7+ progresso**: **5/5 sub-passos materializados** ✓✓✓
  (M7+1 ✓; M7+2 ✓; M7+3 ✓ via cumulativo; M7+5 ✓; **M7+4 ✓ P245**).
- **Marco interno**: M9d / M7+ COMPLETO total; ADR-0081
  IMPLEMENTADO total 5/5; Categoria C.1 Fase 5 desbloqueada;
  Place graded P223 promovido a real P245 (sub-padrão "Promoção
  graded → real" inaugurado).

---

## §7 Notas operacionais para o executor

1. **Audit C1 BLOQUEANTE prioridade absoluta**. Não materializar
   antes de §2.1-§2.7 completos. **Lição N=8 cumulativa**:
   primeira aplicação onde audit C1 expande de "grep variants
   antes de assumir ausência" (lição P244) para "grep fields/arms
   já implementados antes de assumir trabalho original". Se
   §2.2 revelar consumer parcialmente activado (improvável dado
   o `float: _, clearance: _` literal documentado), criar
   `P245.div-2`.

2. **Algoritmo float vanilla — referência empírica obrigatória
   em C2**. Antes de fixar implementação Decisões 1-5, ler
   `lab/typst-original/crates/typst-layout/src/flow/place.rs`
   (ou nome empírico análogo). Documentar divergências graded
   permitidas per ADR-0054 (ex: cristalino single-pass não
   suporta float-overflow-cross-page; vanilla suporta via
   recursão pós-page-break). Divergências documentadas em
   relatório §"Limitações conscientes P245".

3. **Algoritmo float é semantic não trivial**. Reserva de
   espaço (Decisão 5) interage com page break detection
   existente. Cuidado especial:
   - Float top-aligned + body grande que excede página actual
     → float emit na página actual; body overflow para nova
     página (sem float repetido).
   - Float bottom-aligned + body que termina antes do
     `cursor_y_bottom_reserve` → float emit ao fundo; flow
     termina cedo (sem desperdício mas com gap visual). Aceite
     per ADR-0054.
   - Múltiplos floats top-aligned na mesma página → stack
     vertical do topo para baixo + clearance Y entre cada
     (cumulative `cursor_y_top_reserve`).
   - Múltiplos floats bottom-aligned → stack vertical do fundo
     para cima.

4. **Tests E2E são críticos**. Tests unit Layouter podem ser
   suficientes para validar mecânica mas E2E (placement real
   no PDF output) validam que renderização final corresponde.
   Mínimo 2-3 E2E layout.

5. **Custo real esperado**: ~5-8h (paridade ADR-0081 §"Escopo").
   Maior parcela: implementação consumer Layouter + tests E2E
   (~70% do tempo). Audit C1 + decisões + anotações ADR
   (~30%).

6. **Sem `P245.div-N` antecipado**. Único cenário de divergência
   material seria audit C1 revelar consumer já parcialmente
   activado (improvável). Se ocorrer → re-escopo formal.

7. **Anti-inflação 37ª aplicação cumulativa** pós-P205D preservar:
   Opção β L0 intocados + Opção α anotação cumulativa minimal
   ADRs + Opção α promoção ADR-0081 → IMPLEMENTADO total +
   Opção α sub-padrão novo inaugurado anotado (sem ADR meta
   prematura) + Opção α scope-outs não-promovidos (foco em
   Place float).

8. **Decisão humana pós-P245 sobre ADR-0079**: ADR-0079 ficará
   com Categoria C.1 cumprida + Categoria C.2 pendente. Humano
   decide entre:
   - Promover ADR-0079 → IMPLEMENTADO graded (paridade ADR-0061
     Caminho 2 humano "scope-out formal").
   - Manter ADR-0079 PROPOSTO até C.2 materializado (passo
     dedicado L+ ~8-12h futuro).
   - Categoria C.2 NÃO-reservada per política P158.
