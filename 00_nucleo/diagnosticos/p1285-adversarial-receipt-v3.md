# P1285 — receipt adversarial v3 de refinamento final

Estado: campanha de refinamento v7 executada; este documento **não emite
veredito final**.

## 1. Regime, entradas e ambiente

- Regime: protocolo completo de materialização segregada; papel adversarial.
- Linguagem de atestação: segregado por entradas/capacidades e executado sem
  atestação de isolamento ambiental forte.
- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Cópia explícita: `/tmp/p1285-adversarial-v7.0Av7Ub/repo`.
- Backups byte a byte: `/tmp/p1285-adversarial-v7.0Av7Ub/backups`.
- Janela da campanha: `2026-08-30T17:19:48-03:00` a
  `2026-08-30T17:25:53-03:00`.
- Cada mutação ocorreu somente na cópia; cada rerun terminou por
  `cp --preserve=all` do backup e `sha256sum` de `cli.rs`.
- Não foram lidos nem executados oráculo, baseline, runner, receipt do oráculo,
  `00_nucleo/materialization/` ou `00_nucleo/context/`.
- Produção, testes, L0, contrato, RED, plano v6 e receipt v2 não foram editados.

| Entrada | SHA-256 |
|---|---|
| contrato | `647176b3d0840a3c14a2ac837f401d43090fdb4ad253ce8e5f90ba13810899a2` |
| RED receipt final | `dd1ab3b91a9aeef9c95f02003e21a5f5d89816d8cfdb09370dcec8c4fa307455` |
| plano v6 preservado | `24a5c942b45dcccb4a03860f10e55e4358e6bbad81913d86abbc75565fa4f740` |
| receipt v2 preservado | `249ecd54e57a09496ec8c306d12c1fc04130bef42240ca090b8ebcc70a814f5e` |
| plano v7 | `968128b32e09b9cbf8685103c3960fc575fbe4822ac07f3cf630c21ac2d27d64` |

## 2. Delta v6→v7 e partição executável

A comparação direta do backup v6 contra a candidata v7 encontrou somente
`02_shell/src/cli.rs` divergente. O delta integral é confinado ao teste histórico
`p1225_eval_json_stroke_e_nominal_e_raw_permanece_proibido`: a expectativa de
`Color::rgb(0,0,0)` em JSON passou de erro para os bytes da string de linguagem
`rgb("#000000")`, mais newline. Produção e owners P1285 dentro do ficheiro não
mudaram; os outros oito consumers completos permanecem byte-idênticos.

Por política conservadora de alvo integral, foram rerodados todos os 15 mutantes
que escrevem em `cli.rs`:

```text
M-Y1 M-Y2 M-Y3 M-Y4
M-J1 M-J2 M-J3 M-J4 M-J5
M-Q1 M-Q2 M-Q3 M-Q4 M-Q5 M-Q6
```

Os 15 restantes foram transferidos do receipt v2 apenas porque o consumer que
recebe a transformação é byte-idêntico:

```text
M-Y5 M-Q7 M-Q8
M-S1 M-S2 M-S3 M-S4 M-S5 M-S6 M-S7 M-S8 M-S9 M-S10
M-M1 M-M2
```

Partição reconciliada: `15 reruns + 15 transferências = 30`.

## 3. Baseline final antes e depois

Os mesmos comandos passaram antes da primeira mutação (`17:19:56-03:00`) e
depois da última restauração (`17:25:42-03:00`). Total em cada lado: 19 testes,
19 GREEN.

```text
cargo test -p typst-core repr_value_complex_types -- --nocapture                     # 1/1
cargo test -p typst-core p1285 -- --nocapture                                        # 9/9
cargo test -p typst-shell p1285_ -- --nocapture                                      # 5/5
cargo test -p typst-shell p1225_eval_json_stroke_e_nominal_e_raw_permanece_proibido -- --nocapture # 1/1
cargo test -p typst-infra p1285_query_elements_ -- --nocapture                       # 2/2
cargo test -p typst-wiring --test cli p1285_query_stdin_transporta_yaml_ate_stdout -- --nocapture # 1/1
```

Antes do baseline pós-restauração, os nove consumers foram tocados sem alteração
de bytes para invalidar artefactos incrementais do Cargo; os nove SHA
continuaram iguais aos backups.

## 4. Quinze mutantes rerodados

| # | Mutante / owner | SHA-256 mutante | Comando ou witness | Resultado | Janela / restore `cli.rs=87f798…` |
|---:|---|---|---|---|---|
| 1 | `M-Y1` / `A-Y-HELP-EVAL` | `a14ade81b35dd749f537ea8fc3dd8190d85248f29fbaf6c23226bd07db8926c4` | build L4; `target/debug/typst eval --help` | **Morto**: help listou somente `json, raw` | `17:20:14–17:20:19`; ✔ |
| 2 | `M-Y2` / `A-Y-TREE` | `540b67ca9b8747f1e1686a5d403c70ff1658c7323fa90e295cbade5747cc0092` | build L4; eval composto com `--format yaml` | **Morto**: stdout JSON sob YAML | `17:20:31–17:20:33`; ✔ |
| 3 | `M-Y3` / `A-Y-PRETTY` | `b770532a31aea3e20ac686bba89113d5f771a69ec234ef9ea4c7f3d7d62d9de5` | eval YAML sem/com `--pretty` | **Morto**: segundo output virou JSON pretty | `17:20:49–17:20:51`; ✔ |
| 4 | `M-Y4` / `A-Y-QUERY-RAW` | `6fc7a3a9e7e342aef8d235062df28eedb12596399aad0b3a05261c326797949f` | build L4; stdin metadata com query `--format raw` | **Morto**: exit 0/YAML; esperado parser exit 2 | `17:21:20–17:21:23`; ✔ |
| 5 | `M-J1` / `A-J-VERSION-ACCEPT` | `6561817809b95becbba85b062236755ce2729af4a6dcabe29b0ea6a42249f672` | focal L2 version/fallbacks | **Morto**: `cannot serialize version` | `17:21:37–17:21:39`; ✔ |
| 6 | `M-J2` / `A-J-VERSION-EXACT` | `9205d469dabeb590294c38e58a2518d5bdabd7df470b967c058910808b00cf8e` | focal L2 version/fallbacks | **Morto**: `"0.15.1"` difere da repr da linguagem | `17:21:52–17:21:54`; ✔ |
| 7 | `M-J3` / `A-J-STRUCTURAL` | `e6ea1b5d487e3f3407a52ec8190ca47d4f2f34dd1d0deb08926b9d7cf86f2a0e` | focal L2 content estrutural | **Morto**: content virou string `sequence(...)` | `17:22:06–17:22:07`; ✔ |
| 8 | `M-J4` / `A-J-NOMINAL-REPR` | `f8eaae37c4e2b614ec3d18e1cc200f39bfa8ef2653da88090ffd52d143a6cebe` | focal L2 version/fallbacks | **Morto**: Version em Debug Rust | `17:22:17–17:22:18`; ✔ |
| 9 | `M-J5` / `A-J-RAW-REJECT` | `a0c25ecc8ef99b1ef3b7c0da9b87f7ae27a0decab0805d86138261901fad45f3` | build L4; `typst eval 'sys.version' --format raw` | **Morto**: exit 0/`version(0, 15, 1)` | `17:22:28–17:22:31`; ✔ |
| 10 | `M-Q1` / `A-Q-NONHEADING` | `f9e6f0b09147e5ca2959a125446dbe75132d5193038a257ae6f5d0a5acef0f14` | focal L2 metadata/figure/label | **Morto**: metadata tornou-se `null` | `17:22:43–17:22:44`; ✔ |
| 11 | `M-Q2` / `A-Q-SHAPE` | `07139ae5e03461b5a9ef2b2a783028ee6400060f5fcd7c1d16dc0bd6d6bec5d7` | focal L2 metadata/figure/label | **Morto**: campo `func` ausente | `17:22:55–17:22:57`; ✔ |
| 12 | `M-Q3` / `A-Q-ORDER` | `b6c261c2ad6cfa1929e634fa77daf4465e778e9ecc539895c56581f5c94e3e40` | focal L2, par figure/equation | **Morto**: ordem invertida | `17:23:11–17:23:12`; ✔ |
| 13 | `M-Q4` / `A-Q-FIELD-SPLIT` | `6866f39904c2fc188903e79d40e8f534212998310cc0d681b187667c1d18c58f` | focal L2 field/one | **Morto**: lista errou em field ausente | `17:23:28–17:23:29`; ✔ |
| 14 | `M-Q5` / `A-Q-ONE-FIRST` | `9c45cd0f22999fe51509e5e6dc592a14b6ccc42c7b23da78b29d5c60bb85c669` | focal L2 field/one | **Morto**: `found 0` substituiu erro de field | `17:23:45–17:23:46`; ✔ |
| 15 | `M-Q6` / `F-L2-LABEL-SERIALIZE` | `63d75e31308996364b2946034c7736d504dad23961f6fcbf995b17b2c084eb91` | focal L2 metadata/figure/label | **Morto**: label tornou-se `null` | `17:23:58–17:24:00`; ✔ |

Resultado dos reruns: 15 mortos, 0 sobreviventes, 0 `Unknown`.

## 5. Quinze resultados transferidos por identidade byte a byte

Fonte da evidência: receipt v2 SHA
`249ecd54e57a09496ec8c306d12c1fc04130bef42240ca090b8ebcc70a814f5e`.
Cada linha preserva integralmente o consumer que recebeu a transformação v6.

| Mutante | Consumer alvo | SHA v6 = SHA v7 | Resultado transferido |
|---|---|---|---|
| `M-Y5` | `04_wiring/src/main.rs` | `073c4b8967ac20c5cc3d69ab0072283b6f06754ad8d54ebc207e475dde4371b6` | **Morto** |
| `M-Q7` | `04_wiring/src/main.rs` | `073c4b8967ac20c5cc3d69ab0072283b6f06754ad8d54ebc207e475dde4371b6` | **Morto** |
| `M-Q8` | `04_wiring/src/main.rs` | `073c4b8967ac20c5cc3d69ab0072283b6f06754ad8d54ebc207e475dde4371b6` | **Morto** |
| `M-S1` | `01_core/src/compiler/eval/selector_matching.rs` | `09014da2b4a4737db6e1e996a3d001cc9fbdcd356f84890f5020f4a324ec80f9` | **Morto** |
| `M-S2` | `01_core/src/compiler/eval/selector_matching.rs` | `09014da2b4a4737db6e1e996a3d001cc9fbdcd356f84890f5020f4a324ec80f9` | **Morto** |
| `M-S3` | `01_core/src/compiler/eval/selector_matching.rs` | `09014da2b4a4737db6e1e996a3d001cc9fbdcd356f84890f5020f4a324ec80f9` | **Morto** |
| `M-S4` | `01_core/src/compiler/stdlib/foundations/selector.rs` | `b8c1460f1ddd97b70b20a00366d1a5e8c2cd76e37cefb295e1baf757b1cdd179` | **Morto** |
| `M-S5` | `03_infra/src/query_helpers.rs` | `e3157124d94bc83259be375bb0798c0cee41a9f20516c0c45ef0e351182e0aa8` | **Morto** |
| `M-S6` | `01_core/src/compiler/eval/selector_matching.rs` | `09014da2b4a4737db6e1e996a3d001cc9fbdcd356f84890f5020f4a324ec80f9` | **Morto** |
| `M-S7` | `01_core/src/compiler/stdlib/foundations/selector.rs` | `b8c1460f1ddd97b70b20a00366d1a5e8c2cd76e37cefb295e1baf757b1cdd179` | **Morto** |
| `M-S8` | `01_core/src/compiler/stdlib/foundations/selector.rs` | `b8c1460f1ddd97b70b20a00366d1a5e8c2cd76e37cefb295e1baf757b1cdd179` | **Morto** |
| `M-S9` | `01_core/src/compiler/stdlib/foundations/selector.rs` | `b8c1460f1ddd97b70b20a00366d1a5e8c2cd76e37cefb295e1baf757b1cdd179` | **Morto** |
| `M-S10` | `01_core/src/compiler/eval/selector_matching.rs` | `09014da2b4a4737db6e1e996a3d001cc9fbdcd356f84890f5020f4a324ec80f9` | **Morto** |
| `M-M1` | `01_core/src/compiler/eval/math.rs` | `2c7d920337ee46a3423db5a0809bab73e11685c8cf0f32e4984091143976ba1e` | **Morto** |
| `M-M2` | `01_core/src/compiler/eval/math.rs` | `2c7d920337ee46a3423db5a0809bab73e11685c8cf0f32e4984091143976ba1e` | **Morto** |

## 6. Reconciliação, Unknown e score

```text
rerodados mortos       = 15
transferidos mortos    = 15
total reconciliado     = 30
sobreviventes          = 0
unknown                = 0
score combinado        = 30 / (30 - 0) = 1.0
```

`Unknown` não mata. Nenhum timeout, erro ambiental, mutante opaco ou dúvida de
target foi contado como morte. O score foi medido; não constitui veredito.

## 7. Hashes finais e proveniência

Às `2026-08-30T17:25:53-03:00`, origem e cópia pós-restauração/baseline
apresentaram os mesmos hashes:

| Consumer | SHA-256 |
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

`git diff HEAD --stat` na origem: `94 files changed, 645608 insertions(+),
1390 deletions(-)`. A árvore compartilhada já estava massivamente suja; este
papel acrescentou somente o plano v7 e este receipt v3.

Este receipt fecha a medição adversarial v7 sem emitir veredito final.
