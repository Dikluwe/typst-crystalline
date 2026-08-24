# Diagnóstico P1140.20.2 — gate de canvas e camadas

**Data:** 2026-08-24  
**Estado:** Fase A fechada; aguarda confirmação ADR-0127

## Contrato preparado

- PageBleedSpec preserva relativos, lados lógicos e fold; PageBleed é físico;
- width/height continuam TrimBox; canvas soma bleed por lado;
- PageFill distingue Auto, None e Paint até o target;
- Page conserva background/body/foreground em vetores separados;
- ordem visual fill→background→body→foreground;
- layers são decorativos e não entram em texto/query/tagging;
- PDF usa MediaBox expandida e TrimBox somente com bleed não zero;
- SVG/raster resolvem fill auto como branco e respeitam render_bleed;
- SetPage/PageRun transportam deltas e page-run restaura LIFO;
- running matter/supplement e binding page continuam adiados.

## Por que o gate é obrigatório

Serão acrescentados campos públicos a Content, PageRunElem, PageConfig e Page;
o default visual passa a ser resolvido explicitamente por target e as caixas
PDF mudam com bleed. Isso combina contrato público e comportamento padrão.

## Hashes SHA-256

- page_canvas `6c7e72d6a72d2660a80d5414dfc776e3263a6d0441fb31e17935bb0936fcde69`;
- layout_types `c6ff31cf769fe89d6d033d98dd87b485e06958e3e79a54378a4be92a31bb3652`;
- content `2658803c6ebd289792a647994d3a24676767ead50e30b991c201faf42eb7cc87`;
- page_run `875395c735bda0dd8020fc6ffe0ed57ade47a8315841394a0ed1acc42ec27764`;
- eval `5854961c438bd7b0ffd46847d03fef74e2bfe82eca955c44725571b379ffda33`;
- layout `465aa86945124a17c6050c54e0ea96eb9a9ab9be4e43fa86b081d2698d083f39`;
- stdlib/layout `c31f09a0b8ace59416c3f437c1f9a624b240f4379fc8b2ca6cd6caf05f154a4f`;
- PDF builder `908d63d7e74c6f2ff8da36612f77a28f0a9b57af427b220109861ef52fe0530f`;
- SVG `c9500ba4fdaba2cb42dc5386a74a087ad855e824411291aa39a990cae5d0890d`;
- raster `4adefc6eb3aaf12ac1d9fe65e4c1abeacbd6b8be38cc6c02160919efc7d116e6`.

## Proveniência e validação

- HEAD `45b547073d7686cdd5d3e3030c82de3e22ec395f`;
- working tree não commitada, contendo passos anteriores;
- hora dos hashes `2026-08-24T14:40:13-03:00`;
- stat rastreado: `113 files changed, 1488 insertions(+), 553 deletions(-)`;
- resselo alterou 33 headers, somente @prompt-hash;
- validar lint/diff após este relatório; não repetir build/testes, pois a Fase A
  muda apenas L0, diagnóstico, passo e headers.

## Depois da confirmação

RED por domínio, eval, layout, PDF, SVG/raster, layers e restauração; depois
implementação atomizada e validação completa. Sem confirmação, parar aqui.
