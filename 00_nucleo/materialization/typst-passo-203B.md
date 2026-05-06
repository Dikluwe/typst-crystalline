# Passo 203B — Pivot para lacuna #1 real (Figure kind=None ↔ Introspector)

**Série**: 203 (sub-passo `B` = implementação após
diagnóstico-primeiro de P203A).
**Tipo**: implementação (com correcção administrativa
embutida).
**Magnitude planeada**: M (S–M para feature + S para
correcção embutida).
**Pré-condição**: P203A concluído; auditoria
`typst-passo-203A-auditoria-position.md` produzida;
diagnóstico `typst-passo-203A-diagnostico.md` produzido;
divergência `P203A.div-1` registada (lacunas #1/#1b/#2 não
são Position).
**Output**: 3 ficheiros + alterações concretas em código
+ correcção dos 3 ficheiros administrativos.

---

## §1 Contexto

P203A revelou que as lacunas #1/#1b/#2 não são sobre
Position. Definição empírica canónica (P200 consolidado §7
+ `m1-lacunas-captura.md`):

| # | Lacuna real |
|---|---|
| #1 | Figure kind=None ↔ Introspector |
| #1b | from_tags arm Figure sem gate `is_counted` |
| #2 | reservada |

P203B pivota: endereça a lacuna #1 real e, como subproduto
trivial, a #1b (são facetas do mesmo problema). A correcção
da nomenclatura nos 3 ficheiros administrativos
(snapshot 2026-05-05, auditoria delta P201 §2,
historiograma) fica embutida neste passo.

---

## §2 Objectivo literal

Quatro outputs, com propósitos distintos:

### Output 1 — código

Corrigir o desalinhamento entre walk arm e from_tags arm
para `Content::Figure`. Especificamente:

- Walk arm usa `kind.as_deref().unwrap_or("image")` para
  contador (default fallback).
- from_tags arm para `Figure` ou usa o mesmo default ou
  omite o gate `is_counted`, criando divergência.

Lacuna #1 fecha quando os dois caminhos produzem o mesmo
mapping para o mesmo input. Lacuna #1b fecha quando o gate
`is_counted` é aplicado consistentemente em ambos os
arms.

### Output 2 — correcção administrativa embutida

Reescrita parcial de:

- `00_nucleo/snapshot-2026-05-05.md` §7 (lacunas).
- `00_nucleo/diagnosticos/typst-passo-201-auditoria-delta.md`
  §2 (lacunas residuais).
- `00_nucleo/historiograma-passos.md` (entradas que
  citam #1/#1b/#2 com nomenclatura errada).

### Output 3 — ADR opcional

Decidir se P203B exige ADR. Provavelmente **não** —
trabalho é resolução de divergência catalogada, não
decisão arquitectural nova. Decisão fixada em §4 C7.

### Output 4 — relatório do passo

Padrão habitual.

---

## §3 Material de partida verificado em P203A

Antes de qualquer alteração, verificar empíricamente:

- `Content::Figure { kind, body, caption, ... }` existe
  em L1 com campo `kind: Option<EcoString>` (ou
  similar — confirmar tipo exacto).
- Walk arm em `01_core/src/rules/introspect.rs` aplica
  `kind.as_deref().unwrap_or("image")` para counter
  step.
- `extract_payload` para `Content::Figure` produz
  `ElementPayload::Figure { kind, is_counted, ... }`.
- `from_tags` arm para `ElementPayload::Figure` consome
  `kind` e `is_counted`.

Sem isto, a auditoria não tem fundamento. Recuar para
P203A se algum item não estiver confirmado.

---

## §4 Cláusulas de execução (sem condicionais)

### C1 — Inventário do desalinhamento empírico

Antes de tocar em código, listar **literalmente** em
diagnóstico interno:

- Walk arm `Content::Figure` — linhas exactas; condição
  de step counter; valor usado para `kind`.
- `extract_payload` arm `Content::Figure` — linhas
  exactas; valor de `kind` e `is_counted` produzido.
- `from_tags` arm `ElementPayload::Figure` — linhas
  exactas; condição de população; gate `is_counted`
  consultado ou não.

Output: tabela de 3 linhas (walk / extract_payload /
from_tags) com 3 colunas (linha, condição, kind usado).

A discrepância exacta é o input para C2.

### C2 — Forma da correcção

Decisão entre dois caminhos canónicos:

- **Caminho A — Alinhar `from_tags` ao walk**: aplicar o
  mesmo default `unwrap_or("image")` em from_tags; aplicar
  o mesmo gate `is_counted` em from_tags.
- **Caminho B — Alinhar walk ao extract_payload**: walk
  passa a delegar a decisão a `extract_payload` (que já
  produz `is_counted` resolvido) em vez de re-derivar.

Caminho A é menos invasivo. Caminho B é mais alinhado com
o desenho post-P181 (extract_payload como autoridade).

Decisão fica fixada em P203B com base em C1.

### C3 — Tests

Adicionar tests E2E que cobrem os 4 casos:

- `#figure([img])` — kind=None, sem caption.
- `#figure([img], caption: [c])` — kind=None, com caption.
- `#figure(kind: "table", [t], caption: [c])` — kind=Some,
  com caption.
- `#figure(kind: "table", [t])` — kind=Some, sem caption.

Para cada caso, asserir:

- Resultado de walk arm (counter step ou não).
- Resultado de extract_payload (`kind`, `is_counted`).
- Resultado de from_tags (entry em sub-store ou não).
- Output observable (numeração visível no PDF; reference
  resolvida).

Critério: walk e from_tags produzem mapping idêntico para
o mesmo input. Lacuna #1 fecha.

### C4 — Correcção do snapshot 2026-05-05 §7

Reescrever a tabela de lacunas com nomenclatura empírica
correcta:

```
| # | Tópico | Estado | Último passo | Bloqueia M8? |
|---|--------|--------|--------------|--------------|
| #1 | Figure kind=None ↔ Introspector | fechada P203B | P203B | não |
| #1b | from_tags arm Figure sem gate is_counted | fechada P203B | P203B | não |
| #2 | reservada / vazia | — | — | não |
| #3 | headings_for_toc | fechada P200B | P200B | não |
| #4 | numbering_active StyleChain-like | fechada P182F | P182F | não |
| #5 | CounterRegistry hierárquico | fechada P170 | P170 | não |
| #6 | Bibliography full-stack | fechada P181I | P181I | não |
| #7 | Outline locatable | fechada P178 | P178 | não |
```

Resultado: **zero lacunas residuais**. Position passa a
ser concern coberto por ADR-0066 + M8, não lacuna
catalogada.

### C5 — Correcção do snapshot §13 (resumo)

Reescrever o bloco "Lacunas residuais" do resumo para
nova sessão:

```
**Lacunas residuais** (todas catalogadas em
m1-lacunas-captura.md):
- #1 e #1b — fechadas em P203B.
- #2 — slot reservado, vazio.
- #3-#7 — fechadas no ciclo P156B-P200.

**Concerns ortogonais (não-catalogados)**:
- Position concrete — adiada para M8 por ADR-0066.
```

### C6 — Correcção da auditoria delta P201 §2

Anotar §2 da auditoria delta com correcção retroactiva.
Não reescrever o conteúdo histórico — adicionar bloco
explícito no início da secção:

```
**CORRECÇÃO RETROACTIVA aplicada por P203B (2026-05-05)**:

A tabela abaixo atribui "Position", "Position-related" e
"Counter at locations" às lacunas #1/#1b/#2. Esta atribuição
está empíricamente errada. As lacunas reais (per
m1-lacunas-captura.md e P200 consolidado §7) são:
- #1 — Figure kind=None ↔ Introspector.
- #1b — from_tags arm Figure sem gate is_counted.
- #2 — reservada / vazia.

O conteúdo abaixo é preservado para histórico mas não é
canónico. Para estado actual, ver
00_nucleo/snapshot-2026-05-05.md §7.
```

### C7 — ADR para P203B?

Decisão: **não criar ADR**. Justificação:

- P203B resolve divergência empírica catalogada (lacuna
  #1/#1b), não introduz mecanismo novo.
- Padrão "alinhar walk / extract_payload / from_tags"
  está coberto por ADR-0069 e ADR-0071.
- Caso C2 escolha Caminho B (walk delega a
  extract_payload), pode haver argumento para
  micro-ADR — decisão fica para output empírico de C2.

### C8 — Correcção do historiograma

Procurar entradas em `00_nucleo/historiograma-passos.md`
que citem #1/#1b/#2 com nomenclatura errada (Position).
Para cada uma:

- Adicionar marca `[corrigido P203B]`.
- Substituir nomenclatura.
- Preservar conteúdo histórico.

Não rescrever o historiograma inteiro. Edições cirúrgicas.

### C9 — Critério de fecho de P203B

P203B está concluído quando:

- Tests workspace verdes (provavelmente +4 a +8 tests
  adicionados em C3).
- Crystalline-lint 0 violations.
- Walk e from_tags produzem mapping idêntico para os 4
  casos de C3.
- Snapshot §7 e §13 reescritos.
- Auditoria delta P201 §2 anotada.
- Historiograma corrigido cirurgicamente.
- Lacunas #1/#1b marcadas fechadas.

### C10 — Sem cláusulas condicionais

C1 produz dados empíricos. C2 fixa caminho com base em C1.
C3–C9 executam o caminho fixado.

C2 não tem `if Caminho A else Caminho B` — fixa **um**
caminho em P203B com base em C1.

---

## §5 Outputs concretos

### Ficheiro 1 — diagnóstico interno

Localização:
`00_nucleo/diagnosticos/typst-passo-203B-inventario.md`.

Conteúdo: tabela de C1 (walk / extract_payload /
from_tags). Decisão C2 fixada. Plano de implementação
literal.

### Ficheiro 2 — relatório

Localização:
`00_nucleo/materialization/typst-passo-203B-relatorio.md`.

Conteúdo:
- O que foi feito.
- Caminho escolhido (C2 = A ou B).
- Tests adicionados.
- Métricas (tests baseline → tests pós; LOC delta).
- Lacunas #1/#1b marcadas fechadas.
- Tempo de execução.
- Decisões tomadas durante a leitura.
- Sugestão para próximo passo.

### Ficheiro 3 — alterações em código

Não é ficheiro discreto — é o conjunto de alterações em:

- `01_core/src/rules/introspect.rs` (walk arm Figure).
- `01_core/src/rules/introspect/extract_payload.rs`
  (verificar se precisa alteração).
- `01_core/src/rules/introspect/from_tags.rs` (from_tags
  arm Figure).
- Tests E2E em ficheiro relevante.

### Ficheiros administrativos a corrigir (não são outputs
"novos" — são edições)

- `00_nucleo/snapshot-2026-05-05.md` §7 e §13.
- `00_nucleo/diagnosticos/typst-passo-201-auditoria-delta.md`
  §2 (anotação retroactiva).
- `00_nucleo/historiograma-passos.md` (edições cirúrgicas).

---

## §6 Critério de progressão

P203B está concluído quando C9 cumprido na totalidade.

Em caso de divergência empírica relevante face a P203A
(ex: walk arm já não usar `unwrap_or("image")`,
extract_payload já não produzir `is_counted`), registar
em `P203B.div-N` e:

- Re-executar C1 com os valores actuais.
- Re-fixar C2 com os novos dados.

---

## §7 Convenções mantidas

- Sem condicionais em C2 (fixa um caminho, não dois).
- 3 outputs principais (diagnóstico interno + relatório
  + código).
- Distinção fecho estrutural vs arquitectural mantida.
- Preservação histórica: auditoria delta P201 §2 não é
  reescrita; é anotada com correcção retroactiva.
- Sem inflação retórica.

---

## §8 Não-objectivos

P203B não:

- Materializa Position concrete (concern de M8 por
  ADR-0066).
- Decide M8.
- Promove ADR-0067.
- Toca em outros consumers além dos 3 arms (walk /
  extract_payload / from_tags) e os tests.
- Reescreve o historiograma inteiro — edições cirúrgicas.
- Reescreve a auditoria delta P201 — anotação
  retroactiva apenas.
- Cria sub-passos `*C+`.

---

## §9 Particularidade — execução

P203B tem componente de código (não apenas administrativo).
Pode ser executado pela sessão actual (Opus, com
bash_tool para verificações) ou pelo Claude Code.

Se executado pelo Claude Code, segue padrão dos passos
de feature anteriores. Se pela sessão actual, magnitude
M é manejável.

---

## §10 Erro a não repetir

P203A spec herdou nomenclatura errada de P201/P202 sem
verificar empíricamente. P203B previne isso ao começar
com C1 (inventário empírico) antes de C2 (decisão).

Pattern: cada passo de feature começa com inventário
empírico do estado actual, não com afirmações herdadas
de specs anteriores.

Sub-passo `*B` não escapa a esta regra. Mesmo sem sufixo
`A`, a primeira cláusula é sempre verificação empírica.
