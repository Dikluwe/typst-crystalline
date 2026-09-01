# P1285 — receipt de verificação independente final v3

**Estado:** `FINAL_VERIFICATION_V3_COMPLETE`  
**Veredito binário:** `Refined`  
**Regime:** protocolo completo de materialização segregada.  
**Papel:** terceiro e novo verificador independente; não participou da autoria da
intenção, contrato, oráculos, testes, implementação, mutações ou verificações v1/v2.  
**Janela total de auditoria:** `2026-08-30T17:29:09-03:00` a
`2026-08-30T17:33:08-03:00`.  
**Janela dos gates executáveis:** `2026-08-30T17:30:32-03:00` a
`2026-08-30T17:32:53-03:00`.  
**HEAD:** `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.  
**Estado do repositório:** `working tree não commitado`.

Este receipt atesta somente o fragmento P1285, as entradas identificadas e os
gates reproduzidos abaixo. Não alega equivalência funcional geral. O verificador
não corrigiu nenhum artefato julgado e escreveu somente este receipt e
`00_nucleo/diagnosticos/typst-passo-1285-relatorio-v3.md`.

## 1. Autoridade, capacidades e limites

Entradas lidas: `AGENTS.md`, a skill `tekt-materializacao-segregada`, suas duas
referências obrigatórias, exclusivamente o Passo 1285 autorizado, os artefatos
P1285 finais, os nove L0, os nove consumers e as verificações históricas v1/v2.
Escrita limitada aos dois diagnósticos v3 autorizados. Não foram listados nem
lidos outros ficheiros de `00_nucleo/materialization/` e nenhum ficheiro de
`00_nucleo/context/` foi lido. Nenhuma falha foi corrigida.

Linguagem de atestação: cadeia segregada por capacidades e artefatos, executada
sem atestação de isolamento ambiental forte. A árvore compartilhada já estava
massivamente suja antes deste papel.

## 2. Entradas finais verificadas antes e depois dos gates

| Artefato | SHA-256 exigido e observado |
|---|---|
| contrato v4+precision | `647176b3d0840a3c14a2ac837f401d43090fdb4ad253ce8e5f90ba13810899a2` |
| oracle receipt repinado | `998a9972305c2e710152cd10829996010f9b579a8f1a4b23d76e038c022502b9` |
| runner | `08c75c14b86c74858964f1a17efd0f2b499d0b9cfe3c80f5b980d44dda988c16` |
| baseline | `5cf69aed9117b9b9fbb7880df751b56a44871b538be84ddc8f33ecf40a1cfec7` |
| RED receipt final | `dd1ab3b91a9aeef9c95f02003e21a5f5d89816d8cfdb09370dcec8c4fa307455` |
| adversarial receipt v3 | `d7d16fad24b111e047a74e37d5997244d2e5cbcf6a2537625754f0cf9d566d0d` |
| mutation plan v7 | `968128b32e09b9cbf8685103c3960fc575fbe4822ac07f3cf630c21ac2d27d64` |

Entradas normativas auxiliares também conferiram:

| Entrada | SHA-256 observado |
|---|---|
| `AGENTS.md` | `bc50c0c6d54c0e301a5fe3c5c5869dbeef8fdf624096b0fa20185122c64d7da0` |
| skill | `33a32f7bc439de3fe3aa530bd65518e512a93f40152c91b2ace0789de34a3a56` |
| `papeis-e-capacidades.md` | `f59f44c4e53e89651963115c582872b4d3cd59d89689d103baa9ef8b464d2417` |
| `artefatos-e-gates.md` | `bf218259b4454974bf8889ce319e04c0c7ec668b9a542d0eb3b4d0492a623963` |
| Passo 1285 | `5a69234ba5340c92e7cfe523212b102869e8a7fcefafb4647e4acf1dc3690a6c` |

O binário `target/debug/typst` observado após `cargo build` tinha SHA-256
`08e0b7cd1967b00a26178c7daa240989f0c1e0e7c418ddc268c552b427a43509`.

## 3. Nove pins L0 e nove consumers finais

O pin normativo é o SHA-256 após remover somente a linha
`Hash do Código:`, com `sed '/^Hash do Código:/d'`. Todos os nove pins
coincidem com o contrato.

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

O full SHA atual de `eval/math.md` difere do full SHA congelado inicial, mas o
pin normativo é byte-idêntico. Pela política explícita da seção 2.2 do
contrato, isto é drift somente de selo e não drift semântico.

| Consumer | SHA-256 observado antes/depois |
|---|---|
| `01_core/src/compiler/eval/mod.rs` | `b4cee9b297457726cf42fad5737edb45f0b17904ec6ebdf7f6ddc13ef24fa9ea` |
| `01_core/src/compiler/eval/repr.rs` | `cbd8205d779b40407e3406f6b4e6aeaa486e5fc84ca669cb30ab4b4cd3a7007b` |
| `01_core/src/compiler/eval/selector_matching.rs` | `09014da2b4a4737db6e1e996a3d001cc9fbdcd356f84890f5020f4a324ec80f9` |
| `01_core/src/compiler/eval/rules.rs` | `d9bef037b7c4fb025193fec210f7015a5aceae2f73c1317d7db36bbaab01ae3f` |
| `01_core/src/compiler/eval/math.rs` | `2c7d920337ee46a3423db5a0809bab73e11685c8cf0f32e4984091143976ba1e` |
| `01_core/src/compiler/stdlib/foundations/selector.rs` | `b8c1460f1ddd97b70b20a00366d1a5e8c2cd76e37cefb295e1baf757b1cdd179` |
| `02_shell/src/cli.rs` | `87f7983ec6ebc261603af3d9147faa936c79bb20582c7c1359fe29dd9ff11e85` |
| `03_infra/src/query_helpers.rs` | `e3157124d94bc83259be375bb0798c0cee41a9f20516c0c45ef0e351182e0aa8` |
| `04_wiring/src/main.rs` | `073c4b8967ac20c5cc3d69ab0072283b6f06754ad8d54ebc207e475dde4371b6` |

As sete entradas protegidas e os nove consumers permaneceram estáveis durante
todos os gates.

## 4. Gate adversarial v3 auditado

A auditoria mecânica do receipt v3 contou exatamente:

```text
linhas de mutantes rerodados = 15
linhas rerodadas mortas      = 15
linhas transferidas          = 15
linhas transferidas mortas   = 15
IDs únicos reconciliados     = 30
```

A partição coincide com o plano v7:

- rerodados: `M-Y1..M-Y4`, `M-J1..M-J5`, `M-Q1..M-Q6`;
- transferidos por identidade byte a byte: `M-Y5`, `M-Q7`, `M-Q8`,
  `M-S1..M-S10`, `M-M1`, `M-M2`.

O receipt registra `15` reruns mortos e `15` resultados mortos transferidos,
total combinado `30/30`, `0` survivors, `0` `Unknown` e score `1.0`. Registra
também baseline idêntico antes/depois de `19/19` testes GREEN, restauração por
backup após cada mutante, invalidação dos artefatos incrementais sem mudança de
bytes e os nove hashes restaurados na cópia e na origem. Todos os nove hashes
declarados coincidem com os consumers observados nesta verificação. A fonte das
transferências está preservada no receipt v2 SHA-256
`249ecd54e57a09496ec8c306d12c1fc04130bef42240ca090b8ebcc70a814f5e`;
o plano v6 preservado tem SHA-256
`24a5c942b45dcccb4a03860f10e55e4358e6bbad81913d86abbc75565fa4f740`.

## 5. Gates reproduzidos independentemente

| # | Comando | Janela | Exit | Contagem/resultado |
|---:|---|---|---:|---|
| 1 | `python3 lab/surface-inventory/run_p1285_oracles.py --typst target/debug/typst --baseline lab/surface-inventory/p1285-oracle-baseline.json` | `17:30:32–17:30:49` | 0 | `Preserved=42`, `Violated=0`, `Unknown=0`; dois passes internos em ordem oposta |
| 2 | `cargo test -p typst-shell p1285 -- --nocapture` | `17:30:56–17:30:56` | 0 | 5 passed, 0 failed |
| 3 | `cargo test -p typst-core p1285 -- --nocapture` | `17:31:01–17:31:01` | 0 | 9 passed, 0 failed |
| 4 | `cargo test -p typst-infra p1285_query_elements -- --nocapture` | `17:31:06–17:31:06` | 0 | 2 passed, 0 failed; integração filtrada 0/0 |
| 5 | `cargo test -p typst-wiring --test cli p1285_query_stdin_transporta_yaml_ate_stdout -- --nocapture` | `17:31:12–17:31:12` | 0 | 1 passed, 0 failed |
| 6 | `cargo test -p typst-core compiler::math:: --lib` | `17:31:20–17:31:20` | 0 | 336 passed, 0 failed |
| 7 | `cargo test -p typst-core repr_value_complex_types -- --nocapture` | `17:31:26–17:31:26` | 0 | 1 passed, 0 failed |
| 8 | `cargo test -p typst-shell p1225_eval_json_stroke_e_nominal_e_raw_permanece_proibido -- --nocapture` | `17:31:31–17:31:31` | 0 | 1 passed, 0 failed |
| 9 | `cargo build` | `17:31:36–17:31:36` | 0 | build concluído; warnings não fatais |
| 10 | `cargo test` | `17:31:44–17:31:59` | 0 | workspace integral: 6366 passed, 0 failed; doctests 0 passed, 3 ignored |
| 11 | `cargo fmt --all -- --check` | `17:32:14–17:32:16` | 0 | árvore formatada |
| 12 | `git diff --check` | `17:32:21–17:32:21` | 0 | nenhum erro de whitespace |
| 13 | `crystalline-lint .` | `17:32:26–17:32:26` | 0 | zero violations de nível error |
| 14 | `crystalline-lint . 2>&1 \| awk ...` | `17:32:52–17:32:53` | 0 | `0` errors, `209` warnings, `1054` notes; `1263` resultados |

A decomposição observada da suíte integral foi:

```text
typst-core                 5318 passed, 0 failed
typst-infra                 911 passed, 0 failed
p1250_svg_integrated          1 passed, 0 failed
typst-shell                  61 passed, 0 failed
typst-wiring main             2 passed, 0 failed
typst-wiring cli             71 passed, 0 failed
crystalline_lint              2 passed, 0 failed
total executável           6366 passed, 0 failed
doctests                      0 passed, 0 failed, 3 ignored
```

O teste `p1137_watch_dependencias_recuperacao_e_filtro`, cuja falha por timeout
sob carga havia sido comunicada como flake histórico, passou dentro desta
primeira execução integral. Não houve falha, timeout nem repetição excepcional;
o protocolo condicional de duas repetições focais e uma integral não foi
acionado.

## 6. Histórico v1/v2 preservado e resolvido

Os quatro documentos históricos permaneceram byte-idênticos antes e depois da
verificação:

| Documento histórico | SHA-256 atual |
|---|---|
| `p1285-verification-receipt.md` | `684a01203bc571ebd77c6fff5a39362325e0af8701110a5430c80d850bcebef3` |
| `typst-passo-1285-relatorio.md` | `9f7dc0e68eec53dc8cf713e4f70b3d5f9ef057d8a5c84cc9c72b0d646342c956` |
| `p1285-verification-receipt-v2.md` | `23a896f3ce746e3019fac3e1a21038e1cec8c2deb3b063926770d8d848318781` |
| `typst-passo-1285-relatorio-v2.md` | `12bfb4af39346d7d69d747f4286b2aad2865cd453ddda2810248873e95183499` |

V1 emitiu `Not refined` por gradient, formatação e pin documental. V2 emitiu
`Not refined` por Color RGB nominal no teste legado P1225. Nesta rodada, todas
essas causas históricas foram revalidadas como resolvidas: repr complex `1/1`,
formatação verde, oracle receipt final repinado, P1225 exato `1/1` e suíte
integral verde. O histórico não foi apagado nem sobrescrito.

## 7. Proveniência exata da medição

Medição final pré-receipt: `2026-08-30T17:33:08-03:00`; HEAD
`53d21c5a602f4045a769a0ab0c935baa5ecd3b88`; `working tree não commitado`.

Saída exata de `git diff HEAD --stat` nesse estado:

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
 00_nucleo/prompts/infra/query-helpers.md            |     43 +
 00_nucleo/prompts/shell/cli.md                     |    140 +-
 00_nucleo/prompts/wiring.md                        |     25 +
 01_core/Cargo.toml                                 |      1 +
 01_core/src/compiler/eval/bindings/field_access.rs |     45 +-
 01_core/src/compiler/eval/bindings/mod.rs          |      2 +-
 .../src/compiler/eval/bindings/value_methods.rs    |     40 +-
 01_core/src/compiler/eval/call_dispatch.rs         |    555 +-
 01_core/src/compiler/eval/math.rs                  |     13 +-
 01_core/src/compiler/eval/mod.rs                   |     10 +-
 01_core/src/compiler/eval/repr.rs                  |     57 +-
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
 02_shell/src/cli.rs                                |    794 +-
 03_infra/fixtures/fonts/LICENSE-NotoSans.md        |     93 +
 03_infra/fixtures/fonts/NotoSans-Regular.ttf       |    Bin 0 -> 556216 bytes
 03_infra/fixtures/p307b/MANIFEST.md                |     36 +-
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
 94 files changed, 645608 insertions(+), 1390 deletions(-)
```

Os dois diagnósticos v3 são untracked e, por definição, não aparecem em
`git diff HEAD --stat`; sua criação é a única mutação deste papel.

## 8. Veredito

`Refined`.

Razão binária: todos os gates finais obrigatórios passaram; não houve survivor,
falha, lacuna ou `Unknown`. O oráculo preservou `42/42`, o gate adversarial
reconciliou `30/30` com score `1.0`, a matriz focal e as regressões passaram, a
suíte integral terminou verde na primeira execução, e build, formatação,
whitespace e arquitetura passaram. O veredito fica limitado ao fragmento e aos
hashes registrados neste receipt.
