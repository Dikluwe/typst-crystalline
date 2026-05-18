# Relatório P273.15 — Bbox medido pós-layout (SCOPE-OUT-RECONFIRMED via NO-GO Fase A; reaplicação)

**Data**: 2026-05-18.
**Status**: **SCOPE-OUT-RECONFIRMED** (NO-GO outcome legítimo — não falha; não IMPLEMENTADO).
**Magnitude real**: zero LOC; 0 tests novos; documentação Fase A + trabalho prévio externo.
**Cluster**: Visualize / Gradient (sexto sub-passo na sequência terminar cluster).
**Tipo**: Fase A com decisão go/no-go binária; outcome NO-GO confirmado empíricamente — segunda aplicação consolidando padrão.
**Spec**: `00_nucleo/materialization/typst-passo-273-15.md`.

---

## §1 — Sumário executivo

**Pendência P273.X-bis-bbox-medido-pos-layout reconfirmada como
scope-out preserved** via decisão NO-GO Fase A factual com razão
quádrupla:

1. **Zero demanda empírica**: 8 sub-passos consecutivos
   (P273.6-P273.13) sem caso registado onde 3γ.2.γ produziu output
   observable incorrecto. Verificação literal via grep em 20
   documentos `00_nucleo/`.
2. **Caminho 1 (eager) custo perf inaceitável**:
   `measure_content_constrained` em todos os Blocks sem dimensions,
   mesmo sem gradient `relative=parent` interno. Pior caso O(N²)
   onde antes era O(N).
3. **Caminho 2 (lazy) custo impl desproporcional**: walker novo
   (~60-100 LOC L1) + manutenção sem demanda registada.
4. **3γ.2.γ aceito por ADR-0054 graded**: "menor mudança suficiente"
   preserved; refino sem demanda é over-engineering.

### Marcos arquitecturais P273.15

**(1) Sub-padrão "Scope-out reconfirmado por Fase A" N=1 → N=2
cumulativo emergente** — primeira reaplicação do padrão inaugurado
P273.14 consolida a mecânica:
- N=1 (P273.14): CMYK-ICC; constraints externas (licensing + crate).
- N=2 (P273.15): Bbox pós-layout; ausência de demanda + custo perf.

Distinção arquitectural: ambos legítimos per ADR-0054 graded mas
razões NO-GO diferentes — P273.14 por **constraints externas**;
P273.15 por **ausência de demanda + custo perf**.

**(2) Sub-padrão "Sub-passos consecutivos do mesmo cluster" N=10 →
N=11 cumulativo emergente** — P273.5/6/7/8/9/10/11/12/13/14/15.
Cluster Gradient mantém folga máxima sobre limiar formalização
N=3-4. Caminho **mais longo de sub-passos consecutivos do mesmo
cluster** documentado no projecto cristalino.

**(3) Decisão P273.6 §A.3 (3γ.2.γ) preserved literal** — 8 sub-passos
sem contraproba; ADR-0091 §"Anotação cumulativa P273.6" preserved.

### Decisão fixada Fase A

1. **Decisão 1 (caminho)**: **3 — scope-out preserved**.
2. **Decisão 2 (apenas se GO)**: **N/A** — NO-GO.
3. **Decisão 3 (sempre)**: documento `trabalho-previo-externo.md`
   produzido como output do passo per §6 workflow obrigação.

### Defaults preservados P262-P273.14 bit-exact

- Zero alterações código L1/L3 (ADR-0029 preserved absoluto).
- Block com dimensions literais → 3γ.2.γ literal preserved.
- Block sem dimensions → fallback page_bbox L3 P273.5 (identity
  transform); aceito por defaults.
- 2644 baseline P273.14 preserved bit-exact.

---

## §2 — Outputs do passo

### §2.1 — Diagnóstico Fase A (documento principal)

`00_nucleo/diagnosticos/typst-passo-273-15-diagnostico.md`:
- §A.1 inventário demanda empírica (zero casos via grep verificação).
- §A.2 inventário 3 caminhos com custo perf concreto.
- §A.3 decisão **NO-GO** fixada com fundamento quádruplo.
- §A.4 critério para GO — não cumprido.
- §A.5 critério para NO-GO — cumprido absoluto (4 critérios).
- §A.6 análise de risco — 5 mitigações.
- §A.7 decisões fixadas — Caminho 3.
- §A.8 critério aceitação cumprido.

### §2.2 — Trabalho prévio externo (output independente NO-GO)

`00_nucleo/diagnosticos/typst-passo-273-15-trabalho-previo-externo.md`:
- §1.1 caso empírico concreto identificado (aguardar / comparar com
  vanilla).
- §1.2 decisão executiva sobre custo perf.
- §2 critérios para reabrir P273.15 como GO futuro (2 itens).
- §3 pendência registada permanente.
- §4 sub-padrão "Scope-out reconfirmado por Fase A" N=2 cumulativo
  consolidando.

### §2.3 — ADR-0091 anotação cumulativa (décima quinta consecutiva)

§"Anotação cumulativa P273.15 — Bbox pós-layout scope-out
reconfirmado (NO-GO Fase A; reaplicação)":
- Razão concreta quádrupla.
- Trabalho prévio externo identificado (2 pré-requisitos).
- Decisão P273.6 §A.3 (3γ.2.γ) preserved literal.
- Sub-padrão "Scope-out reconfirmado por Fase A" N=2 cumulativo.
- 14 sub-padrões aplicados.

### §2.4 — L0 `entities/gradient.md` anotação

Adicionada anotação P273.15 após P273.14. Hash propagado via
`crystalline-lint --fix-hashes`:
`01_core/src/entities/gradient.rs:9c2ff872`.

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P273.15 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=21 → N=22 cumulativo consolidação clara persistente** | Décima quinta anotação consecutiva ADR-0091 |
| Reutilização literal helpers cross-passos | **N=17 preserved** | |
| Cap LOC hard vs soft explícito | **N=16 preserved** | NO-GO — sem cap aplicado |
| Aplicação meta-ADR (ADR-0093) | **N=10 → N=11 cumulativo** | Pattern 2 anotação |
| Aplicação meta-ADR (ADR-0094) | **N=12 preserved** | NO-GO — sem Pattern 1 cap |
| Pattern DEBT-37 replicado | **N=4 preserved** | |
| Template-passo replicado literal | **N=2 preserved** | |
| Sub-passos consecutivos do mesmo cluster | **N=10 → N=11 cumulativo emergente** | P273.5-P273.15; folga máxima sobre limiar N=3-4 |
| Layout duplo arquitectural aceite | **N=1 preserved** | (NO-GO não cresce; só GO crescia) |
| L3-only parent_bbox | **N=2 preserved** | |
| Dedup Arc::as_ptr resources | **N=3 preserved** | |
| Bug arquitectural intencional corrigido | **N=1 preserved** | |
| Triplicação Group bbox | **N=1 preserved** | |
| **Scope-out reconfirmado por Fase A** | **N=1 → N=2 cumulativo emergente** | Primeira reaplicação consolida padrão inaugurado P273.14 |
| Diagnóstico imutável | **N=30 → N=31 cumulativo** | Vigésimo sexto consumo |

---

## §4 — Métricas finais

| Métrica | Pré-P273.15 | Pós-P273.15 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2644 | **2644** | 0 (zero alterações código) |
| Tests P273.15 novos | — | 0 | NO-GO — sem materialização |
| Tests P262-P273.14 (verdes) | preserved | preserved | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Build warnings (novos) | 0 | 0 | 0 |
| Hashes propagados L0 | — | 1 (`gradient.rs:9c2ff872`) | +1 (anotação P273.15) |
| ADRs totais | 81 | **81** | 0 (sem nova ADR; décima quinta anotação ADR-0091) |
| LOC L3 (additions) | — | 0 | NO-GO — literal |
| LOC L1 (additions) | — | 0 | literal (ADR-0029 preserved) |
| Documentos novos | — | 2 | Diagnóstico Fase A + Trabalho prévio externo |

### §política condições verificadas

- ✓ Fase A produzida + critério §A.8 cumprido absoluto.
- ✓ ADR-0091 anotada (décima quinta consecutiva — versão NO-GO).
- ✓ Documento `trabalho-previo-externo.md` produzido (output legítimo
  per §6 workflow obrigação).
- ✓ Zero alterações código L3/L1.
- ✓ Tests workspace 2644 preserved bit-exact (sem mudança).
- ✓ Lint zero preserved.
- ✓ Hash L0 propagado.
- ✓ ADR-0029 pureza física L1 preserved (absoluto — zero código).
- ✓ ADR-0054 graded — NO-GO é cumprimento honesto, não falha.
- ✓ Sub-padrão "Scope-out reconfirmado por Fase A" N=2 cumulativo
  (primeira reaplicação consolidando).

**10 condições §política verificadas — 10 satisfeitas absolutas**
per ADR-0094 Pattern 1 (NO-GO outcome) e §6 spec workflow.

---

## §5 — Verificação regressão zero P262-P273.14

**2644 baseline preservado bit-exact**:

- typst-core: 2179 preserved.
- typst-shell: 24 preserved.
- typst-infra: 418 preserved.
- typst-wiring + bins + outros: 23 preserved.

**Total: 2644 → 2644 preserved** (NO-GO — zero alterações código;
apenas anotações ADR + L0 + 2 documentos Fase A + trabalho prévio
externo).

**2 testes typst-core skipped** (stack overflow pré-existente) —
NÃO regressão P273.15.

---

## §6 — Comparação P273.14 vs P273.15

| Aspecto | P273.14 (CMYK-ICC) | P273.15 (Bbox pós-layout) |
|---|---|---|
| Tipo pendência | Refino qualitativo opcional | Refino qualitativo opcional |
| Bloqueador identificado | Profile licensing + invariante L0 | Custo perf + ausência demanda |
| Caminhos viabilidade | 3 (crate/hardcoded/scope-out) | 3 (eager/lazy/scope-out) |
| Outcome Fase A | NO-GO | NO-GO |
| Razão NO-GO | Constraints **externas** | Constraints **internas** (perf+demanda) |
| Pré-requisitos GO futuro | 3 (ADR + profile + licença + size decision) | 2 (caso empírico + decisão perf) |
| Sub-padrão | **Inaugural N=1** | **Reaplicação N=2** |
| Padrão consolidação | Inauguração | Primeira reaplicação consolida |

Distinção arquitectural: P273.14 NO-GO por constraints externas
(licensing, invariante L0); P273.15 NO-GO por constraints internas
(custo perf, ausência demanda). Ambos legítimos per ADR-0054 graded.

---

## §7 — Pendências preservadas pós-P273.15

Inalteradas vs P273.14:

- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

**P273.X-bis-bbox-medido-pos-layout permanece** como pendência
formal aberta com **trabalho prévio externo identificado** (2
pré-requisitos: caso empírico + decisão perf). Cluster Gradient
**pode declarar-se feature-complete sem este refino**.

**P-Gradient-CMYK-ICC** preserved (P273.14 NO-GO output).

Sequência para fechar cluster Gradient pós-P273.15:

- ✓ P273.5-P273.14 (fechados).
- ✓ **P273.15** — Bbox medido pós-layout (este passo; **NO-GO
  SCOPE-OUT-RECONFIRMED**).
- **P273.16** — Bbox.y topo-exacto inline (M-L; **BLOQUEADO** por
  DEBT-56; predição NO-GO obrigatório por construção).

Pendência descoberta P273.13 §9 preserved:
- **P273.X-bis-helper-group-bbox** — extract helper compartilhado
  3 sítios.

**Predição revisada**: cluster Gradient pode fechar **com P273.16
NO-GO bloqueado por DEBT-56** (trabalho externo de refactor
multi-region) ou directamente após P273.15. Se cluster declarar
feature-complete agora — 3 sub-passos consecutivos de scope-out
reconfirmados (P273.14, P273.15, e possivelmente P273.16) sinalizam
clara saturação do escopo cluster.

---

## §8 — Limitações conscientes P273.15

- 3γ.2.γ continua a ser caminho para Block sem dimensions — gradient
  `relative=parent` aninhado em Block sem dims cai no fallback
  page_bbox.
- Refino futuro candidato se §1 do trabalho-previo-externo for
  cumprido (caso empírico + decisão perf).
- NO-GO **não é falha** — é cumprimento honesto do critério
  "verificar empíricamente" registado em todos os relatórios
  anteriores P273.6-P273.13.

---

## §9 — Marco final P273.15

**P273.X-bis-bbox-medido-pos-layout pendência reconfirmada como
scope-out preserved**:

- Fase A factual: 3 caminhos inventariados (eager + lazy +
  scope-out); custo perf quantificado.
- Decisão NO-GO quádruplamente fundamentada.
- Zero código L1/L3 alterado — 3γ.2.γ actual preserved.
- Trabalho prévio externo documentado: 2 pré-requisitos para futuro.
- 2644 baseline P273.14 preserved bit-exact.

Sub-padrão **"Scope-out reconfirmado por Fase A" N=1 → N=2
cumulativo emergente** — primeira reaplicação consolida mecânica
inaugurada P273.14. Padrão estabelecido como output legítimo para
refinos qualitativos opcionais sem demanda empírica ou bloqueador
arquitectural.

Sub-padrão **"Sub-passos consecutivos do mesmo cluster" N=10 →
N=11 cumulativo emergente** (P273.5-P273.15). Cluster Gradient
atinge **caminho mais longo de sub-passos consecutivos do mesmo
cluster** documentado — folga máxima sobre limiar formalização
N=3-4 preservada extensivamente.

Cluster Gradient feature-complete + qualitativo + refino estrutural
extensivamente encerrado + cleanup intra-cluster + dedup bbox-aware
+ render real Groups + **CMYK-ICC scope-out reconfirmado** + **Bbox
pós-layout scope-out reconfirmado** — próximo passo: **P273.16**
Bbox.y topo-exacto inline (M-L; bloqueado DEBT-56; predição NO-GO
obrigatório por construção).

---

## §10 — Referências

- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa
  P273.15; décima quinta anotação consecutiva).
- ADR-0093 — Meta-metodologia evolução ADRs (Pattern 2 aplicação
  prática N=11 cumulativo).
- ADR-0094 — Meta-operacional specs (Pattern 1 cap LOC; N=12
  preserved; NO-GO sem cap aplicado).
- ADR-0085 — Diagnóstico imutável (vigésimo sexto consumo).
- ADR-0029 — Pureza física L1 (preserved absoluto; P273.15 zero
  código).
- ADR-0054 — Critério fecho DEBT-1 (graded — NO-GO output legítimo;
  scope-out reconfirmado por ausência de demanda).
- P273.6 §A.3 — opções 3γ.2.α/β/γ documentadas; escolha 3γ.2.γ
  preserved 8 sub-passos.
- P273.14 — Sub-padrão "Scope-out reconfirmado por Fase A" inaugural
  N=1 (CMYK-ICC); P273.15 reaplica → N=2 cumulativo.
- P273.9 — Stack/Pad usam 3γ.2.β porque sem dimensions literais é
  o caminho natural (não over-engineering).
- `00_nucleo/diagnosticos/typst-passo-273-15-diagnostico.md` — Fase
  A empírica + decisão NO-GO + critério §A.8 cumprido.
- `00_nucleo/diagnosticos/typst-passo-273-15-trabalho-previo-externo.md`
  — 2 pré-requisitos para reanálise futura GO.
- Spec P273.15 — `00_nucleo/materialization/typst-passo-273-15.md`.

---

*Relatório imutável produzido em 2026-05-18. Pendência
P273.X-bis-bbox-medido-pos-layout reconfirmada como scope-out
preserved via NO-GO Fase A factual; sub-padrão "Scope-out
reconfirmado por Fase A" N=1 → N=2 cumulativo emergente — primeira
reaplicação consolida mecânica inaugurada P273.14; "Sub-passos
consecutivos do mesmo cluster" N=11 cumulativo emergente — cluster
Gradient atinge caminho mais longo documentado no projecto
cristalino.*
