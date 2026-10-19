# Prompt — typst-passo-805 (achado P798 #10): `text::lorem_` — falta ponto final no output de `#lorem(n)`

**Origem**: P798 (lote 3 de triagem em lote, corrigido), tabela "Achados de P798, aguardando passo dedicado"
**Handoff**: `00_nucleo/handoff-novo-chat-p798.md`
**Módulo afectado**: `text::lorem_`
**Estado**: aguardando execução, ainda não corrigido

---

## Achado (texto exacto do handoff)

> Falta ponto final no output de `#lorem(n)`

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" ou "mecanicamente correto" sem execução mostrada. Cada afirmação do relatório deste passo tem de vir acompanhada do comando exacto e da saída literal, comparando vanilla (`lab/typst-original/target/release/typst`) e cristalino (`./target/release/typst`). A contagem de testes da suíte `typst-core` tem de aparecer no relatório e bater com o número de testes novos declarados.

---

## Passo 1 — Sonda (obrigatória antes de qualquer alteração de código)

1. Compilar `#lorem(10)` (e mais um ou dois valores de `n`, incluindo `n=1`, para confirmar se a regra depende do comprimento) com os dois binários. Registar a saída literal completa de cada um, com atenção ao último carácter.
2. Localizar no código-fonte do vanilla (`lab/typst-original/`) a função `lorem`/`lorem_` e confirmar se o ponto final é sempre adicionado, condicional ao último token gerado, ou parte do corpus de texto usado.
3. Localizar no código do cristalino a função equivalente e identificar onde o ponto final está a ser omitido.
4. Registar os dois pontos (vanilla e cristalino) no relatório antes de tocar em código.

## Passo 2 — Implementação

Corrigir a função `lorem_` no cristalino para terminar a saída com ponto final nas mesmas condições do vanilla (confirmadas no Passo 1.2).

## Passo 3 — Validação

1. Recompilar o cristalino.
2. Repetir o comando do Passo 1 e mostrar a saída literal, agora igual à do vanilla.
3. Adicionar casos de teste cobrindo pelo menos dois valores de `n`, incluindo o caso limite identificado no Passo 1.1.
4. Correr a suíte `typst-core` completa e mostrar o comando e a contagem de testes antes/depois.

## Passo 4 — Relatório

Produzir `00_nucleo/materialization/typst-passo-805-relatorio.md` com:
- Comando + saída literal do Passo 1 (antes da correcção).
- Trecho do código vanilla e do código cristalino identificados no Passo 1.
- Diff da correcção.
- Comando + saída literal do Passo 3 (depois da correcção).
- Contagem de testes antes/depois.
