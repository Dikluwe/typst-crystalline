# P1321 R2 — recibo A/B sucessor

**PASS delimitado ao contrato congelado R2.** Focal: 140/140; integral:
1548/1548, zero falhas e zero Unknown. As 516 saídas por ordem são idênticas
em normal/repeat/reverse. Execução **sem atestação técnica de isolamento**.
Não foram lidos produto, testes cristalinos, diff real ou JSONs do coordenador.

## Cadeia causal e entradas

A falha R1 permanece em `p1321-ab2-candidate.json`, SHA-256
`d1699b5ae42327c7ba7254f5f1f2830453d0cd225fce8f79807cf329aa2a1a25`:
missing source tinha mensagem correta e âncora incompleta. O coordenador
reabriu os dois L0s antes do candidato R2; ambos foram lidos integralmente.
Loading continua dono da validação; dispatch legitima transporte da chamada
inteira por identidade nativa CSV e With recursivo. O resultado esperado
original não mudou.

O freeze R1 `p1321-ab2-freeze.json` continua byte-idêntico, SHA-256
`b90c53cc2d982160dce2faca2f4afb6910d07b06e321b8bf8090bc51ead0d6ca`.
Também permanecem íntegros:

| Entrada R1 | SHA-256 |
|---|---|
| cases | `74597d5240431f79c8dfde8ff53589458e7cbe94481a0dcacfcfbe79c5c14d26` |
| expectations | `e33302de9ba05691d9e74388b76c06eb7b909db435e18b4d42e8ecc76616d296` |
| runner | `4feb2a60445d6aa51c35a829d91d679a6617f3bc7fde31f5a7a26e0d07f8d3e8` |

Freeze sucessor `p1321-ab2-r2-freeze.json`, SHA-256
`67c6b92d62018b1cc8673fe28e75d8c55fd28f1c4e15d15ab6a4bb535ecfc8dd`.
O manifesto referencia os originais, a falha R1 e as novas medições/expectativas.
Hashes normativos, excluindo somente a única linha canônica `Hash do Código`:

| L0 | SHA-256 normativo |
|---|---|
| `compiler/stdlib/loading.md` | `aae08307fdd8602d6a4bd2700d282b89aa35ea5d4e28e189f64ae3bbb110558b` |
| `compiler/eval/call_dispatch.md` | `95c34064e36713e44f239d168029153d3c8010d2af5c9a5984369d61ffc9bfa7` |

## Medição nova anterior ao candidato

Somente os controles novos foram medidos: 20 fontes × quatro perfis × dois
binários = 160 processos, custo acumulado 9,53 s. O corpus original não foi
reexecutado nesta fase. Baseline R1 preservado:
`/tmp/p1321-r1-preserved.Tdbgy8/typst`, SHA-256
`deb6aed85d133665b00d416fabee92aeda04196ff76b6a734ef0451bbdeaea3c`.
Vanilla `/usr/local/bin/typst`, upstream ratificado `a51e02804`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.

Medição em `2026-09-08T20:42:20.570643Z`, working tree não commitado sobre
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`:

```text
00_nucleo/prompts/compiler/eval/call_dispatch.md |  51 ++-
00_nucleo/prompts/compiler/stdlib/loading.md     | 198 +++++++++-
01_core/src/compiler/eval/call_dispatch.rs       |  84 ++++
01_core/src/compiler/stdlib/loading.rs           | 466 ++++++++++++++++++++---
4 files changed, 749 insertions(+), 50 deletions(-)
```

`p1321-ab2-r2-measurement.json` SHA-256:
`a03ecad03af222ca9979650933e759929ac4b1c4dee4e2667ae50bb07fae67b9`.

Foram congelados 19 controles aplicáveis: função usuária chamada csv (direta,
alias, With e valor), read/json/CBOR missing, selector float vigente, três
encoders e With, panic e With, função usuária panic, CSV With encadeado,
origem source-named e duas chamadas math por alias/With. As expectativas dos
controles protegidos usam a saída R1 integral; as de CSV usam vanilla integral.
Os controles novos produziram 12 células RED no baseline R1, todas corrigidas
no candidato R2.

Uma sonda exploratória foi explicitamente excluída do gate antes do candidato:
`$csv()$` não resolve csv no namespace math vanilla, mas o cristalino R1 já
resolve esse nome global e chega ao missing. A medição bilateral integral
permanece preservada. Essa diferença prévia está fora do recorte R2 e não é
convertida em paridade nem em sucesso. As formas por alias e With alcançam a
função CSV nos dois binários e integram o gate. Não há alegação de paridade
math geral.

## Candidato e resultados

O coordenador liberou `/tmp/p1321-target.669PuL/release/typst`, SHA-256
`756b1c85a5879ea0afb79fc35184525aebfafa6ccb926ae86679a868691f99fa`.
O SHA foi confirmado antes da execução. O focal iniciou em
`2026-09-08T20:49:14.980728Z` e a integral em
`2026-09-08T20:49:38.749322Z`, mesmo HEAD, working tree não commitado:

```text
00_nucleo/prompts/compiler/eval/call_dispatch.md |  53 ++-
00_nucleo/prompts/compiler/stdlib/loading.md     | 198 +++++++++-
01_core/src/compiler/eval/call_dispatch.rs       |  92 ++++-
01_core/src/compiler/stdlib/loading.rs           | 466 ++++++++++++++++++++---
4 files changed, 756 insertions(+), 53 deletions(-)
```

Focal: oito casos falhos R1 + oito fronteiras originais + 19 controles novos,
35 × quatro perfis = 140 PASS. O resultado GREEN foi comunicado ao coordenador
antes da integral. Custo acumulado: 35,56 s. Artefato
`p1321-ab2-r2-focal.json`, SHA-256
`1b9bf87a405c47dccd0ab603bc8de51830bfc25f7bea718230a19055aecf8f94`.

Integral: todos os 110 casos originais + 19 controles novos, quatro perfis,
três ordens = 1548 PASS. Custo acumulado: 186,59 s. Artefato
`p1321-ab2-r2-full.json`, SHA-256
`d384d510f380b6cc67bada50a7cb6270b5a68cc01611891db728bfdea06e93c7`.

O runner verificou hashes de todos os inputs e das duas normas antes/depois
de cada gate. Compara exit, stdout e stderr completos: mensagens, hints,
âncoras, traces e saídas não são normalizados. Nenhuma expectativa original,
inclusive os oito missing antes falhos, foi enfraquecida ou substituída.

Reprodução, usando o binário cujo SHA foi registrado:

```text
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1321-ab2-r2-runner.py focal --binary /tmp/p1321-target.669PuL/release/typst
PYTHONDONTWRITEBYTECODE=1 python3 00_nucleo/diagnosticos/p1321-ab2-r2-runner.py full --binary /tmp/p1321-target.669PuL/release/typst
```

## Limites do veredito

Este A/B confirma somente os diagnósticos/valores públicos nas fontes e
perfis congelados. Não atesta sandbox de capacidades, API Rust sintética,
contadores internos de I/O, lint, build ou testes unitários: esses gates
pertencem ao coordenador/revisor. A evidência negativa de captura nominal é
comportamental, não inspeção do selector. Nenhuma fonte produtiva foi lida.
As dívidas anteriores de parsing Path/resolução, Symbol e math direto seguem
explicitamente fora de qualquer alegação de equivalência geral.
