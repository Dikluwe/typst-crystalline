# Passo 1380 — prefixo inline composto antes de `fr` segmentado

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Tratar como uma única linha fixa prefixos inline simples, estilizados ou
compostos que antecedem o primeiro Block `fr` de uma coluna explícita.

## Medição anterior à decisão

Em página 200pt × 240pt, margem 10pt, colunas de 85pt e fonte 11pt:

- Styled: vanila `Lead=7,404`, `One=27,842`, `After=220,166`; cristalino
  P1379 pagina `After` para `7,404` da página seguinte;
- Strong: vanila `Lead=7,261`, `One=27,699`, `After=220,166`; cristalino
  pagina `After` para a página seguinte;
- composto `Lead` + Styled `More`: vanila mantém ambos em `7,404`, usa
  `One=27,842` e `After=220,166`; cristalino usa `One=21,792` e
  `After=201,576`.

Medição em 2026-09-13T19:55:33-03:00, HEAD
`8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não commitado:
47 ficheiros no diff (`3969` inserções, `350` remoções) e 295 entradas de
status. Binários: vanila `eb60986b...` e cristalino `4be48896...`.

## Decisão

1. Atualizar primeiro o L0 proprietário de `sequence.rs`.
2. Reconhecer iterativamente apenas prefixo inline sem wrap composto por
   Text/Space/Empty e wrapper único Styled/Strong/Emph sobre Text.
3. Rejeitar variações verticais, canal custom, wrapper aninhado e largura que
   exceda a região.
4. Contar o prefixo como uma linha fixa: top-edge inicial + um único flush +
   fronteira `par.spacing - par.leading`.
5. Aplicar a mesma fronteira uma vez antes do primeiro Block fracionário.
6. Preservar P1374–P1379 e todos os contextos não segmentados.

Correção interna de paridade, sem contrato público, default ou fase nova;
fluxo contínuo permitido pelo ADR-0127.

## RED → GREEN

- RED: Styled, Strong e composto reproduzem as divergências medidas;
- GREEN: os três casos, P1379 direto e família P1374–P1380;
- ataques: somar uma linha por fragmento, ignorar estilo do primeiro texto,
  aplicar fronteira duas vezes, aceitar wrap, globalizar a regra e regredir
  o prefixo direto.

## Memória

Testes/link seriais sob 4 GiB, debug desligado e ficheiro máximo 512 MiB.
Check/lint sob 3 GiB. Matriz caso a caso sob 2 GiB, ficheiro máximo 16 MiB e
timeout 45 s. Nenhuma suíte ampla e nenhuma recursão/coleção nova no preflight.
