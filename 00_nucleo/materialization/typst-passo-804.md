# Prompt — typst-passo-804 (achado P798 #9): `visualize` — `#line(length: ...)` rejeitado, vanilla aceita

**Origem**: P798 (lote 3 de triagem em lote, corrigido), tabela "Achados de P798, aguardando passo dedicado"
**Handoff**: `00_nucleo/handoff-novo-chat-p798.md`
**Módulo afectado**: `visualize`
**Estado**: aguardando execução, ainda não corrigido

---

## Achado (texto exacto do handoff)

> `#line(length: ...)` — argumento nomeado rejeitado, vanilla aceita

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" ou "mecanicamente correto" sem execução mostrada. Cada afirmação do relatório deste passo tem de vir acompanhada do comando exacto e da saída literal, comparando vanilla (`lab/typst-original/target/release/typst`) e cristalino (`./target/release/typst`). A contagem de testes da suíte `typst-core` tem de aparecer no relatório e bater com o número de testes novos declarados.

---

## Passo 1 — Sonda (obrigatória antes de qualquer alteração de código)

1. Compilar `#line(length: 3cm)` (e uma variante com ângulo, se o vanilla também aceitar `angle:` combinado com `length:`) com os dois binários. Registar a saída literal de cada um — o cristalino deve estar a rejeitar o argumento nomeado `length`.
2. Localizar no código-fonte do vanilla (`lab/typst-original/`) a assinatura da função `line` e confirmar a lista completa de argumentos nomeados aceites (`start:`, `end:`, `length:`, `angle:`, `stroke:`) e como `length`/`angle` interagem com `start`/`end` (mutuamente exclusivos ou combináveis).
3. Localizar no código do cristalino a assinatura equivalente de `native_line` (ou nome correspondente) e confirmar que `length` está ausente ou mal validado.
4. Registar os dois pontos (vanilla e cristalino) no relatório antes de tocar em código.

## Passo 2 — Implementação

Adicionar o argumento nomeado `length` (e `angle`, se também estiver em falta) a `line`, replicando a semântica do vanilla: `length` define o comprimento da linha a partir de `start` (ou origem), combinável com `angle` para definir a direcção. Validar as combinações de argumentos que o vanilla rejeita (ex.: `length`/`angle` simultâneos com `end` explícito) com a mesma mensagem de erro.

## Passo 3 — Validação

1. Recompilar o cristalino.
2. Repetir o comando do Passo 1 e mostrar a saída literal, agora igual à do vanilla (incluindo a geometria da linha resultante, não só a ausência de erro).
3. Adicionar casos de teste cobrindo `length` sozinho, `length` + `angle`, e a combinação inválida que deve continuar a ser rejeitada.
4. Correr a suíte `typst-core` completa e mostrar o comando e a contagem de testes antes/depois.

## Passo 4 — Relatório

Produzir `00_nucleo/materialization/typst-passo-804-relatorio.md` com:
- Comando + saída literal do Passo 1 (antes da correcção).
- Trecho do código vanilla e do código cristalino identificados no Passo 1.
- Diff da correcção.
- Comando + saída literal do Passo 3 (depois da correcção).
- Contagem de testes antes/depois.
