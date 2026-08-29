# P1262 — oito máximos interiores saturados

**Estado:** EXECUTADO — OITO WITNESSES FECHADOS, PROMOÇÃO NÃO APLICADA  
**Regime:** protocolo Tekt completo, executado sem atestação de isolamento

## Causa medida e owners

A matriz causal de 48 linhas localizou a primeira divergência produtiva em
`export/gradients/adaptive`: o vanilla limita a `[0, 1]` os canais sRGB float
das extremidades antes da mistura `0.5/0.5` e limita novamente exato e
aproximação antes de premultiplicar alpha. O candidato comparava os canais
fora de gamut. A decisão permanecia em float; quantização `u8` não era owner.

Depois dessa correção, a fronteira residual estava no owner SVG. O vanilla
serializa todos os offsets adaptativos Oklab por `Ratio::repr`, preservando a
ordem `offset * 100` seguida de arredondamento a duas casas. Converter antes a
`f32` ou condensar as multiplicações alterava empates binários. LinearRgb
conserva a representação localizada do P1261 e o sampler ponderado comum,
sem receber o clamp Oklab.

Não houve mudança em `Color`, API pública, comportamento por defeito, limiar
`0.001`, cap 64, stopsets ou budgets. O L0 dos dois owners foi atualizado
antes da implementação e o fluxo seguiu continuamente conforme ADR-0127.

## RED → GREEN e envelope

O contrato RED inicial expôs 58 stops candidatos contra 43 no vanilla para o
stopset base. A primeira correção isolada produziu 42 e revelou a segunda
fronteira; após alinhar a mistura das extremidades, as quatro contagens Oklab
ficaram em `43`, `19`, `39` e `37`, iguais para Linear/Radial e ao vanilla.

Os oito `color_max` ficaram, respectivamente, em:

- base: `0.0051554922420529335 <= 0.005156492`;
- coincident: `0.005082228550131032 <= 0.005083229`;
- alpha-first: `0.005783089524116328 <= 0.005784089`;
- alpha-mid: `0.006697761370810965 <= 0.0066987620000000005`.

Os p95 de cor também passaram e alpha não regrediu. Os resultados completos
estão em `p1262-saturated-results.tsv`; a matriz por fronteira está em
`p1262-boundary-matrix.tsv`.

## Regressão, P1234 e elegibilidade

P1234 permaneceu em zero divergências nos oito probes públicos. A varredura
P1237 completa passou 20/24 fixtures e não reabriu nenhuma das dez fixtures
preservadas no P1261. Linear/Oklab e Radial/Oklab medem agora 6/6 gráfico e
6/6 numérico; são elegíveis para um passo explícito de promoção, mas o P1262
não alterou o fallback nem o comportamento por defeito. Promoções aplicadas:
zero. LinearRgb permanece `Unknown` em 4/6 por variante.

## Ataques e reprodutibilidade

Sete mutantes executáveis foram rejeitados: ausência de clamp, clamp precoce
no espaço nativo, comparação somente após `u8`, sucesso automático por
saturação, coerção silenciosa a sRGB, exclusão de fixture e budget duplicado.
Mutation score: `7/7 = 1.0`; `Unknown` foi excluído do numerador. Duas
execuções completas produziram artefatos byte-idênticos.

Passaram `cargo fmt --check`, `git diff --check`, o teste de não regressão do
P1261, 5 testes do owner adaptativo, 43 testes do owner SVG, `cargo build` e
`crystalline-lint .` com zero violações. Os warnings Rust e informativos
V16–V20 preexistentes não são violações do linter.

Contrato, adversário, implementação e integração do veredito foram exercidos
sob a mesma autoridade `/root`. Os oráculos P1231 estavam congelados, mas não
há separação forte de capacidades. Veredito proporcional:
**EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO**.
