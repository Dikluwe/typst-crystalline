# Passo 201 — Revisão do projecto: historiograma actualizado + auditoria do delta

**Série**: 201 (passo **L0-puro / administrativo**;
actualização do historiograma desde a última revisão geral
+ auditoria do delta de estado do projecto).
**Precondição**: M5+M6+M7+M9 fechados (M7 estruturalmente);
P192B encerrado; 1.802 tests verdes; 0 violations; 7 ADRs
ACEITES no ciclo M5/M6/M7; 33 aplicações consecutivas de
diagnóstico-primeiro; historiograma anterior gerado em
P156A (referência ~2026-04-26).

**Numeração**: 201. Sem sufixo `A` — não é
diagnóstico-primeiro de feature. É revisão de processo,
análoga a P156A. **Não bloqueia** o início de M8 (próximo
passo de feature será P202 ou seguinte, fixado **depois**
desta revisão com base no que ela revelar).

**Não é P193A nem P201A**: a tentativa anterior de
estruturar P193A com auditoria + diagnóstico M8 + plano
`*B-G` foi um **erro de percepção** — definir múltiplos
sub-passos de uma vez quebra o padrão diagnóstico-primeiro.
Cada sub-passo `*B+` deve ser fixado a partir do output
empírico do anterior, não antecipado.

**Natureza**: passo **L0-puro / administrativo**. **Zero
código**. **Zero testes**. **Zero ADRs criadas**. **Zero
DEBTs criados**. Outputs são documentos markdown.

---

## §1 Contexto

P156A produziu o primeiro historiograma do projecto. Desde
então:

- Passos P157–P200 executados (~44 passos, número exacto a
  confirmar no diagnóstico).
- Marcos M5, M6, M7, M9 encerrados (M7 estruturalmente).
- 7 ADRs novas no ciclo (0066–0072).
- Padrão diagnóstico-primeiro aplicado 33 vezes consecutivas.
- `CounterStateLegacy` eliminado (P190I).
- Walk fn signature redesenhada (ADR-0071).
- 2 loops fixpoint complementares operacionais.

O historiograma de P156A está **desactualizado**. Decisões
sobre M8 dependem de saber:

1. Quais padrões empíricos novos emergiram desde P156A.
2. Quais padrões anteriores se mantiveram, evoluíram ou
   quebraram.
3. Quais passos recuaram ou foram reformulados desde P156A.
4. Estado consolidado pré-M8 com a mesma forma do snapshot
   2026-05-05 mas verificado empíricamente face ao
   repositório actual.

P201 produz isso.

---

## §2 Objectivo literal

Dois outputs separados, com propósitos distintos:

### Output 1 — `historiograma-passos.md` actualizado

Substitui (não sobrescreve sem backup) o ficheiro produzido
em P156A. Cobre **todos os passos** do projecto, da origem
ao P200, com:

- Linha temporal completa (todos os passos, com
  data/etiqueta/magnitude/output principal).
- Padrões agregados detectados (incluindo padrões novos do
  ciclo P157–P200).
- Análise comparativa face ao historiograma de P156A:
  padrões mantidos, evoluídos, novos, abandonados.
- Passos que recuaram ou foram reformulados.
- Dependências entre passos (ex: P190G/H/I como cadeia,
  P191A→B→C como diagnóstico→prova→aceite).

### Output 2 — `auditoria-delta-P156A-P200.md`

Auditoria focada **apenas no delta** desde P156A. Não
substitui nada. É o ficheiro de leitura rápida ("o que
mudou") complementar ao historiograma completo (referência
exaustiva).

Cobre:

- Lista de passos executados desde P156A (P157–P200).
- ADRs novas (0061–0072 e qualquer entre P156A e o ciclo M8).
- Marcos fechados desde P156A.
- Padrões novos detectados (ex: pattern ADR-0069 com 5
  variantes, pattern Layouter-runtime → struct dedicada).
- Métricas comparadas (LOC, tests, ADRs ACEITES, padrão
  diagnóstico-primeiro contagem).
- Lacunas residuais e o seu estado.

---

## §3 Particularidade — execução por LLM externa

P201, tal como P156A, é executado pelo **Claude Code**
lendo directamente:

- Todos os ficheiros em `00_nucleo/materialization/` (155+
  relatórios actuais; quantidade exacta a confirmar).
- ADRs em `00_nucleo/adr/` (0001 a 0072).
- Diagnósticos em `00_nucleo/diagnosticos/`.
- Snapshots de transição existentes (incluindo o de
  2026-05-05).
- Historiograma anterior gerado por P156A.

Spec de P201 é o **guião**. Claude Code segue-o.

A sessão actual (Opus, modo conversacional) **não escreve
o historiograma**. Escreve apenas o spec deste passo.

---

## §4 Cláusulas a verificar pelo Claude Code

### C1 — Linha temporal completa

Listar todos os passos da origem ao P200 com:

- Identificador (P0, P1, …, P200).
- Data ou referência temporal.
- Etiqueta sumária (uma linha).
- Magnitude registada (S / M / M+ / L / L cross-modular).
- Output principal (ficheiro produzido).
- Marco que fechou ou avançou.

Critério: nenhum passo omisso. Se um identificador estiver
ausente do `00_nucleo/materialization/`, registar como
"buraco de numeração" com nota.

### C2 — Padrões agregados

Identificar os padrões empíricos do projecto. Esperados (do
snapshot 2026-05-05) mas a confirmar:

- Diagnóstico-primeiro / L0-puro / `*A` zero código tocado.
- Pattern ADR-0069 com 5 variantes operacionais.
- Pattern ADR-0070 — eliminação de write paralelo.
- Pattern ADR-0071 — walk pipeline com Introspector.
- Pattern Layouter-runtime → struct dedicada.
- Pattern auditoria sobre estado existente (P192A).
- Pattern substitution-with-fallback (M4-residual).
- Pattern auditor #1 (ajustar fixture vs violar restrição).

Para cada padrão: data de emergência, número de aplicações,
estado actual (activo / dormente / abandonado).

Padrões novos não esperados pela spec mas detectados
empíricamente devem ser listados explicitamente como
"padrões emergentes não-antecipados".

### C3 — Comparação com historiograma de P156A

Para cada padrão registado em P156A, classificar como:

- **Mantido** — segue activo com as mesmas características.
- **Evoluído** — mudou de forma; descrever a mudança.
- **Abandonado** — deixou de ser aplicado; descrever razão.
- **Não aplicável** — só fazia sentido na fase coberta por
  P156A.

E o inverso: padrões novos que P156A não tinha como
detectar.

### C4 — Passos que recuaram ou foram reformulados

Detectar:

- Passos com sufixo de divergência (`P-X.div-N`).
- Passos que foram refeitos (ex: P-X seguido de P-X' ou
  P-Y que substitui P-X).
- ADRs revogadas ou substituídas por outras.
- Decisões de spec que foram revertidas dentro de um
  sub-passo posterior.

Critério: cada caso documentado com identificador, razão,
data.

### C5 — Dependências entre passos

Para passos com cadeia clara (ex: P190G/H/I, P191A/B/C,
P195D→P196B→P200B), registar a cadeia explicitamente.

Não inventar dependências onde não há evidência. Cadeia só
existe se relatório do passo posterior cita o anterior como
pré-requisito.

### C6 — Estado consolidado pré-M8

Validar empíricamente o snapshot 2026-05-05:

- 7 ADRs ACEITES — verificar estado em cada ficheiro.
- 1.802 tests verdes — `cargo test 2>&1 | tail -5`.
- 0 violations — `crystalline-lint .`.
- `CounterStateLegacy` eliminado — `grep -rn` deve dar
  vazio fora de `lab/` e relatórios históricos.
- Walk fn 7 parâmetros — verificar assinatura actual.
- 2 loops fixpoint — verificar localização e MAX=5.
- 33 aplicações de diagnóstico-primeiro — confirmar
  contagem reconstruindo a lista.
- `comemo` em uso — verificar versão e pontos de uso.
- `Introspector` 20 métodos — contagem verificada.
- `TagIntrospector` 9 sub-stores — listagem verificada.
- `Layouter` 19 fields, sem `counter` — verificado.

Cada item: etiqueta CONFIRMADO / DIVERGÊNCIA / NÃO
APLICÁVEL com evidência (output do comando ou citação).

### C7 — Métricas cumulativas

Reportar:

- Total de passos executados (P0 a P200).
- Total de ADRs criadas.
- Total de ADRs ACEITES.
- LOC produção líquido cumulativo (com baseline e ponto
  actual).
- Tests workspace baseline P156A vs actual.
- Aplicações de diagnóstico-primeiro: contagem antes de
  P156A vs ciclo desde P156A vs total.

### C8 — Lacunas residuais

Lista actual:

- #1 (Position) — residual.
- #1b (Position-related) — residual.
- #2 (Counter at locations) — residual.
- #3 (headings_for_toc) — fechada P200B.

Para cada lacuna: estado, último passo que a tocou, se
bloqueia M8 ou não.

### C9 — Convenções estabelecidas

Listar as convenções activas (com data de estabelecimento
quando rastreável):

- Sub-passo `.A` = auditoria L0.
- Sub-passos `*A` = diagnóstico-primeiro.
- Sub-passos `*B+` = implementação sem condicionais.
- 3 outputs padrão por passo.
- Sem código Rust nas specs.
- Distinção fecho estrutural vs arquitectural.
- Preservação histórica de relatórios.
- Palavras banidas (lista actual).

Convenções abandonadas: identificar e marcar com data.

---

## §5 Outputs esperados

### Ficheiro 1 — `historiograma-passos.md`

Localização: `00_nucleo/historiograma-passos.md` (substitui
o anterior; backup do anterior para
`00_nucleo/historiograma-passos.P156A.md` antes de escrever).

Estrutura:

1. Cabeçalho com data e referência ao spec P201.
2. Linha temporal completa (C1).
3. Padrões agregados (C2 + C3).
4. Recuos e reformulações (C4).
5. Cadeias de dependência (C5).
6. Métricas cumulativas (C7).
7. Convenções (C9).

### Ficheiro 2 — `auditoria-delta-P156A-P200.md`

Localização:
`00_nucleo/diagnosticos/typst-passo-201-auditoria-delta.md`.

Estrutura:

1. Cabeçalho com escopo (P157–P200).
2. Estado consolidado pré-M8 com etiquetas (C6).
3. Lacunas residuais (C8).
4. Padrões novos do ciclo (subset de C2 marcado como novo
   desde P156A).
5. ADRs novas do ciclo (subset).
6. Marcos fechados no ciclo.
7. Divergências detectadas face ao snapshot 2026-05-05 (se
   alguma).

### Ficheiro 3 — relatório do passo

Localização:
`00_nucleo/materialization/typst-passo-201-relatorio.md`.

Conteúdo:

- O que foi feito (referência aos dois outputs).
- Tempo de execução (real, da sessão Claude Code).
- Decisões tomadas durante a leitura (quando ambiguidade
  apareceu, qual interpretação foi escolhida).
- Sugestões para o próximo passo (não vinculativas; apenas
  observações).

---

## §6 Critério de progressão

P201 está concluído quando:

- Os 3 ficheiros existem.
- C1–C9 todos endereçados.
- Etiquetas de C6 todas CONFIRMADO; ou cada DIVERGÊNCIA
  registada.
- Backup do historiograma anterior existe.

Após P201 concluído, o **próximo passo** (P202 ou outro,
escolhido pelo developer) pode ser:

- P202A — diagnóstico-primeiro de M8 (caminho do snapshot
  2026-05-05).
- Outro caminho informado pelo que P201 revelou (ex: se a
  auditoria detectar divergência empírica relevante face
  ao snapshot, abrir P202 para corrigir snapshot antes de
  M8).

P201 não decide. Reporta.

---

## §7 Convenções mantidas

- Sem código Rust.
- Sem condicionais em sub-passos (este passo não tem
  sub-passos).
- 3 outputs padrão (historiograma + auditoria delta +
  relatório).
- Distinção fecho estrutural vs arquitectural mantida.
- Preservação histórica: backup do historiograma anterior
  obrigatório.
- Sem inflação: sem "patamar", sem "limiar", sem
  "consolidação", sem "deriva", sem "subpadrão", sem
  "cumulativo", sem "cross-domínio", sem "paridade
  observable" como bandeira retórica.

---

## §8 Não-objectivos

P201 não:

- Decide o caminho de M8.
- Propõe ADR-0073.
- Toca em código.
- Substitui o snapshot 2026-05-05 — verifica-o.
- Pré-define sub-passos M8 (`*B-G`) — esse erro foi
  identificado e não é repetido.
- Introduz padrões novos. Apenas regista os existentes.

---

## §9 Erro a não repetir

A tentativa anterior (versão "P193A com auditoria +
diagnóstico M8 + plano `*B-G`") definia:

1. P193.A auditoria L0.
2. P193A diagnóstico M8 com cláusulas C1–C11.
3. P193B–G plano completo de implementação.

Os três num só pacote. Isto é cláusula condicional
disfarçada — pré-define o conteúdo de P193B–G antes de
P193A produzir output empírico que justifique a forma de
B–G.

**Padrão diagnóstico-primeiro**: cada sub-passo `*B+`
emerge do anterior. Pré-defini-los antecipa estado que
ainda não foi medido.

**Correcção**: P201 produz revisão geral. Próximo passo
de feature (P202+) é fixado **depois**, com base no que
P201 revelar. Sub-passos `*B+` desse próximo passo são
fixados ainda mais à frente, com base no que `*A` revelar.
