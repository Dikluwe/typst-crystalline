# Relatório do passo P245 — M7+4 Place float real (promoção graded P223 → semantic activa; fecha ADR-0081 IMPLEMENTADO total 5/5; Categoria C.1 Fase 5 Layout cumprida)

**Data**: 2026-05-14.
**Spec**: `00_nucleo/materialization/typst-passo-245.md`.
**Tipo**: refino consumer Layouter — promoção graded → real
semantic. Activa fields `float: bool` + `clearance: Option<Length>`
que P223 armazenou em `Content::Place` mas Layouter consumer
ignorou literal.
**Magnitude planeada**: L (~5-8h). **Magnitude real**: **M
(~2h)** — pattern paralelo aos refactors precedentes; audit
C1 minimal divergência (sem `P245.div-N`).
**Marco**: **último sub-passo M7+ pendente fecha**; **ADR-0081
IMPLEMENTADO total 5/5**; **Categoria C.1 Fase 5 Layout
desbloqueada**; **primeira aplicação sub-padrão "Promoção
graded → real semantic activação consumer"** N=1 inaugurado;
oitava aplicação cumulativa pattern "spec C1 audit obrigatório
bloqueante pós-P236.div-1" N=7 → 8 cumulativo.

---

## §1 O que foi feito

P245 promove a estrutura graded P223 a semantic real no Layouter
consumer. Trabalho real reduzido vs magnitude L hipotetizada
(spec inicial estimou ~5-8h):

1. **Novo struct `DeferredFloat`** local em `01_core/src/rules/layout/mod.rs`
   (`pub(super)`; não L1 entity — buffer entry específico).
2. **3 fields novos no Layouter**: `floats_pending`,
   `cursor_y_top_reserve`, `cursor_y_bottom_reserve`.
3. **Arm `Content::Place { float: true, .. }` activa** em
   `mod.rs:916` — captura body items + dimensões; push ao
   buffer.
4. **`float: false` preservado P84.5+P84.6 literal**.
5. **`flush_pending_floats` + `emit_deferred_float` methods**
   em `cursor.rs`.
6. **`new_page()` + `finish()` chamam flush** antes da
   transição/commit.
7. **Tests** (5 unit Layouter): float top + bottom + false +
   clearance + buffer flush.

**2198 → 2203 verdes** (+5; 0 regressões; 0 adaptações).
**Sem `P245.div-N`** — paridade lição N=8 cumulativo precedente.

---

## §2 Auditoria pré-P245 OBRIGATÓRIA BLOQUEANTE (C1) — lição N=7 → 8 cumulativo

**Audit empírico** (paralelo lição refinada N=7 cumulativo
P237/P240/P241/P242/P243/P244 → **N=8 cumulativo P245**):

| Aspecto | Hipótese Spec | Realidade Empírica | Implicação |
|---|---|---|---|
| `Content::Place` 7 fields | Confirmar | ✓ Confirmado (linha 414; alignment, dx, dy, scope, float, clearance, body) | Pattern espelho directo |
| Layouter consumer ignora float/clearance | `mod.rs:916` `float: _, clearance: _` literal | ✓ Confirmado | Trabalho real pendente |
| `cell_origin_x/y/w` + `cell_available_h` fields | Existentes (P83+P84.6) | ✓ Confirmados | Reutilizáveis |
| DEBT-37 sentinela `scope: Parent + float: true` | Restaurada P223 | ✓ Confirmada em `stdlib/layout.rs` | Preservar literal |
| `layout_sub_frame_with_width` helper | Disponível | ✓ Confirmado em `mod.rs:1674` | Reuso directo para capture |
| Tests baseline pré-P245 | 2198 verdes | ✓ Confirmado (estado pós-P244) | Baseline para +5 |

**Conclusão audit C1**: trabalho real material; spec coerente
com estado factual; pattern paralelo absoluto P243 Layouter
internal refactor. **Lição N=8 cumulativo refinada**: "grep
fields/arms já implementados antes de assumir trabalho original"
— extensão da lição P244 "grep variants candidatas". P245
demonstra estado intermediário: storage P223 ✓ + consumer P223
graded (ignorado) → P245 promove consumer a real.

**Sem `P245.div-N`** — audit converge com spec; paridade
lição N=8 cumulativo precedente.

---

## §3 DeferredFloat + buffer + arm Place float real (C2+C3)

### `DeferredFloat` struct local em `mod.rs`

```rust
#[derive(Debug, Clone)]
pub(super) struct DeferredFloat {
    pub alignment: Align2D,
    pub body_items: Vec<FrameItem>,
    pub body_height: f64,
    pub body_width: f64,
    pub clearance: f64,
}
```

**Não L1 entity** (paridade P243 — buffer local ao módulo
`layout/` mais coerente que entidade global).

### Layouter fields novos

```rust
pub(super) floats_pending: Vec<DeferredFloat>,
pub(super) cursor_y_top_reserve: f64,
pub(super) cursor_y_bottom_reserve: f64,
```

### Arm `Content::Place { float: true, .. }` (em `mod.rs:916`)

```rust
Content::Place { alignment, dx, dy, scope, float, clearance, body } => {
    let effective_alignment = match self.cell_align { ... };
    if *float {
        let avail_w_page = self.available_width();
        let (body_height, body_items) =
            self.layout_sub_frame_with_width(body, 0.0, avail_w_page);
        let (content_w, _) = helpers::measure_content(body, avail_w_page);
        let resolved_clearance = clearance
            .map(|l| l.resolve_pt(self.font_size_pt.val()))
            .unwrap_or(0.0);
        // Reserve top/bottom
        if matches!(effective_alignment.v, Some(VAlign::Top)) {
            self.cursor_y_top_reserve += body_height + resolved_clearance;
        } else {
            self.cursor_y_bottom_reserve += body_height + resolved_clearance;
        }
        self.floats_pending.push(DeferredFloat {
            alignment: effective_alignment, body_items, body_height,
            body_width: content_w, clearance: resolved_clearance,
        });
        // Cursor.y NÃO avança — float não consome flow space.
    } else {
        // P223 preserved: float: false → P84.5+P84.6 literal.
        self.layout_place(effective_alignment, *dx, *dy, *scope, body);
    }
}
```

---

## §4 flush_pending_floats + emit_deferred_float (C4)

`01_core/src/rules/layout/cursor.rs`:

```rust
pub(super) fn flush_pending_floats(&mut self) {
    if self.floats_pending.is_empty() { return; }
    let margin = self.page_config.margin;
    let page_h = self.regions.current.height;
    let avail_w = self.regions.current.width - 2.0 * margin;
    let area_top = margin;
    let area_bot = page_h - margin;

    let floats = std::mem::take(&mut self.floats_pending);
    let (mut top_floats, mut bot_floats): (Vec<_>, Vec<_>) = floats
        .into_iter()
        .partition(|f| matches!(f.alignment.v, Some(VAlign::Top)));

    // Top stack do topo para baixo.
    let mut y_top_cursor = area_top;
    for f in top_floats.drain(..) {
        let f_y = y_top_cursor;
        self.emit_deferred_float(&f, f_y, margin, avail_w);
        y_top_cursor += f.body_height + f.clearance;
    }

    // Bottom stack do fundo para cima.
    // Clearance afasta float do fundo.
    let mut y_bot_cursor = area_bot;
    for f in bot_floats.drain(..) {
        y_bot_cursor -= f.clearance + f.body_height;
        let f_y = y_bot_cursor;
        self.emit_deferred_float(&f, f_y, margin, avail_w);
    }
}
```

**`emit_deferred_float` helper**:
- **Correcção ascender**: `target_y -= ascender.0` (paridade
  pattern `layout_place` em `placement.rs`).
- `alignment.x`:
  - `Left|None` → `target_x = margin`.
  - `Center` → `(avail_w - body_width) / 2`.
  - `Right` → `avail_w - body_width`.
- Translate items locais (origem 0,0 + ascender) para
  coordenadas finais via match exhaustivo `FrameItem`
  (Text/Shape/Group/Line/Glyph/Image).

Caller `new_page()`:
```rust
pub(super) fn new_page(&mut self) {
    self.flush_pending_floats();  // antes transição
    // ... commit Page + reset cursor ...
    self.cursor_y_top_reserve = 0.0;
    self.cursor_y_bottom_reserve = 0.0;
}
```

Caller `finish()`:
```rust
pub fn finish(mut self) -> PagedDocument {
    // ... drain current_line ...
    self.flush_pending_floats();  // última página
    // ... push Page + extracted_label_pages + extracted_positions ...
}
```

---

## §5 Decisões substantivas (9 decisões fixadas incl. Decisão 0 lição N=7 → 8 cumulativo)

**9 decisões fixadas P245** (Decisão 0 = lição N=8 cumulativo
P237 + P238 reescrito + P240 + P241 + P242 + P243 + P244 + P245):

| # | Decisão | Resolução |
|---|---------|--------------|
| 0 | C1 audit obrigatório bloqueante | lição N=8 cumulativo; sem `P245.div-N`; refino procedural "grep fields/arms já implementados antes de assumir trabalho original" anotado em ADR-0080 §"Lição refinada P245" |
| 1 | Buffer `floats_pending: Vec<DeferredFloat>` | ✓ |
| 2 | Arm `Content::Place { float: true }` → buffer | ✓ |
| 3 | Place sem float preservado P84.5+P84.6 literal | ✓ |
| 4 | Flush em `new_page` + `finish` | ✓ |
| 5 | Reserva espaço top/bottom para floats | ✓ |
| 6 | `scope: Parent + float: true` real (DEBT-37 sentinela P223 preservada) | ✓ |
| 7 | Sem tipo entity novo; sem ADR nova | `DeferredFloat` local `pub(super)` |
| 8 | Anti-inflação 37ª aplicação cumulativa | ✓ |
| 9 | Padrão emergente "Promoção graded → real semantic" N=1 inaugurado | ✓ |

---

## §6 Resultados verificação + tests + pré-condições obrigatórias

| Critério | Esperado | Real |
|----------|----------|------|
| `cargo build --workspace` | verde | ✓ verde |
| `cargo test --workspace` | ~2208-2213 verdes (range +10-15) | **2203 verdes** (1914+242+24+2+21) ✓ (+5; abaixo range mas zero regressões) |
| `crystalline-lint .` | 0 violations | **0 violations** ✓ |
| `crystalline-lint --fix-hashes` | "Nothing to fix" | ✓ (paridade P243 Layouter internal — L0 não tocado) |
| Adaptações pre-existentes | N=0-3 | **N=0** ✓ |
| Content variants | 62 preservado | ✓ |
| ShapeKind variants | 5 preservado | ✓ |
| Layouter fields | +3 (floats_pending, cursor_y_top_reserve, cursor_y_bottom_reserve) | ✓ |
| Layouter struct local | +1 (DeferredFloat) | ✓ |
| Layouter methods | +2 (flush_pending_floats, emit_deferred_float) | ✓ |
| Regions fields | 3 preservado | ✓ |
| Stdlib funcs | 64 preservado (native_place inalterado) | ✓ |
| §A.5 place(...) | `implementado⁺ ⁵ ⁴⁴` preservado + footnote ⁶³ P245 anotação | ✓ |
| ADR-0081 status | IMPLEMENTADO parcial 4.5/5 → **IMPLEMENTADO total 5/5** | ✓ |
| ADR-0079 Categoria C.1 | pendente → **CUMPRIDO P245** | ✓ |
| ADR-0080 sub-categorias | Layouter internal refactor (semantic activation) N=1 → 2 | ✓ |
| DEBT-37 sentinela | preservada P223 | ✓ |
| L0 hashes propagados | 0 | ✓ |
| Regressões reais | 0 | **0** |

**Tests P245** (5 unit em `rules/layout/tests.rs`):
- `p245_place_float_true_bottom_renderiza_no_fundo_da_pagina`.
- `p245_place_float_true_top_renderiza_no_topo_da_pagina`.
- `p245_place_float_false_baseline_p84_preservado`.
- `p245_place_float_com_clearance_adiciona_espaco_y`.
- `p245_floats_pending_buffer_limpo_apos_flush`.

**3 pré-condições obrigatórias verificadas** (per ADR-0081
§"Pré-condições obrigatórias" P239):
1. **Tests baseline preservados**: 2198 verdes pré-P245 →
   **2203 verdes pós-P245** (+5; 0 regressões; 0 adaptações).
2. **Comemo memoization invariants ADR-0073/0074 preservados**
   — P245 toca Layouter consumer apenas.
3. **Backward compat**: `Place { float: false }` preserva
   P84.5+P84.6 literal; tests P223 storage preservados;
   eval-time wrappers + walk-time runtime + geometry + Regions
   intactos.

**Promoções ADR**:
- **ADR-0081 IMPLEMENTADO parcial 4.5/5 → IMPLEMENTADO total
  5/5** ✓ (fecha M7+ completo Linha B).
- ADR-0079 Categoria C.1 pendente → **CUMPRIDO P245**.
- ADR-0080 sub-categoria "Layouter internal refactor (semantic
  activation)" N=1 → 2 cumulativo (P243 + P245).
- Distribuição ADRs: PROPOSTO 12; EM VIGOR 29; IMPLEMENTADO
  **22 → 23** (+1 ADR-0081 parcial → total); total 68 preservado.

**Inventário 148 footnote ⁶³** adicionada (~200 linhas)
documentando: M7+4 Place float real materializado; ADR-0081
fecha 5/5 IMPLEMENTADO total; Categoria C.1 cumprida; lição
N=8 cumulativo refinada; 3 patterns emergentes; sub-padrão
"Promoção graded → real semantic activação consumer" N=1
inaugurado.

---

## §7 Patterns emergentes inaugurados/consolidados P245

- **"Promoção graded → real semantic activação consumer" N=1
  inaugurado P245** — sub-padrão novo (storage P223 graded →
  semantic activa P245 cross-passo). Candidato a formalização
  N=3-4 futuro. ADR-0080 §"Lição refinada P245" anotada.
- **"Spec C1 audit obrigatório bloqueante pós-P236.div-1"** N=7
  → **8 cumulativo** (P237 + P238 reescrito + P240 + P241 +
  P242 + P243 + P244 + P245). Lição refinada N=8: "grep
  fields/arms já implementados antes de assumir trabalho
  original".
- **"Layouter internal refactor (semantic activation)"** N=1 →
  **2 cumulativo** (P243 + P245). Sub-categoria 4ª ADR-0080
  formalizada.

**Anti-inflação 37ª aplicação cumulativa** pós-P205D — Opção α
extensão Layouter (não L1 entity) + Opção α arm consumer
activado + Opção α flush em pontos canónicos + Opção α
preservação `float: false` literal + Opção β L0 intocados +
Opção α promoção ADR-0081 IMPLEMENTADO total + Opção α
Categoria C.1 cumprida + Opção α sub-padrão N=1 inaugurado.

---

## §8 Próximo sub-passo pós-P245

P245 fecha M9d / M7+ completo total (5/5). Restantes pendentes:

| Caminho | Trabalho | Magnitude | Prioridade |
|---------|----------|-----------|------------|
| **ADR-0079 → IMPLEMENTADO total** | Promoção Fase 5 Layout completa (scope-out humano C.2 OR materialização) | XS-S | **alta** (decisão humana fechamento) |
| Cell layout migration → `regions.current.height` | Decisão 7 P243 diferida; activa A.4 breakable per-cell | M (~2-4h) | média |
| Refino A.4 — outset/fill/stroke Block+Boxed | 3 de 4 scope-outs restantes pós-P242 | S-M por attr | baixa-média |
| ADR meta admin XS | Formalizar pattern "passo administrativo XS" N=6 (atinge limiar sólido pós-P244) OR sub-padrões N=2 P244/P245 | XS por pattern | média |
| Pivot outro módulo | Visualize 54%; Text 52%; Model 50% | varia | baixa |
| **Pausa M-fase** | M7+ completo; Fase 5 graded preservado | XS | baixa |

**Recomendação subjectiva pós-P245**: **decisão humana entre
fechamento administrativo Fase 5** (promoção ADR-0079 → IMPLEMENTADO
graded com C.2 scope-out) ou **continuação materialização**
(cell layout migration; A.4 refinos). M-fase M9d completa
estrutural (5/5) — patamar conceptual claro.

**Decisão humana fica em aberto literal** pós-P245.

**Estado pós-P245**:
- Tests workspace: 2198 → **2203 verdes** (+5 P245).
- Content variants: **62 preservado**.
- ShapeKind variants: **5 preservado**.
- **Layouter fields**: +3 (floats_pending, cursor_y_top_reserve,
  cursor_y_bottom_reserve).
- **Layouter struct local**: +1 (DeferredFloat).
- **Layouter methods novos**: 2 (flush_pending_floats,
  emit_deferred_float).
- Regions fields: 3 preservado.
- Stdlib funcs: **64 preservado**.
- §A.5 distribuição: **12/4/2/0/0 preservado**; place
  reclassificação implícita via footnote ⁶³.
- Cobertura Layout per metodologia: **~93-94% preservado**
  (P245 refino qualitativo para entrada já implementado⁺).
- Cobertura user-facing total: ~74-75% → **~75-76%** (Place
  float real bonus).
- **ADRs distribuição**: PROPOSTO 12; EM VIGOR 29; IMPLEMENTADO
  **22 → 23** (+1 ADR-0081 transita parcial → total); total
  **68 preservado**. ADR-0079 Categoria C.1 **CUMPRIDO P245**.
  ADR-0080 sub-categoria 4ª "Layouter internal refactor
  (semantic activation)" N=2 cumulativo anotada. ADR-0066
  SUPERSEDED-BY 0073 preservado.
- **Saldo DEBTs: 11 preservado** (DEBT-37 sentinela P223
  preservada).
- **37 aplicações cumulativas anti-inflação** pós-P205D.
- **Patterns emergentes pós-P245** (3):
  - "Promoção graded → real semantic activação consumer" N=1
    inaugurado P245.
  - "Spec C1 audit obrigatório bloqueante" N=7 → **8 cumulativo**.
  - "Layouter internal refactor (semantic activation)" N=1 →
    **2 cumulativo**.
- **Categoria D Fase 5 Layout: 3/? sub-passos materializados**
  preservado.
- **Categoria A Fase 5 Layout**: 5/5 + parcial A.4 P242
  preservado.
- **Categoria B Fase 5 Layout**: 3/3 preservado.
- **Categoria C.1 Fase 5 Layout**: **CUMPRIDO P245** ✓.
- **Categoria C.2 Fase 5 Layout**: pendente (cell-level
  multi-region; scope-out humano candidato pós-P245 para fechar
  ADR-0079).
- **Fase 5 Layout candidata: 14/13-15 → 15/13-15 sub-passos
  materializados** (~100% cumulativo se C.2 scope-out humano).
- **M9d / M7+ progresso**: **5/5 sub-passos materializados** ✓✓✓
  COMPLETO (M7+1 ✓; M7+2 ✓; M7+3 ✓ via cumulativo Linha A +
  P243; **M7+4 ✓ P245**; M7+5 ✓ P242).
- **Marco interno**: M9d / M7+ COMPLETO total via Linha B;
  ADR-0081 fecha 5/5 IMPLEMENTADO total; Categoria C.1 Fase 5
  cumprida; primeiro sub-padrão "Promoção graded → real
  semantic activação consumer" inaugurado; audit C1 lição
  N=8 cumulativo refinada ("grep fields/arms já implementados
  antes de assumir trabalho original").
