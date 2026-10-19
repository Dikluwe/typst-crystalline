# Trabalho prévio externo necessário — P273.16 Bbox.y topo-exacto inline (NO-GO output; bloqueador actualizado)

**Data**: 2026-05-18.
**Passo origem**: P273.16 (NO-GO Fase A).
**Status**: documento de **pré-requisitos** para futuro hipotético GO.
**Cluster**: Visualize / Gradient.

---

## §0 — Propósito + actualização empírica da premissa

Este documento é o **output legítimo** da decisão NO-GO em P273.16
(per spec §A.5 + §6 workflow). P273.16 é a **terceira aplicação**
consecutiva do sub-padrão "Scope-out reconfirmado por Fase A" —
sub-padrão atinge N=3 cumulativo crossing limiar formalização ADR
meta N=3-4.

**Actualização empírica importante**: a spec P273.16 §0 declarava
"DEBT-56 EM ABERTO desde 2026-04-25" como bloqueador. **Verificação
literal em `00_nucleo/DEBT.md:535` confirma DEBT-56 ENCERRADO em
Passo 221 (2026-05-12)** — premissa da spec factualmente
desactualizada (6 dias entre fechamento e escrita da spec).

Conclusão NO-GO permanece correcta mas via fundamentos diferentes —
não é DEBT-56 que bloqueia (está fechado), é **P156H limitação
consciente** sobre inline line_height + **ADR-0054 graded
aceitação** + **zero demanda empírica**.

---

## §1 — Pré-requisitos REAIS para reanálise futura GO

### 1.1 — Caso empírico concreto identificado

**Estado actual** (verificado §A.1 diagnóstico):
- **Zero casos** registados em 9 sub-passos consecutivos
  (P273.7-P273.15) onde 3γ.2.γ-inline-baseline-y produziu output
  visualmente insuficiente.
- Zero tests cristalino exercitam expectativas de bbox.y topo-exacto.
- Decisão P273.7 §A.3 — aproximação aceitável per ADR-0054 graded
  — preservada por 9 sub-passos.

**Decisão a tomar** (fora do escopo P273.16):
1. **Aguardar caso real surgir** — utilizador (ou test E2E vanilla
   paridade) reporta output visualmente inaceitável. Não previsto.
2. **Comparação visual vs vanilla** — gerar PDF cristalino + vanilla
   para Boxed inline com gradient `relative=parent`; identificar
   empíricamente divergência visual significativa. **Magnitude**:
   S documental.

### 1.2 — Refactor inline line_height (caminho 1; escopo L+)

**Estado actual** (verificado §A.2 diagnóstico):
- P156H limitação consciente em
  `00_nucleo/prompts/entities/content.md:817-829`: "`width`/`height`
  armazenados mas não impõem limite real" + "`inset.top`/`inset.bottom`
  armazenados mas não aplicados em layout inline (alterariam
  line_height)".
- **DEBT-56 fechado P221** — referência na L0 content.md:824 está
  factualmente desactualizada (candidato cleanup XS
  P273.X-bis-content-md-debt56-update).
- Refino multi-region flow real → **Fase 4 candidata NÃO-reservada**
  per ADR-0078 §"Decisão" sub-fase (b). Scope-out arquitectural
  documentado.

**Decisão a tomar** (fora do escopo P273.16):
1. **Aguardar Fase 4 multi-region** se for materializada (não
   reservada actualmente).
2. **Refactor inline line_height dedicado** — passo dedicado
   (escopo L+; análogo às sub-fases DEBT-56 P216A+B+P217-P220).
3. **Caminho 2 (font_metrics ad-hoc)** com decisão deliberada de
   aceitar dívida invisível — rejeitado por §A.4 critério 2.

**Magnitude estimada**: L+ (escopo análogo a DEBT-56 sub-fases).

### 1.3 — Decisão arquitectural sobre dívida invisível (caminho 2)

**Estado actual** (§A.2 Caminho 2):
- `font_metrics.ascender` no arm Boxed sem refactor disponível
  imediatamente: ~15-25 LOC L1.
- Risco: divergir de quando hipotética Fase 4 multi-region for
  materializada.

**Decisão a tomar** (fora do escopo P273.16):
1. **Aceitar dívida ad-hoc com code-comment explícito** — "remover
   quando refactor inline line_height materializado". Aceitação
   honesta de dívida visível.
2. **Esperar refactor real** §1.2 — caminho preferido per ADR-0054
   graded.

---

## §2 — Critérios para reabrir P273.16 como GO futuro

Reanálise GO viável apenas quando **todos** os 3 itens §1 forem
resolvidos:

1. ✅ Caso empírico concreto identificado (test ou reporte
   utilizador) onde 3γ.2.γ-inline-baseline-y produz output
   visualmente insuficiente.
2. ✅ Decisão sobre caminho de refactor (Fase 4 multi-region OR
   refactor dedicado inline line_height OR aceitação dívida ad-hoc
   visível).
3. ✅ Trabalho prévio §1.2 ou §1.3 cumprido.

Se algum item permanece pendente — NO-GO continua a ser o resultado
correcto.

---

## §3 — Pendência registada permanente

**P273.X-bis2-bbox-y-topo-exacto-inline** permanece como **pendência
aberta cluster Visualize/Gradient** com:

- **Status**: scope-out reconfirmado por Fase A factual P273.16.
- **Pré-requisitos**: §1 acima (3 itens).
- **Reanálise**: quando §2 critérios cumpridos.
- **Decisão P273.7 §A.3 (3γ.2.γ-inline-baseline-y)** preserved
  literal — 9 sub-passos sem contraproba.
- **P156H limitação consciente** preserved literal per ADR-0054
  graded.

**Cluster Gradient pode declarar-se feature-complete** sem este
refino. 3γ.2.γ-inline-baseline-y continua como caminho para Boxed
inline (aproximação aceitável).

---

## §4 — Sub-padrão "Scope-out reconfirmado por Fase A" N=3 cumulativo crossing limiar formalização

P273.16 é a **terceira aplicação** do sub-padrão — atinge limiar
formalização ADR meta N=3-4 com folga consolidada:

**Aplicações cumulativas**:
- **N=1 (P273.14)**: CMYK-ICC scope-out. Razão: constraints externas
  (profile licensing + crate externa + invariante L0 export.md).
- **N=2 (P273.15)**: Bbox medido pós-layout. Razão: constraints
  internas (custo perf O(N²) + ausência demanda).
- **N=3 (P273.16)**: Bbox.y topo-exacto inline. Razão: bloqueador
  estrutural aceito (P156H limitação consciente per ADR-0054 graded)
  + ausência demanda + actualização empírica da premissa da spec
  (DEBT-56 fechado, não bloqueador imediato como spec previa).

**3 razões NO-GO distintas e legítimas** estabelecem padrão
consolidado:
- Externa: licensing, invariante arquitectural.
- Interna: custo perf, demanda empírica.
- Estrutural: bloqueador documentado per graded.

**Limiar formalização ADR meta N=3-4 atingido**. Candidato
meta-ADR formalização NÃO reservado (sub-padrão consolidado a
nível meta-metodológico).

### Comparação P273.14 vs P273.15 vs P273.16

| Aspecto | P273.14 | P273.15 | P273.16 |
|---|---|---|---|
| Bloqueador inicial spec | Profile licensing + invariante L0 | Custo perf + demanda | DEBT-56 (premissa desactualizada) |
| Bloqueador real Fase A | Mesmo (confirmado) | Mesmo (confirmado) | P156H + ADR-0054 graded (actualização empírica) |
| Razão NO-GO | Constraints externas | Constraints internas | Bloqueador estrutural aceito |
| Trabalho prévio externo | 3 pré-requisitos | 2 pré-requisitos | 3 pré-requisitos |
| Outcome sub-padrão | N=1 inaugural | N=2 reaplicação consolidando | N=3 cumulativo crossing limiar |

---

## §5 — Pendência específica nova candidata XS

Descoberta empírica durante P273.16 Fase A: **L0
`content.md:824` referência a DEBT-56 está desactualizada** após
fechamento P221.

**Pendência candidata**:
- **P273.X-bis-content-md-debt56-update** — actualizar L0
  `content.md:824` para referenciar "Fase 4 multi-region scope-out
  per ADR-0078" em vez de "DEBT-56" fechado.
- Magnitude: **XS** (~1 LOC L0).
- Padrão análogo a P273.8 (cleanup 4 warnings) — sub-padrão
  "Cleanup XS derivado" aplicável.
- **NÃO reservado** — candidato futuro se decisão de manter L0
  factualmente alinhada com DEBT state.

---

## §6 — Cluster Gradient declarável feature-complete pós-P273.16

P273.16 fecha a sequência "terminar cluster Gradient" iniciada em
P273.10:

- ✓ **9 sub-passos materializados** (P273.5-P273.13).
- ✓ **3 sub-passos scope-out reconfirmados** (P273.14-P273.16).
- ✓ **12 sub-passos consecutivos** total — caminho mais longo
  documentado no projecto cristalino.
- ✓ **16 anotações cumulativas ADR-0091** (sub-padrão
  consolidação consolidada).
- ✓ **3 sub-padrões emergentes inaugurados N=1** ao longo da
  sequência.
- ✓ **1 sub-padrão N=3 cumulativo crossing limiar formalização**
  ("Scope-out reconfirmado por Fase A").

**Cluster Gradient feature-complete declarável**:
- Cross-variant runtime fields 3/3 (gradient.linear/radial/conic).
- Adaptive N qualitativo (P274).
- Refino estrutural extensivo (Block + Boxed + Grid + Stack + Pad
  + Group).
- Cleanup intra-cluster (P273.8 + P273.11).
- Dedup bbox-aware (P273.12).
- Render real Groups via pattern dict (P273.13).
- 3 scope-outs documentados com trabalho prévio externo
  identificado.

**Próximo passo natural**: sair do cluster Gradient definitivamente.
Pendências restantes do projecto (ADR-0055bis fonts, P-Footnote-N,
DEBT-33 Bézier, Stroke/Curve/Polygon, Tiling, outro cluster)
disponíveis.

---

## §7 — Referências

- Spec P273.16 — `00_nucleo/materialization/typst-passo-273-16.md`.
- Diagnóstico Fase A — `00_nucleo/diagnosticos/typst-passo-273-16-diagnostico.md`.
- ADR-0091 §"Anotação cumulativa P273.7" — decisão original
  3γ.2.γ-inline-baseline-y preserved.
- ADR-0029 — Pureza física L1 (preserved; este passo não toca L1).
- ADR-0054 — Critério fecho DEBT-1 graded (NO-GO output legítimo;
  scope-out reconfirmado).
- ADR-0078 §"Decisão" sub-fase (b) — Opção A multi-region
  documentada como scope-out per P158.
- DEBT.md DEBT-56 — **ENCERRADO P221 (2026-05-12)**; referenciado
  na premissa da spec mas factualmente desactualizado.
- P273.14 — Sub-padrão "Scope-out reconfirmado por Fase A" inaugural
  N=1 (CMYK-ICC; constraints externas).
- P273.15 — Reaplicação N=2 (bbox pós-layout; constraints internas).
- P273.16 — **Esta aplicação N=3 crossing limiar formalização**
  (bloqueador estrutural aceito).
- P273.7 §A.3 — Decisão 1 (3γ.2.γ-inline-baseline-y) preserved.
- P156H limitação consciente em `content.md:817-829` preserved per
  ADR-0054 graded.

---

*Documento imutável produzido em 2026-05-18 como output legítimo
da decisão NO-GO Fase A P273.16. Trabalho prévio externo (3 itens)
identificado; reanálise futura GO viável apenas quando §2 critérios
cumpridos. Premissa da spec actualizada por verificação factual
(DEBT-56 fechado P221, não EM ABERTO). Sub-padrão "Scope-out
reconfirmado por Fase A" N=2 → N=3 cumulativo crossing limiar
formalização ADR meta N=3-4 — terceira aplicação consolida padrão
com 3 razões NO-GO distintas e legítimas.*
