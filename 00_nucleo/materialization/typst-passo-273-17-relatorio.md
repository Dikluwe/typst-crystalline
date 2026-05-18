# Relatório P273.17 — Reflexão metodológica formal cluster Gradient + 3 ADRs meta novas

**Data**: 2026-05-18.
**Status**: **IMPLEMENTADO** (admin S+).
**Magnitude real**: zero LOC código; 3 ADRs novas EM VIGOR (~1100 linhas markdown) + documento reflexão (~500 linhas) + ADR-0091 décima sétima anotação + README atualizado.
**Cluster**: Visualize / Gradient (**último** sub-passo da sequência; cluster encerrado definitivamente).
**Tipo**: passo administrativo S+ — formalização sub-padrões empíricos via meta-ADRs novas.
**Spec**: `00_nucleo/materialization/typst-passo-273-17.md`.

---

## §1 — Sumário executivo

**Cluster Gradient encerrado definitivamente pós-P273.17**:

- **3 ADRs meta novas EM VIGOR** formalizando sub-padrões empíricos
  N=3-4 cumulativos atingidos durante cluster.
- **1 documento reflexão** standalone como output legível.
- **17 anotações cumulativas ADR-0091** consecutivas (P273.5-P273.17).
- **Zero alterações código L1/L3** (ADR-0029 preserved absoluto).
- **2644 baseline preservado bit-exact**.

### ADRs criadas EM VIGOR (paridade P271)

| ADR | Sub-padrão | N cumulativo | Inauguração |
|---|---|---|---|
| **ADR-0095** | Dedup `Arc::as_ptr` resources | **N=3** crossing limiar | P73 image_resources |
| **ADR-0096** | Pattern DEBT-37 campo Layouter consumer-pending | **N=4** com folga | P84.6 Grid cell |
| **ADR-0097** | Scope-out reconfirmado por Fase A | **N=3** com 3 razões NO-GO distintas | P273.14 CMYK-ICC |

### Documento reflexão

`00_nucleo/diagnosticos/typst-cluster-gradient-reflexao.md`:
- §1 Trajectória factual P273.5-P273.16 (tabela 13 sub-passos).
- §2 Sub-padrões emergentes (17 cumulativos registados).
- §3 Limiares formalização atingidos (3 → ADRs novas).
- §4 Descobertas metodológicas (caps soft sub-estimados; Fase A
  factual prevalece; cleanups XS; bugs latentes).
- §5 Pendências residuais (3 scope-outs + 2 candidatos XS + 1
  fora cluster).
- §6 Trade-offs aceitos (`/DeviceCMYK` + 3γ.2.γ + baseline-y;
  todos ADR-0054 graded).
- §7 Anti-padrões evitados (over-formalização + scope creep cego +
  NO-GO cobertura + inserção não-documentada).
- §8 Reflexão final.
- §9 Conclusão.
- §10 Referências.

### Decisões fixadas Fase A

1. **Decisão 1 (localização)**: **1β
   `00_nucleo/diagnosticos/typst-cluster-gradient-reflexao.md`**.
2. **Decisão 2 (estado inicial ADRs)**: **2α EM VIGOR directo**
   (paridade P271 ADR-0093/0094).
3. **Decisão 3 (numeração)**: **cronológica por inauguração**:
   - 0095 — Dedup Arc (P73 inaugural mais antigo).
   - 0096 — Pattern DEBT-37 (P84.6).
   - 0097 — Scope-out reconfirmado (P273.14 mais recente).

### Marcos arquitecturais P273.17

**(1) ADRs vigentes 81 → 84** — primeira criação directa de ADRs
durante cluster Gradient (P273.5-P273.16 anotaram sem criar).

**(2) Sub-padrão "Passo administrativo XS/S criar ADRs meta" N=3
cumulativo** — P156K + P271 + **P273.17**. **NÃO formalizado**
nesta sessão (anti-padrão over-formalização explícito spec §0).

**(3) Sub-padrão "Sub-passos consecutivos do mesmo cluster" N=12 →
N=13 cumulativo emergente** — P273.5-P273.17. **Caminho mais longo
de sub-passos consecutivos do mesmo cluster documentado no projecto
cristalino**.

---

## §2 — Outputs do passo

### §2.1 — Diagnóstico Fase A

`00_nucleo/diagnosticos/typst-passo-273-17-diagnostico.md`:
- §A.1 inventário ADRs (81 vigentes; 93 files; 0095/96/97 livres).
- §A.2 verificação empírica N=3/4/3 para 3 sub-padrões.
- §A.3 estrutura documento reflexão (8 secções).
- §A.4-A.6 Decisões 1β + 2α + 3 cronológica.
- §A.7 análise de risco — 5 mitigações.
- §A.8 critério aceitação cumprido.

### §2.2 — ADR-0095 "Dedup `Arc::as_ptr` resources"

EM VIGOR directo. ~300 linhas markdown. Estrutura canónica:
- Contexto (3 aplicações cumulativas: P73 + P263 + P273.12).
- Decisão (forma canónica + forma estendida).
- Análise pureza ADR-0029 (L3 puro).
- Consequências + Alternativas.
- Precedentes citáveis.
- Próximos passos + Critério revisão.

### §2.3 — ADR-0096 "Pattern DEBT-37 campo Layouter consumer-pending"

EM VIGOR directo. ~350 linhas markdown. Estrutura canónica:
- Contexto (4 aplicações cumulativas: P84.6 + P273.5 + P273.6 + P273.9).
- Decisão (passo introdução + passo consumer; DEBT registado).
- Análise pureza ADR-0029 (L1 puro).
- Consequências + Alternativas.
- Precedentes citáveis.
- Próximos passos + Critério revisão.

### §2.4 — ADR-0097 "Scope-out reconfirmado por Fase A"

EM VIGOR directo. ~400 linhas markdown. Estrutura canónica:
- Contexto (3 aplicações com 3 razões NO-GO distintas: P273.14 +
  P273.15 + P273.16).
- Decisão (quando aplicar NO-GO; output mecânico; quando NÃO aplicar).
- Análise pureza ADR-0029 (N/A — metodológica).
- Consequências + Alternativas.
- Precedentes citáveis.
- Relação com sub-padrão "passo administrativo XS/S".

### §2.5 — Documento reflexão `typst-cluster-gradient-reflexao.md`

10 secções; ~500 linhas markdown. Standalone legível.

### §2.6 — ADR-0091 anotação cumulativa décima sétima

§"Anotação cumulativa P273.17 — Reflexão metodológica formal cluster
Gradient + 3 ADRs meta novas" cobrindo:
- Marco final cluster Gradient (13 sub-passos consecutivos).
- ADRs criadas (paridade P271).
- Documento reflexão como output independente.
- Sub-padrões NÃO formalizados (8 preserved emergentes).
- Sub-padrão meta-meta NÃO formalizado (anti-padrão over-formalização).
- 15 sub-padrões aplicados.

### §2.7 — L0 `entities/gradient.md` anotação

Anotação P273.17 após P273.16. Hash propagado via
`crystalline-lint --fix-hashes`:
`01_core/src/entities/gradient.rs:8d9730a3`.

### §2.8 — README ADRs atualizado

Linha "Total pós-P273.17: 84 ADRs (+3 ADRs EM VIGOR criadas
directamente)" adicionada após linha "Total pós-P273.6". Linha
condensada "Total pós-P273.7-P273.16: 81 ADRs preservado"
documentando os 10 sub-passos consecutivos sem criação ADR também
adicionada.

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P273.17 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=22 → N=23 cumulativo consolidação clara persistente** | Décima sétima anotação consecutiva ADR-0091 |
| Reutilização literal helpers cross-passos | **N=17 preserved** | |
| Cap LOC hard vs soft explícito | **N=16 preserved** | Passo administrativo sem cap LOC |
| Aplicação meta-ADR (ADR-0093) | **N=12 → N=13 cumulativo** | Pattern 2 anotação |
| Aplicação meta-ADR (ADR-0094) | **N=12 preserved** | Documental sem Pattern 1 cap |
| Pattern DEBT-37 `cell_origin_*` replicado | **N=4 preserved** | **Formalizada ADR-0096** |
| Template-passo replicado literal | **N=2 preserved** | |
| Sub-passos consecutivos do mesmo cluster | **N=12 → N=13 cumulativo emergente** | P273.5-P273.17; **caminho mais longo documentado** |
| Layout duplo arquitectural aceite | **N=1 preserved** | |
| L3-only parent_bbox | **N=2 preserved** | |
| Dedup Arc::as_ptr resources | **N=3 preserved** | **Formalizada ADR-0095** |
| Bug arquitectural intencional corrigido | **N=1 preserved** | |
| Triplicação Group bbox | **N=1 preserved** | |
| Scope-out reconfirmado por Fase A | **N=3 preserved** | **Formalizada ADR-0097** |
| Diagnóstico imutável | **N=32 → N=33 cumulativo** | Vigésimo oitavo consumo |
| **Passo administrativo XS/S criar ADRs meta** | **N=2 → N=3 cumulativo** (NÃO formalizado) | P156K + P271 + **P273.17**; limiar atingido mas preserved per anti-padrão |

---

## §4 — Métricas finais

| Métrica | Pré-P273.17 | Pós-P273.17 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2644 | **2644** | 0 (zero alterações código) |
| Tests P273.17 novos | — | 0 | Admin documental |
| Tests P262-P273.16 (verdes) | preserved | preserved | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Build warnings (novos) | 0 | 0 | 0 |
| Hashes propagados L0 | — | 1 (`gradient.rs:8d9730a3`) | +1 |
| **ADRs vigentes** | **81** | **84** | **+3 EM VIGOR (ADR-0095/0096/0097)** |
| ADRs files | 93 | 96 | +3 ficheiros |
| LOC L3 | — | 0 | literal |
| LOC L1 | — | 0 | literal (ADR-0029 preserved) |
| Documentos novos | — | 5 | Diagnóstico Fase A + 3 ADRs + documento reflexão |
| Linhas markdown novas | — | ~1700 | Cap soft 1200 estourado 42% (admin S+ aceito) |

### §política condições verificadas

- ✓ Fase A produzida + critério §A.8 cumprido absoluto.
- ✓ **ADR-0091 anotada** (décima sétima consecutiva).
- ✓ **ADR-0095 criada EM VIGOR** com estrutura canónica.
- ✓ **ADR-0096 criada EM VIGOR** com estrutura canónica.
- ✓ **ADR-0097 criada EM VIGOR** com estrutura canónica.
- ✓ **Documento reflexão** `typst-cluster-gradient-reflexao.md`
  criado em `00_nucleo/diagnosticos/`.
- ✓ **README ADRs atualizado** (81 → 84; EM VIGOR cresce 3).
- ✓ Zero código L1/L3.
- ✓ Tests workspace 2644 preserved bit-exact.
- ✓ Lint zero.
- ⚠ Cap documental soft 1200 estourado ~42% (3 ADRs ~1100 + reflexão
  ~500 + diagnóstico ~250 = ~1700). Justificado por magnitude S+
  documental + 3 ADRs simultâneas; estouro registado per ADR-0094
  Pattern 1 documental.
- ✓ Sub-padrões §4 atualizados.

**12 condições §política verificadas — 11 satisfeitas absolutas + 1
estouro soft documental registado** per ADR-0094 Pattern 1.

---

## §5 — Verificação regressão zero P262-P273.16

**2644 baseline preservado bit-exact**:

- typst-core: 2179 preserved.
- typst-shell: 24 preserved.
- typst-infra: 418 preserved.
- typst-wiring + bins + outros: 23 preserved.

**Total: 2644 → 2644 preserved** (admin documental — zero alterações
código).

---

## §6 — Pendências preservadas pós-P273.17

### Pendências fora cluster

Inalteradas vs P273.16:
- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

### Pendências cluster Gradient

- **3 scope-outs reconfirmados** P273.14/15/16 (com trabalho prévio
  externo documentado).
- **2 candidatos XS NÃO reservados**: `P273.X-bis-helper-group-bbox`
  + `P273.X-bis-content-md-debt56-update`.
- **1 pendência fora cluster** exposta: `P273.X-bis-draw-item-local-text-image`.

**Cluster Gradient encerrado definitivamente**:
- ✓ Feature-complete user-facing.
- ✓ Adaptive N qualitativo.
- ✓ Refino estrutural extensivo (P273.5-P273.13).
- ✓ Cleanup intra-cluster (P273.8 + P273.11).
- ✓ Dedup bbox-aware (P273.12).
- ✓ Render real Groups via pattern dict (P273.13).
- ✓ 3 scope-outs documentados (P273.14/15/16).
- ✓ **Sub-padrões formalizados** via 3 ADRs meta (P273.17).
- ✓ **Reflexão metodológica documentada** (P273.17).

**Próximo passo natural**: sair do cluster Gradient definitivamente
para outro cluster do projecto.

---

## §7 — Limitações conscientes P273.17

- 3 ADRs meta novas captam **apenas os sub-padrões com N≥3 e valor
  metodológico claro**. 8 sub-padrões emergentes N=1-2 preserved
  sem formalização (anti-padrão over-formalização explícito).
- Documento reflexão é **narrativa retrospectiva** — não substitui
  passos individuais (cada passo tem o seu relatório imutável).
- Estouro cap documental soft 1200 → ~1700 (~42%) aceito por
  magnitude S+ documental + 3 ADRs simultâneas. Registado per
  ADR-0094 Pattern 1.
- Sub-padrão meta-meta "Passo administrativo XS/S criar ADRs meta"
  N=3 atingiu limiar mas **NÃO formalizado** — coerência com
  anti-padrão.
- Reflexão captura trajectória mas **não prescreve metodologia**
  para clusters futuros — cada cluster tem natureza própria.

---

## §8 — Marco final P273.17

**Cluster Gradient encerrado definitivamente**:

- 9 sub-passos materializados código (P273.5-P273.13).
- 3 sub-passos scope-out reconfirmados (P273.14-P273.16).
- 1 sub-passo administrativo S+ (P273.17 — este passo).
- 13 sub-passos consecutivos total — **caminho mais longo
  documentado no projecto cristalino**.
- 17 anotações cumulativas ADR-0091 consecutivas.
- 3 ADRs meta novas (ADR-0095/0096/0097) EM VIGOR.
- 1 documento reflexão standalone.
- 2644 baseline preservado bit-exact em todos os 4 últimos
  sub-passos (P273.14-P273.17).
- 0 regressões em todo o cluster.

Sub-padrão **"Passo administrativo XS/S criar ADRs meta" N=3
cumulativo** atingiu limiar formalização N=3-4 (P156K + P271 +
P273.17). **NÃO formalizado** por anti-padrão over-formalização
explícito — documentado em `typst-cluster-gradient-reflexao.md` §7
para futuro hipotético se N=4 surgir noutro cluster.

Sub-padrão **"Sub-passos consecutivos do mesmo cluster" N=12 → N=13
cumulativo emergente** — **caminho mais longo de sub-passos
consecutivos do mesmo cluster documentado no projecto cristalino**.

Cluster Gradient laboratório metodológico documentado do projecto.
Próximo passo natural: **sair do cluster Gradient definitivamente**
para outro cluster (ADR-0055bis fonts / P-Footnote-N / DEBT-33
Bézier / Stroke/Curve/Polygon / Tiling / outro cluster do projecto).

---

## §9 — Reflexão meta-meta (per spec §10)

P273.17 documenta a trajectória metodológica do cluster Gradient —
mas é ele mesmo parte dessa trajectória. Sub-padrão **"Passo
administrativo XS/S criar ADRs meta"** tem precedente:

- **P156K** — ADR-0064 + ADR-0065 EM VIGOR.
- **P271** — ADR-0093 + ADR-0094 EM VIGOR.
- **P273.17 (este passo)** — ADR-0095 + ADR-0096 + ADR-0097 EM VIGOR.

N=3 cumulativo. Padrão consolidado: clusters complexos terminam
com passo administrativo que formaliza sub-padrões emergentes via
ADRs meta.

Esta reflexão meta-meta **não é formalizada** em P273.17 — coerência
total com anti-padrão over-formalização explícito spec §0.
Documentada em `typst-cluster-gradient-reflexao.md` §7 e ADR-0095
§"Sub-padrão Passo administrativo XS/S criar/promover ADRs meta"
e ADR-0097 §"Relação com sub-padrão Passo administrativo".

Se algum cluster futuro terminar com mesmo padrão, então N=4 e
candidato meta-ADR formalização clara.

**A disciplina é**: documentar honestamente o que existe,
formalizar apenas o que atinge limiar com valor metodológico claro,
**resistir à tentação de formalizar tudo**.

---

## §10 — Referências

- **ADR-0091** — Gradient ColorSpace runtime + CMYK strategy (anotada
  cumulativa P273.17; décima sétima anotação consecutiva).
- **ADR-0095** (criada P273.17) — Dedup `Arc::as_ptr` resources.
- **ADR-0096** (criada P273.17) — Pattern DEBT-37 campo Layouter
  consumer-pending.
- **ADR-0097** (criada P273.17) — Scope-out reconfirmado por Fase A.
- **ADR-0093** — Meta-metodologia evolução ADRs (Pattern 2 aplicação
  prática N=13 cumulativo).
- **ADR-0094** — Meta-operacional specs (Pattern 1 cap LOC; N=12
  preserved; documental sem cap aplicado; estouro soft documental
  registado).
- **ADR-0054** — Critério fecho DEBT-1 graded (fundamento dos
  trade-offs preservados).
- **ADR-0085** — Diagnóstico imutável (vigésimo oitavo consumo).
- **ADR-0029** — Pureza física L1 (preserved absoluto; P273.17 zero
  código).
- **P156K** — Precedente "Passo administrativo XS/S criar ADRs meta"
  N=1 (ADR-0064 + 0065).
- **P271** — Precedente "Passo administrativo XS/S criar ADRs meta"
  N=2 (ADR-0093 + 0094).
- **P273.5-P273.16** — 12 sub-passos cluster Gradient anteriores
  documentados.
- **Documento reflexão** —
  `00_nucleo/diagnosticos/typst-cluster-gradient-reflexao.md`.
- **3 documentos `trabalho-previo-externo.md`** (P273.14/15/16) —
  outputs legítimos NO-GO.
- **`00_nucleo/diagnosticos/typst-passo-273-17-diagnostico.md`** —
  Fase A empírica + Decisões 1β/2α/3 cronológica + critério §A.8.
- Spec P273.17 —
  `00_nucleo/materialization/typst-passo-273-17.md`.

---

*Relatório imutável produzido em 2026-05-18. Cluster Gradient
encerrado definitivamente — 13 sub-passos consecutivos
(P273.5-P273.17) caminho mais longo documentado no projecto
cristalino; 3 ADRs meta novas formalizam sub-padrões empíricos
N=3-4 (Dedup Arc + Pattern DEBT-37 + Scope-out reconfirmado por
Fase A); documento reflexão standalone como output legível;
sub-padrões emergentes N=1-2 preserved per anti-padrão
over-formalização explícito; trabalho prévio externo documentado
para 3 scope-outs reconfirmados; ADRs vigentes 81 → 84; zero
alterações código L1/L3 (ADR-0029 preserved absoluto); 2644
baseline preservado bit-exact.*
