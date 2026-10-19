# P1217 — fechamento de `ordering-comparables`

**Resultado:** `ORDERING COMPARABLES GREEN — QUEUE RESOLVED`  
**Data:** 2026-08-26  
**Regime:** protocolo completo numa única sessão, **sem atestação de isolamento**.  
**Vanilla ratificado:** `a51e02804`.

## Veredito

Os quatro ramos públicos medidos no P1216 agora coincidem com o vanilla:

- `Fraction ↔ Fraction`, sem coerção cruzada;
- `Decimal ↔ Int` nos dois sentidos, com conversão decimal exata;
- `Ratio ↔ Relative` nos dois sentidos, somente sob `abs == 0`;
- `Datetime` entre kinds homogêneos, com erro nominal entre `date`, `time` e
  `datetime`.

Arrays propagam os novos resultados e os erros de kind internos. Nenhuma
entidade pública, sintaxe, igualdade ou fase do pipeline mudou.

## Medição e refinamento

A matriz final reuniu os 24 casos P1216 e 34 casos P1217. As duas rodadas
foram byte-idênticas e obtiveram `58/58 MATCH`. Quatro sondas por ficheiro
coincidiram em mensagem, linha, coluna e extensão. A forma do path `/tmp`
continua relativa no vanilla e absoluta no cristalino, fora deste cluster.

Os 14 mutantes dirigidos possuem testemunha e foram rejeitados:
`mutation_score = 14/14 = 1.0`. Não houve motor externo nem isolamento real;
o score é reproduzível, mas não independentemente atestado. Literal negativo
de fraction foi rejeitado pelos dois parsers e permaneceu `Unknown` para a
semântica do dispatcher.

## Proveniência inicial

Congelamento em `2026-08-26T15:45:40-03:00`, HEAD
`e7b962921c7fc5a13a4000da452e9dfb4469a245`, working tree não commitado com
as alterações P1215/P1216 listadas por `git diff HEAD --stat`.

- vanilla: `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- cristalino inicial: `351c6446e29804978be883fcf250513ea9786f52abefbf6d9cfe79fe6aae235f`;
- lente: `9b49489c4afbbe38199cf8025a386eac235a3435bb9657e781f3e97e0435fc18`;
- mapa inicial: `2cb50971a326c4d1a2ac3c97af16f908558f199a1c80df721818a0025ee83dc6`;
- passo: `46d760981bf0610ea5e678b9f7614bda71fb7a153d8e1099b40df2b72671bebc`;
- duas rodadas A/B: `ac33163aaf1cd14158833fe8a3d8f567adf31b9dadd0e7f974e7ba8aa00053ee`.

## Proveniência final e gates

Gates concluídos em `2026-08-26T15:52:21-03:00`, no mesmo HEAD e working
tree acumulado:

- cristalino release final: `cf5cac11d5d0105f8a2271a367ae0d2fc147c9550473bc82e10b639f9343dda8`;
- mapa final: `938ade7a18f01eb4ec9b756ffaee927b2839ea861a9116ab946d2550cffa63cc`;
- duas lentes com mapa: `5d03c8ac0954f31705e8b451c8e437698ecd950c1400d4cbb257f7fa1ebabc29`;
- oráculos: `2bd9b1648afd2f1b4bdbe42fa9462f9d2742624c8671e82962a1451177cf382f`;
- resultados: `af98476ec3198f3b677bbc3d29504bade1ad50daaa381f304ec6b5298ac844de`;
- ataques: `d122b3cd1d50da16091432e3ff5766de99aea91973d3cfbcdea01b3ba0b1c6e9`.

Gates:

- P1217: 4/4; P1216: 3/3;
- `cargo test --workspace`: todos os testes funcionais passaram; o teste
  temporal de watch repetiu o timeout conhecido de 20 s e passou isolado;
- `cargo build --workspace --quiet` e release: GREEN;
- `crystalline-lint .`: exit 0, sem violations;
- `cargo fmt --all -- --check` e `git diff --check`: GREEN;
- lente com mapa: duas saídas byte-idênticas.

## Limite da alegação

`ordering-comparables` está fechado para a matriz pública declarada. A relação
DSM `eval-apply-binary` permanece `parcial`, pois operadores unários continuam
nominalmente `Unknown`; isto não declara equivalência geral do eval.
