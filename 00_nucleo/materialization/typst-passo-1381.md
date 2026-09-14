# Passo 1381 — `text.size` no prefixo anterior a `fr`

## Objetivo

Estender a linha inline P1380 para um único fragmento textual com tamanho de
fonte reduzido, preservando a distribuição fracionária local ao segmento.

## Medição anterior à decisão

Em página 200pt × 240pt, margem 10pt e colunas de 85pt:

- `size: 18pt`: vanila `One=40,848`, `After=Tail=220,166`; cristalino P1380
  `One=35,554` e `After=7,404` na página seguinte;
- `size: 8pt`: vanila `One=25,868`, `After=Tail=220,166`; cristalino
  `One=17,868` e pagina `After`;
- misto 11pt + 18pt: vanila `One=32,448`, `After=Tail=220,166`; cristalino
  `One=30,948`, `After=202,752`.

Medição em 2026-09-13T20:27:47-03:00, HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado:
47 ficheiros no diff (`4272` inserções, `350` remoções) e 306 entradas de
status. Binários: vanila `eb60986b...` e cristalino `9ef1ea39...`. Pico
observado: 37 220 KiB.

## Decisão

1. Atualizar primeiro o L0 proprietário de `sequence.rs`.
2. Permitir somente `size`, um único fragmento textual e tamanho não superior
   ao exterior.
3. Acumular top-edge do primeiro texto, maior estilo da linha pela mesma regra
   estrita de `flush_line` e leading do último texto.
4. Manter largura por fragmento e rejeitar wrap.
5. Preservar como `Unknown` todos os demais eixos verticais, wrappers
   aninhados e contextos não segmentados.

Correção interna de paridade; fluxo contínuo autorizado pelo ADR-0127.

## RED → GREEN e ataques

RED nos casos grande, pequeno e misto. A primeira candidata fechou páginas,
mas não reproduziu as coordenadas do corpo. A leitura da fonte mostrou que o
primeiro fragmento maior também altera o estilo compartilhado e o colapso de
spacing; essa forma foi reclassificada `Unknown`. A segunda candidata acertou
bloco/cauda do misto, mas deslocou toda a linha em 4,606pt; misto também foi
reclassificado `Unknown`. GREEN exige o caso pequeno e controles fail-closed,
P1380 e P137. Mutantes: ignorar o leading do fragmento, aceitar composto com
tamanho variável, aceitar primeiro fragmento maior, aceitar `leading` e
regredir P1380.

## Memória

Testes/link seriais sob 4 GiB, debug desligado e ficheiro máximo 512 MiB;
check/lint sob 3 GiB; matriz sequencial sob 2 GiB, ficheiro máximo 16 MiB e
timeout 45 s. Nenhuma suíte ampla, recursão ou coleção proporcional nova.
