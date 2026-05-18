# Relatório P273.14 — CMYK-ICC paridade (SCOPE-OUT-RECONFIRMED via NO-GO Fase A)

**Data**: 2026-05-18.
**Status**: **SCOPE-OUT-RECONFIRMED** (NO-GO outcome legítimo — não falha; não IMPLEMENTADO).
**Magnitude real**: zero LOC; 0 tests novos; documentação Fase A + trabalho prévio externo.
**Cluster**: Visualize / Gradient (quinto sub-passo na sequência terminar cluster).
**Tipo**: Fase A com decisão go/no-go binária; outcome NO-GO confirmado empíricamente.
**Spec**: `00_nucleo/materialization/typst-passo-273-14.md`.

---

## §1 — Sumário executivo

**Pendência P-Gradient-CMYK-ICC reconfirmada como scope-out
preserved** via decisão NO-GO Fase A factual:

1. **Caminho 1 (crate externa)**: requer ADR nova revogando
   invariante L0 `export.md` linha 18 "sem `crates` externas de
   PDF". Decisão arquitectural maior **fora do escopo P273.14** per
   spec §A.3.
2. **Caminho 2 (profile bytes hardcoded)**: zero profiles CMYK
   royalty-free industry-recognized para redistribuição em produto.
   Todos proprietários (Adobe SWOP / ECI FOGRA / IDEAlliance
   GRACoL); "generic CMYK no-profile" royalty-free não existe per
   ICC.org Tech Note 7 explícito.
3. **Caminho 3 (scope-out)**: ADR-0091 §"ICC profile scope-out
   preserved" decisão original P270.2 reconfirmada por evidência
   empírica P273.14.

### Marcos arquitecturais P273.14

**(1) Sub-padrão emergente inaugural "Scope-out reconfirmado por
Fase A" N=0 → N=1** — passo executado até critério go/no-go binário;
output legítimo é **documento de pendência preserved + trabalho
prévio externo identificado**. Trabalho de diagnóstico legítimo per
ADR-0054 graded.

**(2) Distingue de "Bug arquitectural intencional corrigido"
P273.12** (limitação fechável por refino deliberado vs dependente de
trabalho externo). P273.14 NÃO inaugura "Refino qualitativo opcional
materializado" (sub-padrão GO-only).

**(3) Sub-padrão "Sub-passos consecutivos do mesmo cluster" N=9 →
N=10 cumulativo emergente** — P273.5/6/7/8/9/10/11/12/13/14.
Cluster Gradient atingiu folga máxima sobre limiar formalização
N=3-4.

### Decisão fixada Fase A

1. **Decisão 1 (caminho)**: **3 — scope-out preserved**.
2. **Decisão 2 (apenas se GO)**: **N/A** — NO-GO.
3. **Decisão 3 (sempre)**: documento `trabalho-previo-externo.md`
   produzido como output do passo per §A.4 obrigação.

### Defaults preservados P262-P273.13 bit-exact

- Zero alterações código L1/L3 (ADR-0029 preserved absoluto).
- Gradient não-CMYK preserved literal.
- Gradient CMYK pré-P273.14 continua via `/DeviceCMYK` directo
  (P270.2 caminho actual).
- 2644 baseline P273.13 preserved bit-exact.

---

## §2 — Outputs do passo

### §2.1 — Diagnóstico Fase A (documento principal)

`00_nucleo/diagnosticos/typst-passo-273-14-diagnostico.md`:
- §A.1 inventário 3 caminhos (4 crates + 7 profiles + invariante L0).
- §A.2 decisão **NO-GO** fixada com fundamento literal triplicado.
- §A.3 critério para GO — não cumprido.
- §A.4 critério para NO-GO — cumprido absoluto (3 critérios).
- §A.5 análise de risco — 5 mitigações.
- §A.6 decisões fixadas — Caminho 3.
- §A.7 critério aceitação cumprido.

### §2.2 — Trabalho prévio externo (output independente NO-GO)

`00_nucleo/diagnosticos/typst-passo-273-14-trabalho-previo-externo.md`:
- §1.1 decisão arquitectural sobre invariante L0 (ADR nova).
- §1.2 profile concreto + licença (decisão executiva/legal).
- §1.3 decisão arquitectural sobre PDF size impact.
- §2 critérios para reabrir P273.14 como GO futuro.
- §3 pendência registada permanente.
- §4 sub-padrão "Scope-out reconfirmado por Fase A" N=1 inaugural.

### §2.3 — ADR-0091 anotação cumulativa (décima quarta consecutiva)

§"Anotação cumulativa P273.14 — CMYK-ICC scope-out reconfirmado
(NO-GO Fase A)":
- Razão concreta triplicada.
- Trabalho prévio externo identificado.
- ADR-0091 §"ICC profile scope-out" preserved literal.
- Sub-padrão emergente "Scope-out reconfirmado por Fase A" N=1.
- 14 sub-padrões aplicados.

### §2.4 — L0 `entities/gradient.md` anotação

Adicionada anotação P273.14 após P273.13. Hash propagado via
`crystalline-lint --fix-hashes`:
`01_core/src/entities/gradient.rs:7d2b05c8`.

---

## §3 — Sub-padrões + N cumulativo

| Subpadrão | N pós-P273.14 | Nota |
|---|---|---|
| Anotação cumulativa em vez de ADR nova | **N=20 → N=21 cumulativo consolidação clara persistente** | Décima quarta anotação consecutiva ADR-0091 |
| Reutilização literal helpers cross-passos | **N=17 preserved** | |
| Cap LOC hard vs soft explícito | **N=16 preserved** | NO-GO — sem cap aplicado |
| Aplicação meta-ADR (ADR-0093) | **N=9 → N=10 cumulativo** | Pattern 2 anotação |
| Aplicação meta-ADR (ADR-0094) | **N=12 preserved** | NO-GO — sem Pattern 1 cap |
| Pattern DEBT-37 replicado | **N=4 preserved** | |
| Template-passo replicado literal | **N=2 preserved** | |
| Sub-passos consecutivos do mesmo cluster | **N=9 → N=10 cumulativo emergente** | P273.5/6/7/8/9/10/11/12/13/14 |
| Layout duplo arquitectural aceite | **N=1 preserved** | |
| L3-only parent_bbox | **N=2 preserved** | |
| Dedup Arc::as_ptr resources | **N=3 preserved** | |
| Bug arquitectural intencional corrigido | **N=1 preserved** | |
| Triplicação Group bbox | **N=1 preserved** | |
| **Scope-out reconfirmado por Fase A** | **N=0 → N=1 inaugural emergente** | P273.14 inaugura |
| Diagnóstico imutável | **N=29 → N=30 cumulativo** | Vigésimo quinto consumo |

---

## §4 — Métricas finais

| Métrica | Pré-P273.14 | Pós-P273.14 | Delta |
|---|---|---|---|
| Tests workspace (verdes) | 2644 | **2644** | 0 (zero alterações código) |
| Tests P273.14 novos | — | 0 | NO-GO — sem materialização |
| Tests P262-P273.13 (verdes) | preserved | preserved | **0 regressões** |
| Lint violations | 0 | 0 | 0 |
| Build warnings (novos) | 0 | 0 | 0 |
| Hashes propagados L0 | — | 1 (`gradient.rs:7d2b05c8`) | +1 (anotação P273.14) |
| ADRs totais | 81 | **81** | 0 (sem nova ADR; décima quarta anotação ADR-0091) |
| LOC L3 (additions) | — | 0 | NO-GO — literal |
| LOC L1 (additions) | — | 0 | literal (ADR-0029 preserved) |
| Documentos novos | — | 2 | Diagnóstico Fase A + Trabalho prévio externo |

### §política condições verificadas

- ✓ Fase A produzida + critério §A.7 cumprido absoluto.
- ✓ ADR-0091 anotada (décima quarta consecutiva — versão NO-GO).
- ✓ Documento `trabalho-previo-externo.md` produzido (output legítimo
  per spec §A.4 obrigação).
- ✓ Zero alterações código L3/L1.
- ✓ Tests workspace 2644 preserved bit-exact (sem mudança).
- ✓ Lint zero preserved.
- ✓ Hash L0 propagado.
- ✓ ADR-0029 pureza física L1 preserved (absoluto — zero código).
- ✓ ADR-0054 graded — NO-GO é cumprimento honesto, não falha.
- ✓ Sub-padrão "Scope-out reconfirmado por Fase A" N=1 inaugural.

**10 condições §política verificadas — 10 satisfeitas absolutas**
per ADR-0094 Pattern 1 (NO-GO outcome) e §6 spec workflow.

---

## §5 — Verificação regressão zero P262-P273.13

**2644 baseline preservado bit-exact**:

- typst-core: 2179 preserved.
- typst-shell: 24 preserved.
- typst-infra: 418 preserved.
- typst-wiring + bins + outros: 23 preserved.

**Total: 2644 → 2644 preserved** (NO-GO — zero alterações código;
apenas anotações ADR + L0 + documentação Fase A + trabalho prévio
externo).

**2 testes typst-core skipped** (stack overflow pré-existente) —
NÃO regressão P273.14.

---

## §6 — Pendências preservadas pós-P273.14

Inalteradas vs P273.13:

- **ADR-0055bis variant-aware fonts** (M).
- **P-Footnote-N** (M).
- **DEBT-33 Bézier bbox** (S+M).
- **Stroke\<Length\> / Curve / Polygon** (S+M).
- **Tiling activação** (Paint::Tiling).
- **Outro cluster — saída Visualize/Gradient**.

**P-Gradient-CMYK-ICC permanece** como pendência formal aberta com
**trabalho prévio externo identificado** (3 pré-requisitos em
documento dedicado). Cluster Gradient **pode declarar-se
feature-complete sem este refino**.

Sequência para fechar cluster Gradient pós-P273.14 NO-GO:

- ✓ P273.5/6/7/8/9/10/11/12/13 (fechados).
- ✓ **P273.14** — CMYK-ICC paridade (este passo; **NO-GO
  SCOPE-OUT-RECONFIRMED**).
- **P273.15** — Bbox medido pós-layout (M).
- **P273.16** — Bbox.y topo-exacto inline (M-L; **BLOQUEADO** por
  DEBT-56).

Pendência descoberta P273.13 §9 preserved:
- **P273.X-bis-helper-group-bbox** — extract helper compartilhado
  3 sítios.

**Predição revisada**: P273.14 confirma que cluster Gradient pode
fechar **sem materializar CMYK-ICC** — `/DeviceCMYK` actual é
caminho aceito por scope-out reconfirmado. Cluster avança para
P273.15.

---

## §7 — Limitações conscientes P273.14

- `/DeviceCMYK` continua a ser ColorSpace para gradient CMYK —
  interpretação device-dependent preserved (variável entre PDF
  viewers).
- PDF/A compliance fica como pendência inalterada (PDF/A exige
  ColorSpace ICC-based para profissional CMYK).
- Trabalho prévio externo documentado em §1 do
  `trabalho-previo-externo.md` — 3 pré-requisitos para futuro
  hipotético GO (ADR + profile + licença + decisão PDF size).
- NO-GO **não é falha** — é cumprimento honesto do critério
  "verificar Fase A se krilla API existe" registado em todos os
  relatórios anteriores P273.10/11/12/13.

---

## §8 — Marco final P273.14

**P-Gradient-CMYK-ICC pendência reconfirmada como scope-out
preserved**:

- Fase A factual: 3 caminhos inventariados (crate + profile
  hardcoded + scope-out).
- Decisão NO-GO triplamente fundamentada (Caminho 1 ADR
  pré-requisito; Caminho 2 profile inexistente; Caminho 3
  reconfirmado).
- Zero código L1/L3 alterado — `/DeviceCMYK` actual preservado.
- Trabalho prévio externo documentado: 3 pré-requisitos para futuro.
- 2644 baseline P273.13 preserved bit-exact.

Sub-padrão **"Scope-out reconfirmado por Fase A" N=0 → N=1
inaugural emergente** — passo executado até critério binário;
outcome NO-GO honesto é output legítimo per ADR-0054 graded.
**Distingue de**:
- "Bug arquitectural intencional corrigido" P273.12 (limitação
  fechável vs dependente de trabalho externo).
- "Refino qualitativo opcional materializado" sub-padrão GO-only
  **não inaugurado** por P273.14.

Sub-padrão **"Sub-passos consecutivos do mesmo cluster" N=9 → N=10
cumulativo emergente** (P273.5-P273.14). Cluster Gradient atingiu
folga máxima sobre limiar formalização N=3-4 — consolidação total
do padrão preserved.

Cluster Gradient feature-complete + qualitativo + refino estrutural
extensivamente encerrado + cleanup intra-cluster + dedup bbox-aware
+ render real Groups + **CMYK-ICC scope-out reconfirmado por Fase
A** — próximo passo: **P273.15** Bbox medido pós-layout (M;
materialização ou NO-GO consoante Fase A).

---

## §9 — Referências

- ADR-0091 — ColorSpace runtime + CMYK strategy (anotada cumulativa
  P273.14; décima quarta anotação consecutiva).
- ADR-0093 — Meta-metodologia evolução ADRs (Pattern 2 aplicação
  prática N=10 cumulativo).
- ADR-0094 — Meta-operacional specs (Pattern 1 cap LOC; N=12
  preserved; NO-GO sem cap aplicado).
- ADR-0085 — Diagnóstico imutável (vigésimo quinto consumo).
- ADR-0029 — Pureza física L1 (preserved absoluto; P273.14 zero
  código).
- ADR-0054 — Critério fecho DEBT-1 (graded — NO-GO output legítimo;
  scope-out reconfirmado).
- ADR-0019 — precedente "autorização de crates externas para domínio
  específico" (ttf-parser/rustybuzz). Referência para futuro ADR
  hipotético sobre ICC crate.
- L0 `00_nucleo/prompts/infra/export.md` linha 18 — invariante
  "sem crates externas de PDF" preserved literal.
- P270.2 + ADR-0091 §"ICC profile scope-out" — decisão original
  reconfirmada por P273.14.
- ICC.org Tech Note 7 — ausência design industry de profile CMYK
  genérico royalty-free.
- `00_nucleo/diagnosticos/typst-passo-273-14-diagnostico.md` — Fase
  A empírica + decisão NO-GO + critério §A.7 cumprido.
- `00_nucleo/diagnosticos/typst-passo-273-14-trabalho-previo-externo.md`
  — 3 pré-requisitos para reanálise futura GO.
- Spec P273.14 — `00_nucleo/materialization/typst-passo-273-14.md`.

---

*Relatório imutável produzido em 2026-05-18. Pendência
P-Gradient-CMYK-ICC reconfirmada como scope-out preserved via NO-GO
Fase A factual; sub-padrão "Scope-out reconfirmado por Fase A"
N=0 → N=1 inaugural emergente; "Sub-passos consecutivos do mesmo
cluster" N=10 cumulativo emergente — cluster Gradient atinge folga
máxima sobre limiar formalização N=3-4.*
