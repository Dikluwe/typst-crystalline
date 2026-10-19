# Prompt — typst-passo-821: `foundations::target_` — `#target()` fora de `#context` não erra (achado #8 de P810)

**Origem**: achado #8 da tabela de P810
**Estado**: aguardando execução

---

## Achado (texto do relatório de P810)

> `#target()` fora de `#context` não erra (vanilla: erro + 2 hints); colateral: `#context type()` vazio

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## Passo 1 — Sonda

1. Compilar `#target()` fora de qualquer `#context` (no nível de documento) com os dois binários — confirmar que o vanilla erra com 2 hints (registar o texto exacto de ambos) e o cristalino não erra (registar o que devolve em vez disso).
2. Compilar `#context type()` (chamada de `type()` sem argumento dentro de `#context`, ou a variante exacta citada como "colateral" no achado — conferir o relatório de materialização de P810 para o caso exacto) — confirmar que devolve vazio/incorrecto no cristalino.
3. Localizar no vanilla (`lab/typst-original/`) onde `target()` verifica se está a correr dentro de um contexto de introspecção (`Context`/`Locatable`) e o texto exacto dos dois hints.
4. Confirmar se o "colateral" do Passo 2 é o mesmo mecanismo (checagem de contexto) ou uma causa separada — não assumir, testar isoladamente `#context type()` puro, sem `target()` envolvido, para isolar a variável.
5. Registar os pontos antes de tocar em código.

## Passo 2 — Implementação

Adicionar a verificação de contexto a `target()`, replicando os dois hints do vanilla literalmente. Se o colateral do Passo 2/1.2 for causa separada, corrigir também, documentando que são dois pontos distintos dentro do mesmo achado.

## Passo 3 — Validação

1. Recompilar. Repetir os comandos do Passo 1, mensagens e hints batendo com o vanilla.
2. Confirmar que `#context target()` (dentro do contexto correcto) continua a funcionar sem regressão.
3. Testes novos cobrindo o erro fora de contexto (com os dois hints) e o caso colateral.
4. Suíte `typst-core` completa, comando + contagem antes/depois.

## Passo 4 — Relatório

`00_nucleo/diagnosticos/typst-passo-821-relatorio.md` com: medição antes, código vanilla/cristalino identificado, diff, medição depois, contagem de testes.
