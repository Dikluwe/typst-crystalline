# V16 — helpers de projeção e estilos de eval

Etapa concluída em 2026-10-17T03:57:25.283638+00:00. **V16 233 → 222**, **V19 907 → 918 notas**, V20 **871 notas preservadas**. O gate global continua com exit 1 pelos V16 restantes; o objetivo geral está aberto.

## Correção e equilíbrio V19

Foram fechados onze fallbacks no consumer cfg(test) `01_core/src/compiler/eval/tests.rs`. Os corpos anteriores None/false/no-op/herança, os ramos atendidos, a ordem de busca, short-circuit, estilos, fixtures e asserções permanecem. Cada fallback recebeu o complemento nominal das variantes atuais, forçando nova decisão quando Content ou Value crescer. Nenhum código produtivo de avaliação ou contrato público mudou.

| ID | Linha de entrada | Helper | Complemento |
|---|---:|---|---:|
| E01 | 129 | `eval_for_test_with_registry` (Content) | 117 |
| E02 | 494 | `go` (Content) | 117 |
| E03 | 538 | `go` (Content) | 117 |
| E04 | 566 | `go` (Value) | 37 |
| E05 | 571 | `go` (Content) | 117 |
| E06 | 632 | `find_matrix_or_vec_delim` (Content) | 114 |
| E07 | 645 | `find_op_limits` (Content) | 115 |
| E08 | 771 | `go` (Content) | 116 |
| E09 | 941 | `has_dynamic` (Content) | 117 |
| E10 | 963 | `find_styled_custom` (Content) | 118 |
| E11 | 1098 | `find_custom_in_styled` (Content) | 118 |


As onze notas V19 novas são esperadas e justificadas individualmente no [00_nucleo/prompts/compiler/eval/tests.md](/repos/Antigravity/typst-crystalline/00_nucleo/prompts/compiler/eval/tests.md), após a medição: variantes excluídas compartilham o mesmo resultado daquele helper. Não se alega cobertura de execução de todas as alternativas. Não foram adicionados allow/expect, limiares, bindings capturados ou reescritas em if-let. As notas continuam visíveis; não se alega ganho medido para leitura de IA. [scope.json](scope.json), [stage.patch](stage.patch).

## Validação

Os **1.410 testes de eval passaram antes da alteração**. A sonda estrutural em cópia descartável do workspace, com cfg(test) realmente compilado e uma variante sentinela em cada enum, passou de **0 para 11 E0004 nos matches selecionados**; total **58 → 69**, sem outros erros. [probe-baseline-result.json](probe-baseline-result.json), [probe-green-result.json](probe-green-result.json). Isso é RED→GREEN de exaustividade, sem alegar correção funcional onde o comportamento foi preservado.

Suíte workspace: **8669 passaram, 0 falharam, 7 ignorados**, mesmos totais da etapa anterior. Sem adicionar testes que espelhem a implementação. Build workspace, fmt, Clippy workspace/all-targets com -D warnings e -D unfulfilled_lint_expectations, Rustdoc -D warnings com quatro índices HTML, V1/V5/V15/V26 estrito, linhagem reversa e diff-check passaram. [validation-result.json](validation-result.json), [workspace/workspace-result.json](workspace/workspace-result.json).

O diferencial SARIF confere todos os campos dos demais diagnósticos, inclusive mensagens, níveis, multiplicidades e posições mapeadas: somente onze V16 removidos e onze V19 acrescentados. [architecture-delta.json](architecture-delta.json). O replay confirma que apenas padrões/metadados e os selos do par proprietário mudaram, com enums, assinaturas, asserções e arquivos externos preservados. [guard-result.json](guard-result.json).

A primeira execução do replay falhou porque o rustfmt no diretório temporário não carregou rustfmt.toml do repositório. Repetir a partir da entrada congelada com --config-path explícito reproduziu exatamente o código esperado. O verificador foi corrigido; fontes/L0 não precisaram de reparo. A tentativa e seu consumo estão preservados em guard-repair.json e materialization-resources.json.

## Investigação Source(Table)

O ramo local table_root continua classificando Source como false, enquanto Table/Located/ShowRecipe têm reconhecimento próprio. É uma hipótese de transparência a medir causalmente, não correção certificada por fechar seu wildcard. Foram executadas seis sondas bilaterais de query(heading) através de table/cell show e alias, contra o pin `9f2b6e8715237cb086899a42873660fe744622e8`:

- `plain`: vanilla 1, Tekt 1; campos iguais.
- `show-it`: vanilla 1, Tekt 1; campos iguais.
- `show-hide`: vanilla 0, Tekt 0; campos iguais.
- `show-cell-it`: vanilla 1, Tekt 1; campos iguais.
- `show-cell-hide`: vanilla 0, Tekt 0; campos iguais.
- `alias-show-it`: vanilla 1, Tekt 1; campos iguais.

Fontes, comandos, stdout/stderr, hashes dos binários e UTC estão em [source-table-measurement.json](source-table-measurement.json). A análise conserva divergências e execuções inválidas; os resultados não provam todos os carriers ou paridade global de tabelas. Nenhuma regra de tabela foi alterada nesta etapa. [source-table-analysis.json](source-table-analysis.json).

## RAM, proveniência e continuidade

Comandos pesados sequenciais, **MemoryMax=12 GiB**, **swap zero**, **Cargo um job**. Pico de memória do cgroup **12884901888 bytes (12.000 GiB)**; sem OOM ou OOM kill. Todos os recibos *-resources.json, inclusive a tentativa do replay, foram conservados.

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, branch Tekt, working tree não commitado. [before.json.gz](before.json.gz) e [final-state.json.gz](final-state.json.gz) preservam UTC, hashes, status e lista exata git diff HEAD --stat. O L0 foi atualizado antes do código e só seu par foi ressellado. Fontes/L0/configuração ficaram congelados durante os gates. Linter `12797073972e375d9ab078cf6750c8b30b71e2b9`, SHA-256 `e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2`; oráculo pinado SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, proveniência ratificada em CLAUDE.md. Sem alteração do pin CI ou dependências; sem commit, push, merge ou release.

Restam **222 V16**, incluindo **52** neste consumer. [Lista organizada de pendências](pendencias.md), [inventário integral](remaining-v16.json). As etapas seguintes exigem medição/L0 próprios; nenhuma pendência foi declarada corrigida por este relatório.
