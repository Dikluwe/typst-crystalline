# Prompt — typst-passo-825: `typst_library::math` — classes, field access bare, fence espaçado (achado #12 de P810)

**Origem**: achado #12 da tabela de P810
**Estado**: aguardando execução

---

## Achado (texto do relatório de P810)

> classes aceites 15 vs 10; field access bare em math compila; fence spaced sem efeito (scope-out L0); LeftRightAlternator em mat ausente

Quatro sub-pontos. Tratar cada um com sonda própria — não confundir com o achado #13 (`math::style`), já fechado em P811/P812; este achado é sobre `typst_library::math` em geral (classes de símbolo, sintaxe, matrizes).

---

## Regra da linha de trabalho (obrigatória)

Não aceitar "corrigido" sem execução mostrada. Comando exacto + saída literal (vanilla vs cristalino) para cada afirmação. Contagem de testes de `typst-core` a bater com os testes novos declarados.

---

## Sub-achado A — classes matemáticas: 15 no vanilla vs 10 no cristalino

### Sonda
Listar as classes de `MathClass` no vanilla (`lab/typst-original/`, provavelmente em `math.class()` ou tabela de classes usada em P772y para espaçamento automático) e comparar com as 10 já implementadas no cristalino (achado adjacente já tratado em P772y/P795). Identificar as 5 que faltam.

### Implementação
Adicionar as classes em falta, com o mesmo mecanismo já usado para as 10 existentes (mesma tabela THIN/MEDIUM/THICK de P772y).

---

## Sub-achado B — field access bare compila em modo math (não devia)

### Sonda
Testar `$ x.campo $` (acesso a campo sem `#` dentro de modo math, sem ser uma chamada explícita de função) nos dois binários — confirmar que o vanilla rejeita nesse contexto (ou trata de forma restrita) e o cristalino compila sem restrição. Relacionar com o achado adjacente já registado no handoff antigo (P782, "splice de `#expr`/field-access bare em modo math") — confirmar se é o mesmo mecanismo ou uma lacuna nova dentro dele.

### Implementação
Depende da sonda — se for a mesma área de P782, a correcção deve reforçar a validação já implementada lá, não duplicar.

---

## Sub-achado C — `fence` espaçado sem efeito

### Sonda
Testar a sintaxe de fence com espaço (a forma exacta está no relatório de materialização de P810 — conferir antes de assumir) — confirmar que o achado já regista isto como "scope-out L0" (ou seja, já foi decidido não implementar e documentado). Se for esse o caso, este sub-achado não precisa de código — só confirmar que a decisão está de facto registada no L0 correspondente e, se não estiver, formalizá-la.

### Implementação
Só se a sonda revelar que a decisão de scope-out não está formalizada — nesse caso, formalizar (entrada de dívida ou nota no L0), não implementar a funcionalidade sem decisão explícita (mesmo padrão de P807/P812-C).

---

## Sub-achado D — `LeftRightAlternator` em `mat` (matriz) ausente

### Sonda
Testar a funcionalidade de alternância de delimitadores esquerda/direita em `mat()` (matrizes) — confirmar a sintaxe exacta no vanilla e testar se o cristalino a ignora ou rejeita.

### Implementação
Implementar o mecanismo no layout de `mat()`, replicando o comportamento do vanilla.

---

## Validação (comum aos quatro sub-achados)

1. Recompilar. Repetir todos os comandos de sonda, saída literal batendo com o vanilla (ou decisão formal registada, para o sub-achado C se for o caso).
2. Testes novos por sub-achado, nomeados `p825a_...` a `p825d_...`.
3. Suíte `typst-core` completa, comando + contagem antes/depois.

## Relatório

`00_nucleo/diagnosticos/typst-passo-825-relatorio.md`, um bloco por sub-achado (A-D), cada um com medição antes, código identificado, diff (ou decisão registada), medição depois. Contagem de testes final consolidada.
