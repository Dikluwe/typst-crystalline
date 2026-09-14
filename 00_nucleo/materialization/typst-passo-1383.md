# Passo 1383 — prefixo misto antes de distribuição fracionária

## Objetivo

Combinar a baseline mista selada no P1382 com o preflight escalar já medido no
P1381, aceitando somente `Text` exterior seguido por um `Styled(Text)` maior
antes do primeiro Block fracionário de um segmento explícito de colunas.

## Medição anterior à decisão

No caso 11pt→18pt, após P1382:

- vanila: `Lead=12,010`, `More=5,752`, `One=32,448`, `Two=7,404` e
  `After=Tail=220,166`;
- cristalino: linha exata (`12,010`/`5,752`), mas `One=26,398`,
  `After=198,202` e `Tail=220,166`.

Na ordem inversa, a linha também é exata, mas corpo/cauda divergem. Um único
fragmento maior ainda pagina. Medição em 2026-09-13T21:30:28-03:00, HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado:
47 ficheiros no diff (`4565` inserções, `419` remoções) e 328 entradas de
status. Binários: vanila `eb60986b...`, cristalino `e4a176a4...`. Pico:
37 536 KiB.

## Decisão

1. Atualizar primeiro o L0 proprietário de `sequence.rs`.
2. Aceitar somente dois fragmentos: primeiro direto/exterior, segundo
   `Styled(Text)` com delta exclusivo de `text.size` e tamanho maior.
3. Reutilizar a fórmula P1381 sem constante nem simulação.
4. Preservar como `Unknown` ordem inversa, fragmento único maior, três
   fragmentos, deltas compostos e wrap.

A candidata v1 aceitou a forma correta, porém deixou `After` 4,55pt antes de
`Tail`: reservou `par.spacing`, mas entregou a boundary do último fragmento,
enquanto P1382 passou a consumir o leading exterior. A revisão v2 mantém a
soma e resolve as duas parcelas pelo mesmo estilo exterior.

Correção interna de paridade; fluxo contínuo autorizado pelo ADR-0127.

## RED → GREEN e ataques

RED exige as seis coordenadas do caso positivo. Ataques mantêm fail-closed os
casos inverso, único maior, três fragmentos e size+fill, além dos controles
P1381/P1380. Mutantes ampliam forma, removem fronteira, trocam leading ou
ignoram wrap.

## Memória

Testes/link seriais sob 4 GiB, debug desligado e ficheiro máximo 512 MiB;
check/lint sob 3 GiB; matriz sequencial sob 2 GiB, ficheiro máximo 16 MiB e
timeout 45 s. Nenhuma suíte ampla, recursão ou coleção proporcional nova.
