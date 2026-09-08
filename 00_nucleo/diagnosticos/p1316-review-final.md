# P1316 — veredito independente

**PASS no fragmento contratado:** origem dos erros de parsing da rota
nativa `csv(Bytes)`. Sem achado acionável na revisão de código ou nos gates.
Regime A/B executado sem atestação de isolamento técnico; não é selo de
refinamento completo nem prova de paridade geral CSV.

## Entradas e proveniência

Executor revisor `/root/p1316_review`; artefatos de intenção, implementação
e oráculos somente lidos. Escritas restritas a `p1316-review-*`.
Freeze independente `p1316-ab-freeze.json`, SHA-256
`200816280fc2e1c18934103574c6d33c3d972bdfb576221c4a74d3175d385e5f`.
O L0 normativo SHA-256
`f429ebf31bd6723cf8a69b7be82f2c223ced005bf367655ef17cadd26026b112`
permaneceu congelado; a exclusão é somente a linha canônica `Hash do Código`.
Fonte final SHA-256
`2278316a90408fc6dae9ba2fd0f3d5479571055ef4bcde84019147a84b12d75a`;
L0 raw SHA-256
`a4e4a71be35e5cf9008a47badb8a51e830616b6a768d0c185c24f78d79bce7d3`.

Auditoria reproduzível por
`node 00_nucleo/diagnosticos/p1316-review-audit.cjs`, script SHA-256
`455abda2b982284ee50300f34a983b840a2a64fa6a9b92b1771ba59c1eb22d8b`.
Resultado `p1316-review-final-audit.json`, SHA-256
`e34e68b1f1b94787f581159d6731d91905dbf2b224764031e0cb8169c23552b5`,
em `2026-09-08T14:47:02.620Z`, HEAD
`bc8213f36b7a29b4fdc30cfc74ddc23586117c64`, working tree não commitado:

```text
 00_nucleo/prompts/compiler/stdlib/loading.md | 124 +++++++++++++++-
 01_core/src/compiler/stdlib/loading.rs       | 213 +++++++++++++++++++++++++--
 2 files changed, 322 insertions(+), 15 deletions(-)
```

Binário final `/tmp/p1316-target.1c6HK7/release/typst`, SHA-256 conferido
diretamente
`1178fcde18dee54cb6b5c0feadcc79c8066346e0bb005060db7a2217a072e8ba`.
O baseline P1315 e vanilla ratificado `a51e02804` são identificados no
preflight e no freeze; a versão impressa não foi usada como identidade.

## Gates auditados

Todos os comandos e estados antes/depois estão nos recibos JSON abaixo.
Nos gates finais de build/testes/lint/fmt/linhagem, os hashes de fonte e L0
coincidem com os finais e permanecem iguais nos extremos de cada execução.

| Gate e recibo | Resultado | SHA-256 do recibo |
|---|---|---|
| RED `p1316-unit-red-r1.json` | Duas falhas de span e dois controles passando; código produtivo baseline | `41a1ec7626e8e37d61644af4608388b8a203e844a7320f966f35c830c48ec431` |
| GREEN `p1316-unit-green.json` | Quatro testes passaram | `5428ecdeaa3d85bcde4086f34d327e0abe7184c1bf03eb3b075b8f9a1bc3128e` |
| Build `p1316-build.json` | `cargo build --workspace --release`, exit 0 | `127c5eb0505f90012619906fc7e3bced8da2e12ec8a9db342043a7766b46a93f` |
| Workspace `p1316-workspace-tests.json` | 6652 passaram, nenhuma falha, três ignorados | `3c0f23a0b279b0cefaa9c4b223efa32959a7594ceaae7fa4f079280e49220486` |
| Lint `p1316-lint.json` | Zero errors, 240 warnings, 1137 infos; exit 0 | `7c81f950adb5b138338674f91293f47c75c09d9f133346716f3595bce92663f5` |
| Linhagem `p1316-lineage-final.json` | `Nothing to fix` | `4efd9b8066b2e86e58b55c451d2b45efb3a17d7d0ae428baf8830328e9688d8e` |
| Formatação `p1316-fmt.json` | `cargo fmt --all --check`, exit 0 | `f192f309b8fc06c00e052af2fcc5f843770adf4d9bb2e54623d9ec1b34ec5f53` |
| Diff `p1316-diff-check.json` | `git diff --check`, exit 0 | `886532fa18d14d56c62b79752d64c535aa47b164550bb4766983d1eb4ec14fc0` |
| A/B `p1316-ab-comparison.json` | 3960 comparações, nenhuma falha ou Unknown | `d80d70ca2b1a598df9491a6eab6b716fd44191975f366c405ba626c9ccb2ccae` |

O workspace executou de `2026-09-08T14:40:45.818205+00:00` a
`2026-09-08T14:44:30.026165+00:00`. O A/B executou de
`2026-09-08T14:42:25.757472+00:00` a
`2026-09-08T14:45:51.470755+00:00`; dados integrais em
`p1316-ab-candidate-runs.json`, SHA-256
`5f784d7248a14d47ca449bbea99287a9bdb202aabd07e8d582a60c420fd26bc4`.

O revisor reconstruiu todas as 1320 expectativas a partir dos 330 casos e
observações baseline/vanilla, conferiu os 290 replays históricos antes do
delta de origem e comparou novamente as 3960 observações candidatas em
normal/repeat/reverse. Nenhuma diferença. Todos os 29 pins do freeze,
28 artefatos P1315 e testes anteriores continuam intactos.

## Alcance do veredito

A única mudança produtiva P1316 atribui a `error.span` o `value_span` da
primeira ocorrência posicional de Args no ramo Bytes. O decoder puro,
Path/Str, opções, validações, ordinal P1315 e mensagens permanecem preservados.
Os testes locais distinguem named/chamada/argumento inteiro/segunda ocorrência,
origens synthetic/detached e ausência de I/O; A/B cobre as rotas públicas
With/Args/spread/sink/map e os controles congelados.

As quatro medições novas de parsing com positional excedente continuam
guardadas nos artefatos originais. Elas revelam dívida de precedência; a
origem nesse caso é efeito normativo local previsto no L0 e testado, sem
alegar paridade com o erro de excesso vanilla. Formatação UTF-8, sufixos de
posição e diagnóstico de arquivo continuam dívidas fora do recorte.

Fluxo contínuo ADR-0127 confirmado: L0 antes do candidato, freeze e RED
estável antes da implementação, GREEN e revalidação completos. Não houve
alteração de API pública, fase, flag ou dependência. Nenhum commit realizado
por este papel; nenhuma leitura às pastas materialization/context.
