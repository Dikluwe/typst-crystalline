# Passo 1377 — fronteira vertical de bloco do contentor de colunas

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Fazer `#columns(N)` contínuo participar do espaçamento vertical de Block que
o vanila lhe atribui, tanto na entrada quanto na saída, sem abrir os modos
segmentado, multi-região, grid, box ou place.

## Medição anterior à decisão

O vanila realiza Columns como `BlockElem::multi_layouter` e o flow coleta
`above`/`below` com fallback em `par.spacing`. Na sonda 200pt × 240pt,
margem 10pt e fonte 11pt:

- `Intro`, `Inside`, `Outside`: vanila `7,404`, `27,842`, `48,280`;
- cristalino P1376: `7,404`, `21,792`, `36,180`;
- a diferença por fronteira é `6,05pt = (1,2 - 0,65) × 11`;
- quebra explícita e Block anterior no vanila preservam `One=27,842`,
  `Two=96,349990`, `After=220,166`, confirmando colapso, não soma.

Medição em 2026-09-13T17:21:02-03:00, HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado:
45 ficheiros no diff (`3269` inserções, `336` remoções) e 260 entradas de
status no instante da medição.

## Decisão

1. Atualizar primeiro o Prompt L0 proprietário de `columns.rs`.
2. Na entrada, fechar linha pendente e representar o topo do próximo Block
   como `baseline anterior + par.spacing`, deixando a baseline pendente; numa
   cadeia existente, colapsar pelo máximo como Block. O avanço observável é
   `max(0, par.spacing - par.leading)`, sem antecipar o `top-edge` ao preflight.
3. Na saída, manter `par.spacing` pendente para o próximo conteúdo e o excesso
   sobre leading no cursor estrutural.
4. No topo, não inserir espaçamento.
5. Preservar P1374–P1376 e manter `colbreak()` como `Unknown`.

É correção interna de paridade e segue fluxo contínuo pelo ADR-0127.

## RED → GREEN

- RED: ausência do helper/contrato de fronteira e matriz offset divergente;
- GREEN: unitário do delta 6,05pt, P1374–P1377 focais e matriz bilateral
  topo/offset/saída/quebra;
- ataques: remover entrada, remover saída, somar spacing inteiro, aplicar no
  topo e abrir segmentado.

## Memória

Execução estritamente serial. `cargo check` sob 3 GiB; link focal sob 4 GiB,
debug desligado e ficheiro máximo 512 MiB. Cada compilação da matriz roda
isolada sob 2 GiB, ficheiro máximo 16 MiB e timeout 45 s. Sem suíte ampla,
sem paralelismo e sem repetição com símbolos de debug completos.
