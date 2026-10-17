# Caminhos para corrigir a dupla avaliação

A [auditoria](dupla-avaliacao-auditoria-2026-10-05.md) demonstrou que duas avaliações separadas não conservam, por si, a relação entre elementos originais e elementos gerados por show. A correção precisa resolver essa representação e reduzir execução repetida. Este diagnóstico compara direções de trabalho; não é L0 nem autorização para mudar o pipeline.

## Resultado que a correção deve produzir

O código comum do documento deve ser avaliado uma vez. Transformações show e conteúdo contextual continuam podendo executar trabalho próprio, inclusive em tentativas de estabilização quando suas dependências mudam. Avaliação única do programa não significa uma única passagem de todo o compilador.

Ao transformar um heading em texto, o heading original continua consultável. Se a receita produz novos headings, a introspecção também conhece os novos elementos segundo a identidade, a ordem e a localização observáveis da linguagem. Não escolher uma árvore pela quantidade de resultados nem deduplicar por igualdade de corpo: dois headings com o mesmo texto podem ser elementos distintos.

Preservar o elemento original não implica desenhá-lo duas vezes. Uma realização pode conservar seu registro sem emitir seu conteúdo visual antigo. Código explícito, enum fechado, despacho estático, garantias do Rust e separação de domínio puro permanecem restrições do projeto.

## Alternativas e seus limites

| Direção | O que resolve | Limite e decisão necessária |
| --- | --- | --- |
| Capturar origem durante as transformações atuais | Pode conservar elementos originais e produzidos na mesma execução | Precisa de modelo causal de identidade, estilos, localização e ordem; um coletor de nós sem essas relações não basta. Não foi prototipada |
| Avaliar conteúdo e receitas uma vez e realizar transformações separadamente | Separa a execução do programa da transformação contextual e permite conservar origem no percurso de realização | Exige revisão dos L0, transportes de receita e efeitos; mover aplicação de show de eval para outra fase é gate ADR-0127 |
| Reutilizar uma passagem quando comprovadamente equivalente | Pode reduzir custo de documentos simples como intervenção transitória | Ausência textual de `#show` não é prova: imports, includes, callbacks e lógica ligada à flag podem alterar equivalência. Não corrige por si as divergências de introspecção |
| Memoizar as duas avaliações atuais | Pode evitar algumas recomputações com dependências corretas | Mantém a separação defeituosa entre índices e não resolve as falhas linguísticas encontradas |

**Direção a investigar primeiro:** seguir a relação causal durante a transformação, tomando o vanilla pinado como referência. Comparar captura dentro da fase atual com realização separada antes de escolher a mudança. Adicionar uma fase sem reduzir trabalho ou preservar identidade não é melhoria demonstrada.

## Sequência proposta

1. Ampliar os casos de linguagem: original substituído, elementos produzidos, corpos iguais com identidades distintas, labels, regras aninhadas, estilos locais, imports/includes e conteúdo contextual.
2. Definir quando nasce a identidade de um elemento e como uma receita liga seu original aos resultados. Escolher onde conservar a informação: conteúdo, índice ou transporte entre fases, com ownership claro.
3. Contar execuções reais e leituras de World com um mundo de teste; verificar ordem, erros e efeitos. Arrays finais idênticos não demonstram execução única.
4. Redigir a alteração dos L0 proprietários e resolver a divergência de linhagem apontada na auditoria. Se houver mudança de fase ou contrato público, apresentar a especificação concreta para o gate ADR-0127 antes do código.
5. Materializar a menor mudança coerente, revisar compensações de matrizes e adaptar os consumidores de query e pipeline. Preservar contextos e regras capturados sem depender de uma repetição global do programa.
6. Medir contagem de avaliações, tempo e memória com proveniência, junto da validação linguística em PDF, HTML e bundle. Só depois ampliar a memoização para a unidade bem definida.

## Pesquisa em outros projetos

Pesquisa complementar pode confrontar alternativas, mas outros projetos não definem a semântica Typst. As fontes primárias consultadas em 2026-10-05 sugerem mecanismos distintos:

- [rustc, lowering AST para HIR](https://rustc-dev-guide.rust-lang.org/hir/lowering.html): distingue identidades mapeadas do AST e identidades novas criadas pelo lowering. A possível aplicação ao Tekt é uma inferência: distinguir origem e elemento gerado durante transformação. Não é prova de introspecção ou paginação Typst.
- [Docutils, transforms](https://docutils.sourceforge.io/docs/api/transforms.html): organiza transformações da árvore, referências e processamento contextual em ordem de prioridade. Ajuda a estudar responsabilidades de fases; não demonstra preservação dos originais com as regras show do Typst.
- [Salsa, algoritmo](https://salsa-rs.github.io/salsa/reference/algorithm.html): conserva resultados e dependências para determinar quando reexecutar funções. É referência para uma etapa posterior de memoização; não substitui a representação correta da transformação.

Esses padrões podem ser reexpressos em Rust seguro e código explícito. Sua identificação não é recomendação de adicionar crates nem de copiar as implementações completas. A leitura do vanilla na auditoria permanece a evidência diretamente ligada ao comportamento desejado.

## Uso do Bateia e limites da busca

Atualização após autorização explícita do usuário: as buscas remotas foram executadas no Sourcegraph, GitHub, JOSS e zbMATH. Seus resultados, fontes verificadas e limites estão na [pesquisa autorizada](dupla-avaliacao-pesquisa-bateia-2026-10-05.md). O bloqueio descrito abaixo registra a tentativa anterior à autorização e já não impede a pesquisa.

Foi usado `/tmp/bateia-grid-cells`, SHA-256 `4d5b7ffc4152eeb4fe58a0094ab9fa47bad923eb3286e2dab48c7cef9e21ae9d`, build histórico de Bateia HEAD `9d00e2f58661289a12d67bb9bd40f0027dc6fe2e` com working tree não commitado. Estado observado, hashes de inputs e dossiês estão no [recibo](dupla-avaliacao-auditoria-2026-10-05/bateia-research.json.gz). Nenhuma alteração de código no Bateia foi feita.

A execução local sobre o vanilla, run `261b580ffa412861`, retornou `INCONCLUSIVE`: o catálogo reportou `CATALOG_GAP` para identidade durante transformação, transformação adiada e memoização com invalidação. Isso é limite de verificação da ferramenta, não ausência desses mecanismos no vanilla.

A tentativa live inicial no sandbox, run `6e592deb82082af4`, ficou `PARTIAL` porque a conexão DNS foi bloqueada. A tentativa inicial de permitir rede foi rejeitada pela revisão automática: envio da descrição técnica ao Sourcegraph sem autorização explícita para destino e payload. A descrição é genérica e não inclui código ou nome do Tekt; a confirmação humana foi posteriormente concedida e a busca remota executada. Zero resultados da tentativa bloqueada não significa ausência de candidatos.

Vale usar o Bateia para localizar implementações e preservar recibos, com buscas por mecanismo e leitura posterior da fonte. Nesta versão observada, não deve decidir sozinho qual arquitetura resolve as falhas encontradas. O resultado externo precisa ser validado contra os casos de linguagem do Tekt e o pin do vanilla.
