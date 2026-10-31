# Issue 8976 — geometria do retângulo sem dimensões

Correção interna implementada na branch Tekt, sem commit. O caso literal agora
mede 45×30 pt em SVG e no conteúdo rasterizado de PDF/SVG; a altura também participa
da medição e do fluxo. A política de dimensões explícitas/percentuais, os campos
públicos e os exportadores foram preservados. O mecanismo Edge/SMask e a diferença
prévia de alfa não foram adotados como requisitos.

L0 dos proprietários de Shape e measurement precederam o RED→GREEN e foram
resselados. Build passou, quatro testes específicos passaram e 16 fontes passaram
na geometria SVG e em rasters independentes PDF/SVG. Todos os artefatos compilados
e renders estão no target exclusivo em RAM da issue8976.

A validação global ainda tem três falhas dos novos testes 8790 e um V5 em uma
proposta L0 de ShapeElem de outro trabalho. Esses arquivos foram preservados;
não se declara a suíte global nem crystalline-lint verdes.

[Relatório e recibos](../../lab/results/construction/diagnostics/issue-8976-geometry-2026-10-31/report.md)
contêm HEAD, estado não commitado/diffstat, UTC, fontes, binários, REDs, rasters,
resultados e bloqueios exatos. Os números pertencem ao estado de completion.json,
final.json.gz e aos binários ali vinculados.
