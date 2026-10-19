# Memoização do índice semântico inicial

Gate ADR-0127 confirmado pelo usuário em 2026-10-05 para `00_nucleo/prompts/entities/module.md`, SHA-256 `b1499e45b2c79e61ceb0b4e6f079caf639ae610ffa09a37bab53c057edd89e54`. Este documento é passo de execução, não L0.

Escopo: conservar o resultado já produzido por realização e reaproveitá-lo em paged, HTML e query. Chave implícita: avaliação dona do Module; cache de resultado imutável, sem globals/unsafe. Conteúdo alterado invalida. Bundle conserva o índice próprio de documentos e sementes. Sem memoizar callbacks, layout ou fontes nesta etapa.

Os owners do Module, eval, testes de eval, fachada de realização e consumers paged/HTML/query foram atualizados antes de código. RED→GREEN precisa provar transporte/invalidação/cópias independentes e compatibilidade de colunas. Baseline otimizado preservado em `/tmp/tekt-index-reuse-2026-10-05/typst-baseline`, SHA-256 `f03000b982aa07e4dd5ddad5042099757cb48e5f5ec674edc69d658e7b4bc20e`. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, working tree da avaliação única.

Medir pares intercalados contra este baseline, conferindo linguagem e geometria. A rodada anterior da dupla avaliação não é baseline numérico desta etapa.


## Conclusão

Implementado e medido. O Module conserva seu índice realizado em Arc; os carriers de conteúdo invalidam o snapshot. Query toma empréstimo; PDF/HTML tomam cópia independente para a tentativa. Bundle mantém seu algoritmo próprio.

Passaram 6.508 testes de núcleo (5 ignorados), 1.143 de infraestrutura, build release, sondas CLI e a geometria/raster das sete fixtures. Contagem GDB: PDF 6→5, HTML 2→1, query 2→1 entradas puras na sonda. Consultas de 500/2.000 headings com show melhoraram 17,2%/22,0%; compilação completa sem ganho convincente para a tabela. Testes RED e fontes foram preservados antes da implementação de publicação/invalidação.

Gate ownership/núcleos limpo e sem V5/V6/V15/V26 nos consumers do conjunto pendente. Lint global ainda contém avisos externos; não é zero violations. [Resultado e recibos](../diagnosticos/memoizacao-indice-2026-10-05.md).
