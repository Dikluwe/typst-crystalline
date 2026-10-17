# Pesquisa Bateia para corrigir a dupla avaliação

O usuário autorizou explicitamente consultas ao Sourcegraph, GitHub e demais provedores do Bateia nesta conversa em 2026-10-05. A pesquisa remota foi executada após essa autorização. O resultado apoia a investigação de uma avaliação única seguida de realização que conserve identidade e dependências. Nenhum candidato externo demonstrou sozinho a semântica de introspecção do Typst; o vanilla pinado permanece a referência direta.

Este é um diagnóstico complementar à [auditoria](dupla-avaliacao-auditoria-2026-10-05.md) e aos [caminhos de correção](dupla-avaliacao-caminhos-2026-10-05.md), sem mudança de código ou L0.

## Proveniência e recibos

O [recibo completo](dupla-avaliacao-auditoria-2026-10-05/bateia-authorized-results.json.gz) preserva IdeaSpec, política, planos, resultados dos provedores, estado rastreado/não rastreado do Tekt, horários e hashes das fontes primárias lidas. HEAD Tekt `06848cdcb5450f2c555626bf116bc7acd82bd82b`, `git diff HEAD --stat` vazio na coleta. As contagens abaixo são resultados de busca, não benchmarks de compilação ou provas de capacidade.

Bateia utilizado: `/tmp/bateia-grid-cells`, SHA-256 `4d5b7ffc4152eeb4fe58a0094ab9fa47bad923eb3286e2dab48c7cef9e21ae9d`, com proveniência histórica e working tree Bateia registrada no recibo. Não foi recompilado nem modificado. Dados brutos locais em `/tmp/bateia-single-eval-2026-10-05/`; os JSON essenciais foram incorporados ao recibo comprimido.

## Cobertura das buscas

| Provedor e execução | Resultado observado | Limite |
| --- | --- | --- |
| Sourcegraph, IdeaSpec completa, run `6e592deb82082af4` em `live-network` | `PARTIAL`: primeira consulta vazia; consulta seguinte atingiu prazo | Frases longas não produziram triagem útil; não concluir ausência de soluções |
| Sourcegraph, consultas curtas | Quatro consultas concluídas: três com resultados e uma com timeout; nove candidatos | Identidades reportadas como `HEAD`, sem pin; resultados são localização inicial de código |
| GitHub, catálogo com consultas curtas | Quatro consultas bem-sucedidas, doze candidatos com commit observado | Muitos candidatos irrelevantes por coincidência lexical; qualidade exige leitura |
| GitHub, nomes e linguagem específicos | Duas consultas, quatro candidatos; inclui `salsa-rs/salsa` e `egraphs-good/egg` | Descoberta não valida comportamento, licença de adaptação ou adequação |
| zbMATH | `EMPTY_OBSERVED` | Não é prova de ausência acadêmica |
| JOSS | Dez registros | Nenhum demonstrou mecanismo aplicável a show/introspecção/paginação; não foi promovido a solução |

O primeiro plano enviado ao catálogo GitHub foi rejeitado por falta de discriminadores. O plano foi corrigido para explicitar os termos específicos de cada consulta; a execução posterior passou. Não houve modificação ou desativação do gate do Bateia.

O catálogo de evidência da execução local anterior continuava sem regras para comprovar os requisitos desta pesquisa. Portanto, não usar a lista de candidatos como recomendação automática nem transformar `INCONCLUSIVE` em ausência de capacidade.

## Mecanismos verificados em fontes primárias

### Sphinx conserva uma árvore processada para resolução posterior

No commit `b04a2101295ac3fb725b16111eda0284b6da4cca`, [environment/__init__.py](https://github.com/sphinx-doc/sphinx/blob/b04a2101295ac3fb725b16111eda0284b6da4cca/sphinx/environment/__init__.py#L650) recupera a árvore serializada em `get_doctree`; `get_and_resolve_doctree` reutiliza a árvore disponível ou a recupera e aplica post-transforms para resolver referências. A leitura cobriu `:650–699`; URL, SHA-256 do arquivo e horário estão no recibo.

**Inferência para Tekt:** guardar um produto da avaliação e realizar trabalho contextual sobre esse produto, evitando repetir o programa inteiro. Não adotar pickle nem tratar o cache de árvore do Sphinx como equivalente à memoização de avaliação Typst. Essa fonte não prova preservação de heading original após show.

### Docutils coordena transformações e trabalho descoberto durante o percurso

No commit `f8602b0fc745405a2262872bebae9f343940adef`, [transforms/__init__.py](https://github.com/docutils/docutils/blob/f8602b0fc745405a2262872bebae9f343940adef/docutils/docutils/transforms/__init__.py#L159) mantém transformações pendentes, aplica prioridades e reordena a fila quando uma transformação acrescenta outras. O trecho lido é `:159–174`.

**Inferência para Tekt:** separar a execução inicial de contribuições produzidas durante realização, preservando causalidade. A ordem por prioridade do Docutils não substitui as regras léxicas e de terminação do Typst.

### Mozilla transporta origem junto do resultado gerado

No commit `1b78c2032d67a995f6f8010836ab62b1250d0842`, [lib/source-node.js](https://github.com/mozilla/source-map/blob/1b78c2032d67a995f6f8010836ab62b1250d0842/lib/source-node.js#L339) percorre fragmentos para produzir código e mapeamentos de origem no mesmo percurso (`:339–378`).

**Inferência para Tekt:** a origem pode acompanhar a transformação em vez de ser reconstruída executando o programa novamente. Um source map conserva coordenadas de fonte, não identidade semântica de elementos; dois elementos gerados no mesmo span continuam precisando ser distinguíveis.

### Salsa valida dependências para reutilizar resultados

O catálogo GitHub identificou `salsa-rs/salsa` no commit `30b614d826d697c47bc0f21591fab218f2004032`. A [documentação primária do algoritmo](https://salsa-rs.github.io/salsa/reference/algorithm.html), consultada em 2026-10-05, descreve armazenamento de resultados, dependências e revisões; a validação permite reutilizar resultados e limitar propagação quando o resultado observado não muda. A documentação é uma leitura atual, não foi comprovadamente vinculada àquele commit pelo catálogo.

**Inferência para Tekt:** após definir uma unidade pura de realização, considerar cache de resultado e dependências de contexto. Não há recomendação de introduzir Salsa ou suas macros; `comemo` já faz parte do projeto. A correção de identidade vem antes da estratégia de memoização.

### Egg resolve um problema diferente

O catálogo identificou `egraphs-good/egg` no commit `73975c9819efb4af2f3c6c1b44fa53f8c5742574`. Seu [tutorial primário](https://docs.rs/egg/latest/egg/tutorials/_01_background/index.html), consultado em 2026-10-05, explica representação de expressões equivalentes em e-graphs e equality saturation.

**Classificação:** isso não demonstra o registro ordenado de elementos originais e gerados exigido pela introspecção Typst. Não é candidato prioritário para esta correção. Equivalência de termos não substitui identidade de dois elementos observáveis.

## Decisão de investigação após a pesquisa

As fontes reforçam três responsabilidades distintas: executar o programa, transformar conteúdo conservando origem e validar dependências para reúso. Elas não exigem `unsafe`, vtables ou geração de código na materialização Tekt.

A direção principal é estudar o mecanismo de realização do vanilla pinado e projetar sua expressão no domínio puro do Tekt: avaliação única produz conteúdo e receitas; realização produz conteúdo visual e registros coerentes de elementos originais e gerados; layout usa o visual enquanto a introspecção consulta os registros. Essa é uma direção de investigação, ainda sem contrato público ou representação concreta escolhidos. A realização contextual pode repetir contribuições dependentes de páginas, sem repetir integralmente o programa comum.

Antes de escolher a alteração, comparar com a opção de conservar a relação causal dentro da fase eval atual. Essa alternativa pode evitar uma mudança de fase, mas precisa resolver os mesmos testes de identidade, ordem, efeitos e contexto. A pesquisa não mediu qual opção custa menos ou é mais fácil de materializar.

Próximo trabalho concreto: especificar identidade e transporte de origem a partir dos casos RED já reproduzidos; contar execução de expressões e efeitos; comparar os modelos; redigir L0 antes de implementar. Se aplicação de show mudar de fase, cumprir ADR-0127 com a especificação concreta. Não introduzir um simples merge das árvores por igualdade nem ressellar a divergência de hash sem verificar sua causa.
