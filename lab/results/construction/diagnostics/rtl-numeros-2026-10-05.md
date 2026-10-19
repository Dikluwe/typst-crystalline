# RTL: sufixos, índices de linhas e slot reservado — 2026-10-05

Corrigidos dois defeitos internos na separação de sufixos RTL: referências invalidadas por inserções e posicionamento fora do slot já reservado. O caso longo numerado, contextual e ordinário, voltou a coincidir em texto com o vanilla pinado. Não foram alterados wrapping, shaping, reconciliação, fases ou contratos públicos; nenhum `unsafe` foi acrescentado. Não foi medido ganho de tempo nesta correção.

## Proveniência e medição anterior à decisão

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. [initial.json.gz](rtl-numeros-2026-10-05/initial.json.gz) registra UTC, status, lista exata `git diff HEAD --stat`, fontes de todas as mudanças produtivas/L0 herdadas, fontes das fixtures, comandos, extrações e caixas. Os dois arquivos afetados estavam limpos nesse status; suas fontes iniciais foram recuperadas de HEAD e arquivadas com método explícito em [scope-baseline.json.gz](rtl-numeros-2026-10-05/scope-baseline.json.gz). [execution.json.gz](rtl-numeros-2026-10-05/execution.json.gz) registra fontes finais, hashes, logs e horários. [review.diff](rtl-numeros-2026-10-05/review.diff) isola esta mudança; [scope-review.json](rtl-numeros-2026-10-05/scope-review.json) confirma que as fontes herdadas ficaram intactas.

Baseline release SHA-256 `93e3678323f163c81567ed00b959381bd0e2dd416689421c6500d3671abf20d9`, preservado em `/tmp/tekt-rtl-numeros/baseline-typst`; candidato final `a0caf9bba4692f764b4cb3af3a30555e5b83995f59372775a3bf258419ec1f20`, receita `cargo build --release`, fontes finais no recibo. Vanilla `lab/typst-original/target/release/typst`, pin `9f2b6e8715237cb086899a42873660fe744622e8`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, proveniência ratificada no P1489. `SOURCE_DATE_EPOCH=0`, `RAYON_NUM_THREADS=1` nos controles CLI. String de versão não foi usada como prova de identidade.

A redução bilateral inicial usou 1, 2, 4, 8, 12 e 16 grupos, com números colados ou separados por espaço, fonte DejaVu Sans, largura 500pt e altura 1600pt. Até oito grupos, o texto normalizado coincidia. Com doze colados, o Tekt extraía `0…7,10,8,9,11`, contra `0…11` no vanilla. Os controles com espaço coincidiam. Isso localizou um gatilho; não provou que a causa fosse contextual ou apenas do extrator.

## Duas causas isoladas

1. **Índices posteriores inválidos.** `layout_bidi.rs:411–422,467–486` no baseline percorria linhas e inseria sufixos no vetor da página, atualizando somente os índices da linha corrente. As outras linhas conservavam referências anteriores à inserção. O teste causal com três baselines RTL e linha LTR intermediária falhou: a última palavra permaneceu `عالم2`, sem a separação esperada. O reparo captura os índices elegíveis da página antes de qualquer inserção e processa índices originais em ordem decrescente. Essa é uma escolha de mecânica interna para preservar texto/estilo/baseline, não um requisito de paridade com o algoritmo do vanilla.
2. **Sufixo fora do slot.** Corrigir os índices não resolveu a extração longa: [index-fix-verification.json.gz](rtl-numeros-2026-10-05/index-fix-verification.json.gz). O código colocava o sufixo em `x_original − suffix_width`, mantendo a base em `x_original`, embora o item original já reservasse a largura da palavra inteira. O grupo com `10` começava em x=143,524530, enquanto o sufixo no vanilla começava em x=157,521600; a palavra vizinha terminava em x≈154,025016. A sobreposição era geométrica, não só uma ordem diferente de operadores PDF. O reparo coloca o sufixo no x original do slot e a base em `x_original + suffix_width`. O teste de slot falhou antes dessa mudança.

Os REDs estão em [red-state.json.gz](rtl-numeros-2026-10-05/red-state.json.gz) e [slot-red-state.json.gz](rtl-numeros-2026-10-05/slot-red-state.json.gz). O primeiro conjunto teve um teste falhando e um passando; não afirmar que todos os testes novos reproduziam o defeito. O teste histórico P569 de posição foi atualizado porque fixava o deslocamento incorreto (92,8 em vez de 100 no slot de uma fixture); controles de texto e espaço foram mantidos.

A seção nova do L0 foi escrita antes de cada correção, com medição anterior à decisão. Gate ADR-0127: correções internas e de paridade, fluxo contínuo; nenhuma API pública, flag, fase ou quebra de compatibilidade. Resselo somente do owner `00_nucleo/prompts/infra/layout_bidi.md` e consumer `03_infra/src/layout_bidi.rs`; os pins de Núcleos não mudaram.

## Origem histórica e limite da atribuição

[paridade-producao-p569.md](paridade-producao-p569.md) registra a introdução da separação de sufixos para preservar palavras/espaços em extração RTL. Os controles descritos ali incluíam sufixo, espaço e combinação. Isso confirma a origem da operação e sua intenção documentada. Não reconstitui a árvore exata daquela medição nem prova que estes dois defeitos já existiam no mesmo estado histórico; a prova causal desta rodada é sobre o baseline atual. Não atribuir automaticamente todo o algoritmo à construção por inércia.

## Verificação de língua e geometria

[verify.py](rtl-numeros-2026-10-05/verify.py), [verification.json.gz](rtl-numeros-2026-10-05/verification.json.gz): **18 controles positivos** coincidem com vanilla em texto normalizado e sequência de números após a correção. Incluem as doze reduções, parágrafos de 160 grupos ordinário/contextual, quebras explícitas, linha LTR intermediária, pontuação e LTR puro. Antes, quatro casos divergiam no texto: 12/16/160 colados e 160 contextual. Extrações normalizam whitespace e controles bidi U+202A–U+202E, sem reordenar texto ou remover números.

Não se exige raster idêntico no reparo, pois as posições incorretas devem mudar. Os casos de números separados e LTR puro conservaram bbox baseline/candidato; o recibo contém os hashes dos rasters dos três binários a 36dpi. O script final exige coincidência de texto com vanilla em todos os casos.

[geometry.py](rtl-numeros-2026-10-05/geometry.py) compara caixas de palavras, agrupando as caixas vanilla adjacentes de sufixo/base quando o extrator agrupa esses mesmos caracteres numa única caixa Tekt. A comparação usa texto e baseline e escolhe a ocorrência física mais próxima em x; não exige a mesma fragmentação de itens. Os **17 controles sem a linha LTR intermediária** ficaram abaixo de 0,05pt em todas as quatro coordenadas de cada grupo; maior diferença observada ≈0,01305pt. Nos dois casos longos de 160 grupos, maior diferença ≈0,01187pt. [geometry.json.gz](rtl-numeros-2026-10-05/geometry.json.gz) preserva cada associação e delta, inclusive faltas (nenhuma). Isso verifica as caixas medidas, não garante pixels idênticos ou paridade universal RTL.

### Divergência residual descoberta

No controle `mixed_ltr`, o texto coincide, mas a linha `#text(dir: ltr)[ABC 123]` fica à esquerda no Tekt, contra a direita no vanilla dentro do contexto RTL. `ABC` começa em x=59,523810 no baseline e candidato, contra x=393,425400 no vanilla; delta≈333,90pt. As caixas dessa linha ficaram exatamente iguais entre baseline e candidato. Os grupos RTL do mesmo controle coincidem dentro da tolerância geométrica acima.

O assert geométrico inicial, que exigia paridade também dessa linha LTR, falhou; preservado em [geometry-initial.json.gz](rtl-numeros-2026-10-05/geometry-initial.json.gz) e nos logs do recibo. O verificador final registra o caso residual, exige preservação exata da linha LTR baseline/candidato e continua exigindo geometria dos grupos RTL. Não mascarar a divergência como teste de paridade integral aprovado. Próxima investigação: separar direção de caracteres e âncora/alinhamento do parágrafo, lendo os respectivos L0 e fonte upstream antes de escolher uma fórmula. Nenhuma hipótese causal dessa nova divergência foi materializada nesta rodada.

## Checks finais e decisão

`SOURCE_DATE_EPOCH=0 cargo test -p typst-infra --lib`: **1160 testes passaram**; L1 não mudou e não foi reexecutado. `cargo build`, `cargo build --release` e diff-check passaram. V15/V26 limpos antes do resselo. Linter completo: zero erros, 534 warnings e 1574 notes herdados; nenhum finding novo contra o estado anterior, comparando regra/arquivo/mensagem. Sem V5/V6/V15/V26 no owner tocado; não declarar zero violations global ou ressellar pendências alheias.

Manter ambas as correções: RED→GREEN causal, recuperação da ordem dos números no PDF e caixas próximas ao vanilla nas entradas afetadas. A divergência longa que motivou esta frente foi corrigida nos controles medidos. A linha LTR dentro de contexto RTL permanece como frente própria; a auditoria não está encerrada para toda a linguagem.
