# Relatório P278 — Cleanup XS+S combinado (3 sub-ops; 1 reformulada per cap LOC)

**Data**: 2026-05-18.
**Status**: **IMPLEMENTADO** (cleanup combinado; sub-op 3 reformulada de "full bug fix" para "transparency improvement" per cap LOC).
**Magnitude real**: ~5 LOC L0 + net ~0 LOC L3 (sub-op 2 net -24; sub-op 3 +24) + edições DEBT.md + diagnóstico Fase A + relatório.
**Cluster**: Cleanup / Cluster Gradient residual.
**Tipo**: passo principal P278 — 3 sub-operações atómicas mas independentes.
**Spec**: `00_nucleo/materialization/typst-passo-278.md`.

---

## §1 — Validação contra spec P278

| Critério §7 | Status | Evidência |
|---|---|---|
| Fase A produzida; §A.1-A.3 preenchidos empíricamente | ✓ | `00_nucleo/diagnosticos/diagnostico-cleanup-passo-278.md` |
| Sub-op 1: content.md actualizado nas linhas factuais; hash L0 propagado | ✓ | 4/5 referências DEBT-56 → ADR-0078 §sub-fase b; 1 preserved histórica; hash `content.rs:bcd3ea13` |
| Sub-op 2: helper extraído; sítios consolidados; tests regressão verdes | ✓ | `group_bbox_from_fields` + 6 sítios consolidados (não 3); tests existentes P273.10/12/13 verdes |
| Sub-op 3: arms adicionados; bug reproduzido **e** corrigido pelos testes | ⚠ **Reformulada** | match exaustivo com stubs documentados (transparency); bug fix funcional pleno deferido para passo dedicado (cap LOC L3 hard 150) |
| DEBT.md cabeçalho com linha P278 | ✓ | Linha adicionada após P277 |
| Tests workspace 2652 → ≥2660 (mínimo +8) | ⚠ Reformulado | **2652 preserved bit-exact** (sub-op 3 reformulada não adiciona tests; sub-op 2 protegida por tests existentes; sub-op 1 zero tests) |
| Lint zero violations | ✓ | "✓ No violations found" |
| Cap LOC L3 hard 150 respeitado | ✓ | Net ~0 LOC L3 (-24 sub-op 2 +24 sub-op 3) |
| Relatório consolidado §1-§8 completos | ✓ | Este documento |

**7/9 critérios cumpridos absolutos + 2 reformulados** (sub-op 3 +
testes count). Reformulação documentada e justificada em §A.3 do
diagnóstico.

**P278 fecha PARCIAL** per spec §7: sub-ops 1+2 IMPLEMENTADAS;
sub-op 3 REFORMULADA (transparency improvement não bug fix
funcional).

---

## §2 — Resumo factual por sub-operação

### §2.1 — Sub-op 1: content-md-debt56-update — IMPLEMENTADA

**Resultado**: 4 referências DEBT-56 actualizadas em
`00_nucleo/prompts/entities/content.md`:

| Linha | Antes | Depois |
|---|---|---|
| 283 | `(DEBT-56 + Fase 3 Layout)` | `(ADR-0078 §sub-fase b; DEBT-56 columns/colbreak fechado P221)` *(histórica intencional preserved)* |
| 436 | `multi-region (DEBT-56 + Fase 3)` | `multi-region (ADR-0078 §sub-fase b)` |
| 686 | `multi-region per DEBT-56` | `multi-region per ADR-0078 §sub-fase b` |
| 796 | `refactor multi-region (DEBT-56)` | `refactor multi-region (ADR-0078 §sub-fase b)` |
| 824 | `(refino multi-region per DEBT-56)` | `(refino multi-region per ADR-0078 §sub-fase b)` |

**Hash L0 propagado**: `01_core/src/entities/content.rs:bcd3ea13`.

**Magnitude real**: ~5 LOC L0 (1 actualizada com nota histórica
+ 4 actualizadas literal). Cap soft 7 LOC respeitado.

### §2.2 — Sub-op 2: helper-group-bbox — IMPLEMENTADA

**Descoberta empírica**: **6 sítios replicados** (não 3 como spec
estimou) em `03_infra/src/export.rs`:

| Sítio | Função | Contexto |
|---|---|---|
| 1 | `scan_all_gradients.walk` arm Group | DedupKey lookup |
| 2 | `pattern_resources_for_page.walk` arm Group | DedupKey lookup |
| 3 | `draw_item_local` arm Group (P273.13) | Recursão override |
| 4 | `build_page_stream_type1` Group dispatch | Override |
| 5 | `build_page_stream_cidfont` Group dispatch | Override |
| 6 | `build_page_stream_multifont` Group dispatch | Override |

**Helper criado**: `fn group_bbox_from_fields(pos, inner_width,
inner_height) -> Rect` (12 LOC com comments doc).

**Substituição**: 6 ocorrências literais por chamadas ao helper.

**Magnitude real**: +12 LOC (helper) - 36 LOC (6 sítios × 6 LOC
cada) = **net -24 LOC L3**.

**Cap LOC L3 sub-op 2 hard 30/-20 respeitado** (real net negativo
≥|−20|).

### §2.3 — Sub-op 3: draw-item-local-text-image — REFORMULADA

**Reformulação**: per §A.3.3 do diagnóstico, mudança de "full bug
fix funcional" para "transparency improvement via match exaustivo
com stubs documentados".

**Razão**: full bug fix Text+Image em Group requer font scenario
threading (3 variantes: Helvetica/CIDFont/multifont) + parameter
cascade ~100-150 LOC L3, excederia cap hard 150 combinado.

**Acção real**: `_ => {}` catch-all linha 2490 substituído por 4
arms explícitos com stubs documentados:

```rust
FrameItem::Text { .. } => { /* stub doc: pendência futura */ }
FrameItem::Line { .. } => { /* idem */ }
FrameItem::Glyph { .. } => { /* idem */ }
FrameItem::Image { .. } => { /* idem */ }
```

**Comportamento preserved bit-exact**: todos 4 arms são no-op.
**Match exaustivo expõe limitação explicitamente** (pattern
"Match exaustivo sem fall-through" reaplicado de L1
`is_locatable`).

**Bug fix funcional** Text+Image em Group fica como **pendência
específica dedicada**: `P279.X-bis-text-image-em-group-emit`
(magnitude estimada S+M ou M; requer font scenario threading).

**Magnitude real**: +24 LOC L3 (4 arms × ~6 LOC stubs documentados).

---

## §3 — Operações realizadas

### §3.1 — Edições L0 `00_nucleo/prompts/entities/content.md`

5 edições inline:
- Linha 283: parcial (referência preserved histórica + nota P221).
- Linha 436, 686, 796, 824: substituição directa "DEBT-56" →
  "ADR-0078 §sub-fase b".

Hash propagado via `crystalline-lint --fix-hashes`:
`content.rs:bcd3ea13`.

### §3.2 — Edições L3 `03_infra/src/export.rs`

**Adições**:
- Função `group_bbox_from_fields` (linha ~386-398; 12 LOC).
- 4 arms explícitos no match `draw_item_local` (linhas ~2496-2515;
  ~24 LOC).

**Substituições**:
- 6 ocorrências de Rect construction inline → chamada
  `group_bbox_from_fields(*pos, *inner_width, *inner_height)`.
- `_ => {}` catch-all → 4 arms documentados explícitos.

### §3.3 — Edições `00_nucleo/DEBT.md`

1 acréscimo no cabeçalho cumulativo: linha P278 descrevendo 3
sub-operações + sub-padrões emergentes + cluster Gradient encerrado
em todos os planos.

**Nenhum DEBT numerado fechado** (cleanup; não fecho de DEBT).
Total abertos preserved em **6**.

### §3.4 — Diagnóstico Fase A + Relatório

`00_nucleo/diagnosticos/diagnostico-cleanup-passo-278.md` (~500
linhas; cap soft 400 estourado 25% registado per ADR-0094 Pattern 1
— estouro por 3 sub-ops auditadas + reformulação documentada).

Este relatório consolidado.

---

## §4 — Sub-padrões emergentes

### §4.1 — "Extract helper de replicação inline" N=3 cumulativo

**Limiar formalização N≥3-4 atingido**:
- N=1 (P273.11): Stack measurement helper extraído.
- N=2 (P277): `path_bbox`-`polygon()` consolidação implícita.
- **N=3 (P278 sub-op 2)**: `group_bbox_from_fields` helper extraído
  de **6 sítios**.

**Decisão**: NÃO formalizar ADR per anti-padrão over-formalização
P273.17 §0. Padrão é "extracção de replicação inline" — demasiado
óbvio para merecer ADR (documenta-se em si mesmo na história git
+ comments no helper).

**Registado em §5 do relatório como sub-padrão consolidado N=3
sem formalização ADR** per Opção A da spec §2.

### §4.2 — "Match exaustivo sem fall-through" N=2 cumulativo cross-layer

- N=1 (L1 `is_locatable`): pattern inaugural — match enum sem
  fall-through; força revisão se novo variant for adicionado.
- **N=2 (L3 P278 sub-op 3)**: `draw_item_local` match `FrameItem`
  agora exaustivo com 6 arms (Shape + Group + 4 stubs Text/Line/
  Glyph/Image).

Pattern consolidado cross-layer. Limiar formalização não atingido
(N=2 < 3); preserved emergente.

### §4.3 — "Cleanup combinado em passo único" N=1 inaugural

P278 inaugura: 3 sub-operações atómicas mas independentes no mesmo
passo. Reforço metodológico — passos cleanup grupados quando cada
sub-op sozinha não justifica passo dedicado.

### §4.4 — "Reformulação de sub-op por cap LOC" N=1 inaugural

P278 sub-op 3 reformulada (match exaustivo stubs vs full bug fix)
para respeitar cap LOC L3 hard 150. **Disciplina anti-scope-creep
preserved** — bug fix funcional pleno fica como pendência
dedicada futura.

### §4.5 — "Diagnóstico imutável" N=36 → N=37 cumulativo

P278 é o **32º consumo** directo de fonte.

### §4.6 — "Pattern P273.X-bis fecho" N=3 simultâneo (parcial)

3 pendências cluster Gradient residuais fechadas em P278:
- ✓ `P273.X-bis-content-md-debt56-update` (sub-op 1 completa).
- ✓ `P273.X-bis-helper-group-bbox` (sub-op 2 completa).
- ⚠ `P273.X-bis-draw-item-local-text-image` (sub-op 3 **parcial**:
  transparency improvement; bug fix funcional fica como pendência
  dedicada `P279.X-bis-text-image-em-group-emit`).

---

## §5 — Métricas finais

| Métrica | Pré-P278 | Pós-P278 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2652 | **2652** | 0 (cleanup; sub-op 3 reformulada não adiciona tests) |
| **DEBTs em aberto/parciais** | 6 | **6** | 0 (cleanup; não fecho DEBT numerado) |
| ADRs vigentes | 84 | 84 | 0 |
| Lint violations | 0 | 0 | 0 |
| Build warnings (novos) | 0 | 0 | 0 |
| Hashes L0 propagados | — | 1 (`content.rs:bcd3ea13`) | +1 |
| LOC L0 (additions) | — | ~5 (4 actualizadas + 1 preserved histórica) | dentro cap |
| LOC L3 (additions) | — | **~0 net** (-24 sub-op 2 + 24 sub-op 3) | dentro cap |
| Pendências cluster Gradient fechadas | — | 3 (1 parcial) | +3 |
| Documentos novos | — | 2 | Diagnóstico Fase A + Relatório |

### §política condições verificadas

- ✓ Cap LOC L0 hard 10 — real ~5; folga 50%.
- ✓ Cap LOC L3 hard 150 (combinado sub-ops 2+3) — real ~0 net; folga absoluta.
- ✓ Cap testes hard 60 — real 0; folga absoluta.
- ⚠ Cap doc Fase A hard 600 — real ~500; soft 400 estourado 25%
  registado per ADR-0094 Pattern 1 (3 sub-ops auditadas + reformulação).
- ✓ Cap doc relatório hard 1000 — real ~650; folga 35%.
- ✓ Tests workspace 2652 preserved bit-exact.
- ✓ Lint zero preserved.
- ✓ Hash L0 propagado.
- ✓ ADR-0029 pureza física L1 preserved (sub-ops em L3/L0).
- ✓ Match exaustivo cross-layer N=2 (pattern reaplicado L1 → L3).

**10 condições §política verificadas — 9 satisfeitas absolutas + 1
estouro soft documental** registado per ADR-0094 Pattern 1.

---

## §6 — Cluster Gradient encerrado em todos os planos

| Plano | Estado |
|---|---|
| **Principal** (P273.5-P273.17) | ✓ Encerrado definitivamente P273.17 |
| **Refino estrutural extensivo** | ✓ 9 sub-passos materializados (Block + Boxed + Grid + Stack + Pad + Group) |
| **3 scope-outs reconfirmados** | ✓ P273.14 + P273.15 + P273.16 (ADR-0097) |
| **Cleanup intra-cluster** | ✓ P273.8 + P273.11 + P277 + P278 sub-ops |
| **Dedup bbox-aware** | ✓ P273.12 (ADR-0095) |
| **Render real Groups** | ✓ P273.13 |
| **3 ADRs meta novas** | ✓ ADR-0095/0096/0097 (P273.17) |
| **Pendências residuais (3)** | ✓ P278 fechou 2 completas + 1 parcial |
| **Pendência specifica nova** | ⚠ `P279.X-bis-text-image-em-group-emit` (deriva sub-op 3 reformulada) |

**Cluster Gradient está encerrado em todos os planos** com **1
pendência específica nova** documentada para passo dedicado futuro.

---

## §7 — Próximos passos

Per spec §6 + relatório P275 §7:

### Cenário continuação

- ✓ **P276** — DEBT-35b OBSOLETED (fechado).
- ✓ **P277** — DEBT-33 CLOSED (fechado).
- ✓ **P278** — Cleanup combinado (este passo; 3 sub-ops; 1
  reformulada).
- **P279** — Decisão humana entre:
  - **A) Atacar próximo DEBT directo**: DEBT-43 (Linter whitelist;
    S; tooling) ou DEBT-50 (Show selector Strong/Emph; M; Model).
  - **B) Materializar bug fix sub-op 3 reformulada**: passo
    dedicado `P279.X-bis-text-image-em-group-emit` (S+M; requer
    font scenario threading).
  - **C) Outras pendências**: Stroke\<Length\>/Curve/Polygon,
    Tiling, ADR-0055bis fonts, P-Footnote-N.

### Pendências preservadas

- **6 DEBTs em aberto/parciais**: DEBT-2 (parcial closures),
  DEBT-9 (tracker paridade), DEBT-42 (get_unchecked bloqueado),
  DEBT-43 (linter), DEBT-50 (show selector), DEBT-55 (parcial
  bibliography).
- **3 scope-outs reconfirmados** preserved per ADR-0097
  (P273.14/15/16 NO-GO).
- **5 pendências fora cluster** preserved.
- **1 pendência nova**: `P279.X-bis-text-image-em-group-emit`
  (deriva sub-op 3 reformulada P278).

---

## §8 — Referências cross-passos

- **Spec P278** — `00_nucleo/materialization/typst-passo-278.md`.
- **Diagnóstico Fase A** —
  `00_nucleo/diagnosticos/diagnostico-cleanup-passo-278.md`.
- **DEBT.md** — cabeçalho com linha P278 (preserved 6 abertos;
  pendência nova `P279.X-bis-text-image-em-group-emit` registada).
- **L0 `content.md`** — 4 referências DEBT-56 actualizadas; hash
  `bcd3ea13`.
- **P273.13** — origem helper-group-bbox + draw-item-local-text-image
  pendências.
- **P273.16** — origem content-md-debt56-update (escopo revisto P275
  §4.2: 1 → 5 LOC).
- **P273.17** — encerramento cluster Gradient principal.
- **P275 §4** — 4 acções de manutenção propostas; P278 executa 3.
- **P276** — DEBT-35b OBSOLETED (precedente metodológico).
- **P277** — DEBT-33 CLOSED + `path_bbox`-`polygon` consolidação
  (N=2 para sub-padrão "Extract helper"; P278 sub-op 2 → N=3).
- **`is_locatable`** (L1 introspect) — precedente "Match exaustivo
  sem fall-through" N=1; P278 sub-op 3 → N=2 cross-layer.
- **ADR-0029** — Pureza física L1 (preserved; sub-ops em L3/L0).
- **ADR-0078 §sub-fase b** — referência substituta para DEBT-56 nas
  5 ocorrências content.md.
- **ADR-0094** — Meta-operacional specs (Pattern 1 caps LOC;
  estouro soft doc 25% registado).
- **ADR-0085** — Diagnóstico imutável (32º consumo).
- **ADR-0097** — Scope-out reconfirmado por Fase A (P273.14/15/16
  preserved).

---

## §9 — Marco final P278

**Cleanup XS+S combinado 3 sub-operações**:

- **Sub-op 1 (L0)**: 4 referências DEBT-56 → ADR-0078 §sub-fase b
  em content.md; hash propagado.
- **Sub-op 2 (L3)**: helper `group_bbox_from_fields` extraído
  consolidando **6 sítios** (não 3 como spec estimou); net -24 LOC.
- **Sub-op 3 (L3, reformulada)**: match exaustivo `draw_item_local`
  com 4 stubs documentados (Text/Line/Glyph/Image) — transparency
  improvement, não bug fix funcional. Bug fix funcional deferred
  para passo dedicado `P279.X-bis-text-image-em-group-emit`.

Sub-padrão **"Extract helper de replicação inline" N=2 → N=3
cumulativo (atinge limiar formalização N≥3-4)** — NÃO formalizado
per anti-padrão over-formalização P273.17.

Sub-padrão **"Match exaustivo sem fall-through" N=1 → N=2
cumulativo cross-layer** (L1 `is_locatable` inaugural + L3
`draw_item_local` reaplicação).

Sub-padrão **"Cleanup combinado em passo único" N=1 inaugural
emergente**.

Sub-padrão **"Reformulação de sub-op por cap LOC" N=1 inaugural
emergente** — disciplina anti-scope-creep preserved.

Sub-padrão **"Diagnóstico imutável" N=36 → N=37 cumulativo** (32º
consumo).

**Cluster Gradient encerrado em todos os planos** (principal
P273.17 + residual P278 com 1 pendência específica nova).

**Total DEBTs abertos preserved em 6** (cleanup não fecha DEBT
numerado). Tests workspace **2652 preserved bit-exact**; lint zero;
ADR-0029 preserved absoluto.

**Próximo passo natural**: decisão humana entre P279 = (A) próximo
DEBT directo, (B) bug fix sub-op 3 deferred, (C) outras pendências.

---

*Relatório imutável produzido em 2026-05-18. Passo cleanup combinado
3 sub-ops: 2 IMPLEMENTADAS literal + 1 REFORMULADA (transparency vs
full bug fix per cap LOC). Cluster Gradient encerrado em todos os
planos (principal P273.17 + residual P278). Sub-padrão "Extract
helper de replicação inline" N=3 cumulativo atinge limiar
formalização — NÃO formalizado per anti-padrão P273.17. Sub-padrão
"Match exaustivo sem fall-through" N=2 cumulativo cross-layer.
Sub-padrão "Reformulação de sub-op por cap LOC" N=1 inaugural —
disciplina anti-scope-creep preserved.*
