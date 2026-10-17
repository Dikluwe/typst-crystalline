# Escopos de estilo: transferência do backup exterior — 2026-10-10

## Medição antes da decisão

A coleta nova `stacks-current.json.gz` usou o binário anterior, interposer limitado a frames do executável e amostragem periódica de 1/64 pedidos glibc. Há clones de TextStyle dentro de Styled; em local-set há também reconstrução de TextStyle e StyleDelta. São amostras inclusivas, possivelmente sujeitas a alias periódico; não são custos exclusivos nem uma medida de tempo. A fonte `01_core/src/compiler/layout/style_scope.rs:37,130,148` antes da alteração clonava o exterior para backup; FontList/variations podem alocar (`layout_types.rs:199,265`). Esta observação motivou o experimento, não uma alegação sobre a intenção do vanilla.

## Experimento rejeitado e restauração

No candidato somente `01_core/src/compiler/layout/style_scope.rs` mudou como consumer produtivo. Styled, Strong e Emph passam o exterior para o backup por `mem::replace`, depois de resolver o estilo interior. Raw defaults e tamanho relativo ainda leem o exterior antes da transferência. Permanecem chain lexical, tracking, paragrafação, idioma, Raw e restauração integral. L0 atualizado primeiro e selado somente neste owner; fluxo interno ADR-0127, sem contrato público novo.

O teste local `lexical_scopes_restore_owned_font_buffers` foi RED: a igualdade do estilo passou, mas a identidade do buffer exterior falhou. GREEN cobre os três escopos. Identidade do buffer é critério mecânico desta otimização, separado da aceitação linguística. Esta é a redução comprovada: remover clones para backup. Não afirmar redução geral de reconstruções, históricos ou captura lexical.

## Alocações, tempo e RSS

Alocações: três execuções por caso com interposer; o gate exige contagem estritamente menor. Tempo e RSS: sem interposer, três rodadas, quatro pares medidos por caso por rodada, warm-up excluído, ordem embaralhada, uma CPU fixa, Rayon/jobs=1, stack64MiB. Os caminhos before/after têm basename e comprimento iguais. Testes/build/coletas de saída terminaram antes da medição de tempo. A contagem glibc é de chamadas acumuladas, não memória retida. Não usar os tempos do gate com interposer para concluir latência.

| Caso | Alocações antes → depois | Tempo mediano ms | Mediana da diferença pareada | RSS mediano KiB | Diferença RSS pareada |
|---|---:|---:|---:|---:|---:|
| local-set-400 | 154,052 → 153,650 (-402) | 111.95 → 112.27 | -0.16% | 31038 → 30986 | -0.19% |
| local-set-1600 | 328,505 → 326,903 (-1,602) | 147.37 → 148.95 | +1.98% | 42184 → 42104 | -0.27% |
| no-fill-1600 | 253,234 → 253,232 (-2) | 132.24 → 132.18 | +0.47% | 38116 → 37996 | -0.34% |
| inline-set-1600 | 459,588 → 457,986 (-1,602) | 172.13 → 174.57 | +0.31% | 47742 → 47662 | -0.21% |
| variants-1600 | 1,121,236 → 1,118,034 (-3,202) | 793.04 → 795.27 | +0.36% | 293004 → 292914 | -0.03% |
| families-1600 | 391,060 → 387,858 (-3,202) | 150.83 → 150.07 | +0.16% | 38542 → 38340 | -0.43% |

Diferença de tempo pareada por rodada:

- `local-set-400`: -0.62%, -0.77%, +1.55%
- `local-set-1600`: +1.32%, +1.17%, +3.32%
- `no-fill-1600`: +1.12%, +1.16%, -1.32%
- `inline-set-1600`: +3.09%, +0.23%, -0.58%
- `variants-1600`: +0.81%, +0.40%, -0.19%
- `families-1600`: -0.33%, -0.09%, +0.98%

A melhoria de alocações é determinística nestas coletas. Os tempos e o RSS devem ser interpretados pelas rodadas acima: percentuais pequenos ou sinais mistos não demonstram aceleração persistente, redução residente ou maior homogeneidade. Não usar contagem de alocações como substituto para essas alegações. A hipótese foi rejeitada: a repetição confirmou regressão local e inline. Código e L0 foram restaurados exatamente aos bytes anteriores. Não fica alteração produtiva deste experimento; não fecha o custo residual dos estilos locais.

## Revalidação

- No candidato, 6.681 testes L1 passaram; 5 ignorados. Teste local RED→GREEN preservado em `owner-diff.patch`; não foi incorporado ao código final restaurado.
- 25 tabelas e 8 controles de escopos (aninhamento, fontes, Raw defaults/override/substituição, tracking, paragrafação e vazios): texto, palavras, dimensões/páginas, bbox e raster36dpi idênticos entre before/after. Receipts `qualification-tables.json.gz`, `qualification-scopes.json.gz`. Certifica preservação deste conjunto, não paridade total com vanilla.
- `cargo build`, `cargo build --release` e `crystalline-lint .` saíram0. Há avisos preexistentes, inclusive V5 de outros owners; nenhum V15/V26 e nenhum drift deste owner. Não declarar zero avisos global.
- Não alterei os caches de fontes, PDF, enum Content, captura lexical ou interfaces.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Lista exata de arquivos e diffstat em `before.json.gz`, nas medições e `final-validation.json.gz`; inventário de fontes/fontes Rust e bytes anteriores do owner incluídos. O candidato mudou apenas o consumer citado; após rejeição, todos os consumers do inventário e os dois arquivos source/L0 voltaram aos hashes anteriores.

- UTC do benchmark: `2026-10-10T04:28:47.613024+00:00` → `2026-10-10T04:29:50.179899+00:00`.
- Tekt anterior SHA256 `bb138eb8d2f8a5762ed693bfe2dda526c092780a47e0dc4c5e8e4f7af35b81a8`.
- Tekt candidato rejeitado SHA256 `a133ee4e206ec417998138a4b6a5871ebe1ccffb1426b24f8e28defeeefea53d`.
- Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`; receita/tree histórica em P1489.
- Comandos, fixtures, fontes, ambiente, CPU, scripts e hashes em `measurements-equal-argv.json.gz`; contagens em `allocation-equal-{red,green}.json.gz`; fontes antes, diff restrito e seals em `before.json.gz`, `owner-diff.patch`, `seal.json`. Logs completos do candidato em `candidate-validation.json.gz`; estado restaurado em `final-validation.json.gz` e `restoration.json`.

A linha variants é um controle do custo deste Tekt, não prova de trabalho equivalente ao vanilla. Divergências anteriores de renderização/paridade permanecem abertas.

## Repetição e decisão final

`measurements-repeat.json.gz`: UTC `2026-10-10T04:30:19.061537+00:00` → `2026-10-10T04:30:44.859230+00:00`, três rodadas de oito pares por caso, warm-up excluído, mesmos controles de afinidade/argv/ambiente e sem interposer. A coleta inicial indicou +1,98% local com todas as três rodadas positivas; a repetição foi feita para resolver esse risco concreto.

| Caso | Mediana de tempo pareada | Rodadas |
|---|---:|---|
| local-set-1600 | +1.84% | +1.34%, +1.95%, +1.80% |
| no-fill-1600 | -0.13% | -0.33%, +0.05%, -0.12% |
| inline-set-1600 | +1.64% | +2.55%, +0.12%, +1.64% |

O controle no-fill permanece próximo de zero; local e inline pioram novamente. Isso sustenta rejeitar este candidato, não identifica a causa exata da regressão (codegen, layout de memória ou movimentação continuam hipóteses). A redução de chamadas glibc não é prova de latência menor. O binário candidato permanece na cópia de reprodução; o executável de trabalho deve voltar ao baseline validado.

## Próximo candidato

Medir o custo de `StyleChain::font` (`01_core/src/entities/style_chain.rs:633–654`): `compiler/eval/rules/set/text.rs:269` persiste o caminho set como Array custom; quando a propriedade está no canal `text.font` como Array, cada resolução reconstrói FontFamily/FontList e normaliza nomes. A coleta contém reconstruções de TextStyle dentro de Styled, mas não isola esta conversão. O próximo experimento deve contar as conversões/alocações e preservar precedência, arrays inválidos, herança, regex/fontes tipadas e restauração antes de escolher cache ou representação. Nenhuma dessas alternativas foi implementada ou ratificada neste passo.

## Confirmação da restauração

`final-validation.json.gz` confirma arquivos source/L0 e inventário produtivo idênticos ao baseline. Builds dev/release e lint saíram0 novamente; os avisos preexistentes permanecem. O executável restaurado tem SHA256 `bb138eb8d2f8a5762ed693bfe2dda526c092780a47e0dc4c5e8e4f7af35b81a8`, idêntico ao binário anterior do experimento. Assim, o resultado final mantém a melhoria de fontes anterior e adiciona somente o diagnóstico/artefatos desta hipótese rejeitada.
