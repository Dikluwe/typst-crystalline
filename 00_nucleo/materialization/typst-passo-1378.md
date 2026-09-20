# Passo 1378 — origem lógica dos segmentos explícitos de colunas

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Dar a cada segmento separado por `colbreak()` a mesma origem vertical lógica
e fazer a saída do contentor seguir o segmento mais profundo. Altura
fracionária segmentada permanece fora deste passo.

## Medição anterior à decisão

Em página 200pt × 240pt, margem 10pt e fonte 11pt:

- topo: vanila `One=Two=7,404`, `Outside=27,842`; cristalino P1377
  `One=17,238`, `Two=10,000`, `Outside=30,438`;
- após `Intro`: vanila `One=Two=27,842`; cristalino `37,676` e `30,438`;
- segmento desigual: vanila `Lower=21,792`, `Outside=42,230`; cristalino
  `Lower=31,626`, mas `Outside=30,438`, portanto a saída sobrepõe a coluna;
- primeiro segmento vazio: vanila mantém `Two=7,404`.

Estado medido em 2026-09-13T17:49:43-03:00, HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado:
47 ficheiros no diff (`3383` inserções, `343` remoções) e 273 entradas de
status.

## Decisão

1. Atualizar o L0 proprietário de `columns.rs` antes do código.
2. Remover o ascender pré-somado de `layout_segmented`; o Cursor continua
   dono de fixar a baseline inicial.
3. Restaurar, para cada segmento irmão, os snapshots escalares da entrada.
4. Reter como baseline de saída a maior baseline de segmento não vazio.
5. Preservar P1374–P1377 e manter `fr` segmentado como `Unknown`.

Correção interna de paridade, sem mudança pública ou de fase; fluxo contínuo
autorizado pelo ADR-0127.

## Verificação de propagação aos sub-layouts — ADR-0124

- [x] `grid`: não participa de `layout_segmented`; permanece `Unknown` P1376.
- [x] `box`: não participa desta origem irmã; permanece `Unknown` P1376.
- [x] `place`: não participa desta origem irmã; permanece `Unknown` P1376.
- [x] colunas: forma segmentada é o alvo; contínua é controle P1377.

## RED → GREEN

- RED: três testes focais — origem comum, saída pela coluna profunda e
  primeiro segmento vazio;
- GREEN: P1377/P1378, matriz bilateral top/offset/uneven/empty e controle
  contínuo;
- ataques: manter ascender, vazar flag inicial, usar último segmento, contar
  vazio e abrir `fr`.

## Memória

Tudo serial. RED/GREEN focal sob 4 GiB, debug de testes desligado e ficheiro
máximo 512 MiB. Check sob 3 GiB. Matriz caso a caso sob 2 GiB, ficheiro
máximo 16 MiB e timeout 45 s. Nenhuma suíte ampla.
