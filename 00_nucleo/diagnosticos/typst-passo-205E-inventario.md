# P205E — Inventário interno (encerramento série F3)

**Data**: 2026-05-07.
**Spec**: `00_nucleo/materialization/typst-passo-205E.md`.
**Output 1 de 4** (paralelo a P204H §C4 estrutura).

---

## §1 C1 — Auditoria das condições de validação ADR-0074

ADR-0074 §"Plano de validação" fixou 7 condições para
transitar PROPOSTO → ACEITE (verificadas em P205E).
Auditoria literal:

| # | Condição | Estado | Evidência empírica |
|---|----------|--------|---------------------|
| 1 | P205B materializado: `SealedPositions` struct + `#[comemo::track]` impl; `Layouter::finish` retorna sealed sub-store | **CUMPRIDA** | `01_core/src/entities/sealed_positions.rs` (hash `89baeda9`); `#[comemo::track] impl SealedPositions { fn position_of(...) }` linhas 59-66; `Layouter::finish` produz `doc.extracted_positions = SealedPositions::from_runtime(self.runtime.positions)` em `01_core/src/rules/layout/mod.rs:1187-1189`; 4 tests novos (2 sentinelas + 2 unit) |
| 2 | P205C materializado: `TagIntrospector::position_of` retorna `Some(Position)` via consumer com sealed sub-store | **CUMPRIDA** | `TagIntrospector` linha 246: `pub positions: SealedPositions`; método `pub fn inject_positions(&mut self, sealed)` linhas 271-276; trait impl linha 312: `self.positions.position_of(location)`; 4 tests (3 unit `p205c_*` em `introspector.rs::tests` linhas 948-999 + 1 E2E `p205c_pipeline_layout_seal_inject_query_devolve_some` em `layout/tests.rs`) |
| 3 | P205D materializado: `SealedLabelPages` (se benefício se materializar; senão, decisão de não prosseguir documentada) | **CUMPRIDA** (branch B) | P205D fixou Caminho B em C2 (adiar). Decisão de não prosseguir documentada em: `00_nucleo/diagnosticos/typst-passo-205D-inventario.md` (6 sub-secções de inventário empírico); `00_nucleo/materialization/typst-passo-205D-relatorio.md` (relatório); ADR-0074 §P205D anotado `✅ DEFERIDO 2026-05-07` com fundamento em 5 pontos |
| 4 | Tests workspace verdes (estimativa 1852 → 1862-1870; ∆+10 a +18) | **CUMPRIDA** | 1852 (pré-P205A) → 1860 (pós-P205E) verdes; ∆+8 real (+4 P205B + +4 P205C; 0 P205D/P205E). Critério literal "verdes" cumprido. Estimativa numérica 1862-1870 foi orientadora; subestimou-se porque P205D deferred eliminou ~4-6 tests previstos para `SealedLabelPages` |
| 5 | `crystalline-lint .` 0 violations | **CUMPRIDA** | `✓ No violations found` confirmado pós-cada sub-passo (P205B, P205C, P205D) e novamente em P205E C9 (verificação final) |
| 6 | Sealing point identificado e implementado: `Layouter::finish` produz sealed sub-stores reproduzivelmente | **CUMPRIDA** | `mod.rs:1187-1189` produz `extracted_positions` 1× por iteração (incluindo iterações fixpoint TOC); pattern `from_runtime` consolidado e reutilizável |
| 7 | Consumers de `layouter.runtime.positions` directamente migrados para `Introspector::position_of` (P205C) | **CUMPRIDA-vacuously** | P205C C1.1 grep empírico confirmou zero consumers de produção lendo `layouter.runtime.positions` directamente (P204F SKIP `here-locate` impede emergência de novos). Migração formal limitada a tests E2E novos (decisão D3 P205C). Infraestrutura activa para futuros consumers de produção (DEBT-53/54 expansion) |

### §1.1 Notas de auditor

**Cond 4** — discrepância numérica: estimativa
1862-1870 vs real 1860. Análise: estimativa baseou-se
em P205B/C +4 cada + P205D +2-4 (sentinelas) + P205D
unit tests +4-6 (caminho A). Real: P205B +4, P205C +4,
P205D 0 (Caminho B), P205E 0 (documental). Total +8 vs
estimativa +10-18. Estimativa foi razoável pelo critério
da época mas subestimou Caminho B em P205D. **Critério
literal da ADR é "verdes"**; condição cumprida sem
ambiguidade.

**Cond 7** — etiqueta "CUMPRIDA-vacuously" reflecte
que migração de consumers existentes é trivialmente
satisfeita por ausência de consumers (P205C D3). Não é
falha — infraestrutura está activa via
`inject_positions`. Quando consumers reais emergirem
(DEBT-53/54 expansion ou stdlib `here()`/`locate()`
materialização), ficam migráveis sem rework.

**Cond 3** — branch B vs branch A: ADR-0074 cond 3
texto literal aceita ambas explicitamente ("se
benefício se materializar; senão, decisão de não
prosseguir P205D documentada"). P205D deferred é
**CUMPRIDA**, não PARCIAL. Distinção importante face a
P204H cond 9 PARCIAL (que tinha excepção real).

---

## §2 C2 — Forma de fecho fixada: **Completo (final)**

Justificação literal:

1. ADR-0074 §"Decisão" declarou explicitamente P205D
   como "**opcional dependendo de benefício observado
   em P205B/C**". Não fixou materialização — fixou
   **condicionalidade**.
2. Plano de validação cond 3 aceita ambas as branches
   ("se benefício se materializar; senão, decisão de
   não prosseguir P205D documentada"). P205D deferred
   com 6 sub-secções de inventário + relatório + ADR
   anotação satisfaz o branch "documentada".
3. As 7 condições obrigatórias estão **CUMPRIDAS**
   (1, 2, 4, 5, 6, 7) ou **CUMPRIDAS no branch
   condicional aceite** (3).
4. Não há condição PARCIAL ou NÃO CUMPRIDA com
   justificação não-cosmética (contraste com P204H
   cond 9 PARCIAL por DEBT-53/54).

### §2.1 Distinção face a P204H

P204H fixou "estruturalmente fechado" porque cond 9
(sanity-check vanilla observable) era PARCIAL — exigia
DEBT-53/54 que era pre-existing scope-out. A excepção
era real, não construída.

P205E fixa "Completo (final)" porque P205D deferred é
**dentro do escopo declarado** — ADR-0074 declarou
P205D condicional desde início. Não há excepção.

### §2.2 Anti-padrão evitado (per spec §8)

Spec P205E §8 alertou para risco invertido de P204H:
**inflar para "Completo" sem honestidade** se P205D
deferred for "realmente uma falha disfarçada de
condicional".

Auditoria empírica confirma que **não é falha
disfarçada**:

- ADR-0074 declarou P205D condicional **desde a sua
  produção em P205A** (não retro-edited).
- Spec P205D §1 + §8 anteviu Caminho B como hipótese
  mais provável **antes** de P205D executar.
- P205D C1 inventário em 6 sub-secções produziu
  evidência empírica concreta (zero consumers de
  produção; vanilla também não trackeia label_pages).

Etiqueta "Completo" é honesta empiricamente. Inflar para
"estruturalmente fechado" seria sob-estimar a auditoria
sem necessidade.

---

## §3 C3 — Transições fixadas

### §3.1 Transição 1 — ADR-0074

**PROPOSTO → ACEITE (final)**.

- Estado actualizado em
  `00_nucleo/adr/typst-adr-0074-...:1` ("Status:
  ACEITE final, P205E 2026-05-07").
- Bloco "Validação P205A–E — ACEITE final" adicionado
  no início (após cabeçalho) listando 7 condições com
  estado + evidência (resumo de §1 acima).
- Forma de fecho documentada: "Completo" com
  justificação literal.
- P205D deferred documentado como dentro do escopo
  declarado, não excepção.

### §3.2 Transição 2 — ADR-0066 (afirmativa)

**Adicionar anotação cirúrgica** "Pendência §C6a
fechada por F3 (P205B+C 2026-05-07)" no início do
conteúdo histórico.

Justificação per spec C6:

- ADR-0066 já tem bloco "Superseded em P204H per
  ADR-0073 ACEITE 2026-05-07" listando P204B–G
  cronológicamente.
- Adicionar F3 (P205B+C+E) **estende a cadeia**
  chronologicamente: introspection runtime adiada
  (ADR-0066) → M8 adoptou comemo (ADR-0073) → F3
  fechou §C6a (ADR-0074).
- **Útil para auditor futuro** entender que pendência
  §C6a foi fechada em série diferente (P205) — sem a
  anotação, auditor seguia trail até P204G e perdia o
  fecho real em P205B+C.
- Conteúdo histórico **não reescrito** — anotação
  cirúrgica preserva original (per padrão P201/P202).

### §3.3 Decisão sobre ADR-0073

**Não anotar**. Razões:

- ADR-0073 §C6a permanece textualmente como registo
  histórico do estado intermédio.
- F3 fecha §C6a estruturalmente sem alterar a ADR
  (per padrão de preservação histórica).
- Anotação em ADR-0074 + ADR-0066 (P205E) já
  documentam o fecho de §C6a com cross-references
  bilaterais — adicionar em ADR-0073 seria
  redundante.

---

## §4 Decisões durante a leitura

### D1 — Distinção "Completo" vs "Estruturalmente fechado" baseou-se em ADR-text

Spec §2 ("Tensão consciente entre os dois inputs")
explicitou que decisão depende de interpretação:

- "P205D condicional desde início" → deferred dentro
  do escopo → **Completo**.
- "P205D expectativa que falhou" → deferred fora do
  escopo → **Estruturalmente fechado**.

Auditoria literal de ADR-0074 (texto produzido em
P205A; texto de plano de validação cond 3) confirma a
primeira leitura. **Completo** é a etiqueta empírica
correcta. Decisão não baseou-se em preferência mas em
texto da ADR pré-existente.

### D2 — Cond 4 cumprida apesar de subestimar tests

Estimativa 1862-1870; real 1860 (∆+8 vs ∆+10-18). Spec
P205A previu margem de erro razoável; deferral de P205D
em Caminho B reduziu tests previstos. Critério literal
"verdes" cumprido sem ambiguidade. Não é falha de
auditoria — é diferença entre estimativa orientadora e
critério literal. Decisão: **cumprida**, com nota.

### D3 — Cond 7 "vacuously" reflecte realidade

Migrar consumers existentes é trivialmente satisfeito
por ausência de consumers. Etiqueta "CUMPRIDA-vacuously"
captura a realidade sem inflar (CUMPRIDA-stricto seria
desonesto se sugerisse migração activa) ou deflacionar
(NÃO CUMPRIDA seria desonesto porque infraestrutura
está activa). Pattern útil para futuras condições que
dependam de existência prévia de consumers.

### D4 — Anotação em ADR-0066, não ADR-0073

ADR-0066 é a "ancestor" do chain (introspection
runtime adiada PROPOSTO 2026-04-27); ADR-0073 é a
adopção comemo (ACEITE 2026-05-07; supersede 0066).
ADR-0066 já tem block "Superseded em P204H" listando
P204B–G — adicionar F3 é extensão natural cronológica.
ADR-0073 §C6a permanece como registo histórico — não
precisa ser editada para reflectir fecho (preservação
histórica). Decisão D4 mantém pattern P201/P202.

### D5 — Blueprint marca §3.0bis em vez de reescrever §3.0

Spec C7 pediu "edições cirúrgicas" e padrão "marca
[P205E]". Adicionar nova subsecção §3.0bis adjacente a
§3.0 [P204H] preserva ambas as marcas
chronologicamente. Reescrever §3.0 para incluir P205E
quebraria preservação histórica. Pattern: cada série
de fecho adiciona marca própria; futuras séries
adicionam §3.0ter, §3.0quater, etc.

### D6 — Distinção F3 vs M8 explicitada no blueprint

Marca §3.0bis distingue F3 como "refactor pontual
cristalino-only" face a M8 ("paridade vanilla"). Útil
para auditor futuro: M8 tem condição vanilla observable
(PARCIAL); F3 não tem condição vanilla (cristalino-only
por divergência arquitectónica). Distinção evita
expectativa errada de paridade vanilla em F3.

### D7 — Magnitude agregada série P205 vs P204

P204 série foi L cross-modular (~3-4h aggregated);
P205 série é M agregado (~2h). Diferença reflecte
escopo: P204 atacou paridade vanilla M8 (16 cláusulas
A1–A16); P205 atacou refactor cristalino-only F3 (14
cláusulas A1–A14, escopo menor explícito em P205A).
Marca §3.0bis no blueprint regista esta distinção.

---

## §5 Resumo — métricas previstas

| Métrica | Valor |
|---------|-------|
| Forma de fecho fixada | **Completo (final)** |
| Tests workspace antes (P205E) | 1860 |
| Tests workspace depois (P205E) | **1860** (sem alteração — documental) |
| Linter violations | 0 (sem alteração) |
| ADRs editadas | 2 (0074 transitada; 0066 anotada) |
| ADRs novas em P205E | 0 |
| Ficheiros modificados (docs) | 4 (ADR-0074; ADR-0066; blueprint; este inventário) |
| Ficheiros novos (docs) | 3 (este inventário; consolidado série; relatório P205E) |
| LOC novas (código) | 0 |
| Cargo deps adicionados | 0 |
| Refactor mid-execution | 0 |
