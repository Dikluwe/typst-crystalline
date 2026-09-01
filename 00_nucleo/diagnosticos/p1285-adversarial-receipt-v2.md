# P1285 — receipt adversarial v2 de refinamento

Estado: campanha de refinamento executada; este documento **não emite veredito final**.

## 1. Regime, entradas e ambiente

- Regime: protocolo completo de materialização segregada; papel adversarial.
- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Janela dos reruns: `2026-08-30T16:50:51-03:00` a
  `2026-08-30T16:54:41-03:00`.
- Fecho probatório: `2026-08-30T16:56:27-03:00`.
- Receipt escrito: `2026-08-30T16:58:13-03:00`.
- Cópia explícita: `/tmp/p1285-adversarial-v6.c131KJ/repo`.
- Backups byte a byte: `/tmp/p1285-adversarial-v6.c131KJ/backups`.
- Mutação somente na cópia; restauração após cada rerun por
  `cp --preserve=all`, seguida de `sha256sum`.
- Não foram lidos nem executados oráculo, baseline, runner, receipt do oráculo,
  `00_nucleo/materialization/` ou `00_nucleo/context/`.
- Linguagem de atestação: segregado por entradas/capacidades e executado sem
  atestação de isolamento ambiental forte.

| Entrada | SHA-256 final |
|---|---|
| contrato | `647176b3d0840a3c14a2ac837f401d43090fdb4ad253ce8e5f90ba13810899a2` |
| RED receipt refinado | `5f6d145a6489d23df306b9d9f216ed2668acb63b5bf8e4b04ccde0f7ea6e9391` |
| plano v5 preservado | `9cf5b7304c9761c245ea8be544bb7dbdd8bf6c833822e3f268dbe0c0f23a743a` |
| receipt v5 preservado | `d2a278e1bc77e677715c1b033e7ae27d0c862c19a9ce68b413371b82b25e8a3b` |
| plano v6 | `24a5c942b45dcccb4a03860f10e55e4358e6bbad81913d86abbc75565fa4f740` |

## 2. Delta v5→v6 e decisão de rerun

A comparação direta dos backups v5 contra a candidata v6 encontrou somente:

```diff
--- cli.rs v5
+++ cli.rs v6
- Value::Length(length) => value_to_semantic(&Value::Relative(Rel {
-     rel: 0.0,
-     abs: *length,
- })),
+ Value::Length(length) => {
+     value_to_semantic(&Value::Relative(Rel { rel: 0.0, abs: *length }))
+ }

--- repr.rs v5
+++ repr.rs v6
- None => "tiling(..)".to_string(),
+ None => "tiling(...)".to_string(),
- "gradient(...)"
+ "gradient.linear()"
```

`cli.rs` mudou apenas por rustfmt. Em `repr.rs`, a primeira diferença é
produtiva e a segunda está em `#[cfg(test)]`. Nenhuma transformação do plano
v5 foi aplicada diretamente a `repr.rs`; os cinco `M-J*`, porém, atravessam a
fachada de repr e foram rerodados conservadoramente. Assim:

- 15 reruns: `M-Y1..M-Y4`, `M-J1..M-J5`, `M-Q1..M-Q6`;
- 15 transferências por consumer alvo byte-idêntico: `M-Y5`, `M-Q7`, `M-Q8`,
  `M-S1..M-S10`, `M-M1`, `M-M2`.

## 3. Baseline final antes e depois

Os mesmos comandos passaram antes da primeira mutação e depois da última
restauração. Total: 18 testes, 18 GREEN em cada lado.

```text
cargo test -p typst-core repr_value_complex_types -- --nocapture                    # 1/1
cargo test -p typst-core p1285 -- --nocapture                                       # 9/9
cargo test -p typst-shell p1285_ -- --nocapture                                     # 5/5
cargo test -p typst-infra p1285_query_elements_ -- --nocapture                      # 2/2
cargo test -p typst-wiring --test cli p1285_query_stdin_transporta_yaml_ate_stdout -- --nocapture # 1/1
```

Antes do baseline pós-restauração, os nove ficheiros foram tocados sem alterar
bytes para invalidar artefactos incrementais do Cargo; os nove SHA continuaram
iguais aos backups v6.

## 4. Quinze mutantes rerodados

| # | Mutante / owner | SHA-256 mutante | Comando ou witness | Resultado | Janela / restore `cli.rs=a5c711…` |
|---:|---|---|---|---|---|
| 1 | `M-Y1` / `A-Y-HELP-EVAL` | `44976ace82f0bb07c6df5765934b1543207de02bbda9ffe9251d3682bdc45f63` | build L4; `target/debug/typst eval --help` | **Morto**: help listou somente `json, raw` | `16:50:51–16:50:58`; ✔ |
| 2 | `M-Y2` / `A-Y-TREE` | `29268d2ec0ce4742106622a633c3603d2716b66a2de14e4d134d8cd2fd846b36` | build L4; eval composto `--format yaml` | **Morto**: stdout JSON sob YAML | `16:51:06–16:51:12`; ✔ |
| 3 | `M-Y3` / `A-Y-PRETTY` | `cc42e212421cb4f2e44d99a711e51806d2b900aec4949229e62e7f6f3827c1fd` | eval YAML sem/com `--pretty` | **Morto**: segundo output virou JSON pretty | `16:51:22–16:51:29`; ✔ |
| 4 | `M-Y4` / `A-Y-QUERY-RAW` | `951950d09d739153d593cad2aa53626c6052a0729426964658744d35aaac73b5` | query stdin metadata com `--format raw` | **Morto**: exit 0/YAML; esperado parser exit 2 | `16:51:37–16:51:45`; ✔ |
| 5 | `M-J1` / `A-J-VERSION-ACCEPT` | `1e039c5768525bcd464cd6aa39073f95d5017fd132c38e5f1238d3c9ef681240` | `cargo test -p typst-shell p1285_eval_version_e_fallbacks_preservam_repr_publica -- --nocapture` | **Morto**: `cannot serialize version` | `16:51:52–16:51:57`; ✔ |
| 6 | `M-J2` / `A-J-VERSION-EXACT` | `bd4922ee41a8c2b853b689e22df1d27023cc0655fa1f92e720d386b71fac1911` | mesmo focal L2 | **Morto**: `"0.15.1"` ≠ repr da linguagem | `16:52:14–16:52:19`; ✔ |
| 7 | `M-J3` / `A-J-STRUCTURAL` | `0ba17fbae89ef8f25b56f0329559d8cd15274435acd6dc3a49639b171bdfe09f` | `cargo test -p typst-shell p1285_eval_content_sequence_e_strong_sao_estruturais -- --nocapture` | **Morto**: content virou string `sequence(...)` | `16:52:27–16:52:34`; ✔ |
| 8 | `M-J4` / `A-J-NOMINAL-REPR` | `25784e5c6a82605ecd82e439aa6295a7f14eaf304f3ac5cf569af420bd92a78e` | focal L2 de fallbacks | **Morto**: Version em Debug Rust | `16:52:42–16:52:50`; ✔ |
| 9 | `M-J5` / `A-J-RAW-REJECT` | `f4ea5094ac9a4d088e566f80b3c9ec4ead4aa1fd26c5249c7af4fc0b925e0377` | build L4; `typst eval 'sys.version' --format raw` | **Morto**: exit 0/`version(0, 15, 1)` | `16:52:59–16:53:08`; ✔ |
| 10 | `M-Q1` / `A-Q-NONHEADING` | `079acae0109e343fdd7b07cca216bb4341eeed33cefb6859e2974428e131eda3` | `cargo test -p typst-shell p1285_query_metadata_figure_e_label_preservam_morfologia -- --nocapture` | **Morto**: metadata perdeu `func` | `16:53:15–16:53:22`; ✔ |
| 11 | `M-Q2` / `A-Q-SHAPE` | `df7973980d60ea8545b9711f51da5fc23bbb14110cb9231802bda63c9a87f464` | mesmo focal de shape | **Morto**: `func` ausente | `16:53:30–16:53:37`; ✔ |
| 12 | `M-Q3` / `A-Q-ORDER` | `57731654a27c16c943a6829b8d8b06c7cbaeca094f62ec2cccbddb8213c6f028` | mesmo focal, par figure/equation | **Morto**: ordem invertida | `16:53:45–16:53:52`; ✔ |
| 13 | `M-Q4` / `A-Q-FIELD-SPLIT` | `9e87e3d71b583499b530392dda7129bc284ab640fda76b1360a518128f4b30a9` | `cargo test -p typst-shell p1285_query_field_e_one_seguem_ordem_e_fallbacks -- --nocapture` | **Morto**: lista errou em field ausente | `16:54:02–16:54:09`; ✔ |
| 14 | `M-Q5` / `A-Q-ONE-FIRST` | `2a3280f16aadf32b0acb0455796c0f690e54d2357600708849de3e279b45780a` | mesmo focal field/one | **Morto**: `found 0` substituiu erro de field | `16:54:19–16:54:25`; ✔ |
| 15 | `M-Q6` / `F-L2-LABEL-SERIALIZE` | `abc0d9cfe84a99404b83e7f9b9fb68eac95f9a6464f6178946af72ee89cc2885` | focal de metadata/figure/label | **Morto**: label tornou-se `null` | `16:54:35–16:54:41`; ✔ |

Resultado dos reruns: 15 mortos, 0 sobreviventes, 0 `Unknown`.

## 5. Quinze resultados transferidos por identidade byte a byte

Fonte da evidência: receipt v5 SHA
`d2a278e1bc77e677715c1b033e7ae27d0c862c19a9ce68b413371b82b25e8a3b`.
Cada linha abaixo preserva o consumer que recebeu a transformação v5; o SHA é
idêntico no backup v5 e na candidata v6.

| Mutante | Consumer alvo v5 | SHA v5 = SHA v6 | Resultado transferido |
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

`Unknown` não mata. Nenhum timeout, erro ambiental, ambiguidade de target ou
mutante opaco foi convertido em sucesso. O score requerido foi medido, mas não
é um veredito final.

## 7. Hashes finais e proveniência

Os mesmos hashes foram observados na origem às `2026-08-30T16:56:27-03:00` e
na cópia após restauração/baseline:

| Consumer | SHA-256 |
|---|---|
| `01_core/src/compiler/eval/mod.rs` | `b4cee9b297457726cf42fad5737edb45f0b17904ec6ebdf7f6ddc13ef24fa9ea` |
| `01_core/src/compiler/eval/repr.rs` | `cbd8205d779b40407e3406f6b4e6aeaa486e5fc84ca669cb30ab4b4cd3a7007b` |
| `01_core/src/compiler/eval/selector_matching.rs` | `09014da2b4a4737db6e1e996a3d001cc9fbdcd356f84890f5020f4a324ec80f9` |
| `01_core/src/compiler/eval/rules.rs` | `d9bef037b7c4fb025193fec210f7015a5aceae2f73c1317d7db36bbaab01ae3f` |
| `01_core/src/compiler/eval/math.rs` | `2c7d920337ee46a3423db5a0809bab73e11685c8cf0f32e4984091143976ba1e` |
| `01_core/src/compiler/stdlib/foundations/selector.rs` | `b8c1460f1ddd97b70b20a00366d1a5e8c2cd76e37cefb295e1baf757b1cdd179` |
| `02_shell/src/cli.rs` | `a5c7110577501564004ad2f6d84a29ed1a282c94a412364b73e3b493e75d329f` |
| `03_infra/src/query_helpers.rs` | `e3157124d94bc83259be375bb0798c0cee41a9f20516c0c45ef0e351182e0aa8` |
| `04_wiring/src/main.rs` | `073c4b8967ac20c5cc3d69ab0072283b6f06754ad8d54ebc207e475dde4371b6` |

`git diff HEAD --stat` na origem: `94 files changed, 645599 insertions(+),
1384 deletions(-)`. A árvore compartilhada já estava massivamente suja; este
papel escreveu nela somente o plano v6 e este receipt v2. Plano v5, receipt v5,
contrato, RED, testes, L0, produção e oráculos não foram editados.
