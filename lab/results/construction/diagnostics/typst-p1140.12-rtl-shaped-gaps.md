# Diagnóstico P1140.12 — quebra explícita e gaps RTL

**Data:** 2026-08-24  
**Baseline:** vanilla pinado `a51e02804`  
**Resultado:** RTL true/false em paridade nas sondas

## Causa e correção

`layout_bidi::reflow_rtl_paragraphs` fundia a linha seguinte porque o Frame
não transportava a causa do flush. Z reaparecia na primeira baseline inclusive
no controle false.

A correção adiciona `SemanticKind::ExplicitLinebreakBoundary`, envelope
transparente emitido por `layout/linebreak.rs`, e uma barreira absoluta em
`same_paragraph`. Tags PDF continuam exclusivas de `SemanticKind::Formula`.
A reconciliação shaped RTL preserva gaps originais e borda direita.

Não há limiar nem delta empírico na implementação.

## Oracle

True e false recompilados produziram caixas idênticas ao vanilla, inclusive Z
na segunda baseline. A sonda true termina a última palavra visual na margem
esquerda e mantém a primeira na margem direita; a false preserva gaps normais.

## Validação e proveniência

Em `2026-08-24T12:12:30-03:00`, base
`ca28f4ab74ae66985cdc66805c16c2ddc8f08366`, árvore não commitada:

- diff tracked: **56 ficheiros, 1242 inserções, 211 remoções**;
- status: **65 entradas**;
- L1: **5155 passed**;
- L3: **830 passed**;
- build, fmt, crystalline-lint e diff-check: exit 0.

As contagens incluem os passos acumulados na mesma árvore.
