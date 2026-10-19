# Relatório — fronteira Par/Box e paginação finita

Etapa concluída. Corrigida a transição de parágrafos/blocos para caixas inline não fracionárias e a admissão da linha inteira em páginas finitas ordinárias. **Nenhuma API pública mudou.** O gate global continua aberto: **125 V16**, **1065 notas V19** e **867 notas V20**, integralmente preservados no diferencial. Esta etapa trata falhas funcionais encontradas na validação do lote anterior; não contabiliza redução fictícia de V16. Não houve commit/push do Typst, merge ou release.

## Correções

Três consumers e seus L0 proprietários mudaram: boxed.rs, paragraph.rs e line_commit.rs. L0 primeiro, correção interna/de paridade ADR-0127, com resselo restrito dos três pares; dependências/núcleos e contratos públicos foram preservados.

- Box com dimensão não fracionária usa a fronteira causal e colapsa a margem de parágrafo com a margem pendente. Margens fortes de Block permanecem fortes. A própria caixa fornece seu ascent; fontes não inventam um strut para sua geometria.
- O helper privado de espaçamento recebe o tamanho ativo; o helper lexical anterior delega a ele e conserva seus consumidores. Valores relativos e restauração de escopos permanecem.
- O fechamento ordinário compara o fundo real da linha e o ajuste tardio de ascent antes de comitar. A linha inteira, seus wrappers e posições internas migram juntos para a página correta; avanço horizontal e indentação acompanham a margem da nova página. A baseline usa o ascent real uma vez, inclusive quando texto maior vem depois da caixa.
- Primeiro frame maior que a região é comitado uma vez; não cria página terminal vazia. A testemunha de unidade anterior pertence à região vigente: altura herdada de uma página anterior não provoca migração adicional depois de pagebreak.

A primeira caixa não fracionária de uma nova raiz finita ordinária vazia recupera o topo geométrico, em vez de herdar strut/margem da página anterior. As rotas de regiões reservadas, colunas, floats/transações e collector externo de decoração conservam o protocolo anterior. Elas não receberam certificação universal neste lote. Box com largura `1fr` conserva a composição B explicitamente incompleta e sua testemunha divergente.

## Validação

**17 testes novos passaram.** O primeiro RED teve 14 falhas e um controle aprovado. A contagem bruta de carriers de um teste incluía Text vazio: o teste foi refinado para conteúdo visível e geometria, sem atribuir esse erro de fixture a uma falha do produto. O candidato inicial passou 13/15; um dos problemas restantes era essa fixture e o outro a inclusão da própria linha no critério de conteúdo prévio. Um refutador adicional após pagebreak confirmou o risco de altura herdada e precedeu o ajuste da testemunha regional. Outro RED comprovou o topo indevido da primeira caixa pequena após pagebreak; sua admissão geométrica também foi corrigida. Logs e candidatos intermediários foram conservados, sem contar erro mecânico de teste como RED funcional.

A repetição contra o vanilla pinado mediu **50 pares de execução, somados entre os corpora**, incluindo entradas repetidas entre famílias: **49 certificados**, **43 observações divergentes corrigidas** e **6 controles preservados**; **1 caso fracionário permanece aberto e inalterado**. Não são 50 defeitos independentes. Páginas e palavras são exatas; dimensões de página, xMin/yMin/yMax e bounds vermelhos do corpus SVG principal usam tolerância fixa de 0.002 pt. A cor foi verificada como #ff4136. Cinco pares de pagebreak usam vetores de todas as páginas PDF convertidos por pdftocairo, com comandos/hashes por página. O binário anterior dessas cinco sondas é comprovadamente idêntico ao checkpoint e registra separadamente o build_source_hashes e a árvore observada. xMax tem resíduo de até 0.004780 pt, registrado separadamente e sem ampliar a tolerância geométrica. Primeiro frame alto pode naturalmente exceder a página; a aceitação verifica progresso/páginas, sem prometer que tinta fora da região passe a ser visível.

Suíte completa: **8792 passaram, 0 falhas e 7 ignorados**. Build workspace, fmt, Clippy all-targets com -D warnings/-D unfulfilled_lint_expectations, Rustdoc -D warnings/quatro índices, V1/V5/V15/V26 estrito, linhagem reversa e diff check passaram. O erro de sintaxe inicial do harness regional foi corrigido antes de medir e não conta como RED funcional. SVG nativo com template permanece uma pendência CLI: os arquivos medidos gravam literalmente {p} e só a primeira página; a geometria regional foi verificada pelo PDF.

SARIF global continua com exit 1 pelos 125 V16 conhecidos; o diferencial preserva todos os diagnósticos, com linhas atualizadas e nenhuma supressão.

## RAM e proveniência

Comandos pesados sequenciais, cada gate em cgroup próprio: **MemoryMax 12 GiB, MemoryHigh 10 GiB, swap zero, Cargo jobs=1**. Pico **8838008832 bytes (8.231 GiB)**; eventos memory.max **0**, memory.high **0**, sem OOM/OOM-kill. É memória do grupo, incluindo cache, não RSS isolado nem prova de melhoria de desempenho por comparação entre políticas de contabilização.

HEAD **06848cdcb5450f2c555626bf116bc7acd82bd82b**, working tree não commitado. before.json.gz, extended-before.json.gz, red-state.json.gz, medições bilaterais, validation-result.json e final-state.json.gz guardam UTC, status/diffstat exatos, hashes de fontes/L0/configuração/binários, argv e fontes tipográficas. Oráculo 9f2b6e8715237cb086899a42873660fe744622e8, binário SHA-256 7fa91e57df43593beef70967898d776d69c8f593b1f324ccb552290821c171ea. Linter 12797073972e375d9ab078cf6750c8b30b71e2b9, SHA-256 e2e51f3074cee2d0179bfa89f5b12884728f3d2d8f3113589dd095ef1d9eadb2.

[Inventário V16 restante](inventario-v16-restante.md), [pendências](pendencias.json) e [próximas auditorias](next-audit.md). O pacote RG1, manifesto a47fe97272fd6a5bd96022be959ba277a317a74086f5e5f4a569bba008f4fae9, continua aguardando aprovação e foi preservado, assim como seus consumers/owners. O objetivo geral permanece ativo.
