# P1264 — consolidar as correções SVG e readjudicar os quatro pares

**Estado:** EXECUTADO — QUATRO PARES CERTIFICADOS NO FRAGMENTO  
**Predecessores:** P1261, P1262 e P1263

## Objetivo

Reexecutar o corpus completo depois das correções focais, sem usar resultados
parciais para promover pares. Este passo não implementa novas correções: apenas
integra, repete, readjudica e certifica.

## Entradas seladas

- vanilla `a51e02804`;
- 24 fixtures e budgets P1237;
- contratos/ataques/receipts de P1261–P1263;
- L0s e consumers finais com hashes válidos;
- política `Unknown` vigente.

## Execução

1. reexecutar P1234 e P1236 para impedir regressão L1;
2. reexecutar P1237 duas vezes, em ordem direta e inversa;
3. exigir igualdade semântica dos outputs repetidos;
4. medir grafo, `color_max`, `color_p95`, `alpha_max`, `alpha_p95` e custo;
5. executar testes dos owners, workspace build/test, fmt, diff-check e lint
   Tekt V1/V5/V15/V26;
6. registrar HEAD, working tree, hora e SHA-256 de todas as entradas/saídas.

## Gate de promoção

Cada par é independente:

- grafo `6/6`;
- numérico `6/6` em todas as quatro métricas;
- zero `Unknown` na cadeia causal necessária;
- contratos focais e mutantes aplicáveis aprovados;
- nenhuma regressão nos controles.

Linear não promove Radial; Oklab não promove LinearRgb. Um par que falhar
permanece `Unknown-native-approximation` ou `Unknown-fallback`.

## Fechamento

Publicar tabela antes/depois — a base permanece P1255: 18/24 violadas e 6/24
preservadas —, manifesto e certificado limitado ao fragmento. Não alegar
equivalência geral nem mutation score ausente.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO` se aplicável.

## Resultado

Readjudicação e certificado:
`00_nucleo/diagnosticos/typst-p1264-svg-readjudication.md`.
Veredito: **EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO**.
