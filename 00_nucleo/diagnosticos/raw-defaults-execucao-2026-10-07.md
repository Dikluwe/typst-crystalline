# Defaults Raw — execução de paridade 2026-10-07

A autorização `Pode prosseguir` foi aplicada à pendência de fonte/tamanho Raw do D08B3-A. Correção interna de paridade em fluxo contínuo ADR-0127: L0 primeiro, RED→GREEN e revalidação. Não há campo, trait, assinatura pública ou fase nova. Fontes e comprimentos físicos continuam resolvidos no render; show transporta uma fronteira de estilo pelo Styles existente.

## Medição anterior à decisão e resultado

A fonte pinada `lab/typst-original/crates/typst-library/src/text/raw.rs:658–670` instala fonte DejaVu Sans Mono e tamanho0.8em antes dos overrides show-set. As 16 sondas de `raw-defaults-2026-10-07/before.json.gz` mediram Serif/tamanho exterior no Tekt, perda de cor herdada, tamanho em zero e perda dos defaults no produto substituído por show. O teste P1055 de tamanho100% era uma expectativa incorreta para este pin; foi corrigido conforme a medição atual, sem apagar seu histórico.

O owner Raw agora define Mono80%, fill herdado e restauração do exterior. Apply transporta `raw.defaults` exterior aos overrides apenas quando Raw é transformado; Styled instala os defaults no render e resolve comprimento em contra o tamanho anterior nesse escopo. Raw devolvido por `it` conserva a aplicação única; não há caso especial de fonte para Boolean. Átomos true/false `typc` mantêm a cor constant.language da paleta Typst Light quando a bundle não possui gramática para esse token. Outros highlights e a emissão atual de whitespace não foram reescritos.

Testes RED: 0/2, por tamanho e família incorretos, em red.log/red-state. O primeiro GREEN teve1/2: text.fill reside no canal custom que StyleChain.fill não lê; o helper Raw passou a consultar esse canal, sem alterar a resolução global de StyleChain. green-second.log registra2/2. A validação dos pins detectou cabeçalho não reconhecido e referências não ordenadas; lint-final-sorted.sarif registra o grafo corrigido, sem novos erros ou warnings em relação à base. Os arquivos intermediários não são certificados finais.

Família, tamanho e cor coincidem nas16 sondas: inline, Boolean, tamanho/fonte globais, show-set absoluto/em/fonte/combinado, bold/italic, fill, substituição, identidade, composição show, bloco e Boolean em tabela. As listas de palavras e o texto extraído também coincidem; isso não prova geometria universal. O controle Boolean de tabela tem delta máximo bbox0.001793pt; os demais controles D08B3-A de nested/filtro/aliases têm deltas até0.00004pt.

## Limites numéricos que permanecem abertos

Raw em bloco ainda diverge no espaçamento vertical: na recoleta, Before yMin68.27018, abc85.806243 e After95.064120 no Tekt, versus68.270160,91.304203 e108.594100 no vanilla. A maior diferença é13.52998pt. Antes da correção tipográfica já havia diferença: abc82.658180/91.304203 e After97.046180/108.594100. A tipografia corrigida não legitima fechar a fronteira de bloco; ela exige medição/contrato de espaçamento próprio.

Nas sondas inline há resíduos de bbox até0.14303pt sob bold/italic e0.015041pt sob fonte Sans; família/tamanho/cor corretos não demonstram igualdade desses avanços. Não introduzir tolerância posterior para converter esses resíduos em fechamento. Whitespace de Raw, gramáticas customizadas, Raw novo reconstruído em show e defaults HTML não foram certificados. D08B3-B mantém Cell isolada/reconstrução fresca; igualdade genérica de Table e três negativos D06 continuam abertos.

## Proveniência e preservação

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree não commitado. before-state, red-state, tested-second e tested-final guardam UTC, diffstat, status, conteúdo e hashes exatos. Coleções guardam fontes, comandos, cwd, status, texto, famílias/tamanhos/cores, bboxes e SHA dos PDFs/binários congelados. NO_COLOR=1, SOURCE_DATE_EPOCH=0, RAYON_NUM_THREADS=1. O oracle pinado é `9f2b6e8715237cb086899a42873660fe744622e8`, binário SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`, árvore/receita P1489.

Os resultados, logs finais e hashes estão em `raw-defaults-2026-10-07/final-results.json`. O escopo tem quatro pares owner/consumer e um núcleo pinado, nove arquivos de implementação/L0; zero alterações de implementação fora desse conjunto em relação ao estado inicial. implementation.patch usa essa base, preservando o trabalho anterior. Sem commit, reset, reseal global ou atualização global de interfaces.

## Fecho do recorte tipográfico

Binário final SHA-256 `43207349fba51afbdbcac3847c1a7d884e9ff845b6401022be22272679c00575`. A reconstrução release após reparo dos headers/pins produz exatamente os mesmos bytes do executável congelado usado nas coletas: a medição não foi atribuída a outro binário. tested-second documenta o estado medido; tested-final documenta a linhagem canônica final. release-stable.log e final-results.json registram essa prova.

6602 core +1194 infra +90 shell +10 integração query passaram (5 ignorados core); builds debug/release passaram. Linter final:0 erros,544 warnings,1621 notes; avisos não equivalem a zero ocorrências. Presença92/92, selector23/23, D05 29/29, D07 29/29+14/14, query36/36 e D06 38/41 preservados. B3 60/62 e refutadores16/18 mantêm somente os negativos nomeados na etapa anterior. O recorte de família/tamanho/cor/overrides está corrigido; os limites geométricos numericamente registrados acima permanecem abertos.

## Atualização posterior — fronteira de Raw bloco

raw-block-spacing-execucao-2026-10-07.md supersede o negativo de13.52998pt
de Raw bloco registrado acima. Os25 controles Raw dessa etapa coincidem
até0.00005pt vertical. O texto acima conserva a proveniência histórica da
medição, não representa o renderer corrente. Sizes-down de parágrafo anterior
e demais resíduos/limites explícitos continuam abertos.
