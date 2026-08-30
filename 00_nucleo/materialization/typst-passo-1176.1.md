# P1176.1 — materializar lote residual normal HTML

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN; PARADO ANTES DE STAGING`
**Baseline:** P1175.1 GREEN + P1176 aprovado
**Vanilla:** `a51e02804`

## Objetivo

Ressellar, obter RED e materializar `datalist`, `noscript`, `summary` como
global-only. Corrigir `summary` block e whitespace top-level junto a HtmlElem
não agrupável, preservando `datalist` inline dentro de bodies.

## Aceitação

- exatamente três bindings novos, zero específicos;
- 76 globais e body none/Content reutilizados;
- datalist/noscript/summary preservam as categorias medidas;
- nenhum parágrafo de whitespace junto a boundary top-level;
- summary remove espaço adjacente dentro de details;
- noscript vazio preserva pre-wrap e datalist interno continua inline;
- fixtures tipadas coincidem com vanilla;
- suíte/build/formato/lint GREEN;
- índice vazio e nenhum commit.

## Resultado

Os três bindings foram materializados pela via global-only. `summary` entrou
na tabela block e `block_sequence` passou a distinguir boundary de parágrafo
da proteção inline. Os REDs passaram a GREEN e as fixtures tipadas coincidiram
com o vanilla. Fecho em
`00_nucleo/diagnosticos/typst-p1176.1-materializacao-html-tags-residuais-normais.md`.
