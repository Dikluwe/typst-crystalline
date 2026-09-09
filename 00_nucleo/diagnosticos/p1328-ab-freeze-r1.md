# P1328 — freeze independente A/B r1

Congelado em 2026-09-09T11:43 UTC, antes de C. Executor: subagente
`/root/p1328_tests`, workspace compartilhado, capacidades sem isolamento
atestado. Regime A/B sem contrato de refinamento nem selo de refinamento.

Entradas: skill Tekt e ambas referências integralmente; L0 calc integral
SHA-256 `2c7494c628038975cd52d6e093f66dc7e78a435d69effdc17ee6eb312213367e`;
APIs públicas World/Args/Value/Source/Span/EvalContext/Locator e exemplos
test-only P1326 e eval/tests.rs. Leitura runtime calc.rs e patch C vedadas e
não realizadas. Leitura p1328-baseline.json, materialization e context vedada
e não realizada. Escritas somente diagnósticos p1328-ab-*. Sem acesso a
veredito/implementação; root integra snippet sem alteração e executa RED.

Fontes normativas e interfaces foram lidas diretamente; nenhum código
produtivo foi escrito. A busca limitada às ADRs não encontrou ADR específica
de segregação. ADR-0127 foi lida integralmente. Árvores/bits binários completos
de medição são responsabilidade do recibo de proveniência do root, pois a
autoridade A/B não lê diffs produtivos. HEAD dos recibos e hashes binários
identificam as entradas realmente executadas; métricas abaixo não encerram C.

Artefatos congelados:

- Snippet rustfmt integral `p1328-ab-tests-r1.rs`, SHA-256
  `fa971dded3237a1742905db2a45e3d94aef51a853fed17ca8046a0a8ba1cd353`.
  Integração: append EOF calc.rs, módulo cfg(test) p1328_tests,
  `cargo test -p typst-core --release --locked p1328` no target do root.
- Runner `p1328-ab-cli.py`, SHA-256
  `326235f263b308a59cb3d50726270ad6c262afe8a3591f1b2c23498fc3b0875d`.
- Medição válida BASE/VANILLA `p1328-ab-cli-baseline-r1.json`, SHA-256
  `8f9ddaead847be551b7eb779fbf6719ece3043e84714d872b03c278c9e78db5c`.
- Expectativas literais `p1328-ab-cli-expected-r1.json`, SHA-256
  `9d633ccf379d5a1124723ff1919f388b0603aab8cd4cd940456aa33144369623`.

A medição válida contém 28 casos × 4 perfis, exit/stdout/stderr completos,
argv e hashes. BASE SHA-256
`75e8b97b3788c0feaf457cb4c06b1c2c6735cff5ef3b9804a75ec57f8b148e31`;
VANILLA ratificado upstream `a51e02804`, SHA-256
`7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
Os três positivos numéricos já têm paridade integral nos quatro perfis.
Rejeições String/Symbol/Length/Angle/Ratio/Fr, guards, overflow e sqrt
preservam saída BASE, expressamente sem alegação de paridade.

Os casos `content-with-bound`, `content-with-nested`, `content-spread-args`
têm diagnóstico primário e origem vanilla, mas conservam o nome do trace
externo BASE `calc.abs` (vanilla `abs`): dívida de dispatcher explicitamente
preservada pelo L0, que não muda esse owner. As saídas sucessoras completas
foram materializadas antes de C no arquivo de expectativas; comparação
candidata é igualdade literal, sem normalização. Qualquer outra diferença,
origem ausente ou output opaco é falha/Unknown, jamais sucesso implícito.

O recibo `p1328-ab-cli-baseline.json` anterior é instrumentação inválida:
vanilla não aceita `eval --color never`. Não é evidência RED nem paridade.
Hipótese reparada antes do freeze: retirar flag dos argv de ambos; pipes
capturam diagnóstico sem cor. Custo: duas execuções BASE/VANILLA, sem C.

Próximos gates: RED do snippet congelado; GREEN mesmos bytes;
runner --candidate em ordem normal, repetida e --order reverse. Manter
artefatos congelados. Sem expansão do corpus ou alegação geral de calc.
