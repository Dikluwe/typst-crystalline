# Prompt — typst-passo-818: `foundations::ops` — ordenação e operadores ausentes (achado #5 de P810)

**Origem**: achado #5 da tabela de P810
**Estado**: aguardando execução

---

## Achado (texto do relatório de P810)

> ordenação str/array/bool ausente; div Relative/Relative, Ratio/Ratio; Str*Int; eq/ord Length↔Relative; coerção não aninhada

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## Passo 1 — Sonda

Para cada operação, testar nos dois binários e registar a saída literal:
1. Ordenação: `#("b" < "a")`, `#((1,2) < (1,3))`, `#(false < true)` — confirmar se o cristalino rejeita (`cannot compare`/erro) onde o vanilla compara.
2. Divisão: `#(50% / 25%)` (Ratio/Ratio), `#(1cm + 1% / (1cm + 2%))` ou equivalente para Relative/Relative — confirmar se o cristalino rejeita.
3. `#("a" * 3)` (Str*Int) — confirmar se o vanilla repete a string (`"aaa"`) e o cristalino rejeita.
4. Comparação/igualdade entre `Length` e `Relative` (ex.: `#(1cm == 1cm + 0%)`) — confirmar divergência.
5. Coerção não aninhada: reproduzir o caso exacto do relatório de materialização de P810 (achado #5), que não está detalhado neste resumo — conferir lá antes de assumir qual coerção está em causa.

## Passo 2 — Implementação

Para cada operação confirmada como ausente: localizar no vanilla (`lab/typst-original/`, `crates/typst-library/src/foundations/ops.rs` ou equivalente) a implementação de `PartialOrd`/`Mul`/`Div`/`PartialEq` para os tipos envolvidos, e replicar no cristalino (`01_core/src/engine/eval/ops.rs` ou equivalente).

## Passo 3 — Validação

1. Recompilar. Repetir os comandos do Passo 1, saída literal batendo com o vanilla.
2. Testes novos por operação (ordenação, cada divisão, `Str*Int`, comparação Length/Relative, coerção não aninhada).
3. Suíte `typst-core` completa, comando + contagem antes/depois.

## Passo 4 — Relatório

`00_nucleo/diagnosticos/typst-passo-818-relatorio.md` com: medição antes por operação, código vanilla/cristalino identificado, diff, medição depois, contagem de testes.
