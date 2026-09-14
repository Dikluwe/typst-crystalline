# Passo 1391 — wrap único entre wrapper exterior e maior

## Objetivo

Materializar a célula adjacente a P1390 em que exatamente um dos dois
wrappers size-only usa o tamanho exterior de 11pt e o outro é maior, nas duas
ordens, com uma palavra por body e um único wrap entre os fragmentos.

## Medição anterior à decisão

Medição bilateral em 2026-09-14T12:09:11-03:00, HEAD
`70f7725d19fbfec892b6a0ee80a48479cad4f490`, working tree com apenas
`00_nucleo/diagnosticos/p1391-fixtures/` não rastreado. Binários: vanila
`eb60986b...`; cristalino `4f5b7544...`.

11→39, 39→11, 11→40 e 40→11 quebram antes de `More` em ambos. O vanila
mantém uma página e `After=Tail=220,166`; o cristalino produz duas páginas.
Em 11→39 e 11→40, `More` já coincide, isolando o erro na reserva. Nas ordens
inversas, `More` cristalino também fica tarde. Pico da medição: 37.208 KiB,
execução serial sob 2 GiB/45 s.

## Decisão

1. Atualizar os L0 de Sequence e testes antes do código.
2. Certificar exatamente dois `Styled(Text)` size-only, uma palavra cada,
   tamanhos desiguais, um igual ao exterior e o outro maior, com um wrap na
   fronteira e fitting vertical completo.
3. Reutilizar o carrier/fechamentos P1389 e a soma linha-a-linha P1390.
4. Tratar a boundary como única e separada da reserva de duas linhas.
5. Manter `Unknown` para tamanho abaixo do exterior, dois tamanhos exteriores,
   multiword, delta composto, terceiro fragmento, wrapper aninhado, wrap
   interno/múltiplo e reserva insuficiente.

Correção interna de paridade em fluxo contínuo pelo ADR-0127.

## RED → GREEN, ataques e memória

O RED exige uma página, duas baselines e `After=Tail` nos quatro positivos.
Os ataques removem a nova célula, admitem tamanho abaixo do exterior, repetem
o máximo global, omitem uma linha, retiram fechamento exterior ou duplicam a
boundary. P1390, P1389, P1388 e o controle abaixo-do-exterior devem sobreviver.

Execução serial: testes/link sob 6 GiB e 180 s (o teto inicial de 4 GiB foi
refutado pelo LLVM sem OOM do host), ficheiro máximo 512 MiB; matriz sob
2 GiB/45 s; check/lint sob 3 GiB. Duas revisões sem ganho na mesma causa
reabrem o desenho.
