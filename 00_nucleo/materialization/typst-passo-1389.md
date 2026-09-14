# Passo 1389 — prefixo uniforme com um wrap entre fragmentos

## Objetivo

Materializar o primeiro prefixo fracionário de duas linhas: dois wrappers
`Styled(Text)` size-only iguais, cada body com uma palavra, e exatamente um
wrap antes do segundo fragmento.

## Medição anterior à decisão

Medição bilateral em 2026-09-14T10:56:36-03:00, HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado e
agregado com P1356–P1388, 47 ficheiros no diff (`5270` inserções, `419`
remoções) e 394 entradas de status. Binários: vanila `eb60986b...` e
cristalino `6739562e...`.

Nos tamanhos iguais 21, 22, 24, 30, 39 e 40pt, ambos produzem duas linhas. O
vanila usa `More=26,012/26,434/27,278/29,810/33,608/34,030010` e mantém
`After=Tail=220,166`. O cristalino usa
`32,512/33,584/35,728/42,160/51,808/52,880` e pagina `After`. A matriz foi
serial sob 2 GiB/45 s e atingiu 37 276 KiB.

## Decisão

1. Atualizar primeiro os L0 proprietários de Sequence e Cursor.
2. Certificar apenas dois wrappers iguais size-only, uma palavra por body e um
   único wrap na fronteira entre eles, com reserva vertical que cabe.
3. Transportar privadamente o índice do wrap; fechar as duas linhas pelo modo
   exterior exclusivo do Cursor e conservar máximo/descent do item.
4. Acrescentar à reserva P1387 exatamente um avanço exterior de linha; manter
   a boundary P1388 separada e única.
5. Preservar `Unknown` para multiword, tamanhos diferentes, delta adicional,
   wrap interno/múltiplo, reserva insuficiente e formas opacas.

Correção interna de paridade em fluxo contínuo pelo ADR-0127.

## RED → GREEN, ataques e memória

O RED exige duas baselines, distância externa entre elas, uma página e
`After=Tail`. Mutantes removem o carrier, fecham uma das linhas pelo modo
ordinário, duplicam a boundary, concedem multiword, ignoram fitting vertical ou
regredem P1388/P1387.

A candidata v1 reduziu o avanço FixedMetrics de 28,35pt para 21,85pt. A v2
interpretou esse resíduo como excesso e suprimiu a correção de ascent, fazendo
o teste sintético passar sob um oráculo incorreto de 14,85pt. A matriz
produtiva refutou o oráculo: `More` ficou cedo demais exatamente por
`top(interno)-top(exterior)` em todos os quatro positivos. O contrato foi
reaberto; a v3 conserva a correção do ascent corrente e externaliza somente a
previsão da próxima baseline e o leading. A reserva recebe o mesmo máximo.

Execução serial: matriz sob 2 GiB e timeout 45 s; testes e link sob 4 GiB,
debug desligado e ficheiro máximo 512 MiB; check/lint sob 3 GiB. Duas revisões
sem ganho na mesma causa obrigam reabrir o desenho.
