# P1175.1 — materializar quarto lote HTML e proteção de espaço

**Data:** 2026-08-25
**Estado:** `EXECUTADO — GREEN; PARADO ANTES DE STAGING`
**Baseline:** P1174.1 GREEN + P1175 aprovado
**Vanilla:** `a51e02804`

## Objetivo

Ressellar, obter RED e materializar exatamente `nav`, `picture`, `pre`, `s`,
`samp`, `search`, `section`, `small`, `sub`, `sup`, `u`, `var`, todos
global-only. Corrigir em L3 a proteção de espaço isolado sem conteúdo inline
supportive nos dois lados.

## Aceitação

- exatamente 12 bindings novos, zero específicos;
- body none/Content e 76 globais reutilizados;
- quatro block e oito phrasing preservados;
- inline vazios produzem span `white-space: pre-wrap` entre eles;
- inline com conteúdo preserva espaço literal normal;
- fixtures tipadas coincidem com vanilla;
- suíte, build, formato e lint GREEN;
- índice vazio e nenhum commit.

## Resultado

Os 12 bindings foram materializados pela via global-only. O exporter passou a
distinguir boundary, inline vazio e conteúdo supportive, protegendo somente o
espaço que o HTML colapsaria. Os dois REDs passaram a GREEN e as fixtures
principal/adversarial ficaram byte-idênticas ao vanilla. Fecho em
`00_nucleo/diagnosticos/typst-p1175.1-materializacao-html-tags-global-only-lote4.md`.
