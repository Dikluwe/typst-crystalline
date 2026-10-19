# Relatório P273.12 — Dedup bbox-aware (refino arquitectural pós-P273.10)

**Data**: 2026-05-18.
**Magnitude**: S (~85 LOC L3; 0 L1; 6 testes; cap soft L3 70 estourado 21%).
**Cluster**: Visualize / Gradient (terceiro de 6 sub-passos para fechar cluster).
**Tipo**: refino arquitectural L3-puro — chave de dedup expandida `Arc::as_ptr → DedupKey { arc_ptr, bbox }`.
**Spec**: `00_nucleo/materialization/typst-passo-273-12.md`.

---

## §1 — Sumário executivo

**Limitação P273.6 §9 quarto bullet fechada definitivamente** —
limitação preserved em todos os relatórios P273.6-P273.11 ("gradient
com mesmo Arc usado em contextos distintos: actualmente primeiro
wins") é resolvida via refino arquitectural L3-puro:

- **L3 `DedupKey { arc_ptr: usize, bbox: Option<RectKey> }`** — chave
  de dedup expandida (Decisão 1β + 1γ Fase A).
- **L3 `RectKey(i32, i32, i32, i32)`** — quantização milipontos
  resolve problemas de `f64` em HashMap key + preserva precisão
  sub-typográfica.
- **L3 `dedup_key_for(g, effective_bbox)` helper** — uniformiza
  construção da chave em scan + emit sites.
- **L3 `scan_all_gradients.walk`** — `HashMap<DedupKey, usize>` em
  vez de `HashMap<usize, usize>`.
- **L3 `pattern_resources_for_page.walk`** — threading
  `parent_bbox_override` paralelo ao scan + DedupKey lookup.
- **L3 `emit_stroke_paint(_type1)`** — ganha `effective_bbox:
  Option<Rect>` param para construir DedupKey lookup.
- **L3 3 callsites de emit** — destructure `parent_bbox_at_emit`
  (revertendo P273.7.1 `_`) + passa como `effective_bbox`.
- **L3 3 signatures de `build_page_stream_*`** — type alias
  `HashMap<usize, usize>` → `HashMap<DedupKey, usize>`.
- **L1**: 0 LOC — ADR-0029 pureza física L1 preserved.

### Marcos arquitecturais P273.12

**(1) Sub-padrão "Dedup Arc::as_ptr resources" N=2 → N=3 cumulativo
crossing limiar formalização N=3-4** — P73 (image) + P263 (pattern) +
**P273.12 (pattern bbox-aware)**. Candidato meta-ADR formalização
NÃO reservado.

**(2) Sub-padrão emergente "Bug arquitectural intencional corrigido"
N=0 → N=1 inaugural** — limitação documentada P273.6 §9 corrigida 6
sub-passos depois com refino arquitectural deliberado. Distingue de
"Bug latent corrigido em scope creep" (defeito não-detectado).

**(3) Sub-padrão "Sub-passos consecutivos do mesmo cluster" N=7 →
N=8 cumulativo emergente** — P273.5/6/7/8/9/10/11/12.

**(4) Reverter P273.7.1 cleanup parcialmente** — 3 sites em
`build_page_stream_*` voltaram a destructurar `parent_bbox_at_emit`
(eram `_` per P273.7.1). Sub-padrão **"Cleanup tactical revertido
quando consumer real activa"** emergente, mas N=1 não documentado
como sub-padrão (ocorrência pontual).

### Decisões fixadas Fase A

1. **Decisão 1 (chave dedup)**: **1β + 1γ combinados** — struct
   `DedupKey` nomeado + `RectKey` quantizada milipontos.
2. **Decisão 2 (callsite vs scan-side)**: **2β scan-side** —
   scan computa effective_bbox; emit constrói DedupKey lookup com
   o mesmo `dedup_key_for` helper.
3. **Decisão 3 (cross-page)**: **3α global ao documento** —
   `pat_ptr_to_idx` permanece global; pattern PDF reusado entre
   pages.

### Defaults preservam P262-P273.11 bit-exact

- `relative=self/None` (`bbox=None`) → DedupKey factorizes a
  singleton key per Arc; preserved 1 pattern por Arc.
- Arc usado em context único → idem (1 entry no map).
- Apenas Arc com bboxes effective distintos em N contexts produz N
  PDF patterns (semântica correcta vs primeira-wins).
- 2632 baseline P273.11 preserved.

---

## §2 — Diff L3 antes/depois

### §2.1 — Tipos novos (~30 LOC)

```rust
// 03_infra/src/export.rs — antes de scan_all_gradients

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct RectKey(i32, i32, i32, i32);

fn rect_to_key(r: Rect) -> RectKey {
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

fn dedup_key_for(
    g: &Gradient,
    effective_bbox: Option<Rect>,
) -> DedupKey {
    let arc_ptr = match g {
        Gradient::Linear(l) => Arc::as_ptr(l) as usize,
        Gradient::Radial(r) => Arc::as_ptr(r) as usize,
        Gradient::Conic(c) => Arc::as_ptr(c) as usize,
    };
    DedupKey { arc_ptr, bbox: effective_bbox.map(rect_to_key) }
}
```

### §2.2 — `scan_all_gradients.walk` refactor (~10 LOC)

```diff
- fn walk(items, parent_bbox_override, ptr_to_idx: &mut HashMap<usize, usize>, ...) {
+ fn walk(items, parent_bbox_override, ptr_to_idx: &mut HashMap<DedupKey, usize>, ...) {
      for item in items {
          match item {
              FrameItem::Shape { stroke: Some(...), parent_bbox_at_emit, .. } => {
-                 let (ptr, kind) = match g { Linear(l) => (Arc::as_ptr(l) as usize, ...), ... };
-                 if ptr_to_idx.contains_key(&ptr) { continue; }
-                 // ... allocate ids + push GradientObject ...
-                 ptr_to_idx.insert(ptr, idx);
+                 let effective_bbox = parent_bbox_at_emit.or(parent_bbox_override);
+                 let key = dedup_key_for(g, effective_bbox);
+                 if ptr_to_idx.contains_key(&key) { continue; }
+                 // ... allocate ids + push GradientObject (com effective_bbox) ...
+                 ptr_to_idx.insert(key, idx);
              }
              FrameItem::Group { ... } => { walk(items, Some(group_bbox), ...); }
          }
      }
  }
```

### §2.3 — `pattern_resources_for_page.walk` refactor (~20 LOC)

```diff
- fn walk(items, ptr_to_idx: &HashMap<usize, usize>, refs, entries, seen) {
+ fn walk(items, parent_bbox_override: Option<Rect>, ptr_to_idx: &HashMap<DedupKey, usize>, refs, entries, seen) {
      for item in items {
          match item {
-             FrameItem::Shape { stroke: Some(Stroke { paint: Paint::Gradient(g), .. }), .. } => {
-                 let ptr = Arc::as_ptr(...) as usize;
-                 if let Some(&idx) = ptr_to_idx.get(&ptr) { ... }
+             FrameItem::Shape { stroke: ..., parent_bbox_at_emit, .. } => {
+                 let effective_bbox = parent_bbox_at_emit.or(parent_bbox_override);
+                 let key = dedup_key_for(g, effective_bbox);
+                 if let Some(&idx) = ptr_to_idx.get(&key) { ... }
              }
-             FrameItem::Group { items, .. } => { walk(items, ...); }
+             FrameItem::Group { pos, inner_width, inner_height, items, .. } => {
+                 let group_bbox = Rect { /* ... */ };
+                 walk(items, Some(group_bbox), ...);
+             }
          }
      }
  }
```

### §2.4 — `emit_stroke_paint(_type1)` signature (~10 LOC)

```diff
  fn emit_stroke_paint(
      ops: &mut String, paint: &Paint, thickness: f64,
+     effective_bbox: Option<Rect>,
-     pat_ptr_to_idx: &HashMap<usize, usize>,
+     pat_ptr_to_idx: &HashMap<DedupKey, usize>,
      pat_refs: &[PatternRef],
  ) {
      match paint {
          Paint::Gradient(g) => {
-             let ptr = match g { Linear(l) => Arc::as_ptr(l) as usize, ... };
-             if let Some(&idx) = pat_ptr_to_idx.get(&ptr) { ... }
+             let key = dedup_key_for(g, effective_bbox);
+             if let Some(&idx) = pat_ptr_to_idx.get(&key) { ... }
          }
      }
  }
```

### §2.5 — 3 callsites emit + 3 signatures `build_page_stream_*` (~15 LOC)

3 callsites em `build_page_stream_type1/cidfont/multifont`:

```diff
- FrameItem::Shape { ..., parent_bbox_at_emit: _ } => {
+ FrameItem::Shape { ..., parent_bbox_at_emit } => {
      // ...
      if let Some(s) = stroke {
-         emit_stroke_paint(&mut ops, &s.paint, s.thickness, pat_ptr_to_idx, pat_refs);
+         emit_stroke_paint(&mut ops, &s.paint, s.thickness,
+             *parent_bbox_at_emit, pat_ptr_to_idx, pat_refs);
      }
  }
```

3 signatures `build_page_stream_*`:
```diff
- pat_ptr_to_idx: &HashMap<usize, usize>,
+ pat_ptr_to_idx: &HashMap<DedupKey, usize>,
```

### §2.6 — L1 inalterado

- 0 LOC L1 — ADR-0029 pureza física preserved.
- Layouter inalterado desde P273.11.

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P273.12 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=18 → N=19 cumulativo consolidação clara persistente** | Décima segunda anotação consecutiva ADR-0091 |
| Reutilização literal helpers cross-passos | **N=17 preserved** | |
| Cap LOC hard vs soft explícito | **N=14 → N=15 cumulativo** | L3 hard 100 / soft 70 — real ~85 (estouro soft 21%; estouro hard zero). Registado per ADR-0094 Pattern 1 |
| Aplicação meta-ADR (ADR-0093) | **N=7 → N=8 cumulativo** | Pattern 2 |
| Aplicação meta-ADR (ADR-0094) | **N=10 → N=11 cumulativo** | Pattern 1 |
| Pattern DEBT-37 `cell_origin_*` replicado | **N=4 preserved** | P273.12 sem touch DEBT-37 |
| Template-passo replicado literal | **N=2 preserved** | |
| Sub-passos consecutivos do mesmo cluster | **N=7 → N=8 cumulativo emergente** | P273.5/6/7/8/9/10/11/12 |
| Layout duplo arquitectural aceite | **N=1 preserved** | |
| L3-only parent_bbox | **N=1 reused** | P273.12 também é L3-only |
| Extract helper de replicação inline | **N=1 preserved** | |
| **Dedup Arc::as_ptr resources** | **N=2 → N=3 cumulativo crossing limiar formalização N=3-4** | P73 image + P263 pattern + **P273.12 pattern bbox-aware**. Candidato meta-ADR NÃO reservado |
| **Bug arquitectural intencional corrigido** | **N=0 → N=1 inaugural emergente** | Limitação documentada 6 sub-passos antes, corrigida com refino deliberado |
| Diagnóstico imutável | **N=27 → N=28 cumulativo** | Vigésimo terceiro consumo |

---

## §4 — Métricas finais

| Métrica | Pré-P273.12 | Pós-P273.12 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2632 | **2638** | +6 |
| Tests P273.12 novos | — | 6 | 6 export L3 integration tests |
| Tests P262-P273.11 (verdes) | preserved | preserved | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Build warnings (novos) | 0 | 0 | 0 |
| Hashes propagados L0 | — | 1 (`gradient.rs:c39d7e15`) | +1 |
| ADRs totais | 81 | **81** | 0 (sem nova ADR; décima segunda anotação ADR-0091) |
| LOC L3 (additions+changes) | — | **~85** | RectKey/DedupKey/helper ~30 + scan refactor ~10 + pattern_resources refactor ~20 + emit_stroke_paint ~10 + 3 callsites + 3 signatures ~15 |
| LOC L1 (additions) | — | 0 | literal (ADR-0029 preserved) |

### §política condições verificadas

- ✓ Cap LOC L3 hard 100 — real ~85; folga 15%.
- ⚠ Cap LOC L3 soft 70 — real ~85; **estouro soft 21%** registado
  per ADR-0094 Pattern 1.
- ✓ Cap LOC L1 hard 0 — real 0; literal.
- ✓ Cap testes hard 12 — real 6; folga 50%.
- ✓ Cap testes soft 8 — real 6; folga 25%.
- ✓ Defaults preservam pipeline P262-P273.11 bit-exact.
- ✓ Lint zero preserved.
- ✓ Regressão tests P262-P273.11 zero (2632 baseline preserved).
- ✓ ADR-0029 pureza física L1 preserved (P273.12 L3-only).
- ✓ E2E observable: 6 testes export confirmam bbox-aware dedup
  (same/different/3-contexts/observable diff).

**10 condições §política verificadas — 9 satisfeitas absolutas + 1
estouro soft L3** registado per ADR-0094 Pattern 1.

**Análise estouro**: Spec §3 caps (hard 100 / soft 70) estimou ~73
LOC. Real ~85 vem de: scope creep `pattern_resources_for_page` walk
threading override (~10 LOC); signatures `build_page_stream_*` (3
sites) (~3 LOC); destructure revertendo P273.7.1 cleanup (3 sites)
(~5 LOC). Soma fica 12 LOC acima estimativa Fase A. Estouro hard cap
zero — feature contida.

---

## §5 — Verificação regressão zero P262-P273.11

**2632 baseline preservado bit-exact**:

- typst-core: 2179 preserved.
- typst-shell: 24 preserved.
- typst-infra: 406 → **412** (+6 P273.12 export tests).
- typst-wiring + bins + outros: 23 preserved.

**Total: 2632 → 2638 (+6 net)**.

**2 testes typst-core skipped** (stack overflow pré-existente) —
NÃO regressão P273.12.

Mecânica:
- Shapes com `parent_bbox_at_emit=None` → DedupKey `{arc_ptr, None}`
  factorizes a singleton key per Arc; preserved behavior (Self_/None).
- Shapes top-level com bbox populated → DedupKey `{arc_ptr, Some(bbox)}`;
  se único Arc/bbox combo, 1 pattern (preserved P273.6).
- Arc usado em 2+ callsites com mesma bbox → DedupKey idêntica; 1
  pattern (preserved P273.6).
- Apenas Arc + bboxes effective DIFERENTES → DedupKeys distintas; N
  patterns (semântica correcta vs primeira-wins).

---

## §6 — Anotação cumulativa ADR-0091 (décima segunda consecutiva)

Adicionada §"Anotação cumulativa P273.12 — Dedup bbox-aware" cobrindo:

- Decisões 1β+1γ (DedupKey + RectKey milipontos), 2β (scan-side),
  3α (global ao documento).
- Cascade emit_stroke_paint + 3 callsites + 3 build_page_stream
  signatures.
- Sub-padrões: Dedup Arc::as_ptr resources N=3 crossing limiar; Bug
  arquitectural intencional corrigido N=1 inaugural; Sub-passos
  consecutivos N=8.
- Defaults preservam P262-P273.11 bit-exact.
- Trade-off PDF size aceito.

---

## §7 — L0 `entities/gradient.md` anotação P273.12

Adicionada anotação P273.12 após P273.10 — fecho limitação P273.6 §9
quarto bullet via chave dedup expandida + sub-padrões aplicados.
Hash propagado: `01_core/src/entities/gradient.rs:c39d7e15`.

---

## §8 — Pendências preservadas pós-P273.12

Inalteradas vs P273.11 (nível cluster):

- **P-Gradient-CMYK-ICC** (S-M; **VERIFICAR Fase A se krilla API existe**).
- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

Sequência para fechar cluster Gradient:

- **P273.13** — CMYK-ICC krilla paridade (S-M; possível bloqueador API).
- **P273.14** — Bbox medido pós-layout (M).
- **P273.15** — Bbox.y topo-exacto inline (M-L; **BLOQUEADO** por
  DEBT-56).

**Predição factual**: cluster Gradient fecha empíricamente entre
P273.13 e P273.15 consoante disponibilidade da krilla API.

---

## §9 — Limitações conscientes P273.12

- Quantização milipontos (1 mpt = 0.001 pt) — sub-mpt precision
  perde-se. Aceitável; typografia trabalha em pt.
- N callsites mesmo Arc + N bboxes distintos → N PDF patterns (PDF
  size inflation no pior caso). Caso comum (Arc único context ou
  contextos idênticos) preserved.
- Dedup é per-`(Arc, bbox)` — gradient com mesma definition mas Arc
  diferente continua a ser tratado como gradient distinto. Aceitável
  (Arc é identidade de definition).
- `draw_item_local` Group recursion path (linhas 2347+) usa solid
  fallback `s.paint.to_color()` para gradient strokes — NÃO consume
  pattern dict. Limitação pré-existente P273.10 preserved; gradients
  efectivos dentro de Groups via draw_item_local renderizam como
  solid color. Refino candidato P273.X-bis.
- Não toca dedup de outros resources (imagens via P73). Cluster
  específico.

---

## §10 — Marco final P273.12

**Limitação documentada P273.6 §9 corrigida definitivamente**:

- L3 `DedupKey { arc_ptr, bbox: Option<RectKey> }` substitui
  `Arc::as_ptr` only.
- L3 `dedup_key_for` helper uniformiza construção em scan + emit.
- L3 cascade controlado: 3 emit sites + 3 build_page_stream
  signatures + 1 emit_stroke_paint signature.
- L1: 0 LOC (ADR-0029 preserved).
- 6 tests P273.12 + zero regressão P262-P273.11.

Sub-padrão **"Dedup Arc::as_ptr resources" N=2 → N=3 cumulativo
crossing limiar formalização N=3-4** com folga consolidada — P73
(image) + P263 (pattern) + **P273.12 (pattern bbox-aware)** —
candidato meta-ADR formalização NÃO reservado.

Sub-padrão **"Bug arquitectural intencional corrigido" N=0 → N=1
inaugural emergente** — estabelece precedente para limitações
documentadas com decisão deliberada de adiar fix arquitectural;
corrigidas em sub-passo subsequente quando contexto madura.
Distingue de "Bug latent corrigido em scope creep".

Sub-padrão **"Sub-passos consecutivos do mesmo cluster" N=7 → N=8
cumulativo emergente** (P273.5/6/7/8/9/10/11/12) — consolidação
máxima preservada.

Cluster Gradient feature-complete + qualitativo + refino estrutural
extensivamente encerrado + cleanup intra-cluster + **dedup
bbox-aware** — próximo passo: **P273.13** CMYK-ICC krilla paridade
(verificar Fase A se krilla API existe — possível bloqueador
externo).

---

## §11 — Referências

- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa
  P273.12; décima segunda anotação consecutiva).
- ADR-0093 — Meta-metodologia evolução ADRs (Pattern 2 aplicação
  prática N=8 cumulativo).
- ADR-0094 — Meta-operacional specs (Pattern 1 cap LOC; aplicação
  prática N=11 cumulativo; estouro soft 21% registado).
- ADR-0085 — Diagnóstico imutável (vigésimo terceiro consumo).
- ADR-0029 — Pureza física L1 (preserved; P273.12 L3-only).
- ADR-0054 — Critério fecho DEBT-1 (graded — bug arquitectural
  intencional aceito).
- P273.6 §9 quarto bullet — limitação fechada definitivamente.
- P273.10 — Group L3-only (preserved; P273.12 reusa walk infra).
- P273.7.1 — cleanup `parent_bbox_at_emit: _` (3 sites revertidos
  por consumer real activo).
- P73 — image dedup `Arc::as_ptr` (precedente sub-padrão N=1).
- P263 — pattern dedup `Arc::as_ptr` (precedente sub-padrão N=2).
- Spec P273.12 — `00_nucleo/materialization/typst-passo-273-12.md`.
- `00_nucleo/diagnosticos/typst-passo-273-12-diagnostico.md` — Fase
  A empírica + Decisões 1β+1γ/2β/3α + critério §A.8.

---

*Relatório imutável produzido em 2026-05-18. Limitação P273.6 §9
quarto bullet preserved em 6 sub-passos corrigida via refino
arquitectural deliberado; sub-padrão "Dedup Arc::as_ptr resources"
N=3 cumulativo crossing limiar formalização N=3-4 com folga; "Bug
arquitectural intencional corrigido" N=1 inaugural emergente.*
