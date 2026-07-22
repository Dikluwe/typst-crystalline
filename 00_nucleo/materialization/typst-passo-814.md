# Prompt — typst-passo-814: `typst_eval` — `#eval` sem `mode:`/`scope:`, erros genéricos, span detached (achado #1 de P810)

**Origem**: achado #1 da tabela de P810
**Estado**: aguardando execução

---

## Achado (texto do relatório de P810)

> `#eval` sem `mode:`/`scope:`; erros genéricos + span detached

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## Passo 1 — Sonda

1. Compilar `#eval("1+1")`, `#eval("1+1", mode: "code")`, `#eval("= Title", mode: "markup")`, e `#eval("x", scope: (x: 5))` com os dois binários. Registar a saída literal de cada um — confirmar que `mode:`/`scope:` são rejeitados ou ignorados no cristalino.
2. Testar um caso de erro dentro do código avaliado (ex.: `#eval("1/")`) e comparar a mensagem — confirmar "erro genérico" (o que o achado aponta) contra a mensagem específica do vanilla, e confirmar se o span do erro no cristalino é `<detached>` (sem posição útil) enquanto o vanilla aponta para a posição real dentro da string avaliada.
3. Localizar no vanilla (`lab/typst-original/`) a assinatura de `eval()` (`crates/typst-library/src/foundations/...`) — argumentos `mode`/`scope`, e como o motor de avaliação propaga spans de dentro da string avaliada (provavelmente usa um `Source` sintético ancorado na posição da chamada, para que erros internos apontem para dentro da string original).
4. Localizar `native_eval` (ou equivalente) no cristalino e confirmar a ausência dos argumentos e do mecanismo de span sintético.
5. Registar os pontos antes de tocar em código.

## Passo 2 — Implementação

Adicionar `mode:` (`"code"`/`"markup"`/`"math"`, default `"code"`) e `scope:` (dict de bindings extra) a `native_eval`, replicando a semântica do vanilla. Implementar o `Source` sintético (ou mecanismo equivalente já usado no projecto) para que erros dentro da string avaliada tenham span útil em vez de `<detached>`.

## Passo 3 — Validação

1. Recompilar. Repetir os comandos do Passo 1, saída literal batendo com o vanilla.
2. Teste cobrindo `mode:`, `scope:`, e o span do erro interno (confirmar posição, não só que não é `<detached>`).
3. Suíte `typst-core` completa, comando + contagem antes/depois.

## Passo 4 — Relatório

`00_nucleo/diagnosticos/typst-passo-814-relatorio.md` com: medição antes, código vanilla/cristalino identificados, diff, medição depois, contagem de testes.
