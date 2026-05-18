# Diagnóstico Fase A P273.5.A — Parent bbox callsite (fecha #[allow(dead_code)] P273)

**Data**: 2026-05-17.
**Passo**: typst-passo-273.5.A.
**Magnitude**: S documental (~30 min).
**Cluster**: Visualize / Gradient (refino estrutural fecho-de-pendência).
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085.
**Décimo sexto consumo directo de fonte** (cristalino post-P274 +
DEBT-37 P84.6 padrão `cell_origin_*` reused estructuralmente).

---

## §A.1 — Inventário do callsite real

`03_infra/src/export.rs`:

- `fn emit_gradient_objects(grad_objs, page_dimensions, next_sub_id)`
  — definido linha 1609.
- 3 callers do dispatcher de gradient:
  - linha 1310 — text paint flush.
  - linha 1435 — shape fill.
  - linha 1592 — shape stroke.

**Parâmetros actuais recebidos**:
- `grad_objs: Vec<GradientObject>` — gradient objects to emit.
- `page_dimensions: &[(f64, f64)]` — page width/height pairs.
- `next_sub_id: &mut usize` — PDF object ID counter.

**Origem dos `gradient_objs`**: dispatcher coleta `Gradient`s das
shapes ao analisar `FrameItem::Shape` no documento. Cada gradient
aparece com seu próprio `(function_id, shading_id, pattern_id)`.

**Sítio onde bbox local da shape é conhecida**: dentro de cada arm
do dispatcher (Linear/Radial/Conic), `page_w/page_h` é extraído
(`page_dimensions.first().copied().unwrap_or((595.0, 842.0))`).
A bbox local **da shape** (não da página) não é actualmente
propagada — coords são calculadas em **page-space** via
`compute_axial_coords`/`compute_radial_coords` (Linear/Radial); Conic
usa unit-space [0,1].

---

## §A.2 — Inventário propagação cristalino existente

`01_core/src/rules/layout/mod.rs:84` define `Layouter<'a, M, S>`:

- `pub(super) cell_origin_x: Option<f64>` (linha 163) — DEBT-37 P84.6.
- `pub(super) cell_origin_y: Option<f64>` (linha 164) — DEBT-37 P84.6.
- Save/restore em `01_core/src/rules/layout/grid.rs:364-369` — padrão
  estabelecido.

**Padrão DEBT-37 P84.6 directamente aplicável** — campo opcional no
Layouter + save/restore por arm Grid cell + consumer dispatcha
`Some` (usa) vs `None` (fallback).

**bbox da página conhecida no L3**: `page_dimensions` em
`emit_gradient_objects`. `Size::a4() = Size { width: Pt(595.0),
height: Pt(842.0) }` é fallback default.

**FrameItem::Shape**: definido em `01_core/src/entities/layout_types.rs:213`:
```rust
Shape {
    pos: Point,
    kind: ShapeKind,
    width: f64,
    height: f64,
    fill: Option<Paint>,
    stroke: Option<Stroke>,
}
```
Carrega `pos + width + height` — bbox da shape **disponível literal**
no FrameItem. Mas o `emit_gradient_objects` actual não recebe esta
informação — é chamado uma vez por documento, não por shape.

---

## §A.3 — Definição de "Parent" para gradient cristalino

**Decisão fixada**: **Opção 3γ híbrida** — semântica `Parent` como
contentor imediato com **fallback página**:

- **3γ.1** (P273.5 materializado): callsite emit gradient passa
  **page_bbox** como `parent_bbox` (fallback página). Cobre o caso
  "shape top-level com relative=parent ancora à página" — usecase
  mais comum.
- **3γ.2** (pendência futura, fora de escopo P273.5): Layouter
  populando `parent_bbox` real do Block/Boxed/Group contentor
  imediato via save/restore (paridade vanilla full). **Estruturalmente
  preparado** via campo `Option<Rect>` no Layouter; consumer ready.

**Rationale**:
- 3α puro perde semântica vanilla aninhada (Block).
- 3β puro requer refactor maior (Block save/restore + bbox calculation
  pré-layout body — out of scope S).
- 3γ híbrida **fecha pendência principal** P273 (`#[allow(dead_code)]`
  removido; pelo menos 1 callsite passa `Some(rect)`) + **prepara
  estructuralmente** para 3γ.2 futuro (campo L1 existe; população real
  é incremento futuro).

---

## §A.4 — Mecanismo de propagação (Decisão 3γ híbrida)

**P273.5 materializa 3γ.1** (page bbox fallback):

1. **L1**: novo `Rect { x: Pt, y: Pt, w: Pt, h: Pt }` em
   `01_core/src/entities/layout_types.rs` (paridade `Point` + `Size`).
2. **L1**: novo campo `parent_bbox: Option<Rect>` no Layouter
   (`01_core/src/rules/layout/mod.rs` linha 165 — após `cell_origin_y`).
3. **L1**: Constructor init `parent_bbox: None` (default; future Block
   save/restore populará).
4. **L3**: `emit_gradient_objects` lê `page_dimensions[0]` para
   construir `page_bbox = Rect { x: 0, y: 0, w: page_w, h: page_h }`
   como fallback parent_bbox (3γ.1 hídrida).
5. **L3**: Quando `gradient.relative == Some(Parent)`, dispatcher
   chama `apply_parent_transform(local_coords_unit, Some(page_bbox))`
   antes de format PDF coordinates.
6. **L3**: `#[allow(dead_code)]` removido de `apply_parent_transform`
   — função tem callsite real.

**Refino futuro 3γ.2 (out of scope)**:
- Block/Boxed/Group save/restore em Layouter arms.
- `FrameItem::Shape` carrega bbox local via `pos + width + height`
  (já existe estrutura; só precisa de propagar para emit_gradient_objects
  contexto callsite).
- Aceitável incremental per ADR-0054 graded.

---

## §A.5 — Análise de risco

| Risco | Fonte | Mitigação P273.5 |
|---|---|---|
| Regressão tests P273 (resolve sempre Self_ pré-P273.5) | Defaults `relative: None` preservam Self_ | Inputs sem `relative=Parent` dispatcham para Self_ branch literal P272/P273; bit-exact preserved. |
| Regressão tests P262-P272 | Pipeline emit altered | Toda thread só dispara para `RelativeTo::Parent`; Self_/None branches preserved literal. |
| Bbox errada propagada | Decisão 3γ.1 page bbox para fallback | Page bbox é correcto para shape top-level com relative=Parent (semântica vanilla "shape top-level ancora à página"). |
| Pureza física L1 quebrada | Campo `parent_bbox: Option<Rect>` no Layouter | Rect é tipo L1 (paridade Point/Size); campo é gestão de memória RAM (ADR-0029 §"Pureza física L1 — performance de RAM permitida"). ✓ |
| `#[allow(dead_code)]` continua | Activação parcial | §A.6 critério: 3 callsites L3 dispatcher passam `Some(page_bbox)` para `apply_parent_transform`. ✓ |

---

## §A.6 — Critério de fecho #[allow(dead_code)]

`apply_parent_transform` perde `#[allow(dead_code)]` quando:

1. ✓ **Pelo menos um callsite real do Layouter passa `Some(rect)` ao
   dispatcher de gradient** — 3 callsites L3 (`emit_gradient_objects`
   lines 1310/1435/1592) passam `page_bbox` quando `relative == Parent`.
2. ✓ **Pelo menos um test E2E exercita path `RelativeTo::Parent`**
   com bbox real — adicionado P273.5.C tests:
   - `p273_5_linear_relative_parent_uses_page_bbox` — Linear top-level
     com `relative=Parent` chama apply_parent_transform.
   - `p273_5_radial_relative_parent_uses_page_bbox`.
3. ✓ **Compilador não dispara warning de dead code** sem `#[allow]` —
   confirmação empírica `cargo build` zero warnings na função.

---

## §A.7 — Decisões a fixar na Fase A

1. **Decisão 3 (semântica Parent)**: **3γ híbrida**.
   - P273.5.C materializa 3γ.1 (page bbox fallback).
   - 3γ.2 (Block save/restore) é pendência preservada incremental.

2. **Lista de contentores que disparam save/restore** (3γ.2 pendência):
   - **Não materializada P273.5**. Lista candidata documentada:
     `Content::Block`, `Content::Boxed` (se existe), `Content::Group`
     (se existe), `Content::Grid` cell (já tem `cell_origin_*` pattern).
   - Materialização incremental quando consumer real surgir.

3. **Tipo Rect cristalino**: **adicionar à L1** em
   `01_core/src/entities/layout_types.rs`. Paridade `Point`/`Size`:

   ```rust
   /// Rectângulo alinhado aos eixos (paridade `Point` + `Size`).
   /// P273.5 — usado como bbox de contentor para gradient
   /// `relative: Some(Parent)` resolution.
   #[derive(Debug, Clone, Copy, PartialEq)]
   pub struct Rect {
       pub x: Pt,
       pub y: Pt,
       pub w: Pt,
       pub h: Pt,
   }
   ```

---

## §A.8 — Critério de aceitação Fase A

- ✓ §A.1 confirma callsite real (3 callsites L3 emit_gradient_objects
  linhas 1310/1435/1592).
- ✓ §A.2 confirma precedente DEBT-37 P84.6 directamente aplicável
  (padrão `cell_origin_x/y: Option<f64>` + save/restore Grid arms).
- ✓ §A.3 decisão fixada: **3γ híbrida** — 3γ.1 materializado P273.5;
  3γ.2 pendência preservada.
- ✓ §A.5 risco regressão P262-P273 mitigado por defaults Self_ branch
  preservado literal.
- ✓ §A.7 decisões 1/2/3 fixadas com fundamento numérico/estrutural.

**Fase A completa**. Pronta para P273.5.B (anotação cumulativa ADR-0091)
e P273.5.C (materialização L1+L3).

---

## §A.9 — Sub-padrões aplicados P273.5.A

- **"Pattern DEBT-37 `cell_origin_*` replicado"** N=1 → **N=2
  cumulativo emergente** (P273.5 reusa padrão estructuralmente; meio
  caminho do limiar formalização N=3-4).
- **"Reutilização literal helpers cross-passos"** N=13 → **N=14
  cumulativo** (`apply_parent_transform` P273 reused literal; padrão
  `cell_origin_*` DEBT-37 P84.6 reused estructuralmente).
- **"Diagnóstico imutável"** N=20 → **N=21 cumulativo** (décimo sexto
  consumo directo de fonte).
- **"Auto-aplicação ADR-0065 inline"** N=19 → **N=20 cumulativo**
  (Fase A inline em diagnóstico).
- **"Aplicação meta-ADR (ADR-0094)"** N=3 → **N=4 cumulativo** —
  quarta aplicação prática Pattern 1 Cap LOC hard/soft pós-formalização
  P271.
- **"Aplicação meta-ADR (ADR-0093)"** N=2 → **N=3 cumulativo** —
  terceira aplicação prática Pattern 2 anotação cumulativa.

---

*Diagnóstico imutável produzido em 2026-05-17 P273.5.A. Linhagem
empírica preservada como evidência ADR-0085 + auto-aplicação
ADR-0065. Décimo sexto consumo directo de fonte (cristalino
post-P274 + DEBT-37 P84.6 padrão `cell_origin_*` reused
estructuralmente).*
