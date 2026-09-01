# P1285 — receipt adversarial de mutation testing

Estado: campanha executada; este documento **não emite veredito final**.

## 1. Autoridade, segregação e estado medido

- Regime: protocolo completo de materialização segregada, papel adversarial independente.
- Início da campanha: `2026-08-30T16:07:45-03:00`.
- Fim das mutações: `2026-08-30T16:26:36-03:00`.
- Fecho probatório: `2026-08-30T16:29:35-03:00`.
- Receipt escrito: `2026-08-30T16:32:27-03:00`.
- HEAD: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Execução: cópia explícita `/tmp/p1285-adversarial-v5.AuqIKr/repo`, criada com `cp -a --reflink=always`; backups byte a byte em `/tmp/p1285-adversarial-v5.AuqIKr/backups`.
- A árvore principal não recebeu nenhuma mutação. Cada mutação foi aplicada somente à cópia, executada isoladamente e seguida por `cp --preserve=all` do backup e `sha256sum` do ficheiro restaurado.
- Não foram lidos nem executados oráculo, baseline do oráculo, runner, receipt do oráculo, `00_nucleo/materialization/` ou `00_nucleo/context/`.
- Atestação: segregação operacional comprovada pelos paths e hashes; não se alega isolamento ambiental forte além disso.

### Cadeia pinada

| Artefacto | SHA-256 |
|---|---|
| contrato | `647176b3d0840a3c14a2ac837f401d43090fdb4ad253ce8e5f90ba13810899a2` |
| RED receipt | `3f093f748769ccbb128183ca73cfab9052f2a7d6b097503ab5e5ccff2532f25d` |
| plano v4 preservado | `306ae3032df39661b71b313f1bf7251eedf913ddf7b0287005b8c6d849285212` |
| plano v5 repinado | `9cf5b7304c9761c245ea8be544bb7dbdd8bf6c833822e3f268dbe0c0f23a743a` |

### Candidato congelado, antes e depois da campanha

Os mesmos hashes foram observados na árvore principal, na cópia antes da primeira mutação e na cópia após a última restauração:

| Ficheiro | SHA-256 congelado/restaurado |
|---|---|
| `01_core/src/compiler/eval/mod.rs` | `b4cee9b297457726cf42fad5737edb45f0b17904ec6ebdf7f6ddc13ef24fa9ea` |
| `01_core/src/compiler/eval/repr.rs` | `aa47dd1f62f8ccfb25e54166b32c9c64b796f743968c28708d87d42ee2ff853b` |
| `01_core/src/compiler/eval/selector_matching.rs` | `09014da2b4a4737db6e1e996a3d001cc9fbdcd356f84890f5020f4a324ec80f9` |
| `01_core/src/compiler/eval/rules.rs` | `d9bef037b7c4fb025193fec210f7015a5aceae2f73c1317d7db36bbaab01ae3f` |
| `01_core/src/compiler/eval/math.rs` | `2c7d920337ee46a3423db5a0809bab73e11685c8cf0f32e4984091143976ba1e` |
| `01_core/src/compiler/stdlib/foundations/selector.rs` | `b8c1460f1ddd97b70b20a00366d1a5e8c2cd76e37cefb295e1baf757b1cdd179` |
| `02_shell/src/cli.rs` | `e00dbc1f445f6c3ab1976740daeebb4c4e2a6668fb409d8c25d14ae0e206857b` |
| `03_infra/src/query_helpers.rs` | `e3157124d94bc83259be375bb0798c0cee41a9f20516c0c45ef0e351182e0aa8` |
| `04_wiring/src/main.rs` | `073c4b8967ac20c5cc3d69ab0072283b6f06754ad8d54ebc207e475dde4371b6` |

## 2. Baseline dos testes congelados

Antes da primeira mutação, os oito comandos do receipt RED passaram: 17 testes, 17 GREEN. Após a última restauração, os mesmos oito comandos passaram novamente: 17 testes, 17 GREEN.

```text
cargo test -p typst-shell p1285_ -- --nocapture                                      # 5/5
cargo test -p typst-core p1285_selector_ -- --nocapture                              # 2/2
cargo test -p typst-core p1285_native_selector_ -- --nocapture                       # 3/3
RUSTFLAGS=-Awarnings cargo test -p typst-core p1285_splice_regex_ -- --nocapture      # 2/2
cargo test -p typst-core p1285_show_literal_e_regex_entregam_cada_ocorrencia_a_recipe -- --nocapture # 1/1
cargo test -p typst-core p1285_math_grapheme_e_numero_preservam_variantes_distintas -- --nocapture   # 1/1
cargo test -p typst-infra p1285_query_elements_ -- --nocapture                       # 2/2
cargo test -p typst-wiring --test cli p1285_query_stdin_transporta_yaml_ate_stdout -- --nocapture    # 1/1
```

No primeiro fecho, `cp --preserve=all` também restaurou o mtime e o Cargo reutilizou um artefacto incremental anterior, produzindo uma falha espúria em `p1285_query_field_e_one_seguem_ordem_e_fallbacks`. Esse resultado foi invalidado, os nove ficheiros foram tocados sem alteração de bytes (todos os SHA permaneceram congelados) e a bateria completa foi reconstruída e passou 17/17. Nenhum número de score usa a execução espúria.

## 3. Matriz executada — 30 mutantes

`Morto` significa que o owner observou a divergência requerida. O SHA na coluna “mutante” é o hash do ficheiro mutado imediatamente antes da execução. “restore” significa comparação com o SHA congelado da secção 1 após o teste.

| # | Mutante / owner | Mutação e SHA do mutante | Comando/witness executado | Resultado observado | Janela / restore |
|---:|---|---|---|---|---|
| 1 | `M-S7` / `F-S-SELECTOR-IDENTITY` | rejeitar `Value::Selector`; `c9d86cd13008172c070a41354ba0dd8cb9b774db3da95448b85318a588195217` | `RUSTFLAGS=-Awarnings cargo test -p typst-core p1285_native_selector_preserva_selector_por_identidade -- --nocapture` | **Morto**: owner recebeu `Err` em vez da identidade | `16:11:39–16:12:18`; restore ✔ |
| 2 | `M-J3` / `A-J-STRUCTURAL` | content por repr; `818c28794dda95c2bb798f7621b4baa3e575647dde8e5510093b72c254aaf1d4` | `cargo test -p typst-shell p1285_eval_content_sequence_e_strong_sao_estruturais -- --nocapture` | **Morto**: string `sequence(...)` em vez de árvore | `16:12:41–16:13:00`; restore ✔ |
| 3 | `M-Q6` / `F-L2-LABEL-SERIALIZE` | perder label; `0a83a2862929f24f58a22692b596dd3183a488050ebb796526c3627abbf2cbb1` | `cargo test -p typst-shell p1285_query_metadata_figure_e_label_preservam_morfologia -- --nocapture` | **Morto**: `label` tornou-se `null` | `16:13:13–16:13:19`; restore ✔ |
| 4 | `M-Y2` / `A-Y-TREE` | YAML despachado por JSON; `0863590e7dc276311e89aa2686cd72f058de887428125689cec66673658b0f09` | owner congelado + `cargo run -q -p typst-wiring -- eval '(a: 1, b: (2, 3), v: sys.version, t: [Hi])' --format yaml` | **Morto**: stdout foi JSON (`{"a":1,...}`) sob YAML | `16:13:27–16:13:57`; restore ✔ |
| 5 | `M-M1` / `F-M-MORPHOLOGY` | Grapheme→MathText; `18ad2bf1f6eb18bb8eadfc6c73b018cf0bfa660dfd409a12e3f029405c92f0f7` | `cargo test -p typst-core p1285_math_grapheme_e_numero_preservam_variantes_distintas -- --nocapture` | **Morto**: `$x$` observado como `math.text("x")` | `16:14:13–16:14:53`; restore ✔ |
| 6 | `M-S1` / `A-S-DOT` | literal compilado como regex; `11cfca295d4a25982a2d5ee8ed7a078969cbed4b1f3d627828e607b12fb5195e` | build L4 + `typst compile .../s1.typ .../s1-mut.svg --format svg`, fonte `#show ".": it => [X]` / `A.B` | **Morto**: stack overflow/abort; baseline termina com sucesso | `16:16:07–16:16:25`; restore ✔ |
| 7 | `M-Q3` / `A-Q-ORDER` | inverter resultados; `3c99688bf99215c757ee229ce3aa1a61afc3a92131f094add2b2876481d7271e` | `cargo test -p typst-shell p1285_query_metadata_figure_e_label_preservam_morfologia -- --nocapture` | **Morto**: equation precedeu figure | `16:16:51–16:16:59`; restore ✔ |
| 8 | `M-J1` / `A-J-VERSION-ACCEPT` | rejeitar Version; `fe95532b2d353cac4794fd8d01c29eaf5d6059f77d8947914834a01b488821b4` | `cargo test -p typst-shell p1285_eval_version_e_fallbacks_preservam_repr_publica -- --nocapture` | **Morto**: `cannot serialize version` | `16:17:07–16:17:13`; restore ✔ |
| 9 | `M-Y5` / `F-L4-STDIN-YAML` | L4 força JSON; `843dfe59381a4e7b679d2a8e44d312f5f188acc3d429292c1e00ee7e6a669086` | `cargo test -p typst-wiring --test cli p1285_query_stdin_transporta_yaml_ate_stdout -- --nocapture` | **Morto**: JSON em vez de YAML | `16:17:27–16:17:33`; restore ✔ |
| 10 | `M-S10` / `F-S-SHOW-OCCURRENCE` | recipe regex uma vez/nó inteiro; `b558c5e8f9546924d27de0b2f0955f86d4bee5939d7c93c0c13947c5096111e9` | `RUSTFLAGS=-Awarnings cargo test -p typst-core p1285_splice_regex_substitui_todas_as_ocorrencias_em_ordem -- --nocapture` | **Morto**: `['foo fxo']` em vez de `['foo','fxo']` | `16:17:55–16:18:39`; restore ✔ |
| 11 | `M-Q1` / `A-Q-NONHEADING` | serializer heading-only; `3c9b30dd6f771460caffa99695832a5c2f1f95738098e9576606370dbf490511` | `cargo test -p typst-shell p1285_query_metadata_figure_e_label_preservam_morfologia -- --nocapture` | **Morto**: metadata perdeu `func` | `16:18:50–16:18:58`; restore ✔ |
| 12 | `M-S4` / `F-S-EMPTY-REJECT` | aceitar os três vazios; `97bef5b654dab488dd4336a77e8024ad3bbbcfaa08b2371f12d7e162790df9ba` | `RUSTFLAGS=-Awarnings cargo test -p typst-core p1285_native_selector_distingue_tres_diagnosticos_de_vazio -- --nocapture` | **Morto**: string vazia retornou selector | `16:19:09–16:19:41`; restore ✔ |
| 13 | `M-J5` / `A-J-RAW-REJECT` | permitir Version raw; `b67d2159033c3d6beb892dc45103fccd8494b2b46e864f76743c3236d75d895a` | build L4 + `target/debug/typst eval 'sys.version' --format raw` | **Morto**: exit 0 e `version(0, 15, 1)`; esperado exit 1 | `16:20:01–16:20:09`; restore ✔ |
| 14 | `M-Y1` / `A-Y-HELP-EVAL` | ocultar YAML do enum/help; `7189e5d2157abda8fb8ca029451395aa7846ce5a4d7711467eea5a0f9748692f` | build L4 + `target/debug/typst eval --help` | **Morto**: possíveis valores `[json, raw]` | `16:20:28–16:20:35`; restore ✔ |
| 15 | `M-Q8` / `A-Q-STDIN-IDENTITY` | stdin com parser code; `8792155ba04fa97072e6a08a2fd40981139789c391b66b9da4d2c34a23f2c6f0` | `cargo test -p typst-wiring --test cli p1285_query_stdin_transporta_yaml_ate_stdout -- --nocapture` | **Morto**: `#` inválido em code; exit 1 | `16:20:56–16:21:04`; restore ✔ |
| 16 | `M-S2` / `A-S-ALL` | só primeiro match; `d467244d0974d9a82c7f8b85534803efd53087ff53b44f13f47a25c46e8ef588` | `RUSTFLAGS=-Awarnings cargo test -p typst-core p1285_splice_regex_substitui_todas_as_ocorrencias_em_ordem -- --nocapture` | **Morto**: só `foo`, sem `fxo` | `16:21:18–16:21:35`; restore ✔ |
| 17 | `M-J4` / `A-J-NOMINAL-REPR` | fallback Debug; `9e24059286f4840ee6bc21c08b8d6fe9c71905e1b54a05f409017906703aabd7` | `cargo test -p typst-shell p1285_eval_version_e_fallbacks_preservam_repr_publica -- --nocapture` | **Morto**: `Version(Version { components: ... })` | `16:21:43–16:21:50`; restore ✔ |
| 18 | `M-Q5` / `A-Q-ONE-FIRST` | field antes da cardinalidade; `1b394bd4f25b3b3a291e4563f007365ffd2dd147c468ec43a07750a9bdbd7423` | `cargo test -p typst-shell p1285_query_field_e_one_seguem_ordem_e_fallbacks -- --nocapture` | **Morto**: `found 0` substituiu `no such field` | `16:21:59–16:22:05`; restore ✔ |
| 19 | `M-M2` / `F-M-MORPHOLOGY` | Number→MathIdent; `ad8febc4fec4a6bca9be9eaabe987143478241904ce2f64d9e5abadb3f06f4ff` | `RUSTFLAGS=-Awarnings cargo test -p typst-core p1285_math_grapheme_e_numero_preservam_variantes_distintas -- --nocapture` | **Morto**: `$2$` observado como `math.ident("2")` | `16:22:14–16:22:30`; restore ✔ |
| 20 | `M-S8` / `F-S-REGEX-CONSTRUCT` | devolver Regex cru; `656322d19c14eda15996ad75f005f8b40d7c401359534462e5bdc20aa78a2b6f` | `RUSTFLAGS=-Awarnings cargo test -p typst-core p1285_native_selector_converte_regex_valida_em_selector -- --nocapture` | **Morto**: `Regex(...)` ≠ `Selector(Regex(...))` | `16:22:37–16:22:52`; restore ✔ |
| 21 | `M-Y3` / `A-Y-PRETTY` | pretty muda YAML para JSON pretty; `fad2e5e78fc1477c1934afc5bd9d5f2665a8bd9c8366b33bd21bd2831b4c474f` | duas execuções de `target/debug/typst eval '(a: 1, b: (2, 3))' --format yaml`, segunda com `--pretty` | **Morto**: outputs `a: 1...` vs `{ "a": 1... }` | `16:23:13–16:23:21`; restore ✔ |
| 22 | `M-Q2` / `A-Q-SHAPE` | omitir `func`; `dde577b1c60d416a5b80a54d063b94ca5bbdc8befbd3e034b2f48f71e6d6df7e` | `cargo test -p typst-shell p1285_query_metadata_figure_e_label_preservam_morfologia -- --nocapture` | **Morto**: `func` nulo | `16:23:28–16:23:36`; restore ✔ |
| 23 | `M-S6` / `A-S-PARTIAL` | literal exige nó inteiro; `09aec20078789a5503ec25212bb840dbaaa7e2c868d2125b4c52fc3c694bdc38` | `RUSTFLAGS=-Awarnings cargo test -p typst-core p1285_selector_literal_casa_ocorrencia_local_sem_virar_node_rule -- --nocapture` | **Morto**: `b` deixou de casar `abc` | `16:23:46–16:23:59`; restore ✔ |
| 24 | `M-J2` / `A-J-VERSION-EXACT` | Version→`0.15.1`; `08c1efd02e573a0502465611708e64fc4887f8b24a3d737ddd2cfa073a9b2b07` | `cargo test -p typst-shell p1285_eval_version_e_fallbacks_preservam_repr_publica -- --nocapture` | **Morto**: `"0.15.1"` ≠ repr da linguagem | `16:24:08–16:24:13`; restore ✔ |
| 25 | `M-Q7` / `A-Q-STDIN-EQUAL` | `-` tratado como path; `9153a2df4fa6d6eb6f5ce48538945a407431bf0e861476bef98fc4dca0584a5c` | `cargo test -p typst-wiring --test cli p1285_query_stdin_transporta_yaml_ate_stdout -- --nocapture` | **Morto**: `main file not found: ./-`, exit 2 | `16:24:21–16:24:27`; restore ✔ |
| 26 | `M-S9` / `F-S-EMPTY-DISTINCT` | mensagens vazias confladas; `eb8208893b9e14370850fae8768d70b32e462edeae2668139ddc752ef751c8fb` | `RUSTFLAGS=-Awarnings cargo test -p typst-core p1285_native_selector_distingue_tres_diagnosticos_de_vazio -- --nocapture` | **Morto**: `selector is empty` ≠ diagnósticos distintos | `16:24:41–16:24:58`; restore ✔ |
| 27 | `M-Y4` / `A-Y-QUERY-RAW` | aceitar raw em query; `eaf445d66e4ea32a1ed3d13a4eaf341e568de293cba625f2095052d8359f62ca` | build L4 + stdin metadata em `target/debug/typst query - metadata --field value --one --format raw` | **Morto**: exit 0 e YAML; esperado parser exit 2 | `16:25:16–16:25:24`; restore ✔ |
| 28 | `M-S3` / `A-S-NONMATCH` | transformar no-match; `69238c0f7349b01e56b0df11aaad278f820be11a80acc40d20f34784f35589c0` | `RUSTFLAGS=-Awarnings cargo test -p typst-core p1285_splice_regex_preserva_no_match_e_ignora_match_somente_vazio -- --nocapture` | **Morto**: `Some(text("substituído"))` em vez de `None` | `16:25:32–16:25:48`; restore ✔ |
| 29 | `M-Q4` / `A-Q-FIELD-SPLIT` | field ausente sempre erra; `b8f108094cbd149468cfaa26daf0fb245e7c3962073b4427da4d1772d32db907` | `cargo test -p typst-shell p1285_query_field_e_one_seguem_ordem_e_fallbacks -- --nocapture` | **Morto**: lista recebeu `no such field` em vez de `[]` | `16:25:59–16:26:05`; restore ✔ |
| 30 | `M-S5` / `A-S-NONLOCATABLE` | text/regex viram Heading locatável; `2ddba8d3d3e0d518b026bcd79d88ba9ffea753bec92a64a788e4b9ce177e07c8` | build L4 + queries com `"foo"` e `regex("f.o")` | **Morto**: ambos exit 0/`[]`; esperado diagnóstico não-locatável | `16:26:29–16:26:36`; restore ✔ |

Duas tentativas de orquestração foram descartadas e não entram na matriz: a primeira execução de M-S7 restaurou o ficheiro enquanto o processo ainda terminava e foi repetida corretamente; a primeira forma de M-S4 não compilava por usar um variant inexistente e foi corrigida para um mutante semântico compilável antes da medição contada.

## 4. Unknown e score

Política aplicada: timeout, dependência externa ou incapacidade ambiental seria `Unknown`; `Unknown` não mata e não entra no numerador. Não ocorreu nenhum caso.

```text
obrigatórios = 30
mortos       = 30
sobreviventes = 0
unknown       = 0
denominador   = obrigatórios - unknown = 30
score         = mortos / denominador = 30 / 30 = 1.0
```

O score requerido pelo contrato (`1.0`) foi medido. Esta frase reporta a campanha e não substitui o veredito segregado posterior.

## 5. Proveniência da árvore de trabalho

Medição em `2026-08-30T16:29:35-03:00`, HEAD `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`:

```text
git diff HEAD --stat
94 files changed, 645599 insertions(+), 1383 deletions(-)
```

O stat é da árvore compartilhada e já estava massivamente sujo; inclui trabalho do utilizador e de outros papéis. A campanha não atribui ownership dessas mudanças. O inventário de paths do stat observado foi:

```text
.gitignore
.typ/sec_04.typ
.typ/sec_07.typ
.typ/sec_09.typ
.typ/sec_10.typ
.typ/sec_12.typ
.typ/sec_16.typ
.typ/sec_17.typ
.typ/sec_18.typ
.typ/sec_22.typ
.typ/sec_27.typ
00_nucleo/diagnosticos/p1282-crystalline-default.json
00_nucleo/diagnosticos/p1282-crystalline-html.json
00_nucleo/diagnosticos/p1282-inventory-default.json
00_nucleo/diagnosticos/p1282-inventory-html.json
00_nucleo/diagnosticos/p1282-probes-default.json
00_nucleo/diagnosticos/p1282-probes-html.json
00_nucleo/diagnosticos/p1282-summary.json
00_nucleo/diagnosticos/p1282-vanilla-default.json
00_nucleo/diagnosticos/p1282-vanilla-html.json
00_nucleo/diagnosticos/typst-passo-1281-relatorio.md
00_nucleo/diagnosticos/typst-passo-1282-relatorio.md
00_nucleo/prompts/compiler/eval.md
00_nucleo/prompts/compiler/eval/bindings/field_access.md
00_nucleo/prompts/compiler/eval/bindings/value_methods.md
00_nucleo/prompts/compiler/eval/call_dispatch.md
00_nucleo/prompts/compiler/eval/math.md
00_nucleo/prompts/compiler/eval/repr.md
00_nucleo/prompts/compiler/eval/rules.md
00_nucleo/prompts/compiler/eval/selector_matching.md
00_nucleo/prompts/compiler/stdlib/collections.md
00_nucleo/prompts/compiler/stdlib/color.md
00_nucleo/prompts/compiler/stdlib/emoji.md
00_nucleo/prompts/compiler/stdlib/foundations/color.md
00_nucleo/prompts/compiler/stdlib/foundations/selector.md
00_nucleo/prompts/compiler/stdlib/foundations/str.md
00_nucleo/prompts/compiler/stdlib/structural/math.md
00_nucleo/prompts/compiler/stdlib/structural/outline.md
00_nucleo/prompts/compiler/stdlib/sym.md
00_nucleo/prompts/entities/color.md
00_nucleo/prompts/entities/selector.md
00_nucleo/prompts/infra/export-fixtures.md
00_nucleo/prompts/infra/query-helpers.md
00_nucleo/prompts/shell/cli.md
00_nucleo/prompts/wiring.md
01_core/Cargo.toml
01_core/src/compiler/eval/bindings/field_access.rs
01_core/src/compiler/eval/bindings/mod.rs
01_core/src/compiler/eval/bindings/value_methods.rs
01_core/src/compiler/eval/call_dispatch.rs
01_core/src/compiler/eval/math.rs
01_core/src/compiler/eval/mod.rs
01_core/src/compiler/eval/repr.rs
01_core/src/compiler/eval/rules.rs
01_core/src/compiler/eval/selector_matching.rs
01_core/src/compiler/eval/tests.rs
01_core/src/compiler/stdlib/collections.rs
01_core/src/compiler/stdlib/color.rs
01_core/src/compiler/stdlib/emoji.rs
01_core/src/compiler/stdlib/foundations/color.rs
01_core/src/compiler/stdlib/foundations/selector.rs
01_core/src/compiler/stdlib/foundations/str.rs
01_core/src/compiler/stdlib/mod.rs
01_core/src/compiler/stdlib/state.rs
01_core/src/compiler/stdlib/structural/math.rs
01_core/src/compiler/stdlib/structural/outline.rs
01_core/src/compiler/stdlib/sym.rs
01_core/src/entities/color.rs
01_core/src/entities/selector.rs
02_shell/src/cli.rs
03_infra/fixtures/fonts/LICENSE-NotoSans.md
03_infra/fixtures/fonts/NotoSans-Regular.ttf
03_infra/fixtures/p307b/MANIFEST.md
03_infra/src/export/svg.rs
03_infra/src/p307b_snapshot_tests.rs
03_infra/src/query_helpers.rs
04_wiring/src/main.rs
04_wiring/tests/cli.rs
Cargo.lock
Cargo.toml
crystalline.toml
lab/parity/matrix/manifest.yaml
lab/parity/matrix/runner.py
lab/parity/matrix/test_runner.py
lab/surface-inventory/Cargo.lock
lab/surface-inventory/extra_seeds.json
lab/surface-inventory/merge.py
lab/surface-inventory/probes.json
lab/surface-inventory/run_probes.py
lab/surface-inventory/src/main.rs
lab/surface-inventory/summarize.py
lab/surface-inventory/test_merge.py
lab/surface-inventory/test_run_probes.py
lab/typst-original/crates/p1140-inventory/src/main.rs
```

Artefactos escritos por este papel na árvore principal: somente `00_nucleo/diagnosticos/p1285-adversarial-mutation-plan-v5.md` e este receipt. O plano v4, contrato, RED receipt, testes, L0, produção e oráculos não foram editados.
