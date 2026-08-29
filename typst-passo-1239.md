# P1239 — corrigir ou delimitar Luma

**Estado:** FECHADO — LUMINÂNCIA L1 PRESERVADA  
**Predecessor:** P1238  
**Saída:** Linear/Luma e Radial/Luma preservados ou causa L1 explicitamente aberta.

## Objetivo

Atacar a maior divergência observada: cor máxima `0.721348211` e alpha máximo
`0.6`. Verificar definição de Luma, conversão de endpoints, interpolação e alpha
contra a fonte vanilla. Não tentar compensar em SVG um erro originado na
linguagem.

## Gate

Se a correção pertence a L1, criar/atualizar primeiro o L0 owner e obedecer ao
gate ADR-0127. Se exigir mudança pública, terminar este passo após o L0 e pedir
confirmação. Se a semântica fechar, reexecutar Linear e Radial separadamente;
caso contrário manter ambos `Unknown` e registrar o witness mínimo.

## Resultado

O witness `luma(red)` isolou coeficientes sRGB abreviados em L1. A linha Y foi
corrigida para os valores exatos de `palette 0.7.6` e o corpus P1236 foi
reexecutado: `0/42` pares com delta de luminância, `28/42` preservados e
`14/42` divergentes apenas pelo alpha `Known-Upstream-Bug` de P1252.
Linear/Luma e Radial/Luma estão preservados no fragmento público medido;
nenhuma whitelist SVG foi promovida. Os cinco ataques históricos não possuem
recibos de mutação reproduzíveis e foram revogados; não se alega mutation score.

Uma execução atual confirmou também `54.02%` para `luma(red)` e para os
endpoints Linear/Radial nos dois binários. Duas execuções completas foram
byte-idênticas.

Diagnóstico: `00_nucleo/diagnosticos/typst-p1239-luma.md`.

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
