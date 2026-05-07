# Passo 204H — Encerramento M8: consolidado + ADR-0073 ACEITE + ADR-0066 SUPERSEDED-BY

**Série**: 204 (sub-passo `H` = encerramento da série
M8).
**Tipo**: passo L0-puro / administrativo.
**Magnitude planeada**: S documental.
**Pré-condição**: P204A–G concluídos; 6 sub-passos
materializados; ADR-0073 mantém PROPOSTO; ADR-0066 mantém
ACEITE com nota "intermediário até M8"; tests 1852
verdes; 0 violations; 17 sentinelas activas;
`P204F.div-1` registada (vanilla integration deferred).
**Output**: 1 ficheiro consolidado + edições cirúrgicas
em ADRs e blueprint.

---

## §1 Propósito

Encerrar a série M8 com:

1. Relatório consolidado P204 (B–G).
2. Transições de ADR: ADR-0073 PROPOSTO → ACEITE (final);
   ADR-0066 ACEITE → SUPERSEDED-BY 0073.
3. Actualização do blueprint do projecto com M8 fechado.
4. Registo honesto de `P204F.div-1` no fecho — paridade
   vanilla deferred per pre-existing DEBT-53/54.

P204H respeita a convenção: começa com inventário
empírico antes de qualquer alteração.

---

## §2 Tensão consciente entre os dois inputs

A clarificação inicial fixou:

- **Forma do fecho de M8**: decisão no inventário inicial
  de P204H com base nas 9 condições de validação de
  ADR-0073.
- **Forma das transições de ADR**: ADR-0073 ACEITE final
  + ADR-0066 SUPERSEDED-BY 0073 (transições completas).

A tensão: se o inventário detectar que **não** todas as
9 condições de ADR-0073 estão cumpridas, há contradição
entre a forma do fecho (que pode legitimar M8
"estruturalmente fechado") e as transições fixas (ACEITE
final + SUPERSEDED-BY).

P204H resolve assim:

- C1 audita as 9 condições.
- C2 decide forma de fecho com base em C1.
- **Caso C1 mostre que ≥1 condição NÃO está cumprida**,
  P204H tem 2 caminhos possíveis (decididos por C3):
  - **Caminho R — Recuar** — adiar transições; abrir
    `P204H.div-N` e identificar sub-passo correctivo
    (P204I ou similar) antes de fechar.
  - **Caminho A — Aceitar parcialmente** — documentar a
    condição não cumprida como excepção justificada e
    proceder com transições. Justificação tem de ser
    explícita (não cosmética).
- **Caso C1 mostre que todas as 9 estão cumpridas**, C3
  é trivial — proceder com ACEITE final +
  SUPERSEDED-BY.

A tensão é **registada explicitamente no relatório** —
não escondida.

---

## §3 Cláusulas de execução (sem condicionais)

### C1 — Auditoria das 9 condições de ADR-0073

Listar literalmente as 9 condições do plano de validação
de ADR-0073 (per P204A diagnóstico §15 / output da spec
P204A C13). Para cada uma:

- Estado actual (cumprida / parcial / não cumprida).
- Evidência empírica (referência a sub-passo + ficheiro +
  asserção concreta).
- Notas de auditor se aplicável.

Output: tabela de 9 linhas com etiqueta CUMPRIDA /
PARCIAL / NÃO CUMPRIDA + evidência.

Hipótese específica: a condição "validação saída
cristalino == vanilla observable em corpus de paridade"
provavelmente está PARCIAL devido a `P204F.div-1`
(vanilla integration deferred). Se confirmado, C2 / C3
endereçam.

### C2 — Forma de fecho fixada com base em C1

Com base na tabela C1, fixar:

- **"Fechado completo"** se 9/9 cumpridas.
- **"Estruturalmente fechado"** (análogo a M7) se
  alguma condição PARCIAL ou NÃO CUMPRIDA com
  justificação.
- Outra etiqueta se C1 sugerir.

C2 fixa **uma** etiqueta. A escolha não é cosmética —
afecta o título do consolidado, o estado de M8 no
blueprint, e a justificação das transições de ADR em C3.

### C3 — Resolução da tensão (caminho R vs A)

Com base em C1 + C2:

- **Caminho R (Recuar)** — se C1 detectar condições
  bloqueantes para ACEITE final, adiar P204H. Registar
  divergência. P204I (ou similar) endereça antes de
  encerrar série.
- **Caminho A (Aceitar parcialmente)** — se condições
  não cumpridas forem justificáveis (DEBT pre-existente
  + decisão consciente em P204A C9 de "validação
  reduzida"), documentar como excepção e proceder.

Critério para escolha: condição não cumprida é
**bloqueante** se sub-marco depende dela para
funcionalidade observável; é **justificável** se foi
explicitamente reduzida em P204A com fundamento e M8
mantém valor estrutural sem ela.

C3 fixa **uma** alternativa.

A clarificação inicial fixou as transições como
"completas" (ADR-0073 ACEITE final + ADR-0066
SUPERSEDED-BY). Caminho A respeita isso explicitando a
excepção. Caminho R respeita isso adiando até estar
realmente possível.

### C4 — Relatório consolidado da série M8

Localização:
`00_nucleo/materialization/typst-passo-204-relatorio-consolidado.md`.

Estrutura (per padrão dos consolidados anteriores P181,
P184, P190, P192, P200, P203):

1. Cabeçalho com escopo (P204A–H).
2. Trajectória da série:
   - P204A diagnóstico-primeiro de profundidade máxima.
   - P204B–G implementação progressiva.
   - Divergências detectadas e resolvidas
     (`P204B.div-1`, `P204F.div-1`).
3. Outputs concretos por sub-passo (tabela referência).
4. Achados consolidados (per C1 + decisões em C2/C3).
5. Métricas agregadas:
   - Tests workspace: 1824 → 1852 (+28 ao longo da
     série; +24 testes M8 + outros).
   - LOC produção, tests, documental.
   - ADRs alteradas: 0073 transitada; 0066 transitada.
   - Sentinelas activas: 17 (3 + 2 + 2 + 2 + 6 + 2).
   - Ficheiros novos: ~3 (Position, measurements + L0).
6. Divergências da série (`P204B.div-1`, `P204F.div-1`)
   com causa e resolução.
7. Padrão demonstrado: 6 aplicações consecutivas de
   diagnóstico-primeiro em sub-passo de feature; 2
   divergências detectadas e absorvidas sem inflação.
8. Estado pós-série face ao snapshot 2026-05-05
   reconciliado.
9. Convenções consolidadas pela série (lições
   aprendidas):
   - Mesmo sub-passos `*B+` começam com inventário
     empírico (formalizado em P203 §9.1).
   - L4 é estritamente para wiring sem criação de tipos
     (V12 disciplina).
   - L0 prompts criados via `--fix-hashes` para
     sincronização automática.
   - Hipóteses de obstrução listadas em specs reduzem
     iteração ao detectar antecipadamente.
10. Não-objectivos respeitados.
11. Sugestão para próximo passo (não-vinculativa) —
    qual o próximo marco arquitectónico? F3 completo?
    Outro?

### C5 — Transição ADR-0073 PROPOSTO → ACEITE

Se C3 = Caminho A:

- Estado actualizado de PROPOSTO para `ACEITE`
  (final).
- Adicionar bloco "Validação P204A–G" listando as 9
  condições com etiquetas + evidência (resumo de C1).
- Documentar excepções (per C2).

Se C3 = Caminho R:

- Não modificar estado.
- Documentar bloqueio em ADR + abrir P204I.

Edição em `00_nucleo/adr/typst-adr-0073-comemo-introspector.md`.

### C6 — Transição ADR-0066 ACEITE → SUPERSEDED-BY 0073

Se C3 = Caminho A e C5 transitou ADR-0073:

- Estado de ADR-0066 actualizado para `SUPERSEDED-BY 0073`.
- Adicionar bloco no início da ADR: "Superseded em
  P204H per ADR-0073 ACEITE 2026-05-07. Conteúdo
  histórico preservado."
- Conteúdo histórico **não reescrito** — preservação
  per padrão estabelecido por P201/P202.

Se C3 = Caminho R: não modificar.

Edição em `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`.

### C7 — Actualização do blueprint do projecto

Ficheiro: `00_nucleo/projecto/blueprint-projecto.md`
(ou caminho real — confirmar em C1).

Edições cirúrgicas:

- M8 marca de estado (pendente → fechado / estruturalmente
  fechado, conforme C2).
- Sub-passo final: P204H.
- Magnitude agregada real.
- Nota sobre paridade vanilla deferred (se C2 ≠ "fechado
  completo").

Sem reescrita ampla. Edições cirúrgicas, com marca
`[P204H]` se padrão do blueprint exigir.

### C8 — Sentinelas

P204H não adiciona sentinelas novas (encerramento
documental). 17 sentinelas activas (3 + 2 + 2 + 2 + 6 +
2) preservadas.

### C9 — Verificação final

```
cargo test --workspace 2>&1 | tail -10
crystalline-lint .
```

Critério: 1852 verdes; 0 violations.

Se algum teste falhar nesta verificação, é regressão
acidental — não é trabalho de P204H endereçar (recuar
ao sub-passo onde regressão ocorreu).

### C10 — Critério de fecho de P204H

P204H concluído quando:

- C1 auditoria das 9 condições completa.
- C2 forma de fecho fixada.
- C3 caminho de resolução fixado (R ou A).
- C4 relatório consolidado escrito.
- C5 ADR-0073 transitada (ou bloqueio documentado).
- C6 ADR-0066 transitada (ou bloqueio documentado).
- C7 blueprint actualizado.
- C9 verificação final passa.
- Inventário registado.

### C11 — Sem cláusulas condicionais

C1 produz dados. C2 e C3 fixam **uma** alternativa cada.
C4–C9 executam decisões fixas.

A tensão entre "decisão no inventário" (forma do fecho)
e "transições completas" (forma das ADRs) é **resolvida
em C3** com critério explícito — não é ramo na spec, é
decisão fixa baseada em evidência empírica.

---

## §4 Outputs concretos

### Ficheiro 1 — Inventário interno

Localização:
`00_nucleo/diagnosticos/typst-passo-204H-inventario.md`.

Conteúdo:
- §1 C1 — auditoria das 9 condições (tabela 9 linhas).
- §2 C2 — forma de fecho fixada com justificação.
- §3 C3 — caminho de resolução (R ou A) com critério.
- §4 Decisões durante a leitura.

### Ficheiro 2 — Relatório consolidado da série

Localização:
`00_nucleo/materialization/typst-passo-204-relatorio-consolidado.md`.

Padrão dos consolidados anteriores. 11 secções (per C4).

### Ficheiro 3 — Edições cirúrgicas em ADRs e blueprint

Não é ficheiro discreto. Conjunto de edições em:

- `00_nucleo/adr/typst-adr-0073-comemo-introspector.md`
  (estado + bloco de validação).
- `00_nucleo/adr/typst-adr-0066-introspection-runtime-adiada.md`
  (estado + bloco de superseded).
- `00_nucleo/projecto/blueprint-projecto.md` (M8
  fechado).

### Ficheiro 4 — Relatório do passo (P204H)

Localização:
`00_nucleo/materialization/typst-passo-204H-relatorio.md`.

Conteúdo:
- O que foi feito.
- Tempo de execução.
- Decisões.
- Sugestão para próximo passo.

(Distinto do relatório consolidado da série — este
relatório é específico do sub-passo P204H, análogo aos
relatórios individuais de P204B–G.)

---

## §5 Critério de progressão para próximo marco

P204H fechado quando C10 cumprido.

Após P204H, série P204 está fechada. M8 está marcado
como concluído (estruturalmente ou completo, conforme
C2).

**Próximo passo (P205+)**: depende do que P204H revelar
sobre estado pós-série e de prioridades do humano.
Caminhos plausíveis:

- Sub-passo correctivo se C3 = R (P204I).
- Próximo marco arquitectónico (F3 completo? Outro?).
- Sub-passo dedicado para fechar `P204F.div-1` (vanilla
  integration).
- Pausa estratégica.

P204H não decide. Reporta.

---

## §6 Convenções mantidas

- Sem condicionais estruturais.
- 4 outputs (inventário + consolidado da série + edições
  + relatório do passo).
- Inventário empírico antes de implementação.
- Localização canónica:
  `00_nucleo/diagnosticos/` para inventário;
  `00_nucleo/materialization/` para relatórios.
- Distinção fecho estrutural vs final mantida.
- Preservação histórica: ADR-0066 não é reescrita; é
  anotada com bloco superseded.
- Sem inflação retórica.

---

## §7 Não-objectivos

P204H não:

- Toca em código produção.
- Cria ADR nova além das transições de 0073/0066.
- Reescreve relatórios anteriores P204B–G.
- Reescreve historiograma inteiro.
- Endereça pre-existing breaks lab/parity (DEBT-53/54
  preservados).
- Decide próximo marco arquitectónico.
- Pré-define sub-passos pós-M8.
- Modifica trait `Introspector`, Layouter, ou
  consumers.
- Adiciona ficheiros ao corpus paridade.

---

## §8 Erro a não repetir

P204H corre risco específico de **fechar M8 sem honestidade
sobre `P204F.div-1`**. A clarificação inicial fixou
transições "completas", o que pode pressionar para
declarar M8 fechado completo mesmo com vanilla integration
deferred.

C1 verifica empíricamente. C2 fixa etiqueta com base em
evidência. Se a etiqueta certa for "estruturalmente
fechado" (análogo a M7), C2 tem de a usar — não inflar
para "fechado completo" por pressão da forma das
transições.

A tensão entre as duas decisões da clarificação inicial
é resolvida pela honestidade empírica em C1, não por
acomodação.

Padrão: cada passo da série P204 detectou e absorveu
divergências sem inflar. P204H mantém esse padrão.

---

## §9 Particularidade — execução

P204H é trabalho documental:

- 1 ficheiro novo (inventário ~5 KB).
- 1 ficheiro novo (relatório consolidado ~15 KB).
- 1 ficheiro novo (relatório P204H ~5 KB).
- Edições cirúrgicas em 3 ficheiros (ADR-0073,
  ADR-0066, blueprint).

Volume baixo. Magnitude S documental.

Pode ser executado pela sessão actual (Opus,
conversacional) ou pelo Claude Code. Sem necessidade de
exploração ampla do repositório — leitura é dos
relatórios já produzidos por P204A–G + ADRs + blueprint.

Recomendado pela sessão actual se houver disponibilidade.
Padrão dos consolidados (P200, P203) favorece sessão
actual.
