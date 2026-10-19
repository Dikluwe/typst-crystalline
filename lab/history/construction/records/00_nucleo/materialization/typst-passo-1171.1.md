# P1171.1 — materializar `html.br`, void e whitespace local

**Data:** 2026-08-25  
**Estado:** `EXECUTADO — GREEN; PARADO ANTES DE STAGING`  
**Baseline:** working tree P1170.1 GREEN + P1171 aprovado  
**Vanilla:** `a51e02804`

## Objetivo

Ressellar os L0s, obter RED e materializar exatamente `html.br`, a tabela das
13 tags void e a normalização L3 de whitespace medida. Não alterar entidade,
pipeline ou agrupamento phrasing de topo.

## Execução

1. ressellar L0s L1/L3;
2. RED para binding, serialização void, filho void inválido e bordas de body;
3. constructor sem body, 76 globais e `HtmlBody::Unset`;
4. tabela void única em L3, start tag sem fecho e erro para body content;
5. remover somente espaços de borda do body HTML e adjacentes a `br`,
   preservando espaço entre inline siblings;
6. decalcar fixtures compactas contra o vanilla;
7. suíte integral, build, formato, lint e diff;
8. diagnóstico e parada antes de staging/commit.

## Aceitação

- exatamente um binding público novo;
- zero específicos e zero body;
- 13 tags void, sem slash/end tag;
- body void rejeitado no export;
- `A<br>B`, spans formatados e blocks coincidem;
- `Linebreak` não regride;
- agrupamento phrasing permanece fora;
- tudo GREEN e índice vazio.
