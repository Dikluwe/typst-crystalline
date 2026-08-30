# P1266 — generalizar o envelope SVG Linear/Radial × Oklab/LinearRgb

**Estado:** EXECUTADO — QUATRO PARES `Unknown-generalization`; FALLBACK MANTIDO  
**Predecessores:** P1264 e decisão B do P1265  
**Regime:** protocolo Tekt completo; nenhuma promoção produtiva autorizada

## Objetivo

Decidir se os quatro pares certificados apenas no fragmento P1237 podem ser
generalizados com segurança antes de voltar ao gate ADR-0127:

- Linear/Oklab;
- Radial/Oklab;
- Linear/LinearRgb;
- Radial/LinearRgb.

Este passo mede e certifica um envelope maior. Não edita L0, não altera
`paint_is_svg_native` e não remove o fallback `gradient-color-space`.

## Entradas congeladas

- vanilla ratificado `a51e02804` e binário `/usr/local/bin/typst`;
- certificado e manifesto P1264;
- L0s vigentes de entidade, stdlib e SVG;
- consumers finais `gradient.rs`, `adaptive.rs` e `svg.rs` com hashes válidos;
- política `Unknown` de `p1266-unknown-policy.tsv`;
- matriz `p1266-generalization-matrix.tsv`.

Qualquer alteração dessas entradas invalida o preseal e reinicia a cadeia.

## População válida

A matriz define 24 seeds aplicados separadamente a cada par, totalizando 96
fixtures válidas. O desenho é estratificado e cobre conjuntamente:

- 2, 3, 4, 8, 16 e 32 stops;
- offsets automáticos, uniformes, desiguais, concentrados nas extremidades,
  quase coincidentes e coincidentes no início/meio/fim;
- cores saturadas e não saturadas, extremos de gamut, alpha nas extremidades,
  alpha interior, transparência alternada e quase transparente/opaco;
- constructors (`anti_alias=true`), `sharp` (`false`) e `repeat` preservando o
  estado recebido;
- fill e stroke;
- identidade, rotação, escala anisotrópica, reflexão, skew, translação e
  composição de transformações;
- caixas square/wide/tall, quase degeneradas e degeneradas de largura ou altura
  zero;
- ângulos lineares canônicos, negativos, múltiplos de volta e próximos da
  costura;
- centro, raio, focal-center e focal-radius radiais válidos, inclusive quase
  tangentes e raios positivos muito pequenos.

O mínimo público é dois stops. Se um stop tem offset explícito, todos têm;
offsets válidos são monotônicos e pertencem a `[0,1]`. Violações dessas regras
entram como probes de rejeição, nunca como gradients válidos nem `Unknown`.

## Oráculos e observáveis

Antes de executar qualquer candidato, o autor de oráculos deve materializar o
mesmo source Typst no vanilla e congelar:

1. source, AST pública relevante e hash;
2. grafo SVG esperado: variant, URL resolvida, ausência de raster, papel,
   geometria e transformação aplicada uma vez;
3. stops públicos efetivos, ordem, coincidência, right-continuity, cores e
   alpha;
4. budgets `V_color_max`, `V_color_p95`, `V_alpha_max`, `V_alpha_p95` em malha
   8192, derivados somente do vanilla;
5. máscara raster local fixada pela geometria antes da comparação;
6. custo vanilla e cap candidato, sem usar aprovação do candidato.

As lentes são complementares:

- curva de paint em ponto flutuante antes de `u8`;
- morfologia/grafo SVG;
- raster local para geometria, transformação, anti-alias e caixas positivas;
- morfologia de área zero para caixas degeneradas, que não podem produzir
  paint de área positiva.

## Gate pair-local

Cada par é adjudicado isoladamente e só passa quando:

- 24/24 fixtures válidas passam grafo e morfologia;
- 24/24 passam simultaneamente os quatro budgets numéricos aplicáveis;
- caixas degeneradas satisfazem a obrigação de área zero;
- todos os probes inválidos aplicáveis são rejeitados pelo domínio;
- zero `Unknown` permanece na cadeia necessária;
- todos os mutantes aplicáveis são executados e rejeitados;
- `mutation_score=1.0`, excluindo apenas casos opacos declarados;
- execução direta, inversa e repetida é semanticamente idêntica;
- nenhum controle sRGB/P1234/P1236/P1264 regride;
- custo e cap são publicados por fixture.

Linear não promove Radial; Oklab não promove LinearRgb. Aprovação agregada
96/96 não substitui quatro decisões 24/24.

## Travas

- Nenhum fixture, budget, máscara ou seed pode ser escolhido depois de observar
  a aprovação candidata.
- Não ampliar budgets, apagar p95, quantizar antes da comparação ou inferir
  sucesso de outra geometria/espaço.
- `Unknown` nunca é sucesso e bloqueia apenas o par cuja cadeia necessita da
  evidência.
- O resultado máximo deste passo é `Generalization-Preserved` por par.
  Promoção produtiva volta obrigatoriamente ao dono em novo gate ADR-0127.

## Fechamento esperado

Publicar manifesto, contrato, oráculos, corpus, ataques executados, resultados
direto/inverso, custos, readjudicação pair-local e certificado limitado. Se
qualquer mutante sobreviver ou evidência necessária ficar `Unknown`, o par
permanece `Unknown-generalization` e o fallback continua.

## Fechamento executado — 2026-08-28

O corpus foi materializado com 96 fixtures válidas, 24 por par, e 28
instâncias de probes inválidos aplicáveis. O adaptador diagnóstico executou o
algoritmo vigente de `adaptive.rs`; o binário produtivo foi medido em separado
e manteve o fallback explícito em 96/96 fixtures.

Nenhum par satisfez a conjunção pair-local:

- Linear/Oklab: 5/24 fixtures integralmente preservadas;
- Radial/Oklab: 5/24;
- Linear/LinearRgb: 6/24;
- Radial/LinearRgb: 6/24.

Grafo do adaptador e raster local passaram 96/96, mas a superfície pública
passou apenas 22/96, os quatro budgets numéricos passaram 72/96, o cap global
de 64 stops passou 60/96 e a obrigação de área zero falhou nas quatro
instâncias S20 com stroke. O máximo observado foi 774 stops. Os 28/28 probes
inválidos foram rejeitados; direto/inverso/repetido produziu 192/192 recibos
preservados; 24/24 mutantes foram rejeitados (`mutation_score=1.0`).

Veredito: os quatro pares permanecem `Unknown-generalization`. Não existe
autorização para novo gate de promoção ADR-0127; L0, `paint_is_svg_native` e o
fallback `gradient-color-space` permanecem inalterados. A execução ocorreu sob
uma só autoridade `/root`, logo o limite de linguagem é
**EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO**.
