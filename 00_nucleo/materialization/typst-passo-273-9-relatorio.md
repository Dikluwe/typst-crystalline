# Relatório P273.9 — Containers estendidos (Grid + Stack + Pad — escopo 1γ)

**Data**: 2026-05-18.
**Magnitude**: M (~58 LOC L1; 7 testes novos; cap soft L1 60 respeitado limite).
**Cluster**: Visualize / Gradient (encerra refino estrutural extensivo).
**Tipo**: sub-passo decimal P273.9 — extensão Decisão 3 de `{Block, Boxed}` para `{Block, Boxed, Grid cell, Stack, Pad}`.
**Spec**: `00_nucleo/materialization/typst-passo-273-9.md`.

---

## §1 — Sumário executivo

**Cluster Gradient refino estrutural extensivamente encerrado** via
P273.9 — escopo 1γ (Decisão utilizador, M magnitude reconhecida):

- **L1 arm `Content::Grid`** (`grid.rs`) ganha save/restore real do
  `parent_bbox` paralelo a `cell_origin_*` (DEBT-37 N=4 cumulativo).
- **L1 arm `Content::Stack`** (`mod.rs`) ganha measurement inline
  via replicação handler `measure_content_constrained` Stack arm +
  save/restore.
- **L1 arm `Content::Pad`** (`mod.rs`) ganha
  `measure_content_constrained` call + save/restore inner bbox.
- **L1 helper `translate_frame_item`** (`helpers.rs`): bug-fix
  cascade P273.6 — preserva `parent_bbox_at_emit` (era descartado
  para `None`, impedindo a propagação a Shapes filhas de cells/groups
  com translate).

### Marcos arquitecturais P273.9

**(1) Cluster Gradient refino estrutural extensivamente encerrado** —
Decisão 3 estende `{Block, Boxed}` → `{Block, Boxed, Grid cell, Stack, Pad}`.
Stack/Pad usam **layout duplo arquitectural aceite** (sub-padrão
emergente inaugural N=1).

**(2) Pattern DEBT-37 `cell_origin_*` replicado N=4 cumulativo** —
**crossing limiar formalização N=3-4 com folga consolidada**:
- N=1: P84.6 (DEBT-37 `cell_origin_x/y/w`).
- N=2: P273.5 (`parent_bbox` estrutural).
- N=3: P273.6 (`parent_bbox` save/restore Block + consumer real).
- **N=4: P273.9** Grid cell save/restore paralelo a `cell_origin_*`.

**(3) Sub-padrão emergente "Layout duplo arquitectural aceite" N=1
inaugural** — Stack/Pad estabelecem precedente para containers sem
dimensions literais usarem `measure_content_constrained` pre-layout
para construir `parent_bbox`. Custo perf ~1.5-2× **apenas em
pipelines com gradient `relative=parent`** (defaults Self_/None
preservam zero overhead).

**(4) Sub-padrão "Template-passo replicado literal" N=1 → N=2
cumulativo** — Grid replica template Block/Boxed literal; Stack/Pad
replicam com adaptação layout duplo.

**(5) Bug latent P273.6 corrigido** — `translate_frame_item` em
`helpers.rs` descartava `parent_bbox_at_emit` da Shape ao mover
posição. Bug não-observable pré-P273.9 porque Grid/Stack/Pad não
populavam `parent_bbox` (e Block/Boxed Shapes não passam por
translate). P273.9 expõe + corrige.

### Decisões fixadas Fase A

1. **Decisão 1 (escopo)**: **1γ** — Grid cell + Stack + Pad.
   Decisão utilizador; trade-offs M magnitude + risco regressão alto
   aceites.
2. **Decisão 2 (Grid bbox)**: **2α — bbox exacto cell** =
   `Rect { x: body_x, y: body_y, w: body_w, h: body_h }`.
3. **Decisão 3 (Stack + Pad bbox)**: medição inline via
   `measure_content_constrained` pre-layout. Stack — vertical
   `max_w × sum_h`; horizontal `sum_w × max_h`. Pad — bbox INNER
   (body region, sem insets) paralela a Block.

### Defaults preservam P262-P273.8 bit-exact

- Grid cell `body_w/h <= 0` (degenerate) → `parent_bbox` outer
  preservado.
- Stack vazio (n=0) ou measured 0×0 → preservado.
- Pad com body vazio → preservado.
- Self_/None relative ignora `parent_bbox_at_emit` (paridade).
- 2620 baseline P273.8 preserved (excluindo +7 testes novos P273.9).

---

## §2 — Diff L1 antes/depois

### §2.1 — L1 `grid.rs` arm Grid cell (~13 LOC)

```rust
// 01_core/src/rules/layout/grid.rs — dentro do loop de cells
// pós-cell_origin_* save existente

// P273.9 — save/restore parent_bbox paralelo a cell_origin_*
// (Decisão 2α: bbox exacto cell body). Defaults rigorosos:
// popular apenas se body_w/h > 0. Pattern DEBT-37 reused N=4.
let saved_parent_bbox_p273_9 = self.parent_bbox;
if body_w > 0.0 && body_h > 0.0 {
    self.parent_bbox = Some(crate::entities::layout_types::Rect {
        x: Pt(body_x),
        y: Pt(body_y),
        w: Pt(body_w),
        h: Pt(body_h),
    });
}

// ... layout_sub_frame_with_width(cell, body_x, body_w) existente ...

// P273.9 — restore parent_bbox (LIFO).
self.parent_bbox = saved_parent_bbox_p273_9;
```

### §2.2 — L1 `mod.rs` arm Stack (~30 LOC com replicação inline)

```rust
// 01_core/src/rules/layout/mod.rs — Content::Stack arm
// pós-flush_line + early return n==0

// P273.9 — measure stack bbox inline (paridade handler
// measure_content_constrained Stack arm). Layout duplo
// arquitectural aceite (sub-padrão N=1 inaugural).
let saved_parent_bbox_p273_9 = self.parent_bbox;
let stack_avail_w = self.available_width();
let (stack_w, stack_h) = if dir.is_vertical() {
    let mut max_w = 0.0_f64;
    let mut sum_h = 0.0_f64;
    for child in children.iter() {
        let (w, h) = self.measure_content_constrained(child, stack_avail_w);
        max_w = max_w.max(w);
        sum_h += h;
    }
    (max_w, sum_h + ((n - 1) as f64) * space_pt)
} else {
    let mut sum_w = 0.0_f64;
    let mut max_h = 0.0_f64;
    for child in children.iter() {
        let (w, h) = self.measure_content_constrained(child, stack_avail_w);
        sum_w += w;
        max_h = max_h.max(h);
    }
    (sum_w + ((n - 1) as f64) * space_pt, max_h)
};
if stack_w > 0.0 && stack_h > 0.0 {
    self.parent_bbox = Some(crate::entities::layout_types::Rect {
        x: self.regions.current.cursor_x,
        y: self.regions.current.cursor_y,
        w: Pt(stack_w),
        h: Pt(stack_h),
    });
}

// ... iteration loop existente ...

// P273.9 — restore parent_bbox (LIFO).
self.parent_bbox = saved_parent_bbox_p273_9;
```

### §2.3 — L1 `mod.rs` arm Pad (~13 LOC)

```rust
// 01_core/src/rules/layout/mod.rs — Content::Pad arm
// pós-cursor/width setup, antes layout_content(body)

// P273.9 — save/restore parent_bbox INNER (body region;
// paralela a Block semantic). Layout duplo via
// measure_content_constrained pre-layout.
let saved_parent_bbox_p273_9 = self.parent_bbox;
let pad_avail_inner = self.available_width();
let (pad_body_w, pad_body_h) =
    self.measure_content_constrained(body, pad_avail_inner);
if pad_body_w > 0.0 && pad_body_h > 0.0 {
    self.parent_bbox = Some(crate::entities::layout_types::Rect {
        x: self.regions.current.cursor_x,
        y: self.regions.current.cursor_y,
        w: Pt(pad_body_w),
        h: Pt(pad_body_h),
    });
}

self.layout_content(body);
self.flush_line();

// P273.9 — restore parent_bbox (LIFO).
self.parent_bbox = saved_parent_bbox_p273_9;
```

### §2.4 — L1 `helpers.rs` bug-fix `translate_frame_item` (~2 LOC)

```diff
- FrameItem::Shape { kind, width, height, fill, stroke, .. } =>
-     FrameItem::Shape { pos: Point { x: new_x, y: new_y }, kind, width, height, fill, stroke, parent_bbox_at_emit: None },
+ FrameItem::Shape { kind, width, height, fill, stroke, parent_bbox_at_emit, .. } =>
+     FrameItem::Shape { pos: Point { x: new_x, y: new_y }, kind, width, height, fill, stroke, parent_bbox_at_emit },
```

**Bug latent P273.6** — translate descartava `parent_bbox_at_emit`.
Não-observable pré-P273.9 porque Grid cell era único caller que
emitia Shapes através de translate, e Grid não populava `parent_bbox`
antes deste passo. P273.9 expõe + corrige sem cascade adicional.

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P273.9 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=16 → N=17 cumulativo consolidação clara persistente** | Décima anotação consecutiva ADR-0091 |
| Reutilização literal helpers cross-passos | **N=16 → N=17 cumulativo consolidação clara persistente** | Template save/restore + `measure_content_constrained` handlers reused |
| Cap LOC hard vs soft explícito | **N=11 → N=12 cumulativo** | L1 hard 80 / soft 60 — real ~58 (cap soft limite respeitado) |
| Aplicação meta-ADR (ADR-0093) | **N=5 → N=6 cumulativo** | Pattern 2 anotação cumulativa |
| Aplicação meta-ADR (ADR-0094) | **N=7 → N=8 cumulativo** | Pattern 1 cap LOC |
| **Pattern DEBT-37 `cell_origin_*` replicado** | **N=3 → N=4 cumulativo (crossing limiar com folga)** | P273.9 Grid cell paralelo a `cell_origin_*` consolida limiar formalização N=3-4 |
| **Template-passo replicado literal** | **N=1 → N=2 cumulativo** | Grid replica template Block/Boxed; Stack/Pad replicam com adaptação layout duplo |
| Sub-passos consecutivos do mesmo cluster | **N=4 → N=5 cumulativo emergente** | P273.5/6/7/8/9 |
| **Layout duplo arquitectural aceite** | **N=0 → N=1 inaugural emergente** | P273.9 inaugura: Stack/Pad usam `measure_content_constrained` pre-layout para `parent_bbox`. Custo perf apenas em pipelines com gradient relative=parent |
| Diagnóstico imutável | **N=24 → N=25 cumulativo** | Vigésimo consumo |

---

## §4 — Métricas finais

| Métrica | Pré-P273.9 | Pós-P273.9 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2618 | **2625** | +7 |
| Tests P273.9 novos | — | 7 | 7 layout integration tests |
| Tests P262-P273.8 (verdes) | preserved | preserved | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Build warnings (parent_bbox related) | 0 | 0 | 0 |
| Hashes propagados L0 | — | 1 (`gradient.rs:af1d57d2`) | +1 |
| ADRs totais | 81 | **81** | 0 (sem nova ADR; décima anotação ADR-0091) |
| LOC L1 (additions) | — | **~58** | grid.rs ~13 + mod.rs Stack ~30 + mod.rs Pad ~13 + helpers.rs ~2 |
| LOC L3 (additions) | — | 0 | literal (dispatcher inalterado desde P273.6) |

### §política condições verificadas

- ✓ Cap LOC L1 hard 80 — real ~58; folga 28%.
- ✓ Cap LOC L1 soft 60 — real ~58; limite respeitado (margem 3%).
- ✓ Cap LOC L3 hard 0 — real 0; literal.
- ✓ Cap testes hard 12 — real 7; folga 42%.
- ✓ Cap testes soft 8 — real 7; folga 13%.
- ✓ Defaults preservam pipeline P262-P273.8 bit-exact.
- ✓ Lint zero preserved.
- ✓ Regressão tests P262-P273.8 zero (baseline preserved).
- ✓ **Regressão DEBT-37 P246 cell_origin_* consumption** zero —
  Grid cell emite Shapes correctamente; tests Grid existentes
  preserved.
- ✓ ADR-0029 pureza física L1 preserved (Rect/parent_bbox/
  `measure_content_constrained` são tipos dados + medição pura).

**10 condições §política verificadas — 10 satisfeitas absolutas**
per ADR-0094 Pattern 1.

---

## §5 — Verificação regressão zero P262-P273.8

**2618 baseline preservado bit-exact**:

- typst-core: 2172 → **2179** (+7 P273.9 layout tests).
- typst-shell: 24 preserved.
- typst-infra: 399 preserved.
- typst-wiring + bins + outros: 23 preserved.

**Total: 2618 → 2625 (+7 net)**.

**2 testes typst-core skipped** (`recursao_profunda_retorna_err`,
`recursao_infinita_retorna_err_sem_crash`) — stack overflow
pré-existente; **não é regressão P273.9**.

Mecânica:
- Grid cell `body_w/h <= 0` → `parent_bbox` outer preservado.
- Stack vazio / measured 0×0 → preservado.
- Pad com body vazio → preservado.
- Self_/None relative branch literal preserved.
- `translate_frame_item` bug-fix preserva `parent_bbox_at_emit` —
  Grid cell tests P246 preserved bit-exact (era `None` antes; agora
  propaga o valor real do Layouter, que para tests pré-P273.9 era
  sempre `None`).

---

## §6 — Anotação cumulativa ADR-0091 (décima consecutiva)

Adicionada §"Anotação cumulativa P273.9 — Containers estendidos
(Grid + Stack + Pad — escopo 1γ)" cobrindo:

- Decisão 1 fixada: 1γ (Grid + Stack + Pad).
- Decisão 2 fixada: 2α (bbox exacto cell).
- Decisão 3 fixada: bbox medido inline / INNER (Stack + Pad).
- Mecanismo de propagação L1 — sem cascade novo.
- Pattern DEBT-37 N=3 → N=4 cumulativo crossing limiar com folga.
- Layout duplo arquitectural aceite N=1 inaugural.
- Template-passo replicado literal N=1 → N=2 cumulativo.
- Defaults preservam P262-P273.8 bit-exact.
- 10 sub-padrões aplicados.

**Sub-padrão "Anotação cumulativa em vez de ADR nova"** N=16 →
**N=17 cumulativo consolidação clara persistente** — décima anotação
consecutiva ADR-0091 (P270/P270.1/P270.2/P270.3/P273/P274/P273.5/
P273.6/P273.7/**P273.9**).

---

## §7 — L0 `entities/gradient.md` anotação P273.9

Adicionada anotação P273.9 após P273.7 — extensão Decisão 3 para
Grid + Stack + Pad; Decisões 1γ/2α/3 fixadas; bug-fix
`translate_frame_item` documentado; pattern DEBT-37 N=4 cumulativo;
layout duplo arquitectural aceite N=1 inaugural. Hash propagado via
`crystalline-lint --fix-hashes`:
`01_core/src/entities/gradient.rs:af1d57d2`.

---

## §8 — Pendências preservadas pós-P273.9

Inalteradas vs P273.8 (nível cluster):

- **P-Gradient-CMYK-ICC** (S-M).
- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

Pendências específicas pós-P273.9 (incremental per ADR-0054 graded):
- **P273.X-bis-group** — `FrameItem::Group` L3-only parent_bbox
  (era Decisão 1ε candidata; não materializada com 1γ).
- **P273.X-bis — Bbox medido pós-layout** (refino 3γ.2.β se layout
  duplo não basta).
- **P273.X-bis2 — Bbox.y topo-exacto inline** (P273.7 herdada).
- **Dedup bbox-aware** (P273.6 §9 herdada).

**Pós-P273.9 cluster Gradient refino estrutural extensivamente
encerrado** — Block + Boxed + Grid cell + Stack + Pad cobertos.
Próximo passo natural: sair do cluster Gradient (Group L3-only fica
como pendência incremental opcional).

---

## §9 — Limitações conscientes P273.9

- Stack/Pad usam layout duplo via `measure_content_constrained` —
  custo perf ~1.5-2× **apenas em pipelines com gradient
  `relative=parent`** (defaults Self_/None preservam zero overhead).
- Stack bbox medido replica handler `measure_content_constrained`
  Stack arm inline em vez de chamar via `Content::Stack {...}`
  construido — evita alocação de Content temporário; ~30 LOC L1
  vs 5 LOC se houvesse helper extraído. Refino candidato XS futuro.
- Pad bbox INNER (sem insets) — consistente com Block semantic.
  Refino OUTER (full pad rectangle com insets) fica candidato
  futuro se houver demanda empírica diferente.
- `FrameItem::Group` L3-only parent_bbox **NÃO** materializado em
  P273.9 (era Decisão 1β/1ε candidata) — fica como pendência
  específica `P273.X-bis-group`.
- Bug-fix `translate_frame_item` é tactical — não introduz cascade
  adicional; corrige apenas o site onde Shape passa por translate
  (Grid cell). Outros translate sites preserved literal.

---

## §10 — Marco final P273.9

**Cluster Gradient refino estrutural extensivamente encerrado**:

- L1 arms Grid + Stack + Pad save/restore real (escopo 1γ M).
- L1 helper translate_frame_item bug-fix preservar parent_bbox.
- L3 dispatcher inalterado desde P273.6 (sem cascade novo).
- 7 tests P273.9 + zero regressão P262-P273.8.
- Pattern DEBT-37 N=4 cumulativo crossing limiar com folga.
- Layout duplo arquitectural aceite N=1 inaugural emergente.

Cristalino oferece gradient API user-facing paridade vanilla em:
- Cross-variant runtime fields canónica 3/3 (focal + space + relative).
- Adaptive N multispace refino qualitativo (Linear+Radial).
- Parent bbox callsite real com consumer real em **Block + Boxed +
  Grid cell + Stack + Pad** (P273.6 + P273.7 + **P273.9**).

Sub-padrão **"Pattern DEBT-37 cell_origin_* replicado" N=4
cumulativo** crossing limiar formalização ADR meta N=3-4 com folga
consolidada — candidato meta-ADR formalização futura preserved
NÃO reservado.

Sub-padrão **"Layout duplo arquitectural aceite" N=1 inaugural
emergente** — estabelece precedente para containers sem dimensions
literais usarem `measure_content_constrained` pre-layout para
`parent_bbox`.

Cluster Gradient feature-complete + qualitativo + refino estrutural
extensivamente encerrado — pronto para saída cluster definitiva
(Group L3-only fica como pendência opcional).

---

## §11 — Referências

- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa
  P273.9; décima anotação consecutiva).
- ADR-0093 — Meta-metodologia evolução ADRs (Pattern 2 aplicação
  prática N=6 cumulativo).
- ADR-0094 — Meta-operacional specs (Pattern 1 cap LOC; aplicação
  prática N=8 cumulativo).
- ADR-0085 — Diagnóstico imutável (vigésimo consumo).
- ADR-0029 — Pureza física L1 (preserved; Rect/parent_bbox/
  measure_content_constrained são tipos dados + medição pura).
- ADR-0054 — Critério fecho DEBT-1 (graded — Decisão 1γ M magnitude
  aceite com risco regressão alto mitigado por defaults rigorosos).
- DEBT-37 (P84.6) — Pattern `cell_origin_*: Option<f64>` reused com
  consumer real P273.6 + P273.9 (sub-padrão N=4 cumulativo crossing
  limiar com folga).
- `00_nucleo/diagnosticos/typst-passo-273-9-diagnostico.md` — Fase
  A empírica + Decisões 1γ/2α/3 fixadas + critério §A.8.
- P273.7 — Boxed save/restore template (replicado em P273.9 Grid
  literal; Stack/Pad com adaptação).
- P273.6 — Cascade ~86 sites bulk-patched (bug latent
  `translate_frame_item` exposto + corrigido em P273.9).
- Spec P273.9 — `00_nucleo/materialization/typst-passo-273-9.md`.

---

*Relatório imutável produzido em 2026-05-18. Cluster Gradient refino
estrutural extensivamente encerrado para containers structural+inline
canónicos + Grid cell + Stack + Pad; pattern DEBT-37 N=4 cumulativo
crossing limiar com folga consolidada; sub-padrão "Layout duplo
arquitectural aceite" N=1 inaugural emergente estabelece precedente
metodológico para containers sem dimensions literais.*
