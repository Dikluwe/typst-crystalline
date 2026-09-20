# Passo 1384 — ordem decrescente no prefixo fracionário misto

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Estender o preflight fracionário escalar somente à ordem 18pt→11pt cuja linha
já foi corrigida pelo P1382, preservando todas as formas vizinhas como
`Unknown` até medição própria.

## Medição anterior à decisão

Na fixture corrigida `2fd7f64d...`, em 2026-09-13T21:59:06-03:00:

- vanila: uma página; `Lead=5,752`, `More=12,010`, `One=32,448`,
  `Two=7,404` e `After=Tail=220,166`;
- cristalino P1383: uma página e linha exata, mas `One=26,398`,
  `After=198,202` e `Tail=220,166`.

Estado: HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não
commitado, 47 ficheiros no diff (`4728` inserções, `419` remoções) e 345
entradas de status. Binários: vanila `eb60986b...`, cristalino `5213f5ce...`.
Picos da sonda: 30 956 KiB e 37 336 KiB.

A primeira cadeia foi invalidada antes da implementação final: a fixture
histórica `3c140d02...` continha 18pt→8pt, não 18pt→tamanho exterior. Candidata
e testes foram retirados, a fixture correta foi medida bilateralmente e L0,
contrato e oráculos foram reabertos a partir desta medição.

## Decisão

1. Atualizar primeiro o L0 proprietário de `sequence.rs`.
2. Reconhecer somente `Styled(Text size-only maior)` seguido por `Text` direto
   exterior, exatamente dois fragmentos e sem wrap.
3. Reservar a mesma soma P1383, usando o top-edge exterior como próxima
   parcela de ascent e o par exterior de leading/boundary.
4. Manter fragmento único maior, dois wrappers, size+fill, terceiro fragmento
   e wrap fail-closed.

Correção interna de paridade em fluxo contínuo pelo ADR-0127.

## RED → GREEN e ataques

O RED exige uma página, baseline comum e diferença sintética `One−Two=25,8pt`,
além de `After=Tail`; a matriz PDF exige `25,044pt`. Controles opacos exercitam fragmento único maior, primeiro
delta size+fill e dois wrappers; P1383 e P1374–P1382 permanecem controles.

Budget: duas revisões focais. Duas falhas consecutivas com a mesma causa
reabrem o desenho antes de nova candidata.

## Memória

Testes/link seriais sob 4 GiB, debug desligado e ficheiro máximo 512 MiB;
check/lint sob 3 GiB; matriz sequencial sob 2 GiB, ficheiro máximo 16 MiB e
timeout 45 s. Nenhuma suíte ampla, recursão ou coleção proporcional nova.
