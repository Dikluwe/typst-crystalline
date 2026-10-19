# Fixtures ativos e resíduos de CI

O teste de New Computer Modern Math passou a participar da suíte. Foram
retiradas chamadas de introspecção duplicadas nos fixtures, locals sem uso
e comparações booleanas desnecessárias da CLI. O inventário de Clippy
passou de 333 para 252 diagnósticos, sem supressões adicionais.

## Proveniência e medição anterior à decisão

Branch Tekt, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree não
commitado**, medição inicial em `2026-10-14T01:03:51.805190+00:00`.
`before.json.gz` conserva fontes, owners, spans, status e
`git diff HEAD --stat`. O registro antes das alterações foi obtido do
libtest real, não por contagem textual de funções. L0 atualizado antes
do código; resselo limitado aos quatro consumers e respectivos owners.

## Correções e evidência

- `embedded_fonts.rs`: a função p754_newcm_math_is_not_in_text_group não
  tinha `#[test]` e não aparecia na lista executável. Seu registro foi
  ativado mantendo as asserções de presença em math/code e exclusão de
  text. `registration-red.json` prova ausência no registro anterior;
  `registration-green.json` e o log comprovam registro e execução final.
  RED aqui significa falha de registro, não falha prévia da classificação.
- A sonda p797_probe_list_all_embedded_families apenas imprimia o catálogo.
  Foi retirada, com conteúdo preservado em removed-probe.rs.txt. O teste
  de seleção Regular/Bold/Italic e suas asserções continuam. O teste DejaVu
  agora exige presença real em math/code, além da exclusão de text: um
  catálogo vazio já não satisfaz esse controle. Nenhuma fonte é opcional
  nesses controles de assets embutidos.
- `integration_tests.rs`: 65 resultados de introspect(content) eram
  descartados imediatamente antes de layout(content). O wrapper em
  compiler/introspect.rs:135 e o início de layout em layout/mod.rs:1703
  usam a mesma introspect_with_introspector. Foram retiradas somente as
  chamadas duplicadas: 63 diagnosticadas e duas que usavam _state.
  O import introspect ficou sem uso e saiu; introspect_with_introspector
  segue usado no helper. A inspeção adicional está em refinement-before;
  a primeira validação completa está em initial-validation. Inputs,
  asserções e nomes são idênticos.
  Não se atribui ganho de desempenho sem ensaio específico.
- `shaper/tests.rs`: retirados o cálculo sem uso de self.fonts.len() e
  o FontBook local que não era usado pelo SystemWorld do fixture.
  Asserções e condições de disponibilidade de fontes continuam iguais.
  Esses testes antigos condicionais não recebem certificação adicional
  por esta limpeza.
- `cli.rs`: 14 assert_eq! contra booleanos passaram a assert! com a
  mesma polaridade, preservando argumentos de Never/Always/Auto, TTY e
  NO_COLOR. Nenhuma assinatura, default ou código produtivo mudou.

`preservation-proof.json` verifica igualdade do prefixo produtivo nos
módulos mistos, igualdade dos fixtures restantes após as remoções
selecionadas e equivalência textual das polaridades de CLI. É um controle
mecânico delimitado, não prova de paridade de linguagem.

## Validação

Checagens estáticas concluídas em `2026-10-14T01:08:07.477352+00:00`; suítes shell e
infra concluídas em `2026-10-14T01:09:49.345069+00:00`: **1409 passaram,
0 falhas, 2 ignorados**. Os ignorados não recebem crédito.
Execução serial com Cargo offline/locked, TMPDIR isolado em /dev/shm,
linter publicado pinado e oráculo `9f2b6e8715237cb086899a42873660fe744622e8`
com SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`.
Ambiente, comandos e hashes de fonte estão em workspace-pre.json.gz;
apesar do nome do arquivo, o comando usa `-p typst-infra -p typst-shell`,
não `--workspace`. Nenhuma fonte mudou durante a execução.

Registro infra: 1284 → 1284 nomes, troca exata da sonda
p797 pelo teste p754; registro shell preservado com 90 nomes.
Build, check workspace/all-targets, formato, Rustdoc com -D warnings,
linhagem estrita, reversa e diff check passaram. Linter arquitetural:
0 erros, 380 avisos e 1611 notas.

Clippy: menos 65 unused_variables, um unused_mut, um dead_code e
14 bool_assert_comparison. Distribuição final: core 37,
shell 0, infra 202, wiring 13.
O inventário usa --cap-lints warn apenas para coletar todos os crates;
o gate real -D warnings permanece pendente. O zero em shell refere-se
a este inventário, não à certificação integral da CLI.

Este lote não repete a suíte completa anterior do workspace nem a matriz
remota. Pendências funcionais da auditoria, avisos de Clippy e verificação
de release continuam abertos. Nenhuma release foi publicada.

Snapshots comprimidos e `evidence.json.gz` preservam fontes, owners,
receitas, logs, diffs e inventários; hashes do arquivo de evidências são
conferidos após descompressão.
