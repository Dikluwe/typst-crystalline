# P1393 — relatório da auditoria global de paridade

## Veredito

**CONTINUE_WITH_EXPLICIT_GAPS.** A matriz atual tem 20 casos: 17 `MATCH`, uma
`DIFFERENCE` e dois `UNKNOWN` quando cada caso é contado no perfil em que está
ativo. Nas 54 células ativas dos três perfis, são 45 `MATCH`, três
`DIFFERENCE` e seis `UNKNOWN`. Os resultados direto e reverso foram idênticos.

Isso não é paridade geral. O inventário público mais amplo P1357 contém 8.724
células profile/path, mas mede presença/kind com um binário cristalino anterior
ao HEAD. Seus 145 gaps retidos e 127 paths de target drift continuam anexos
nominais; P1393 não os promove a estado semântico atual sem reexecução.

## Evidência principal

Proveniência congelada em `2026-09-14T13:47:55-03:00`:

- HEAD `c1f6f3930fa2fce80cb6e6b7765ee69c34de3b83`;
- working tree inicial: somente
  `?? 00_nucleo/materialization/typst-passo-1393.md`;
- vanila `586e1bd43fae6c9a973218163d3165c53ab8d16d`, binário
  `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`,
  52.149.176 bytes;
- cristalino rebuildado do HEAD, binário
  `9088dfe1b298482722476bb3e4d6d5245f56d3ed26f34f73eb77af7752719479`,
  49.150.488 bytes;
- manifesto
  `8f443c60e426d814f931ec4c9097406275e4c8b83ec70a2c8b793edd344f127d`.

O rebuild era necessário porque o executável anterior reportava `d0ab401c`,
uma revisão atrás. `cargo build --release --bin typst` terminou em 5,94 s,
pico de 354.360 KiB, zero swap, sob teto de 4 GiB/180 s. O build emitiu os
warnings já existentes; nenhum warning foi tratado como erro nem corrigido
neste passo diagnóstico.

## Matriz

| Perfil | MATCH | DIFFERENCE | UNKNOWN | Desativado | RSS máximo |
|---|---:|---:|---:|---:|---:|
| default | 14 | 1 | 2 | 3 | 51.760 KiB |
| html | 16 | 1 | 2 | 1 | 51.796 KiB |
| a11y-extras | 15 | 1 | 2 | 2 | 51.620 KiB |

Todos rodaram serialmente sob 2 GiB, timeout externo de 600 s, timeout interno
de 30 s por processo e zero swap. Exit 1 significa expectativa não satisfeita,
não crash do runner.

Os JSONs temporários integrais são identificados por:

- default `ec039d13340cda4d52a37b4616f6e0da6856b60e6c1373ebf6c178998587aa4d`;
- html `ddafeb1cc8b6fe95e41de72672dca24f533e3d7a98964c460611cfd55668aa38`;
- a11y-extras `6707bdc76b74b6f3ff2205780282aec4c2f429b95d5238736ad48ec20f1fedb5`.

## Incidente de memória contido

`P1137-X-003` e `P1138-X-002` fazem o vanila abortar com
`memory allocation of 41568028160 bytes failed` quando o PPI é implícito. O
limite de 2 GiB impediu consumo dessa magnitude; o pico do processo observado
antes do abort ficou abaixo de 27 MiB. A repetição focal confirmou a causa.

O help vanila declara PPI default 144. Com `--ppi 144`, o mesmo documento gera
PNG 1191×1684, exit 0, em 0,30 s e 43.212 KiB. O cristalino sem flag gera as
mesmas dimensões, mas rejeita `--ppi`. Logo:

1. os dois casos da matriz permanecem `UNKNOWN`, não falha do cristalino;
2. o harness deve explicitar 144 somente no comando vanila;
3. `--ppi` ausente no cristalino é lacuna de CLI separada;
4. o comando perigoso não deve ser repetido sem limite.

## Gap de linguagem confirmado

Duas fixtures temporárias, uma característica por caso, demonstraram a
precedência lexical em math:

| Caso | Vanila | Cristalino | Estado |
|---|---|---|---|
| closure local `abs` | `LOCAL-ABS` | `|1|` | DIFFERENCE |
| closure local `sqrt` | `LOCAL-SQRT` | raiz nativa de 1 | DIFFERENCE |

Todos os processos terminaram com exit 0 e zero swap. Os hashes das fixtures
são `8a2f9a49...` e `856ac0ad...`. O observável é morfologia/texto público, não
estrutura Rust ou bytes de PDF.

## O que falta

O inventário e a fila completos estão em
`p1393-paridade-inventario-atualizado.md` e `p1393-paridade-gaps.md`. Em ordem:

1. selar o observador PNG com PPI explícito, removendo dois UNKNOWN sem OOM;
2. corrigir a precedência local `abs`/`sqrt` no owner math;
3. estabelecer ownership 1:1 para a família `format.*`;
4. reexecutar e classificar semanticamente as 145 rotas retidas;
5. dividir loaders/repr/diagnósticos e layout restante por owner e witness.

O próximo passo único é **P1394, correção do observador PNG**. Ele deve ser
harness-only. O primeiro passo produtivo depois dele será a precedência local
em math.

## Gates e limites

- validação estrutural do manifesto: 20 casos, exit 0;
- três perfis executados; ordem direta/reversa estável;
- nenhum OOM do processo coordenador, nenhum swap e nenhum lote paralelo;
- produto, L0, manifesto e fixtures permanentes não foram alterados;
- nenhuma equivalência geral é alegada;
- regime de diagnóstico de autor único, sem alegação de independência.
