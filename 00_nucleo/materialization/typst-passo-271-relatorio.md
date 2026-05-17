# Relatório P271 — Meta-formalização sub-padrões metodológicos via ADR-0093 + ADR-0094 EM VIGOR

**Data**: 2026-05-17.
**Magnitude**: XS (0 LOC L1/L3/stdlib; só documentação ADRs + anotações + README).
**Cluster**: Metodologia / Specs / Documentação (administrativo).
**Tipo**: passo administrativo XS análogo P260 (ADR-0084/0085) + P268.1 (ADR-0090).
**Spec**: `00_nucleo/materialization/typst-passo-271.md`.

---

## §1 — Sumário executivo

**Cluster metodológico formalizado** via 2 meta-ADRs criadas EM VIGOR
directas (paridade P260 ADR-0084/0085 + P268.1 ADR-0090), cobrindo 5
sub-padrões empíricos atingidos durante a série P270:

- **ADR-0093 EM VIGOR** — Meta-metodologia evolução ADRs (2 padrões):
  - "ADR scope-out revogado parcialmente" N=6.
  - "Anotação cumulativa em vez de ADR nova" N=10.
  - Sub-caso complementar "Anotação cumulativa cross-ADR" N=5
    incorporado como Pattern 2.
- **ADR-0094 EM VIGOR** — Meta-operacional specs (3 padrões):
  - "Cap LOC hard vs soft explícito" N=4.
  - "Reutilização literal helpers cross-passos" N=10.
  - "Fase A com industry research proactiva" N=4.

**Marco**: cristalino consolida ferramentas operacionais para specs
futuras (caps explícitos, reutilização literal, pesquisa preventiva,
evolução ADRs via scope-out parcial + anotação cumulativa).

### Princípios preservados

- **Regra de Ouro CLAUDE.md preservada literal** — sem código L1/L3/
  stdlib alterado; cap XS 0 LOC.
- **Sem hash drift L0** — meta-ADRs metodológicas; não tocam semantic
  prompts.
- **Sem Fase A filesystem nem web research nova** — diagnóstico
  empírico é o histórico cumulativo aplicado em §1.A-§1.E spec P271 e
  embebido literal em ADR-0093/ADR-0094 §"Histórico empírico".

---

## §2 — ADR-0093 estrutura sumário

**Status**: EM VIGOR. **Data**: 2026-05-17. **Origem**: P271.

### Pattern 1 — ADR scope-out revogado parcialmente (N=6)

- **Quando aplicar**: ADR com §"scope-outs" cujos elementos são
  materializados localmente sem invalidar decisão de fundo.
- **Como aplicar**: anotar §"scope-outs" (revogado/strikethrough);
  preservar status; adicionar §"Anotação cumulativa P<passo>";
  cross-reference.
- **Quando NÃO aplicar**: revogação invalida decisão de fundo (use
  REVOGADO + ADR substituta).

**Histórico empírico** (N=6 cumulativo):

| N | Passo | ADR alvo |
|---|---|---|
| 1 | P267 | ADR-0088 §Conic |
| 2 | P269 | ADR-0088 §focal_* |
| 3 | P270 | ADR-0083 §ColorSpace runtime |
| 4 | P270.2 | ADR-0083 §DeviceCMYK PDF |
| 5 | P270.3 | ADR-0090 §Type 6 Coons |
| 6 | P270.4 | ADR-0083 §DeviceCMYK + ADR-0091 §Conic CMYK (final) |

### Pattern 2 — Anotação cumulativa em vez de ADR nova (N=10)

- **Quando aplicar**: actualização significativa preserva decisão de
  fundo; refino paramétrico; materialização condicional.
- **Como aplicar**: §"Anotação cumulativa P<passo>" no fim da
  ADR-alvo; status preserved; conteúdo motivo + alteração + helpers
  + sub-padrões; cross-reference README §passos-chave.
- **Quando NÃO aplicar**: decisão arquitectural distinta surge;
  anotação tornaria-se mais larga que ADR original.

**Histórico empírico** (N=10 consolidação clara):

| N | Passo | ADRs anotadas |
|---|---|---|
| 1 | P258.B | ADR-0070 |
| 2 | P259.B | ADR-0073 |
| 3 | P263 | ADR-0087 |
| 4 | P265 | ADR-0088 |
| 5 | P268 | ADR-0089 |
| 6 | P268.2 | ADR-0089 |
| 7 | P270 | ADR-0083 + 5 ADRs cluster (cross-ADR inaugural) |
| 8 | P270.1 | ADR-0091 + ADR-0087/0088/0089/0090 |
| 9 | P270.2 | ADR-0091 + ADR-0083 + ADR-0087/0088/0089/0090 |
| 10 | P270.4 | ADR-0092 + ADR-0091 + ADR-0083 + ADR-0089 + ADR-0054 |

### Decisão complementar — Anotação cumulativa cross-ADR (sub-caso de Pattern 2)

Sub-padrão derivado N=5 cumulativo. Cobre passo único anotando
múltiplas ADRs cluster simultâneo (P270/P270.1/P270.2/P270.3/P270.4).
Incorporado como Pattern 2 sub-caso; revisão futura se N≥5 com
complexidade distinta separa ADR-0095.

---

## §3 — ADR-0094 estrutura sumário

**Status**: EM VIGOR. **Data**: 2026-05-17. **Origem**: P271.

### Pattern 1 — Cap LOC hard vs soft explícito (N=4)

**Motivação**: lição P270 (cap "ou" L1/stdlib estouro ~6%/~60%
silencioso). P270.1 inaugurou distinção explícita.

**Formato**:

```text
| Camada | Cap hard | Cap soft | Estimativa empírica       |
|--------|----------|----------|---------------------------|
| L3     | 250 LOC  | 150 LOC  | ~80-120 (§A.X diagnóstico)|
| Testes | 35       | 25       | ~20-25                    |
```

**Histórico empírico**:

| N | Passo | Cap hard / soft (L3) | Real | Estado |
|---|---|---|---|---|
| 1 | P270.1 | 400 / 250 | ~45 | inaugural |
| 2 | P270.2 | 250 / 150 | ~138 | soft estourou ~5% |
| 3 | P270.3 | 350 / 250 | ~250 | soft no limite |
| 4 | P270.4 | 200 / 150 | ~120 | ambos respected |

### Pattern 2 — Reutilização literal helpers cross-passos (N=10)

**Motivação**: cap LOC contenção; consistência cross-passos; redução
duplicação.

**Como aplicar**: §0 princípio identifica helpers + referência passo
origem; §1 Fase A confirma `rg`; §política condição cobre gap; §3
mostra reutilização explícita.

**Histórico empírico** (N=10 consolidação clara):

| N | Passo | Helpers reutilizados |
|---|---|---|
| 1 | P265 | P263 Linear PDF emit |
| 2 | P267 | Oklab P262 em L1 Conic sample |
| 3 | P268 | Oklab N=16 P263+P265 |
| 4 | P268.2 | oklab_delta_e + interpolate_oklab P262 |
| 5 | P269 | Oklab P262/P265 em Radial focal |
| 6 | P270 | Color P257 conversões |
| 7 | P270.1 | interpolate_in_space P270 + 3 L3 templates P263/P265/P268 |
| 8 | P270.2 | dispatcher arm Cmyk P270 + 3 L3 templates P270.1 |
| 9 | P270.3 | dispatcher P270 + helpers L3 P270.1 Coons |
| 10 | P270.4 | 3 helpers Coons P270.3 + CMYK helpers P270.2 |

### Pattern 3 — Fase A com industry research proactiva (N=4)

**Motivação**: lição P268.1 + P268.1-correção (Type 1 vs Type 6 vs
Type 4 confusion custou re-trabalho). P270 inaugurou pesquisa
preventiva.

**Quando aplicar**: decisão arquitectural não-trivial; vanilla usa
abstracção opaca; bug conhecido sem causa raiz; standard permite
múltiplas estratégias.

**Histórico empírico**:

| N | Passo | Pesquisa |
|---|---|---|
| 1 | P270 | Vanilla docs + blog 2023 + issue #4422 |
| 2 | P270.2 | Vanilla CMYK + pdfkit #532 + ISO 32000-1 §7.5.7 |
| 3 | P270.3 | 9 fontes (Cairo/Inkscape/W3C/pdf.js/PDFBOX/matplotlib/Adaszewski/Typst blog/ISO) |
| 4 | P270.4 | Bug #4422 vanilla GitHub validation |

---

## §4 — Anotações cumulativas 4 ADRs + ADR-0054

5 anotações P271 paralelas adicionadas:

| ADR | Anotação P271 — conteúdo |
|---|---|
| ADR-0083 | "Scope-out revogado parcialmente" aplicado em P270/P270.2/P270.4 → ADR-0093; "Anotação cumulativa" N=10 → ADR-0093 |
| ADR-0089 | 8 anotações cumulativas P268-P270.4 são instâncias canónicas → ADR-0093 + ADR-0094 |
| ADR-0091 | Centro de aplicação dos 5 sub-padrões formalizados (4 anotações + cross-ADR + helpers + industry) → ADR-0093 + ADR-0094 |
| ADR-0092 | Aplica todos os 5 sub-padrões formalizados → ADR-0093 + ADR-0094 |
| ADR-0054 | Perfil graded DEBT-1 reforçado por mecanismos operacionais (caps + reutilização + industry + evolução ADRs) → ADR-0093 + ADR-0094 |

**L0 `entities/gradient.md` não tocado** — meta-ADRs metodológicas;
sem hash drift.

---

## §5 — README distribuição actualizada

| Métrica | Pré-P271 | Pós-P271 | Delta |
|---|---|---|---|
| Total ADRs | 79 | **81** | +2 (ADR-0093 + ADR-0094 EM VIGOR) |
| PROPOSTO | 11 | 11 | 0 |
| IDEIA | 2 | 2 | 0 |
| EM VIGOR | 33 | **35** | +2 |
| IMPLEMENTADO | 31 | 31 | 0 |
| REVOGADO | 2 | 2 | 0 |
| ADIADO | 1 | 1 | 0 |

**Tabela tabular**: 2 linhas novas (ADR-0093 + ADR-0094) inseridas
após ADR-0090. **Passos-chave**: entrada P271 adicionada (~50 linhas).

---

## §6 — Métricas finais

| Métrica | Pré-P271 | Pós-P271 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2572 | **2572** | 0 (preservados) |
| Tests novos | — | 0 | 0 |
| Lint violations | 0 | 0 | 0 |
| Hashes propagados L0 | — | 0 | 0 |
| ADRs totais | 79 | **81** | +2 EM VIGOR |
| LOC L1/L3/stdlib adicionado | — | 0 | 0 (XS) |
| Ficheiros ADR novos | — | 2 | ADR-0093 + ADR-0094 |
| Anotações cumulativas P271 | — | 5 | ADR-0083/0089/0091/0092/0054 |

### §política condições verificadas

- 1 (ADR-0093 + ADR-0094 estrutura clara; sem ambiguidade
  cross-padrão). ✓
- 2 (Crystalline-lint zero violations pós anotações). ✓
- 3 (README distribuição consistente — 79→81; EM VIGOR 33→35;
  PROPOSTO 11 + IMPLEMENTADO 31 preserved). ✓
- 4 (Tests workspace 2572 baseline preservado; sem código alterado). ✓
- 5 (Cap LOC XS 0 LOC respeitado). ✓
- 6 (Tabelas históricas §1.A-§1.E factualmente verificadas contra
  relatórios cumulativos). ✓
- 7 (Sem anotações cumulativas duplicadas — primeira aplicação P271
  em cada ADR). ✓

**7 condições §política verificadas — todas satisfeitas**.

---

## §7 — Sub-padrões aplicados + N cumulativo

| Subpadrão | N pós-P271 | Nota |
|---|---|---|
| **Passo administrativo XS criar/promover ADR** | **N=5 → N=6 cumulativo** | + P271 (P156K/P160A/P229/P254/P268.1/**P271**) |
| **Meta-formalização sub-padrões empíricos N≥4** | **N=2 → N=3 cumulativo emergente** | P260 ADR-0084/0085 + P268.1 ADR-0090 + **P271 ADR-0093/0094** |
| Auto-aplicação ADR-0065 inline | **N=15 → N=16 cumulativo** | + P271 (diagnóstico inline §"Histórico empírico" ADR-0093/0094) |
| Anotação cumulativa em vez de ADR nova | **N=10 cumulativo (formalizado!)** | P271 aplica + formaliza simultâneo (auto-referencial) |

**Marco metodológico**: sub-padrão emergente "Meta-formalização
sub-padrões empíricos N≥4" atinge N=3 — pode atingir limiar
formalização se passos futuros gerarem novas meta-ADRs (candidato
meta-meta-ADR no horizonte; não urgente).

---

## §8 — Pesquisa empírica embebida nas ADRs

P271 **não cria ficheiro diagnóstico filesystem** (paridade P260
auto-aplicação ADR-0065 inline). Diagnóstico empírico embebido:

- ADR-0093 §"Histórico empírico" — tabelas N=6 (scope-out parcial) +
  N=10 (anotação cumulativa).
- ADR-0094 §"Histórico empírico" — tabelas N=4 (cap hard/soft) +
  N=10 (reutilização) + N=4 (industry research).
- Spec P271 §1.A-§1.E — fonte literal usada para embebimento ADRs.

**Sub-padrão "Diagnóstico imutável" não incrementado** — diagnóstico
inline em ADR (sub-padrão "Auto-aplicação ADR-0065 inline" N=15 →
N=16 cumulativo cobre).

---

## §9 — Critério aceitação checklist

- [x] ADR-0093 criada EM VIGOR (`00_nucleo/adr/typst-adr-0093-meta-metodologia-evolucao-adrs.md`).
- [x] ADR-0094 criada EM VIGOR (`00_nucleo/adr/typst-adr-0094-meta-operacional-specs.md`).
- [x] ADR-0083 anotada cumulativa P271.
- [x] ADR-0089 anotada cumulativa P271.
- [x] ADR-0091 anotada cumulativa P271.
- [x] ADR-0092 anotada cumulativa P271.
- [x] ADR-0054 anotada cumulativa P271.
- [x] README tabela: 2 linhas ADR-0093 + ADR-0094 EM VIGOR adicionadas.
- [x] README distribuição: 79→81; EM VIGOR 33→35.
- [x] README passos-chave: entrada P271 (~50 linhas) adicionada.
- [x] L0 `entities/gradient.md` NÃO tocado (sem hash drift).
- [x] Lint zero violations pós P271.
- [x] Tests workspace 2572 preservados.
- [x] Cap XS 0 LOC L1/L3/stdlib respeitado.

---

## §10 — Referências

- **P260** — ADR-0084/0085 criadas EM VIGOR (precedente meta-ADR;
  pattern aplicado P271).
- **P268.1** — ADR-0090 criada EM VIGOR (precedente meta-ADR; pattern
  aplicado P271).
- **P262-P270.4** — cluster Gradient cumulativo (origem 5 sub-padrões
  formalizados P271).
- ADR-0093 — Meta-metodologia evolução ADRs (criada EM VIGOR P271).
- ADR-0094 — Meta-operacional specs (criada EM VIGOR P271).
- ADR-0083 — Color paridade (anotada cumulativa P271).
- ADR-0089 — Gradient Conic-only (anotada cumulativa P271).
- ADR-0091 — ColorSpace runtime (anotada cumulativa P271).
- ADR-0092 — Conic Coons Patches (anotada cumulativa P271).
- ADR-0054 — Perfil graded DEBT-1 (anotada cumulativa P271).
- ADR-0085 — Diagnóstico imutável (preservado; pesquisa empírica
  inline neste passo paridade P260).
- ADR-0065 — Auto-aplicação inline (N=15 → N=16 cumulativo P271).

---

*Relatório imutável produzido em 2026-05-17. Linhagem completa
preservada — cluster metodológico formalizado.*
