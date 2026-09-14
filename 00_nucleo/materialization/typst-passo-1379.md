# Passo 1379 — distribuição fracionária por segmento explícito

## Objetivo

Fazer cada segmento separado por `colbreak()` distribuir seus próprios Blocks
`fr` até o fundo da região de coluna, preservando a origem e saída P1378.

## Medição anterior à decisão

Em página 200pt × 240pt, margem 10pt e fonte 11pt:

- básico: vanila `One=Two=7,404`, `After=Tail=220,166`; cristalino P1378
  conserva os corpos, mas usa `After=Tail=42,230`;
- depois de `Intro`: vanila conserva as duas caudas em `220,166`; cristalino
  usa `62,668`;
- pesos `1fr:2fr`: vanila `Two=82,724670`, `After=220,166`; cristalino
  `Two=42,230`, `After=77,056`;
- prefixo fixo: vanila `Lead=7,404`, `One=27,842`, `After=220,166`;
- `#set page(columns: 2)`: vanila termina ambas as caudas em `220,166`.

Estado medido em 2026-09-13T18:41:53-03:00, HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado:
47 ficheiros no diff (`3547` inserções, `348` remoções) e 284 entradas de
status.

## Decisão

1. Atualizar os L0 proprietários de `sequence.rs` e `columns.rs` antes do código.
2. Parametrizar internamente a base do preflight: normal ou cursor-relativa.
3. Criar entrada de Sequence exclusiva para segmento explícito, cobrindo
   Sequence, singleton e vazio sem reconstrução da árvore.
4. Reservar guarda de arredondamento proporcional à coordenada para a cauda
   não cruzar o fundo por um ULP após recomposição.
5. Medir texto direto anterior ao primeiro `fr` desde o topo de entrada até o
   início real do Block: top-edge inicial pendente + avanço de `flush_line` +
   fronteira positiva `par.spacing - par.leading`; realizar a mesma fronteira
   no fluxo segmentado.
6. Transportar um `Styled` exterior pelos segmentos sintéticos de page columns.
7. Fazer `layout_segmented` usar essa entrada por coluna.
8. Preservar P1374–P1378, footnotes e fail-closed fora do escopo.

Correção interna de paridade, sem mudança pública ou de fase; fluxo contínuo
autorizado pelo ADR-0127.

## Verificação de propagação aos sub-layouts — ADR-0124

- [x] `grid`: continua barrado por região de célula; `Unknown`.
- [x] `box`: continua barrado por subframe; `Unknown`.
- [x] `place`: continua barrado por subframe; `Unknown`.
- [x] colunas: segmentos explícitos de `#columns()` e de page columns são alvo.

## RED → GREEN

- RED: testes focais para segmento simples, pesos independentes, offset e
  preservação do modo comum/footnote fail-closed;
- GREEN: P1374–P1379 e matriz bilateral basic/offset/weights/fixed-prefix/
  page-columns mais controle P1378;
- ataques: base da página inteira após offset, soma global de pesos, abrir
  todo `column_mode`, reutilizar distribuição entre irmãos e alterar contínuo.

## Memória

Tudo serial. RED/GREEN focal sob 4 GiB, debug de testes desligado e ficheiro
máximo 512 MiB. Check sob 3 GiB. Matriz caso a caso sob 2 GiB, ficheiro máximo
16 MiB e timeout 45 s. Nenhuma suíte ampla.
