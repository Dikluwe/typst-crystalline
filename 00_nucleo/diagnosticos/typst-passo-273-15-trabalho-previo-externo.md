# Trabalho prévio externo necessário — P273.15 Bbox medido pós-layout (NO-GO output)

**Data**: 2026-05-18.
**Passo origem**: P273.15 (NO-GO Fase A).
**Status**: documento de **pré-requisitos** para futuro hipotético GO.
**Cluster**: Visualize / Gradient.

---

## §0 — Propósito

Este documento é o **output legítimo** da decisão NO-GO em P273.15
(per spec §A.5 + §6 workflow). Descreve o trabalho **externo a um
passo futuro** que precisaria de existir antes de P273.15 poder
voltar como GO.

P273.15 NO-GO **não é falha** — é cumprimento honesto do critério
"verificar empíricamente" registado em todos os relatórios anteriores
P273.6-P273.13. Pendência **P273.X-bis-bbox-medido-pos-layout**
permanece como item formal aberto mas com **2 pré-requisitos
identificados**.

P273.15 é a **segunda aplicação** do sub-padrão "Scope-out reconfirmado
por Fase A" inaugurado por P273.14 — distingue de P273.14 (constraints
externas) por NO-GO ser por **ausência de demanda + custo perf**.

---

## §1 — Pré-requisitos para reanálise futura GO

### 1.1 — Caso empírico concreto identificado

**Estado actual** (verificado §A.1 diagnóstico):
- **Zero casos** registados em 8 sub-passos consecutivos
  (P273.6-P273.13) onde 3γ.2.γ produziu output observable incorrecto.
- Zero tests cristalino exercitam `Content::Block { width: None,
  height: None, ... }` com gradient `relative=parent` aninhado
  expectando bbox real medida.
- Zero issues utilizador (cristalino sem issue tracker; documentos
  L0 sem queixas).

**Decisão a tomar** (fora do escopo P273.15):

1. **Aguardar caso real surgir** — utilizador (ou test E2E vanilla
   paridade) reporta output incorrecto em pipeline real. Quando
   isso acontecer, P273.15 reanálise viável como GO.
2. **Adoptar pro-actively** sem demanda registada — over-engineering
   per ADR-0054 graded; rejeitado.
3. **Comparar com vanilla** — gerar PDF cristalino + vanilla para
   pipeline com Block sem dimensions + gradient `relative=parent`;
   identificar empíricamente divergence.

**Forma do trabalho**:
- Se opção 1: aguardar reporting. Não requer trabalho activo.
- Se opção 3: estudo comparativo cristalino vs vanilla; magnitude
  S documental (sem código).

**Magnitude estimada**: variável.

### 1.2 — Decisão arquitectural sobre custo perf

**Estado actual** (verificado §A.2 diagnóstico):
- Caminho 1 (eager): `measure_content_constrained` em **TODOS os
  Blocks sem dimensions**, mesmo quando não há gradient
  `relative=parent` interno. Pior caso O(N²) onde antes era O(N).
- Caminho 2 (lazy): walker novo para detectar presença de gradient
  `relative=parent` antes de medir. Custo de implementação
  (~60-100 LOC L1) + manutenção walker.
- Caminho 3 (scope-out): zero custo. Aceito por ADR-0054 graded.

**Decisão a tomar** (fora do escopo P273.15):
- Aceitar custo Caminho 1 (perf O(N²) inaceitável).
- Aceitar impl Caminho 2 (walker novo; magnitude desproporcional
  sem demanda).
- Optimizar Caminho 1 com cache de `measure_content_constrained`
  (ADR nova; cache adicional em L1 — viola pureza física? — pode
  precisar de discussão arquitectural).
- Implementar Caminho 2 com walker minimal (não recursivo
  completo; só inspecciona top-level + Sequence children;
  trade-off precisão vs custo).

**Forma do trabalho**:
- Decisão executiva (não decisão técnica do agente automatizado).
- Se aceitar Caminho 1 ou 2: paralelo a §1.1 caso empírico.
- Se opção optimizar/minimal: ADR nova ou anotação P273.X (~S-M
  documental + impl).

**Magnitude estimada**: S-M.

---

## §2 — Critérios para reabrir P273.15 como GO futuro

Reanálise GO viável apenas quando **ambos** os itens §1 forem
resolvidos:

1. ✅ Caso empírico concreto identificado (test ou reporte
   utilizador) onde 3γ.2.γ produz output observable incorrecto.
2. ✅ Decisão executiva sobre custo perf (Caminho 1 aceito /
   Caminho 2 aceito / optimização específica decidida).

Se algum item permanece pendente — NO-GO continua a ser o resultado
correcto.

---

## §3 — Pendência registada permanente

**P273.X-bis-bbox-medido-pos-layout** permanece como **pendência
aberta cluster Visualize/Gradient** com:

- **Status**: scope-out reconfirmado por Fase A factual P273.15.
- **Pré-requisitos**: §1 acima (2 itens).
- **Reanálise**: quando §2 critérios cumpridos.
- **Decisão P273.6 §A.3 (3γ.2.γ)** preserved literal — 8 sub-passos
  sem contraproba.

**Cluster Gradient pode declarar-se feature-complete** sem este
refino. 3γ.2.γ continua como caminho para Block sem dimensions
(gradient `relative=parent` aninhado cai no fallback page_bbox
P273.5 — comportamento aceito).

---

## §4 — Sub-padrão "Scope-out reconfirmado por Fase A" N=2 cumulativo

P273.15 é a **segunda aplicação** do sub-padrão:

**Aplicações cumulativas**:
- **N=1 (P273.14)**: CMYK-ICC scope-out via NO-GO. Razão:
  constraints externas (profile licensing + crate externa).
- **N=2 (P273.15)**: Bbox medido pós-layout via NO-GO. Razão:
  ausência de demanda empírica + custo perf inaceitável.

**Padrão consolidado**: ambos os passos cumprem mecânica:
1. Fase A com inventário factual.
2. Decisão go/no-go binária.
3. NO-GO → output documento Fase A + trabalho prévio externo
   identificado.
4. Zero alterações código (output cumprido por documentação).
5. Sub-padrão cresce no N cumulativo.

**Distingue de**:
- **"Bug arquitectural intencional corrigido"** P273.12 (limitação
  fechável por refino deliberado).
- **"Refino qualitativo opcional materializado"** (sub-padrão
  GO-only; NÃO inaugurado por P273.14 nem P273.15).

**Precedente metodológico consolidado**: cluster admite que nem
toda pendência é fechável imediatamente — algumas exigem trabalho
prévio externo. Sub-padrão "Scope-out reconfirmado por Fase A"
cobre estes casos legítimos com documentação rigorosa.

**Limiar formalização N=3-4 ainda longe** — candidato meta-ADR
futuro NÃO reservado.

---

## §5 — Referências

- Spec P273.15 — `00_nucleo/materialization/typst-passo-273-15.md`.
- Diagnóstico Fase A — `00_nucleo/diagnosticos/typst-passo-273-15-diagnostico.md`.
- ADR-0091 §"Anotação cumulativa P273.6" — decisão original 3γ.2.γ
  preserved.
- ADR-0029 — Pureza física L1 (preserved; este passo não toca L1).
- ADR-0054 — Critério fecho DEBT-1 graded (NO-GO output legítimo;
  refino sem demanda é over-engineering).
- P273.14 — Sub-padrão "Scope-out reconfirmado por Fase A" inaugural
  N=1 (CMYK-ICC). P273.15 reaplica → N=2 cumulativo.
- P273.6 §A.3 — opções 3γ.2.α/β/γ documentadas; escolha 3γ.2.γ
  fundamentada.
- P273.9 — Stack/Pad usam 3γ.2.β porque sem dimensions literais é
  o caminho natural (não over-engineering).

---

*Documento imutável produzido em 2026-05-18 como output legítimo
da decisão NO-GO Fase A P273.15. Trabalho prévio externo (2 itens)
identificado; reanálise futura GO viável apenas quando §2 critérios
cumpridos. Cluster Gradient avança sem este refino — pendência
permanente registada. Sub-padrão "Scope-out reconfirmado por Fase A"
N=2 cumulativo consolidação.*
