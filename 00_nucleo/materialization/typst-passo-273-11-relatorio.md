# Relatório P273.11 — Extract Stack measurement helper (cleanup §9 P273.9)

**Data**: 2026-05-18.
**Magnitude**: XS literal (~35 LOC adicionados em helper; ~49 LOC removidos em 2 sítios; **net -14 LOC**).
**Cluster**: Visualize / Gradient (segundo de 6 sub-passos para fechar cluster).
**Tipo**: cleanup XS sem decisão arquitectural — extract helper standard.
**Spec**: `00_nucleo/materialization/typst-passo-273-11.md`.

---

## §1 — Sumário executivo

**Replicação inline P273.9 §2.2 eliminada** via extracção do helper
`Layouter::measure_stack`. Lógica unificada partilhada entre os 2
sítios (handler Stack em `measure_content_constrained` + save/restore
`parent_bbox` em arm Stack `layout_content`). LOC net negativo (-14
LOC); zero regressões; tests bit-exact preserved.

### Marcos arquitecturais P273.11

**(1) Sub-padrão emergente "Extract helper de replicação inline"
N=1 inaugural** — P273.11 estabelece precedente para cleanup
explícito quando replicação inline é detectada em sub-passo
subsequente (acordo no momento de criar — tratado como refino
posterior).

**(2) Sub-padrão "Sub-passos consecutivos do mesmo cluster"
N=6 → N=7 cumulativo emergente** — P273.5/6/7/8/9/10/11.

**(3) Cluster Gradient cleanup intra-cluster encerrado** — pendência
§9 P273.9 segundo bullet resolvida; código reduzido líquido.

### Decisões fixadas Fase A

1. **Decisão 1 (forma helper)**: **1β — método em Layouter**. Ambos
   callers são métodos `&self`; reusa `measure_content_constrained`
   recursivo.
2. **Decisão 2 (`max_w` source)**: **parâmetro explícito** — preserva
   contrato existente do Sítio 1 P156I; Sítio 2 passa
   `self.available_width()` como param.

### Defaults preservam P262-P273.10 bit-exact

- Lógica do helper bit-exact equivalente às 2 replicações inline
  (Fase A §A.2 verificação 8 linhas).
- Tests P273.9 Stack (`p273_9_stack_*`) preserved.
- Tests P156I `measure_content_constrained` Stack handler preserved.
- 2632 baseline P273.10 preserved.

---

## §2 — Diff L1 antes/depois

### §2.1 — Helper novo: `Layouter::measure_stack` (~35 LOC)

```rust
// 01_core/src/rules/layout/mod.rs — antes de measure_content_constrained

impl Layouter {
    /// P273.11 — Mede um Stack (children + dir + spacing) com `max_w`.
    /// Helper extraído da replicação inline P273.9 §2.2 (cleanup §9 P273.9).
    /// Decisão 1β Fase A: método em Layouter (reutiliza
    /// `measure_content_constrained` via `&self`).
    pub(super) fn measure_stack(
        &self,
        children: &[Content],
        dir: crate::entities::dir::Dir,
        spacing: Option<crate::entities::layout_types::Length>,
        max_w: f64,
    ) -> (f64, f64) {
        let n = children.len();
        if n == 0 { return (0.0, 0.0); }
        let space_pt = spacing.map_or(0.0, |l| l.resolve_pt(self.font_size_pt.val()));
        if dir.is_vertical() {
            let mut max_child_w = 0.0_f64;
            let mut sum_h = 0.0_f64;
            for child in children.iter() {
                let (w, h) = self.measure_content_constrained(child, max_w);
                max_child_w = max_child_w.max(w);
                sum_h += h;
            }
            (max_child_w, sum_h + ((n - 1) as f64) * space_pt)
        } else {
            let mut sum_w = 0.0_f64;
            let mut max_child_h = 0.0_f64;
            for child in children.iter() {
                let (w, h) = self.measure_content_constrained(child, max_w);
                sum_w += w;
                max_child_h = max_child_h.max(h);
            }
            (sum_w + ((n - 1) as f64) * space_pt, max_child_h)
        }
    }
}
```

### §2.2 — Sítio 2 (P273.9 inline replication) substituído

```diff
- let (stack_w, stack_h) = if dir.is_vertical() {
-     let mut max_w = 0.0_f64;
-     let mut sum_h = 0.0_f64;
-     for child in children.iter() {
-         let (w, h) = self.measure_content_constrained(child, stack_avail_w);
-         max_w = max_w.max(w);
-         sum_h += h;
-     }
-     (max_w, sum_h + ((n - 1) as f64) * space_pt)
- } else {
-     let mut sum_w = 0.0_f64;
-     let mut max_h = 0.0_f64;
-     for child in children.iter() {
-         let (w, h) = self.measure_content_constrained(child, stack_avail_w);
-         sum_w += w;
-         max_h = max_h.max(h);
-     }
-     (sum_w + ((n - 1) as f64) * space_pt, max_h)
- };
+ let (stack_w, stack_h) =
+     self.measure_stack(children, *dir, *spacing, stack_avail_w);
```

LOC removidos: ~22; adicionados: ~2. Net Sítio 2: **-20 LOC**.

### §2.3 — Sítio 1 (P156I Stack arm em `measure_content_constrained`) substituído

```diff
- Content::Stack { children, dir, spacing } => {
-     let font = self.font_size_pt.val();
-     let space_pt = spacing.map_or(0.0, |l| l.resolve_pt(font));
-     let n = children.len();
-     if n == 0 { return (0.0, 0.0); }
-
-     if dir.is_vertical() {
-         let mut max_w = 0.0_f64;
-         let mut sum_h = 0.0_f64;
-         for child in children.iter() {
-             let (w, h) = self.measure_content_constrained(child, max_width);
-             max_w = max_w.max(w);
-             sum_h += h;
-         }
-         let total_h = sum_h + ((n - 1) as f64) * space_pt;
-         (max_w, total_h)
-     } else {
-         let mut sum_w = 0.0_f64;
-         let mut max_h = 0.0_f64;
-         for child in children.iter() {
-             let (w, h) = self.measure_content_constrained(child, max_width);
-             sum_w += w;
-             max_h = max_h.max(h);
-         }
-         let total_w = sum_w + ((n - 1) as f64) * space_pt;
-         (total_w, max_h)
-     }
- }
+ Content::Stack { children, dir, spacing } => {
+     self.measure_stack(children, *dir, *spacing, max_width)
+ }
```

LOC removidos: ~27; adicionados: ~3. Net Sítio 1: **-24 LOC**.

### §2.4 — Net total

| Mudança | LOC |
|---|---|
| Helper `measure_stack` adicionado | +30 |
| Sítio 2 inline → call | -20 |
| Sítio 1 inline → call | -24 |
| **Net L1** | **-14** |

Cap soft `≤ 10 LOC net adicionado` — **respeitado com folga** (real
net é -14, ou seja 14 LOC líquidos removidos).

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P273.11 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=18 preserved** | Sem anotação (cleanup XS sem decisão arquitectural) |
| Reutilização literal helpers cross-passos | **N=17 preserved** | `measure_stack` é novo helper interno; não reuso cross-passo |
| Cap LOC hard vs soft explícito | **N=13 → N=14 cumulativo** | L1 net -14 (cap soft 10 net adicionado respeitado com folga) |
| Aplicação meta-ADR (ADR-0094) | **N=9 → N=10 cumulativo** | Pattern 1 |
| Pattern DEBT-37 `cell_origin_*` replicado | **N=4 preserved** | P273.11 sem touch DEBT-37 |
| Template-passo replicado literal | **N=2 preserved** | Mecanismo cleanup; não template |
| Sub-passos consecutivos do mesmo cluster | **N=6 → N=7 cumulativo emergente** | P273.5/6/7/8/9/10/11 |
| Layout duplo arquitectural aceite | **N=1 preserved** | Helper apenas extracção; mecanismo P273.9 inalterado |
| L3-only parent_bbox | **N=1 preserved** | P273.11 sem touch L3 |
| **Extract helper de replicação inline** | **N=0 → N=1 inaugural emergente** | P273.11 inaugura: cleanup explícito de replicação inline em sub-passo subsequente |
| Diagnóstico imutável | **N=26 → N=27 cumulativo** | Vigésimo segundo consumo |

---

## §4 — Métricas finais

| Métrica | Pré-P273.11 | Pós-P273.11 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2632 | **2632** | 0 (cleanup mecânico preserved bit-exact) |
| Tests P262-P273.10 (verdes) | preserved | preserved | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Build warnings (novos) | 0 | 0 | 0 |
| Hashes propagados L0 | — | 0 | 0 (refactor L1 puro; sem mudança L0) |
| ADRs totais | 81 | **81** | 0 (sem nova ADR; sem anotação per §2 spec) |
| LOC L1 (delta net) | — | **-14** | Helper +30 / Sítio 2 -20 / Sítio 1 -24 |
| LOC L3 (additions) | — | 0 | literal |

### §política condições verificadas

- ✓ Cap LOC L1 hard 15 net — real -14 (negativo, respeitado).
- ✓ Cap LOC L1 soft 10 net — real -14 (negativo, respeitado com folga).
- ✓ Cap LOC L3 hard 0 — real 0; literal.
- ✓ Cap testes hard 0 — real 0; literal.
- ✓ Defaults preservam pipeline P262-P273.10 bit-exact.
- ✓ Lint zero preserved.
- ✓ Regressão tests P262-P273.10 zero (2632 baseline preserved).
- ✓ Tests P273.9 Stack inalterados bit-exact (`p273_9_stack_*`).
- ✓ Tests P156I `measure_content_constrained` Stack handler
  inalterados bit-exact (verificados via 2179 typst-core preserved).
- ✓ ADR-0029 pureza física L1 preserved.

**10 condições §política verificadas — 10 satisfeitas absolutas** per
ADR-0094 Pattern 1.

---

## §5 — Verificação regressão zero P262-P273.10

**2632 baseline preservado bit-exact**:

- typst-core: 2179 preserved (5 P273.9 layout tests + 7 P273.7 tests
  + 2167 anteriores; Stack handlers via helper inalterados).
- typst-infra: 406 preserved (P273.10 +7 export tests + 399 anteriores).
- typst-shell: 24 preserved.
- typst-wiring + bins + outros: 23 preserved.

**Total: 2632 → 2632 preserved** (cleanup XS é cosmético; bytes PDF
idênticos).

Mecânica: helper `measure_stack` produz output `(f64, f64)` bit-exact
equivalente à replicação inline (Fase A §A.2 tabela 8 linhas
confirmou equivalência). Substituição mecânica preserva semântica.

---

## §6 — Anotação ADR: zero (§2 spec)

P273.11 NÃO anota ADR — cleanup XS sem decisão arquitectural; ADR-0091
décima primeira anotação (P273.10) permanece a anotação corrente do
cluster Gradient.

§política condição "Cluster Gradient ADR-0091 anotação literal"
preserved absoluta.

---

## §7 — Pendências preservadas pós-P273.11

Inalteradas vs P273.10 (nível cluster):

- **P-Gradient-CMYK-ICC** (S-M).
- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

Sequência para fechar cluster Gradient inalterada:

- **P273.12** — Dedup bbox-aware (S-M; refino arquitectural per
  P273.6 §9).
- **P273.13** — CMYK-ICC krilla paridade (S-M; verificar API).
- **P273.14** — Bbox medido pós-layout (M).
- **P273.15** — Bbox.y topo-exacto inline (M-L; **BLOQUEADO** por
  DEBT-56).

---

## §8 — Limitações conscientes P273.11

- Helper é método de `Layouter` (Decisão 1β) — não é "puro" funcional
  estrito. Refino para função livre (1α) candidato XS futuro se houver
  demanda de testabilidade isolada.
- Refactor é mecânico; preserva semântica bit-exact. Sem refino de
  comportamento.
- Cleanup intra-cluster Gradient — não toca outros sítios análogos
  noutros clusters. Pendências `Extract helper` em outros sub-passos
  preservadas como candidatos NÃO reservados.
- Sub-padrão emergente "Extract helper de replicação inline" N=1
  longe do limiar formalização N=3-4.

---

## §9 — Marco final P273.11

**Cluster Gradient cleanup intra-cluster encerrado**:

- L1 helper `Layouter::measure_stack` extraído + 2 sítios delegando.
- Net -14 LOC L1 (cap soft 10 respeitado com folga negativa).
- 0 testes novos; 0 ADRs anotadas.
- 0 regressões; 2632 verdes preserved bit-exact.

Sub-padrão emergente inaugural **"Extract helper de replicação inline"
N=1** — estabelece precedente para cleanup explícito quando
replicação inline é detectada em sub-passo subsequente. Acordo no
momento de criar; tratado como refino posterior.

Sub-padrão **"Sub-passos consecutivos do mesmo cluster" N=6 → N=7
cumulativo emergente** (P273.5/6/7/8/9/10/11) atinge consolidação
máxima preservada — limiar formalização N=3-4 atingido com folga
extensiva.

Cluster Gradient feature-complete + qualitativo + refino estrutural
extensivamente encerrado + **cleanup intra-cluster encerrado** —
próximo passo na sequência: **P273.12** Dedup bbox-aware.

---

## §10 — Referências

- ADR-0094 — Meta-operacional specs (Pattern 1 cap LOC; aplicação
  prática N=10 cumulativo).
- ADR-0085 — Diagnóstico imutável (vigésimo segundo consumo).
- ADR-0029 — Pureza física L1 (preserved; refactor L1 puro).
- ADR-0054 — Critério fecho DEBT-1 (graded — cleanup XS aceito; sem
  decisão arquitectural).
- P273.9 §9 segundo bullet — pendência "Stack measurement helper
  extracted" resolvida.
- P273.9 §2.2 — replicação inline removida.
- P156I — handler Stack em `measure_content_constrained` original
  (refactored para delegar ao helper).
- `00_nucleo/diagnosticos/typst-passo-273-11-diagnostico.md` — Fase
  A empírica + Decisões 1β + 2 fixadas + critério §A.6.
- Spec P273.11 — `00_nucleo/materialization/typst-passo-273-11.md`.

---

*Relatório imutável produzido em 2026-05-18. Cleanup XS literal:
helper `Layouter::measure_stack` extraído; 2 sítios delegando;
net -14 LOC; zero regressões; sub-padrão "Extract helper de
replicação inline" N=1 inaugural emergente.*
