# Diagnóstico Fase A P273.7.A — Boxed save/restore parent_bbox (completa Decisão 3 P273.6)

**Data**: 2026-05-17.
**Passo**: typst-passo-273.7.A.
**Magnitude**: XS-S documental (~20 min).
**Cluster**: Visualize / Gradient (extensão final do refino estrutural).
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085.
**Décimo oitavo consumo directo de fonte** (cristalino post-P273.6 +
template Block save/restore replicado literal — sub-padrão emergente
"Template-passo replicado").

---

## §A.1 — Inventário do arm `Content::Boxed` no Layouter

`01_core/src/rules/layout/mod.rs`:

- **Linha 1340** — arm `Content::Boxed { body, width, height, inset,
  baseline, outset, radius, clip, fill, stroke }` inicio.
- **Linha 1341** — `let font = self.font_size_pt.val();`.
- **Linhas 1342-1356** — inset/outset resolvidos; `has_shape` /
  `has_outset` derivados de `fill`/`stroke`.
- **Linha 1360** — `items_before` snapshot para Z-order Shape pré-body.
- **Linhas 1366-1371** — `start_x` captado pré-outset; cursor.x avança
  outset_left + inset_left.
- **Linhas 1377-1382** — width promotion (`saved_width` /
  `regions.current.width` clamp; P243).
- **Linha 1384** — `let _ = baseline;` (armazenado; refino futuro).
- **Linha 1392** — `body_items_before` snapshot pré-body.
- **Linha 1394** — `self.layout_content(body);` (callsite único do body).
- **Linha 1397** — restore saved_width.
- **Linhas 1400-1433** — clip-overflow handling (P248): se
  `height: Some(h) && clip && body_h_real > h_pt` envolve items
  emitidos em `FrameItem::Group { clip_mask: Rect }`.
- **Linhas 1436-1439** — cursor.x += inset_right + outset_right.
- **Linhas 1485-1496** — emit `FrameItem::Shape` final do próprio
  Boxed (pos, width, height, fill, stroke, **`parent_bbox_at_emit:
  self.parent_bbox`** — populated desde P273.6).

**Largura efectiva do Boxed**: `width: Option<Length>`:
- `Some(w) → w.resolve_pt(font)` (clamp width literal).
- `None → avanço horizontal natural` (line-fill).

**Altura efectiva do Boxed**: `height: Option<Length>`:
- `Some(h) → h.resolve_pt(font)` (literal).
- `None → line_height` proxy (P156H limitação consciente).

**Eixo Y inline**: cursor.y é **baseline da linha em curso** —
contexto inline não avança cursor.y. Topo do box ≈
`cursor.y - line_h` (computed apenas no shape emit, linha 1459).

---

## §A.2 — Inventário do arm `Content::Block` pós-P273.6 (referência template)

`01_core/src/rules/layout/mod.rs`:

- **Linha 1514** — arm `Content::Block { ... }` inicio.
- **Linhas 1642-1661** — save/restore parent_bbox path `clip=true`:

```rust
// P273.6 — save/restore parent_bbox (Decisão 3γ.2.γ:
// popular apenas quando width+height literais).
let saved_parent_bbox = self.parent_bbox;
if let (Some(w), Some(h)) = (width, height) {
    let w_pt = w.resolve_pt(font);
    let h_pt = h.resolve_pt(font);
    self.parent_bbox = Some(crate::entities::layout_types::Rect {
        x: self.regions.current.cursor_x,
        y: self.regions.current.cursor_y,
        w: Pt(w_pt),
        h: Pt(h_pt),
    });
}

// Layout body (acumula items em current_items).
self.layout_content(body);
self.flush_line();

// P273.6 — restore parent_bbox (LIFO).
self.parent_bbox = saved_parent_bbox;
```

- **Linhas 1685-1703** — save/restore parent_bbox path `clip=false`
  (literal idêntico ao clip=true sem `flush_line` adicional após).
- **Linha 1767-1778** — emit `FrameItem::Shape` final do Block usa
  `self.parent_bbox` (restaurado outer — emit do próprio Block
  resolve para contentor outer per spec P273.6 §2.3).

**Diferenças estruturais Block vs Boxed para o save/restore**:

| Aspecto | Block | Boxed |
|---|---|---|
| Paths `layout_content(body)` | 2 (clip=true + clip=false) | 1 (linha 1394) |
| `flush_line` pós-body | Sim (estrutural) | Não (inline) |
| `cursor.y` semântica | Topo do block (estrutural) | Baseline da linha (inline) |
| Shape emit do próprio container | Pós-cursor_y advance | Pós-cursor_x advance |
| `parent_bbox_at_emit` no emit próprio | `self.parent_bbox` (outer) | `self.parent_bbox` (outer; já P273.6) |

---

## §A.3 — Decisão 1 fixada: semântica bbox.y para Boxed inline

**Opções analisadas** (per spec §A.3):

1. **3γ.2.γ-inline-baseline-y**: `bbox.y = self.regions.current.cursor_y`
   literal (baseline-relative). **Recomendada pela spec.**
2. **3γ.2.γ-inline-topo-estimado**: `bbox.y = cursor.y - ascender`.
   Tenta o topo do box via font ascender.
3. **3γ.2.γ-inline-defer**: não popular `parent_bbox` em Boxed;
   fallback page_bbox preservado.

**Decisão fixada**: **3γ.2.γ-inline-baseline-y**.

**Razões**:

1. **Sem dependência adicional na font**: o save/restore não precisa
   chamar `self.metrics.vertical_metrics(...)`; mantém o template
   Block literal com apenas a substituição `cursor.x/cursor.y`.
2. **Coerente com limitação consciente pré-existente P156H**: "height
   em contexto inline alteraria line_height — refino futuro".
   Aproximar bbox.y por baseline é a única posição empiricamente
   conhecida e estável no arm Boxed sem refactor inline line_height.
3. **Observable diff alcançável mesmo com aproximação**: para
   gradient `relative=parent` num box 200×100pt dentro de page
   595×842pt, a transform PDF gerada é **distinta** do fallback
   page_bbox (escalas e offsets diferentes). Test E2E §5 confirma
   observable.
4. **ADR-0054 graded**: precisão exacta (refino topo-relative) fica
   pendente como `P273.X-bis2` se houver demanda empírica.

**Implementação literal**:

```rust
self.parent_bbox = Some(Rect {
    x: self.regions.current.cursor_x,
    y: self.regions.current.cursor_y,  // baseline-relative
    w: Pt(w_pt),
    h: Pt(h_pt),
});
```

---

## §A.4 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Regressão tests P262-P273.6 | Save/restore Boxed afecta outras shapes inline | Defaults: Boxed sem dimensions literais → `parent_bbox` outer preservado (idêntico Block). 2612 baseline preserved. |
| bbox.y semanticamente errada | Aproximação baseline-relative vs topo-relative | Limitação consciente documentada §5 spec; coerente com P156H. ADR-0054 graded. Refino diferido `P273.X-bis2`. |
| Cap LOC estourado | Spec menor que P273.6 (apenas 1 arm save/restore) | Cap hard L1 30 LOC; soft 20 LOC. Real esperado ~15-20 LOC. |
| `#[allow(dead_code)]` reabrir | Nenhuma estrutura nova; emit shape sites já lêem `self.parent_bbox` desde P273.6 | Zero `#[allow]` introduzido. `cargo build` zero warnings preserved. |
| Test E2E não-observable | Boxed bbox.y aproximada produz transform pouco distinguível | Test E2E #5 usa box 200×100pt vs page 595×842pt — bbox real visualmente distinta de page; bytes PDF DIFEREM (paralelo `p273_6_shape_inside_block_carries_parent_bbox_observable_diff`). |
| Interacção clip-overflow P248 | Clip-overflow handling pós-body (linhas 1400-1433) chama `measure_content_constrained` | `measure_content_constrained` é puro de medição; não emite shapes; `parent_bbox` restaurado ANTES da medição. Sem efeito colateral. |

---

## §A.5 — Decisões a fixar na Fase A

1. **Decisão 1 (semântica bbox.y inline)**: **3γ.2.γ-inline-baseline-y**
   — `bbox.y = self.regions.current.cursor_y` literal. Aproximação
   aceitável per §A.3 razões 1-4. ADR-0054 graded.
2. **Confirmação herdadas P273.6 literal**:
   - **Decisão 2 (semântica bbox W/H)**: 3γ.2.γ — popular apenas
     quando `width.is_some() && height.is_some()`. Idêntica Block.
   - **Decisão 3 (propagação L1→L3)**: Prop-A revisitada. Inalterada
     P273.6 — emit shape sites já populam `parent_bbox_at_emit:
     self.parent_bbox` desde P273.6 (linha 1495 confirmada literal).
   - **Decisão 4 (escopo)**: P273.7 estende Decisão 3 P273.6 de
     `{Block}` para `{Block, Boxed}`. Stack/Pad/Group/Grid cell
     continuam scope-out per ADR-0054 graded.

3. **Confirmação que P273.7 NÃO introduz `#[allow(dead_code)]` novo**:
   - Nenhuma estrutura nova adicionada (FrameItem::Shape,
     GradientObject, Layouter `parent_bbox` campos pré-existem).
   - O arm Boxed apenas ganha 2 statements (save + restore) +
     1 if-block (set).
   - `cargo build` zero warnings preserved (P273.6 fechou já o
     `parent_bbox` dead_code; emit shape sites no Boxed já lêem
     `self.parent_bbox` desde P273.6).

---

## §A.6 — Critério de aceitação Fase A

- ✓ §A.1 cita arm Boxed literal (`mod.rs:1340`) + body callsite
  (`mod.rs:1394`) + emit shape site interno (`mod.rs:1485-1496`,
  `parent_bbox_at_emit: self.parent_bbox` populated desde P273.6).
- ✓ §A.2 cita template P273.6 Block (clip=true `mod.rs:1642-1661`,
  clip=false `mod.rs:1685-1703`) que será replicado para Boxed.
- ✓ §A.3 Decisão 1 fixada: **3γ.2.γ-inline-baseline-y** (baseline-
  relative cursor.y; aproximação documentada).
- ✓ §A.4 risco "regressão P262-P273.6" mitigado por save/restore
  análogo Block (defaults LIFO restore).
- ✓ §A.5 confirmação literal — P273.7 NÃO introduz `#[allow(dead_code)]`
  novo; herdam Decisões 2/3/4 de P273.6.

**Fase A produzida — critério §A.6 cumprido absoluto.**

---

## §A.7 — Plano de implementação (Fase C)

### Cap LOC (ADR-0094 Pattern 1)

- **L1 hard cap**: ≤ 30 LOC.
- **L1 soft cap**: ≤ 20 LOC.
- **L3 hard cap**: 0 LOC.
- **Tests hard cap**: ≤ 8 novos.
- **Tests soft cap**: ≤ 5 novos.

### Ordem literal

1. Fase A — completa neste documento.
2. ADR-0091 — nona anotação cumulativa (§"Anotação cumulativa P273.7").
3. L0 `entities/gradient.md` — anotação P273.7.
4. `crystalline-lint --fix-hashes` — propaga hash gradient.md.
5. Tests-first (3-5 novos; cap soft 5):
   - `p273_7_boxed_save_restore_parent_bbox` — entrar em Boxed com
     `width=Some, height=Some` guarda bbox; sair restaura LIFO.
     Smoke test estrutural (Rect compose).
   - `p273_7_gradient_relative_parent_inside_boxed_observable_diff`
     — E2E PDF: box bbox real ≠ page → bytes DIFEREM vs fallback.
     **Crítico para confirmar 3γ.2 dá semântica real Boxed.**
   - `p273_7_relative_self_preserved_with_parent_bbox_boxed` — Self_
     ignora `parent_bbox_at_emit` (paridade Block).
6. Código L1 (~15-20 LOC esperado):
   - Arm `Content::Boxed` — adicionar save/restore parent_bbox
     antes/depois `self.layout_content(body)` (linha 1394).
7. Verificação final — cargo build, cargo test, lint zero.

### Mecânica de inserção

Antes de `self.layout_content(body);` (linha 1394 actual):

```rust
// P273.7 — save/restore parent_bbox análogo Block P273.6
// (Decisão 3γ.2.γ). Decisão 1: bbox.y baseline-relative (cursor.y
// literal; coerente com P156H limitação line_height inline).
let saved_parent_bbox = self.parent_bbox;
if let (Some(w), Some(h)) = (width, height) {
    let w_pt = w.resolve_pt(font);
    let h_pt = h.resolve_pt(font);
    self.parent_bbox = Some(crate::entities::layout_types::Rect {
        x: self.regions.current.cursor_x,
        y: self.regions.current.cursor_y,
        w: Pt(w_pt),
        h: Pt(h_pt),
    });
}
```

Depois de `self.layout_content(body);` (linha 1394 actual):

```rust
// P273.7 — restore parent_bbox (LIFO).
self.parent_bbox = saved_parent_bbox;
```

**Posicionamento literal**: restore IMEDIATAMENTE após
`layout_content(body)`. Razão: shape emission do próprio Boxed
(linha 1485-1496) usa `self.parent_bbox` outer — paridade Block
(spec P273.6 §2.3); clip-overflow handling (linhas 1400-1433) é
medição pura sem efeito sobre `parent_bbox`.

---

## §A.8 — Sub-padrões emergentes

- **"Template-passo replicado literal"** N=0 → **N=1 emergente** —
  P273.7 aplica save/restore P273.6 literalmente a outro arm com
  diferença mínima (bbox.y semantic baseline-relative vs topo).
  Análogo histórico P156H replicando P156G ("padrão Block reaplicado
  a Boxed sem nova decisão arquitectural"). Promoção a sub-padrão
  consolidado candidato N=3-4.
- **"Sub-passos decimais consecutivos do mesmo cluster"** N=2 →
  **N=3 emergente** — P273.5 + P273.6 + P273.7.
- **"Diagnóstico imutável"** N=22 → **N=23 cumulativo** (décimo
  oitavo consumo directo de fonte).

---

*Diagnóstico imutável produzido em 2026-05-17. Decisão 1 fixada
absoluta; critério §A.6 cumprido; pronto para Fase B (ADR-0091
anotação) + Fase C (materialização L1 ~15-20 LOC + 3-5 tests).*
