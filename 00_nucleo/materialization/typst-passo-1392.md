# Passo 1392 — wrap único abaixo/acima do exterior

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Materializar a célula adjacente a P1391 em que dois wrappers size-only ficam
em lados opostos do tamanho exterior de 11pt, nas duas ordens, com uma palavra
por body e um único wrap entre os fragmentos.

## Medição anterior à decisão

Medição bilateral em 2026-09-14T12:42:12-03:00, HEAD
`d0ab401c12e7aa34b93748d00c00281f860ee00e`, working tree com apenas
`00_nucleo/diagnosticos/p1392-fixtures/` não rastreado. Binários: vanila
`eb60986b...`; cristalino `ae5b3050...`.

8→40, 40→8, 7→40 e 40→7 quebram antes de `More` em ambos. O vanila
mantém uma página e `After=Tail=220,166`; o cristalino produz duas páginas.
Na ordem menor→maior, `More` cristalino fica 1,95pt/2,60pt acima do vanila.
Na ordem maior→menor, o fechamento automático atrasa `More` em
39,906pt/40,564pt. Pico da matriz: 37.356 KiB, execução serial.

## Decisão

1. Atualizar os cinco L0 envolvidos antes do código.
2. Certificar exatamente dois `Styled(Text)` size-only, uma palavra cada,
   um tamanho abaixo e outro acima do exterior, um wrap na fronteira e fitting
   vertical completo.
3. Reutilizar reserva P1390, boundary P1388 e carrier/arm P1389–P1391.
4. No singleton menor certificado, comparar o item textual real ao exterior,
   não o máximo com fallback exterior; baseline/ascent continuam próprios e
   somente o leading usa o exterior. Na ordem maior→menor, antecipar a segunda
   baseline uma vez pelo delta de top-edge, já espelhado pela reserva.
5. Manter `Unknown` para delta composto, multiword, terceiro/nested,
   wrap interno/múltiplo e reserva insuficiente.

Correção interna de paridade em fluxo contínuo pelo ADR-0127.

## RED → GREEN, ataques e memória

O RED exige uma página, duas baselines e `After=Tail` nos quatro positivos.
Os ataques removem a célula, aceitam delta composto, usam o máximo fallback
para decidir mistura, desarmam o overlong, repetem máximo global ou duplicam
a boundary. P1391/P1390/P1389/P1388 devem sobreviver.

Execução serial: testes/link sob 6 GiB e 180 s; release sob 4 GiB/180 s;
matriz sob 2 GiB/45 s; check/lint sob 3 GiB. Duas revisões consecutivas sem
ganho reabrem o desenho.
