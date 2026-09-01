# P1285 — receipt de verificação independente final

**Estado:** `FINAL_VERIFICATION_COMPLETE`  
**Veredito binário:** `Not refined`  
**Regime:** protocolo completo de materialização segregada.  
**Papel:** verificador independente; não participou da autoria da intenção, contrato,
oráculos, testes, implementação ou mutações.  
**Janela de verificação:** `2026-08-30T16:33:59-03:00` a
`2026-08-30T16:38:40-03:00`.  
**HEAD:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.  
**Estado:** `working tree não commitado`.

Este receipt atesta somente o fragmento P1285, os artefatos identificados e os gates
abaixo. Não alega equivalência funcional geral. A única escrita deste papel foi este
receipt e `00_nucleo/diagnosticos/typst-passo-1285-relatorio.md`; nenhum artefato
verificado foi editado.

## 1. Autoridade e capacidades

Entradas lidas: `AGENTS.md`, a skill `tekt-materializacao-segregada`, suas duas
referências obrigatórias, exclusivamente o Passo 1285 autorizado, os artefatos P1285
selados, os nove L0 e os nove consumers. Escrita limitada aos dois diagnósticos finais
autorizados. Não foram lidos outros ficheiros de `materialization/` nem quaisquer
ficheiros de `context/`. Nenhuma falha foi corrigida.

Linguagem de atestação: cadeia segregada por capacidades e artefatos, sem alegação de
isolamento ambiental forte. A árvore compartilhada estava massivamente suja antes
deste papel.

## 2. Entradas externas verificadas antes e depois dos gates

| Artefato | SHA-256 exigido e observado |
|---|---|
| contrato | `647176b3d0840a3c14a2ac837f401d43090fdb4ad253ce8e5f90ba13810899a2` |
| oracle receipt | `1f4e98c40ef44355e04134821d21c4d523dffad9ab6a6c11eae39067f2d6d32b` |
| runner | `08c75c14b86c74858964f1a17efd0f2b499d0b9cfe3c80f5b980d44dda988c16` |
| baseline | `5cf69aed9117b9b9fbb7880df751b56a44871b538be84ddc8f33ecf40a1cfec7` |
| RED receipt | `3f093f748769ccbb128183ca73cfab9052f2a7d6b097503ab5e5ccff2532f25d` |
| adversarial receipt | `d2a278e1bc77e677715c1b033e7ae27d0c862c19a9ce68b413371b82b25e8a3b` |
| mutation plan v5 | `9cf5b7304c9761c245ea8be544bb7dbdd8bf6c833822e3f268dbe0c0f23a743a` |

Entradas normativas auxiliares também conferiram: `AGENTS.md`
`bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0`,
skill `33a32f7bc439de3fe3aa530bd65518e512a93f40152c91b2ace0789de34a3a56`,
`papeis-e-capacidades.md`
`f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417`,
`artefatos-e-gates.md`
`bf218259b4454974bf8889ce319e04c0c7ec668b9a542d0eb3b4d0492a623963`
e Passo 1285
`5a69234ba5340c92e7cfe523212b102869e8a7fcefafb4647e4acf1dc3690a6c`.

## 3. Nove L0 e consumers congelados

O pin normativo é o SHA-256 após remover somente a linha `Hash do Código:`. Todos os
nove pins conferiram antes e depois dos gates.

| L0 | Pin normativo observado | Full SHA observado |
|---|---|---|
| `prompts/shell/cli.md` | `8fe484152e1371d7ef4a79659d6388bf5e63ff63ff84dc5698db7147a33543a7` | `fa2d3c4471822550994dfe2e4d23e503925efd9a067aac2e0054ad2c77ae004c` |
| `prompts/compiler/eval.md` | `7b2dc94f28457d84022f59a80b858c12428f671e67e3a5e90e6c0865d773e6c3` | `1fb66ec889304cd3a21accd84b009388cd1613c6fc613937c3af60015483cf56` |
| `prompts/compiler/eval/repr.md` | `3f1bcd114c5f880e10fedd3f5fbaee868d456b9c31b244244cb7a8a15d23860b` | `a2b35c8b3e622432b5a129a4cad0bb20ab94b450d3e793fd1441c5af6bc2775c` |
| `prompts/compiler/eval/selector_matching.md` | `557abd0a6fb52f8b198606e928ba23fd65234e3b75c2a14703b07a9975ba1b11` | `031ba52f7a6d5870479f0fe699a572950cce594db7688d59fc484175c10505a3` |
| `prompts/compiler/eval/rules.md` | `3655f2922835fd0bfcf0a1857a6d12d70ac9c30eae23f9f7b9eea1f69d9fc7c0` | `740919d6a874f136a6f176451977a51352266c08d347329f91207319a38018aa` |
| `prompts/compiler/stdlib/foundations/selector.md` | `9bea0284d242754ca1103baf45c53b7614817cfff1e5358fd708980f971f7c22` | `1a603e05d436e5f2359f77e3e71c0e233478ecf13209f71801fb45ae40214799` |
| `prompts/infra/query-helpers.md` | `352604527883503c72e349f9171cdb92cf730af3772cde28820275f11005713d` | `42debc22810cf3eebf16185d91543cedc36be4e179221f5b6d15b8f2ff6c6705` |
| `prompts/wiring.md` | `2142e48948482f1f86a8390b4dbd40cdc84f67f1783d5c9f3f370e9ed1401a44` | `f8db4e993fd1931d8f77b69a2ec416377f0700569e3942cc78e32daf945e0ec8` |
| `prompts/compiler/eval/math.md` | `c2ba2c5f028cd7424d15f92486e1d281a6efca1b74696f75249c6b9f1a018a9c` | `58b9f497fe8c1dbb1c464c7b402a01ce21a7adaf056e9d88814f14ae8f59e5d6` |

O full SHA atual de `eval/math.md` difere do full SHA pré-candidata
`7cdf5a1c0f93d1f58cda7f93eaae09eb0cbdcead3ca78864919569755c3321b2`,
mas o pin normativo permanece idêntico. Isto é drift exclusivamente de selo permitido
pela seção 2.2 do contrato, não drift semântico.

| Consumer | SHA-256 congelado/restaurado e observado |
|---|---|
| `02_shell/src/cli.rs` | `e00dbc1f445f6c3ab1976740daeebb4c4e2a6668fb409d8c25d14ae0e206857b` |
| `01_core/src/compiler/eval/mod.rs` | `b4cee9b297457726cf42fad5737edb45f0b17904ec6ebdf7f6ddc13ef24fa9ea` |
| `01_core/src/compiler/eval/repr.rs` | `aa47dd1f62f8ccfb25e54166b32c9c64b796f743968c28708d87d42ee2ff853b` |
| `01_core/src/compiler/eval/selector_matching.rs` | `09014da2b4a4737db6e1e996a3d001cc9fbdcd356f84890f5020f4a324ec80f9` |
| `01_core/src/compiler/eval/rules.rs` | `d9bef037b7c4fb025193fec210f7015a5aceae2f73c1317d7db36bbaab01ae3f` |
| `01_core/src/compiler/stdlib/foundations/selector.rs` | `b8c1460f1ddd97b70b20a00366d1a5e8c2cd76e37cefb295e1baf757b1cdd179` |
| `03_infra/src/query_helpers.rs` | `e3157124d94bc83259be375bb0798c0cee41a9f20516c0c45ef0e351182e0aa8` |
| `04_wiring/src/main.rs` | `073c4b8967ac20c5cc3d69ab0072283b6f06754ad8d54ebc207e475dde4371b6` |
| `01_core/src/compiler/eval/math.rs` | `2c7d920337ee46a3423db5a0809bab73e11685c8cf0f32e4984091143976ba1e` |

## 4. Gates reproduzidos

| # | Comando | Janela | Exit | Resultado |
|---:|---|---|---:|---|
| 1 | `python3 lab/surface-inventory/run_p1285_oracles.py --typst target/debug/typst --baseline lab/surface-inventory/p1285-oracle-baseline.json` | `16:35:16–16:35:33` | 0 | `Preserved=42`, `Violated=0`, `Unknown=0`; binário `1dda9c9b11453401b05738659b4c5de45db0a9923aac2d8d628895042e601088` |
| 2 | `cargo test -p typst-shell p1285 -- --nocapture` | `16:35:40–16:35:40` | 0 | 5 passed, 0 failed |
| 3 | `cargo test -p typst-core p1285 -- --nocapture` | `16:35:45–16:35:45` | 0 | 9 passed, 0 failed |
| 4 | `cargo test -p typst-infra p1285_query_elements -- --nocapture` | `16:35:50–16:35:50` | 0 | 2 passed, 0 failed |
| 5 | `cargo test -p typst-wiring --test cli p1285_query_stdin_transporta_yaml_ate_stdout -- --nocapture` | `16:35:55–16:35:55` | 0 | 1 passed, 0 failed |
| 6 | `cargo test -p typst-core compiler::math:: --lib` | `16:36:00–16:36:00` | 0 | 336 passed, 0 failed |
| 7 | `cargo build` | `16:36:06–16:36:06` | 0 | build concluído; warnings não fatais |
| 8 | `cargo test` | `16:36:12–16:36:47` | **101** | **FAILED**: 5317 passed, 1 failed no crate `typst-core`; a execução integral parou na primeira unidade de teste falhada |
| 9 | `cargo fmt --check` | `16:37:03–16:37:05` | **1** | **FAILED**: diff de rustfmt em `02_shell/src/cli.rs:924` |
| 10 | `git diff --check` | `16:37:10–16:37:10` | 0 | sem erro de whitespace |
| 11 | `crystalline-lint .` | `16:37:14–16:37:15` | 0 | 0 violações de nível error; SARIF: 1263 resultados, 209 warnings, 1054 notes |

### 4.1 Falha da suíte integral

Teste: `compiler::eval::repr::tests::repr_value_complex_types`, em
`01_core/src/compiler/eval/repr.rs:1064`.

```text
assertion `left == right` failed
  left: "gradient.linear()"
 right: "gradient(...)"
```

O mesmo módulo implementa `Gradient::Linear` por `repr_gradient` como
`gradient.linear({stops})`; logo o teste integral congelado está em conflito com o
comportamento atual. O verificador não classifica qual lado deve ser corrigido e não
edita nenhum deles: a existência da falha basta para o gate binário.

### 4.2 Falha de formatação

`cargo fmt --check` exigiu reformatar o braço `Value::Length` em
`02_shell/src/cli.rs:924`. Nenhuma alteração foi aplicada.

## 5. Gate adversarial auditado

A tabela do adversarial receipt contém exatamente 30 linhas de mutantes e declara, com
`restore ✔` em cada linha:

```text
obrigatórios  = 30
mortos        = 30
sobreviventes = 0
unknown       = 0
score         = 30 / 30 = 1.0
```

Os nove hashes restaurados declarados no receipt coincidem com os nove consumers
observados antes e depois desta verificação. Assim, o gate adversarial registrado é
`30/30`, score `1.0`, sem survivor ou `Unknown`. Isto não supera as falhas independentes
dos gates finais.

## 6. Lacuna na cadeia de evidência

O oracle receipt atual tem SHA-256
`1f4e98c40ef44355e04134821d21c4d523dffad9ab6a6c11eae39067f2d6d32b` e descreve os
nove L0 finais/restart v4, mas sua tabela `2. Entradas congeladas` ainda pina o contrato
antigo `9897a6f893734b66b2d1c93ee653bc1155ab0c51f705c89f22011ee73a3c77ec`, não o
contrato final v4+precision
`647176b3d0840a3c14a2ac837f401d43090fdb4ad253ce8e5f90ba13810899a2`.
O RED receipt e o adversarial receipt pinam o contrato final corretamente. Pela skill,
um receipt precisa referenciar o manifesto/contrato efetivamente congelado; esta
inconsistência documental é uma lacuna adicional e não é convertida em sucesso.

## 7. Proveniência exata da medição

Medição final: `2026-08-30T16:38:40-03:00`; HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`; `working tree não commitado`.

Saída exata de `git diff HEAD --stat`:

```text
 .gitignore                                         |     12 +-
 .typ/sec_04.typ                                    |     24 +
 .typ/sec_07.typ                                    |     21 +
 .typ/sec_09.typ                                    |     23 +
 .typ/sec_10.typ                                    |     25 +
 .typ/sec_12.typ                                    |     20 +
 .typ/sec_16.typ                                    |     22 +
 .typ/sec_17.typ                                    |     18 +
 .typ/sec_18.typ                                    |     18 +
 .typ/sec_22.typ                                    |     21 +
 .typ/sec_27.typ                                    |     17 +
 .../diagnosticos/p1282-crystalline-default.json    |  21432 ++
 00_nucleo/diagnosticos/p1282-crystalline-html.json |  22266 ++
 .../diagnosticos/p1282-inventory-default.json      | 128832 +++++++++++
 00_nucleo/diagnosticos/p1282-inventory-html.json   | 213769 ++++++++++++++++++
 00_nucleo/diagnosticos/p1282-probes-default.json   |   4012 +
 00_nucleo/diagnosticos/p1282-probes-html.json      |   4802 +
 00_nucleo/diagnosticos/p1282-summary.json          |  47835 ++++
 00_nucleo/diagnosticos/p1282-vanilla-default.json  |  57158 +++++
 00_nucleo/diagnosticos/p1282-vanilla-html.json     | 138527 ++++++++++++
 .../diagnosticos/typst-passo-1281-relatorio.md     |    164 +
 .../diagnosticos/typst-passo-1282-relatorio.md     |    259 +
 00_nucleo/prompts/compiler/eval.md                 |     21 +-
 .../prompts/compiler/eval/bindings/field_access.md |     65 +-
 .../compiler/eval/bindings/value_methods.md        |     30 +-
 00_nucleo/prompts/compiler/eval/call_dispatch.md   |     81 +-
 00_nucleo/prompts/compiler/eval/math.md            |     28 +-
 00_nucleo/prompts/compiler/eval/repr.md            |     13 +-
 00_nucleo/prompts/compiler/eval/rules.md           |     26 +-
 .../prompts/compiler/eval/selector_matching.md     |     57 +-
 00_nucleo/prompts/compiler/stdlib/collections.md   |     86 +-
 00_nucleo/prompts/compiler/stdlib/color.md         |     76 +-
 00_nucleo/prompts/compiler/stdlib/emoji.md         |     53 +-
 .../prompts/compiler/stdlib/foundations/color.md   |    109 +-
 .../compiler/stdlib/foundations/selector.md        |     25 +-
 .../prompts/compiler/stdlib/foundations/str.md     |     23 +-
 .../prompts/compiler/stdlib/structural/math.md     |     17 +
 .../prompts/compiler/stdlib/structural/outline.md  |     22 +
 00_nucleo/prompts/compiler/stdlib/sym.md           |     55 +-
 00_nucleo/prompts/entities/color.md                |     13 +
 00_nucleo/prompts/entities/selector.md             |     22 +
 00_nucleo/prompts/infra/export-fixtures.md         |     12 +-
 00_nucleo/prompts/infra/query-helpers.md           |     43 +
 00_nucleo/prompts/shell/cli.md                     |    140 +-
 00_nucleo/prompts/wiring.md                        |     25 +
 01_core/Cargo.toml                                 |      1 +
 01_core/src/compiler/eval/bindings/field_access.rs |     45 +-
 01_core/src/compiler/eval/bindings/mod.rs          |      2 +-
 .../src/compiler/eval/bindings/value_methods.rs    |     40 +-
 01_core/src/compiler/eval/call_dispatch.rs         |    555 +-
 01_core/src/compiler/eval/math.rs                  |     13 +-
 01_core/src/compiler/eval/mod.rs                   |     10 +-
 01_core/src/compiler/eval/repr.rs                  |     55 +-
 01_core/src/compiler/eval/rules.rs                 |     79 +-
 01_core/src/compiler/eval/selector_matching.rs     |    104 +-
 01_core/src/compiler/eval/tests.rs                 |     91 +-
 01_core/src/compiler/stdlib/collections.rs         |   1355 +-
 01_core/src/compiler/stdlib/color.rs               |     33 +-
 01_core/src/compiler/stdlib/emoji.rs               |    680 +-
 01_core/src/compiler/stdlib/foundations/color.rs   |    184 +-
 .../src/compiler/stdlib/foundations/selector.rs    |     84 +-
 01_core/src/compiler/stdlib/foundations/str.rs     |      6 +-
 01_core/src/compiler/stdlib/mod.rs                 |     40 +-
 01_core/src/compiler/stdlib/state.rs               |      2 +-
 01_core/src/compiler/stdlib/structural/math.rs     |     56 +-
 01_core/src/compiler/stdlib/structural/outline.rs  |      2 +-
 01_core/src/compiler/stdlib/sym.rs                 |    189 +-
 01_core/src/entities/color.rs                      |      2 +-
 01_core/src/entities/selector.rs                   |      2 +-
 02_shell/src/cli.rs                                |    780 +-
 03_infra/fixtures/fonts/LICENSE-NotoSans.md        |     93 +
 03_infra/fixtures/fonts/NotoSans-Regular.ttf       |    Bin 0 -> 556216 bytes
 03_infra/fixtures/p307b/MANIFEST.md                 |     36 +-
 03_infra/src/export/svg.rs                         |      6 +-
 03_infra/src/p307b_snapshot_tests.rs               |      6 +-
 03_infra/src/query_helpers.rs                      |     90 +-
 04_wiring/src/main.rs                              |     66 +-
 04_wiring/tests/cli.rs                             |     33 +
 Cargo.lock                                         |     67 +
 Cargo.toml                                         |      1 +
 crystalline.toml                                   |      5 +
 lab/parity/matrix/manifest.yaml                    |      6 +-
 lab/parity/matrix/runner.py                        |     92 +-
 lab/parity/matrix/test_runner.py                   |    120 +
 lab/surface-inventory/Cargo.lock                   |     83 +-
 lab/surface-inventory/extra_seeds.json             |     47 +
 lab/surface-inventory/merge.py                     |    407 +-
 lab/surface-inventory/probes.json                  |      4 +-
 lab/surface-inventory/run_probes.py                |    193 +-
 lab/surface-inventory/src/main.rs                  |    235 +-
 lab/surface-inventory/summarize.py                 |    201 +
 lab/surface-inventory/test_merge.py                |    234 +
 lab/surface-inventory/test_run_probes.py           |     36 +
 .../crates/p1140-inventory/src/main.rs             |    192 +-
 94 files changed, 645599 insertions(+), 1383 deletions(-)
```

## 8. Veredito

`Not refined`.

Razões suficientes e independentes:

1. `cargo test` integral falhou com 1 teste;
2. `cargo fmt --check` falhou;
3. o oracle receipt não pina o contrato final na sua tabela de entradas.

Nenhuma dessas condições é `Unknown` convertido em sucesso. Conforme o protocolo,
30/30 mutantes mortos, os focais verdes e 42/42 oráculos não autorizam `Refined` quando
qualquer gate final ou elo protegido falha.
