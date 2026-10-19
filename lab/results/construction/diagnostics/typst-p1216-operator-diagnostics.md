# P1216 — diagnósticos de operadores

**Resultado:** `OPERATOR DIAGNOSTICS GREEN — QUEUE RESOLVED`  
**Data:** 2026-08-26  
**Regime:** protocolo completo numa única sessão, **sem atestação de isolamento**.  
**Vanilla ratificado:** `a51e02804`.

## Veredito

O cluster estritamente nominal fechou. `in` e `not in` usam o spelling Typst
entre aspas simples; comparações parcialmente indefinidas preservam repr dos
valores e `with`; arrays propagam a primeira causa interna. Tipos distintos
continuam usando nomes longos e `and`.

A matriz final tem 24 casos, repetida duas vezes com resultado byte-idêntico:
20 coincidem entre vanilla e cristalino. Os quatro restantes não são falhas do
patch diagnóstico: são lacunas semânticas públicas descobertas para
`Fraction`, `Decimal↔Int`, `Ratio↔Relative` e `Datetime`. Por isso
`eval-apply-binary` permanece `parcial` e ganhou uma lista nominal reproduzível.

## Evidência e ataques

Os testes RED falharam por tipo-em-vez-de-repr e perda de causa em array; após
a implementação, 3/3 testes P1216 e o teste de fronteira P842 passaram. Os 12
mutantes dirigidos têm testemunha e foram rejeitados:
`mutation_score = 12/12 = 1.0`. Não houve motor externo nem isolamento real;
o score não é independentemente atestado. Operadores unários permaneceram
`Unknown`, como cluster separado.

Quatro sondas por ficheiro (`in`, `not in`, length e array) coincidiram em
mensagem, linha, coluna e extensão. A apresentação do path de `/tmp` difere:
vanilla mostra path relativo e cristalino absoluto; isso não foi promovido a
gap deste cluster.

## Proveniência

Congelamento: `2026-08-26T15:32:39-03:00`, HEAD
`e7b962921c7fc5a13a4000da452e9dfb4469a245`, working tree não commitado com
as alterações P1215 listadas por `git diff HEAD --stat` no início da execução.

- vanilla inicial: `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`;
- cristalino inicial: `3ab23bff36e3f8391c95b53982acff62d4a26d465d5ba7cf7c28b0beaa549a4c`;
- lente inicial: `31bc1f60041523037aa15fb0e6bc93bda8ffbe567c7bace2e1c4502c4372ef63`;
- mapa inicial: `89f8b8c0546152a0ffd50330d3ac0c81c2964c229c8dd859da999a4b7ebae017`;
- duas rodadas A/B: `08d015c40515a216a4fd046382facd930966071377b037666e716c775a1cf39a`.
- cristalino release final: `351c6446e29804978be883fcf250513ea9786f52abefbf6d9cfe79fe6aae235f`;
- mapa final: `2cb50971a326c4d1a2ac3c97af16f908558f199a1c80df721818a0025ee83dc6`;
- lente final: `9b49489c4afbbe38199cf8025a386eac235a3435bb9657e781f3e97e0435fc18`;
- duas execuções da lente com mapa: `08305162dcb6581d6769dfe4fa8072a362ee6602cbdbfba6606d36863a71bf30`.

## Gates finais

- `cargo test --workspace`: GREEN integral, inclusive watch;
- `cargo build --workspace --quiet`: GREEN;
- `cargo build --release --bin typst`: GREEN;
- `crystalline-lint .`: exit 0, sem violations (warnings informativos existentes);
- `cargo fmt --all -- --check`: GREEN;
- `git diff --check`: GREEN;
- lente com mapa: duas saídas byte-idênticas.

## Limite da alegação

O veredito fecha somente formatação e propagação diagnóstica binária medida.
Não declara paridade geral de operadores, da stdlib, de paths diagnósticos ou
dos quatro ramos semânticos descobertos.
