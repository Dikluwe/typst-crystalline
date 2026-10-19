# Prompt — typst-passo-801 (achado P798 #4): `utils::protected` — representação de array de 1 elemento diverge

**Origem**: P798 (lote 3 de triagem em lote, corrigido), tabela "Achados de P798, aguardando passo dedicado"
**Handoff**: `00_nucleo/handoff-novo-chat-p798.md`
**Módulo afectado**: `utils::protected`
**Estado**: aguardando execução, ainda não corrigido

---

## Achado (texto exacto do handoff)

> Representação de array de 1 elemento diverge (`(0,)` vanilla vs `(0)` cristalino) via `counter().get()`

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" ou "mecanicamente correto" sem execução mostrada. Cada afirmação do relatório deste passo tem de vir acompanhada do comando exacto e da saída literal, comparando vanilla (`lab/typst-original/target/release/typst`) e cristalino (`./target/release/typst`). A contagem de testes da suíte `typst-core` tem de aparecer no relatório e bater com o número de testes novos declarados.

---

## Passo 1 — Sonda (obrigatória antes de qualquer alteração de código)

1. Reproduzir com um documento `.typ` mínimo que chame `counter().get()` de forma a devolver um array de 1 elemento (ex.: `#counter("x").update(1); #counter("x").get()`), compilar com os dois binários, e registar a saída literal de cada um.
2. Confirmar que a diferença é especificamente a vírgula final (`(0,)` vs `(0)`) e não outra diferença de valor.
3. Localizar no código-fonte do vanilla (`lab/typst-original/`) a rotina de formatação/`Repr` para `Value::Array` que decide quando incluir a vírgula final (tipicamente ligada ao comprimento do array — arrays de 1 elemento usam vírgula para se distinguirem de parênteses de agrupamento).
4. Localizar no código do cristalino a rotina equivalente e confirmar que não aplica essa regra para o caso de 1 elemento.
5. Registar os dois pontos (vanilla e cristalino) no relatório antes de tocar em código.

## Passo 2 — Implementação

Corrigir a rotina de representação (`Repr`/`Display`, ou equivalente) do `Value::Array` no cristalino para que um array de exactamente 1 elemento seja impresso com vírgula final, replicando a regra do vanilla. Não alterar o comportamento para arrays de 0 ou 2+ elementos — confirmar que esses casos já estavam correctos e continuam correctos.

## Passo 3 — Validação

1. Recompilar o cristalino.
2. Repetir o comando do Passo 1 e mostrar a saída literal, agora igual à do vanilla.
3. Adicionar caso de teste cobrindo array de 1 elemento na representação (e, se ainda não existirem, casos de 0 e de 2+ elementos como controlo de regressão).
4. Correr a suíte `typst-core` completa e mostrar o comando e a contagem de testes antes/depois.

## Passo 4 — Relatório

Produzir `00_nucleo/materialization/typst-passo-801-relatorio.md` (ou o próximo número de passo na sequência do projecto) com:
- Comando + saída literal do Passo 1 (antes da correcção).
- Trecho do código vanilla e do código cristalino identificados no Passo 1.
- Diff da correcção.
- Comando + saída literal do Passo 3 (depois da correcção).
- Contagem de testes antes/depois.
