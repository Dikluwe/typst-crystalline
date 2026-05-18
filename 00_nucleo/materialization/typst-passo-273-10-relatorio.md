# Relatório P273.10 — Group L3-only parent_bbox (sub-padrão "L3-only" inaugural)

**Data**: 2026-05-18.
**Magnitude**: S (~75 LOC L3 com scope creep `pattern_resources_for_page`; 0 L1; 7 testes; cap soft L3 50 estourado 50%).
**Cluster**: Visualize / Gradient (primeiro de 6 sub-passos para fechar cluster).
**Tipo**: sub-passo decimal P273.10 — extensão Decisão 3 P273.9 para FrameItem::Group via mecanismo L3 puro.
**Spec**: `00_nucleo/materialization/typst-passo-273-10.md`.

---

## §1 — Sumário executivo

**Sub-padrão "L3-only parent_bbox" N=1 inaugural emergente** —
P273.10 estabelece precedente metodológico para contentores
post-layout cuja bbox é conhecida apenas em L3 emit-time:

- **L3 `scan_all_gradients`** refactor com helper recursivo interno
  `walk(items, parent_bbox_override, ...)` — itera todos os items
  recursivamente (incluindo dentro de `FrameItem::Group`), constrói
  `group_bbox` no arm Group e passa como override; Shape arm aplica
  Inner-wins via `parent_bbox_at_emit.or(parent_bbox_override)`.
- **L3 `pattern_resources_for_page`** refactor symmetric (scope creep
  arquitectural §A.7 do diagnóstico) — bug latent pré-existente onde
  gradients dentro de Groups não eram registados em page resources
  `/Pattern << >>` corrigido em paralelo. Sem isso, P273.10 não
  produz observable behavior.
- **L1**: 0 LOC — ADR-0029 pureza física L1 preserved.

### Marcos arquitecturais P273.10

**(1) Sub-padrão "L3-only parent_bbox" N=1 inaugural emergente** —
distingue de Pattern DEBT-37 (N=4 P273.9; L1 save/restore) e Layout
duplo arquitectural aceite (N=1 P273.9; L1 `measure_content_constrained`).
Mecanismo L3 dispatcher override via parameter threading.

**(2) Bug latent corrigido em scope creep** — `scan_all_gradients` +
`pattern_resources_for_page` não recurse em `FrameItem::Group`
pré-P273.10. Gradients dentro de Groups actualmente não eram
registados → PDF emit quebrado para esse caso. Corrigido em paralelo
sem cascade adicional.

**(3) Sub-padrão "Sub-passos consecutivos do mesmo cluster" N=5 →
N=6 cumulativo emergente** — P273.5/6/7/8/9/10.

### Decisões fixadas Fase A

1. **Decisão 1 (mecanismo override)**: **1α** — parameter threading
   explícito (helper recursivo interno com `parent_bbox_override:
   Option<Rect>`).
2. **Decisão 2 (Group bbox)**: **2α** — geometric exact em coords
   cristalino (`Rect { pos, inner_width, inner_height }`; sem
   Y-inversion).
3. **Decisão 3 (precedence)**: **3α — Inner wins** via
   `parent_bbox_at_emit.or(parent_bbox_override)`.

### Defaults preservam P262-P273.9 bit-exact

- Shapes com `parent_bbox_at_emit` populated (P273.9 5 containers
  Layouter: Block/Boxed/Grid/Stack/Pad) → mantêm o próprio campo;
  override Group ignorado.
- Shapes top-level (não dentro de Group) → `override = None`
  propagado; preserved literal P273.9.
- Self_/None relative ignora override (paridade dispatcher).
- Dedup gradient por Arc preserved (limitação P273.6 §9).
- 2625 baseline P273.9 preserved.

---

## §2 — Diff L3 antes/depois

### §2.1 — `scan_all_gradients` refactor (~60 LOC)

```rust
// 03_infra/src/export.rs

fn scan_all_gradients(doc, first_id) -> (..., Vec<GradientObject>) {
    // P273.10 — helper recursivo: itera items + tratamento
    // FrameItem::Group com `parent_bbox_override: Option<Rect>`
    // (Decisão 1α parameter threading). Inner-wins: Shape's próprio
    // `parent_bbox_at_emit` prevalece sobre `override` via `.or()`.
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
                    // ... register gradient (preserved P273.6) ...
                    // P273.10 — Inner wins (Decisão 3α).
                    let effective_bbox = parent_bbox_at_emit
                        .or(parent_bbox_override);
                    grad_objs.push(GradientObject {
                        kind, function_id, shading_id, pattern_id,
                        parent_bbox_at_emit: effective_bbox,
                    });
                }
                FrameItem::Group { pos, inner_width, inner_height, items, .. } => {
                    // P273.10 — Group bbox L3-only override (Decisão 2α):
                    // geometric exact em coords cristalino.
                    let group_bbox = Rect {
                        x: Pt(pos.x.0),
                        y: Pt(pos.y.0),
                        w: Pt(*inner_width),
                        h: Pt(*inner_height),
                    };
                    walk(items, Some(group_bbox), ...);
                }
                _ => {}
            }
        }
    }
    // ... init state + iterar pages chamando walk(items, None, ...) ...
}
```

### §2.2 — `pattern_resources_for_page` refactor (~15 LOC — scope creep §A.7)

```rust
fn pattern_resources_for_page(page, ptr_to_idx, refs) -> String {
    // P273.10 — helper recursivo symmetric a scan_all_gradients:
    // enumera gradients dentro de Groups para listar em /Pattern << >>.
    fn walk(
        items: &[FrameItem],
        ptr_to_idx: &HashMap<usize, usize>,
        refs:       &[PatternRef],
        entries:    &mut Vec<String>,
        seen:       &mut BTreeSet<usize>,
    ) {
        for item in items {
            match item {
                FrameItem::Shape { stroke: Some(Stroke { paint: Paint::Gradient(g), .. }), .. } => {
                    // ... dedup + push entry (preserved P265) ...
                }
                FrameItem::Group { items, .. } => {
                    walk(items, ptr_to_idx, refs, entries, seen);
                }
                _ => {}
            }
        }
    }
    walk(&page.items, ptr_to_idx, refs, ...);
    // ... format `/Pattern << ... >>` ...
}
```

### §2.3 — L1 inalterado

- 0 LOC L1 — ADR-0029 pureza física preserved.
- Layouter inalterado desde P273.9.
- Dispatcher consumer L3 (linha 1638-1669) inalterado — já consumia
  `parent_bbox_at_emit` correctamente.

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P273.10 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=17 → N=18 cumulativo consolidação clara persistente** | Décima primeira anotação consecutiva ADR-0091 |
| Reutilização literal helpers cross-passos | **N=17 preserved** | Primeiro mecanismo L3-only — não reusa helper L1 |
| Cap LOC hard vs soft explícito | **N=12 → N=13 cumulativo** | L3 hard 60 / soft 50 — real ~75 (estouro soft 50%; estouro hard 25%; per §A.7 scope creep arquitectural assumido). Registado per ADR-0094 Pattern 1 |
| Aplicação meta-ADR (ADR-0093) | **N=6 → N=7 cumulativo** | Pattern 2 |
| Aplicação meta-ADR (ADR-0094) | **N=8 → N=9 cumulativo** | Pattern 1 |
| Pattern DEBT-37 `cell_origin_*` replicado | **N=4 preserved** | P273.10 sem touch Layouter |
| Template-passo replicado literal | **N=2 preserved** | Mecanismo diferente; não template |
| Sub-passos consecutivos do mesmo cluster | **N=5 → N=6 cumulativo emergente** | P273.5/6/7/8/9/10 |
| Layout duplo arquitectural aceite | **N=1 preserved** | P273.10 sem layout duplo |
| **L3-only parent_bbox** | **N=0 → N=1 inaugural emergente** | P273.10 inaugura; mecanismo L3 dispatcher override via parameter threading |
| **Bug latent corrigido em scope creep** | **N=0 → N=1 emergente** | `pattern_resources_for_page` recurse em Groups (paralelo `scan_all_gradients` recurse) — sem ele a feature P273.10 não produz observable behavior |
| Diagnóstico imutável | **N=25 → N=26 cumulativo** | Vigésimo primeiro consumo |

---

## §4 — Métricas finais

| Métrica | Pré-P273.10 | Pós-P273.10 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2625 | **2632** | +7 |
| Tests P273.10 novos | — | 7 | 7 export L3 integration tests |
| Tests P262-P273.9 (verdes) | preserved | preserved | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Build warnings (parent_bbox related) | 0 | 0 | 0 |
| Hashes propagados L0 | — | 1 (`gradient.rs:4117e06b`) | +1 |
| ADRs totais | 81 | **81** | 0 (sem nova ADR; décima primeira anotação ADR-0091) |
| LOC L3 (additions) | — | **~75** | `scan_all_gradients` ~60 + `pattern_resources_for_page` ~15 |
| LOC L1 (additions) | — | 0 | literal (ADR-0029 preserved) |

### §política condições verificadas

- ⚠ Cap LOC L3 hard 60 — real ~75; **estouro hard 25%** registado per
  ADR-0094 Pattern 1 (§A.7 scope creep arquitectural reconhecido
  pós-Fase A; cap recalibrado mas estourou ainda mais).
- ⚠ Cap LOC L3 soft 50 — real ~75; estouro soft 50%.
- ✓ Cap LOC L1 hard 0 — real 0; literal.
- ✓ Cap testes hard 12 — real 7; folga 42%.
- ✓ Cap testes soft 8 — real 7; folga 13%.
- ✓ Defaults preservam pipeline P262-P273.9 bit-exact.
- ✓ Lint zero preserved.
- ✓ Regressão tests P262-P273.9 zero (baseline preserved).
- ✓ ADR-0029 pureza física L1 preserved (P273.10 L3-only).
- ✓ E2E observable: 7 testes export confirmam gradient inside Group
  agora registado + emitido com Inner-wins.

**10 condições §política verificadas — 8 satisfeitas absolutas + 2
estouros caps L3 (soft + hard) registados** per ADR-0094 Pattern 1.

**Análise dos estouros**: Spec §1.6 caps (hard 60 / soft 40) assumiu
`scan_all_gradients` refactor isolado. Fase A §A.7 identificou scope
creep arquitectural (`pattern_resources_for_page` paralelo
necessário) e recalibrou soft para 50. Real ~75 estoura ambos. Razão
empírica: helper recursivo interno é mais verboso que apenas adicionar
param (signature noise + braços `Group` em dois sítios). Aceitação:
sem o refactor completo, feature não é observable.

---

## §5 — Verificação regressão zero P262-P273.9

**2625 baseline preservado bit-exact**:

- typst-core: 2179 preserved (P273.9 +7 layout tests inalterados).
- typst-shell: 24 preserved.
- typst-infra: 399 → **406** (+7 P273.10 export tests).
- typst-wiring + bins + outros: 23 preserved.

**Total: 2625 → 2632 (+7 net)**.

**2 testes typst-core skipped** (stack overflow pré-existente) —
NÃO regressão P273.10.

Mecânica:
- Shapes com `parent_bbox_at_emit` populated (P273.9 5 containers)
  unaffected via Inner-wins (`.or()` retorna early).
- Shapes top-level → `override = None`; preserved P273.9.
- Pre-P273.10 tests com gradient top-level (sem Group) — preserved
  bit-exact via path Group-less.
- Pre-P273.10 tests com Shape dentro de Group via `draw_item_local`
  recursão — preserved bit-exact via Inner-wins.

---

## §6 — Anotação cumulativa ADR-0091 (décima primeira consecutiva)

Adicionada §"Anotação cumulativa P273.10 — Group L3-only parent_bbox"
cobrindo:

- Decisão 1α (parameter threading).
- Decisão 2α (geometric exact bbox).
- Decisão 3α (Inner wins).
- Scope creep arquitectural (`pattern_resources_for_page` recursão).
- Sub-padrão "L3-only parent_bbox" N=1 inaugural — distingue de
  Pattern DEBT-37 e Layout duplo.
- Defaults preservam P262-P273.9 bit-exact.
- 11 sub-padrões aplicados.

**Sub-padrão "Anotação cumulativa em vez de ADR nova"** N=17 →
**N=18 cumulativo consolidação clara persistente** — décima primeira
anotação consecutiva ADR-0091 (P270.x/P273/P274/P273.5/6/7/9/**10**).

---

## §7 — L0 `entities/gradient.md` anotação P273.10

Adicionada anotação P273.10 após P273.9 — extensão para
`FrameItem::Group` via mecanismo L3 puro; Decisões 1α/2α/3α
fixadas; scope creep arquitectural (pattern_resources_for_page)
documentado; sub-padrão "L3-only parent_bbox" N=1 inaugural
emergente; bug latent registo+listing gradient inside Group
corrigido. Hash propagado:
`01_core/src/entities/gradient.rs:4117e06b`.

---

## §8 — Pendências preservadas pós-P273.10

Inalteradas vs P273.9 (nível cluster):

- **P-Gradient-CMYK-ICC** (S-M).
- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

Pendências específicas pós-P273.10 — sequência para fechar cluster
Gradient (per spec §7):

- **P273.11** — Extract Stack measurement helper (XS; cleanup §9
  P273.9).
- **P273.12** — Dedup bbox-aware (S-M; refino arquitectural per
  P273.6 §9).
- **P273.13** — CMYK-ICC krilla paridade (S-M; **VERIFICAR Fase A
  se krilla API existe**).
- **P273.14** — Bbox medido pós-layout (M).
- **P273.15** — Bbox.y topo-exacto inline (M-L; **BLOQUEADO** por
  DEBT-56 refactor multi-region).

**Predição factual**: cluster Gradient fecha empíricamente entre
P273.13 e P273.15 consoante disponibilidade da krilla API.

---

## §9 — Limitações conscientes P273.10

- Group bbox é geometric exact (`inner_width × inner_height`); refino
  para bbox lógico fora de escopo per ADR-0054 graded.
- Inner-wins (Decisão 3α) — Shapes com `parent_bbox_at_emit` populated
  (P273.9 5 containers) ignoram Group override; semântica "contentor
  mais próximo vence" preserved.
- Override só aplicado quando o helper recursivo desce nos children
  — Shapes via outros caminhos (e.g. recursão directa fora do
  contexto Group) preserved literal.
- Estouros caps L3 (hard 25% / soft 50%) — registados per ADR-0094
  Pattern 1; razão empírica: scope creep arquitectural (pattern_resources
  paralelo) + helper recursivo interno verboso vs param simples.
- Dedup bbox-aware (P273.6 §9) preserved aberto — gradients dedup'd
  por Arc; primeira occurrence captura bbox effective. Refino fica
  candidato P273.12.

---

## §10 — Marco final P273.10

**Cluster Gradient — Group L3-only encerrado**:

- L3 `scan_all_gradients` + `pattern_resources_for_page` refactored
  com helpers recursivos.
- L3 dispatcher inalterado desde P273.6.
- L1: 0 LOC (ADR-0029 preserved).
- 7 tests P273.10 + zero regressão P262-P273.9.
- Sub-padrão "L3-only parent_bbox" N=1 inaugural emergente.
- Bug latent (gradients inside Groups não registados) corrigido em
  scope creep paralelo.

Cristalino oferece gradient API user-facing paridade vanilla em:
- Cross-variant runtime fields canónica 3/3 (focal + space + relative).
- Adaptive N multispace refino qualitativo (Linear+Radial).
- Parent bbox callsite real com consumer real em **Block + Boxed +
  Grid cell + Stack + Pad + FrameItem::Group** (P273.6 + P273.7 +
  P273.9 + **P273.10**).

Sub-padrão **"L3-only parent_bbox" N=1 inaugural emergente** —
estabelece precedente metodológico para futuros contentores
post-layout cuja bbox é conhecida apenas em L3 emit-time.

Cluster Gradient feature-complete + qualitativo + refino estrutural
extensivamente encerrado (L1 + L3) — próximo passo natural na
sequência: P273.11 Extract Stack helper.

---

## §11 — Referências

- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa
  P273.10; décima primeira anotação consecutiva).
- ADR-0093 — Meta-metodologia evolução ADRs (Pattern 2 aplicação
  prática N=7 cumulativo).
- ADR-0094 — Meta-operacional specs (Pattern 1 cap LOC; aplicação
  prática N=9 cumulativo; estouros caps L3 registados).
- ADR-0085 — Diagnóstico imutável (vigésimo primeiro consumo).
- ADR-0029 — Pureza física L1 (preserved; P273.10 L3-only).
- ADR-0054 — Critério fecho DEBT-1 (graded — scope creep §A.7
  aceito; estouros caps aceitos).
- P273.9 — Containers estendidos L1 (Block + Boxed + Grid + Stack
  + Pad); pendência `P273.X-bis-group` resolvida em P273.10.
- P273.6 — Cascade ~86 sites + dispatcher consumer L3 (preserved).
- Spec P273.10 — `00_nucleo/materialization/typst-passo-273-10.md`.
- `00_nucleo/diagnosticos/typst-passo-273-10-diagnostico.md` —
  Fase A empírica + Decisões 1α/2α/3α + scope creep §A.7.

---

*Relatório imutável produzido em 2026-05-18. Cluster Gradient
refino estrutural extensivamente encerrado para containers
structural+inline+post-layout canónicos (Block + Boxed + Grid + Stack
+ Pad + FrameItem::Group); sub-padrão "L3-only parent_bbox" N=1
inaugural emergente estabelece precedente metodológico para
contentores post-layout; cluster Gradient prossegue sequência para
fechar (P273.11 → P273.15).*
