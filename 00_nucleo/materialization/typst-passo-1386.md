# Passo 1386 — dois wrappers decrescentes ainda acima do exterior

## Objetivo

Estender o preflight fracionário escalar à faixa adjacente de exatamente dois
`Styled(Text)` size-only, estritamente decrescentes e ambos maiores que o
estilo exterior.

## Medição anterior à decisão

Matriz bilateral em 2026-09-13T23:10:42-03:00, sob o binário P1385:

- 18pt→12pt: linha exata; vanila `One=32,448`, `After=Tail=220,166`;
  cristalino `One=26,398`, `After=212,198`, `Tail=220,166`;
- 18pt→14pt: linha exata; cristalino `After=209,918`;
- 20pt→14pt: linha exata; vanila `One=33,764`, cristalino `One=27,714` e
  `After=208,954`;
- 20pt→18pt: linha exata; cristalino `After=204,394`; vanila mantém
  `After=Tail=220,166`.

Estado: HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não
commitado, 47 ficheiros no diff (`4941` inserções, `419` remoções) e 361
entradas de status. Binários: vanila `eb60986b...`, cristalino `9b089823...`.
Pico da matriz: 37 584 KiB.

## Decisão

1. Atualizar primeiro o L0 proprietário de `sequence.rs`.
2. Aceitar somente dois wrappers size-only quando `primeiro > segundo > exterior`
   e o gate P1380 existente prova que a linha não envolve wrap.
3. Usar o máximo inicial, top-edge/leading exteriores e entregar a boundary
   exterior no runtime sem duplicá-la na reserva fixa.
4. Preservar `Unknown` para igualdade, ordem crescente, primeiro não maior,
   delta adicional, terceiro fragmento, wrapper aninhado e wrap.

A candidata v1 retirou a boundary duplicada e os dois casos 18pt passaram. Os
testes 20pt continuaram em duas páginas mesmo com uma guarda de ULPs (v2) e com
alturas de 240pt até 242pt. A sonda interna então mostrou que eles nunca
alcançavam a fórmula P1386: o gate de largura P1380 os rejeitava antes. O
contrato foi reaberto para distinguir a rejeição sintética da execução
produtiva. A matriz final provou que 20pt atravessa o gate com as métricas reais
e coincide; ela prevalece sobre a sonda sintética.

Na primeira matriz release após o GREEN sintético, zero literal colocou os
casos 18pt exatamente no fundo e o backend real paginou `After`. Isso reabilita
a v2: sua guarda relativa P1379 era correta para os casos que efetivamente
atravessam o gate; os resultados 20pt que a haviam mascarado eram fail-closed
anteriores à fórmula. A candidata final conserva somente essa guarda de ULPs.

A segunda e a terceira matrizes release mostraram que ULPs, 0,000001pt e
0,0009pt na reserva não atravessavam a decisão discreta. O desenho foi
reaberto: `One` já coincidia, e o ponto restante era a comparação de baseline
terminal em Cursor. O L0 de Cursor passa a aceitar somente uma cauda
fracionária marcada cuja ultrapassagem não excede `below - leading`; nesse
caso a baseline é fixada no fundo. Acima dessa boundary, ou em fluxo ordinário,
a quebra não muda. A reserva P1386 volta a zero, sem spacing artificial.

Correção interna de paridade em fluxo contínuo pelo ADR-0127.

## RED → GREEN e ataques

RED cobre 18→12 e 18→14. Controles cobrem 20pt rejeitado por largura, as
fronteiras P1385, igualdade, crescimento, primeiro pequeno e delta composto. Mutantes removem a
ordem estrita, incluem novamente a boundary na reserva, deixam o segundo
estilo governar métricas ou ampliam a forma estrutural.

Budget: duas revisões focais. Duas falhas consecutivas com a mesma causa
reabrem o desenho antes de nova candidata.

## Memória

Testes/link seriais sob 4 GiB, debug desligado e ficheiro máximo 512 MiB;
check/lint sob 3 GiB; matriz sequencial sob 2 GiB, ficheiro máximo 16 MiB e
timeout 45 s. Nenhuma suíte ampla, recursão ou coleção proporcional nova.
