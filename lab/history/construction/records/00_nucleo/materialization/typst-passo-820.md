# Prompt — typst-passo-820: `foundations::scope` — `Deprecation` ausente (achado #7 de P810, prioridade alta)

**Origem**: achado #7 da tabela de P810, marcado prioridade alta no handoff (`handoff-novo-chat-p810.md`)
**Estado**: aguardando execução

---

## Achado (texto do relatório de P810)

> `Deprecation` ausente (`join` erro vs warning; `bowtie` ausente); núcleo em paridade verbatim (12 testes)

Nota: o núcleo de `scope` (resolução de nomes, mutação de constantes, etc. — já tratado extensamente em P772l/P772n/P772q no handoff antigo) está confirmado em paridade por 12 testes. Este achado é especificamente sobre o mecanismo de depreciação, que já tinha sido registado como débito sem fonte de dados no handoff de P798 (§"Achados menores registados, sem correcção": "Avisos de depreciação de símbolos — falta fonte de dados, P772l §2.6"). Este passo revisita esse débito com um caso concreto.

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## Passo 1 — Sonda

1. Testar o caso citado: uma chamada a algo relacionado com `join` que no vanilla deveria emitir `warning: deprecated` (ou similar) e no cristalino dá erro directo. Reproduzir a chamada exacta usada no relatório de materialização de P810 (achado #7) antes de assumir qual `join` está em causa (pode ser `array.join`, `str.join`, ou outro).
2. Testar `bowtie` — confirmar que é um símbolo ou função depreciado no vanilla, ausente por completo no cristalino.
3. Localizar no vanilla (`lab/typst-original/`) o mecanismo geral de depreciação — normalmente uma tabela ou atributo que marca símbolos/funções antigas, emitindo warning em vez de erro e apontando para o substituto.
4. Confirmar se o cristalino tem algum mecanismo equivalente (mesmo que incompleto) ou se está totalmente ausente — este é o "falta fonte de dados" já registado no handoff antigo; a fonte de dados pode ser extraída directamente do código-fonte do vanilla (a mesma lista usada lá).
5. Registar os pontos antes de tocar em código.

## Passo 2 — Implementação

Implementar o mecanismo de depreciação (se ausente) ou estendê-lo (se existir parcialmente), cobrindo pelo menos os dois casos medidos (`join`, `bowtie`). Extrair a lista de itens depreciados directamente do código-fonte do vanilla (fonte de dados que faltava). Não é necessário cobrir a lista inteira do vanilla de uma vez — cobrir o que os casos de teste exigem e registar o resto como scope-out explícito, no mesmo padrão já usado para tabelas grandes no projecto.

## Passo 3 — Validação

1. Recompilar. Repetir os comandos do Passo 1, saída literal batendo com o vanilla.
2. Confirmar que o núcleo de `scope` (os 12 testes já em paridade) continua sem regressão.
3. Testes novos cobrindo os dois casos e o mecanismo geral de depreciação.
4. Suíte `typst-core` completa, comando + contagem antes/depois.

## Passo 4 — Relatório

`00_nucleo/diagnosticos/typst-passo-820-relatorio.md` com: medição antes, código vanilla/cristalino identificado, alcance da lista de depreciação implementada e o que ficou fora, diff, medição depois, contagem de testes.
