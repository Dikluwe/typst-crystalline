# Separação de medição, quebra, emissão e regiões

A separação foi materializada. O cursor conserva os seus adaptadores e operações de baseline/inline, enquanto a lógica extraída tem owners independentes na camada de layout. Cada arquivo produtivo possui seu Prompt L0 exclusivo; os prompts foram atualizados antes da implementação e resselados somente no escopo desta operação.

## Medição e proveniência

Working tree não commitada sobre `17f13b1df79b87ea065ee62e78164b18e477da3d`. O estado inicial, UTC, lista exata de arquivos alterados, `git diff HEAD --stat` e SHA-256 estão em `separacao-texto-linhas-regioes-2026-09-28/initial.json`. As fontes anteriores estão arquivadas como `.rs.txt` em `before/`; não são consumers produtivos. `validation-state.json` identifica o estado selado, que permaneceu igual durante as verificações finais; `final-receipt.json` e as proveniências das sondas registram os binários e o estado completo. Nenhum arquivo produtivo ou L0 fora de `scope.json` foi alterado nesta operação.

A auditoria anterior identificou responsabilidades misturadas e falhas observáveis. Essa evidência precede a decisão nos prompts. A extração conserva decisões existentes para poder avaliar futuras correções em cada owner. Não há medição de desempenho nem conclusão de superioridade algorítmica nesta rodada.

## Responsabilidades materializadas

| Unidade em `01_core/src/compiler/layout/` | Responsabilidade |
| --- | --- |
| `text_measure.rs` | Fórmula padrão de largura; medição de palavra e espaço; preserva overrides de FontMetrics |
| `break_selection.rs` | Decisão pura Keep/Wrap/Hyphenate, com métricas/estilo/geometria recebidos; não emite nem pagina |
| `word_layout.rs` | Aplica a decisão, emite texto/highlight e avança o cursor; preserva segmentação de scripts |
| `line_commit.rs` | Continua dono da composição, alinhamento, extensões e fechamento da linha |
| `region_progress.rs` | Transição entre colunas/páginas e decisão de progresso pós-linha; delega fitting, notas e montagem aos owners existentes |
| `cursor.rs` | Adaptadores para os novos owners, baseline/inline, RTL e coordenação de markers/drenagem |

O enum de decisão é privado e o despacho é estático. A separação não adiciona estratégia pública, configuração ou escolha de algoritmo no wiring. Ela cria uma fronteira interna onde a seleção pode ser examinada separadamente da emissão e da paginação. A introdução de alternativas e sua seleção pelo wiring continuam sendo trabalho futuro com contrato e medição próprios.

Como indicação estrutural, o cursor passou de 562 para 336 linhas no estado registrado. Essa redução não mede completude da atomização, paridade ou desempenho. Os nove pares de ownership estão em `scope.json` e seus selos em `scoped-seals.json`.

## Validação e limites

- RED→GREEN estrutural: `structural-red.log` refuta a presença da seleção no cursor antes da extração; `structural-green.log` confirma os owners privados e a delegação. `extraction-compile-fix.log` registra os erros de compilação intermediários corrigidos, sem ocultá-los como prova GREEN.
- Domínio: **6.299 passaram, zero falhas, dois ignorados** (`core-tests-final.log`). Os novos testes distinguem limites de wrapping, prefixo Unicode e autoridade das métricas injetadas sem demanda vertical.
- Infraestrutura: **1.122 passaram, zero falhas e zero ignorados**, execução serial incluindo os testes de raster (`infra-tests.log`).
- Builds debug do workspace e release do compilador passaram (`debug-build.log`, `release-build.log`).
- **63/63 entradas da auditoria preservadas** antes/depois, nas duas implementações: mesmas fontes, códigos de saída, texto extraído, páginas e caixas completas de linhas/palavras. Os diagnósticos são iguais após normalizar apenas o diretório das sondas. `behavior-comparison.json`, `compare_behavior.py` e `after/` preservam a prova. São testes de preservação da refatoração; não significam 63 casos de paridade Tekt/vanilla.
- `crystalline-lint` terminou com **zero erros**, **497 avisos** e nenhuma V5/V6/V15/V26 nos owners afetados. O total de avisos preexistentes permaneceu igual. Um aviso V5 de uma cópia histórica em `font-demand-geometry-study/before/line_commit.rs` passou a citar o novo hash L0; não é consumer desta operação. Essa diferença está explicitada em `lint-summary.json`.

O oráculo das sondas continua sendo o vanilla pinado `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, com a mesma Noto Sans versionada e fontes/dimensões explícitas da auditoria. Não se reivindica isolamento universal de fontes.

Continuam abertas as falhas auditadas: propriedades de parágrafo/hifenização descartadas, largura de tracking por codepoints versus clusters emitidos, palavra longa na origem da linha e página terminal vazia. O algoritmo otimizado também não foi acrescentado. Agora medição, seleção e progresso regional podem receber correções próprias sem manter seus algoritmos dentro do cursor.
