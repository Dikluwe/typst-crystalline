# Segunda limpeza do laboratório — 2026-11-14

Repositório: 43.17 → 24.45 GiB. Mais 18.72 GiB liberados, medidos por `du -sx -B1 .`. HEAD `48e5a2570159c6fc325babe883bac7d1c1fbd4b2`, working tree não commitada. O estado anterior e o diffstat exato estão em `lab/results/storage-generated-2026-11-14/before.json`.

Foram processados 496082 arquivos ignorados: saídas PDF/raster/SVG e extrações XML/HTML antigas foram removidas, assim como executáveis superados. Dados brutos grandes foram compactados em gzip e conferidos por SHA-256 descompactado. Fontes, L0, fixtures, assets, snapshots de fonte, arquivos versionados e trabalho local foram preservados. As receitas antigas precisam regenerar as saídas removidas ou descompactar os dados antes de uso.

Conferidos 53340 arquivos protegidos e todos os hashes do recibo ativo de Box e de seu harness. Nenhuma integração ao produto. Sem rebuild, pois a mudança foi apenas armazenamento e documentação.

O espaço restante inclui aproximadamente 2,2 GiB de histórico Git, 2,3 GiB de um arquivo com fontes históricas e resultados/snapshots versionados ou preservados. Esses itens não foram classificados como caches descartáveis. O relatório de Box, baselines congeladas e controles de invalidação atuais continuam diretamente disponíveis.

Plano, contagens por classe, recibos e índice por caminho/hash em `lab/results/storage-generated-2026-11-14/`. Índice: `actions.jsonl.gz`. Ferramenta e instruções em `lab/tools/cleanup-results/README.md`.
