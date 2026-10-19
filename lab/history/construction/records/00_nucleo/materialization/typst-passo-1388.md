# Passo 1388 — boundary do prefixo após terminador de linha

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Materializar o par uniforme 20→20 corrigindo a perda da boundary já certificada
quando whitespace terminal fecha a linha antes do Block fracionário.

## Medição anterior à decisão

Medição bilateral e variação de largura em 2026-09-14T10:04:56-03:00:

- fixture 20→20 identificador administrativo `7dcf3fbf...`, coluna útil 85pt;
- conteúdo medido: `Lead=38,76pt`, espaço exterior=2,75pt,
  `More=43,08pt`, total visível 84,59pt;
- vanila: uma baseline, `One=33,763990`, `After=Tail=220,166`;
- cristalino: uma baseline visível, mas `One=27,714`; o prefixo só ativa com
  página 205pt, cuja coluna oferece 87,5pt;
- fixture 21→21 identificador administrativo `2fce86f1...`: o vanila quebra `More` para uma segunda
  baseline, logo é controle negativo de wrap real.

Estado: HEAD `8ceea20f92dff06b6ca5a34828135c0fd3e35248`, working tree não
commitado e agregado, 47 ficheiros no diff (`5192` inserções, `419` remoções) e
383 entradas de status. Binários: vanila `eb60986b...`, cristalino
`b361893b...`. As sondas foram seriais sob 2 GiB/45 s.

## Decisão

1. Atualizar primeiro o L0 proprietário de Sequence.
2. Ao alcançar o primeiro Block certificado, fechar `current_line` apenas se
   ainda estiver aberta, mas aplicar a boundary do prefixo exatamente uma vez,
   mesmo quando whitespace terminal já chamou `flush_line`.
3. Não alterar a medição horizontal nem o layout ordinário.
4. Preservar `Unknown` para 21→21 e todo wrap, delta adicional ou forma opaca.

Correção interna de paridade em fluxo contínuo pelo ADR-0127.

## RED → GREEN, ataques e memória

O RED comportamental compara duas fontes equivalentes em 16→16. Na primeira,
whitespace terminal fecha a linha antes do Block; na segunda, o Block chega
com a linha ainda aberta. Antes da implementação, `One=36,05pt` contra
`42,10pt`, provando que o estado incidental de `current_line` apagava a
boundary. O GREEN torna as posições iguais.

Quatro candidatas horizontais foram rejeitadas por não alterarem o binário
produtivo: estilo local, checkpoint vazio, largura total/checkpoint e
whitespace-only. O ataque diferencial `leading-only`/`trailing-only` levou à
causa final. Mutantes mantêm a boundary condicional, duplicam-na quando a linha
está aberta, aplicam-na sem distribuição, aceitam 21→21 ou regredem os
controles P1387/P1386.

Execução serial: matriz sob 2 GiB e timeout 45 s; testes e link sob 4 GiB,
debug desligado e ficheiro máximo 512 MiB; check/lint sob 3 GiB. Budget de duas
revisões focais; duas falhas da mesma causa reabrem o desenho.
