# Fluxo de controle e testes com observação efetiva

O teste local de matriz deixou de afirmar apenas `true`: agora executa o
layouter e verifica células, ordem, alinhamento e gaps. O teste de hierarquia
de espaçamento passou a chamar a função produtiva, com valores normativos
independentes das constantes usadas pela implementação.

## Proveniência e medição anterior à decisão

Branch `Tekt`, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, **working tree
não commitado**. O diretório `controle-fluxo-ci-2026-10-13/` preserva UTC,
fontes, L0, status e `git diff HEAD --stat` em `before.json.gz`,
`pre-validation.json.gz` e `after.json.gz`. Logs, receitas, inventários e
diffs delimitados ficam em `evidence.json.gz`.

O inventário de entrada contém 324 diagnósticos distintos do Clippy no core
lib/testes. Esta etapa inspeciona quatro padrões inalcançáveis, dois limites
de range mutados, uma atribuição sem leitura, quatro assertivas constantes e
dois ranges invertidos. Os spans e mensagens anteriores estão em
`clippy-before-inventory.json`; contagens desduplicam código, mensagem e
spans primários completos.

Antes da edição, os filtros passaram com 378 testes de math layout, 30 de
paragraph (e três ignorados) e três de bidi. `baseline-results.json` conserva
comandos e horários. Esses números não certificam o restante do workspace.

## Classificação e correções

`compiler/layout/paragraph_select.rs:181-193` e
`compiler/layout/paragraph_hyphen.rs:283-309` alteravam `active` dentro de
`active..end`. A faixa Rust captura o início antes do loop; os L0 mandam
manter a tentativa overfull atual e descartar prefixos para finais futuros.
Não foi confirmado um bug na seleção. A edição nomeia esse início como
`first_active`, mantendo a faixa e todas as tentativas existentes.

`compiler/math/layout/assembly.rs:36-41` inicializava ratio duas vezes antes
de qualquer leitura. Foi retirada somente a primeira atribuição; o loop
continua inicializando a variável em cada tentativa, inclusive a final.

`compiler/layout/equation.rs:474` repetia um braço Glyph byte a byte idêntico
ao anterior do mesmo match. Foi retirado o segundo braço. Em
`compiler/math/layout/spacing.rs:122,155,390`, os padrões Equation/Styled no
fallback e Opening/Fence–Fence/Closing já eram cobertos pelos braços
anteriores. Os braços prioritários permanecem, incluindo o zero explícito
que impede fallback de espaçamento.

`entities/bidi_context.rs:111,129` usa ranges invertidos como entradas
inválidas, não como iteradores. Os endpoints foram preservados e expressos
como `Range { start, end }`; as assertivas continuam exigindo rejeição.
Não converter esses casos em ranges crescentes nem removê-los.

## Testes substituídos e guarda preservada

O antigo `matrix::tests::test_matrix_stub` tinha somente `assert!(true)`.
Seu substituto `matrix_cells_preserve_rows_columns_and_gap_changes` usa
métricas sintéticas injetadas, matriz 2×2 com textos distintos, nenhum
delimitador e gaps explícitos. Exige os quatro textos na ordem correta,
baselines comuns por linha, colunas alinhadas à esquerda e separação das
linhas. Aumentar column-gap de 8pt para 15pt deve acrescentar 7pt à largura
e à origem da segunda coluna; aumentar row-gap de 12pt para 23pt deve
acrescentar 11pt à separação das baselines sem alterar a largura.

Essas relações vêm dos gaps explícitos e do contrato da matriz; não foram
calibradas pela saída do candidato. Não constituem nova medição bilateral
de paridade ou cobertura de todas as opções de math.mat.

O teste `relation_domina_thick_maior_que_medium` conserva seu nome e passa a
medir a função `spacing_between` para Relation/Binary/Large: 5/4/3pt em
18pt e o dobro em 36pt, com tolerância de 1e-9. Não usa as constantes da
implementação para fabricar a expectativa.

A assertiva em `registry/microbenchmarks.rs:206` é uma guarda intencional
de perfil: exige release mesmo quando alguém seleciona o runner ignorado
explicitamente. A expressão e a mensagem foram preservadas, com
`expect(clippy::assertions_on_constants)` em um bloco limitado a essa assertiva e com
justificativa. O L0 foi atualizado antes da anotação. A execução isolada
em debug precisa selecionar exatamente um teste e falhar com
`run with cargo test --release` antes de Criterion; isso é um controle
negativo, não falha da suíte normal nem resultado de desempenho.

A primeira anotação estava diretamente no macro `assert!`; o compilador a
ignorou e o Clippy continuou falhando, acrescentando `unused_attributes`.
Essa tentativa não foi aceita. `first-validation.json.gz` preserva suas
fontes e logs. A anotação final envolve um bloco com somente a guarda,
mantendo sua expressão e mensagem.

## Validação final

Medição final encerrada em `2026-10-13T23:24:52.602007+00:00`, com fontes
inalteradas durante os comandos. `validation-results.json` e `summary.json`
preservam os resultados finais.

| Verificação | Resultado |
| --- | --- |
| Clippy core lib/testes com `-D warnings` | 324 → 311 diagnósticos distintos; ainda falha |
| Categorias tratadas | 13 diagnósticos a menos; demais contagens inalteradas |
| Teste novo de matriz selecionado por nome completo | um passou |
| Runner ignorado invocado em debug | um rejeitado com a mensagem exigida, conforme controle negativo |
| Suíte core final | 6.701 passaram, zero falhas, cinco ignorados |
| Registro de testes | 6.706 nomes; somente o placeholder de matriz foi substituído pelo teste efetivo |
| Build, formatação e `git diff --check` | passaram |
| Linhagem estrita V1/V5/V15/V26 | passou |
| Linhagem reversa | 965 consumers, zero erros |
| Linter completo | zero erros, 381 avisos, 1.618 notas |

Dos avisos tratados, um corresponde à guarda intencional de perfil mantida
com expectativa local explícita. Não se declara a correção de 13 bugs de
comportamento nem a eliminação de todas as constantes do código.

`matrix.log` usa o nome completo com `--exact` e exige um teste executado.
A tentativa inicial `matrix-first.log` usou um filtro curto com `--exact`
e executou zero testes; ela não é usada como evidência de validação.

## Pendências

Os demais diagnósticos do Clippy, avisos arquiteturais, lacunas funcionais
da auditoria original e a matriz remota de release permanecem abertos.
Nenhum benchmark de desempenho foi executado nesta etapa. Não houve
commit, push ou publicação.
