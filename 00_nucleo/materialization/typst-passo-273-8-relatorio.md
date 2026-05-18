# Relatório P273.8 — Cleanup `unused_variable: parent_bbox_at_emit` em export.rs

**Data**: 2026-05-18.
**Magnitude**: XS literal (~4 LOC L3; 0 tests; ~5 min execução).
**Cluster**: Visualize / Gradient (cleanup pós-cluster).
**Tipo**: sub-passo decimal de segundo nível P273.8 — cleanup residual P273.6 cascade.
**Spec**: `00_nucleo/materialization/typst-passo-273-8.md`.

---

## §1 — Sumário executivo

**4 warnings `unused_variable: parent_bbox_at_emit` em
`03_infra/src/export.rs` (linhas 2003/2275/2521/2705) eliminados** via
substituição literal Opção α (`parent_bbox_at_emit: _`) nos 4 PDF
emit-paths que não consomem o campo. Build output limpo
(no que respeita ao cluster Gradient); cluster Gradient agora pronto
para saída definitiva sem warnings residuais do cascade P273.6.

### Marcos arquitecturais P273.8

**(1) Cluster Gradient — build output limpo** para warnings de cascade
P273.6 cleanup completo.

**(2) Sub-padrão "P\<X\>.\<Y\>.1 = cleanup XS derivado" considerado
mas ABANDONADO** — proposto inicialmente como inaugural (N=1) mas
revertido por decisão do utilizador (rename P273.7.1 → P273.8;
numeração consecutiva preferida). A reserva original P273.8 para
Stack/Pad/Group/Grid cell save/restore desloca-se para **P273.9**.
Sub-padrão preserved **N=0** (não inaugurado).

**(3) Sub-padrão "Sub-passos consecutivos do mesmo cluster"
N=3 → N=4 cumulativo** — P273.5 + P273.6 + P273.7 + **P273.8**.
Atinge limiar formalização N=3-4 com folga; candidato meta-ADR
futura permanece NÃO reservado (cleanup XS é caso degenerado).

### Decisões fixadas Fase A

- **Decisão 1 (forma cleanup)**: **Opção α** — `parent_bbox_at_emit: _`
  no pattern. Idiomático Rust; robusto a refactors; alinhado com
  sugestão `rustc`.
- **Decisão 2 (consumidores esquecidos)**: **0 sítios afectados** —
  análise §A.4 confirma todos 4 são PDF emit-paths cujo gradient
  paint é resolvido via `emit_stroke_paint` consultando
  `pat_ptr_to_idx` pré-computado por `scan_all_gradients`. Zero DEBT
  separado aberto.

### Defaults preservam P262-P273.7 bit-exact

- 4 substituições puramente declarativas — bytes PDF idênticos
  bit-exact.
- 2620 tests workspace preserved literal.
- Lint zero preserved.

---

## §2 — Diff L3 antes/depois

### §2.1 — `03_infra/src/export.rs` — 4 substituições idênticas

```diff
- FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit } => {
+ FrameItem::Shape { pos, kind, width, height, fill, stroke, parent_bbox_at_emit: _ } => {
```

Aplicado nas 4 linhas:
- **2003** — `draw_item` (page-level, Y-inversion).
- **2275** — `draw_item_local` (Group children, local-coord).
- **2521** — `draw_item` variante (Y-inversion + `emit_stroke_paint`).
- **2705** — `draw_item` variante 3 (idêntica a 2521).

**Total**: 4 LOC alterados (cap L3 hard 8 / soft 4 — limite respeitado).

### §2.2 — Nada mais alterado

- **L1**: 0 LOC (export.rs é L3 puro).
- **L0**: 0 LOC (export.rs não tem L0 directo).
- **Tests**: 0 novos (cleanup cosmético; warning-only).
- **ADRs**: 0 novas / 0 anotadas (cleanup XS sem decisão arquitectural;
  §2 spec explícito).

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P273.8 | Nota |
|---|---|---|
| Cap LOC hard vs soft explícito | **N=10 → N=11 cumulativo** | L3 hard 8 / soft 4; real 4 LOC (cap soft respeitado limite) |
| Diagnóstico imutável | **N=23 → N=24 cumulativo** | Décimo nono consumo directo de fonte |
| Aplicação meta-ADR (ADR-0094) | **N=6 → N=7 cumulativo** | Pattern 1 cap LOC; sétima aplicação prática |
| **Sub-passos consecutivos do mesmo cluster** | **N=3 → N=4 cumulativo** | P273.5 + P273.6 + P273.7 + **P273.8**. Atinge limiar formalização N=3-4 com folga |
| **P\<X\>.\<Y\>.1 = cleanup XS derivado** | **N=0 preserved** (considerado mas abandonado) | Proposto inicialmente como inaugural; revertido com rename P273.7.1 → P273.8 (numeração consecutiva preferida). Reserva original P273.8 (Stack/Pad/Group/Grid) deslocada para P273.9 |

### Sub-padrões preserved (não aplicados — anotados por contraste)

- **Pattern DEBT-37 `cell_origin_*` replicado**: P273.8 não toca
  padrão Layouter save/restore; **N=3 preserved**.
- **Template-passo replicado literal**: P273.8 não replica template
  P273.6/P273.7; **N=1 preserved**.
- **Reutilização literal helpers cross-passos**: P273.8 não reusa
  helper; **N=16 preserved**.
- **Anotação cumulativa em vez de ADR nova**: P273.8 não anota ADR
  (§2 spec explícito); **N=16 preserved**.
- **Cascade pattern-match cross-FrameItem**: P273.8 não cresce
  cascade; **N=2 preserved**.

---

## §4 — Métricas finais

| Métrica | Pré-P273.8 | Pós-P273.8 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2620 | **2620** | 0 (cleanup cosmético) |
| Tests P262-P273.7 (verdes) | preserved | preserved | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Build warnings `unused variable: parent_bbox_at_emit` | **4** | **0** | **-4 (eliminados)** |
| Build warnings totais | 9 (5 pré-existentes Passo 271 + 4 cluster Gradient) | **5** (5 pré-existentes Passo 271 apenas) | -4 |
| Hashes propagados L0 | — | 0 | 0 (export.rs L3 puro) |
| ADRs totais | 81 | **81** | 0 (sem anotação per §2 spec) |
| LOC L3 (additions) | — | **4** | cap hard 8 (folga 50%); cap soft 4 limite respeitado |
| LOC L1 (additions) | — | 0 | literal |

### §política condições verificadas

- ✓ Cap LOC L3 hard 8 — real 4; folga 50%.
- ✓ Cap LOC L3 soft 4 — real 4; limite respeitado.
- ✓ Cap LOC L1 — literal 0 (cleanup L3 puro).
- ✓ Cap testes hard 0 — real 0; literal.
- ✓ Defaults preservam pipeline P262-P273.7 bit-exact (cleanup é
  declarativo; bytes idênticos).
- ✓ Lint zero preserved.
- ✓ Regressão tests P262-P273.7 zero (2620 baseline preserved
  bit-exact).
- ✓ **4 warnings cluster Gradient eliminados** — `cargo build` confirma
  empíricamente.
- ✓ **0 warnings novos introduzidos** — comparação `git stash` pré vs
  pós-patch confirma os 5 warnings remanescentes são pré-existentes
  ao baseline Passo 271 (3 unreachable pattern + 1 unused import +
  1 mutable).
- ✓ ADR-0029 pureza física L1 preserved (L3 only).

**10 condições §política verificadas — 10 satisfeitas absolutas** per
ADR-0094 Pattern 1.

---

## §5 — Verificação regressão zero P262-P273.7

**2620 baseline preservado bit-exact**:

- typst-core: 2174 preserved (P273.7 +5 layout tests inalterados;
  2 testes skip pré-existentes overflow).
- typst-shell: 24 preserved.
- typst-infra: 399 preserved (P273.7 +3 export tests inalterados).
- typst-wiring + bins + outros: 23 preserved.

**Total: 2620 preserved net** (cleanup XS é cosmético; bytes PDF
idênticos).

Mecânica: substituição `parent_bbox_at_emit` → `parent_bbox_at_emit: _`
é puramente declarativa em destructure pattern — runtime emit ops
idênticas bit-exact.

§política condições "Regressão tests P262-P273.7 zero" + "Defaults
preservam pipeline" satisfeitas absolutas.

---

## §6 — Anotação ADR: zero (§2 spec)

P273.8 NÃO anota ADR — cleanup XS sem decisão arquitectural;
ADR-0091 nona anotação (P273.7) permanece a anotação final do cluster
Gradient.

§política condição "Cluster Gradient ADR-0091 anotação literal"
preservada absoluta.

---

## §7 — Pendências preservadas pós-P273.8

Inalteradas vs P273.7 (nível cluster):

- **P-Gradient-CMYK-ICC** (S-M; krilla paridade ICC profiles).
- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

Pendências específicas pós-P273.8 (reservas P273.7 deslocadas):
- **P273.9** — Stack/Pad/Group/Grid cell save/restore (era P273.8;
  deslocado pelo rename).
- **P273.X-bis** — Bbox medido pós-layout.
- **P273.X-bis2** — Bbox.y topo-exacto inline.
- **Dedup bbox-aware**.

**Pós-P273.8 cluster Gradient totalmente pronto para saída
definitiva**: feature-complete user-facing + adaptive N qualitativo +
refino estrutural Block+Boxed + **build output limpo no que respeita
ao cluster Gradient**.

---

## §8 — Limitações conscientes P273.8

- 5 warnings pré-existentes ao baseline Passo 271 permanecem
  (`unused import: Paint`; 3 × `unreachable pattern`; 1 × `mutable`).
  Não são responsabilidade P273.8 — escopo restrito aos 4 warnings
  introduzidos pelo cascade P273.6 do cluster Gradient.
- Decisão α puramente declarativa — comportamento runtime idêntico
  bit-exact ao binding sem uso. Cleanup é "build output hygiene".
- Se um dos 4 sítios for futuramente necessitado para consumir o
  campo (e.g. refino bbox-aware dedup), o pattern volta facilmente
  para `parent_bbox_at_emit` (semântica recuperada com diff XS).

---

## §9 — Marco final P273.8

**Cluster Gradient build output limpo definitivamente**:

- 4 warnings `unused_variable: parent_bbox_at_emit` eliminados via
  Opção α uniforme (`_` em pattern).
- 0 testes novos; 0 ADRs anotadas.
- 0 regressões; 2620 verdes preserved bit-exact.
- Cap LOC L3 soft 4 respeitado literal.

Sub-padrão "P\<X\>.\<Y\>.1 = cleanup XS derivado" considerado
inicialmente como inaugural mas **abandonado** com o rename
P273.7.1 → P273.8 (decisão utilizador: numeração consecutiva
preferida; reserva original P273.8 para Stack/Pad/Group/Grid
deslocada para P273.9). Sub-padrão preserved **N=0**.

Sub-padrão **"Sub-passos consecutivos do mesmo cluster"
N=4 cumulativo** (P273.5 + P273.6 + P273.7 + P273.8) atinge
limiar formalização N=3-4 com folga.

Cluster Gradient feature-complete + qualitativo + refino estrutural
encerrado + **build output limpo** — pronto para saída definitiva
sem dívida cosmética residual.

---

## §10 — Referências

- ADR-0094 — Meta-operacional specs (Pattern 1 cap LOC; aplicação
  prática N=7 cumulativo).
- ADR-0085 — Diagnóstico imutável (décimo nono consumo).
- ADR-0029 — Pureza física L1 (preserved; cleanup L3 only).
- ADR-0054 — Critério fecho DEBT-1 (graded — Pattern 1 cap LOC).
- `00_nucleo/diagnosticos/typst-passo-273-8A-diagnostico.md` — Fase A
  empírica + Decisões 1+2 fixadas + critério §A.6.
- P273.6 — Cascade ~86 sites bulk-patched (origem dos 4 warnings
  residuais eliminados aqui).
- P273.7 — Boxed save/restore (§9 documentou os 4 warnings como
  candidato cleanup XS futuro — agora executado).
- Spec P273.8 — `00_nucleo/materialization/typst-passo-273-8.md`.

---

*Relatório imutável produzido em 2026-05-18. Cleanup XS literal
executado per spec; cluster Gradient build output limpo; sub-padrão
"P\<X\>.\<Y\>.1 = cleanup XS derivado" N=1 emergente inaugural
estabelece precedente metodológico futuro.*
