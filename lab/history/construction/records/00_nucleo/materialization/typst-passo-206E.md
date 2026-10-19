# Passo 206E — Encerramento série vanilla integration: ADR-0075 ACEITE + ADR-0073 transição retroactiva + DEBT fechos

**Série**: 206 (sub-passo `E` = encerramento da série
P206).
**Tipo**: passo L0-puro / administrativo com transições
de ADR.
**Magnitude planeada**: S documental (paralelo a P204H
/ P205E mas com mais transições — primeira da
trajectória a alterar estado retroactivo de série
anterior).
**Pré-condição**: P206A–D concluídos; 4 sub-passos
materializados; ADR-0075 mantém PROPOSTO; tests
workspace cristalino 1873 verdes; tests `lab/parity` 75
verdes; 0 violations; matriz consolidada produzida em
`lab/parity/reports/latest.md` + `history/2026-05-08-passo-206D.md`;
manifest `lab/parity/SKIPS.md` documentado;
`P206C.div-1` registada (CLI subcomando deferred);
ADR-0073 mantém ACEITE estruturalmente fechado;
ADR-0066 SUPERSEDED-BY 0073 (P204H + P205E
anotações).
**Output**: 4 ficheiros (inventário + relatório
consolidado + edições cirúrgicas + relatório do passo
P206E) — paralelo a P204H/P205E.

---

## §1 Propósito

Encerrar a série P206 com:

1. Auditoria das condições de validação de ADR-0075.
2. Forma de fecho de P206 (decisão no inventário com
   base nas condições).
3. Transições de ADR (decisão no inventário):
   - ADR-0075 PROPOSTO → ACEITE (final ou estrutural).
   - **ADR-0073 "estruturalmente fechado" →
     transição retroactiva** (forma decidida no
     inventário).
4. Fechos de DEBT:
   - DEBT-53 → CLOSED (vanilla integration
     materializada).
   - DEBT-54 → OBSOLETED (workspace setup
     desnecessário; confirmado em P206A D3).
5. Tratamento do consolidado P204H (decisão no
   inventário com base em padrão P201/P202).
6. Relatório consolidado P206 (A–E).
7. Actualização do blueprint do projecto com P206
   fechado (§3.0ter [P206E]).
8. Registo honesto da `P206C.div-1` (CLI subcomando
   deferred).

P206E respeita o padrão: começa com inventário empírico
antes de qualquer alteração.

---

## §2 Tensão consciente entre os inputs

A clarificação inicial fixou:

- **Forma da transição ADR-0073**: decisão no
  inventário com base em "como cond 9 fica cumprida
  (estritamente vs vacuously vs com excepções)".
- **Tratamento do consolidado P204**: decisão no
  inventário com base em "padrão de preservação
  histórica P201/P202".

A tensão a registar:

- P206E é o **primeiro encerramento da trajectória que
  altera retroactivamente estado de série anterior**
  (M8 via ADR-0073). P204H/P205E foram encerramentos
  em-série; P206E toca em estado fora da própria
  série.
- A decisão sobre como ADR-0073 transita depende
  empiricamente de:
  - **Cond 9 cumprida estritamente?** Se a paridade
    observable for completa via P206 matriz (todas as
    36 entradas com match cristalino vs vanilla), é
    "estritamente cumprida" → "completo (final)"
    análogo a P205E.
  - **Cond 9 cumprida vacuously?** Se algumas SKIP-feature
    impedem cobertura completa mas o que é testável
    está testado, é "CUMPRIDA-vacuously" análogo a
    cond 7 P205E → "completo (final)" possível mas
    com nota.
  - **Cond 9 cumprida com excepções?** Se há
    divergências arquitectónicas legítimas (P206C
    documentou 3) que deixam parte do escopo fora →
    fórmula intermediária ou nota explícita.
- A decisão sobre tratamento de P204H depende de:
  - Se P201/P202 tiveram precedente de transição
    retroactiva — verificar empiricamente.
  - Se a anotação retroactiva é cirúrgica (preserva
    histórico) ou reescrita (apaga histórico).
  - Pattern de "marca-por-fecho" (§3.0bis) suporta
    extensão para "fecho retroactivo de cond 9"?

A pré-fixação no inventário não é absoluta — é guidance
empírica. Auditoria literal de cond 9 + grep de
P201/P202 históricos fixa decisões em C2/C3.

---

## §3 Cláusulas de execução (sem condicionais)

### C1 — Auditoria das condições de validação de ADR-0075

Listar literalmente as condições do plano de validação
de ADR-0075 (per output de P206A C9 + ADR-0075 escrito).
Para cada uma:

- Estado actual (cumprida / parcial / não cumprida /
  cumprida-vacuously / com excepções).
- Evidência empírica (referência a sub-passo + ficheiro
  + asserção concreta).
- Notas de auditor se aplicável.

Output: tabela com etiqueta + evidência.

Hipótese específica: ADR-0075 condições são esperadas
todas CUMPRIDAS por P206B+C+D, eventualmente com
`P206C.div-1` registada como honestidade sobre CLI
subcomando deferred (não falha estrutural — divergência
cosmética).

### C2 — Auditoria de cond 9 ADR-0073 face a P206

Listar literalmente cond 9 (per ADR-0073 plano de
validação + P204H §1 PARCIAL):

- Texto literal de cond 9 ("Saída cristalino
  sanity-check vs vanilla nos 5–7 ficheiros corpus
  paridade — sem regressões observable" — confirmar em
  C1).
- Confronto com matriz P206D (20/36 INCLUDE com
  structural matches; 13 SKIP justificados; 3
  divergências arquitectónicas documentadas).

Output: análise da forma como cond 9 fica cumprida:

- **CUMPRIDA estritamente** se a literal "5–7
  ficheiros corpus paridade" for satisfeita por P206
  (matriz cobre os 5–7 introspection P204F + mais 13
  ficheiros adicionais).
- **CUMPRIDA-vacuously** se a literal expressar
  "sem regressões" e P206 demonstrou empiricamente
  ausência de regressões (matriz 20/36 reflecte
  estado, não regressão).
- **CUMPRIDA com excepções** se há divergências
  arquitectónicas legítimas (3 documentadas em P206C)
  que ficam fora do escopo de cond 9 mas dentro do
  escopo de "regressão observable".

C2 fixa **uma** etiqueta com base em literal + matriz.

### C3 — Forma da transição ADR-0073

Com base em C2, fixar:

- **Caminho A — "completo (final)"** análogo a P205E.
  Aplicável se C2 = CUMPRIDA estritamente ou
  CUMPRIDA-vacuously sem excepções.
- **Caminho B — "completo retroactivo"** fórmula
  intermediária. Aplicável se C2 = CUMPRIDA com
  excepções; marca explicitamente que o fecho
  aconteceu noutra série.
- **Caminho C — "estruturalmente fechado" preservado
  com nota de progresso parcial**. Aplicável se C2
  ainda PARCIAL (improvável — cond 9 PARCIAL deveria
  estar resolvida pelo trabalho P206).

Critério literal: empírico, não preferência.

C3 fixa **uma**.

### C4 — Tratamento do consolidado P204H

Auditoria de patterns P201/P202:

- Verificar empiricamente se P201/P202 (ou outras
  séries históricas) tiveram transições retroactivas.
- Identificar o pattern: anotação cirúrgica vs
  reescrita.
- Aplicar a P204H.

Output: decisão fixada com base em padrão histórico
empírico + literal de C3.

- **Caminho a — Anotação cirúrgica** preservando P204H
  histórico + nota de fecho retroactivo em P206E. Per
  pattern "marca-por-fecho" (§3.0bis estabelecido em
  P205E).
- **Caminho b — Reescrita** do consolidado P204H
  reflectindo fecho de cond 9. Apaga histórico;
  improvável aceitável.
- **Caminho c — Ambos** — pattern intermediário.

Critério: pattern P201/P202 + integridade histórica.

C4 fixa **uma**.

### C5 — Forma de fecho de P206

Com base em C1 + decisões cumulativas:

- **"Completo (final)"** se condições obrigatórias 100%
  cumpridas e `P206C.div-1` é divergência cosmética
  documentada (não falha estrutural).
- **"Estruturalmente fechado"** (análogo a M8) se
  alguma condição PARCIAL ou NÃO CUMPRIDA com
  justificação não-cosmética.

Hipótese específica: P206 fecha como "Completo (final)"
porque `P206C.div-1` é divergência cosmética
(satisfeita parcialmente via Caminho B em P206C).

C5 fixa **uma**.

### C6 — DEBT fechos

Lista literal:

- **DEBT-53** (vanilla integration): per P206A D3
  pattern, fechar como **CLOSED** (materializado por
  P206B+C+D).
- **DEBT-54** (vanilla workspace setup): per P206A D3,
  fechar como **OBSOLETED** (irrelevância empírica).

C6 fixa transições com referência a localização do
DEBT registry (per CLAUDE.md ou similar).

### C7 — Relatório consolidado da série P206

Localização:
`00_nucleo/materialization/typst-passo-206-relatorio-consolidado.md`.

Estrutura paralela a P204H + P205E (consolidados das
séries) com 11 secções:

1. Cabeçalho com escopo (P206A–E).
2. Trajectória da série:
   - P206A diagnóstico-primeiro com **vanilla CLI
     pre-built descoberto** (D1; divisor de águas).
   - P206B reactivar harness (sem div).
   - P206C helper L3 + comparação estrutural (com
     `P206C.div-1`).
   - P206D matriz consolidada + sentinelas.
3. Outputs concretos por sub-passo (tabela
   referência).
4. Achados consolidados:
   - Vanilla CLI 0.14.2 pre-built (DEBT-54 obsoleto
     sem código).
   - Pixel-perfect rejeitado por design (ADR-0054).
   - Pattern emergente: DEBT pode fechar via 3
     caminhos (CLOSED / REPLACED-BY / OBSOLETED).
   - Helper L3 `query_to_summary` reusável (futura
     base para CLI subcomando deferred).
   - Cond 9 ADR-0073 fechada estruturalmente via
     P206 matriz.
5. Métricas agregadas:
   - Tests workspace: 1860 → 1873 (+13 ao longo da
     série).
   - Tests `lab/parity`: 52 → 75 (+23 ao longo da
     série).
   - LOC produção, tests, documental.
   - ADRs alteradas: 0075 transitada (forma per C5);
     0073 transitada retroactivamente (forma per C3).
   - DEBTs fechadas: 53 CLOSED + 54 OBSOLETED.
   - Sentinelas: 21 workspace + 4 quarentena dedicadas
     P206 (sentinelas duplicate-path-include não
     contadas).
   - Ficheiros novos: 5 código + 1 L0 + 4 docs +
     2 reports.
6. Divergências da série (`P206C.div-1`) com causa e
   resolução.
7. Padrões demonstrados:
   - 5 sub-passos sem pre-existing breaks (P206B
     confirmou exhaustivamente; sem `P206B.div-N`).
   - Distinção workspace cristalino vs `lab/parity`
     quarentena mantida (workspace invariante em
     P206B/D; afectado em P206C).
   - Tensão entre "novo CLI cristalino" pré-fixado e
     orçamento série navegada honestamente via
     `P206C.div-1`.
   - Honestidade sobre divergências arquitectónicas
     (3 documentadas, não fixadas).
8. Estado pós-série face ao snapshot M8+F3 (1860 →
   1873 + cond 9 fechada).
9. Convenções consolidadas pela série:
   - Lição P206A D1: ferramentas pré-existentes no
     ambiente devem ser auditadas explicitamente em
     A1 antes de assumir construção.
   - Lição P206C D1+D5: `P206C.div-1` legítima quando
     pré-fixação inflaciona magnitude desproporcional;
     `P206C.div-N` cosmética não exige decisão
     humana.
   - Lição P206D D2: preservação histórica
     (`corpus_completo_p3` intacto) per pattern
     P204H/P205E.
   - Pattern emergente: DEBT pode fechar via 3
     caminhos (CLOSED / REPLACED-BY / OBSOLETED).
   - Pattern emergente: encerramento que altera
     retroactivamente série anterior é viável via
     anotação cirúrgica + nota de fecho retroactivo
     (forma decidida em C4).
10. Não-objectivos respeitados.
11. Sugestão para próximo marco arquitectónico
    (não-vinculativa).

### C8 — Transição ADR-0075

Per C5:

Se C5 = "Completo (final)":
- ADR-0075 estado actualizado de PROPOSTO para
  `ACEITE` (final).
- Adicionar bloco "Validação P206A–E" listando
  condições com etiquetas + evidência (resumo de C1).
- Documentar `P206C.div-1` como divergência cosmética
  (não excepção estrutural).

Se C5 = "Estruturalmente fechado": estado para
`ACEITE` estrutural; documentar excepção(ões).

Edição em
`00_nucleo/adr/typst-adr-0075-vanilla-integration.md`.

### C9 — Transição retroactiva ADR-0073

Per C3:

Se C3 = Caminho A: ADR-0073 estado actualizado de
"ACEITE estruturalmente fechado" para
`ACEITE (completo final, fecho retroactivo P206E
2026-05-08)`. Bloco "Cond 9 cumprida via P206 matriz"
adicionado.

Se C3 = Caminho B: estado para `ACEITE (completo
retroactivo, P206E 2026-05-08)`. Bloco distingue
explicitamente que cond 9 fechou em série diferente.

Se C3 = Caminho C: estado mantido com nota de
progresso adicional (improvável).

Edição cirúrgica em
`00_nucleo/adr/typst-adr-0073-comemo-paridade.md` (ou
nome correcto — confirmar em C1).

### C10 — Tratamento do consolidado P204H

Per C4:

Se C4 = Caminho a: anotação cirúrgica em
`00_nucleo/materialization/typst-passo-204-relatorio-consolidado.md`
com bloco "Fecho retroactivo de cond 9 — P206E
2026-05-08" adicionado no final, sem reescrever §s
existentes.

Se C4 = Caminho b: reescrita ampla do consolidado P204H
(improvável).

Se C4 = Caminho c: ambos.

### C11 — Fechos DEBT

Per C6: editar registry DEBT (localização confirmada em
C1) ou ADR registry de DEBTs:

- DEBT-53 → CLOSED 2026-05-08 com referência a
  P206A–E.
- DEBT-54 → OBSOLETED 2026-05-08 com referência a
  P206A D3 (vanilla CLI pre-built).

### C12 — Actualização do blueprint do projecto

Ficheiro:
`00_nucleo/diagnosticos/blueprint-projecto.md` (per
P204H D5 + P205E D5).

Edições cirúrgicas:

- Marca §3.0ter [P206E] adicionada adjacente a §3.0bis
  [P205E] (per pattern marca-por-fecho).
- Sub-passo final: P206E.
- Magnitude agregada real.
- Notas sobre `P206C.div-1` (cosmética) + DEBT-53/54
  fechos + transição retroactiva ADR-0073.

Sem reescrita ampla. Edições cirúrgicas com marca
`[P206E]`.

### C13 — Sentinelas

P206E não adiciona sentinelas novas (encerramento
documental). Sentinelas activas preservadas:
- 21 workspace cristalino (M8 + F3).
- 4 + 7 quarentena `lab/parity` (P206B + P206D
  dedicadas + P206C path-included duplicados; net 6).

### C14 — Verificação final

```
cargo test --workspace 2>&1 | tail -10
cargo test --manifest-path lab/parity --all-targets
crystalline-lint .
```

Critério: 1873 + 75 verdes; 0 violations.

Se algum teste falhar, é regressão acidental — recuar.

### C15 — Critério de fecho de P206E

P206E concluído quando:

- C1 auditoria das 7 condições ADR-0075 completa.
- C2 auditoria cond 9 ADR-0073 completa.
- C3 forma da transição ADR-0073 fixada.
- C4 tratamento P204H fixado.
- C5 forma de fecho P206 fixada.
- C6 DEBT fechos fixados.
- C7 relatório consolidado escrito.
- C8 ADR-0075 transitada.
- C9 ADR-0073 transitada retroactivamente.
- C10 P204H anotada (per C4).
- C11 DEBTs fechadas no registry.
- C12 blueprint actualizado (§3.0ter).
- C14 verificação final passa.
- Inventário registado.

### C16 — Sem cláusulas condicionais

C1+C2 produzem dados. C3+C4+C5+C6 fixam **uma**
alternativa cada com base em evidência. C7–C14
executam decisões fixas.

A possibilidade de Caminho A vs B vs C em C3 (e a vs b
vs c em C4) **não é ramo na spec** — é decisão fixa
baseada em evidência empírica de C2 + auditoria
P201/P202.

---

## §4 Outputs concretos

Quatro ficheiros (paralelo a P204H/P205E):

### Ficheiro 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-206E-inventario.md`.

Conteúdo:
- §1 C1 — auditoria 7 condições ADR-0075.
- §2 C2 — auditoria cond 9 ADR-0073.
- §3 C3 — forma transição ADR-0073 fixada.
- §4 C4 — tratamento P204H fixado.
- §5 C5 — forma de fecho P206 fixada.
- §6 C6 — DEBT fechos.
- §7 Decisões durante a leitura.

### Ficheiro 2 — Relatório consolidado da série

Localização:
`00_nucleo/materialization/typst-passo-206-relatorio-consolidado.md`.

Padrão dos consolidados P204H/P205E. 11 secções (per
C7).

### Ficheiro 3 — Edições cirúrgicas em ADRs, P204H consolidado, blueprint, registry DEBT

Não é ficheiro discreto. Conjunto de:

- ADR-0075 (estado + bloco validação per C8).
- ADR-0073 (estado retroactivo + bloco cond 9 per C9).
- P204H consolidado (anotação retroactiva per C10).
- Blueprint (§3.0ter [P206E] per C12).
- Registry DEBT-53 CLOSED / DEBT-54 OBSOLETED per
  C11.

### Ficheiro 4 — Relatório do passo (P206E)

Localização:
`00_nucleo/materialization/typst-passo-206E-relatorio.md`.

Conteúdo:
- O que foi feito.
- Tempo de execução.
- Decisões.
- Sugestão para próximo marco arquitectónico.

(Distinto do relatório consolidado da série — este
relatório é específico de P206E, análogo aos relatórios
individuais P206A–D.)

---

## §5 Critério de progressão para próximo marco

P206E fechado quando C15 cumprido.

Após P206E, série P206 está fechada. Vanilla integration
materializada. Cond 9 ADR-0073 fechada
estruturalmente via P206 matriz (forma exacta per C3).
DEBT-53/54 fechadas.

**Próximo passo (P207+)**: depende do que P206E revelar
e de prioridades do humano. Caminhos plausíveis:

- CLI subcomando cristalino (deferred per
  `P206C.div-1`) — passo dedicado pós-P206.
- Próximo marco arquitectónico (depende do mapa).
- Pausa estratégica.

P206E não decide. Reporta.

---

## §6 Convenções mantidas

- Sem condicionais estruturais.
- 4 outputs (paralelo a P204H/P205E).
- Inventário empírico antes de implementação.
- Localização canónica:
  `00_nucleo/diagnosticos/` para inventário;
  `00_nucleo/materialization/` para relatórios.
- Distinção fecho estrutural vs final mantida.
- Preservação histórica: P204H consolidado **não é
  reescrito** se C4 = a; é anotado.
- Pattern marca-por-fecho (§3.0ter).
- Sem inflação retórica.

---

## §7 Não-objectivos

P206E não:

- Toca em código produção workspace cristalino.
- Toca em código `lab/parity/` (P206B/C/D fechou).
- Cria ADR nova além de transições de 0075 e 0073.
- Reescreve relatórios anteriores P206A–D.
- Reescreve P204H consolidado se C4 = a (preservação
  histórica).
- Reescreve historiograma inteiro.
- Materializa CLI subcomando deferred (`P206C.div-1`)
  — fica para passo dedicado pós-P206.
- Endereça outras DEBTs além de 53/54.
- Decide próximo marco arquitectónico.
- Pré-define sub-passos pós-P206.
- Modifica trait `Introspector`, helpers, ou
  consumers.
- Materializa expectations vanilla nas companions
  `.typ.toml` (deferred per P206C D9).
- Adiciona sentinelas novas (encerramento documental).

---

## §8 Erro a não repetir

Da série P204+P205+P206A–D — pattern empírico:
inventário antes de decisão; honestidade sobre
divergências; preservação histórica.

Risco específico de P206E — primeiro encerramento da
trajectória que altera estado retroactivo de série
anterior:

1. **Inflar transição retroactiva ADR-0073 sem
   honestidade**. Se cond 9 ainda tiver excepções
   legítimas (3 divergências arquitectónicas P206C),
   "completo (final)" pode ser **inflação**. Caminho B
   ("completo retroactivo") com nota de excepções
   pode ser mais honesto. C2 audita literalmente.
2. **Reescrever P204H sem necessidade**. Pattern
   P201/P202 estabelece preservação histórica;
   reescrever consolidado P204H seria romper esse
   pattern. C4 audita empiricamente.
3. **Esquecer `P206C.div-1`** ou tratá-la como falha.
   Per relatório P206C, é divergência cosmética
   (Caminho B aceitável; CLI deferred). Não é
   excepção estrutural; mas merece registo no
   relatório consolidado para auditor futuro.
4. **Fechar DEBT-54 como CLOSED em vez de OBSOLETED**.
   Per P206A D3 pattern: CLOSED implica materialização;
   OBSOLETED implica irrelevância. DEBT-54 nunca foi
   materializada — vanilla CLI pre-built tornou
   irrelevante. Etiqueta exacta importa.

Outro risco: **inflar relatório consolidado** com
auto-elogios sobre a série. Padrão dos consolidados
(P200, P203, P204H, P205E) é literal e factual. P206E
mantém isso.

Hipótese mais provável: C2 = CUMPRIDA-vacuously ou
CUMPRIDA com excepções. C3 = Caminho B (intermediária)
honesto. C4 = Caminho a (anotação cirúrgica). C5 =
"Completo (final)" para P206 (independente de C3 que
afecta retroactivamente M8).

Mas é hipótese, não decisão. C2/C3/C4/C5 fixam-se com
base em evidência empírica.

---

## §9 Particularidade — execução

P206E é trabalho documental:

- 1 ficheiro novo (inventário ~6 KB).
- 1 ficheiro novo (relatório consolidado ~12–14 KB).
- 1 ficheiro novo (relatório P206E ~5 KB).
- Edições cirúrgicas em 4–5 ficheiros (ADR-0075 obriga;
  ADR-0073 obriga; P204H consolidado per C4;
  blueprint; registry DEBT).

Volume baixo a médio. Magnitude S documental (mas com
mais transições do que P204H ou P205E).

Pode ser executado pela sessão actual (Opus,
conversacional) ou pelo Claude Code. Sem necessidade
de exploração ampla do repositório — leitura é dos
relatórios já produzidos por P206A–D + ADR-0073/0075 +
blueprint + P204H consolidado + registry DEBT (se
existir).

Recomendado pela sessão actual se houver
disponibilidade. Padrão dos consolidados (P200, P203,
P204H, P205E) favorece sessão actual.
