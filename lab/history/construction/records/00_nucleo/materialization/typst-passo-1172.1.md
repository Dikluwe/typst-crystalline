# P1172.1 — materializar agrupamento phrasing no topo HTML

**Data:** 2026-08-25  
**Estado:** `EXECUTADO — GREEN; PARADO ANTES DE STAGING`  
**Baseline:** P1171.1 GREEN + P1172 classificado como fluxo contínuo  
**Vanilla:** `a51e02804`

## Objetivo

Ressellar L3, obter RED e corrigir somente `block_sequence`: tags phrasing
visíveis alimentam parágrafo no topo; boundaries block fecham o buffer.

## Aceitação

- `a`, `html.elem("a")` e `span` isolados recebem `<p>`;
- `span/div/a` produz p/div/p;
- parbreak separa parágrafos;
- phrasing dentro de `div` não recebe p;
- `a` transparente continua groupable;
- custom/display-none não são agrupados;
- P1171.1 void/whitespace não regride;
- suíte, build e lint GREEN; índice vazio.
