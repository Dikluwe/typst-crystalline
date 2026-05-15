# Spec do passo P251 — A.4 TableCell row break real via slice frame items at height (γ-Items refino directo P248 clip implícito; segunda aplicação cumulativa citante ADR-0082 PROPOSTO N=1 → 2; promoção real scope-out TableCell N=12 → 13; inaugura sub-padrão "slice frame items at height" N=1; activa Categoria C.2 Fase 5 Layout parcial)

**Data**: 2026-05-14.
**Tipo**: refino consumer Layouter — slice frame items por
altura threshold + buffer cell tails pendentes + flush em page
break. Promove scope-out P157B TableCell.body overflow de
"clip implícito P248" a "row break vertical real cross-page".
**γ-Items** (slicing ao nível Vec<FrameItem> via
`pos.y` local; NÃO ao nível Content reconstruction).
**Magnitude planeada**: **L (~6-8h)** — paridade γ-Items (não
γ-Content L+ ~10-12h hipotetizado inicialmente; audit C1
empírico §2 reverteu magnitude). Audit revelou
`layout_sub_frame_with_width` retorna `(height, items)` com
`pos.y` **local ao sub-frame** (comentário literal mod.rs:1-3)
— slicing por threshold é filter + rebase trivial.
**Marco**: **segunda aplicação cumulativa citante ADR-0082
PROPOSTO** N=1 → 2 (paridade pattern ADR-0065 P156K validado
pós-P156J N=1 → P157A N=2 → P157B N=3 EM VIGOR); **promoção
real scope-out TableCell P157B graded "clip implícito P248"
→ row break vertical real**; **activa Categoria C.2 Fase 5
Layout parcial** (ADR-0079 PROPOSTO §"Categoria C.2 cell-level
multi-region completion"); **primeira aplicação cumulativa
do padrão "slice frame items at height"** N=1 inaugurado;
**décima quarta aplicação cumulativa pattern "spec C1 audit
obrigatório bloqueante pós-P236.div-1"** N=13 → 14 cumulativo
(lição refinada P251: "audit C1 deve confirmar localidade
pos.y antes de fixar abordagem γ-Items vs γ-Content").

---

## §1 O que será feito

### §1.1 Estado pré-P251 (factual; confirmado audit empírico 2026-05-14)

**TableCell overflow actual** (`grid.rs:393-433`):

```rust
let cell_overflow = cell_h_measured > body_h;
// ... emit fill + stroke ...
if cell_overflow {
    // P248: wrap em Group com clip_mask Rect bounds body_w × body_h.
    self.regions.current.current_items.push(FrameItem::Group {
        pos:          Point { x: Pt(body_x), y: Pt(body_y) },
        matrix:       TransformMatrix::identity(),
        clip_mask:    Some(ShapeKind::Rect),
        inner_width:  body_w,
        inner_height: body_h,
        items:        translated_items,
    });
} else {
    for item in translated_items {
        self.regions.current.current_items.push(item);
    }
}
```

**P248 clip implícito** preserva items dentro de Group com
clip_mask Rect — visualmente cell limite respeitado, mas items
overflow permanecem no Group (invisíveis). **Não há split**;
não há continuação na próxima página.

**`layout_sub_frame_with_width` signature** (mod.rs:2046; audit
§2.1):

```rust
pub(super) fn layout_sub_frame_with_width(
    &mut self,
    content: &Content,
    cell_x: f64,
    _cell_width: f64,
) -> (f64, Vec<FrameItem>)
```

Retorna `(height, items)` com **posições locais ao frame
temporário** (comentário literal mod.rs:1-3 do helper). Save/
restore completo cursor state.

**Layouter fields existentes (P245 + P248)**:
- `floats_pending: Vec<DeferredFloat>` (P245; flush em new_page
  + finish).
- `prev_block_below_pending: f64` + `block_chain_active: bool`
  (P250).

### §1.2 Trabalho a fazer P251

**Helper novo `slice_frame_items_at_height`**:

```rust
/// P251 — Divide items em (head, tail) por threshold em pos.y.
/// Tail items são rebased: pos.y -= threshold.
/// Items são considerados atómicos (não dividem-se internamente);
/// items que **começam** abaixo de threshold vão para tail.
/// Items que cruzam threshold (raros — só Group + Shape com
/// inner_height grande) vão para tail completo (paridade vanilla
/// "atomic block can't split mid-paragraph").
///
/// Returns (head, tail). Tail vazio se nenhum item ultrapassa
/// threshold.
pub(super) fn slice_frame_items_at_height(
    items: Vec<FrameItem>,
    threshold: f64,
) -> (Vec<FrameItem>, Vec<FrameItem>) {
    let mut head = Vec::new();
    let mut tail = Vec::new();
    for item in items {
        let y = match &item {
            FrameItem::Text { pos, .. } => pos.y.0,
            FrameItem::Shape { pos, .. } => pos.y.0,
            FrameItem::Group { pos, .. } => pos.y.0,
            FrameItem::Image { pos, .. } => pos.y.0,
        };
        if y >= threshold {
            // Rebase pos.y -= threshold para tail.
            let rebased = rebase_item_y(item, -threshold);
            tail.push(rebased);
        } else {
            head.push(item);
        }
    }
    (head, tail)
}
```

**Helper auxiliar `rebase_item_y`** (private):
```rust
fn rebase_item_y(item: FrameItem, delta: f64) -> FrameItem {
    match item {
        FrameItem::Text { pos, glyphs, style } =>
            FrameItem::Text {
                pos: Point { x: pos.x, y: Pt(pos.y.0 + delta) },
                glyphs, style,
            },
        // ... análogos para Shape, Group, Image.
    }
}
```

**Layouter field novo**:
```rust
pub(super) pending_cell_tails: Vec<DeferredCellTail>,

pub(super) struct DeferredCellTail {
    pub items:        Vec<FrameItem>,
    pub origin_x:     f64,    // cell_x preservado
    pub width:        f64,    // body_w preservado
    pub fill:         Option<Color>,         // bounds para re-emit
    pub stroke:       Option<Stroke>,        // bounds para re-emit
    pub clip_mask:    Option<ShapeKind>,     // se necessário paridade
}
```

**`grid.rs` arm cell overflow substituído**:

```rust
let cell_overflow = cell_h_measured > body_h;

if cell_overflow {
    // P251 — γ-Items: slice items por body_h threshold.
    let (head_items, tail_items) =
        Self::slice_frame_items_at_height(translated_items, body_h);

    // Emit head na página actual (NÃO mais wrap em Group com clip
    // — head items já cabem no body_h por definição do slice).
    for item in head_items {
        self.regions.current.current_items.push(item);
    }

    // Push tail ao pending buffer se há overflow real.
    if !tail_items.is_empty() {
        self.pending_cell_tails.push(DeferredCellTail {
            items:     tail_items,
            origin_x:  body_x,
            width:     body_w,
            fill:      effective_fill.copied(),
            stroke:    effective_stroke.cloned(),
            clip_mask: None, // P251: clip já não necessário no head
        });
    }
} else {
    for item in translated_items {
        self.regions.current.current_items.push(item);
    }
}
```

**Layouter `new_page()` + `finish()` flush**:

```rust
pub(super) fn new_page(&mut self) {
    self.flush_pending_floats();   // P245 preservado
    self.flush_pending_cell_tails(); // P251 NOVO
    // ... lógica existente cursor.rs ...
}

pub(super) fn flush_pending_cell_tails(&mut self) {
    for tail in self.pending_cell_tails.drain(..) {
        // Emit fill atrás (paridade P248 Z-order step 1).
        if let Some(c) = tail.fill {
            // ... emit Shape Rect com bounds (tail.origin_x, top_y,
            //      tail.width, tail_height) ...
        }
        // Emit items.
        for item in tail.items {
            // pos.x preservado (cell column-aligned);
            // pos.y já foi rebased no slice — adicionar top_y
            // da página actual.
            let final_item = rebase_item_y(item,
                self.regions.current.cursor_y.0);
            self.regions.current.current_items.push(final_item);
        }
        // Emit stroke à frente (Z-order step 3).
        if let Some(s) = tail.stroke {
            // ...
        }
    }
}
```

### §1.3 Limitações conscientes γ-Items (per ADR-0054 graded)

**Limitação 1**: items que **internamente** ultrapassam threshold
(ex: Group com inner_height > threshold; Shape com height > threshold)
vão **completos para tail**, não slice mid-item. Paridade vanilla
"atomic block can't split mid-paragraph" preservada — diverge de
γ-Content que teria slice mid-paragraph via re-layout.

**Limitação 2**: tail cell ocupa **toda a largura da row** na
nova página (cell_x + width preservados). Outras cells da row
original **não continuam** (paridade vanilla cell-level mas não
row-level perfeito). Se row tem 3 cells e cell 2 overflow, cells
1 e 3 terminam na página actual; só cell 2 continua na próxima.

**Limitação 3**: row_heights da próxima página são **só do
tail cell** (não há tracking de "row N continua"). Próxima
página começa fresh com cell tail emit + flow continua.

**Limitação 4**: cell tail que ela própria overflow na nova
página → recursivamente push novo tail (Layouter loop em
`new_page`). Bug-prone se loop infinito. **Mitigação**: max
3 iterações de tail forwarding (paridade vanilla heurística).

**Limitação 5**: stroke + fill **re-emit** na nova página com
bounds = tail bounds (não bounds originais do cell). Visualmente
cell pode parecer dois rectângulos separados (paridade vanilla
"split block draws two borders").

Documentar em relatório P251 §"Limitações conscientes γ-Items"
+ ADR-0054 §"Promoções reais cumulativas" tabela.

### §1.4 Tests esperados

Tests P251 novos estimados: **15-25** (range L magnitude;
slicing + buffer + flush + cenários combinatorial):

- 5-7 unit slice_frame_items_at_height (items dentro threshold,
  items acima threshold, items mistos, items que cruzam
  threshold → tail completo, threshold 0.0 → tudo tail, items
  vazios → vazios).
- 4-6 unit rebase_item_y (Text/Shape/Group/Image — 4 variants
  FrameItem cobertos).
- 4-6 unit Layouter flush_pending_cell_tails (tail vazio
  → no-op; tail single → emit; tail múltiplos → emit ordem;
  tail com fill+stroke re-emit).
- 3-5 E2E TableCell row break (cell overflow simples → tail
  na próxima página; cell overflow recursivo limitar 3 iter;
  cell em Table com outras cells normal preservadas; row
  break + page break manual interaction).
- 2-3 unit regression P248 clip implícito → não usar P251
  para cells em rows Fixed (paridade vanilla Fixed clip preservado).

**Workspace pós-P251**: **2276 → ~2291-2301 verdes** (range
+15-25 paridade L magnitude).

### §1.5 Adaptações pre-existentes

Estimativa **N=2-8** adaptações tests pré-existentes (range
menor que P250 N=21 porque P251 é refino consumer Layouter
sem alterações em construtores Content):

- Tests P248 que verificam `FrameItem::Group { clip_mask:
  Some(Rect), .. }` em cells overflow → adaptar para verificar
  ausência de Group (items emit directo) + tail flush em
  next page.
- Tests sentinela `p248_table_cell_overflow_emite_clip_group`
  (linha 4473) **revogado** ou actualizado para reflectir nova
  semantic.
- Tests E2E Table com cell overflow → output PDF muda (split
  visual em vez de clip silencioso). Adaptações cuidadosas.

**Cenário `P251.div-N` se >8 adaptações** → reconciliação
prévia.

---

## §2 Verificação empírica pré-P251 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=13 → 14 cumulativo

Audit C1 obrigatório bloqueante pós-P236.div-1. Lição refinada
N=13 P250 ("refactor cross-arm Sequence consumer exige audit
de todos os patterns de iteração existentes antes de migrar a
peekable") expande para **N=14 cumulativo**: "audit C1 deve
confirmar localidade pos.y antes de fixar abordagem γ-Items
vs γ-Content para slicing".

### §2.1 `layout_sub_frame_with_width` signature + return (confirmado 2026-05-14)

```bash
grep -B2 -A 20 "pub(super) fn layout_sub_frame_with_width" \
  01_core/src/rules/layout/mod.rs
```

**Resultado audit anterior**:
- Signature: `(&mut self, content, cell_x, _cell_width) ->
  (f64, Vec<FrameItem>)`.
- Comentário literal: "Retorna `(height, items)` com **posições
  locais ao frame temporário**".
- Save/restore completo cursor state.

**Decisão arquitectural fixa γ-Items** (não γ-Content).

### §2.2 P248 clip implícito exacto (confirmado 2026-05-14)

`grid.rs:393-433` — `cell_overflow → FrameItem::Group { clip_mask:
Some(ShapeKind::Rect), .. }`. Items overflow preservados dentro
do Group (visualmente clipped).

### §2.3 Slicing patterns pré-existentes

```bash
grep -rn "slice\|split\|fragment\|partial" 01_core/src/rules/layout/
```

Resultado audit anterior: **zero slicing relevante para frame
items**. P251 inaugura pattern "slice frame items at height".

### §2.4 FrameItem variants completos

```bash
grep -B1 -A 8 "pub enum FrameItem" 01_core/src/entities/layout_types.rs
```

Identificar exactos variants (esperado: Text, Shape, Group, Image).
**Rebase_item_y precisa cobrir todos** com match exhaustive.

### §2.5 Algoritmo vanilla row break — REFERÊNCIA EMPÍRICA OBRIGATÓRIA

```bash
cat lab/typst-original/crates/typst-layout/src/grid/rowspans.rs 2>/dev/null \
  || cat lab/typst-original/crates/typst-layout/src/grid/*.rs 2>/dev/null | head -100
ls lab/typst-original/crates/typst-layout/src/grid/
```

Identificar:
- Vanilla algoritmo é γ-Content (re-layout tail Content) ou
  γ-Items (split items pos.y)?
- Vanilla preserva borders/fill em ambos os fragmentos? Como?
- Limitação atomic (paragraph/block não divide mid-) é preservada?
- Recursive overflow handling (tail que overflow): max
  iterações? loop detection?

**Confronto referência empírica obrigatório em C2** antes de
cristalizar.

### §2.6 Layouter pending pattern existente (P245)

```bash
grep -B2 -A 8 "pub(super) struct DeferredFloat\|floats_pending" \
  01_core/src/rules/layout/mod.rs
```

`DeferredFloat` struct + `floats_pending` field + `flush_pending_floats`
method pattern P245 — reusar arquitectura paralela para
`DeferredCellTail` + `pending_cell_tails` + `flush_pending_cell_tails`.

### §2.7 Tests pré-P251 baseline

```bash
cargo test --workspace
```

Esperado: **2276 verdes** (estado pós-P250).

### §2.8 Decisão arquitectural pós-audit

Após §2.1-§2.7 completos, fixar empíricamente:
- **Decisão 1** γ-Items confirmado (§2.1 + §2.5 vanilla
  reference).
- **Decisão 2** signature `slice_frame_items_at_height` final.
- **Decisão 3** algoritmo limitação atomic (paridade vanilla
  §2.5).
- **Decisão 4** algoritmo recursive overflow (max iter ou
  loop detection).
- **Decisão 5** stroke + fill re-emit semantic per fragment
  (paridade vanilla §2.5).

### `P251.div-N` antecipadas — possíveis

- **`P251.div-1`** se §2.5 revelar vanilla é γ-Content
  significativamente diferente de γ-Items → divergência
  graded permitida per ADR-0054 + ADR-0033 — documentar
  como limitação consciente; **NÃO re-escopo**.
- **`P251.div-2`** se §2.4 revelar FrameItem variants
  adicionais (improvável) → rebase_item_y match exhaustive
  estende.
- **`P251.div-3`** se §2.6 revelar `DeferredFloat` pattern
  com side-effects que `DeferredCellTail` não pode replicar
  directo → refactor minor.
- **`P251.div-4`** se §2.7 baseline ≠ 2276 → reconciliação
  prévia.

---

## §3 Decisões fixadas P251 — 11 decisões

### Decisão 0 — Audit C1 lição N=13 → 14 cumulativo

Pattern "spec C1 audit obrigatório bloqueante pós-P236.div-1"
N=13 → **14 cumulativo**. Refino procedural P251: "audit C1
deve confirmar localidade pos.y antes de fixar abordagem
γ-Items vs γ-Content para slicing". Anotação em ADR-0080
§"Lição refinada P251".

### Decisão 1 — γ-Items (slice frame items at height; NÃO γ-Content)

Decisão final fixa: **γ-Items** validado por audit §2.1 +
§2.5. Vantagem: magnitude L (~6-8h) face γ-Content L+
(~10-12h); reuso `layout_sub_frame_with_width` retorno
`(height, items)` directo sem refactor; `pos.y` local
permite slice trivial.

**Limitação consciente** documentada: items atómicos não
dividem mid-item (paridade vanilla); `P251.div-1` se
vanilla é γ-Content significativamente diferente.

### Decisão 2 — Helper `slice_frame_items_at_height` (preliminar)

```rust
pub(super) fn slice_frame_items_at_height(
    items: Vec<FrameItem>,
    threshold: f64,
) -> (Vec<FrameItem>, Vec<FrameItem>);
```

Função pura (sem `&self`); módulo privado novo
`01_core/src/rules/layout/slicing.rs` (paridade subpadrão
"tipo entity em ficheiro próprio" N=6 inverso — helper em
ficheiro próprio).

### Decisão 3 — Helper auxiliar `rebase_item_y` (preliminar)

Function privada per FrameItem variant (4 variants confirmados
§2.4: Text, Shape, Group, Image). Match exhaustive; pos.y
rebase trivial; outros fields preservados.

### Decisão 4 — Layouter field novo `pending_cell_tails`

```rust
pub(super) pending_cell_tails: Vec<DeferredCellTail>,

pub(super) struct DeferredCellTail {
    pub items:    Vec<FrameItem>,
    pub origin_x: f64,
    pub width:    f64,
    pub fill:     Option<Color>,
    pub stroke:   Option<Stroke>,
}
```

Paralelo arquitectural a `DeferredFloat` + `floats_pending`
(P245). Padrão "DeferredX buffer + flush em new_page" N=1
(P245) → **N=2 cumulativo P251**.

### Decisão 5 — Flush em `new_page()` + `finish()`

```rust
pub(super) fn new_page(&mut self) {
    self.flush_pending_floats();      // P245 preservado
    self.flush_pending_cell_tails();  // P251 NOVO
    // ... lógica existente cursor.rs ...
}
```

Ordem: floats primeiro (top/bottom layout independente; P245),
cell tails depois (flow continuation; P251). Paridade vanilla
"flow continuation flushes pendings in dependency order".

### Decisão 6 — Limitação atomic items (preliminar; final pós §2.5)

Items que **internamente** ultrapassam threshold vão completos
para tail. Não há slice mid-item. Paridade vanilla.

**Decisão final §2.8** confirma vanilla.

### Decisão 7 — Recursive overflow limit (preliminar; final pós §2.5)

Max **3 iterações** de tail forwarding via `new_page`. Após
3 iter, tail é **discarded** com warning silencioso (paridade
vanilla heurística). Mitigação loop infinito.

Decisão final §2.8.

### Decisão 8 — Re-emit fill + stroke per fragment (preliminar)

Fragment de cell na nova página re-emit fill (atrás) + items
+ stroke (à frente), com bounds = tail bounds (width preservado;
height = tail items max pos.y + altura conteúdo). **NÃO
preserva** semantic visual "uma única cell" — visualmente
parecem dois rectângulos.

Paridade vanilla a confirmar §2.5.

### Decisão 9 — Cita ADR-0082 PROPOSTO N=1 → 2 (segunda citante)

P251 é **segunda aplicação concreta citante** ADR-0082
PROPOSTO (P250 foi primeira). 4 critérios operacionais ADR-0082
verificados explicitamente:

1. **Storage prévio**: TableCell.body já armazenado P157B
   (scope-out original "ignorados em layout" graded);
   semantic actual P248 "clip implícito".
2. **Consumer Layouter pre-promoção é graded**: P248 "clip
   implícito" é graded — não é semantic real "row break
   vertical cross-page".
3. **Paridade vanilla referência empírica**: audit C1 §2.5
   obrigatório antes de cristalizar.
4. **Backward compat literal**: cells sem overflow + cells
   em rows Fixed (P248 clip preservado paridade vanilla)
   preservam output P248 bit-equivalente; **só cells em rows
   Auto/Fraction com overflow mudam comportamento** (clip
   implícito → row break real).

**Validação ADR-0082 N=2 citante** — segundo passo dum
sequente candidato N=3 para promoção EM VIGOR (paridade
ADR-0065 P156K → P156J/P157A/P157B). **P252 ou P253 podem
materializar N=3 para promoção humana ADR-0082 → EM VIGOR**.

### Decisão 10 — Sub-padrão "Slice frame items at height" N=1 inaugurado

P251 inaugura sub-padrão **N=1**: "Slice frame items at height
via filter + rebase pos.y". Pattern emergente candidato a
formalização N=3-4 futuro se outras features exigirem (ex:
column flow DEBT-56 multi-region; pagination overflow generic).

### Decisão 11 — Anti-inflação 43ª aplicação cumulativa

- Opção β L0 minimal: `layout_types.md` L0 prompt **não tocado**
  (`FrameItem` variants já documentados; helper novo é detalhe
  implementação interno); `entities/region.md` extensão
  documentando `pending_cell_tails` field; hash propagado.
- Opção α activação consumer real (refino directo P248 clip
  implícito → row break real).
- Opção α reuso `DeferredFloat` pattern P245 (paridade
  arquitectural; subpadrão "DeferredX buffer + flush" N=1 → 2).
- Opção α helper novo em ficheiro próprio (`layout/slicing.rs`;
  paridade subpadrão "tipo entity em ficheiro próprio" N=6).
- Opção α anotação cumulativa minimal ADRs (0061 + 0079 + 0080
  + 0054 + **0082 citação segunda** + ADR-0079 Categoria C.2
  parcial).
- Opção α sub-padrão N=1 inaugurado "slice frame items at height".
- Opção α DEBT-34e preservado aberto (P251 trata row break
  vertical cell-level; DEBT-34e é colspan/rowspan placement —
  distinto).

---

## §4 Ficheiros a editar (C2+C3+C4+C5)

| Categoria | Ficheiro | Trabalho |
|-----------|----------|----------|
| L1 helper novo | `01_core/src/rules/layout/slicing.rs` (criar; ~80-120 LoC) | `slice_frame_items_at_height` + `rebase_item_y` private + 4-variant match |
| L1 module | `01_core/src/rules/layout/mod.rs` | Add `mod slicing;` declaration; `+1 field` (`pending_cell_tails`) + struct `DeferredCellTail`; `+1 método` (`flush_pending_cell_tails`) |
| L1 Layouter | `01_core/src/rules/layout/cursor.rs` | `new_page()` chama `flush_pending_cell_tails()` (após `flush_pending_floats` P245) |
| L1 Layouter | `01_core/src/rules/layout/grid.rs` | Arm cell overflow: substituir `FrameItem::Group { clip_mask: Some(Rect), .. }` (P248) por `slice_frame_items_at_height` + push to `pending_cell_tails` (preservar P248 para rows Fixed) |
| L0 prompt | `00_nucleo/prompts/entities/region.md` | Documentar `pending_cell_tails` field + flush em new_page; secção nova ou anotação cumulativa P251 |
| Tests slicing | `01_core/src/rules/layout/slicing.rs` (test module) | 5-7 unit slice + 4-6 unit rebase + edge cases |
| Tests Layouter | `01_core/src/rules/layout/tests.rs` | 4-6 unit flush + 3-5 E2E row break + 2-3 regression P248 preservado Fixed rows |
| Tests adaptações | conforme `P251.div-N` | 2-8 adaptações estimadas (tests P248 que verificam `FrameItem::Group { clip_mask, .. }` em cells overflow) |
| Inventário 148 | `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md` | §A.5 `table(...)` / `table_cell(...)` reclassificadas (footnote ⁶⁸ P251 — row break real); cobertura Layout per metodologia recalculada |
| ADR-0061 | `00_nucleo/adr/typst-adr-0061-layout-fase-x-roadmap.md` | §"Refino futuro" anotação P251 — row break TableCell |
| ADR-0079 | `00_nucleo/adr/typst-adr-0079-fase-5-layout-roadmap.md` | **Categoria C.2 §"Sub-categorias materializadas": TableCell row break real P251** (parcial — cell-level only; multi-region completo continua diferido); anotação cumulativa P251 |
| ADR-0080 | `00_nucleo/adr/typst-adr-0080-l0-minimal-para-refactors.md` | §"Lição refinada P251" N=14 cumulativo; sub-categoria nova "Slice frame items at height" N=1 inaugurada |
| ADR-0054 | `00_nucleo/adr/typst-adr-0054-criterio-fecho-debt-1.md` | §"Promoções reais cumulativas" extensão: P251 ×1 = cumulativo N=12 → **N=13** (P242 ×2 + P247 ×3 + P248 ×3 + P250 ×4 + **P251 ×1**) |
| **ADR-0082** | `00_nucleo/adr/typst-adr-0082-promocoes-reais-scope-outs-graded.md` | **§"Aplicações citantes" sub-secção: P251 segunda aplicação citante explícita** (N=1 → 2); status PROPOSTO preservado (promoção a EM VIGOR pendente N=3 citantes) |
| DEBT.md | `00_nucleo/DEBT.md` | DEBT-30/34c/34e/56 sentinelas preservadas; DEBT-34e anotação cumulativa "P251 não fecha — DEBT-34e é colspan/rowspan placement, distinto de row break vertical cell-level cumprido P251" |
| Relatório P251 | `00_nucleo/materialization/typst-passo-251-relatorio.md` | Estrutura canónica passos materialização L magnitude |

---

## §5 Critério aceitação P251 (C6+C7)

| Critério | Esperado |
|----------|----------|
| `cargo build --workspace` | **verde** |
| `cargo test --workspace` | **2276 → ~2291-2301 verdes** (+15-25 paridade L) |
| `crystalline-lint .` | **0 violations** |
| `crystalline-lint --fix-hashes` | **1 hash propagado** (`entities/region.md` se documentado) |
| Content variants | **62 preservado** |
| ShapeKind variants | **5 preservado** |
| Block / Boxed / TableCell fields | preservados |
| Layouter fields | **+1** (`pending_cell_tails`) |
| Layouter methods | **+1** (`flush_pending_cell_tails`) |
| Layouter struct local | **+1** (`DeferredCellTail`) |
| Layouter helper module | **+1 ficheiro novo** (`layout/slicing.rs`) com 2 fns públicas (`slice_frame_items_at_height` + interna `rebase_item_y`) |
| Regions fields | **4 preservado** |
| Stdlib funcs | **64 preservado** |
| §A.5 `table(...)` / `table_cell(...)` | reclassificação implementado⁺ + footnote ⁶⁸ P251 (row break real) |
| Cobertura Layout per metodologia | **~96-97% → ~97-98%** (+1pp refino qualitativo) |
| Cobertura user-facing total | **~75-76% preservado** |
| Scope-outs Block originais P156G fechados | **10/10 preservado** (Block A.4 COMPLETO) |
| Scope-outs Boxed originais P156H fechados | **5/6 preservado** (stroke-overhang resta) |
| Promoções reais scope-outs ADR-0054 cumulativas granular | **12 → 13** (P251 ×1) |
| ADR-0079 Categoria C.2 | anotação P251 — **parcial cell-level**; multi-region completo continua diferido |
| ADR-0080 sub-categoria | "Slice frame items at height" N=1 inaugurada |
| ADR-0061 §"Refino futuro" | anotação P251 |
| ADR-0054 §"Promoções reais" | cumulativo granular N=13 (P251 ×1) |
| **ADR-0082** | **§"Aplicações citantes" N=1 → 2** (segunda citante explícita) |
| DEBT-30/34c/34e/56 | sentinelas preservadas; DEBT-34e anotação cumulativa "distinção P251" |
| L0 hashes propagados | 1 (`region.md`) |
| Adaptações pre-existentes | **N=2-8** estimadas; `P251.div-N` se >8 |
| Regressões reais | **0** mandatório |
| Patterns emergentes | "Slice frame items at height" N=1 inaugurado; "DeferredX buffer + flush" N=1 → 2 cumulativo; "Aplicação citante ADR-0082" N=1 → 2; "Spec C1 audit obrigatório bloqueante" N=13 → 14 cumulativo |

**3 pré-condições obrigatórias verificadas**:

1. **Tests baseline preservados**: 2276 verdes pré-P251 →
   ~2291-2301 pós-P251 (+15-25 novos; N=2-8 adaptações
   documentadas; alterações **só** em cells Auto/Fraction com
   overflow; cells em rows Fixed P248 clip preservado literal).
2. **Comemo memoization invariants ADR-0073/0074 preservados**:
   P251 toca Layouter consumer apenas; Introspector trait
   intocada.
3. **Backward compat**: cells sem overflow preservam output
   literal P248; cells em rows Fixed preservam clip implícito
   P248 (paridade vanilla); só cells Auto/Fraction com overflow
   ganham semantic nova.

**Promoções ADR esperadas**:

- ADR-0079 Categoria C.2 §"Sub-categorias materializadas":
  TableCell row break real **parcial cell-level** P251.
- ADR-0080 sub-categoria nova "Slice frame items at height"
  N=1 inaugurada + lição refinada N=14 cumulativo.
- ADR-0061 §"Refino futuro" anotação P251.
- ADR-0054 §"Promoções reais" cumulativo granular N=13.
- **ADR-0082 §"Aplicações citantes" N=1 → 2** (segunda citante).
- **Sem novas ADRs criadas**.

---

## §6 Próximo sub-passo pós-P251

P251 fecha row break TableCell real. Restantes pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **A.4 Boxed stroke-overhang** | Único scope-out P156H restante (cita ADR-0082 N=2 → 3 → **promoção EM VIGOR humana possível**) | XS | **alta** (Boxed A.4 completo 6/6; triggera promoção ADR-0082) |
| **ADR-0079 → IMPLEMENTADO graded** | Categoria C.2 cumprida parcialmente P251; Categoria A.4 quase completa | XS-S | alta se humano decide fechamento |
| **ADR-0082 → EM VIGOR** | Decisão humana pós-N=3 citantes (P250+P251+P252 candidato) | XS | **alta** após P252 |
| **DEBT-34e abrir P-passo** | Refactor placement Grid completo (colspan/rowspan real) | L+ | baixa (não-reservado P158) |
| **Pivot outro módulo** | Visualize 54%; Text 52%; Model 50% | varia | baixa-média (Layout muito reforçado) |
| **Cell tail forwarding limit refino** | Limit `max 3 iter` para configurable / robust | XS | baixa |

**Recomendação subjectiva pós-P251**: **A.4 Boxed stroke-overhang**
(XS isolado) — **terceira aplicação citante ADR-0082** N=2 → 3;
fecha Boxed A.4 completo 6/6; **triggera promoção ADR-0082 →
EM VIGOR humana possível** (paridade ADR-0065 P156K validada
via P156J/P157A/P157B sequente). Magnitude controlada XS.

Alternativa: **ADR-0079 → IMPLEMENTADO graded** (XS-S) —
fechamento administrativo Fase 5 Layout agora que A.4 Block
COMPLETO + A.4 Boxed 5/6 + A.4 TableCell row break + C.2
parcial. Patamar conceptual sólido para fechamento administrativo.

**Decisão humana fica em aberto literal** pós-P251.

**Estado esperado pós-P251**:
- Tests workspace: **~2291-2301 verdes** (+15-25 P251).
- Content variants: **62 preservado**.
- Block fields: **14 preservado** (P250 final).
- Boxed fields: **10 preservado**.
- TableCell fields: **5 preservado** (P157B final).
- ShapeKind variants: **5 preservado**.
- Layouter fields: **+1** (`pending_cell_tails`).
- Layouter methods: **+1** (`flush_pending_cell_tails`).
- Layouter struct local: **+1** (`DeferredCellTail`).
- Layouter modules: **+1** (`layout/slicing.rs`).
- Regions fields: **4 preservado**.
- Stdlib funcs: **64 preservado**.
- §A.5 distribuição: refino qualitativo (footnote ⁶⁸ P251).
- Cobertura Layout per metodologia: **~96-97% → ~97-98%**.
- Cobertura user-facing total: **~75-76% preservado**.
- **ADRs distribuição preservada literal**: PROPOSTO 13; EM VIGOR
  29; IMPLEMENTADO 23; total **69 preservado**. Anotações
  cumulativas 0061+0079+0080+0054+**0082 §"Aplicações citantes"
  N=2**.
- **Saldo DEBTs: 11 preservado** (DEBT-30/34c/34e/56 sentinelas
  preservadas; sem reabertura; sem novo DEBT).
- **43 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P251** (4):
  - "Slice frame items at height" N=1 inaugurado.
  - "DeferredX buffer + flush em new_page" N=1 → 2 cumulativo
    (P245 floats + P251 cell tails).
  - "Aplicação citante ADR-0082 PROPOSTO" N=1 → **2 cumulativo**.
  - "Spec C1 audit obrigatório bloqueante" N=13 → **14
    cumulativo**.
- "Promoção real scope-out ADR-0054 graded" granular N=12 →
  **13 cumulativo** (P251 ×1).
- **Scope-outs originais Block fechados**: 10/10 preservado.
- **Scope-outs originais Boxed fechados**: 5/6 preservado.
- **Categoria A.4 Fase 5 Layout**: muito reforçada
  cumulativamente.
- **Categoria C.2 Fase 5 Layout**: **parcialmente cumprida P251**
  (cell-level row break real); multi-region completo continua
  diferido (DEBT-56 fechamento depende).
- **Marco interno**: TableCell row break real cell-level γ-Items
  materializado; segunda aplicação citante ADR-0082; sub-padrão
  N=1 "slice frame items at height" inaugurado; padrão
  "DeferredX buffer + flush em new_page" N=2 cumulativo;
  Categoria C.2 Fase 5 Layout parcialmente activada (cell-level
  apenas; multi-region completo continua diferido); lição C1
  audit N=14 cumulativa refinada procedimentalmente; primeiro
  passo onde audit C1 fixa abordagem γ-Items vs γ-Content
  empíricamente baseado em pos.y locality.

---

## §7 Notas operacionais para o executor

1. **Audit C1 BLOQUEANTE prioridade absoluta**. Não materializar
   antes de §2.1-§2.8 completos. **Lição N=14 cumulativa**:
   refino procedural "audit C1 deve confirmar localidade pos.y
   antes de fixar abordagem γ-Items vs γ-Content para slicing".
   §2.5 (referência vanilla `lab/typst-original/crates/
   typst-layout/src/grid/rowspans.rs` ou análogo) é **crítica**
   para Decisões 6-8 (limitação atomic, recursive overflow,
   re-emit stroke+fill).

2. **Decisões 6, 7, 8 final fixas pós-audit §2.8**. Algoritmos
   preliminares em §3 são hipóteses; decisão final baseada em
   achado empírico §2.5 vanilla.

3. **Refactor `grid.rs:393-433`** crítico — substituir P248
   clip implícito por γ-Items slice + tail push. **Preservar
   P248 clip implícito para cells em rows Fixed** (paridade
   vanilla; documentar limitação consciente). Test sentinela:
   `p251_table_cell_overflow_row_fixed_preserva_p248_clip`.

4. **Ordem de implementação recomendada**:
   1. Audit C1 §2 completo (~30-45 min — inclui leitura
      vanilla obrigatória §2.5).
   2. Decisões finais §3 (~15-20 min documentação).
   3. Helper `slicing.rs` + `rebase_item_y` + tests slice
      (~60-90 min).
   4. Layouter `pending_cell_tails` + `DeferredCellTail` +
      `flush_pending_cell_tails` (~60-90 min).
   5. Refactor `grid.rs` arm cell overflow + tests E2E
      (~90-120 min — maior parcela).
   6. Tests adaptações P248 → P251 + regression Fixed rows
      (~60-90 min).
   7. Anotações ADRs + inventário 148 + relatório (~30-45 min).

   **Total ~6-8h** paridade L magnitude.

5. **Backward compat para cells sem overflow + Fixed rows**:
   defaults preservam output literal P248. Test sentinela
   `p251_cell_sem_overflow_preserva_p248_output_literal` +
   `p251_cell_overflow_row_fixed_preserva_p248_clip`.

6. **Custo real esperado**: ~6-8h (paridade L magnitude). Maior
   parcela: refactor `grid.rs` arm + tests E2E (~40%); helper
   `slicing.rs` (~25%); Layouter pending pattern (~20%);
   audit C1 + anotações (~15%).

7. **`P251.div-N` cenários antecipados em §2.8**. Activar se:
   - Vanilla é γ-Content significativamente diferente
     (`P251.div-1`) — **NÃO re-escopo**, documentar como
     limitação consciente.
   - FrameItem variants adicionais (`P251.div-2`).
   - DeferredFloat pattern incompatível (`P251.div-3`).
   - Baseline ≠ 2276 (`P251.div-4`).

8. **Cita ADR-0082 PROPOSTO explícitamente**. Relatório P251
   §"Citação ADR-0082" lista 4 critérios verificados (paridade
   P250 relatório §"Citação ADR-0082"):
   1. Storage prévio ✓ (scope-out P157B "ignorados em layout"
      → P248 graded "clip implícito" → P251 real).
   2. Consumer Layouter pre-promoção graded ✓ (P248 clip
      implícito é graded — não é semantic real "row break").
   3. Paridade vanilla referência empírica ✓ (audit §2.5).
   4. Backward compat literal ✓ (test sentinela).

   **Validação ADR-0082 N=2 citante**.

9. **Marco "Categoria C.2 Fase 5 Layout parcial"**. P251 activa
   cell-level row break — primeiro elemento da Categoria C.2
   "cell-level multi-region completion" materializado. Multi-
   region completo (DEBT-56 column flow) continua diferido.
   Documentar em relatório §"Marco P251" como milestone
   conceptual: **primeira aplicação cumulativa Categoria C.2
   Fase 5 Layout**.

10. **Anti-inflação 43ª aplicação cumulativa** pós-P205D
    preservar: Opção β L0 minimal (`region.md` hash propagado
    se documentado) + Opção α activação consumer real (refino
    directo P248) + Opção α reuso DeferredFloat pattern P245
    (paridade arquitectural) + Opção α helper novo em ficheiro
    próprio (paridade subpadrão "tipo entity em ficheiro
    próprio") + Opção α anotação cumulativa minimal ADRs
    (0061+0079+0080+0054+**0082 citação segunda**) + Opção α
    sub-padrão N=1 inaugurado "slice frame items at height"
    + Opção α DEBT-34e preservado aberto distinção explícita.

11. **Sticky-style limitation note**: γ-Items limitação atomic
    (items que internamente ultrapassam threshold vão completos
    para tail) é **paridade vanilla preservada** (atomic block
    can't split mid-paragraph). Documentar **explicitamente**
    em §"Limitações conscientes γ-Items" do relatório P251
    para clarificar distinção γ-Items vs γ-Content.
