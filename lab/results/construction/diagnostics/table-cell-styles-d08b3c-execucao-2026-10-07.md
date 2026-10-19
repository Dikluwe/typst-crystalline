# D08B3-C — defaults declarativos de table.cell, 2026-10-07

Implementado no subconjunto medido: set table.cell por identidade nativa/aliases simples, condição e spreads; propriedades privadas preparadas na inserção; precedência de constructor/auto/none; defaults antes da ocupação e spans 1 em padding; inset parcial absoluto, fold de alinhamento com Table e stroke por componentes. Args são avaliados pelo owner existente antes dos casts; Styles recebe somente valores declarativos. Fonte lexical permanece intacta. Aliases guardados que entram em Table não repetem sua regra.

Não houve novo contrato público, fase, flag ou modo de produto. Fluxo contínuo ADR-0127: L0 primeiro, RED→GREEN e resselo restrito. Os dois campos novos (properties no carrier já opaco e stroke_inherited na struct privada) e seus helpers são internos à crate; assinaturas/fields públicos permanecem. Footprint final: 13 owners 1:1 e um núcleo novo cell-style-defaults, com pins completos; V5/V6/V15/V26 sem erros. Nenhum commit/reset/cleanup/resselo global.

## Precisões decididas depois da medição

- Constructor stroke preserva seus componentes no getter avulso; a resolução posicionada combina base Cell capturada e Table. Dois sets e dois show-sets de stroke acumulam componentes. None explícito vence. Alignment explícito vence o estilo Cell e faz fold por eixo com Table.
- Spans observados por show-set de Cell podem mudar depois da ocupação: colspan 2 no getter, sem mover os slots já resolvidos. Padding tem spans explícitos 1. Projeção privada de getters permanece distinta do plano físico.
- Fill/stroke pertencem ao plano da Table e sobrevivem quando a Cell devolve it.body ou substitui o texto. A generalização anterior de sobrevivência do default show estava errada para tinta: a medição ab só demonstrava espaçamento. Inset/alignment do default continuam condicionados à sobrevivência; tinta usa o plano externo, inclusive None conhecido.
- A soma Length+Color nas duas ordens marca paint/thickness como fornecidos. A ausência dessas marcas substituía red+2pt por black 1pt durante o fold. Outros braços/casts ricos não foram ampliados.

## Proveniência

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. Diretório `table-cell-styles-d08b3c-2026-10-07/`: before-state, estados RED, tested-final e final-state guardam UTC, conteúdos/hashes e lista exata dos arquivos alterados com git diff HEAD --stat. Cada coleta conserva fonte, argv/cwd, ambiente de execução, status/stdout/stderr completos e SHA-256 dos executáveis congelados. validation.json registra comandos, UTC/cwd e flags de compilação; final-results.json e verify-final.py vinculam as coletas à fonte final, verificam selos e preservação. implementation.patch compara com o estado inicial, usando HEAD somente para arquivos então limpos.

Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`; árvore/receita P1489; binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Tekt release final `23eb4227d17b332af08414506b81c685c63cfa440d18b18e0c2ec69fd23c675c`; debug `36de4629ebe213a25e584e9419a2f2756b74b19308cf540afcb51d137193101e`. --version não serve de identidade.

## Validação final — rodada verified

- after-verified: 58/58 saídas completas iguais. extra-verified 7/7; show-fold-verified 2/2; additional-verified 8/8. isolated-verified 51/53: os dois controles restantes têm apenas warnings antigos de arquivo vazio (raw-fields/raw-x). Cell-set agora passa.
- refuters-verified: 10/12 saídas completas iguais. header-y-style-conflict tem a mensagem/hint corretos mas falta o caret do argumento cru; local-prepared-inside-table mantém cores corretas e efeitos, porém body é sequence metadata+Cell no Tekt contra styled Cell no vanilla. Ambos já eram negativos em refuters-before; não foram normalizados nem declarados fechados.
- debug-depth-verified: 10/10 erros completos iguais, sem abortar. Os 13 testes novos passaram. REDs de defaults, spans, folds, tinta e literal foram registrados antes de cada correção. O erro de compilação do primeiro teste de borda (RGB float em API byte) e a correção do espaço esperado no teste inicial estão separados de RED semântico; não foram usados como prova de defeito.
- geometry-verified: 20 controles com texto/lista de palavras iguais e delta vertical máximo 0.00002pt. 19 têm tamanho/família/cor iguais; o controle set text(red) posicional conserva a divergência de cor anterior. Stroke red 2pt foi confirmado na saída vetorial nos dois controles. Physical-verified: quatro controles têm texto/fonte, cores RGB8 e espessuras iguais; fundo red permanece em it.body/substituição e borda red 2pt permanece em it.body. Não comparar bytes ou número de comandos PDF/SVG para fechar render.
- Não fechar métricas horizontais: alinhamento combinado tem delta 5pt e inset tem 0.005pt. geometry-before-controls usa o release B pinado 721b7c2a e reproduz os mesmos deltas com constructors explícitos. Na fonte final, set e constructor explícito produzem posições idênticas para os dois controles. Resíduos inline de Auto/Raw também permanecem.
- Regressões: presença 92/92, selector 23/23, Table B3-A 62/62, Equation 29/29, matriz 29/29 +14/14, query 36/36. Math conserva três negativos (38/41); Table refutadores conservam igualdade/defaults gerais e diferença de bytes tbody (DOM equivalente medido anteriormente). Raw defaults/bloco conservam os resultados e resíduos anteriores, incluindo sizes-down 10.8pt. Verificador não encontrou nova diferença de paridade nessas coletas.
- cargo test --workspace --lib: 6.626 core passaram, cinco ignorados; 1.195 infra e 90 shell passaram. CLI query: 10/10. Total: 7.921 testes passaram nesta validação. Builds debug/release passaram. Linter: zero erros, 546 warnings, 1.635 notes; não são zero ocorrências globais.
- Nenhuma mudança produtiva/L0 fora dos 13 owners e núcleo declarado. Documentação de processo registra a decisão e as limitações; não legitima código.

A fixture evaluation-once junta Content com Color e é inválida no vanilla: serve apenas como controle de erro. A fixture com metadata descartada não mede contagem de chamadas. A avaliação única decorre da delegação a eval_args e do uso posterior de valores sem Func/AST; não foi inventado um número de efeitos a partir dessas fixtures.

## Próximo e limites

Prioridade: medir/especificar origem e forma dos argumentos da Table para o caret e body local (D08B3-C2), antes de implementar, classificando o gate ADR-0127 conforme o contrato concreto. Depois, métricas horizontais já reproduzidas no baseline. Permanecem fora da certificação: aliases Func.with/targets arbitrários, funções/gradient/tiling/strokes por lado, inset percentuais/em e rich casts, grupos/spans multipágina, PDF/UA, defaults/igualdade gerais de Table e limites de Raw/math/set-text já registrados. Este relatório fecha o subconjunto validado de defaults e tinta, não o universo Typst/Cell nem um percentual global de paridade.
