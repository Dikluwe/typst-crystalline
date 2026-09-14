# Passo 1376 — altura fracionária em colunas contínuas

## Objetivo

Distribuir `block(height: fr)` no fluxo contínuo de `#columns(N)` e de
`#set page(columns: N)`, usando a altura que resta a partir do início real da
coluna. Não absorver o modo segmentado nem outros sub-layouts.

## Medição anterior à decisão

Em página 200pt × 240pt com margem 10pt, Blocks 1fr/2fr e `After`:

- vanila, colunas contínuas no topo: `One=7,404`, `Two=82,724670`,
  `After=220,166`;
- cristalino P1375: `One=7,404`, `Two=42,230`, `After=77,056`;
- com `Intro` antes do contentor, vanila: `Intro=7,404`, `One=27,842`,
  `Two=96,349990`, `After=220,166`; o cristalino ainda mantém alturas
  intrínsecas.

As formas `#columns(2)` e `#set page(columns: 2)` coincidem nesse observável.
Sondas e identidades ficam no manifesto. Estado medido em
2026-09-13T16:22:40-03:00, HEAD `8ceea20f…5248`, working tree não commitado:
45 ficheiros no diff e 249 entradas de status.

## Decisão

1. `column_mode && page_columns.is_some()` identifica exclusivamente o fluxo
   contínuo; deixa de bloquear o preflight.
2. Nesse fluxo, a base é `page_bottom_limit - cursor_y`, saturada em zero.
3. Fora de colunas contínuas, a base P1374 permanece intacta.
4. `column_mode && page_columns.is_none()` continua bloqueado: cobre o modo
   segmentado e evita crédito sobre baseline ainda divergente.
5. Não há mudança pública nem de fase; ADR-0127 autoriza fluxo contínuo.

## Verificação de propagação aos sub-layouts — ADR-0124

- [x] `grid`: medido; vanila põe `After=220,166`, cristalino P1375
  `After=42,230`; permanece `Unknown`.
- [x] `box`: medido; vanila usa uma página, cristalino P1375 duas;
  permanece `Unknown`.
- [x] `place`: medido; o corpo colocado sobrepõe `After` no vanila e possui
  origem vertical distinta no cristalino; sem contrato comum, `Unknown`.
- [x] colunas: contínuas entram no P1376; segmentadas por `colbreak()` seguem
  `Unknown` devido à divergência anterior de baseline.

O passo fecha somente a propagação em colunas contínuas, não o mecanismo
fracionário em todos os cinco caminhos.

## RED → GREEN

- RED unitário para a base regional no topo e após cursor deslocado;
- GREEN P1374–P1376 e matriz bilateral topo/offset/page-columns;
- ataque à remoção do offset, uso da altura total, abertura segmentada e
  regressão do fluxo direto.

## Memória

Testes e builds seriais. Check sob 3 GiB; link focal sob 4 GiB, debug
desligado e arquivo máximo 512 MiB. Cada compilação da matriz roda sozinha sob
2 GiB, arquivo 16 MiB e timeout 45 s. Nenhuma suíte ampla.
