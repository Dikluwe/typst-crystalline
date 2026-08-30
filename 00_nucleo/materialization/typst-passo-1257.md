# P1257 — separar erro adaptativo de serialização SVG

**Estado:** EXECUTADO — OWNERS SEPARADOS; SEM CORREÇÃO PRODUTIVA  
**Predecessor:** P1256  
**Escopo:** instrumentação `#[cfg(test)]` do owner L3.

## Objetivo

Comparar o envelope contínuo produzido pelos stops de
`svg_adaptive_stops` antes da escrita XML com o envelope dos mesmos offsets
após `Color::to_srgb`, clamp e quantização u8 usados pelo exporter.

## Contrato diagnóstico

- medir Oklab e LinearRgb separadamente em malha 4096;
- usar cor sRGB codificada premultiplicada, como o helper vigente;
- conservar os offsets e apenas substituir endpoints float pelos endpoints
  serializados para o contraste pós-serialização;
- não mudar threshold, cap, algoritmo, budgets ou classificações P1237;
- instrumentação compila somente em testes e não cria API pública.

## Execução

`cargo test -p typst-infra p1257_pre_pos_serializacao -- --nocapture`

## Resultado

- Oklab: 85 stops; `pre_max=0.030778080` em `t=0.367919922`;
  `post_max=1.069208622` em `t=0.654785156` no contraste interno não clampado
  versus endpoints CSS clampados;
- LinearRgb: 65 stops; `pre_max=0.019714614` em `t=0.374023438`;
  `post_max=0.018983630` em `t=0.996093750`;
- os dois testes passaram e repetiram deterministicamente.

## Decisão

Oklab não pode ser corrigido por tuning cego do helper: a fronteira de gamut,
clamp e quantização altera materialmente o observável. O contraste acima não é
o orçamento público P1237 — usa o sample cristalino interno — e, por isso, não
promove nem reprova paridade sozinho. Ele prova que a próxima obrigação Oklab
deve congelar a política de gamut/serialização antes de alterar subdivisão.

LinearRgb não mostra amplificação pela serialização no witness sintético; seu
gap continua candidato ao critério/cap adaptativo. Os owners seguem separados:
Oklab → conversão/gamut/serialização primeiro; LinearRgb → helper adaptativo.

`EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO`
