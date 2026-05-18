# Relatório P273.13 — Fix draw_item_local Group gradient (caminho emit real)

**Data**: 2026-05-18.
**Magnitude**: S (~35 LOC L3; 0 L1; 6 testes; cap soft L3 50 respeitado com folga).
**Cluster**: Visualize / Gradient (quarto sub-passo na sequência terminar cluster — inserido por priorização B).
**Tipo**: refino estrutural L3-puro — fix tactical de pendência específica P263 §8 #3 + P273.12 §9 quarto bullet.
**Spec**: `00_nucleo/materialization/typst-passo-273-13.md`.

---

## §1 — Sumário executivo

**Pendência cumulativa fechada definitivamente**:

- **P263 §8 #3 (2026-05-16)**: "`draw_item_local` Gradient support
  pendência — função recursiva para shapes em groups com `cm`
  transformations não recebe `pat_ptr_to_idx`/`pat_refs` no escopo
  recursivo. Fallback `first_stop_color` para esse caso específico."
- **P273.12 §9 quarto bullet**: "`draw_item_local` Group recursion
  path (linhas 2347+) usa solid fallback `s.paint.to_color()` para
  gradient strokes — NÃO consume pattern dict. Limitação pré-existente
  P273.10 preserved; gradients efectivos dentro de Groups via
  draw_item_local renderizam como solid color."

P273.13 fix tactical L3-puro:
- **L3 `draw_item_local`** ganha 3 params (`parent_bbox_override`,
  `pat_ptr_to_idx`, `pat_refs`) paralelo P273.10
  `scan_all_gradients.walk`.
- **L3 `draw_item_local` arm Shape** destructura
  `parent_bbox_at_emit` (revertendo P273.7.1 `_`); computa
  `effective_bbox` (Inner-wins paridade P273.10); substitui solid
  fallback por `emit_stroke_paint` com DedupKey lookup.
- **L3 `draw_item_local` arm Group novo** (scope creep aceito Fase
  A §A.4): constrói `group_bbox` literal-equivalente; recurse
  paralelo scan.
- **L3 3 callsites em build_page_stream_*** Group arms: passam
  `Some(group_bbox)`, `pat_ptr_to_idx`, `pat_refs`.
- **L1**: 0 LOC — ADR-0029 pureza física L1 preserved.

### Marcos arquitecturais P273.13

**(1) Sub-padrão "L3-only parent_bbox" N=1 → N=2 cumulativo
emergente** — P273.10 inaugural (scan_all_gradients.walk) +
**P273.13 reaplicação (draw_item_local)**. Padrão consolidado:
parameter threading para `(parent_bbox_override, pat_ptr_to_idx,
pat_refs)` em walkers recursivos L3.

**(2) Sub-padrão "Triplicação Group bbox" N=0 → N=1 emergente** —
3 sítios constroem `group_bbox` literal-equivalente
(`scan_all_gradients.walk` + `pattern_resources_for_page.walk` +
`draw_item_local`). Candidato extract helper P273.X-bis-helper-group-bbox
NÃO reservado (precedente P273.11 "Extract helper de replicação
inline" N=1).

**(3) Sub-padrão "Sub-passos consecutivos do mesmo cluster" N=8 →
N=9 cumulativo emergente** — P273.5/6/7/8/9/10/11/12/13.

**(4) Bug pré-existente corrigido implicitamente**: nested Groups
silenciosamente descartados via `_ => {}` catch-all em
`draw_item_local`. P273.13 arm Group novo corrige + suporta recurse
paralelo scan.

### Decisões fixadas Fase A

1. **Decisão 1 (propagação)**: **1α — parameter threading explícito**
   (coerência P273.10).
2. **Decisão 2 (Group bbox source)**: **2α — Group bbox próprio
   literal-equivalente** (paridade exacta scan).
3. **Decisão 3 (coords)**: **3α — coords cristalino** (Y-down;
   paridade scan).

### Defaults preservam P262-P273.12 bit-exact

- Shapes top-level (não dentro de Group) → caminho directo
  `emit_stroke_paint` em `build_page_stream_*` (sem mudança).
- Shapes dentro de Group sem gradient → solid color preserved
  literal.
- Shapes dentro de Group com Self_/None → DedupKey `{arc_ptr, None}`
  lookup encontra pattern registado.
- Shapes dentro de Group com gradient relative=parent → DedupKey
  `{arc_ptr, Some(rect_to_key(group_bbox))}` lookup encontra pattern.
- 2638 baseline P273.12 preserved.

### Patterns registados pós-P273.12 agora consumidos

P273.12 explicitamente registou patterns para gradients dentro de
Groups mas `draw_item_local` continuou a usar solid fallback —
patterns ficavam "unused declarations" em PDF resources. P273.13
fecha esse ciclo: render real chama `emit_stroke_paint` → DedupKey
lookup → renderiza `/Pattern CS /Pi SCN`.

---

## §2 — Diff L3 antes/depois

### §2.1 — `draw_item_local` signature + Shape arm + Group arm novo (~25 LOC)

```diff
- fn draw_item_local(ops: &mut String, item: &FrameItem) {
+ fn draw_item_local(
+     ops: &mut String,
+     item: &FrameItem,
+     parent_bbox_override: Option<Rect>,
+     pat_ptr_to_idx: &HashMap<DedupKey, usize>,
+     pat_refs: &[PatternRef],
+ ) {
      use typst_core::entities::geometry::ShapeKind;
-     use typst_core::entities::layout_types::FrameItem;
+     use typst_core::entities::layout_types::{FrameItem, Pt, Rect};
      match item {
-         FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit: _ } => {
+         FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit } => {
              // ...
              if let Some(s) = stroke {
-                 let (r, g, b, _) = s.paint.to_color().to_rgba_f32();
-                 ops.push_str(&format!("{:.3} {:.3} {:.3} RG\n{:.2} w\n", r, g, b, s.thickness));
+                 // P273.13 — substitui solid fallback pela chamada
+                 // emit_stroke_paint consumindo pattern dict.
+                 let effective_bbox = parent_bbox_at_emit.or(parent_bbox_override);
+                 emit_stroke_paint(ops, &s.paint, s.thickness,
+                     effective_bbox, pat_ptr_to_idx, pat_refs);
              }
              // ... resto preserved ...
          }
+         // P273.13 — arm Group novo (scope creep §A.4): suporta
+         // nested Groups + propaga parent_bbox_override.
+         FrameItem::Group { pos, inner_width, inner_height, items, .. } => {
+             let group_bbox = Rect {
+                 x: Pt(pos.x.0),
+                 y: Pt(pos.y.0),
+                 w: Pt(*inner_width),
+                 h: Pt(*inner_height),
+             };
+             for child in items {
+                 draw_item_local(ops, child, Some(group_bbox),
+                     pat_ptr_to_idx, pat_refs);
+             }
+         }
          _ => {}
      }
  }
```

### §2.2 — 3 callsites em `build_page_stream_*` Group arms (~10 LOC)

```diff
              if let Some(mask) = clip_mask {
                  emit_shape_path_local(&mut ops, mask, *inner_width, *inner_height);
                  ops.push_str("W n\n");
              }

+             // P273.13 — Group bbox literal-equivalente scan_all_gradients.walk
+             // (Decisão 2α + 3α paridade exacta); DedupKey lookup encontra
+             // pattern registado pelo scan.
+             let group_bbox = Rect {
+                 x: Pt(pos.x.0),
+                 y: Pt(pos.y.0),
+                 w: Pt(*inner_width),
+                 h: Pt(*inner_height),
+             };
              for child in items {
-                 draw_item_local(&mut ops, child);
+                 draw_item_local(&mut ops, child, Some(group_bbox),
+                     pat_ptr_to_idx, pat_refs);
              }
              ops.push_str("Q\n");
```

Aplicado em 3 sítios (linhas 2234/2751/2937 — `build_page_stream_type1`,
`build_page_stream_cidfont`, `build_page_stream_multifont`).

### §2.3 — L1 inalterado

- 0 LOC L1 — ADR-0029 pureza física preserved.
- Layouter inalterado desde P273.11.

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P273.13 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=19 → N=20 cumulativo consolidação clara persistente** | Décima terceira anotação consecutiva ADR-0091 |
| Reutilização literal helpers cross-passos | **N=17 preserved** | |
| Cap LOC hard vs soft explícito | **N=15 → N=16 cumulativo** | L3 hard 70 / soft 50 — real ~35 (folga 30%) |
| Aplicação meta-ADR (ADR-0093) | **N=8 → N=9 cumulativo** | Pattern 2 |
| Aplicação meta-ADR (ADR-0094) | **N=11 → N=12 cumulativo** | Pattern 1 |
| Pattern DEBT-37 replicado | **N=4 preserved** | |
| Template-passo replicado literal | **N=2 preserved** | |
| Sub-passos consecutivos do mesmo cluster | **N=8 → N=9 cumulativo emergente** | P273.5/6/7/8/9/10/11/12/13 |
| Layout duplo arquitectural aceite | **N=1 preserved** | |
| **L3-only parent_bbox** | **N=1 → N=2 cumulativo emergente** | P273.10 inaugural + **P273.13 reaplicação**. Padrão consolidado |
| Dedup Arc::as_ptr resources | **N=3 preserved** | Reused via DedupKey lookup |
| Bug arquitectural intencional corrigido | **N=1 preserved** | P273.13 não é arquitectural — fix tactical de pendência específica |
| Bug latent corrigido em scope creep | **N=1 preserved** | |
| Extract helper de replicação inline | **N=1 preserved** | |
| **Triplicação Group bbox** | **N=0 → N=1 emergente** | 3 sítios constroem mesmo Rect (scan + pattern_resources + draw_item_local). Candidato extract helper |
| Diagnóstico imutável | **N=28 → N=29 cumulativo** | Vigésimo quarto consumo |

---

## §4 — Métricas finais

| Métrica | Pré-P273.13 | Pós-P273.13 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2638 | **2644** | +6 |
| Tests P273.13 novos | — | 6 | 6 export L3 integration tests |
| Tests P262-P273.12 (verdes) | preserved | preserved | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Build warnings (novos) | 0 | 0 | 0 |
| Hashes propagados L0 | — | 1 (`gradient.rs:d3924dbf`) | +1 |
| ADRs totais | 81 | **81** | 0 (sem nova ADR; décima terceira anotação ADR-0091) |
| LOC L3 (additions+changes) | — | **~35** | draw_item_local sig+arms ~25 + 3 callsites build_page_stream_* ~10 |
| LOC L1 (additions) | — | 0 | literal (ADR-0029 preserved) |

### §política condições verificadas

- ✓ Cap LOC L3 hard 70 — real ~35; folga 50%.
- ✓ Cap LOC L3 soft 50 — real ~35; folga 30%.
- ✓ Cap LOC L1 hard 0 — real 0; literal.
- ✓ Cap testes hard 10 — real 6; folga 40%.
- ✓ Cap testes soft 6 — real 6; limite respeitado.
- ✓ Defaults preservam pipeline P262-P273.12 bit-exact.
- ✓ Lint zero preserved.
- ✓ Regressão tests P262-P273.12 zero (2638 baseline preserved).
- ✓ ADR-0029 pureza física L1 preserved (P273.13 L3-only).
- ✓ E2E observable: 6 testes export confirmam `/Pattern CS` em PDF
  para gradient inside Group + nested Groups + paridade
  Linear/Radial.
- ✓ **Pendência P263 §8 #3 fechada** — `draw_item_local` consume
  `pat_ptr_to_idx` real via `emit_stroke_paint`.
- ✓ **Pendência P273.12 §9 quarto bullet fechada** — gradient
  dentro de Group renderiza com pattern (não solid fallback).

**12 condições §política verificadas — 12 satisfeitas absolutas**
per ADR-0094 Pattern 1.

---

## §5 — Verificação regressão zero P262-P273.12

**2638 baseline preservado bit-exact**:

- typst-core: 2179 preserved.
- typst-shell: 24 preserved.
- typst-infra: 412 → **418** (+6 P273.13 export tests).
- typst-wiring + bins + outros: 23 preserved.

**Total: 2638 → 2644 (+6 net)**.

**2 testes typst-core skipped** (stack overflow pré-existente) —
NÃO regressão P273.13.

Mecânica:
- Shapes top-level (não dentro de Group) → caminho directo
  `emit_stroke_paint` em build_page_stream_*; sem mudança.
- Shapes dentro de Group sem gradient (solid fill/stroke) → arm
  Shape em draw_item_local preserva path solid fill literal; stroke
  Solid via `emit_stroke_paint` Paint::Solid arm (literal RGB).
- Shapes dentro de Group com gradient previamente fallback solid →
  agora consume pattern via DedupKey lookup. Bytes PDF DIFEREM (mas
  esse é o comportamento esperado — fix do bug).
- Tests P262-P273.12 não dependem de bytes específicos de gradient
  dentro de Group (P273.12 tests verificavam contagem de
  `/ShadingType` — preserved porque scan continua a registar).

---

## §6 — Anotação cumulativa ADR-0091 (décima terceira consecutiva)

Adicionada §"Anotação cumulativa P273.13 — Fix draw_item_local
Group gradient (caminho emit real)" cobrindo:

- Decisão 1α (parameter threading).
- Decisão 2α (Group bbox literal-equivalente).
- Decisão 3α (coords cristalino).
- Scope creep arm Group novo aceito.
- Sub-padrões: L3-only parent_bbox N=2; Triplicação Group bbox N=1;
  Sub-passos consecutivos N=9.
- Defaults preservam P262-P273.12 bit-exact.
- Pendências P263 §8 #3 + P273.12 §9 quarto bullet fechadas.

**Sub-padrão "Anotação cumulativa em vez de ADR nova"** N=19 →
**N=20 cumulativo consolidação clara persistente** — décima terceira
anotação consecutiva ADR-0091.

---

## §7 — L0 `entities/gradient.md` anotação P273.13

Adicionada anotação P273.13 após P273.12 — fecho pendências P263 §8
#3 + P273.12 §9 quarto bullet via parameter threading L3 paralelo
P273.10; sub-padrão "L3-only parent_bbox" N=2 cumulativo. Hash
propagado: `01_core/src/entities/gradient.rs:d3924dbf`.

---

## §8 — Pendências preservadas pós-P273.13

Inalteradas vs P273.12 (nível cluster):

- **P-Gradient-CMYK-ICC** (S-M; **VERIFICAR Fase A se krilla API existe**).
- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

Sequência para fechar cluster Gradient (renumerada pós-inserção
P273.13):

- ✓ P273.10 — Group L3-only (fechado).
- ✓ P273.11 — Extract Stack helper (fechado).
- ✓ P273.12 — Dedup bbox-aware (fechado).
- ✓ **P273.13 — Fix draw_item_local Group gradient (este passo;
  fechado)**.
- **P273.14** — CMYK-ICC krilla paridade (S-M; verificar API; era
  P273.13 pré-inserção).
- **P273.15** — Bbox medido pós-layout (M; era P273.14).
- **P273.16** — Bbox.y topo-exacto inline (M-L; era P273.15;
  **BLOQUEADO** por DEBT-56).

Pendência específica nova candidata XS:
- **P273.X-bis-helper-group-bbox** — extract helper
  `group_bbox_from_frame_item` partilhado entre 3 sítios. Sub-padrão
  "Extract helper de replicação inline" precedente N=2 se
  materializado. NÃO reservado.

**Predição revisada**: cluster Gradient fecha empíricamente entre
P273.14 e P273.16 consoante disponibilidade da krilla API.

---

## §9 — Limitações conscientes P273.13

- 3 sítios constroem `group_bbox` literal-equivalente (sub-padrão
  "Triplicação Group bbox" N=1 emergente). Sensível a divergência
  se um dos 3 sítios for refactored independentemente. Recomendação:
  helper partilhado candidato XS futuro
  (`P273.X-bis-helper-group-bbox`).
- `draw_item_local` continua sem arm Text/Image — texto e imagens
  dentro de Group continuam silenciosamente descartados via `_ => {}`.
  Limitação pré-existente preserved (P273.13 escopo: gradient stroke
  emit apenas).
- `draw_item` top-level e `draw_item_local` Group children mantêm-se
  funções separadas — refactor para unificar fica fora de escopo.
- Cluster específico — não toca outros sítios análogos noutros
  clusters.

---

## §10 — Marco final P273.13

**Pendência cumulativa cluster Gradient fechada definitivamente**:

- L3 `draw_item_local` ganha parameter threading L3-only
  (parent_bbox_override + pat_ptr_to_idx + pat_refs) paralelo
  P273.10.
- L3 arm Shape em `draw_item_local` substitui solid fallback por
  `emit_stroke_paint` com DedupKey lookup.
- L3 arm Group novo em `draw_item_local` (scope creep aceito Fase A
  §A.4): suporta nested Groups + propaga `parent_bbox_override`.
- L3 3 callsites em `build_page_stream_*` Group arms: constroem
  `group_bbox` literal-equivalente + passam.
- L1: 0 LOC (ADR-0029 preserved).
- 6 tests P273.13 + zero regressão P262-P273.12.

Sub-padrão **"L3-only parent_bbox" N=1 → N=2 cumulativo emergente**
— P273.10 inaugural + **P273.13 reaplicação**. Padrão consolidado;
parameter threading L3 walkers recursivos estabelecido como
mecanismo standard.

Sub-padrão **"Triplicação Group bbox" N=0 → N=1 emergente** — 3
sítios constroem mesmo Rect literal. Candidato extract helper
`group_bbox_from_frame_item` (P273.X-bis) per precedente P273.11
"Extract helper de replicação inline" N=1.

Sub-padrão **"Sub-passos consecutivos do mesmo cluster" N=8 → N=9
cumulativo emergente** (P273.5/6/7/8/9/10/11/12/13).

Cluster Gradient feature-complete + qualitativo + refino estrutural
extensivamente encerrado + cleanup intra-cluster + dedup bbox-aware
+ **render real Groups via pattern dict** — próximo passo:
**P273.14** CMYK-ICC krilla paridade (verificar Fase A se krilla
API existe).

---

## §11 — Referências

- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa
  P273.13; décima terceira anotação consecutiva).
- ADR-0093 — Meta-metodologia evolução ADRs (Pattern 2 aplicação
  prática N=9 cumulativo).
- ADR-0094 — Meta-operacional specs (Pattern 1 cap LOC; aplicação
  prática N=12 cumulativo; folga 30% L3 soft).
- ADR-0085 — Diagnóstico imutável (vigésimo quarto consumo).
- ADR-0029 — Pureza física L1 (preserved; P273.13 L3-only).
- ADR-0054 — Critério fecho DEBT-1 (graded — scope creep arm Group
  aceito; sub-padrão Triplicação Group bbox emergente).
- P263 §8 #3 (2026-05-16) — pendência fechada definitivamente.
- P273.10 — Group L3-only scan (sub-padrão inaugural N=1; P273.13
  reaplica → N=2 cumulativo).
- P273.12 §9 quarto bullet — limitação documentada; pendência
  fechada definitivamente.
- P273.11 §X — "Extract helper de replicação inline" N=1 precedente
  para candidato P273.X-bis-helper-group-bbox.
- Spec P273.13 — `00_nucleo/materialization/typst-passo-273-13.md`.
- `00_nucleo/diagnosticos/typst-passo-273-13-diagnostico.md` — Fase
  A empírica + Decisões 1α/2α/3α + scope creep + critério §A.7.

---

*Relatório imutável produzido em 2026-05-18. Pendência P263 §8 #3 +
P273.12 §9 quarto bullet fechadas; sub-padrão "L3-only parent_bbox"
N=1 → N=2 cumulativo emergente consolidado; "Triplicação Group bbox"
N=0 → N=1 emergente candidato extract helper futuro.*
