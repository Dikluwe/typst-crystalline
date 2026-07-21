# Prompt — typst-passo-803 (achado P798 #8): `visualize::curve` — mensagem de erro da API diverge

**Origem**: P798 (lote 3 de triagem em lote, corrigido), tabela "Achados de P798, aguardando passo dedicado"
**Handoff**: `00_nucleo/handoff-novo-chat-p798.md`
**Módulo afectado**: `visualize::curve`
**Estado**: aguardando execução, ainda não corrigido

---

## Achado (texto exacto do handoff)

> Mensagem de erro da API diverge (`expected content, found array` vanilla vs mensagem própria cristalina)

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" ou "mecanicamente correto" sem execução mostrada. Cada afirmação do relatório deste passo tem de vir acompanhada do comando exacto e da saída literal, comparando vanilla (`lab/typst-original/target/release/typst`) e cristalino (`./target/release/typst`). A contagem de testes da suíte `typst-core` tem de aparecer no relatório e bater com o número de testes novos declarados.

---

## Passo 1 — Sonda (obrigatória antes de qualquer alteração de código)

1. Reconstruir o caso concreto que produziu o achado: uma chamada à API de `curve` (ex.: `curve.move`/`curve.line`/argumento de `#curve(...)`) passando um valor do tipo errado, tal que o vanilla responda `expected content, found array`. Registar comando e a mensagem exacta produzida por cada binário.
2. Localizar no código-fonte do vanilla (`lab/typst-original/`) o ponto que gera essa mensagem de erro (provavelmente validação de tipo de argumento genérica, não específica de `curve`).
3. Localizar no código do cristalino o ponto equivalente e a mensagem própria que está a substituir a mensagem esperada.
4. Registar os dois pontos (vanilla e cristalino) no relatório antes de tocar em código.

## Passo 2 — Implementação

Ajustar a mensagem de erro no cristalino para corresponder literalmente à do vanilla (`expected content, found array`, respeitando o padrão `expected X, found Y` já usado no resto do projecto para erros de tipo). Confirmar se este é um caso isolado de `curve` ou se a validação de tipo genérica está a ser contornada — se for genérica, corrigir na origem comum em vez de em `curve` especificamente.

## Passo 3 — Validação

1. Recompilar o cristalino.
2. Repetir o comando do Passo 1 e mostrar a saída literal, agora igual à do vanilla.
3. Adicionar caso de teste cobrindo o erro de tipo em `curve` com a mensagem exacta esperada.
4. Correr a suíte `typst-core` completa e mostrar o comando e a contagem de testes antes/depois.

## Passo 4 — Relatório

Produzir `00_nucleo/materialization/typst-passo-803-relatorio.md` com:
- Comando + saída literal do Passo 1 (antes da correcção).
- Trecho do código vanilla e do código cristalino identificados no Passo 1.
- Nota explícita sobre se a causa é local a `curve` ou genérica de validação de tipo.
- Diff da correcção.
- Comando + saída literal do Passo 3 (depois da correcção).
- Contagem de testes antes/depois.
