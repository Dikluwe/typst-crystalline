# Diagnóstico Fase A P273.6.A — Parent bbox real save/restore (fecho 3γ.2)

**Data**: 2026-05-17.
**Passo**: typst-passo-273.6.A.
**Magnitude**: S documental (~30 min).
**Cluster**: Visualize / Gradient (encerra refino estrutural definitivamente).
**Tipo**: Fase A empírica per ADR-0034 + ADR-0085.
**Décimo sétimo consumo directo de fonte** (cristalino post-P273.5 +
DEBT-37 P84.6 padrão reused; P156C cascade pattern reused).

---

## §A.1 — Inventário do arm `Content::Block` no Layouter

`01_core/src/rules/layout/mod.rs`:

- **Linha 1484** — arm `Content::Block { body, width, height, inset,
  breakable, outset, radius, clip, fill, stroke, spacing, above, below,
  sticky: _ }` inicio.
- **Linha 1629** — primeiro callsite `self.layout_content(body)` (path
  com `clip=true`).
- **Linha 1655** — segundo callsite (path `clip=false`).
- **Linha 1720** — emit `FrameItem::Shape` final com bounds outer
  computados (pos, width, height, fill, stroke).

**Largura efectiva do Block**: `width: Option<Length>` em §1683:
- `Some(w) → w.resolve_pt(font) + inset_left` (literal).
- `None → saved_width - saved_line_start.0` (fallback page-width).

**Altura efectiva do Block**: computada pós-layout body como
`cursor_y - start_y` (linha 1688) — não disponível pré-layout body
sem refactor maior.

---

## §A.2 — Inventário do arm `Content::Boxed` no Layouter

`01_core/src/rules/layout/mod.rs:1330`:

- `Content::Boxed { body, width, height, inset, baseline, outset,
  radius, clip, fill, stroke }` inicio.
- Boxed é inline (sem flush_line antes/depois).
- Bbox computado em runtime via line_height + width resolvido.

**Decisão escopo P273.6**: **Boxed difere para sub-passo futuro
P273.7** (se necessário) — Boxed inline tem semântica bbox mais
sub-tile (baseline-relative); Block é o caso usecase mais comum
e suficiente para fechar pendência principal.

---

## §A.3 — Decisão semântica do bbox: pré-layout vs pós-layout

**Decisão fixada**: **3γ.2.γ — pré-layout com fields conhecidos**.

**Aplicação prática P273.6**:
- `Content::Block` arm save/restore `parent_bbox`:
  - **Save** `let saved_parent_bbox = self.parent_bbox;` no início do arm.
  - **Set** `self.parent_bbox = Some(Rect { ... })` **apenas quando
    `width.is_some() && height.is_some()`** (Decisão 3γ.2.γ).
  - **Restore** `self.parent_bbox = saved_parent_bbox` antes do
    emit Shape (após `layout_content(body)`).

**Justificativa Decisão 3γ.2.γ**:
1. Sem layout duplo (3γ.2.β rejeitado).
2. Sem risco de bbox errada (3γ.2.α rejeitado — width=None case
   problemático).
3. Semântica observable clara: utilizador especifica `width`+`height`
   do Block para obter `relative=parent` rigoroso.
4. Cumulativo: refino futuro pode adicionar medição pós-layout se
   3γ.2.γ for empiricamente insuficiente.

---

## §A.4 — Inventário do callsite L3 P273.5

`03_infra/src/export.rs`:

- **Linhas ~1644-1665** — Linear arm RGB-family `if relative ==
  Parent` constrói `page_bbox = Some((0,0,page_w,page_h))` +
  `apply_parent_transform(local, page_bbox)`.
- **Linhas ~1696-1717** — Radial arm idêntico.

**Page_bbox construído INLINE no L3** — não consulta nenhum field
do `FrameItem::Shape` ou estado do Layouter.

**P273.6 muda**: `FrameItem::Shape` ganha campo
`parent_bbox_at_emit: Option<Rect>` (decisão Prop-A); callsite L3
lê esse campo via `gradient_objects` Vec quando construído. Mas:

**Análise empírica**: `gradient_objects` no L3 é construído a partir
de `FrameItem`s coletados do documento; o `FrameItem::Shape` que
contém o gradient (via `fill`/`stroke`) carrega o bbox. Para chegar
ao callsite L3, o L3 dispatcher recebe `Vec<GradientObject>` — não
o `FrameItem::Shape` directo.

**Refactor mínimo Prop-A revisitado**: Estender `GradientObject` (struct
L3) com campo `parent_bbox_at_emit: Option<Rect>`; preencher quando
construído a partir do `FrameItem::Shape`; consumir no dispatcher.

---

## §A.5 — Análise de risco

| Risco | Fonte | Mitigação |
|---|---|---|
| Regressão tests P262-P274 | Mudança em `FrameItem::Shape` cascade | Cascade pattern-match (~86 sites sem `..`); campo `Option<Rect>` default `None` preserva semantic; bulk-patch via script |
| Regressão tests P273.5 | Callsite L3 muda fonte do bbox | Quando `shape.parent_bbox_at_emit` é `None`, fallback ao `page_bbox` directo P273.5 preservado literal |
| Bbox semanticamente errada | Decisão 3γ.2.γ | Só popula quando `width+height` literais — caso ambíguo cai no fallback page |
| `#[allow(dead_code)]` permanece | Activação parcial | §A.6 critério: `parent_bbox` Layouter consumed por emit shape (write) + arm Block (write); save/restore real |
| Pureza física L1 quebrada | Save/restore arm Block toca Rect (já L1) | Rect é tipo dados L1; save/restore é cursor.x.0 etc. — gestão RAM ADR-0029 §"Pureza física L1 — performance de RAM permitida" ✓ |
| Cascade FrameItem::Shape errado | 86 sites manuais | Bulk-patch Python regex; verificar build cada step |

---

## §A.6 — Critério de fecho `#[allow(dead_code)]`

`parent_bbox: Option<Rect>` no Layouter perde `#[allow(dead_code)]`
quando:

1. ✓ **Arm `Content::Block`** faz save/restore real (write quando
   width+height literais).
2. ✓ **Layouter, no momento de emitir `FrameItem::Shape`**, popula
   `parent_bbox_at_emit` a partir de `self.parent_bbox` (3 sítios
   emit Shape no Block arm: clip=true path + clip=false path).
3. ✓ **Callsite L3 P273.5 consulta `shape.parent_bbox_at_emit`** via
   `GradientObject` campo extendido; usa quando `Some(...)`; fallback
   `page_bbox` directo quando `None`.
4. ✓ **`cargo build` zero warning** de dead code no campo Layouter.

---

## §A.7 — Decisões fixadas

1. **Decisão 1 (semântica bbox)**: **3γ.2.γ** — popular apenas quando
   `width.is_some() && height.is_some()`. Fallback fall-through ao
   parent_bbox outer (LIFO restore).
2. **Decisão 2 (propagação L1→L3)**: **Prop-A revisitada** —
   `FrameItem::Shape` ganha `parent_bbox_at_emit: Option<Rect>`;
   `GradientObject` (L3) propaga; callsite consulta.
3. **Decisão 3 (lista contentores fase actual)**: **{Block} apenas**
   — Boxed difere para P273.7 (se necessário). Stack/Pad/Group/Grid
   cell ficam scope-out per ADR-0054 graded.

---

## §A.8 — Critério aceitação Fase A

- ✓ §A.1 cita arm Block literal (path `01_core/src/rules/layout/mod.rs:1484`).
- ✓ §A.2 cita arm Boxed literal — escopo difere P273.7.
- ✓ §A.4 cita callsite L3 literal pós-P273.5 (`03_infra/src/export.rs:1644+`).
- ✓ §A.5 risco "regressão P273.5" mitigado com fallback explícito
  `None → page_bbox`.
- ✓ §A.7 decisões 1/2/3 fixadas com fundamento empírico.

**Fase A completa**. Pronta para P273.6.B (anotação cumulativa ADR-0091)
e P273.6.C (materialização L1+L3 + cascade Shape).

---

## §A.9 — Sub-padrões aplicados P273.6.A

- **"Pattern DEBT-37 `cell_origin_*` replicado"** N=2 → **N=3
  cumulativo (atinge limiar formalização N=3-4)** — P273.6 é a
  terceira aplicação real (P84.6 + P273.5 estrutural + P273.6 com
  consumer real).
- **"Reutilização literal helpers cross-passos"** N=14 → **N=15
  cumulativo** (`apply_parent_transform` reused literal P273; padrão
  DEBT-37 reused estructuralmente).
- **"Diagnóstico imutável"** N=21 → **N=22 cumulativo** (décimo sétimo
  consumo directo de fonte).
- **"Auto-aplicação ADR-0065 inline"** N=20 → **N=21 cumulativo**
  (Fase A inline em diagnóstico).
- **"Aplicação meta-ADR (ADR-0094)"** N=4 → **N=5 cumulativo** —
  quinta aplicação prática.
- **"Aplicação meta-ADR (ADR-0093)"** N=3 → **N=4 cumulativo** —
  quarta aplicação prática Pattern 2 anotação cumulativa.
- **"Sub-passos decimais consecutivos do mesmo cluster"** N=1 →
  **N=2 cumulativo emergente** (P273.5 + P273.6).
- **"Cascade pattern-match cross-FrameItem"** N=1 → **N=2 cumulativo**
  (P156C N=12 sites; P273.6 N=86 sites — maior cascade até hoje no
  cluster Visualize).

---

*Diagnóstico imutável produzido em 2026-05-17 P273.6.A. Linhagem
empírica preservada como evidência ADR-0085 + auto-aplicação
ADR-0065. Décimo sétimo consumo directo de fonte (cristalino +
DEBT-37 P84.6 reused + P156C cascade pattern reused estructuralmente).*
