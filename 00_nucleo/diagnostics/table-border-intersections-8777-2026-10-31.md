# Interseções de borda de tabela — issue 8777

O caso oficial da issue 8777 foi corrigido para grades retangulares uniformes
com células resolvidas e não quebráveis, stroke sólido sem dash e cap butt.
Uma cobertura NonZero por página elimina o escurecimento nas interseções.
Spans, gaps, strokes distintos, células quebráveis e pinturas ricas conservam
a rota anterior; não há certificação geral de ausência do artefato nesses casos.

RED de cobertura e nativo → GREEN; build passou; suíte de layout: 1664 passaram,
3 ignorados, zero falhas. Lint completo zero, lineage reversa zero e formatação
passaram. PDF/Cairo e SVG rasterizados e inspecionados, compacto/verboso a
72/600dpi; texto, fontes, posições de glifos e contorno SVG preservados.

Relatório, proveniência exata e limites:
`lab/results/construction/diagnostics/issue-8777-fix-2026-10-31/result.md` e
`final-provenance.json.gz` nesse diretório. Binários e resultados brutos ficaram
no target RAM exclusivo `/dev/shm/typst-crystalline-8777-target`.

HEAD `212aed0449150dd296ceea13fc2be665cc5db01a`, árvore não commitada;
UTC/status/diffstat de cada medição nos recibos. Sem nova branch/worktree/commit.
A migração concorrente aprovada de Stroke para Length foi preservada.
