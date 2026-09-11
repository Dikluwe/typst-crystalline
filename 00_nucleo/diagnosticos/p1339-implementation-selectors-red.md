# P1339 — RED local do implementador de seletores

Executor `/root/p1339_observation_design`, subordinado implementador;
regime executado sem atestação de isolamento. Este RED local não substitui
o RED independente já aceito, não é selo e não é veredito.

Autoridade verificada SHA-256
`cab15a1dce423a12fe198a640b541ee00b3f8743ec0f6a9e97519416d940f3c0`;
selo `35f00c4b9e15a010692017f5083ea4f451f4104a3730022136972e151ac0a8ee`;
RED independente `e4e564e606283c15ef792d59de2d36f1a402d6d5d5077f7d6581b23219076f7d`;
aceitação `91f3953682384cfe4f195e3462611ca4c1ab328c5ac859eefc0c0398731852c7`.

Estado: HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, working tree
não commitado. Os consumers já tinham headers mecanicamente ressellados
pelo coordenador. Antes da implementação deste executor, acrescentei apenas
um teste em `01_core/src/compiler/eval/selector_matching.rs`.
Outros implementadores acrescentavam testes em call_dispatch,
stdlib/foundations/float e primitives_constructors/version. A presença de
diff nesses arquivos não é apresentada como produção implementada.

Comando de build/execução:

```text
CARGO_TARGET_DIR=/tmp/p1339-target.UD8gh7 cargo test -p typst-core --lib p1339_where_public_empty_group_and_numeric_equality -- --nocapture
```

Uma primeira invocação com `--exact` e filtro curto foi insuficiente;
o comando acima executou de fato o teste e retornou 101. Repetição direta
em `2026-09-10T05:44:00Z`, cwd `/repos/Antigravity/typst-crystalline`:

```text
/tmp/p1339-target.UD8gh7/debug/deps/typst_core-d51858d7cd5964e0 --exact compiler::eval::selector_matching::tests::p1339_where_public_empty_group_and_numeric_equality --nocapture
```

Binário SHA-256:
`c01e5713ebb7f4beb87d8d0977ef77d40768c423d63fe220050b8b0efe59d3b7`.
Fonte do teste SHA-256 naquele momento:
`df2e866646093d1239bb1c8dc37def7754b2cbeb5c8d482376ea9ae8b7a2cba3`.

Saída relevante integral da repetição, exit 101:

```text
running 1 test

thread 'compiler::eval::selector_matching::tests::p1339_where_public_empty_group_and_numeric_equality' (3) panicked at 01_core/src/compiler/eval/selector_matching.rs:337:14:
P1339 public where accepts an explicitly empty element filter: [SourceDiagnostic { severity: Error, span: Span(287359686831143), message: "selector where não suportado para strong", hints: [], trace: [] }]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test compiler::eval::selector_matching::tests::p1339_where_public_empty_group_and_numeric_equality ... FAILED

failures:
    compiler::eval::selector_matching::tests::p1339_where_public_empty_group_and_numeric_equality

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 5649 filtered out; finished in 0.01s
```

O teste usa eval real com registry vazio e exige grupo explicitamente vazio
de strong, repr `strong.where(:)` e igualdade de filtros level 1/1.0.
Falha na primeira obrigação sem erro de runner ou fixture. O enum novo
ainda não foi escrito por este executor. O total do diff de L1 nessa
captura foi 30 arquivos, 134 inserções e 30 remoções: os 30 headers
ressellados, teste novo de selector_matching (27 linhas), testes novos de
call_dispatch (17), float (36) e version (24). Não foram alteradas
expectativas antigas deste owner.
