# Passo 1382 — baseline de linha textual com tamanhos mistos

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Fechar a divergência de baseline e avanço entre linhas quando uma linha contém
fragmentos textuais com tamanhos distintos, sem reabrir a distribuição
fracionária fail-closed do P1381.

## Medição anterior à decisão

Em página 200pt × 120pt e margem 10pt:

- 11pt→18pt: vanila `21,844 / 21,844 / 36,232`; cristalino
  `17,238 / 17,238 / 40,782`;
- 18pt→8pt: vanila `21,844 / 21,844 / 36,232`; cristalino
  `21,844 / 21,844 / 38,888`;
- numa única linha 11pt→18pt, o deslocamento faltante é 4,606pt.

Medições em 2026-09-13T20:58:15-03:00 e
2026-09-13T20:59:23-03:00, HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado:
47 ficheiros no diff (`4396` inserções, `350` remoções) e 317 entradas de
status. Binários: vanila `eb60986b...` e cristalino `2d353f53...`. Pico
observado: 36 804 KiB.

## Decisão

1. Atualizar primeiro o L0 proprietário de `cursor.rs`.
2. Corrigir a baseline corrente pelo maior ascent textual/inline em relação ao
   ascent assumido.
3. Calcular o avanço com ascent e leading do estilo ativo da linha seguinte,
   mais o maior descent da linha corrente.
4. Usar a mesma correção nas decisões de fitting.
5. Preservar P1374–P1381 e manter o prefixo fracionário misto como `Unknown`.

Integração focal: depois do GREEN do cursor, duas revisões do controle P1381
falharam pela mesma integração com o preflight fracionário (primeiro página,
depois coordenada 2,1pt deslocada). O desenho foi reaberto e a obrigação foi
reduzida a linhas com pelo menos dois tamanhos textuais distintos. Linha de
fragmento único conserva a fórmula selada anterior; distribuição mista segue
`Unknown`.

Correção interna de paridade; fluxo contínuo autorizado pelo ADR-0127.

## RED → GREEN e ataques

RED exige ordens crescente e decrescente, três fragmentos e linha seguinte.
GREEN exige também controles de tamanho uniforme, leading explícito, caixa
inline, P1374 e P1380–P1381. Mutantes trocam máximo por primeiro/último,
reutilizam ascent corrente no avanço, não deslocam o cursor, usam leading do
último fragmento ou relaxam o gate fracionário.

## Memória

Testes/link seriais sob 4 GiB, debug desligado e ficheiro máximo 512 MiB;
check/lint sob 3 GiB; matriz sequencial sob 2 GiB, ficheiro máximo 16 MiB e
timeout 45 s. Nenhuma suíte ampla, recursão ou coleção proporcional nova.
