# Relatório do passo P206E

**Data de execução**: 2026-05-08.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-206E.md`.
**Natureza**: encerramento administrativo / passo
L0-puro com transições retroactivas (paralelo a
P204H/P205E mas com mais transições — primeira da
trajectória a alterar estado retroactivo de série
anterior).
**Sub-passo `E` da série P206** — quinto e último
(A–E).
**Magnitude planeada**: S documental.
**Magnitude real**: **S documental** (~50 min; 0
ficheiros código + 5 ficheiros docs editados + 3
outputs documentais novos).

---

## §1 O que foi feito

P206E encerrou a série P206 com 4 outputs concretos
(paralelo a P204H/P205E):

1. **Auditoria das 7 condições ADR-0075** — todas
   CUMPRIDAS (cond 7 auto-referencial fechada por este
   passo).
2. **Auditoria cond 9 ADR-0073** — etiqueta fixada:
   **CUMPRIDA com excepções** (4/6 introspection P204F
   com matches; 2/6 com excepções documentadas:
   outline-toc design intencional + cite-bibliography
   stdlib gap pre-P206).
3. **Forma de fecho P206** fixada: **Completo (final)**
   per spec C5 — `P206C.div-1` é divergência cosmética
   documentada.
4. **Transições aplicadas**:
   - **ADR-0075** PROPOSTO → **ACEITE final**.
   - **ADR-0073** "ACEITE estruturalmente fechado" →
     **"ACEITE completo retroactivo, P206E
     2026-05-08"** (Caminho B per spec C3 — fórmula
     intermediária honesta face às excepções).
   - **P204H consolidado** anotado §14 cirurgicamente
     (preservação histórica per pattern P201/P202).
   - **DEBT-53** → **ENCERRADO (CLOSED)** (vanilla
     integration materializada).
   - **DEBT-54** → **ENCERRADO (OBSOLETED)**
     (workspace setup obsoleto via vanilla CLI
     pre-built).
   - **Blueprint** actualizado §3.0ter [P206E].
5. **Relatório consolidado série P206A-E** (11 secções)
   escrito.

### Outputs concretos

#### Output 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-206E-inventario.md`.

Conteúdo:
- §1 C1 auditoria 7 condições ADR-0075.
- §2 C2 auditoria cond 9 ADR-0073.
- §3 C3 forma transição ADR-0073 (Caminho B).
- §4 C4 tratamento P204H (Caminho a — anotação
  cirúrgica).
- §5 C5 forma de fecho P206 (Completo final).
- §6 C6 DEBT fechos (CLOSED + OBSOLETED).
- §7 10 decisões durante a leitura (D1-D10).
- §8 métricas.

Tamanho: ~14 KB.

#### Output 2 — Relatório consolidado série P206

Localização:
`00_nucleo/materialization/typst-passo-206-relatorio-consolidado.md`.

Estrutura paralela a P204H/P205E consolidados — 11
secções:
- §1 Trajectória da série (P206A-E).
- §2 Divergências (`P206C.div-1`).
- §3 Outputs concretos por sub-passo.
- §4 Achados consolidados (5 lições).
- §5 Métricas agregadas.
- §6 Divergências da série.
- §7 Padrões demonstrados (7 lições).
- §8 Estado pós-série face ao snapshot M8+F3.
- §9 Convenções consolidadas (6 lições).
- §10 Não-objectivos respeitados.
- §11 Sugestão para próximo marco arquitectónico.
- §12 Cross-references.
- §13 Resumo executivo.

Tamanho: ~24 KB.

#### Output 3 — Edições cirúrgicas

Não é ficheiro discreto. Conjunto de:

- **`00_nucleo/adr/typst-adr-0075-vanilla-integration.md`**
  — Status PROPOSTO → ACEITE final; bloco "Validação
  P206A-E" adicionado com tabela das 7 condições +
  forma de fecho.
- **`00_nucleo/adr/typst-adr-0073-comemo-introspector.md`**
  — Status retroactivo "ACEITE completo retroactivo,
  P206E 2026-05-08"; bloco "Fecho retroactivo cond 9
  — P206E 2026-05-08" adicionado **no início** da ADR
  (após cabeçalho) preservando texto original do plano
  de validação + "Validação P204A-H" intactos.
- **`00_nucleo/materialization/typst-passo-204-relatorio-consolidado.md`**
  — §14 "Anotação cirúrgica P206E — Fecho retroactivo
  cond 9 (2026-05-08)" adicionado **no final**
  preservando §1-§13 intactas (per pattern P201/P202).
- **`00_nucleo/diagnosticos/blueprint-projecto.md`**
  — §3.0ter "Marca de actualização — [P206E] Vanilla
  integration + cond 9 fechada" adicionado adjacente a
  §3.0bis [P205E].
- **`00_nucleo/DEBT.md`** — DEBT-53 ENCERRADO (CLOSED)
  + DEBT-54 ENCERRADO (OBSOLETED) com histórico
  preservado em sub-secções "(Histórico) Estado
  pré-fecho".

#### Output 4 — Relatório P206E (este ficheiro)

Localização:
`00_nucleo/materialization/typst-passo-206E-relatorio.md`.

---

## §2 Tempo de execução

~50 minutos efectivos:

- ~5 min: leitura da spec + setup TaskList + contexto
  P206A-D.
- ~8 min: C1 auditoria 7 condições ADR-0075 com
  evidência empírica literal.
- ~7 min: C2 auditoria cond 9 ADR-0073 (texto literal
  + confronto matriz P206D) + análise excepções.
- ~3 min: C3 Caminho B fixado + justificação literal.
- ~3 min: C4 grep P201/P202 + Caminho a fixado.
- ~2 min: C5 Completo (final) fixado.
- ~2 min: C6 DEBT fechos identificados.
- ~5 min: C8 transição ADR-0075 (cabeçalho + bloco
  validação P206A-E).
- ~5 min: C9 transição retroactiva ADR-0073 (cabeçalho
  + bloco fecho retroactivo cond 9).
- ~3 min: C10 anotação cirúrgica P204H (§14).
- ~2 min: C12 blueprint §3.0ter.
- ~3 min: C11 DEBT-53/54 fechos em DEBT.md.
- ~10 min: C7 escrita do relatório consolidado série
  (11 secções; ~24 KB).
- ~5 min: outputs documentais (inventário + este
  relatório).
- ~2 min: C14 verificação final (cargo test +
  lab/parity + lint).

---

## §3 Métricas

| Métrica | Valor |
|---------|-------|
| Tests workspace antes (P206E) | 1873 |
| Tests workspace depois (P206E) | **1873** (∆ 0 — documental) |
| Tests lab/parity antes | 75 |
| Tests lab/parity depois | **75** (∆ 0 — documental) |
| Tests P206E novos | 0 |
| Linter violations | 0 (sem alteração) |
| ADRs editadas | 2 (0075 transitada; 0073 anotada retroactivamente) |
| ADRs novas em P206E | 0 |
| DEBTs fechadas | 2 (DEBT-53 CLOSED; DEBT-54 OBSOLETED) |
| Ficheiros docs novos | 3 (inventário + consolidado série + este relatório) |
| Ficheiros docs modificados | 5 (ADR-0075; ADR-0073; P204H consolidado §14; blueprint §3.0ter; DEBT.md DEBTs 53+54) |
| LOC novas (código) | 0 |
| LOC novas (docs) | ~5000 (consolidado é o maior; ~24 KB) |
| Cargo deps adicionados | 0 |
| Refactor mid-execution | 0 |

### Tests por crate (sem alteração)

- `typst_core`: 1584.
- `typst_infra`: 242.
- `typst_shell`: 21.
- `typst_wiring`: 2.
- (integration): 24.
- **Total workspace**: 1873.
- **Lab/parity quarentena**: 75 (52 baseline + 23
  P206 série).

---

## §4 Decisões

### D1 — `CUMPRIDA com excepções` é etiqueta honesta

C2 audita literal "sem regressões observable" + matriz
empírica. 4/6 matches limpos + 2/6 excepções
documentadas (não regressões). Etiqueta intermediária
respeita evidência sem inflar (estritamente seria
desonesto) ou sub-estimar (PARCIAL ignoraria progresso).

### D2 — Caminho B retroactivo previne falsa "completo"

Spec §8 risco "inflar transição retroactiva sem
honestidade" — evitado. Caminho B preserva nuance das
excepções no estado da ADR. Auditor futuro lê "ACEITE
completo retroactivo" + bloco "Fecho retroactivo" e
entende contexto.

### D3 — Caminho a P204H confirmado por grep empírico

P201/P202 explicitamente declaram "Modificação
retroactiva quebraria a regra de preservação".
Caminho a (anotação cirúrgica §14) é literal-aplicação
do pattern. §1-§13 preservadas; §14 adicionada
chronologicamente.

### D4 — DEBT-54 OBSOLETED é primeira aplicação formal

Pattern P206A D3 ("DEBT pode fechar via 3 caminhos:
CLOSED / REPLACED-BY / OBSOLETED") aplicado pela
primeira vez em DEBT-54. Documentado em DEBT.md +
ADR-0075 + relatório consolidado P206 §4.3.

### D5 — `P206C.div-1` cosmética não bloqueia "Completo (final)"

Per spec C5 hipótese: 7/7 condições CUMPRIDAS; div-1
é decisão arquitectural durante materialização, não
excepção do plano de validação.

### D6 — Auto-referencialidade de cond 7 ADR-0075

Cond 7 ("Cond 9 ADR-0073 fechada: P206E formaliza
transição") é auto-referencial — P206E é o sub-passo
que cumpre a condição. Pattern paralelo a P205E cond 3.

### D7 — Distinção "completo final" vs "completo retroactivo"

Etiquetas distintas: "completo final" para transições
intra-série (P205E ADR-0074); "completo retroactivo"
para transições inter-séries que afectam série
anterior (P206E ADR-0073).

### D8 — Blueprint §3.0ter chronológico

Pattern marca-por-fecho consolidado: §3.0 [P204H] +
§3.0bis [P205E] + §3.0ter [P206E]. Cada série completa
adiciona subsecção adjacente; chronologia preservada
sem reescrita.

### D9 — Sem sentinelas novas em P206E

Encerramento documental — sem código novo. Sentinelas
P206 série activas (ver §5.4 do consolidado).

### D10 — Magnitude P206 = M agregado

Trajectória P204 (L cross-modular) → P205 (M agregado)
→ P206 (M agregado). P205/P206 têm escopo similar
(refactor moderado vs integração externa); P204 era
escopo maior (paridade vanilla literal cross-modular).

---

## §5 Confronto de hipóteses do passo

A spec listou hipóteses específicas em §8:

| Hipótese | Resultado |
|----------|-----------|
| §8: "C2 = CUMPRIDA-vacuously ou CUMPRIDA com excepções" | **CONFIRMADA** — CUMPRIDA com excepções (2 documentadas) |
| §8: "C3 = Caminho B (intermediária) honesto" | **CONFIRMADA** — Caminho B fixado |
| §8: "C4 = Caminho a (anotação cirúrgica)" | **CONFIRMADA** — Caminho a confirmado por grep P201/P202 |
| §8: "C5 = Completo (final) para P206 (independente de C3)" | **CONFIRMADA** — `P206C.div-1` cosmética não bloqueia |
| §8: "Inflar transição retroactiva sem honestidade" | **EVITADO** — Caminho B preserva nuance das excepções |
| §8: "Reescrever P204H sem necessidade" | **EVITADO** — anotação §14 cirúrgica preserva §1-§13 |
| §8: "Esquecer `P206C.div-1`" | **EVITADO** — documentada em ADR-0075 + consolidado §2.1 |
| §8: "Fechar DEBT-54 como CLOSED em vez de OBSOLETED" | **EVITADO** — etiqueta exacta usada (OBSOLETED) |
| §8: "Inflar relatório consolidado com auto-elogios" | **EVITADO** — consolidado factual e literal (paralelo P204H/P205E) |

9 hipóteses resolvidas pela auditoria empírica.

---

## §6 Sugestão para próximo marco arquitectónico

P206E fechado per C15 com todos os critérios cumpridos:

- ✓ C1 auditoria 7 condições ADR-0075 (todas CUMPRIDAS).
- ✓ C2 auditoria cond 9 ADR-0073 (CUMPRIDA com
  excepções).
- ✓ C3 forma transição ADR-0073 (Caminho B).
- ✓ C4 tratamento P204H (Caminho a).
- ✓ C5 forma de fecho P206 (Completo final).
- ✓ C6 DEBT fechos (53 CLOSED + 54 OBSOLETED).
- ✓ C7 relatório consolidado escrito (11 secções).
- ✓ C8 ADR-0075 transitada.
- ✓ C9 ADR-0073 transitada retroactivamente.
- ✓ C10 P204H anotada §14.
- ✓ C11 DEBTs fechadas em registry.
- ✓ C12 blueprint §3.0ter.
- ✓ C13 sentinelas preservadas (sem novas).
- ✓ C14 verificação final passa (1873 + 75 verdes; 0
  violations).
- ✓ Inventário registado.
- ✓ Relatório escrito (este ficheiro).

**Próximo marco arquitectónico**: **escolha humana**
(per spec §5 "P206E não decide. Reporta.").

Caminhos plausíveis (não-vinculativos; documentados em
relatório consolidado §11):

1. **CLI subcomando cristalino (P207+)** — materializar
   `P206C.div-1` deferred. Magnitude L (~3-5h
   cross-modular).
2. **`Selector::Label` em L1** — extensão minimal P175.
   Magnitude S-M.
3. **Bibliography stdlib completion** — fechar gap
   cite-bibliography. Magnitude M+.
4. **Equation namespace parsing** — suporte vanilla
   `math.equation`. Magnitude S.
5. **Próximo marco não catalogado** — Model Fase 2
   (table/figure-kinds/bibliography per blueprint
   §3.2 OPÇÃO A).
6. **Pausa estratégica** — vanilla integration fechada;
   ponto natural de re-avaliar prioridades.

---

## §7 Cross-references

- **Spec**: `00_nucleo/materialization/typst-passo-206E.md`.
- **Outputs P206E**:
  - `00_nucleo/diagnosticos/typst-passo-206E-inventario.md`.
  - `00_nucleo/materialization/typst-passo-206-relatorio-consolidado.md`.
- **Edições em ADRs**:
  - `00_nucleo/adr/typst-adr-0075-vanilla-integration.md`
    (ACEITE final 2026-05-08).
  - `00_nucleo/adr/typst-adr-0073-comemo-introspector.md`
    (ACEITE completo retroactivo P206E 2026-05-08).
- **Anotação cirúrgica retroactiva**:
  `00_nucleo/materialization/typst-passo-204-relatorio-consolidado.md`
  §14 [P206E].
- **Blueprint actualizado**:
  `00_nucleo/diagnosticos/blueprint-projecto.md` §3.0ter
  [P206E].
- **DEBT registry**:
  `00_nucleo/DEBT.md` — DEBT-53 ENCERRADO (linha 785+)
  + DEBT-54 ENCERRADO (linha 557+).
- **Predecessores na série**:
  - P206A diagnóstico (`typst-passo-206A-relatorio.md`).
  - P206B harness reactivado (`typst-passo-206B-relatorio.md`).
  - P206C helpers L3 + comparação (`typst-passo-206C-relatorio.md`).
  - P206D matriz consolidada (`typst-passo-206D-relatorio.md`).
- **Predecessor série**:
  - P205E (F3 ACEITE final).
  - P204H (M8 ACEITE estruturalmente fechado; agora
    completo retroactivo).
- **Pattern referência**:
  - P204H §C7 + P205E §C4 (consolidados paralelos;
    estrutura 11 secções).
  - P201/P202 (preservação histórica via anotação
    cirúrgica).
- **ADRs vinculadas**:
  - ADR-0073 (M8; cond 9 fechada retroactivamente).
  - ADR-0074 (F3; preservada).
  - ADR-0066 (SUPERSEDED-BY 0073; preservada).
  - ADR-0054 (perfil graded; fundamenta C3=C P206C).
  - ADR-0075 (vanilla integration; ACEITE final).
- **Lab/parity quarentena** (preservada):
  - `lab/parity/src/{vanilla_invoke,structural_compare,
    query_helpers}.rs` (P206C; query_helpers em
    03_infra L3).
  - `lab/parity/tests/{vanilla_cli_smoke,
    structural_parity,consolidado_p206d}.rs`.
  - `lab/parity/SKIPS.md`.
  - `lab/parity/reports/{latest,history/*}.md`.
- **Vanilla typst v0.14.2**:
  - `/usr/local/bin/typst v0.14.2 (b33de9de)` —
    pre-built CLI ambiental.
  - `lab/typst-original/crates/typst-syntax v0.14.2` —
    path dep.
