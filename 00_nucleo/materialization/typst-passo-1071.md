# L0 — Passo 1071: Recuperar e Escrever os 6 Achados Média/Baixa do P1031

**Gate**: nenhum para este passo em si (é preparatório — recuperar e organizar,
não codificar). Cada um dos 6 achados, uma vez escrito como L0 próprio, terá o
seu próprio gate conforme a natureza da mudança que propuser.

**Base**: pendência do documento de continuação — o P1031 catalogou 12
divergências reais de comportamento (família math + resto); os 6 de prioridade
Alta viraram passos (P1034-1037); os 6 de Média/Baixa ficaram catalogados, nunca
escritos.

---

## Passo 0 — Não reconstruir de memória, pedir o documento real

**Não tenho, nesta conversa, o conteúdo do relatório do P1031** — só a menção de
que existem 6 achados de prioridade Média/Baixa, sem saber quais são. Esta é
exactamente a situação que a regra da skill de transição de conversa cobre:
nunca reconstruir conteúdo de um relatório a partir de busca parcial ou memória —
pedir o arquivo real.

**Pedir ao dono**: o relatório do P1031 (`00_nucleo/diagnosticos/typst-passo-1031-relatorio.md`
ou nome equivalente — confirmar o caminho real, não presumir), especificamente a
secção com os 12 achados e as prioridades atribuídas a cada um.

## Passo 1 — Depois de receber o P1031: separar os 6 de Média/Baixa

Confirmar que são de facto 6 (reconciliar o número, mesmo processo já usado
repetidamente nesta conversa) e listá-los com:
- Descrição resumida do achado.
- Prioridade atribuída (Média ou Baixa) e o motivo, se estiver no relatório
  original.
- Se algum dos 6 já foi resolvido incidentalmente por outro passo desta conversa
  (aconteceu antes — P1059 tocou em coisas fora do seu escopo original algumas
  vezes) — verificar antes de escrever um L0 para algo já fechado.

## Passo 2 — Escrever um L0 por achado, não um L0 genérico para os 6

Cada achado é uma divergência de comportamento distinta — mesma disciplina já
aplicada em P1034-1037 (achados de Alta prioridade do mesmo P1031, cada um com
o seu próprio passo, não agrupados). Para cada um dos 6:

- Gate `ADR-0127` se envolver mudança de comportamento por defeito (a maioria dos
  achados deste tipo nesta conversa envolveu).
- Medição real antes de propor a correcção (`pdftotext -bbox-layout` ou
  equivalente), não assumir a partir da descrição do achado original.
- Critério de verificação próprio.

**Não escrever os 6 L0s neste mesmo passo** — só depois do Passo 0/1 estarem
completos. Ordem de prioridade entre os 6 (Média antes de Baixa, ou por outro
critério) é decisão do dono depois de ver a lista completa.

## Critério de conclusão deste passo

- Relatório do P1031 obtido (não reconstruído).
- 6 achados listados com prioridade e estado actual (ainda pendente / já
  resolvido incidentalmente).
- Nenhum L0 de correcção escrito ainda — isso é o passo seguinte, um por achado.
