# Diagnóstico P1140.11 — `linebreak(justify: true)`

**Data:** 2026-08-24  
**Baseline:** vanilla pinado `a51e02804`  
**Resultado:** paridade LTR; RTL permanece divergente

## Resultado entregue

O consumer de `justify` foi atomizado em `compiler/layout/linebreak.rs`.
Espaços registram oportunidades geométricas privadas e a linha LTR distribui
dinamicamente o restante. Nenhum número obtido das sondas entrou no código.

Na página de 160pt com margem de 10pt, o PDF cristalino passou a coincidir com
o vanilla:

| palavra | vanilla | cristalino |
|---|---:|---:|
| Alpha | 10,00–34,73 | 10,00–34,73 |
| Beta | 67,43–85,51 | 67,43–85,51 |
| Gamma | 118,21–150,00 | 118,21–150,00 |
| Z | 10,00–16,04 | 10,00–16,04 |

False, omissão, markup, linha sem oportunidade, largura auto, vazamento entre
linhas e links têm controles verdes. A área interativa do link é recalculada
depois da expansão.

## RTL não mascarado

A sonda hebraica true produziu no vanilla as posições esquerdas
`130,99 / 70,00 / 10,00`; o cristalino produziu
`130,99 / 86,51 / 43,02`. O controle false já mostra divergência de
agregação/geometria shaped no cristalino. Logo os deltas não pertencem ao
algoritmo de justificação e não podem virar constantes corretivas.

O L0 foi retificado: P1140.11 fecha LTR, não RTL. A próxima frente deve medir a
fronteira entre posições lógicas L1 e shaping/export RTL antes de decidir a
camada da correção.

**Resolvido por P1140.12:** a causa era a fusão indevida de linhas explícitas
em `layout_bidi`. O marcador semântico de fronteira restaurou paridade RTL
true/false sem compensação empírica.

## Proveniência

Fecho em `2026-08-24T11:57:21-03:00`, commit base
`ca28f4ab74ae66985cdc66805c16c2ddc8f08366`, working tree não commitado:

- diff tracked: **47 ficheiros, 1031 inserções, 142 remoções**;
- status: **54 entradas**;
- L1: **5155 passed**;
- L3: **828 passed**;
- build, fmt, lint e diff-check: exit 0.

As contagens abrangem a árvore acumulada dos passos anteriores e não medem o
tamanho isolado do P1140.11.
