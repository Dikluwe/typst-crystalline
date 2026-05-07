# Relatório do passo P205D

**Data de execução**: 2026-05-07.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-205D.md`.
**Natureza**: documental — caminho B (adiar) fixado por
auditoria empírica.
**Sub-passo `D` da série P205** — quarto de 5 (A–E).
**Magnitude planeada**: S (se materializar) ou nula (se
adiar).
**Magnitude real**: **S documental** (~15 min; 0
ficheiros de código + 1 ADR anotada + 2 outputs
documentais).

---

## §1 O que foi feito

P205D auditou empíricamente os consumers e benefício de
trackagem de `label_pages` per ADR-0074 §P205D
(condicional). A auditoria mostrou zero benefício
observável; **Caminho B (adiar)** foi fixado em C2.

Resultado: F3 minimal **estruturalmente completo** via
P205B + P205C (`SealedPositions` + `position_of` impl
real activa via `inject_positions`). P205D **não
materializa** `SealedLabelPages` por ausência de
benefício empírico.

### Caminho escolhido: B (adiar)

- **Caminho A (materializar `SealedLabelPages`)**:
  rejeitado. Auditoria C1 demonstrou zero consumers de
  produção, vanilla não trackeia label_pages
  directamente, e materializar duplicaria informação
  já tracked por `SealedPositions` + label registry.
  Risco anti-padrão §8 da spec.
- **Caminho B (adiar)**: fixado. ADR-0074 §P205D
  anotado `✅ DEFERIDO 2026-05-07` com fundamento
  empírico literal.

### Output 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-205D-inventario.md`.

Conteúdo:
- §1 C1 inventário (6 sub-secções: 3 CONFIRMADO + 3
  AJUSTE NECESSÁRIO).
- §2 C2 caminho fixado (B) com 5 pontos de
  justificação.
- §3 (Caminho A) N/A com forma estrutural hipotética
  registada para futura referência.
- §4 6 decisões empíricas durante a leitura (D1–D6).
- §5 métricas previstas (todas zero para código).

Tamanho: ~10 KB.

### Output 2 — Relatório (este ficheiro)

### Output 3 — Alterações em código: **N/A**

Caminho B → zero alterações de código.

Apenas anotação cirúrgica em
`00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md`
§P205D: `✅ DEFERIDO 2026-05-07` + fundamento empírico
em 5 pontos.

---

## §2 Tempo de execução

~15 minutos efectivos:

- ~3 min: leitura da spec + setup TaskList + contexto
  P205B/P205C.
- ~7 min: C1 inventário empírico (6 sub-secções; greps
  consumers + leitura outline.rs + comparação trait
  vanilla vs cristalino).
- ~1 min: C2 fixar Caminho B.
- ~2 min: anotação cirúrgica ADR-0074.
- ~1 min: validação build/lint/tests (zero alterações
  de código → trabalho de salvaguarda).
- ~1 min: outputs documentais (inventário + este
  relatório).

---

## §3 Métricas

| Métrica | Valor |
|---------|-------|
| Tests workspace antes | 1860 |
| Tests workspace depois | **1860** (∆ 0 — Caminho B) |
| Tests P205D novos | 0 |
| Linter violations | 0 (sem alteração) |
| Linter warnings | 0 (sem alteração) |
| Ficheiros novos código | 0 |
| Ficheiros modificados código | 0 |
| Ficheiros novos docs | 2 (inventário + relatório) |
| Ficheiros modificados docs | 1 (ADR-0074 §P205D) |
| LOC novas (código) | 0 |
| LOC novas (docs) | ~330 (inventário + relatório + ADR
patch) |
| Cargo deps adicionados | 0 |
| Refactor mid-execution | 0 |

Distribuição tests:

- `typst_core` unit: **1584** (sem alteração).
- `typst_infra` unit: 24 (sem alteração).
- `typst_shell` unit: 21 (sem alteração).
- `typst_wiring` unit: 2 (sem alteração).
- Integration tests: 229 (sem alteração).
- **Total**: 1860 (sem alteração).

---

## §4 Decisões

### D1 — Caminho B fixado por evidência (não por preferência)

ADR-0074 §"Decisão" declarou P205D condicional. A spec
§8 antecipou Caminho B como hipótese mais provável. C1
confirmou empíricamente: zero consumers de produção;
vanilla não trackeia label_pages directamente;
materializar duplicaria info já tracked. Caminho B é
**resposta empírica honesta**, não cedência por
preguiça. Per spec C13: "Caminho B não é ramo
condicional — é resposta empírica."

### D2 — Distinção semântica `runtime.label_pages` vs `runtime.known_page_numbers`

Achado central em C1.1: `outline.rs:48` lê
`known_page_numbers` (snapshot da iteração anterior),
**não** `label_pages` (write-target durante layout
actual). Os dois fields têm semântica complementar
(separação leitura/escrita declarada em
`mod.rs:1546-1547`). Sem esta distinção, parecia haver
consumer real para tracking; a auditoria desmontou a
ilusão.

### D3 — Vanilla `Introspector` não tem `label_to_page` — implicação central

Em `lab/typst-original/crates/typst-library/src/
introspection/introspector.rs`: trait expõe `page(loc)`,
`pages(loc)`, `page_numbering(loc)`,
`page_supplement(loc)` — todos location-based, **nunca
label-based**. Rota label→page em vanilla é multi-step
(`query_label` + `position`). Cristalino já tem rota
equivalente activa via P205C (`query_by_label` +
`position_of`). Sub-store dedicado seria divergência
**negativa** (mais complexidade, sem paridade vanilla
ou benefício cristalino).

### D4 — Convergência fixpoint não-tracked não beneficiaria de sealing

`mod.rs:1575` faz HashMap equality (`==`) entre
`doc.extracted_label_pages` e `known_page_numbers` —
operação nativa não-tracked. Se `extracted_label_pages`
fosse `SealedLabelPages`, exigiria wrap/unwrap ou
método de comparison custom. Caminho A adicionaria
**fricção sobre o consumer principal** sem benefício
correspondente.

### D5 — Tipo `usize` vs `NonZeroUsize` — divergência cristalino-vanilla

Spec sugeriu `NonZeroUsize` (paridade vanilla); código
cristalino actual usa `usize`. Caminho B deixa esta
divergência intacta — não é trabalho de P205D
harmonizar, e Caminho A teria de decidir entre
preservar `usize` (cascata zero) ou migrar para
`NonZeroUsize` (cascata em `references.rs:30`,
`outline.rs`, `mod.rs:1569`). Caminho B torna a decisão
irrelevante.

### D6 — F3 minimal estruturalmente completo sem P205D

ADR-0074 §"Decisão" declarou escopo F3 minimal como
"2 sub-stores trackable: SealedPositions + (opcional)
SealedLabelPages". P205B materializou o primeiro;
P205C activou impl real via injecção; pendência
ADR-0073 §C6a fechada estruturalmente. P205D condicional
não materializado mantém F3 minimal completo per
escopo declarado — ADR-0074 transita ACEITE em P205E
sem ambiguidade.

---

## §5 Confronto de hipóteses do passo

A spec listou hipóteses específicas em §8 e §1
(critério C2):

| Hipótese | Resultado |
|----------|-----------|
| §8: C1.5 mostra zero benefício empírico observável | **CONFIRMADA** — vanilla não trackeia label_pages directamente; cristalino já tem rota tracked via SealedPositions + labels registry |
| §8: Caminho B (adiar) é resultado provável | **CONFIRMADA** — Caminho B fixado em C2 por dados, não por preferência |
| §8: Risco de inflar P205D por simetria com P205B/P205C | **EVITADO** — anti-padrão reconhecido e rejeitado em C2 |
| §8: Assumir benefício sem verificar | **EVITADO** — auditoria empírica em 6 sub-secções precedeu decisão |
| §1: P205D **condicional** per ADR-0074 | **CONFIRMADA** — P205D não obrigatório; Caminho B legítimo |

Todas as 5 hipóteses do §8 da spec resolvidas pela
auditoria empírica. A spec previu correctamente o
resultado.

---

## §6 Sugestão para próximo passo

P205D fechado per C12 com todos os critérios cumpridos:

- ✓ C1 inventário completo (6 sub-secções; 3 AJUSTE +
  3 CONFIRMADO).
- ✓ C2 caminho fixado (**B**) com justificação
  empírica em 5 pontos.
- ✓ C3–C7 (Caminho A) — N/A per spec C13.
- ✓ C8 tests workspace verdes (1860; sem alteração).
- ✓ C9 linter 0 violations.
- ✓ C10 ADR-0074 §P205D anotada (`✅ DEFERIDO
  2026-05-07`).
- ✓ C11 sentinelas — N/A (Caminho B).
- ✓ Inventário registado.
- ✓ Relatório escrito (este ficheiro).

**Próximo sub-passo**: **P205E — Encerramento + ADR
ACEITE** (per ADR-0074 plano de materialização).

P205E é magnitude S documental:

- Auditoria das condições de validação ADR-0074
  (todas cumpridas: P205B materializado ✓; P205C
  materializado ✓; P205D documentado como deferido ✓;
  tests verdes ✓; linter zero violations ✓; sealing
  point implementado ✓; consumers migrados via
  `inject_positions` ✓).
- Forma de fecho.
- ADR-0074 PROPOSTO → ACEITE.
- Blueprint anotado [P205].
- Relatório consolidado da série P205A–E.

P205D fechou estruturalmente F3 minimal sem alterações
de código — Caminho B encerrou por evidência empírica.

---

## §7 Cross-references

- **Spec**:
  `00_nucleo/materialization/typst-passo-205D.md`.
- **Outputs**:
  - `00_nucleo/diagnosticos/typst-passo-205D-inventario.md`.
- **ADR**:
  `00_nucleo/adr/typst-adr-0074-f3-layouter-substores-trackable.md`
  (§P205D ✅ DEFERIDO 2026-05-07).
- **Predecessores**:
  - P205C (`position_of` impl real via
    `inject_positions`).
  - P205B (sealing infrastructure +
    `SealedPositions`).
  - P205A (diagnóstico-primeiro de F3).
- **Sucessor planeado**: P205E (encerramento + ADR
  ACEITE).
- **Pattern referência arquitectural**: P205C
  (`SealedPositions` materialization padrão; Caminho A
  hipotético seria literal-paralelo).
- **Vanilla referência**:
  `lab/typst-original/crates/typst-library/src/
  introspection/introspector.rs:50-75` (trait com
  métodos page location-based; sem `label_to_page`).
- **Anti-padrão evitado**: P205D §8 — inflar por
  simetria.
