# Passo 1385 — dois wrappers de tamanho em ordem decrescente

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Estender o preflight fracionário escalar a exatamente dois `Styled(Text)`
size-only, quando o primeiro é maior que o estilo exterior e o segundo é
menor ou igual ao exterior.

## Medição anterior à decisão

Matriz bilateral em 2026-09-13T22:51:44-03:00, sob o binário P1384:

- 18pt→8pt: linha exata; vanila `One=32,448`, `After=Tail=220,166`;
  cristalino `One=26,398`, `After=216,758`, `Tail=220,166`;
- 18pt→11pt: linha exata; cristalino `After=213,338`;
- 18pt→14pt: linha exata; cristalino `After=209,918`;
- 16pt→8pt: linha exata; vanila `One=31,132`, cristalino `One=25,082` e
  `After=217,722`; vanila mantém `After=Tail=220,166`.

Estado: HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não
commitado, 47 ficheiros no diff (`4808` inserções, `419` remoções) e 350
entradas de status. Binários: vanila `eb60986b...`, cristalino `99940887...`.
Pico da matriz: 37 324 KiB.

## Decisão

1. Atualizar primeiro o L0 proprietário de `sequence.rs`.
2. Aceitar exatamente dois wrappers size-only: primeiro maior que o exterior,
   segundo menor ou igual ao exterior, sem wrap.
3. Reutilizar máximo textual + top-edge/leading/boundary exteriores P1384.
4. Preservar como `Unknown` segundo acima do exterior, igualdade, ordem
   crescente, primeiro não maior, delta adicional, terceiro fragmento e
   wrapper aninhado.

A candidata v1 ampla aceitou qualquer segundo tamanho abaixo do primeiro.
18→8, 18→11 e 16→8 passaram, mas 18→14 deixou `After` 6,05pt antes de `Tail`.
Ela foi invalidada; L0, contrato e testes foram reabertos antes da candidata
estreita.

Correção interna de paridade em fluxo contínuo pelo ADR-0127.

## RED → GREEN e ataques

RED cobre 18→8, 18→11 e 16→8; os controles opacos cobrem 18→14, 18→18,
18→20 e 10→8. Mutantes removem o teto exterior, deixam o segundo tamanho governar a
altura, aceitam delta composto ou regressam P1383/P1384.

Budget: duas revisões focais. Duas falhas consecutivas com a mesma causa
reabrem o desenho antes de nova candidata.

## Memória

Testes/link seriais sob 4 GiB, debug desligado e ficheiro máximo 512 MiB;
check/lint sob 3 GiB; matriz sequencial sob 2 GiB, ficheiro máximo 16 MiB e
timeout 45 s. Nenhuma suíte ampla, recursão ou coleção proporcional nova.
