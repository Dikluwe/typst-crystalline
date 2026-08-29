# P1242 — inventário SVG saneado

**Veredito:** `INVENTORY_CLOSED_TILING_FIRST_DIAGNOSTIC`.

## Medição antes da decisão

A execução consumiu o resumo e o certificado saneados de P1241: 16 pares,
dois `Preserved`, 14 `Unknown`, sem mutation score. P1240 é somente um
harness raster auxiliar e não sela IDs, ordem de `defs`, translação externa
ou paths Conic.

P1243 v3 está `PRESEAL_SEALED 34/34`, mas o selo protege uma fronteira
negativa: há zero casos SVG adicionais provados `Preserved`; os controles do
construtor são current-only e a paridade compartilhada com o vanilla permanece
`Unknown`. Portanto, ele não autoriza implementação de tiling em P1244.

Os witnesses atuais continuam presentes em
`03_infra/src/export/svg.rs:171-187,241-265,699-704,1086-1142`: tiling
color+size é emitido, glifo matemático direto é omitido, imagem é encaminhada,
`clip_mask` é ignorado e destino interno não recebe identidade SVG.

## Decisão

A ordem diagnóstica permanece: (1) tiling modelável; (2) clip geométrico;
(3) imagem visível; (4) paint multi-space; (5) links internos; e (6) glifo
matemático direto. “Tiling primeiro” significa fechar a fronteira negativa em
P1244 sem código, salvo nova fixture bilateral. Não significa que exista
implementação de paridade já autorizada.

P1246 e P1248 continuam com contratos rejeitados. P1247 e P1249 continuam
`CONTRACT-GAP` e exigem gate ADR-0127 antes de expansão de identidade ou
pipeline. Os 14 pares multi-space continuam `Unknown`. Diferenças de IDs,
ordem de `defs` e bytes de serialização permanecem mecânica, conforme
ADR-0107.

## Execução e proveniência

O runner `lab/parity/matrix/p1242_inventory.py` validou dependências,
contagens, fronteira P1243 e witnesses do exporter. Duas execuções com as
mesmas entradas produziram saída byte-idêntica.

- HEAD: `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`.
- Working tree não commitada: 59 ficheiros tracked alterados, 2247 inserções e
  188 remoções.
- Medição: `2026-08-28T06:34:10-03:00`.

Diagnóstico somente leitura sobre produção: segregação Tekt não aplicável.
Nenhum código produtivo, Prompt L0, whitelist, mapa ou fila foi alterado.
