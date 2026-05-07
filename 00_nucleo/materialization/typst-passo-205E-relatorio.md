# Relatório do passo P205E

**Data de execução**: 2026-05-07.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-205E.md`.
**Natureza**: encerramento administrativo / passo
L0-puro (paralelo a P204H).
**Sub-passo `E` da série P205** — quinto e último (A–E).
**Magnitude planeada**: S documental.
**Magnitude real**: **S documental** (~25 min; 0
ficheiros de código + 2 ADRs editadas + 1 blueprint
actualizado + 3 outputs documentais novos).

---

## §1 O que foi feito

P205E encerrou a série F3 com 4 outputs concretos
(paralelo a P204H):

1. **Auditoria das 7 condições** do plano de validação
   ADR-0074 (per spec C1) — todas CUMPRIDAS (com
   nuances "vacuously" para cond 7 e "branch B
   condicional aceite" para cond 3).
2. **Forma de fecho fixada**: **Completo (final)** per
   spec C2 — porque ADR-0074 declarou P205D condicional
   desde início (não retro-edited) e plano de validação
   cond 3 aceita explicitamente ambas as branches
   (materializar OU documentar deferral).
3. **Transições de ADR fixadas** per spec C3:
   - **Transição 1**: ADR-0074 PROPOSTO → **ACEITE
     final**. Bloco "Validação P205A–E" adicionado no
     início da ADR.
   - **Transição 2 (afirmativa)**: anotação cirúrgica
     em ADR-0066 ("Pendência §C6a fechada por F3
     P205B+C 2026-05-07") no início do conteúdo
     histórico, preservando o original.
4. **Blueprint actualizado** com marca §3.0bis
   [P205E] F3 fechado completo (paralelo a §3.0
   [P204H] M8 estruturalmente fechado).
5. **Relatório consolidado da série** P205A–E (11
   secções) escrito em
   `00_nucleo/materialization/typst-passo-205-relatorio-consolidado.md`.

### Outputs concretos

#### Output 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-205E-inventario.md`.

Conteúdo:
- §1 C1 auditoria das 7 condições com tabela + notas
  de auditor.
- §2 C2 forma de fecho ("Completo final") com
  justificação literal e distinção face a P204H.
- §3 C3 transições (ADR-0074 + ADR-0066) com decisão
  D4 sobre não-anotação de ADR-0073.
- §4 7 decisões durante a leitura (D1–D7).
- §5 resumo de métricas.

Tamanho: ~12 KB.

#### Output 2 — Relatório consolidado da série P205

Localização:
`00_nucleo/materialization/typst-passo-205-relatorio-consolidado.md`.

Estrutura paralela a P204H consolidado:

- §1 Trajectória (P205A diagnóstico; P205B–C
  implementação; P205D deferred; P205E encerramento).
- §2 Divergências detectadas e absorvidas
  (`P205A.div-1` arquitectura vanilla diferente;
  `P205A.div-2` Categoria B reduzida).
- §3 Outputs concretos por sub-passo (tabela
  referência + ficheiros novos por camada).
- §4 Achados consolidados (pendência §C6a fechada;
  divergência cristalino-vanilla documentada;
  sealing point reproduzível; P205D condicional
  honesto).
- §5 Métricas agregadas (tests 1852→1860 +8; LOC; ADRs;
  sentinelas).
- §6 Divergências da série (tabela).
- §7 Padrão demonstrado (5 lições).
- §8 Estado pós-série face ao snapshot 2026-05-05.
- §9 Convenções consolidadas (5 lições).
- §10 Não-objectivos respeitados.
- §11 Sugestão para próximo marco arquitectónico
  (não-vinculativa).
- §12 Cross-references.
- §13 Resumo executivo.

Tamanho: ~17 KB.

#### Output 3 — Edições cirúrgicas

Não é ficheiro discreto. Conjunto de:

- **`00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md`**:
  - Cabeçalho actualizado: `Status: ACEITE (final,
    P205E 2026-05-07)`.
  - Bloco "Validação P205A–E — ACEITE final" adicionado
    com tabela das 7 condições + forma de fecho.
- **`00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`**:
  - Cabeçalho data actualizada: "; 2026-05-07
    (anotação F3 fecho §C6a — P205E)".
  - Bloco "Pendência §C6a fechada por F3 (P205B+C
    2026-05-07) — anotação P205E" adicionado no início
    do conteúdo histórico, preservando original.
- **`00_nucleo/diagnosticos/blueprint-projecto.md`**:
  - Subsecção §3.0bis "Marca de actualização —
    [P205E] F3 fechado completo" adicionada após §3.0
    [P204H], chronologicamente.

#### Output 4 — Relatório do passo P205E (este ficheiro)

Localização:
`00_nucleo/materialization/typst-passo-205E-relatorio.md`.

---

## §2 Tempo de execução

~25 minutos efectivos:

- ~3 min: leitura da spec + setup TaskList + contexto
  pré-existente (relatórios P205B/C/D + ADR-0074 +
  ADR-0066 + blueprint).
- ~5 min: C1 auditoria das 7 condições com evidência
  empírica literal (tabela com referências a ficheiros
  + linhas concretas).
- ~2 min: C2 fixar "Completo (final)" com
  justificação literal de ADR-0074 cond 3.
- ~2 min: C3 fixar transições (ADR-0074 obrigatória +
  ADR-0066 afirmativa).
- ~3 min: C5 transitar ADR-0074 (cabeçalho + bloco de
  validação).
- ~2 min: C6 anotar ADR-0066 cirúrgicamente.
- ~2 min: C7 actualizar blueprint com §3.0bis [P205E].
- ~5 min: C4 escrever relatório consolidado da série
  (11 secções).
- ~1 min: outputs documentais (inventário + este
  relatório).
- ~1 min: C9 verificação final (build + tests + lint).

---

## §3 Métricas

| Métrica | Valor |
|---------|-------|
| Tests workspace antes (P205E) | 1860 |
| Tests workspace depois (P205E) | **1860** (∆ 0 — documental) |
| Tests P205E novos | 0 |
| Linter violations | 0 (sem alteração) |
| Linter warnings | 0 (sem alteração) |
| Ficheiros novos código | 0 |
| Ficheiros modificados código | 0 |
| Ficheiros novos docs | 3 (inventário + consolidado série + este relatório) |
| Ficheiros modificados docs | 3 (ADR-0074 transitada; ADR-0066 anotada; blueprint marca §3.0bis) |
| LOC novas (código) | 0 |
| LOC novas (docs) | ~1500 (consolidado é o maior; ~17 KB) |
| Cargo deps adicionados | 0 |
| Refactor mid-execution | 0 |

### Tests por crate (sem alteração)

- `typst_core` unit: 1584.
- `typst_infra` unit: 24.
- `typst_shell` unit: 21.
- `typst_wiring` unit: 2.
- Integration tests: 229.
- **Total**: 1860.

---

## §4 Decisões

### D1 — "Completo (final)" empírico, não inflado

Auditoria literal de ADR-0074 cond 3 ("se benefício se
materializar; senão, decisão de não prosseguir
documentada") confirma que P205D deferred é dentro do
escopo declarado. ADR-0074 declarou P205D condicional
desde produção P205A — não foi retro-edited para
"acomodar" deferral. Etiqueta "Completo" é honesta;
inflar para "estruturalmente fechado" seria
sob-estimar a auditoria sem necessidade. Spec §8 alertou
para risco invertido (inflar para "Completo" sem
honestidade); auditoria evidence-based evita o
anti-padrão.

### D2 — ADR-0066 anotada (cross-reference útil)

Spec C3 deu liberdade entre adicionar / não adicionar
anotação. Decisão afirmativa porque:

- ADR-0066 já tem bloco "Superseded em P204H" listando
  P204B–G chronologicamente.
- F3 (P205B+C+E) é extensão natural do mesmo trail —
  fecha §C6a estruturalmente.
- Anotação preserva chain-of-custody completo:
  introspection runtime adiada → M8 adoptou comemo →
  F3 fechou §C6a.
- Útil para auditor futuro entender que pendência
  §C6a foi fechada em série diferente (P205) — sem a
  anotação, trail acabaria em P204G.

### D3 — ADR-0073 NÃO anotada

ADR-0073 §C6a permanece textualmente como registo
histórico do estado intermédio. F3 fecha §C6a
estruturalmente sem alterar ADR-0073 (per padrão
P201/P202 de preservação histórica). Cross-references
em ADR-0074 + ADR-0066 (P205E) já documentam fecho
bilateralmente — adicionar em ADR-0073 seria
redundante.

### D4 — Blueprint §3.0bis em vez de reescrever §3.0

Adicionar nova subsecção adjacente preserva ambas as
marcas chronologicamente (§3.0 [P204H] M8; §3.0bis
[P205E] F3). Pattern: cada série de fecho adiciona
marca própria. Reescrever §3.0 para incluir P205E
violaria preservação histórica. Distinção F3 vs M8
explicitada na marca (F3 cristalino-only; M8 paridade
vanilla com excepção).

### D5 — Cond 7 "CUMPRIDA-vacuously" é etiqueta honesta

Etiqueta diferenciadora face a CUMPRIDA-stricto
(que sugeriria migração activa de consumers
pré-existentes; mas P205C confirmou zero) e NÃO
CUMPRIDA (que sugeriria infraestrutura faltante; mas
está activa via `inject_positions`). Pattern útil
para futuras condições que dependam de existência
prévia de consumers que não existem.

### D6 — Estimativa numérica vs critério literal

Cond 4 estimativa 1862-1870; real 1860. Spec
P205A produzido com estimativa razoável (∆+10-18
incluindo tests previstos para Caminho A em P205D).
Real ∆+8 reflecte deferral de P205D — não falha de
auditoria. **Critério literal "verdes" cumprido sem
ambiguidade**. Pattern: estimativa é orientadora;
critério literal é vinculativo.

### D7 — Magnitude P205 menor que P204 — explicitada

Marca §3.0bis no blueprint regista que P205 série é
M agregado (vs P204 L cross-modular) por escopo F3
mínimo declarado em ADR-0074. Distinção útil para
auditor futuro avaliar magnitude de séries análogas.

---

## §5 Confronto de hipóteses do passo

A spec listou hipóteses específicas em §1, §2 e §8:

| Hipótese | Resultado |
|----------|-----------|
| §1 "P205D era condicional desde início" → deferred dentro do escopo → "Completo" | **CONFIRMADA** — auditoria literal de ADR-0074 §"Decisão" + cond 3 confirma condicionalidade explícita |
| §2 Tensão consciente: "Completo" vs "Estruturalmente fechado" depende de auditoria | **RESOLVIDA empiricamente** — Completo per cond 3 aceita ambas as branches |
| §8 Risco "inflar para Completo sem honestidade" | **EVITADO** — ADR-text não foi retro-edited; spec P205D antecipou Caminho B; auditoria empírica confirma |
| C3 "anotação ADR-0066 útil para auditor futuro" | **CONFIRMADA** — chain-of-custody completo (0066 → 0073 → 0074); decisão afirmativa |
| C2 "todas obrigatórias cumpridas + condicional honesto = Completo" | **CONFIRMADA** — 7/7 condições com etiquetas literais |

5 de 5 hipóteses resolvidas pela auditoria empírica. A
spec previu correctamente os critérios; P205E
executou-os literalmente.

---

## §6 Sugestão para próximo passo

P205E fechado per C10 com todos os critérios cumpridos:

- ✓ C1 auditoria das 7 condições completa.
- ✓ C2 forma de fecho fixada (**Completo final**).
- ✓ C3 transições fixadas (ADR-0074 + ADR-0066
  afirmativa; ADR-0073 sem anotação).
- ✓ C4 relatório consolidado escrito (11 secções).
- ✓ C5 ADR-0074 transitada PROPOSTO → ACEITE final.
- ✓ C6 ADR-0066 anotada cirúrgicamente.
- ✓ C7 blueprint actualizado §3.0bis [P205E].
- ✓ C8 sentinelas preservadas (24 activas: 19 M8 + 5
  F3).
- ✓ C9 verificação final passa (1860 verdes; 0
  violations; build verde).
- ✓ Inventário registado.

**Próximo marco arquitectónico**: **escolha humana**
(per spec §5 "P205E não decide. Reporta.").

Caminhos plausíveis (não-vinculativos; documentados em
relatório consolidado §11):

1. **P206+ — Vanilla integration (DEBT-53/54)**.
   Caminho identificado em P204H §6 e P204F.div-1.
   Magnitude XL+. F3 infraestrutura é pré-requisito
   desbloqueado.
2. **P210+ — Refactor Categoria A/C/D do Layouter**.
   18 fields restantes do snapshot 2026-05-05.
   Magnitude variável.
3. **Próximo marco não catalogado** (Model Fase 2;
   table/figure-kinds/bibliography per blueprint §3.2
   OPÇÃO A).
4. **Pausa estratégica** — F3 fechou pendência herdada;
   ponto natural de re-avaliar prioridades.

---

## §7 Cross-references

- **Spec**: `00_nucleo/materialization/typst-passo-205E.md`.
- **Outputs P205E**:
  - `00_nucleo/diagnosticos/typst-passo-205E-inventario.md`.
  - `00_nucleo/materialization/typst-passo-205-relatorio-consolidado.md`.
- **Edições em ADRs**:
  - `00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md`
    (ACEITE final 2026-05-07 — P205E).
  - `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`
    (anotação F3 fecho §C6a 2026-05-07 — P205E).
- **Blueprint actualizado**:
  `00_nucleo/diagnosticos/blueprint-projecto.md` §3.0bis
  [P205E].
- **Predecessores na série**:
  - P205A diagnóstico (`typst-passo-205A-relatorio.md`).
  - P205B sealing infrastructure
    (`typst-passo-205B-relatorio.md`).
  - P205C `position_of` impl real
    (`typst-passo-205C-relatorio.md`).
  - P205D `label_pages` deferred
    (`typst-passo-205D-relatorio.md`).
- **Predecessor série**: P204H (M8 estruturalmente
  fechado;
  `typst-passo-204-relatorio-consolidado.md`).
- **Pattern referência**: P204H §C4 (consolidado
  paralelo; estrutura 11 secções).
- **ADRs vinculadas**:
  - ADR-0066 (SUPERSEDED-BY 0073; anotada P205E).
  - ADR-0072 (M7 fixpoint preservado).
  - ADR-0073 (M8 ACEITE; §C6a fechada por F3 sem
    editar).
  - ADR-0074 (F3 ACEITE final P205E).
