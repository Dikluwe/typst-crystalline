# Diagnóstico Fase A P273.12.A — Dedup bbox-aware (refino arquitectural pós-P273.10)

**Data**: 2026-05-18.
**Passo**: typst-passo-273.12.A.
**Magnitude**: S documental (~30 min).
**Cluster**: Visualize / Gradient (terceiro de 6 sub-passos para fechar cluster).
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085.
**Vigésimo terceiro consumo directo de fonte** (cluster Gradient
refino arquitectural; sub-padrão "Dedup Arc::as_ptr resources" N=2
→ N=3 cumulativo crossing limiar formalização N=3-4).

---

## §A.1 — Inventário do dedup actual em `scan_all_gradients` + callers

`03_infra/src/export.rs`:

- **Linha 357** — `fn scan_all_gradients(doc, first_id) -> (Vec<PatternRef>, HashMap<usize, usize>, Vec<GradientObject>)`.
- **Linha 371-447** — pós-P273.10 helper recursivo `walk(items, parent_bbox_override, ptr_to_idx, refs, grad_objs, next_id, counter)`.
- **Linha 400** — chave de dedup actual: `ptr_to_idx.contains_key(&ptr)` onde `ptr = Arc::as_ptr(g) as usize`.
- **Linha 417** — insert: `ptr_to_idx.insert(ptr, idx);`.

**Limitação identificada P273.6 §9 quarto bullet** (preserved em
todos os relatórios subsequentes até P273.11):
- Mesmo `Arc<Gradient>` em N callsites com `parent_bbox_effective`
  diferentes → apenas a primeira ocorrência captura o bbox; restantes
  renderizam com bbox da primeira.

### Callers consumidores de `pat_ptr_to_idx`

3 sítios chamam `emit_stroke_paint(...)` que faz lookup
`pat_ptr_to_idx.get(&ptr)`:
- **Linha 2078** — `emit_stroke_paint_type1` em `build_page_stream_type1`
  (Shape arm page-level Y-inversion).
- **Linha 2588** — `emit_stroke_paint` em `build_page_stream_cidfont`
  (Shape arm).
- **Linha 2772** — `emit_stroke_paint` em `build_page_stream_multifont`
  (Shape arm).

Todos 3 sítios são **page-level top-level Shapes** (não dentro de
Groups via draw_item_local — esse path usa solid fallback per
`s.paint.to_color()` linha 2339). Logo, para `effective_bbox` no
emit, basta destructure `parent_bbox_at_emit` directamente (sem
`parent_bbox_override` cascade).

### `pattern_resources_for_page` (pós-P273.10 walk)

- **Linha 454** — função; helper `walk(items, ptr_to_idx, refs,
  entries, seen)` linha 469.
- **Linha 484** — lookup `ptr_to_idx.get(&ptr)`.

Pós-P273.12 chave muda → walk precisa de `parent_bbox_override`
threading (mesma lógica de scan_all_gradients walk).

---

## §A.2 — Inventário `GradientObject`

`03_infra/src/export.rs:330-346`:

```rust
struct GradientObject {
    kind:                GradientObjectKind,
    function_id:         usize,
    shading_id:          usize,
    pattern_id:          usize,
    parent_bbox_at_emit: Option<Rect>,  // P273.6
}
```

Inserção em `scan_all_gradients`:
- Pre-P273.10: `parent_bbox_at_emit: *parent_bbox_at_emit` directo do
  FrameItem::Shape.
- Pos-P273.10 Inner-wins: `parent_bbox_at_emit: parent_bbox_at_emit.or(parent_bbox_override)`.

Consumer no dispatcher (linhas 1638-1669):
- `effective_parent_bbox` resolvido por `Some(rect) | None → page_w/h`
  fallback.

Para P273.12, `GradientObject` é criado **N vezes** (N PDF patterns)
quando mesmo Arc tem N bboxes effective distintas. Cada um terá o
seu `parent_bbox_at_emit` específico → dispatcher renderiza
correctamente cada um.

---

## §A.3 — Decisão 1 fixada: forma da chave de dedup

**Fixada**: **1β + 1γ combinados**.

```rust
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct RectKey(i32, i32, i32, i32);  // (x, y, w, h) em milipontos quantizados

fn rect_to_key(r: typst_core::entities::layout_types::Rect) -> RectKey {
    RectKey(
        (r.x.0 * 1000.0).round() as i32,
        (r.y.0 * 1000.0).round() as i32,
        (r.w.0 * 1000.0).round() as i32,
        (r.h.0 * 1000.0).round() as i32,
    )
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct DedupKey {
    arc_ptr: usize,
    bbox:    Option<RectKey>,
}
```

Razões:
1. `f64` em `HashMap` key é problemático (NaN, precision creep) —
   quantização milipontos resolve.
2. 1 mpt = 0.001 pt — precisão sub-typográfica preserved (typografia
   trabalha em pt; sub-pt raramente significativo).
3. Struct nomeado auto-documenta intent + facilita testes unitários
   (1 test `rect_to_key_quantization`).

---

## §A.4 — Decisão 2 fixada: scan-side

**Fixada**: **2β — scan-side**.

`scan_all_gradients.walk` computa `effective_bbox =
parent_bbox_at_emit.or(parent_bbox_override)` e usa-o como dedup key.
`pattern_resources_for_page.walk` faz o mesmo (consistência).
`emit_stroke_paint` recebe `effective_bbox: Option<Rect>` como param
adicional e constrói DedupKey para lookup.

Razões:
1. Dedup precisa decidir antes de criar PDF pattern objects.
2. `effective_bbox` é determinístico de `(parent_bbox_at_emit,
   parent_bbox_override)`.
3. Refactor mínimo — apenas a função dedup decide.

---

## §A.5 — Decisão 3 fixada: cross-page (3α — global ao documento)

`pat_ptr_to_idx: HashMap<DedupKey, usize>` global ao documento
(actual). Pattern PDF reusado entre pages via `/Pattern << >>` page
resource dict.

Mudança P273.12: chave passa a `DedupKey` com `bbox` inline; não
altera escopo.

---

## §A.6 — Cascade detalhado para emit_stroke_paint

`emit_stroke_paint(ops, paint, thickness, pat_ptr_to_idx, pat_refs)` →
muda para `emit_stroke_paint(ops, paint, thickness, effective_bbox,
pat_ptr_to_idx, pat_refs)`.

3 sítios calleros (linhas 2078, 2588, 2772) — todos page-level
top-level Shape destructure. Currently destructure
`parent_bbox_at_emit: _` (P273.7.1 cleanup) — mudar para
`parent_bbox_at_emit` e passar como `effective_bbox`.

**No override cascade necessário** para emit_stroke_paint sites —
todos 3 são page-level. `draw_item_local` Group recursion usa solid
fallback (`s.paint.to_color()`) — não chama emit_stroke_paint.

---

## §A.7 — Análise de risco

| Risco | Estado |
|---|---|
| Regressão tests P262-P273.11 | Mitigado — bbox=None preserved literal; Arc-único preserved |
| PDF size explosion | Aceitável — dedup ainda funciona quando bbox=igual; só duplica quando bboxes diferem (semântica correcta) |
| Float in HashMap key | Mitigado — Decisão 1γ milipontos quantizados |
| Refactor scan_all_gradients quebra Group recursion P273.10 | Decisão 2β preserva signature walk; só altera lookup key |
| Bug latent análogo a P273.9/P273.10 | Inventário §A.1 confirma 0 bugs adicionais |

---

## §A.8 — Critério de aceitação Fase A

- ✓ §A.1 cita `scan_all_gradients` + `pat_ptr_to_idx` literal
  (linhas 357/371-447/400/417) + 3 emit_stroke_paint callsites.
- ✓ §A.2 confirma `GradientObject` fields actuais (5 fields pós-P273.6).
- ✓ §A.3 Decisão 1 fixada: **1β + 1γ** (RectKey i32×4 milipontos +
  DedupKey nomeado).
- ✓ §A.4 Decisão 2 fixada: **2β scan-side**.
- ✓ §A.5 Decisão 3 fixada: **3α global ao documento**.
- ✓ §A.6 confirma cascade emit_stroke_paint mínimo (3 sites
  page-level; sem override threading para draw_item_local).

**Fase A produzida — critério §A.8 cumprido absoluto.**

---

## §A.9 — Plano de implementação (Fase C)

### Cap LOC (ADR-0094 Pattern 1)

- **L3 hard cap**: ≤ 100 LOC.
- **L3 soft cap**: ≤ 70 LOC.
- **L1 hard cap**: 0 LOC.
- **Tests hard cap**: ≤ 12.
- **Tests soft cap**: ≤ 8.

### Estimativa LOC

| Site | LOC esperado |
|---|---|
| `RectKey` + `DedupKey` structs + `rect_to_key` | ~20 |
| `scan_all_gradients.walk` refactor (HashMap key) | ~10 |
| `pattern_resources_for_page.walk` refactor (override threading + DedupKey lookup) | ~20 |
| `emit_stroke_paint` signature + DedupKey lookup | ~10 |
| `emit_stroke_paint_type1` wrapper | ~2 |
| 3 emit callsites (destructure + pass effective_bbox) | ~6 |
| Type aliases / signatures | ~5 |
| **Total L3** | **~73** |

Próximo do cap soft 70 — estouro estimado ~5%.

### Ordem literal

1. Fase A (este documento).
2. ADR-0091 décima segunda anotação cumulativa.
3. L0 `entities/gradient.md` anotação P273.12.
4. `crystalline-lint --fix-hashes`.
5. Tests-first (~6-7 testes; cap soft 8 respeitado).
6. Implementação L3 (~73 LOC).
7. Verificação final.

### Sub-padrões esperados

- **"Dedup Arc::as_ptr resources"** N=2 → **N=3 cumulativo crossing
  limiar formalização N=3-4** (P73 image + P263 pattern + P273.12
  pattern bbox-aware).
- **"Bug arquitectural intencional corrigido"** N=0 → **N=1 inaugural
  emergente** — limitação documentada P273.6 §9 corrigida 6 sub-passos
  depois com refino arquitectural deliberado.
- **"Sub-passos consecutivos do mesmo cluster"** N=7 → **N=8 cumulativo
  emergente**.

---

*Diagnóstico imutável produzido em 2026-05-18. Vigésimo terceiro
consumo. Decisões 1β+1γ + 2β + 3α fixadas; pronto para Fase C
(~73 LOC L3; ~6-7 testes; dedup bbox-aware via DedupKey).*
