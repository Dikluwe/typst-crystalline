# Regra — registar a proveniência de cada medição

**Data:** 2026-07-05
**Aplica-se a:** todo o projecto, todos os passos que produzem um número usado para decidir algo.

---

## O que aconteceu

P569 registou "4 linhas" para um documento de teste. P574 e P575, mais tarde, tentaram reproduzir esse número a partir do commit de P569, e obtiveram "2 linhas" — nos dois casos, com e sem o código órfão de L1 que se pensava ser a causa. O número de P569 não foi reproduzido, e a causa real ficou sem explicação.

O problema não é o número em si. É que não há registo de **o que exactamente gerou esse número** — qual o estado do código no momento exacto da medição, se havia alterações não commitadas nessa altura, ou qualquer outra coisa que distinga esse momento do commit final que ficou no histórico.

## Regra

Qualquer número usado num relatório para decidir se algo está fechado ou aberto (contagem de páginas, de linhas, de palavras, posições, tempos de execução) tem de vir acompanhado de:

1. **O hash do commit** em que o teste foi corrido, ou "working tree não commitado" se for o caso, com a lista exacta de ficheiros alterados nesse momento (`git diff HEAD --stat`).
2. **A hora exacta**, se houver razão para pensar que o estado pode ter mudado entre uma medição e outra no mesmo passo.

Isto não substitui as regras já escritas (decisão nova obrigatória; disciplina de verificação). Complementa-as: aquelas dizem para medir antes de decidir; esta diz para registar o suficiente sobre a medição para que outra pessoa, mais tarde, consiga voltar a chegar ao mesmo número, ou perceber porque não consegue.

## Como aplicar

Nos relatórios futuros, sempre que um número apareça, verificar se é possível responder à pergunta "a partir de que estado exacto do código veio este número?" sem ter de adivinhar. Se a resposta for "não sei", o número não deve ser usado para fechar nada — só como indicação a confirmar de novo.

## Ligação às regras anteriores

- **Decisão nova obrigatória** — nenhum item aceite é permanente sem decisão nova.
- **Disciplina de verificação** — uma afirmação sobre desempenho ou grandeza de um problema precisa de número, não de adjectivo.
- **Esta regra** — um número, para servir de prova, precisa de se saber de onde veio.

As três juntas cobrem o ciclo completo: decidir de novo, com prova, e com essa prova a poder ser encontrada outra vez.
