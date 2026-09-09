# Correção explícita do timestamp do freeze V3

O campo `at` do freeze V3, `2026-09-09T00:23:56Z`, foi copiado da leitura de hashes dos insumos e não representa a hora efetiva de criação do arquivo. O autor identifica o erro sem editar o artefato congelado.

A execução que gravou `p1324-ab-freeze-v3.json` via apply_patch, calculou seu SHA be00d384f31ecbd25f2db69727432ee6f9af26ea0befc7cdc1705df0861914bd e leu o relógio retornou `2026-09-09T00:25:27Z`. Depois dessas operações sequenciais, iniciou a primeira focal C, cujo recibo `p1324-ab-candidate-focal-v3.json` registra `started=2026-09-09T00:25:28.192259+00:00`. Esta é a proveniência temporal efetiva. Portanto a criação e validação do freeze precederam a primeira execução/observação do candidato pelo autor, mas aconteceram depois da existência da implementação, como já declarado.

Não usar o campo `at` incorreto como evidência de congelamento às 00:23:56. Hashes, conteúdo, comparador e casos não foram alterados para esta correção. O revisor recebe tanto o artefato original quanto esta correção e os recibos de execução.
