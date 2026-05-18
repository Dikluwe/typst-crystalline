# Relatório P273.7 — P-Gradient-Relative-Callsite-Boxed (completa Decisão 3 P273.6)

**Data**: 2026-05-17.
**Magnitude**: XS-S (L1 ~20 LOC arm Boxed; L3 0 LOC; 8 testes; cap soft L1 20 respeitado).
**Cluster**: Visualize / Gradient (extensão final do refino estrutural).
**Tipo**: sub-passo decimal P273.7 — fecho pendência Boxed P273.6.
**Spec**: `00_nucleo/materialization/typst-passo-273-7.md`.

---

## §1 — Sumário executivo

**Cluster Gradient refino estrutural definitivamente encerrado para
containers structural+inline canónicos** via P273.7 — pendência
P273.6 §8 (Boxed save/restore) resolvida:

- **L1 arm `Content::Boxed`** ganha save/restore real do `parent_bbox`
  (3γ.2.γ — popula apenas quando `width+height` literais; bbox.y
  baseline-relative per Decisão 1 Fase A).
- **L1 emit shape site interno Boxed** (linha ~1516) já populated
  desde P273.6 — **inalterado**.
- **L3 dispatcher** já consome `effective_parent_bbox` desde P273.6
  — **inalterado**.
- **Cascade ~86 sites** já feito P273.6 — **inalterado**.

### Marcos arquitecturais P273.7

**(1) Cluster Gradient refino estrutural definitivamente encerrado
para containers canónicos** — Decisão 3 P273.6 estendida de `{Block}`
para `{Block, Boxed}`. Stack/Pad/Group/Grid cell ficam permanentemente
como pendência incremental per ADR-0054 graded.

**(2) Sub-padrão emergente "Template-passo replicado literal" N=0 →
N=1** — P273.7 inaugura padrão: replicação literal do save/restore
P273.6 a outro arm (Boxed) com diferença mínima (bbox.y semantic
baseline-relative vs topo). Análogo histórico P156H/P156G.

**(3) Sub-padrão "Sub-passos decimais consecutivos do mesmo cluster"
N=2 → N=3 cumulativo emergente** — P273.5 + P273.6 + P273.7.

**(4) Pattern DEBT-37 N=3 cumulativo preserved** — P273.7 é extensão
da 3ª aplicação (mesmo cluster Gradient + mesmo campo `parent_bbox`),
não 4ª aplicação independente. Limiar formalização ADR meta N=3-4
mantido.

### Decisão fixada Fase A

**Decisão 1 (semântica bbox.y inline)**: **`3γ.2.γ-inline-baseline-y`**
— `bbox.y = self.regions.current.cursor_y` literal (baseline-relative).
Aproximação aceitável; coerente com limitação consciente P156H
"height em contexto inline alteraria line_height — refino futuro";
refino topo-exacto fica registado como `P273.X-bis2` per ADR-0054
graded.

### Decisões herdadas P273.6 literal

- **Decisão 2 (semântica W/H)**: 3γ.2.γ — popular apenas quando
  `width.is_some() && height.is_some()`.
- **Decisão 3 (propagação L1→L3)**: Prop-A revisitada — inalterada;
  emit shape sites do Boxed já populam desde P273.6.
- **Decisão 4 (escopo contentores)**: estendida `{Block}` →
  `{Block, Boxed}`. Stack/Pad/Group/Grid cell scope-out per
  ADR-0054 graded.

### Defaults preservam P262-P273.6 bit-exact

- Boxed sem dimensions literais → `parent_bbox` outer preservado
  (LIFO restore; cai eventualmente no page fallback).
- Self_/None relative ignora `parent_bbox_at_emit` (paridade Block).
- 2612 baseline P273.6 preserved.

---

## §2 — Diff L1 antes/depois

### §2.1 — L1 arm `Content::Boxed` save/restore (~20 LOC)

`01_core/src/rules/layout/mod.rs` — inserção antes/depois do callsite
`self.layout_content(body)` (linha 1394 antes do patch):

```rust
// Content::Boxed { body, width, height, inset, baseline,
//                  outset, radius, clip, fill, stroke } arm
// ...
let body_items_before = self.regions.current.current_items.len();

// P273.7 — save/restore parent_bbox análogo Block P273.6
// (Decisão 3γ.2.γ: popular apenas quando width+height
// literais). Decisão 1 Fase A `3γ.2.γ-inline-baseline-y`:
// bbox.y = cursor.y baseline-relative (coerente com P156H
// limitação line_height inline).
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

self.layout_content(body);

// P273.7 — restore parent_bbox (LIFO). Shape emit do
// próprio Boxed (linha ~1516) usa parent_bbox outer
// (paridade Block P273.6 §2.3).
self.parent_bbox = saved_parent_bbox;
// ...
```

### §2.2 — Nada mais alterado

- **L1 emit shape site Boxed**: inalterado — `parent_bbox_at_emit:
  self.parent_bbox` populated desde P273.6 (linha 1495 original,
  agora ~1516).
- **L1 cascade ~86 sites**: inalterado — já feito P273.6.
- **L1 Layouter `parent_bbox` field**: inalterado — já consumed
  desde P273.6.
- **L3 GradientObject + dispatcher**: 0 LOC — já consomem
  `effective_parent_bbox` desde P273.6.

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P273.7 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=15 → N=16 cumulativo consolidação clara persistente** | Nona anotação consecutiva ADR-0091 |
| Reutilização literal helpers cross-passos | **N=15 → N=16 cumulativo consolidação clara persistente** | Template save/restore P273.6 reused literal |
| Pattern DEBT-37 `cell_origin_*` replicado | **N=3 → N=3 cumulativo preserved** | P273.7 é extensão da 3ª aplicação, não 4ª; limiar mantido |
| Cap LOC hard vs soft explícito | **N=9 → N=10 cumulativo consolidação total** | L1 hard 30 / soft 20 — real ~20 LOC (cap soft respeitado) |
| Aplicação meta-ADR (ADR-0093) | **N=4 → N=5 cumulativo** | Pattern 2 anotação cumulativa; quinta aplicação prática |
| Aplicação meta-ADR (ADR-0094) | **N=5 → N=6 cumulativo** | Pattern 1 cap LOC; sexta aplicação prática |
| Sub-passos decimais consecutivos do mesmo cluster | **N=2 → N=3 cumulativo emergente** | P273.5 + P273.6 + P273.7 |
| Cascade pattern-match cross-FrameItem | **N=2 → N=2 cumulativo preserved** | P273.7 sem cascade novo (reutiliza ~86 sites P273.6) |
| **Template-passo replicado literal** | **N=0 → N=1 emergente** | P273.7 inaugura: save/restore P273.6 replicado a Boxed com diferença bbox.y baseline-y |
| Diagnóstico imutável | **N=22 → N=23 cumulativo** | Décimo oitavo consumo directo de fonte |

---

## §4 — Métricas finais

| Métrica | Pré-P273.7 | Pós-P273.7 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2612 | **2620** | +8 |
| Tests P273.7 novos | — | 8 | 5 L1 layout + 3 L3 E2E |
| Tests P262-P273.6 (verdes) | preserved | preserved | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Build dead_code warnings novos | 0 | 0 | 0 (warnings pré-existentes export.rs P273.6 inalterados) |
| Hashes propagados L0 | — | 1 (`gradient.rs:2bb71a44`) | +1 |
| ADRs totais | 81 | **81** | 0 (sem nova ADR; nona anotação ADR-0091) |
| LOC L1 (additions) | — | ~20 | cap hard 30 (folga 33%); cap soft 20 respeitado limite |
| LOC L3 (additions) | — | 0 | cap hard 0 (literal) |

### §política condições verificadas

- ✓ Cap LOC L1 hard 30 — real ~20; folga 33%.
- ✓ Cap LOC L1 soft 20 — real ~20; limite respeitado.
- ✓ Cap LOC L3 hard 0 — real 0; literal.
- ✓ Cap testes hard 8 — real 8; limite respeitado.
- ✓ Cap testes soft 5 — real 8; estouro 60% registado (5 L1 + 3 L3
  E2E necessários para cobrir LIFO + nested + own_shape + observable
  diff + Self_ preserved). Per ADR-0094 Pattern 1.
- ✓ Defaults preservam pipeline P262-P273.6 bit-exact.
- ✓ ADR-0029 pureza física L1 preserved (Rect é tipo dados;
  parent_bbox é metadata; save/restore é gestão RAM).
- ✓ Lint zero; L0 hash drift propagado.
- ✓ Regressão tests P262-P273.6 zero (2612 baseline preserved).
- ✓ **E2E observable diff** confirmado — test
  `p273_7_shape_inside_boxed_carries_parent_bbox_observable_diff`
  verifica que bytes PDF DIFEREM quando bbox Boxed real ≠ page
  (200×100pt @ baseline y=100 vs page 595×842pt).

**12 condições §política verificadas — 11 satisfeitas absolutas + 1
estouro soft (tests soft) registado** per ADR-0094 Pattern 1.

---

## §5 — Verificação regressão zero P262-P273.6

**2612 baseline preservado bit-exact**:

- typst-core: 2169 → **2174** (+5 P273.7 layout tests).
- typst-shell: 24 preserved.
- typst-infra: 396 → **399** (+3 P273.7 export tests).
- typst-wiring + bins + outros: 23 preserved.

**Total: 2612 → 2620 (+8 net)**.

**2 testes typst-core skipped** (`recursao_profunda_retorna_err`,
`recursao_infinita_retorna_err_sem_crash`) — stack overflow
pré-existente em árvore limpa pré-P273.5; **não é regressão P273.7**.
Verificado via `git stash && cargo test recursao_profunda` em árvore
PRE-P273.5 — falha idêntica.

Mecânica:
- Boxed sem dimensions literais (Decisão 2 herdada) → `parent_bbox`
  outer preservado (LIFO restore).
- Self_/None relative branch literal preserved.
- Shape emit do próprio Boxed usa `self.parent_bbox` outer (restore
  acontece ANTES do shape emit).

§política condições "Regressão tests P262-P273.6 zero" + "Defaults
preservam pipeline" satisfeitas absolutas.

---

## §6 — Anotação cumulativa ADR-0091 (nona consecutiva)

Adicionada §"Anotação cumulativa P273.7 — Boxed save/restore (completa
Decisão 3 P273.6)" cobrindo:

- Decisão 1 fixada: 3γ.2.γ-inline-baseline-y (4 razões documentadas).
- Decisões 2/3/4 herdadas P273.6 literal.
- Mudanças L1 apenas (zero L3; zero cascade novo).
- Limitação consciente bbox.y aproximada (refino diferido
  `P273.X-bis2`).
- 10 sub-padrões aplicados (8 cumulativos + 1 preserved + 1
  emergente N=1).

**Sub-padrão "Anotação cumulativa em vez de ADR nova"** N=15 →
**N=16 cumulativo consolidação clara persistente** — nona anotação
consecutiva ADR-0091 (P270/P270.1/P270.2/P270.3/P273/P274/P273.5/
P273.6/**P273.7**).

---

## §7 — L0 `entities/gradient.md` anotação P273.7

Adicionada anotação P273.7 após P273.6 — extensão Decisão 3
`{Block}` → `{Block, Boxed}`; Decisão 1 inline-baseline-y fixada;
sem cascade novo; herdadas Decisões 2/3/4 P273.6 literal. Hash
propagado via `crystalline-lint --fix-hashes`:
`01_core/src/entities/gradient.rs:2bb71a44`.

---

## §8 — Pendências preservadas pós-P273.7

Inalteradas vs P273.6 (nível cluster):

- **P-Gradient-CMYK-ICC** (S-M; krilla paridade ICC profiles).
- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

**Pendências específicas pós-P273.7** (incremental per ADR-0054
graded):
- **P273.9 — Stack/Pad/Group/Grid cell save/restore** (out of scope;
  candidato apenas se houver demanda empírica). [P273.8 foi renomeado
  para cleanup `unused_variable: parent_bbox_at_emit`; esta reserva
  avança para P273.9.]
- **P273.X-bis — Bbox medido pós-layout** (refino 3γ.2.β/α se
  3γ.2.γ for empiricamente insuficiente).
- **P273.X-bis2 — Bbox.y topo-exacto inline** (refino Decisão 1 se
  aproximação baseline-y for visualmente insuficiente; requer
  refactor inline line_height — diferido permanente per ADR-0054
  graded; coerente com P156H).
- **Dedup bbox-aware** — gradient com mesmo Arc em contextos
  distintos.

**Pós-P273.7 cluster Gradient refino estrutural definitivamente
encerrado para containers structural+inline canónicos**. Próximo
passo natural: sair do cluster Gradient.

---

## §9 — Limitações conscientes P273.7

Per spec §5:

- **Decisão 3γ.2.γ-inline-baseline-y** — `bbox.y` populated com
  `cursor.y` baseline-relative em vez de topo do box. Aproximação
  aceitável per ADR-0054 graded; alinhada com limitação P156H
  "height em contexto inline alteraria line_height — refino futuro".
  Refino topo-exacto fica registado como `P273.X-bis2`.
- Boxed sem dimensions literais continua a usar `parent_bbox` outer
  (que pode ser fallback page_bbox).
- Lista de contentores activos: **{Block, Boxed}**. Stack/Pad/Grid
  cell/FrameItem::Group continuam scope-out per ADR-0054 graded.
- Dedup bbox-aware continua aberto — gradients dedup'd por Arc;
  primeira occurrence captura bbox. P273.7 não altera essa
  limitação.
- 4 warnings pré-existentes `unused variable: parent_bbox_at_emit`
  em `03_infra/src/export.rs` (linhas 2003/2275/2521/2705) — vêm do
  cascade P273.6 (binding em destructures sem uso); NÃO introduzidos
  por P273.7. Candidato cleanup XS futuro: ignorar via pattern
  `parent_bbox_at_emit: _`.

---

## §10 — Marco final P273.7

**Cluster Gradient refino estrutural definitivamente encerrado para
containers canónicos**:

- L1 arm `Content::Boxed` save/restore real (3γ.2.γ-inline-baseline-y).
- L1 emit shape site interno Boxed inalterado (populated desde
  P273.6).
- L3 dispatcher + cascade inalterados.
- 8 tests P273.7 (+5 L1 layout, +3 L3 E2E) + zero regressão
  P262-P273.6.
- E2E observable diff confirmado também para Boxed.

Cristalino oferece gradient API user-facing paridade vanilla em:
- Cross-variant runtime fields canónica 3/3 (focal + space +
  relative).
- Adaptive N multispace refino qualitativo (Linear+Radial).
- Parent bbox callsite real com consumer real em **Block + Boxed**
  (P273.6 + **P273.7**).

Sub-padrão **"Template-passo replicado literal" N=1 emergente
inaugural** — P273.7 estabelece precedente para replicação literal
de save/restore patterns cross-arm com diferença mínima
documentada.

Cluster Gradient feature-complete + qualitativo + refino estrutural
definitivamente encerrado para containers structural+inline
canónicos — pronto para saída cluster.

---

## §11 — Referências

- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa
  P273.7; nona anotação consecutiva).
- ADR-0093 — Meta-metodologia evolução ADRs (Pattern 2 aplicação
  prática N=5 cumulativo).
- ADR-0094 — Meta-operacional specs (Pattern 1 cap LOC; aplicação
  prática N=6 cumulativo).
- ADR-0085 — Diagnóstico imutável (décimo oitavo consumo).
- ADR-0029 — Pureza física L1 (preserved; Rect/parent_bbox são tipos
  dados + metadata).
- ADR-0054 — Critério fecho DEBT-1 (graded — Decisão 1
  inline-baseline-y; refino topo-exacto diferido `P273.X-bis2`).
- DEBT-37 (P84.6) — Pattern `cell_origin_*: Option<f64>` reused com
  consumer real (sub-padrão "Pattern DEBT-37 replicado" N=3
  cumulativo preserved; P273.7 é extensão da 3ª aplicação).
- P156H — Limitação consciente line_height inline (motiva Decisão 1
  baseline-y vs topo-exacto).
- `00_nucleo/diagnosticos/typst-passo-273-7A-diagnostico.md` — Fase
  A empírica + Decisão 1 fixada + critério §A.6.
- P273.6 — Spec original; Boxed save/restore deixado como pendência
  (resolvida P273.7).
- Spec P273.7 — `00_nucleo/materialization/typst-passo-273-7.md`.

---

*Relatório imutável produzido em 2026-05-17. Linhagem completa
preservada — cluster Gradient refino estrutural definitivamente
encerrado para containers structural+inline canónicos; 3γ.2
materializada para `{Block, Boxed}`; sub-padrão "Template-passo
replicado literal" N=1 emergente inaugural.*
