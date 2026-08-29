# P1272 — certificado da conjunção completa dos quatro pares

**Estado:** EXECUTADO
**Baseline:** `upstream/main a51e02804` ratificado
**Candidate:** `f152d0dead810054996bdc8e1450022746b7dd2405d73d3237baa024297ae555`
**Regime:** verificação processualmente ordenada, sem atestação de isolamento

## Veredito

Os quatro pares fecharam independentemente como `Generalization-Preserved`:

- `linear/oklab`: 24/24;
- `radial/oklab`: 24/24;
- `linear/linear-rgb`: 24/24;
- `radial/linear-rgb`: 24/24.

A alegação limita-se ao envelope congelado de 96 fixtures. Não demonstra
equivalência SVG geral, não altera fallback e não autoriza promoção produtiva.
A rota logística resultante é P1273.

## Conjunção executada

Grafo, métrica numérica e raster fecharam 96/96. As quatro métricas numéricas
fecharam 384/384; os 28 probes inválidos foram rejeitados; direto, inverso e
repetido produziram 192/192 recibos semanticamente idênticos. Os 180
testemunhos de coincidência/right-continuity e todos os stops originais,
ordem e multiplicidade foram preservados.

O custo foi publicado para todos os 1.224 intervalos. Os totais foram
`M=1320`, `R=848`, `E=11188` e `sum(a_i)=9868`; máximos observados:
`s_i=64`, `a_i=31`, `d_i=63`, `b_i=62`, `q_i=125`, todos dentro da fórmula
selada. O S20 manteve stroke degenerado com 1.920 pixels de suporte em cada
par; o S21 manteve fill sem área com zero pixel nos quatro pares.

## Ataques, controles e gates

Os 24 mutantes válidos foram rejeitados (`mutation_score=1.0`), o controle
positivo foi preservado e os 10 casos opacos continuaram explicitamente
`Unknown`, todos excluídos do sucesso. O fallback produtivo permaneceu em
96/96 saídas e não houve promoção.

P1234, P1236, P1237, P1264 e sRGB foram revalidados por identidade causal: o
binário atual é byte-idêntico ao binário submetido pelo P1271, e os recibos e
artefatos congelados mantêm os mesmos hashes. Os diretórios temporários desses
corpora já não existiam, portanto não se alega uma nova execução deles. Os
testes focais e o workspace completo foram executados novamente e passaram.
Também passaram build, formato, sintaxe dos runners, diff-check,
V1/V5/V15/V26 e `crystalline-lint .` com zero violações.

## Proveniência e limitação de atestação

A primeira execução do envelope foi invalidada e descartada quando o gate de
formato mudou os bytes do probe diagnóstico. O envelope e a adjudicação foram
então repetidos integralmente com os hashes finais da fonte e do binário do
probe; somente a segunda cadeia fundamenta este certificado.

Contrato, execução, adjudicação e verificação final compartilharam a autoridade
`/root`, o filesystem e o contexto conversacional. Logo o resultado
proporcional é: **EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO**.
