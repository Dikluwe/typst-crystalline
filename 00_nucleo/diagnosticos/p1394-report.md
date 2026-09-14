# P1394 — verificação final do observador PNG

## Veredito

**PASS**, limitado ao contrato P1394 e **executado sem atestação de
isolamento**. O instrumento agora passa PPI 144 explicitamente ao vanila
ratificado somente em `P1137-X-003` e `P1138-X-002`; o cristalino, o produto,
os comparadores, os estados esperados, as fixtures e o L0 não mudaram.

Isso corrige o observador. Não implementa `--ppi` no cristalino, não apaga a
diferença raster e não demonstra equivalência funcional geral.

## Proveniência e escopo

A verificação ocorreu em `2026-09-14T14:11:51-03:00`, sobre HEAD
`06e136d19fdc6d5f83fbbe44414b8b91e4dfd658` com working tree não commitado.
O diff produtivo tinha três arquivos: 175 inserções e 2 remoções. Todos os
hashes protegidos por `p1394-preseal.json` conferiram.

O manifesto difere do baseline somente pela anexação do par terminal
`--ppi`, `144` aos dois arrays `oraculo.args` autorizados. A comparação AST do
runner encontrou uma única definição de topo alterada, `validate_manifest`,
e igualdade do módulo fora dessa função. O teste independente conservou o
hash selado `b49a8372fda516ea9142e571e318a319318fe06e69bbbcf5d368cd6de4c85eb0`.

Os binários também conservaram as identidades contratadas:

- vanila ratificado `586e1bd43fae6c9a973218163d3165c53ab8d16d`, SHA-256
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
- cristalino, SHA-256
  `9088dfe1b298482722476bb3e4d6d5245f56d3ed26f34f73eb77af7752719479`.

## Gates funcionais e de memória

O verificador repetiu os gates que não executam Typst:

- `python3 -m unittest lab/parity/matrix/test_runner.py`: 50/50 testes,
  exit 0, RSS máximo 25.972 KiB, swap 0, limite virtual 512 MiB;
- `runner.py --validate-only`: manifesto válido com 20 casos, exit 0, RSS
  máximo 21.452 KiB, swap 0, limite virtual 512 MiB;
- `crystalline-lint .`: zero violations, exit 0, RSS máximo 213.980 KiB,
  swap 0, limite virtual 2 GiB. Há warnings preexistentes fora do escopo;
- `git diff --check`: exit 0.

As duas execuções focais fornecidas pelo executor foram seriadas, com
`ulimit -v 2097152` e timeout externo de 60 s:

| Caso | Estado | Ordem | Dimensões | RSS máximo | Swap | Tempo |
|---|---|---|---:|---:|---:|---:|
| `P1137-X-003` | MATCH esperado | idêntica | 1191×1684 | 51.744 KiB | 0 | 0,95 s |
| `P1138-X-002` | DIFFERENCE esperada | idêntica | 1191×1684 | 388.944 KiB | 0 | 26,15 s |

Os oito PNGs preservados — dois lados, duas ordens e dois casos — são
legíveis e têm 1191×1684. Em cada lado, os hashes forward/reverse coincidem.
O raster mantém a medição substantiva: 11 de 2.005.644 pixels diferentes e
delta máximo de canal 1.

O verificador não repetiu os binários focais: correlacionou os JSONs e PNGs
persistidos com as medições de recursos fornecidas pelo executor. Nenhum
comando PNG sem o PPI contratado foi executado nesta fase.

## Gate adversarial

Os 21 mutantes válidos foram avaliados duas vezes em ordem normal e duas em
ordem inversa, sempre a partir de uma cópia limpa. O vetor canônico foi
idêntico nas quatro passagens, SHA-256
`83ea46bc0a84b064e7f673bf738ffccbf71ece119293e8cab39e217de062e725`.

- M01–M10 foram rejeitados por `validate_manifest`;
- M11–M15 foram rejeitados pelo gate de delta exato contra o baseline;
- M16–M19 foram rejeitados com mocks/classificadores puros, preservando
  `BASELINE_DEFECT`, `HARNESS_DEFECT` e `UNKNOWN` sem crédito;
- M20–M21 foram rejeitados pela inspeção do recibo de teto e serialização.

Resultado: 21 rejeitados, 0 sobreviventes, 0 `Unknown` inesperados,
`mutation_score = 1.0`. Os mutantes perigosos existiram apenas em memória;
nenhum processo vanila foi iniciado por esse gate.

## Limite da conclusão

O P1394 fecha o defeito de medição provocado pelo PPI implícito perigoso.
`P1137-X-003` volta a medir capacidade como MATCH, enquanto `P1138-X-002`
continua corretamente DIFFERENCE. A falta da opção pública `--ppi` no
cristalino permanece uma lacuna separada.

Como os papéis compartilharam o mesmo workspace, a formulação correta é
“executado sem atestação de isolamento”. Os hashes demonstram integridade das
entradas protegidas, mas não provam isolamento físico ou ausência de leitura
cruzada.
