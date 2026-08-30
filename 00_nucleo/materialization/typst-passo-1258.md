# P1258 — testar envelope uniforme ideal antes de corrigir o helper

**Estado:** EXECUTADO — UNIFORMIZAÇÃO REJEITADA  
**Predecessores:** P1255–P1257

## Objetivo

Usar os samples públicos vanilla congelados para medir um limite superior:
qual cap uniforme em potências de dois fecha `color_max` e `color_p95` de cada
fixture, sem depender da implementação candidata. O ensaio não mede alpha e
não é proposta de implementação; quantidade literal de stops é mecânica.

## Gate

- se o envelope vanilla uniforme com cap 64 não fechar uma fixture, aumentar o
  cap do helper cristalino não é correção causal suficiente;
- se fechar mas o SVG cristalino falhar, investigar amostragem/conversão antes
  da serialização;
- nenhum resultado promove pares, altera budgets ou autoriza uniformização da
  produção.

## Saídas

- `p1258-uniform-curve.tsv`;
- `p1258-uniform-minimum.tsv`;
- `p1258-summary.json`;
- `p1258-manifest.tsv`.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`

## Resultado executado

As duas execuções foram byte-idênticas. Com cap uniforme 64, considerando
somente cor, 16/24 fixtures fecharam e 8/24 continuaram violadas.

As oito falhas são os pares Linear/Radial dos mesmos dois padrões em Oklab e
LinearRgb:

- `two-wide-stroke`: p95 passa, mas `color_max` permanece acima do envelope;
- `alpha-mid-wide-stroke`: `color_max` permanece acima do envelope, embora o
  ensaio ainda não adjudique alpha.

Nos demais padrões, Oklab exige cap uniforme 64; LinearRgb fecha entre 32 e 64.
Isso demonstra que trocar o helper por amostragem uniforme até 64 não fecha o
contrato, mesmo usando samples públicos vanilla ideais. A política é rejeitada
como implementação.

## Próxima obrigação

Os dois padrões residuais precisam de refinamento localizado fora do midpoint,
sem aumentar o budget e sem uniformizar todo o intervalo. O próximo teste RED
deve atacar quartos/maior erro do segmento e provar custo limitado. Até essa
prova, as 18/24 violações públicas originais continuam vigentes e nenhuma
alteração produtiva é autorizada.
