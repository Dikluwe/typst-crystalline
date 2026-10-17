# Simplificações de cópias, conversões e assertivas

Este lote remove expressões redundantes apontadas pelo Clippy e reforça o
caso de Clone de `Axes`. Não altera contratos públicos, flags, defaults ou
fases do pipeline. Não há conclusão de desempenho ou nova prova de paridade.

## Proveniência e medição anterior à decisão

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**. O diretório `simplificacoes-ci-2026-10-13/` preserva fontes,
L0, UTC, status e `git diff HEAD --stat` em `before.json.gz`,
`pre-validation.json.gz` e `after.json.gz`. Logs, receitas, sugestões
inspecionadas, inventários e diffs delimitados ficam em `evidence.json.gz`.

O inventário de entrada, proveniente do lote `controle-fluxo-ci`, contém
311 diagnósticos distintos do core lib/testes. Os spans tratados foram
conferidos contra as linhas atuais antes de editar. Foram selecionados
28 casos de comparação booleana em assertivas, 23 de clone em Copy, sete
casts para o próprio tipo, sete conversões para o próprio tipo e 13
matches redundantes de Result. Código, mensagem e spans primários completos
formam a chave de desduplicação; os diagnósticos de lib e testes não são
simplesmente somados.

O filtro de `entities::axes` passou antes da edição. A baseline começou em
`2026-10-13T23:27:26.675409+00:00` e terminou em
`2026-10-13T23:27:27.080964+00:00`; os comandos e resultados estão em
`baseline-results.json`. O registro anterior de nomes está em
`test-list-before.log`.

## Classificação e mudanças

Os casts de duração em `compiler/eval/repr/measure.rs` eram i128 para i128;
os de columns e footnote eram usize para usize. Removê-los conserva o tipo
e o valor. Em strings, foram removidas conversões de identidade após
replace/repeat/upper/lower, na projeção de símbolo e no nome de fonte. As
operações que produzem os textos permanecem.

As definições em `entities/compiler_features.rs:15`,
`entities/layout_types.rs:1208` e `entities/lang.rs:23` foram inspecionadas.
Os clones produtivos tratados operavam sobre `Features`, `Length`, `Lang`
e suas opções. Os tipos têm Clone e Copy derivados, sem Clone customizado;
as definições usadas nessa decisão foram preservadas como fontes somente
de leitura. A cópia conserva os campos e não introduz novo compartilhamento.
Nenhuma economia de tempo ou alocação foi medida ou reivindicada.

As assertivas booleanas passam a usar `assert!(expressão)` ou
`assert!(!expressão)`, mantendo a polaridade e as mensagens. As expressões
continuam avaliadas uma vez. Os 13 casos de fixpoint passam de
`matches!(result, Ok(_))` a `result.is_ok()`; a inspeção e extração posterior
do resultado permanecem nos testes.

O teste `entities::axes::tests::axes_copy_clone` conserva o caso Copy de
`Axes<f64>` e passa a exercitar Clone com `Axes<String>`. Modificar o campo
x do clone deve preservar x e y do original e y do clone. Assim, a retirada
do clone redundante de f64 não elimina a cobertura de Clone. A implementação
derivada de `Axes` não foi modificada; não foi encontrado um bug nela.

O L0 `00_nucleo/prompts/entities/axes.md` recebeu a aceitação do teste antes
da edição. Nos outros 23 owners, o único conteúdo L0 alterado é o Hash do
Código. O resselo é restrito aos 24 consumers nomeados, preservando a
cardinalidade e o grafo de núcleos. `seals.json` e `scope-diffs.json` registram
as mudanças. Não foram adicionadas supressões de lint neste lote.

## Validação final

Medição encerrada em `2026-10-13T23:32:09.110435+00:00`, com fontes e L0
inalterados durante a validação. `validation-results.json` registra cada
comando, horário e código de saída.

| Verificação | Resultado |
| --- | --- |
| Clippy core lib/testes com `-D warnings` | 311 → 233 diagnósticos distintos; ainda falha |
| Categorias tratadas | 78 diagnósticos removidos; demais contagens inalteradas |
| Filtro de Axes | três passaram |
| Suíte core | 6.701 passaram, zero falhas, cinco ignorados |
| Registro de testes | 6.706 nomes, idênticos antes e depois |
| Build, formatação e `git diff --check` | passaram |
| Linhagem estrita V1/V5/V15/V26 | passou |
| Linhagem reversa | 965 consumers, zero erros |
| Linter completo | 0 erros, 381 avisos, 1618 notas |

As 78 ocorrências são redundâncias e melhoria de cobertura, não 78 bugs
comportamentais demonstrados. A suíte do core não certifica toda a matriz
multiplataforma de release.

## Pendências

Continuam abertos os demais diagnósticos do Clippy, os avisos arquiteturais,
as lacunas funcionais da auditoria original e a matriz remota de release.
Este lote não executou benchmark nem nova comparação bilateral com o vanilla.
Não houve commit, push ou publicação.
