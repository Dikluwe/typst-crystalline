# Correção: preparação e seletores de parágrafo — 2026-09-28

A frente certificada está implementada: `linebreaks` conserva a escolha, e `simple`/`optimized` recebem a mesma preparação do parágrafo. O seletor global compara custos de todas as linhas; a emissão aplica expansão ou redução de espaços e conserva estilos, texto e separadores. A certificação cobre texto latino sem hifenização em largura constante. Não é fechamento do algoritmo completo do Typst nem demonstração de estado da arte.

## Medição e proveniência

Vanilla pinado: `9f2b6e8715237cb086899a42873660fe744622e8`, árvore `6ab8bb9434b1bd4d44e1a10fbf1e407b3b86a1a4`, receita P1489; binário em `lab/typst-original/target/release/typst`, SHA-256 `7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea`. Tekt release final: `c626ab4994885b54a998cdc2ce1de86108b557c009ecf18ac701a8145314cc10`.

HEAD: `17f13b1df79b87ea065ee62e78164b18e477da3d`; working tree **não commitada**, com alterações anteriores preservadas. Data/hora final UTC: `2026-09-28T19:15:27.524708+00:00`. [Recibo final](correcao-seletores-paragrafo-2026-09-28/final-receipt.json) registra estado completo, `git diff HEAD --stat`, arquivos, hashes, binários e logs. [Estado inicial](correcao-seletores-paragrafo-2026-09-28/initial.json), arquivos em before/, proveniências de cada lote e [diff desta rodada](correcao-seletores-paragrafo-2026-09-28/review.diff) permitem reproduzir as medições. São 25 arquivos produtivos/L0 alterados no escopo, sem mudanças inesperadas nesses diretórios. Nenhum commit criado.

A retificação final do L0 corrigiu a contagem do extrator: no caso de dois parágrafos, **ambos já tinham 8 linhas físicas**. O Tekt tinha 10 grupos XML, contra 8 do vanilla; o desvio era a escolha das palavras na primeira parte. Contagem XML não foi usada como critério de linhas físicas. O rebuild após corrigir L0/header preservou os bytes exatos do release medido; [ponte de proveniência](correcao-seletores-paragrafo-2026-09-28/rectification-bridge.json).

## Falha e correção

Antes da decisão, before-pdf mede 32 fontes bilaterais. Eval validava `linebreaks`, mas não o transportava (rules.rs:3070 e native par.rs:131, estado before/). A escolha incremental ficava em text/word_layout. Em 140pt com margens 8pt, simple escolhia `delta epsilon zeta eta`; optimized no vanilla escolhia `delta epsilon zeta` e levava eta à linha seguinte. Em 240pt justificado, optimized colocava eta na primeira linha mediante redução de espaços. Os PDFs, caixas e comandos estão nos lotes before-pdf/after-pdf.

A fonte vanilla confirma a seleção automática em inline/mod.rs:194-196, cálculo de razão em inline/linebreak.rs:570 e custo em :626; os limites de espaço ficam em typst-library/src/model/par.rs:521. Classificação: escolher quebras, preservar conteúdo/estilos e posições é comportamento da língua; buffers, predecessores e algoritmo de poda são mecânica. Nosso seletor usa custos exatos e prefixos cumulativos; não copia os bounds aproximados do vanilla.

O problema também aparecia no L0: os owners anteriores declaravam preparação/optimized como incompletos e proibiam sua implementação no seletor local. **Inferência limitada:** esses contratos legitimavam o adiamento e permitiam validação sem consumo. Não demonstram intenção, preguiça ou por que a prioridade histórica foi escolhida. A interpretação seria refutada por um contrato vigente anterior que já exigisse preparação global/consumo e tivesse sido descumprido. Nesta rodada os owners foram atualizados antes da implementação, com núcleo pinado e resselo individual.

## Peças e composição

- `paragraph_prepare.rs`: certificação completa, palavras, fragmentos de estilo e espaços causais; métricas injetadas; nenhuma emissão.
- `paragraph_select.rs`: seletores privados simple e optimized sobre `Prepared`. O global calcula custo cúbico da razão e custo quadrático da linha, trata redução até 1/3, última linha natural, predecessor e desempate. Prefixos tornam cada candidato O(1); pior caso O(n²), memória O(n). Isto é análise do algoritmo, **não benchmark de desempenho**.
- `paragraph_emit.rs`: aplica o plano, conserva origens dos espaços e estilos efetivos, admite baseline e não escolhe novas quebras durante emissão.
- `paragraph_plan.rs`: coordena estratégia e certificação. `auto` segue justify. Certifica os segmentos entre Parbreak antes de emitir qualquer um. Ao recusar, preserva o percurso existente.

Sequence continua dona dos siblings/preflight; Text continua dona do fallback; Par conserva suas fronteiras. Configuração global pode chegar ao wrapper terminal completo sem promover fragmentos com texto posterior. Estado privado do guard participa de checkpoint/rollback. L4 continua composição: a regra de linguagem auto/justify fica em L1. Não foram criados contratos públicos, interfaces dinâmicas ou um plugin de algoritmos no wiring. Cada módulo possui exatamente um L0 owner; o núcleo paragraph-selection compartilha invariantes.

## Verificação

- Testes RED→GREEN de estratégia, entrada terminal após configuração e vários parágrafos; testes de adjacência/origens, recusa/restauração, overlong, custo final, redução e guard. Logs e refinamentos das fixtures preservados. A primeira fixture FixedMetrics também possuía o mesmo ótimo após a implementação; foi substituída por uma que distingue os algoritmos, sem tomar sua primeira falha como prova de custo correto.
- Core: **6.339 passaram, 2 ignorados**. Infra: **1.126 passaram**. Builds debug/release passaram.
- Lint: **0 erros**, 504 warnings e 1.514 notes. Warnings não equivalem a erros; o relatório não declara ausência deles. O aviso V16 do collect recusando conteúdo via None é conservador: o caller mantém o caminho anterior e o teste comprova ausência de emissão parcial.
- **140 entradas bilaterais, 416 verificações aprovadas**: 33 da frente nova e 107 de regressão. As 31 concordâncias geométricas novas incluem um controle com linebreak, sem certificar suas combinações gerais. Erros inválidos conservam mensagem primária e posição causal; demais gates usam palavras, posições/páginas e estilos observáveis, tolerância 0,0005pt, sem exigir bytes PDF iguais. [Aceitação detalhada](correcao-seletores-paragrafo-2026-09-28/acceptance.json).
- Comparação visual do caso 240pt optimized a 144dpi: recortes RGB coincidiram; inspeção mostrou duas linhas legíveis, primeira terminando em eta e última natural. É QA auxiliar, não definição de paridade mecânica.

## Limites e próxima frente

A preparação recusa hifenização ativa, conteúdo Unicode fora do subconjunto latino, tracking, matemática, smallcaps, objetos, links como unidade inteira, mudanças locais de configuração de parágrafo e largura variável/colunas/células/subframes/floats. Mantém o renderer anterior nesses casos, sem excluir conteúdo. O controle de hifenização e o de colunas permanecem divergentes; o controle escolhido de linebreak coincide, mas não fecha interações arbitrárias de quebras obrigatórias.

Continuam abertos: Par aninhado e seu diagnóstico de admissão; show par recursivo; viúvas/órfãs e página onde o parágrafo começa; limites customizados e justificação CJK. A próxima frente é incluir os candidatos de **hifenização na preparação global**, usando a política lexical já corrigida, e comparar os dois seletores sobre os mesmos candidatos. O conjunto certificado desta rodada fica fechado; o algoritmo completo permanece aberto.
