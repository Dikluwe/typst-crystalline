# P1255 — localizar a causa dos envelopes SVG Oklab/LinearRgb

**Estado:** EXECUTADO — DIAGNÓSTICO; IMPLEMENTAÇÃO NÃO AUTORIZADA  
**Predecessores:** P1237, P1250, P1253 e P1254  
**Escopo:** medição causal do helper SVG comum; sem promoção de pares e sem
alteração produtiva.

## Objetivo

Ordenar as violações ainda abertas dos quatro pares Linear/Radial ×
Oklab/LinearRgb e determinar qual observável deve orientar o próximo teste RED.
O passo não aumenta budget, não reduz threshold por tentativa e erro e não usa
o sucesso integrado P1250 para converter `Unknown` em `Preserved`.

## L0 vigente

O owner auditado é
`00_nucleo/prompts/infra/export/gradients/adaptive.md`, materializado por
`03_infra/src/export/gradients/adaptive.rs`. O L0 já exige subdivisão SVG
determinística, métrica sRGB premultiplicada, threshold `0.001`, cap `64`, alpha
separado e aceitação por máximos/p95 contra o vanilla. Nenhuma mudança L0 ou de
código é legitimada apenas pelos agregados deste passo.

## Medição

Consumir somente `p1237-oklab-linear-rgb-numeric.tsv` e calcular, para cada
fixture e métrica, `cristalino - limite vanilla`. Valores positivos são
violações. Manter geometria e espaço separados, mesmo quando Linear/Radial
produzem números iguais.

## Gate para o próximo passo produtivo

Antes de editar o helper, é obrigatório localizar a posição `t`, o intervalo
de stops e o estado de subdivisão da primeira divergência de cada família. O
teste RED deve discriminar pelo menos:

1. erro no máximo com p95 já dentro do envelope;
2. erro distribuído que também viola p95;
3. erro de alpha independente do erro de cor;
4. diferença anterior à serialização versus diferença introduzida por
   offset/cor/opacity serializados.

Se o witness divergir antes do helper, o owner deve ser reclassificado. Se a
diferença aparecer somente após serialização, não se altera `adaptive.rs`.

## Resultado

- 24 fixtures medidas; 18 violações e 6 preservadas;
- `color_max` é o observável dominante: falha em 16 fixtures;
- `color_p95` falha em 6; `alpha_max` falha em 6; `alpha_p95` falha em zero;
- Oklab possui os maiores excessos (`0.00269199686366` e
  `0.00258836060518`) em fixtures fill; portanto não é válido tratar o problema
  como apenas stroke ou alpha;
- LinearRgb tem excesso máximo focal `0.0000784750265825` no witness
  `two-wide-stroke`, enquanto seu p95 já passa nesse caso;
- Linear e Radial repetem exatamente os mesmos vetores por fixture nesta
  evidência. Isso é uma observação do corpus, não autorização para promover ou
  corrigir os dois por inferência.

Veredito: a próxima unidade deve instrumentar os witnesses no domínio contínuo
e separar sampling de serialização. Nenhuma correção produtiva é segura ainda.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`
