# Passo 1387 — wrappers uniformes acima do exterior

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Materializar dois `Styled(Text)` size-only iguais e maiores que o exterior,
corrigindo a classificação de mistura relativa ao estilo ativo.

## Medição anterior à decisão

Matriz bilateral em 2026-09-14T09:34:50-03:00, sob P1386:

- vanila: 12→12, 14→14, 16→16, 18→18 e 20→20 ficam em uma página, com
  `One=28,500/29,816/31,132/32,448/33,764` e `After=Tail=220,166`;
- cristalino: a linha coincide, mas `One=23,758/27,690/31,622/35,554/39,486`
  e `After=216,398/215,770/215,142/214,514/213,886`.

Estado: HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não
commitado, 47 ficheiros no diff (`5087` inserções, `419` remoções) e 372
entradas de status. Binários: vanila `eb60986b...`, cristalino `3a2273ae...`.
Pico da matriz: 37 380 KiB.

## Decisão

1. Atualizar primeiro os L0 proprietários de Sequence e Cursor.
2. Aceitar dois wrappers iguais, size-only, maiores que o exterior e sem wrap,
   reutilizando a reserva P1386.
3. Fazer Cursor considerar mistura também em relação ao estilo exterior.
4. Preservar `Unknown` para crescimento, primeiro não maior, delta composto,
   terceiro fragmento e wrap.

Correção interna de paridade em fluxo contínuo pelo ADR-0127.

## RED → GREEN, ataques e memória

RED cobre os cinco pares medidos; controles cobrem P1386, crescimento e formas
opacas. Mutantes removem a comparação com o exterior, aceitam tamanho não
maior, usam leading uniforme ou duplicam a boundary.

Execução serial: matriz sob 2 GiB/45 s; testes e link sob 4 GiB, debug
desligado e ficheiro máximo 512 MiB; check/lint sob 3 GiB. Budget de duas
revisões focais; duas falhas da mesma causa reabrem o desenho.

A candidata v1 deixou 12pt, 14pt e 16pt verdes no oráculo sintético; 18pt foi
rejeitado pelo gate conservador de largura desse oráculo. Como em P1386,
18pt/20pt permanecem positivos produtivos e são julgados somente pela matriz
com métricas reais, sem ampliar ou contornar o gate.

A candidata v1 ampla regressou os dois singletons P1381. A revisão v2 ativa a
comparação com o exterior somente a partir do segundo item textual no fold;
isso preserva singleton sem segunda varredura ou coleção.

Na matriz produtiva v2, 12pt–18pt ficaram exatos. 20→20 permaneceu com
`One=27,714` contra `33,764`: o gate de largura não reconheceu a forma, embora
Cursor tenha fechado `After=Tail`. O contrato foi estreitado aos quatro pares
provados; largura 20pt continua `Unknown` e não foi contornada.
