# Relatório P273.16 — Bbox.y topo-exacto inline (SCOPE-OUT-RECONFIRMED via NO-GO Fase A; terceira aplicação crossing limiar formalização N=3-4)

**Data**: 2026-05-18.
**Status**: **SCOPE-OUT-RECONFIRMED** (NO-GO outcome legítimo — não falha; não IMPLEMENTADO).
**Magnitude real**: zero LOC; 0 tests novos; documentação Fase A + trabalho prévio externo.
**Cluster**: Visualize / Gradient (**sétimo e último** sub-passo na sequência terminar cluster — escopo máximo decidido).
**Tipo**: Fase A com decisão go/no-go binária; outcome NO-GO confirmado empíricamente; **terceira aplicação consolidando sub-padrão crossing limiar formalização ADR meta N=3-4**.
**Spec**: `00_nucleo/materialization/typst-passo-273-16.md`.

---

## §1 — Sumário executivo

**Pendência P273.X-bis2-bbox-y-topo-exacto-inline reconfirmada como
scope-out preserved** via decisão NO-GO Fase A factual. Conclusão
correcta mas via **fundamentos diferentes da premissa da spec**:

### Descoberta empírica importante

A spec P273.16 §0 declarava "DEBT-56 EM ABERTO desde 2026-04-25"
como bloqueador. **Verificação literal em `00_nucleo/DEBT.md:535`**
confirma DEBT-56 **ENCERRADO em Passo 221 (2026-05-12)** — 6 dias
antes da spec ser escrita. Premissa factualmente desactualizada.

**Fase A empírica prevalece sobre premissa da spec** — cumprimento
honesto do critério "verificar empíricamente" registado em todos os
relatórios anteriores P273.7-P273.15.

### Razão concreta NO-GO (quádrupla, com fundamentos actualizados)

1. **Zero demanda empírica** em 9 sub-passos consecutivos
   (P273.7-P273.15) sem caso onde 3γ.2.γ-inline-baseline-y produziu
   output visualmente insuficiente.
2. **Caminho 1 (refactor inline line_height) fora do escopo
   P273.16** — magnitude L+ vs S-M cluster Gradient. Fase 4
   multi-region per ADR-0078 §"Decisão" sub-fase (b) scope-out
   documentado.
3. **Caminho 2 (font_metrics.ascender ad-hoc) cria dívida
   invisível** sem demanda — over-engineering per ADR-0054 graded.
4. **3γ.2.γ-inline-baseline-y P273.7 aceito por ADR-0054 graded** +
   coerente com P156H limitação consciente preserved literal.

### Bloqueador real identificado

- **P156H limitação consciente** em `00_nucleo/prompts/entities/content.md:817-829`:
  "`inset.top`/`inset.bottom` armazenados mas não aplicados em
  layout inline (alterariam line_height)".
- **ADR-0078 §"Decisão" sub-fase (b)** — Fase 4 multi-region
  scope-out per política P158.
- **NÃO DEBT-56** — fechado P221.

### Marcos arquitecturais P273.16

**(1) Sub-padrão "Scope-out reconfirmado por Fase A" N=2 → N=3
cumulativo crossing limiar formalização ADR meta N=3-4** — terceira
aplicação consolida padrão com 3 razões NO-GO distintas e legítimas:

- **N=1 (P273.14)**: Constraints **externas** (licensing + invariante L0).
- **N=2 (P273.15)**: Constraints **internas** (custo perf + demanda).
- **N=3 (P273.16)**: Bloqueador **estrutural aceito** (P156H limitação consciente per ADR-0054 graded + actualização empírica premissa spec).

Candidato meta-ADR formalização **NÃO reservado** — sub-padrão
consolidado a nível meta-metodológico per ADR-0093 Pattern 2.

**(2) Sub-padrão "Sub-passos consecutivos do mesmo cluster" N=11 →
N=12 cumulativo emergente** — P273.5-P273.16. **Caminho mais longo
de sub-passos consecutivos do mesmo cluster documentado no projecto
cristalino**.

**(3) Cluster Gradient feature-complete declarável** — sequência
"terminar cluster Gradient" iniciada em P273.10 considera-se esgotada
com 7 sub-passos materializados (P273.10-P273.13) + 3 scope-outs
reconfirmados (P273.14-P273.16) + sub-passos precedentes
P273.5-P273.9.

**(4) Pendência candidata XS nova descoberta empíricamente**:
**P273.X-bis-content-md-debt56-update** — L0 `content.md:824`
referência DEBT-56 fechado P221 está desactualizada (~1 LOC L0).
Sub-padrão "Cleanup XS derivado" análogo P273.8. NÃO reservado.

### Decisão fixada Fase A

1. **Decisão 1 (caminho)**: **3 — scope-out preserved**.
2. **Decisão 2 (apenas se GO)**: **N/A** — NO-GO.
3. **Decisão 3 (sempre)**: documento `trabalho-previo-externo.md`
   produzido referenciando bloqueador real (NÃO DEBT-56 fechado).

### Defaults preservados P262-P273.15 bit-exact

- Zero alterações código L1/L3 (ADR-0029 preserved absoluto).
- 3γ.2.γ-inline-baseline-y Boxed inline preserved literal.
- P156H limitação consciente preserved.
- 2644 baseline P273.15 preserved bit-exact.

---

## §2 — Outputs do passo

### §2.1 — Diagnóstico Fase A (documento principal)

`00_nucleo/diagnosticos/typst-passo-273-16-diagnostico.md`:
- §A.0 descoberta empírica + actualização premissa spec.
- §A.1 inventário literal (DEBT-56 ENCERRADO; P156H limitação).
- §A.2 inventário 3 caminhos.
- §A.3 decisão NO-GO fixada com fundamento quádruplo.
- §A.4 critério GO não cumprido.
- §A.5 critério NO-GO cumprido absoluto (4 critérios).
- §A.6 análise de risco — 5 mitigações.
- §A.7 decisões fixadas + pendência candidata XS descoberta.
- §A.8 critério aceitação cumprido.
- §A.9 plano Fase C reduzida + crossing limiar N=3-4.

### §2.2 — Trabalho prévio externo (output independente NO-GO)

`00_nucleo/diagnosticos/typst-passo-273-16-trabalho-previo-externo.md`:
- §0 propósito + actualização empírica.
- §1 pré-requisitos REAIS (3 itens; substitui premissa DEBT-56
  desactualizada).
- §2 critérios para reabrir P273.16 GO futuro.
- §3 pendência registada permanente.
- §4 sub-padrão N=3 cumulativo crossing limiar formalização.
- §5 pendência candidata XS nova
  `P273.X-bis-content-md-debt56-update`.
- §6 cluster Gradient declarável feature-complete pós-P273.16.

### §2.3 — ADR-0091 anotação cumulativa (décima sexta consecutiva)

§"Anotação cumulativa P273.16 — Bbox.y topo-exacto scope-out
reconfirmado":
- Descoberta empírica DEBT-56 fechado P221.
- Razão concreta quádrupla NO-GO.
- Bloqueador real identificado (P156H + ADR-0078 §sub-fase b).
- Trabalho prévio externo (3 itens).
- Sub-padrão N=2 → N=3 cumulativo crossing limiar formalização.
- 14 sub-padrões aplicados.
- Pendência candidata XS nova `P273.X-bis-content-md-debt56-update`.

### §2.4 — L0 `entities/gradient.md` anotação

Adicionada anotação P273.16 após P273.15. Hash propagado via
`crystalline-lint --fix-hashes`:
`01_core/src/entities/gradient.rs:46077a25`.

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P273.16 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=22 → N=23 cumulativo consolidação clara persistente** | Décima sexta anotação consecutiva ADR-0091 |
| Reutilização literal helpers cross-passos | **N=17 preserved** | |
| Cap LOC hard vs soft explícito | **N=16 preserved** | NO-GO — sem cap aplicado |
| Aplicação meta-ADR (ADR-0093) | **N=11 → N=12 cumulativo** | Pattern 2 anotação |
| Aplicação meta-ADR (ADR-0094) | **N=12 preserved** | NO-GO — sem Pattern 1 cap |
| Pattern DEBT-37 replicado | **N=4 preserved** | |
| Template-passo replicado literal | **N=2 preserved** | |
| Sub-passos consecutivos do mesmo cluster | **N=11 → N=12 cumulativo emergente** | P273.5-P273.16; **caminho mais longo documentado** |
| Layout duplo arquitectural aceite | **N=1 preserved** | |
| L3-only parent_bbox | **N=2 preserved** | |
| Dedup Arc::as_ptr resources | **N=3 preserved** | |
| Bug arquitectural intencional corrigido | **N=1 preserved** | |
| Triplicação Group bbox | **N=1 preserved** | |
| **Scope-out reconfirmado por Fase A** | **N=2 → N=3 cumulativo crossing limiar formalização N=3-4** | Terceira aplicação; 3 razões NO-GO distintas; candidato meta-ADR NÃO reservado |
| Diagnóstico imutável | **N=31 → N=32 cumulativo** | Vigésimo sétimo consumo |

---

## §4 — Métricas finais

| Métrica | Pré-P273.16 | Pós-P273.16 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2644 | **2644** | 0 (zero alterações código) |
| Tests P273.16 novos | — | 0 | NO-GO |
| Tests P262-P273.15 (verdes) | preserved | preserved | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Build warnings (novos) | 0 | 0 | 0 |
| Hashes propagados L0 | — | 1 (`gradient.rs:46077a25`) | +1 |
| ADRs totais | 81 | **81** | 0 (sem nova ADR; décima sexta anotação ADR-0091) |
| LOC L3 (additions) | — | 0 | NO-GO |
| LOC L1 (additions) | — | 0 | literal |
| Documentos novos | — | 2 | Diagnóstico + Trabalho prévio externo |

### §política condições verificadas

- ✓ Fase A produzida + critério §A.8 cumprido absoluto.
- ✓ ADR-0091 anotada (décima sexta consecutiva — versão NO-GO N=3).
- ✓ Documento `trabalho-previo-externo.md` produzido (com bloqueador
  REAL; não DEBT-56 fechado).
- ✓ Zero alterações código L3/L1.
- ✓ Tests workspace 2644 preserved bit-exact.
- ✓ Lint zero preserved.
- ✓ Hash L0 propagado.
- ✓ ADR-0029 pureza física L1 preserved (absoluto — zero código).
- ✓ ADR-0054 graded — NO-GO é cumprimento honesto.
- ✓ Sub-padrão "Scope-out reconfirmado por Fase A" N=3 cumulativo
  crossing limiar formalização ADR meta N=3-4.

**10 condições §política verificadas — 10 satisfeitas absolutas**
per ADR-0094 Pattern 1 (NO-GO outcome).

---

## §5 — Verificação regressão zero P262-P273.15

**2644 baseline preservado bit-exact**:

- typst-core: 2179 preserved.
- typst-shell: 24 preserved.
- typst-infra: 418 preserved.
- typst-wiring + bins + outros: 23 preserved.

**Total: 2644 → 2644 preserved**.

**2 testes typst-core skipped** (stack overflow pré-existente) —
NÃO regressão P273.16.

---

## §6 — Comparação P273.14 vs P273.15 vs P273.16

| Aspecto | P273.14 | P273.15 | P273.16 |
|---|---|---|---|
| Bloqueador inicial spec | Licensing + invariante L0 | Custo perf + demanda | DEBT-56 (premissa desactualizada) |
| Bloqueador real Fase A | Mesmo (confirmado) | Mesmo (confirmado) | P156H + ADR-0078 §sub-fase b (actualização empírica) |
| Razão NO-GO | Constraints externas | Constraints internas | Bloqueador estrutural aceito |
| Trabalho prévio externo | 3 pré-requisitos | 2 pré-requisitos | 3 pré-requisitos |
| Sub-padrão | **Inaugural N=1** | **Reaplicação N=2** | **Crossing N=3 limiar formalização** |
| Outcome | Padrão inaugurado | Padrão consolidado | Padrão atinge formalização |

**3 razões NO-GO distintas** estabelecem sub-padrão consolidado:
- Externa (licensing, invariante).
- Interna (perf, demanda).
- Estrutural (graded acceptance, premissa actualizada).

---

## §7 — Pendências preservadas pós-P273.16

Inalteradas vs P273.15 (nível cluster):

- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

Pendências cluster Gradient pós-P273.16:

- **P-Gradient-CMYK-ICC** preserved scope-out (P273.14 NO-GO).
- **P273.X-bis-bbox-medido-pos-layout** preserved scope-out (P273.15
  NO-GO).
- **P273.X-bis2-bbox-y-topo-exacto-inline** preserved scope-out
  (P273.16 NO-GO).
- **P273.X-bis-helper-group-bbox** (P273.13 §9; NÃO reservado).
- **P273.X-bis-content-md-debt56-update** descoberta P273.16
  (cleanup XS; NÃO reservado).

Pendência implícita exposta P273.13 §9 (fora cluster):
- **P273.X-bis-draw-item-local-text-image** — Text + Image em Groups.

**Cluster Gradient declarável feature-complete pós-P273.16**:

- ✓ Feature-complete user-facing (cross-variant runtime fields 3/3).
- ✓ Adaptive N qualitativo (P274).
- ✓ Refino estrutural extensivo (P273.5-P273.13: Block + Boxed +
  Grid + Stack + Pad + Group).
- ✓ Cleanup intra-cluster (P273.8 + P273.11).
- ✓ Dedup bbox-aware (P273.12).
- ✓ Render real Groups via pattern dict (P273.13).
- ✓ CMYK-ICC scope-out reconfirmado (P273.14).
- ✓ Bbox medido pós-layout scope-out reconfirmado (P273.15).
- ✓ Bbox.y topo-exacto scope-out reconfirmado (P273.16).

**Próximo passo natural**: sair do cluster Gradient definitivamente.
Pendências restantes do projecto disponíveis.

---

## §8 — Limitações conscientes P273.16

- 3γ.2.γ-inline-baseline-y continua a ser bbox.y para Boxed
  inline. Aproximação aceitável per ADR-0054 graded + coerente com
  P156H limitação consciente.
- Refino futuro depende de:
  - Caso empírico concreto (demanda visualmente significativa).
  - Refactor inline line_height (Fase 4 multi-region ou dedicated;
    escopo L+).
  - Decisão arquitectural sobre dívida invisível Caminho 2.
- NO-GO **não é falha** — é cumprimento honesto do critério
  "verificar bloqueador" registado em P273.7-P273.15.
- Premissa spec actualizada por verificação empírica — descoberta
  honesta documentada (DEBT-56 fechado P221 vs spec previa EM
  ABERTO).

---

## §9 — Marco final P273.16 — Cluster Gradient feature-complete declarável

**P273.X-bis2-bbox-y-topo-exacto-inline pendência reconfirmada como
scope-out preserved**:

- Fase A factual: 3 caminhos inventariados; descoberta empírica
  actualiza premissa da spec (DEBT-56 fechado P221).
- Decisão NO-GO quadruplamente fundamentada via bloqueador real
  identificado (P156H + ADR-0078).
- Zero código L1/L3 alterado — 3γ.2.γ-inline-baseline-y actual
  preserved.
- Trabalho prévio externo documentado: 3 pré-requisitos.
- 2644 baseline P273.15 preserved bit-exact.

Sub-padrão **"Scope-out reconfirmado por Fase A" N=2 → N=3 cumulativo
crossing limiar formalização ADR meta N=3-4** — terceira aplicação
consolida padrão com 3 razões NO-GO distintas e legítimas
(externa P273.14 + interna P273.15 + estrutural aceita P273.16).
Candidato meta-ADR formalização NÃO reservado.

Sub-padrão **"Sub-passos consecutivos do mesmo cluster" N=11 → N=12
cumulativo emergente** (P273.5-P273.16) — **caminho mais longo de
sub-passos consecutivos do mesmo cluster documentado no projecto
cristalino**.

**Cluster Gradient feature-complete declarável**:
- 9 sub-passos materializados (P273.5-P273.13).
- 3 sub-passos scope-out reconfirmados (P273.14-P273.16).
- 12 sub-passos consecutivos total.
- 16 anotações cumulativas ADR-0091 consolidação clara persistente.
- N sub-padrões emergentes inaugurados + 1 atingindo limiar
  formalização.
- 3 trabalhos prévios externos documentados (com pré-requisitos
  identificados para futuro hipotético GO).

Cluster Gradient feature-complete + qualitativo + refino estrutural
extensivo + cleanup intra-cluster + dedup bbox-aware + render real
Groups + 3 scope-outs reconfirmados com fundamentos diferentes —
**próximo passo natural: sair do cluster Gradient definitivamente**.

---

## §10 — Reflexão metodológica (per spec §10)

A sequência P273.5-P273.16 documentou:

- **9 sub-passos materializados** (P273.5-P273.13).
- **3 sub-passos scope-out reconfirmados** (P273.14-P273.16).
- **12 sub-passos consecutivos** total — caminho mais longo
  documentado no projecto cristalino.
- **16 anotações cumulativas ADR-0091** (sub-padrão consolidação
  consolidada).
- **Vários sub-padrões emergentes** inaugurados ou consolidados:
  - L3-only parent_bbox (N=2 cumulativo, P273.10+P273.13).
  - Dedup Arc::as_ptr resources (N=3 cumulativo crossing limiar,
    P73+P263+P273.12).
  - Bug arquitectural intencional corrigido (N=1 inaugural P273.12).
  - Triplicação Group bbox (N=1 emergente P273.13).
  - Layout duplo arquitectural aceite (N=1 inaugural P273.9).
  - Template-passo replicado literal (N=2 cumulativo P273.7+P273.9).
  - Extract helper de replicação inline (N=1 inaugural P273.11).
  - **Scope-out reconfirmado por Fase A (N=3 cumulativo crossing
    limiar formalização N=3-4, P273.14+P273.15+P273.16)**.

P273.16 **fecha o ciclo metodológico** — sub-padrão "Scope-out
reconfirmado por Fase A" atingiu N=3 (limiar formalização ADR meta).
Cluster Gradient torna-se laboratório metodológico documentado do
projecto.

Trajectória honesta: cluster começou em P262 como feature básica;
cresceu para refino estrutural completo via P273-P273.13;
consolidou-se com 3 scope-outs documentados como cumprimento de
critérios "verificar empíricamente". Output final é **cluster
funcional + dívida documentada + sub-padrões consolidados**.

Descoberta empírica P273.16: premissa da spec sobre DEBT-56 status
actualizada por verificação factual — padrão honesto de
**Fase A factual prevalece sobre premissa documental** quando há
divergência. Mecânica que se aplica recursivamente: o próprio
processo de verificar pode descobrir que o documento de referência
está desactualizado, e isso é parte legítima do output.

---

## §11 — Referências

- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa
  P273.16; décima sexta anotação consecutiva; sub-padrão N=3
  cumulativo crossing limiar).
- ADR-0093 — Meta-metodologia evolução ADRs (Pattern 2 aplicação
  prática N=12 cumulativo).
- ADR-0094 — Meta-operacional specs (Pattern 1 cap LOC; N=12
  preserved; NO-GO sem cap aplicado).
- ADR-0085 — Diagnóstico imutável (vigésimo sétimo consumo).
- ADR-0029 — Pureza física L1 (preserved absoluto; zero código).
- ADR-0054 — Critério fecho DEBT-1 (graded — NO-GO output legítimo;
  scope-out reconfirmado).
- ADR-0078 §"Decisão" sub-fase (b) — Opção A multi-region
  scope-out documentada per P158.
- DEBT.md DEBT-56 — **ENCERRADO P221 (2026-05-12)**; spec premissa
  factualmente desactualizada.
- P273.14 — Sub-padrão inaugural N=1 (CMYK-ICC; constraints
  externas).
- P273.15 — Reaplicação N=2 (bbox pós-layout; constraints internas).
- P273.16 — **Esta aplicação N=3 crossing limiar formalização**
  (bloqueador estrutural aceito).
- P273.7 §A.3 — Decisão 1 (3γ.2.γ-inline-baseline-y) preserved 9
  sub-passos.
- P156H limitação consciente em `content.md:817-829` preserved per
  ADR-0054 graded.
- P156H referência a DEBT-56 em `content.md:824` factualmente
  desactualizada — candidato cleanup XS futuro
  `P273.X-bis-content-md-debt56-update` NÃO reservado.
- `00_nucleo/diagnosticos/typst-passo-273-16-diagnostico.md` — Fase
  A empírica + decisão NO-GO + critério §A.8 cumprido.
- `00_nucleo/diagnosticos/typst-passo-273-16-trabalho-previo-externo.md`
  — 3 pré-requisitos para reanálise futura GO + descoberta candidata
  XS.
- Spec P273.16 — `00_nucleo/materialization/typst-passo-273-16.md`.

---

*Relatório imutável produzido em 2026-05-18. Pendência
P273.X-bis2-bbox-y-topo-exacto-inline reconfirmada como scope-out
preserved via NO-GO Fase A factual; descoberta empírica importante
(DEBT-56 fechado P221, premissa spec desactualizada); sub-padrão
"Scope-out reconfirmado por Fase A" N=2 → N=3 cumulativo
**crossing limiar formalização ADR meta N=3-4** — terceira
aplicação consolida com 3 razões NO-GO distintas; "Sub-passos
consecutivos do mesmo cluster" N=12 cumulativo emergente — **caminho
mais longo de sub-passos consecutivos do mesmo cluster documentado
no projecto cristalino**. **Cluster Gradient feature-complete
declarável pós-P273.16**.*
