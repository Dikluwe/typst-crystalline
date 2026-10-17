# Organização dos módulos de teste para o CI

Os módulos de teste sinalizados pelo Clippy foram movidos para o final dos
respectivos arquivos. Cada bloco conserva seu conteúdo, atributos, comentários,
nome e escopo de módulo. Os demais itens conservam conteúdo e ordem relativa.
Não foram alterados algoritmos, contratos públicos, defaults ou fases.

## Proveniência e medição anterior à decisão

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**. `ordem-testes-ci-2026-10-13/before.json.gz` conserva UTC,
status, `git diff HEAD --stat`, fontes, owners, spans anteriores e os blocos
selecionados. `pre-validation.json.gz` e `after.json.gz` delimitam a validação;
`evidence.json.gz` arquiva comandos, logs, inventários, receitas e diffs.

O inventário de entrada do lote `simplificacoes-ci` contém 233 diagnósticos
distintos no core lib/testes, dos quais 33 são `items_after_test_module`.
Cada span foi conferido contra a fonte atual antes da edição. A checagem
estrita V1/V5/V15/V26 e a listagem de testes passaram antes da mudança;
os horários e comandos ficam em `baseline-results.json`.

## Medição e classificação

Exemplos do estado anterior preservado em `before.json.gz`:

- `compiler/eval/bindings/field_access/fields.rs:531`: o módulo de testes
  precedia os helpers de classificação e projeção de campos.
- `compiler/layout/page_geometry.rs:53` e `compiler/layout/equation.rs:699`:
  os módulos de teste precediam implementações de Layouter.
- `entities/elements/math_underover.rs:357`: o módulo de teste precedia
  implementações de `map_content_if_changed` dos elementos dessa família.
- `entities/layout_types.rs:1351`: os testes precediam o trait visitor
  e sua implementação. A macro `assert_approx_eq` é local ao bloco de teste
  e se move junto com seus usos, sem alteração de escopo.
- `testing/math_oracle.rs:75`: os testes precediam funções de fórmula.
  O módulo inteiro continua exclusivo da árvore de testes.

A inspeção identificou exatamente um bloco de teste inline por arquivo
selecionado, com atributo `#[cfg(test)]`. Não há macros de include ou de
posição (`line!`/`column!`) nesses arquivos. Esta é organização física de
itens Rust dentro do mesmo owner, não extração de módulo nem mudança de
composição arquitetural. Não modifica o critério de paridade da linguagem
nem constitui uma nova medição contra o vanilla.

## Preservação e linhagem

A operação preserva byte a byte cada bloco de teste, incluindo o comentário
histórico que acompanha o módulo em `compiler/eval/rules.rs`. Prefixo e
sufixo produtivos são recompostos sem alterações internas; só as linhas em
branco entre blocos são normalizadas. `movement-proof.json` contém hashes e
as verificações por arquivo. A comparação final exige exatamente essa
permutação, não apenas contagem igual de linhas.

Os 33 Prompts L0 existentes continuam com o mesmo contrato normativo e
recebem somente atualização de `Hash do Código`. Os headers `@prompt`,
seus hashes, owners, imports, limites de visibilidade e núcleos permanecem.
O resselo foi restrito à lista explícita de consumers. Não foi usada uma
supressão de lint e nenhum teste foi eliminado para remover o aviso.

## Validação final

Medição encerrada em `2026-10-13T23:36:20.677162+00:00`. As fontes e owners
permaneceram inalterados durante os comandos registrados em
`validation-results.json`.

| Verificação | Resultado |
| --- | --- |
| Clippy core lib/testes com `-D warnings` | 233 → 200 diagnósticos distintos; ainda falha |
| `items_after_test_module` | 33 → zero; demais categorias com contagens inalteradas |
| Suíte core | 6.701 passaram, zero falhas, cinco ignorados |
| Registro de testes | 6.706 nomes, idênticos antes e depois |
| Build, formatação e `git diff --check` | passaram |
| Linhagem estrita V1/V5/V15/V26 | passou |
| Linhagem reversa | 965 consumers, zero erros |
| Linter completo | 0 erros, 381 avisos, 1618 notas |

Não há novos testes neste lote: a prova de preservação dos blocos é
acompanhada da execução dos testes existentes e da comparação do registro.
As mudanças não demonstram ganho de desempenho nem correção funcional da
linguagem; removem um bloqueio de organização no gate de qualidade.

## Pendências

Os 200 diagnósticos restantes do core, os avisos arquiteturais, as lacunas
funcionais da auditoria original e a matriz remota de release continuam
abertos. Não houve commit, push, publicação ou execução de benchmark.
