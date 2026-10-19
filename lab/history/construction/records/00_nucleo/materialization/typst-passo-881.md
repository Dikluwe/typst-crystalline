# Prompt — typst-passo-881: `array.map(str)` quebra no cristalino (regressão encontrada no benchmark de P880)

**Origem**: P880 descobriu, ao tentar reproduzir o cenário `05-tables` do benchmark, que `..range(50).map(str)` (passando a função nativa `str` diretamente, sem lambda) falha no cristalino com `array.map() espera função, recebeu type`, enquanto compila normalmente no vanilla 0.15.0.
**Estado**: aguardando execução

---

## Achado (medição de P880)

`(0, 1, 2).map(str)` — vanilla: aceita, aplica `str` a cada elemento; cristalino: `error: array.map() espera função, recebeu type`. A mensagem de erro sugere que o cristalino está resolvendo `str` como o **tipo** `str` (o construtor/tipo, `type(str)`), não como a **função** nativa `str(...)` que converte valores em string.

---

## Regra da linha de trabalho (obrigatória)
Comando exacto + saída literal (vanilla vs cristalino). Contagem de testes de `typst-core` a bater com os testes novos.

## Passo 1 — Sonda

1. Reproduzir o caso exato nos dois binários, confirmar a mensagem.
2. Testar outros nomes de tipo/função que colidem no cristalino (`int`, `float`, `bool`, `array` — qualquer construtor nativo que também é nome de tipo) passados como valor de primeira classe para `.map()`/`.filter()`/outras funções de ordem superior — confirmar se o problema é geral a qualquer função nativa cujo nome também nomeia um tipo, ou específico de `str`.
3. Localizar no cristalino onde o identificador `str` é resolvido no escopo global — provavelmente há uma ambiguidade entre o binding do tipo (`Type::Str`) e o binding da função construtora/conversora, e o lookup está pegando o tipo quando deveria pegar a função nesse contexto (uso como valor, não como chamada direta `str(...)`).
4. Confirmar no vanilla como essa ambiguidade é resolvida — provavelmente `str` no escopo global é a função, e o tipo é acessado de outra forma (ou o mesmo binding serve para os dois casos, com despacho diferente dependendo do uso).

## Passo 2 — Implementação

Corrigir a resolução do identificador para que `str` (e os outros nomes afetados, se o Passo 1.2 confirmar que é geral) usado como valor (passado para `.map()`, atribuído a uma variável, etc.) resolva para a função, replicando o comportamento do vanilla — sem quebrar o uso de `str` como tipo em outros contextos (ex.: `type(x) == str`, se essa comparação existir).

## Passo 3 — Validação

1. `(0, 1, 2).map(str)` funcionando, batendo com o vanilla.
2. Confirmar que os outros usos de `str` (chamada direta `str(42)`, comparação de tipo) continuam funcionando sem regressão.
3. Se o Passo 1.2 confirmou que o problema é geral a outros nomes, testar esses também.
4. Suíte completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-881-relatorio.md` com medição antes, código identificado, diff, medição depois, contagem de testes. Depois de fechado, atualizar o benchmark: o `05-tables.typ` original (com `.map(str)`) pode voltar a ser usado sem o ajuste de sintaxe que P880 precisou fazer, tornando a comparação com P872 estritamente maçã-com-maçã de novo.
