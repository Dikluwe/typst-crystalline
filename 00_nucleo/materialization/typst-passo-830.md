# Prompt — typst-passo-830: corrigir proveniência falsa da decisão do Item A de P829 (`#eval` e o escopo do chamador)

**Origem**: o relatório de P829 registrou, para o Item A, que "o dono foi consultado em 2026-07-22 e optou por não corrigir agora". Essa consulta não aconteceu. O comportamento mantido (não corrigir, ficar como está) pode ser exatamente o que o dono teria escolhido — mas isso não foi perguntado a ele, foi assumido pelo executor e escrito como se tivesse sido uma decisão dele.
**Estado**: aguardando execução — correção de registro, não de código

---

## Por que isto é sério e não um detalhe

O padrão de decisão de escopo usado neste projeto desde P807 (e reforçado em P812-C, P825-C, e no próprio Item D de P829) depende de uma coisa: quando um L0 diz "decisão do dono: X", isso precisa ser verdade, porque é nisso que os próximos passos vão confiar sem reconferir. Se um executor pode escrever "dono consultado" sem consultar, e o texto fica indistinguível de uma consulta real, esse mecanismo inteiro perde a garantia que o faz funcionar. Não é um erro de medição de código — é um erro sobre a proveniência de uma decisão, a mesma categoria de coisa que o handoff original já tinha avisado para verificar (relatórios inesperados, execuções fora do fluxo).

---

## Passo 1 — Corrigir o registro

1. Localizar a entrada no L0 `00_nucleo/prompts/engine/stdlib/eval.md` §4 que foi editada por P829 com o texto sobre a consulta ao dono. Reescrever essa entrada removendo qualquer afirmação de que houve consulta real. O texto correto é: o comportamento foi **mantido por omissão** (o executor não decidiu corrigir, não porque o dono tenha sido consultado e decidido isso), com o levantamento de P829 (usos no repositório, trade-off) preservado como está — esse levantamento é factual e continua válido, só a atribuição da decisão está errada.
2. Fazer o mesmo no relatório `00_nucleo/diagnosticos/typst-passo-829-relatorio.md` — não reescrever a história silenciosamente; acrescentar uma nota de correção (data de hoje) explicando que a frase "dono consultado" estava incorreta, mantendo o relatório original visível (risco de reescrever é apagar o rastro do erro, que também é informação).
3. Verificar se algum outro L0 ou relatório deste projeto tem o mesmo padrão de "decisão do dono" sem uma decisão real por trás — buscar por frases como "dono consultado", "decisão do dono", "o dono optou" em `00_nucleo/prompts/` e `00_nucleo/diagnosticos/`, e para cada ocorrência confirmar se há evidência real de que a decisão foi tomada por alguém com autoridade (não por um subagente). Isto pode revelar mais do que só o Item A.

## Passo 2 — Levar a decisão real ao dono

O levantamento de P829 (usos no repositório, trade-off, os dois testes que fixam o comportamento atual) já está pronto e é bom o suficiente para decidir em cima dele — não precisa refazer a sonda. O que falta é a decisão em si, vinda de uma pessoa com autoridade sobre o projeto, não de um executor.

Apresentar ao dono, de forma direta:
- Comportamento vanilla: `#eval` não vê variáveis externas (escopo fresco).
- Comportamento cristalino atual: vê (herda o escopo de quem chamou).
- Dois testes já fixam o comportamento atual (`eval_ve_escopo_actual`, `p394_eval_ve_escopo_exterior`).
- Nenhum outro consumidor no repositório depende disso, segundo o levantamento de P829.
- Se manter: é uma divergência de comportamento de linguagem consciente, não só de mensagem.
- Se corrigir: o ponto exato já foi localizado (P814) — trocar o escopo herdado por um `Scopes` fresco em `native_eval`, e atualizar os dois testes.

## Passo 3 — Registrar a decisão real

Só depois de uma resposta real do dono: atualizar o L0 `stdlib/eval.md` §4 de novo, desta vez com a decisão verdadeira e quem a tomou. Se a decisão for "corrigir", tratar como um passo de implementação normal (a sonda já existe, é só executar). Se for "manter", a entrada do L0 fica igual ao que já estava, só que honesta sobre a origem.

## Relatório

Produzir `00_nucleo/diagnosticos/typst-passo-830-relatorio.md` com: a correção feita no L0 e no relatório de P829 (Passo 1), o resultado da varredura por outras ocorrências do mesmo padrão (Passo 1.3) — mesmo que o resultado seja "nenhuma outra encontrada", isso precisa estar documentado com o comando de busca usado —, e a decisão real do dono quando ela chegar (Passo 2/3). Se a decisão do dono ainda não tiver chegado no momento de fechar este passo, o relatório fica parcial (Passos 1 e 1.3 fechados, Passo 2/3 em aberto) — não inventar uma decisão para fechar o relatório.
