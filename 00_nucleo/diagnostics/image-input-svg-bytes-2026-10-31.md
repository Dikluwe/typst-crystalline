# image(): raster, SVG e SVGZ com source preservado

**Estado raster em 2026-10-31:** admissão em bytes concluída na branch compartilhada Tekt,
com source original preservado. Sem commit, push, branch ou worktree novo.

A mudança pública de ImageElem foi aprovada e materializada no primeiro recorte
PNG. O pedido seguinte ampliou a mesma via para JPEG/GIF/WebP, sem novo schema,
assinatura pública, flag, default ou mudança de fase. Bytes não resolve/carrega
arquivos e tem path=None; source conserva o Value::Bytes original. Arquivos
preservam Str/Path originais separadamente do caminho resolvido. Getters, has(),
fields(), repr e alias/show mantêm a proveniência.

A medição separou a recusa no construtor de um defeito do decoder WebP antigo,
que alterava as cores antes do PDF. O helper L3 privado usa image-webp 0.2.4,
compartilhado pela preparação PDF e textura bitmap, com RGB/alpha e primeiro
frame preservados. Os outros decoders e as APIs públicas permanecem iguais.
A dependência e duas transitivas foram resolvidas offline, sem atualizar versões
existentes. L0 e pins foram atualizados antes de código.

RED: três controles do construtor e nove de infraestrutura executados falharam.
GREEN: 12 controles focados e a suíte completa passaram. Validação final:
10.024 testes aprovados, zero falhas e sete ignorados; build e lint aprovados;
reverse lineage com 1.007 consumidores sem erro. PDF/SVG passaram em 353
verificações e PNG em 28 renders comparados em 14 pares. Os testes incluem
source literal/read/str/path, alias/show, RGB/alpha, primeiro frame GIF/WebP,
WebP lossless/lossy/opaco/animado e dados truncados.

- [Relatório e evidências](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/raster-bytes-2026-10-31/report.md)
- [Patch desta etapa](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/raster-bytes-2026-10-31/own-changes.patch)
- [Recibo final e proveniência](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/raster-bytes-2026-10-31/final-receipt.json)
- [Primeiro recorte PNG aprovado](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/png-bytes-2026-10-31/report.md)
- [Investigação de SVG e formatos](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/image-input-scope-2026-10-31/report.md)

HEAD base 3c6b0ce7b7632cc74b004cea6681e4746e63d28d, árvore compartilhada não
commitada. Snapshots/recibos conservam UTC, arquivos e diffstat exatos. Os inputs
ficaram estáveis durante suíte/build e até os renders. Foi reutilizado somente
o target exclusivo deste chat em RAM, com um job, 12 GiB e zero swap.

No recibo raster de 2026-10-31, SVG/SVGZ ainda estavam pendentes.
PDF como entrada e formato explícito/raw pixels permanecem em seus escopos próprios.
Não há afirmação de playback de animação, paridade universal de todos os perfis
raster nem fechamento de #8996. Não há gate pendente para os formatos entregues.


## SVG/SVGZ entregue — 2026-11-01

Após aprovação humana específica do contrato e fase, SVG/SVGZ por arquivo e bytes
foram implementados, preservando source e os dados originais. A preparação L3 é
chamada por ocorrência no layout com origem e fontes efetivas; o frame recebe
SVG autossuficiente. PDF conserva Form vetorial, SVG embute o preparado e PNG
rasteriza na escala de saída. Fontes/recursos não são reabertos pelos exporters.

L0, Nucleus/pins e linhagem foram atualizados antes da materialização e dos
reseals. RED→GREEN cobriu admissão/source, geometria fracionária, gzip/XML/DOCTYPE,
fontes por contexto/span e aquisição sob demanda. Os rasters anteriores foram
revalidados. Estado final estável: 10.065 testes workspace passam, mais a
integração SVG; build em RAM, lint sem violações, 1.010 consumidores sem erro de
linhagem e 659 verificações de PDF/SVG/PNG aprovadas. Sem commit nem push.

[Relatório e recibos SVG/SVGZ](/repos/Antigravity/typst-crystalline/lab/results/construction/diagnostics/svg-svgz-2026-11-01/report.md).
Os snapshots desse recibo registram HEAD, árvore não commitada, arquivos/diffstat,
UTC e binários. Estes números pertencem ao estado final medido em 2026-11-01,
sem substituir os recibos históricos do raster.
