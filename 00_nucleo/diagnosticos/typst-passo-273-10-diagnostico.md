# Diagnóstico Fase A P273.10.A — Group L3-only parent_bbox (sub-padrão "L3-only" inaugural)

**Data**: 2026-05-18.
**Passo**: typst-passo-273.10.A.
**Magnitude**: S documental (~25 min).
**Cluster**: Visualize / Gradient (primeiro de até 6 sub-passos para fechar cluster).
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085.
**Vigésimo primeiro consumo directo de fonte** (cristalino post-P273.9;
sub-padrão "L3-only parent_bbox" N=1 inaugural emergente).

---

## §A.1 — Inventário do dispatcher Group em export.rs

`03_infra/src/export.rs`:

- **Linha 357** — `fn scan_all_gradients(doc, first_id) -> (...)`.
  Iteração actual: **apenas top-level `page.items`** (linha 372-373):
  ```rust
  for page in &doc.pages {
      for item in &page.items {
          if let FrameItem::Shape { ... } = item { /* register */ }
      }
  }
  ```
  **NÃO recurse em Groups** — bug latent pré-existente: gradients
  dentro de `FrameItem::Group` actualmente não são registados em
  `ptr_to_idx`, logo o emit PDF não tem pattern resources para eles.

- **Linha 418** — `fn pattern_resources_for_page(page, ptr_to_idx, refs)`.
  Análogo: iteração `page.items` linear sem recursão em Groups
  (linhas 430-445). Bug latent análogo.

- **Linhas 2114, 2606, 2790** — 3 sítios de dispatch `FrameItem::Group`
  no PDF emit (`draw_item` page-level + 2 variantes `draw_item_local`).
  Cada um destructura `{ pos, matrix, clip_mask, inner_width,
  inner_height, items }` e recurse em `items` via `draw_item_local`
  (linhas 2138-2140 e análogos). **A recursão de emit já existe** —
  apenas `scan_all_gradients` + `pattern_resources_for_page` não
  recurse.

- **Linhas 1638-1643** — Dispatcher consumidor do `parent_bbox_at_emit`:
  ```rust
  let GradientObject { kind, function_id, shading_id, pattern_id,
                       parent_bbox_at_emit } = go;
  let effective_parent_bbox: (f32, f32, f32, f32) =
      if let Some(rect) = parent_bbox_at_emit {
          (rect.x.0 as f32, ...)
      } else {
          (0.0, 0.0, page_w as f32, page_h as f32)  // page fallback
      };
  ```
  Consumer já sabe usar `parent_bbox_at_emit` populated — só precisa
  que `scan_all_gradients` populate-o correctamente quando Shape está
  dentro de Group.

---

## §A.2 — Inventário do tipo `FrameItem::Group` em L1

`01_core/src/entities/layout_types.rs:253-260`:

```rust
Group {
    pos:          Point,
    matrix:       TransformMatrix,
    clip_mask:    Option<ShapeKind>,
    inner_width:  f64,
    inner_height: f64,
    items:        Vec<FrameItem>,
}
```

**Confirmação Decisão 2 spec**: tamanho do frame é `inner_width: f64
+ inner_height: f64` (NÃO `Size`/`Point`). Bbox construção
literal:

```rust
Rect {
    x: Pt(pos.x.0),
    y: Pt(pos.y.0),
    w: Pt(inner_width),
    h: Pt(inner_height),
}
```

Coords cristalino (sem Y-inversion — paridade Decisão 2α spec
recomendação).

---

## §A.3 — Bug latent verificação (P273.9 §2.4 analógico)

P273.9 §2.4 corrigiu `translate_frame_item` em `helpers.rs:48` que
descartava `parent_bbox_at_emit`. Verificação P273.10:

Sítios que reconstruem `FrameItem::Shape` com `parent_bbox_at_emit`:
- `helpers.rs:48` — **JÁ CORRIGIDO P273.9** (preserva campo).
- `slicing.rs:98-102` — destructure + reconstruct; **confirmar preserva**.
- `math/layout/mod.rs:112-120` — destructure + reconstruct; **confirmar**.
- `cursor.rs:240-244` — destructure + reconstruct; **confirmar**.

Análise empírica via `grep -A 3 "FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit"`:

Lemos os 4 sítios em P273.6 cascade — todos preservam o campo no
reconstruct (ex. `parent_bbox_at_emit,` directamente). **Bug
análogo a P273.9 §2.4 confirmado zero**.

P273.10 não precisa fix tactical adicional. Decisão Fase A: **0 bugs
análogos detectados**; nenhum cascade adicional.

---

## §A.4 — Decisão 1 fixada: mecanismo de override L3

**Fixada**: **Opção 1α — Parameter threading explícito**.

Implementação literal:

```rust
fn scan_all_gradients(doc, first_id) -> (...) {
    // Helper recursivo interno
    fn walk(
        items: &[FrameItem],
        parent_bbox_override: Option<Rect>,
        ptr_to_idx: &mut HashMap<usize, usize>,
        refs:       &mut Vec<PatternRef>,
        grad_objs:  &mut Vec<GradientObject>,
        next_id:    &mut usize,
        counter:    &mut usize,
    ) {
        for item in items {
            match item {
                FrameItem::Shape {
                    stroke: Some(Stroke { paint: Paint::Gradient(g), .. }),
                    parent_bbox_at_emit, ..
                } => {
                    // P273.10 — Inner wins (Decisão 3α).
                    let effective_bbox = parent_bbox_at_emit
                        .or(parent_bbox_override);
                    // ... register (refs, ptr_to_idx, grad_objs) ...
                    grad_objs.push(GradientObject {
                        kind, function_id, shading_id, pattern_id,
                        parent_bbox_at_emit: effective_bbox,
                    });
                }
                FrameItem::Group { pos, inner_width, inner_height, items, .. } => {
                    // P273.10 — Group bbox override.
                    let group_bbox = Rect {
                        x: Pt(pos.x.0),
                        y: Pt(pos.y.0),
                        w: Pt(*inner_width),
                        h: Pt(*inner_height),
                    };
                    walk(items, Some(group_bbox),
                         ptr_to_idx, refs, grad_objs, next_id, counter);
                }
                _ => {}
            }
        }
    }
    // ... init state + iterar pages chamando walk(items, None, ...) ...
}
```

Razões:
1. Sem mutação de FrameItem — preserva `&Frame` imutável.
2. Compose-na-fly via LIFO no parameter — Group dentro de Group
   propaga override correctamente (innermost wins automaticamente).
3. Sub-padrão "L3-only parent_bbox" via signature explícita —
   auto-documentado.

---

## §A.5 — Decisão 2 fixada: semântica Group bbox

**Fixada**: **Opção 2α — bbox exacto frame em coords cristalino**.

`Rect { x: pos.x, y: pos.y, w: inner_width, h: inner_height }` —
geometric exact bbox, **sem Y-inversion**. Coords cristalino
(Y-down) — paridade com `parent_bbox_at_emit` do Layouter
(P273.6/7/9) que também são coords cristalino. Y-inversion é
responsabilidade exclusiva do PDF emit final (`apply_parent_transform`
no dispatcher consumer).

---

## §A.6 — Decisão 3 fixada: override precedence

**Fixada**: **3α (≡ 3γ) — Inner wins**.

Implementação: `effective_bbox = parent_bbox_at_emit.or(parent_bbox_override)`.

- Shapes com `parent_bbox_at_emit: Some(rect)` (P273.9 5 containers
  Layouter) → mantêm o próprio campo; override Group ignorado.
- Shapes com `parent_bbox_at_emit: None` (top-level OR fora dos 5
  containers) **+ dentro de Group** → recebem `group_bbox` via
  override.
- Shapes top-level (não dentro de Group) → preserved literal P273.9
  (`override = None` propagado; fallback page_bbox L3 dispatcher).

Paridade vanilla: "relative=parent" resolve ao contentor mais
próximo; Layouter L1 conhece os contentores canónicos
(Block/Boxed/Grid/Stack/Pad); Group é wrapper estrutural pós-layout
que NÃO redefine "parent" semanticamente — apenas oferece bbox de
fallback quando Layouter não populou.

---

## §A.7 — `pattern_resources_for_page` cascade analógica

Sítio adicional NÃO mencionado na spec mas necessário para
correctness:

`pattern_resources_for_page` (linha 418) também itera apenas
`page.items` top-level. Se Group contém Shape com gradient,
actualmente:
1. `scan_all_gradients` não recurse → gradient não registado.
2. P273.10 corrige (1).
3. **Mas** `pattern_resources_for_page` continua a não recurse →
   gradient registado mas não listado em page resources `/Pattern << >>`
   → PDF emit ainda quebrado.

**P273.10 deve também adicionar recursão a `pattern_resources_for_page`**
(scope creep necessário para correctness — sem ele, P273.10 não
produz observable behavior).

Cap LOC: ~10-15 LOC adicionais para refactor `pattern_resources_for_page`
com helper recursivo simétrico ao do `scan_all_gradients`.

---

## §A.8 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Regressão tests P262-P273.9 | Recursão em scan_all_gradients pode produzir gradients duplicated registrados | `ptr_to_idx.contains_key(&ptr)` linha 394 já dedup — preserved |
| Regressão pattern_resources_for_page | Refactor com helper recursivo | Symmetric helper análogo; preserve dedup via BTreeSet existente |
| Param signature noise | scan_all_gradients ganha 5 params adicionais via recursão | Encapsulamento em helper interno (closure ou nested fn); signature pública preserved |
| Cap LOC L3 estourado | Refactor + recursão + pattern_resources_for_page | Cap hard 60 / soft 40 — estimativa real ~40-55 LOC (perto do soft; pode estourar) |
| Inner-wins quebra dedup bbox-aware | Mesma Arc gradient em contextos diferentes | Limitação preserved literal (dedup primeiro wins per P273.6 §9) |

---

## §A.9 — Critério de aceitação Fase A

- ✓ §A.1 cita dispatcher Group literal (3 sítios linha 2114/2606/2790) +
  `scan_all_gradients` (357) + `pattern_resources_for_page` (418).
- ✓ §A.2 confirma `FrameItem::Group` usa `inner_width: f64 +
  inner_height: f64` (não `Size`).
- ✓ §A.3 confirmação empírica: zero bugs análogos a P273.9 §2.4
  detectados.
- ✓ §A.4 Decisão 1 fixada: **1α — parameter threading explícito**.
- ✓ §A.5 Decisão 2 fixada: **2α — bbox exacto frame coords cristalino**.
- ✓ §A.6 Decisão 3 fixada: **3α — Inner wins** via `.or(override)`.
- ✓ §A.7 cascade analógico `pattern_resources_for_page` identificado
  (scope creep necessário).

**Fase A produzida — critério §A.9 cumprido absoluto.**

---

## §A.10 — Plano de implementação (Fase C)

### Cap LOC (ADR-0094 Pattern 1; caps recalibrados pela §A.7 scope creep)

- **L3 hard cap**: ≤ 60 LOC (preserved spec).
- **L3 soft cap**: ≤ 50 LOC (recalibrado +10 por §A.7 cascade
  analógica `pattern_resources_for_page`).
- **L1 hard cap**: 0 LOC (sem touch Layouter).
- **Tests hard cap**: ≤ 12.
- **Tests soft cap**: ≤ 8.

### Estimativa LOC

| Sítio | LOC esperado | Mecanismo |
|---|---|---|
| `scan_all_gradients` refactor | ~30 | Inner helper recursivo + match Group arm |
| `pattern_resources_for_page` refactor | ~15 | Symmetric helper recursivo |
| **Total L3** | **~45** | Próximo do cap soft 50 |

### Ordem literal

1. Fase A (este documento).
2. ADR-0091 décima primeira anotação cumulativa.
3. L0 `entities/gradient.md` anotação P273.10.
4. `crystalline-lint --fix-hashes`.
5. Tests-first (~6-8 testes — cap soft 8 respeitado).
6. Implementação:
   - 6a. Refactor `scan_all_gradients` com helper recursivo +
        param `parent_bbox_override`.
   - 6b. Refactor `pattern_resources_for_page` symmetric recursão.
7. Verificação final.

### Sub-padrões esperados

- **"L3-only parent_bbox"** N=0 → **N=1 inaugural emergente**.
- **"Sub-passos consecutivos do mesmo cluster"** N=5 → **N=6
  cumulativo emergente**.
- **"Bug latent corrigido + scope creep"** preserved 0 — P273.10
  detecta scope creep arquitectural (§A.7) mas zero bug análogo
  P273.9 §2.4.

---

*Diagnóstico imutável produzido em 2026-05-18. Vigésimo primeiro
consumo directo de fonte. Decisões 1α + 2α + 3α fixadas + scope
creep §A.7 documentado; pronto para Fase C (~45 LOC L3; ~6-8 testes;
sub-padrão "L3-only parent_bbox" N=1 inaugural emergente).*
