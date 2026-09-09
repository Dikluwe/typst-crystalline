# P1322 — anexo C aos canais brutos do transversal projetado

Estado: classificação complementar para julgamento D, sem autoveredito. Este anexo **não altera os nove inputs congelados** em `p1322-classification-freeze.md`, nem executa produto, altera L0 ou reabre corpus. Refina somente a interpretação de seis IDs cuja projeção transversal deu MATCH apesar de canais brutos distintos. Onde o ledger congelado diz `CLOSED_MEASURED_TRANSVERSAL`, ler estritamente **fechado no observável selecionado pelo comparador**, não stdout/stderr completos.

## Medição anterior à decisão

Fonte de dados exclusiva: `00_nucleo/diagnosticos/p1322-transversal-r2.json`, SHA-256 `92be1cf09282fc96354a68deadaed4210fb78aa75b09bb1ef0f488efd6a6a5b3`. A identidade produtiva, HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, diff/stat e binários são os mesmos já pinados nesse recibo e no baseline P1322. Comparar `exit_code`, `stdout` e `stderr` completos dos rows existentes, sem reexecutá-los, mostra:

| ID | Perfis onde MATCH projetado difere nos canais | Células nas três ordens | Delta bruto |
| --- | --- | ---: | --- |
| P1137-S-001 | quatro | 12 | path externo relativo vanilla, absoluto cristalino |
| P1137-E-001 | quatro | 12 | hash próprio do build em `--version` |
| P1137-I-001 | default | 3 | hint concreto de migração de query versus placeholders |
| P1137-C-001 | quatro | 12 | help completo: identidade/editorial, wrapping e metadados de descoberta |
| P1137-X-002 | html e combinado | 6 | warning HTML sem três hints |
| P1138-X-004 | html e combinado | 6 | mesmo warning HTML, pela grafia compile explícita |

Total: 51 células já existentes; **zero adições ao denominador**. Os demais perfis de query continuam classificados separadamente como rejeição íntegra de `--features`; não são testemunhas do hint de query. Casos HTML desligados não são execução do warning habilitado.

## Fonte, intenção, hipótese e refutação por classe

### 1. P1137-S-001 — path no diagnóstico de parser

Normal/default: ambos emitem `error: unclosed delimiter`, linha 1, coluna 9 e o mesmo caret. Vanilla exibe `../../../tmp/p1322-transversal-tya6292a/fixtures/invalid.typ`; cristalino `/tmp/p1322-transversal-tya6292a/fixtures/invalid.typ`. A diferença não foi apagada: está em stderr do recibo.

Fonte causal: `04_wiring/src/main.rs:633–641` tenta remover o prefixo cwd e conserva o path inteiro se não conseguir. Vanilla `lab/typst-original/crates/typst-cli/src/world.rs:156` usa `pathdiff::diff_paths`. L0 proprietário `00_nucleo/prompts/wiring.md:65–71` declara **exatamente** a remoção de prefixo quando possível e fallback ao path de SystemWorld. Esta leitura integral de wiring.md refina a incerteza de intenção registrada antes: há política atual documentada. Não é regressão comparável P1320 nem crédito de paridade do diagnóstico bruto. Compartilha a causa de display externo já registrada para P1138-S-001/002/003; a adição de um ID não multiplica o path causal.

Classificação: `DOCUMENTED_DIAGNOSTIC_PATH_DISPLAY_POLICY`, com equivalência da identidade física demonstrada pela mesma fonte. Refutação: fontes físicas distintas, delta além do spelling ou código que não realize a política L0. Eventual alteração dessa política exige decisão L0 explícita; não selecionar reparo por presumir que todo byte diferente seja defeito ou que toda identidade igual feche um diagnóstico.

### 2. P1137-E-001 — identidade de build

Vanilla: `typst 0.15.1 (e0e8ca4d)`; cristalino: `typst 0.15.1 (d31047d7)`. `02_shell/src/cli.rs:120–127` compõe a versão de paridade e o commit próprio. `00_nucleo/prompts/shell/cli.md:126–140` exige o hash do próprio repositório, não o build vanilla; a definição vanilla está em `typst-cli/src/args.rs:53–57`.

Classificação: identidade/proveniência mecânica documentada, sem dívida de língua. O número de versão coincide; a string sozinha não autentica o binário — os hashes integrais continuam necessários. Refutação: versão de linguagem diferente ou hash que não corresponda ao estado do build atestado.

### 3. P1137-I-001 — hint de query

Default, três ordens: exit 0 e JSON completos são idênticos. Ambos anunciam a deprecação. Vanilla sugere a invocação concreta `typst eval 'query(heading)' --in /tmp/p1322-transversal-tya6292a/fixtures/counter.typ`; cristalino emite `typst eval 'query(...)' --in ...`. Isto é **diagnóstico público aberto**, não equivalência editorial nem feature ausente.

Fonte causal atual: `04_wiring/src/main.rs:512–513` imprime uma constante apesar de ter `QueryIntent` com selector/input. Vanilla `lab/typst-original/crates/typst-cli/src/query.rs:127–160` constrói a expressão de substituição a partir de selector, field, one e input, com escaping de shell; não são placeholders arbitrários. L0 `wiring.md:141–145` exige warning em stderr antes do resultado; `shell/cli.md:545–552` também o exige, mas nenhum deles individualiza aqueles placeholders como divergência intencional. A obrigação genérica de warning não será promovida indevidamente a contradição específica literal de L0: prioridade diagnóstica 3.

Coorte complementar `query-deprecation-hint`: **um path causal**, `CLI.query.warning`. A localização da constante tem um owner atual, wiring. A hipótese mínima é que os dados existentes permitem formar o hint concreto sem mudar a avaliação da query. Entretanto **localizar a constante não prova que toda a correção geral tenha owner único**: `wiring.md:86–98` reserva formatação pública a L2 e composição fina a L4, enquanto o vanilla trata `--one`, `--field`, identificadores não triviais e escaping. O conjunto completo de owners/risco dessa generalização fica Unknown; não se recomenda introduzir lógica semântica de formatter em L4 nem uma nova API L2 sem seu gate. No fragmento mais favorável de um owner/um path, ainda perderia para seis paths da coorte selecionada.

Refutação: dados selector/input irrecuperáveis, necessidade de alterar contrato L2/L4 ou instrução de substituição semanticamente incorreta apesar de texto parecido. Gate futuro: L0 primeiro; paridade interna somente se não houver contrato público novo, caso contrário ADR-0127 com parada. Não certificar toda a query por esta testemunha default.

### 4. P1137-C-001 — help estrutural versus help integral

O inventário de comandos/opções/defaults do comparador coincide, mas o stdout não. `02_shell/src/cli.rs:120–127` usa identidade/about cristalino; `:240–264` fornece docstrings próprias. Vanilla `typst-cli/src/args.rs:49–63` usa help_template, after_help e largura máxima. L0 `shell/cli.md:838–840` **escolhe explicitamente inventário estrutural** e separa identidade, wrapping e texto editorial. Estes fragmentos não são prova de capacidade ausente nem dívida de língua por igualdade de Rust/bytes.

Há também diferenças de descoberta pública que não devem ser escondidas na palavra editorial: o help vanilla mostra alias `c`; o cristalino não o mostra. Fonte: `cli.rs:243` usa `alias = "c"`, enquanto vanilla `args.rs:83` usa `visible_alias = "c"`. O alias existe no parser cristalino; isto não é ausência da capacidade `c`. O help também deixa de mostrar o metadado `TYPST_CERT`, embora `cli.rs:473,606` resolva esse ambiente manualmente. L0 `shell/cli.md:824–837` contrata alias/CLI e `:857–865` o certificado; não foi localizada decisão específica legitimando ou proibindo cada omissão de descoberta do help.

Classificação: fechamento **estrutural selecionado**, identidade/editorial documentadas, com `CLI_HELP_DISCOVERY_METADATA_INTENT_UNRESOLVED` para os metadados omitidos. Não declarar help integral em paridade nem fabricar regressão de comportamento por metadado ausente. Causa de descoberta localizável no owner CLI, um path de help, mas critério normativo e risco completo de uma correção geral permanecem Unknown. Refutação: evidência de que o alias/ambiente não é funcional, divergência estrutural omitida pelo comparador ou decisão L0 literal sobre a visibilidade.

### 5–6. HTML — mesmo warning sem hints

Nos dois IDs, HTML habilitado e combinado, o artefato projetado fecha; stderr não. Vanilla acrescenta os três hints sobre mudança de comportamento, não usar em produção e issue 5512, mais terminação de parágrafo. Cristalino só imprime a headline.

Fonte: `04_wiring/src/main.rs:388–392` emite uma constante de uma linha quando a feature Html está ativa. Vanilla `lab/typst-original/crates/typst/src/lib.rs:246–255` constrói warning e seus três hints. L0 `wiring.md:171–173` exige o warning experimental medido; não há autorização específica para omitir hints, nem cláusula literal suficiente para elevar a dívida a prioridade 2.

Coorte complementar `html-experimental-warning-hints`: prioridade 3, um owner produtivo atual `04_wiring/src/main.rs`, seu único L0 `wiring.md`, **um path causal** `CLI.compile.html.warning`; a grafia legada versus compile explícito não duplica path. Superfície restrita demonstrada: emissão fixa condicionada à feature existente, sem alteração de conteúdo HTML, parsing, target/default ou transporte. Hipótese de suficiência: os três textos fixos e a terminação podem integrar a emissão já contratada no mesmo owner, sem API nova; qualquer necessidade de formatter/API adicional refuta owner único. Gate proposto para esse fragmento: correção interna de paridade, L0-first + RED→GREEN + revalidação. Rank mais favorável `[3,1,-1,1,id]`, inferior ao selecionado `[3,1,-6,1,id]`.

## Efeito na decisão e segregação

A seleção única `namespace-function-missing-field` permanece: nenhuma observação nova de regressão comparável nem contradição L0 específica foi estabelecida aqui. As duas omissões de hints não se fundem pelo fato de ambas estarem em main.rs: warning HTML é constante; query depende semanticamente de argumentos. Não agrupá-las em “warnings L4” para inflar paths ou reduzir owners.

Os números principais 4718 probes/18872 células e as transições P1309 ficam intactos. Este anexo impede que a projeção transversal seja promovida a fechamento de canais completos. Os totais de coortes/eligibilidade do relatório congelado descrevem seu conjunto anterior; este anexo complementa-o sem editar a seleção congelada. Dívida F/S/A e ataques D continuam separados.

L0s lidos integralmente para esta decisão: `wiring.md` (490 linhas) e `shell/cli.md` (998 linhas, leitura já realizada neste turno C), além do Núcleo `wiring/cli-observables.toml`; nenhuma leitura nova de materialization/context. Pins das fontes atuais: wiring.md SHA-256 `4e0661d71303fc367d9f29992809e21ecc3b66fcb6e4869eb45d705f8fe5876d`; main.rs `40a7885a2d4aa6c9a02921461be44928f14e75f072bcda203433c55d1771a6f7`; shell/cli.md `9e44084b7ecf7727c01ad6c4ca3b10e4f22438934f8d8aea6c9f4d2dcc108405`; cli.rs `32ff00e7470caebadbf0cf7acc87aa0c5c14e3a094753475da76e8c884ffb958`; vanilla query.rs `665f64232e6bb9bbf78cf675dc8676aec32b97608612f7dff8696c4bd1b9cbc7`.

Anexo de C sob a skill Tekt: somente fontes/recibos lidos e diagnóstico novo escrito; D julga fonte, regra, limites e efeito no ranking. Sem autoprovação.
