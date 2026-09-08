# Passo 1315 — ordinal correto no erro de campos CSV

Estado: implementado e validado — PASS restrito ao P1315, em 2026-09-08.

O trabalho até P1314 foi commitado em `bc8213f36b7a29b4fdc30cfc74ddc23586117c64`.
Este passo não é L0. O owner normativo é
`00_nucleo/prompts/compiler/stdlib/loading.md`, consumer único
`01_core/src/compiler/stdlib/loading.rs`.

## Problema medido

Com primeiro registro `"a\nb",c` seguido de `1`, o cristalino informa
`found 1 instead of 2 fields in line 3`; o vanilla ratificado `a51e02804`
informa ordinal `line 2` e, separadamente, posição física `at 3:1`.
Medição e proveniência: `00_nucleo/diagnosticos/p1315-measurement.json`.

## Execução

1. Atualizar L0 antes do código: corrigir apenas ordinal UnequalLengths.
2. Congelar A/B independente derivado desse recorte; revisar antes do patch.
3. Acrescentar testes locais e provar RED; implementar e provar GREEN.
4. Build e testes workspace, lint/linhagem, preservação de opções CSV e
   controles de outros loaders; registrar os resultados em diagnósticos.

Regime A/B sem atestação de isolamento técnico. Root faz L0/implementação;
testador independente congela oráculos sem ler candidato; revisor não edita
artefatos julgados. Não há selo de refinamento ou promessa de paridade geral.
Budget proporcional: uma medição/freeze e uma execução final normal/repeat/reverse;
falha focal é corrigida antes de repetir corpus. Duas revisões sem progresso
na mesma causa exigem reabrir o desenho, não enfraquecer expectativas.

Preservar APIs, valores, parser, opções, I/O e todos os demais diagnósticos.
Sufixo de posição, spans e UTF-8 são dívida distinta, explicitada no relatório.
Temporários podem usar RAM autorizada; não sobrescrever o binário P1314.
O pedido autoriza commit do pendente anterior; o novo P1315 será entregue
implementado sem commit adicional, salvo nova instrução.

## Fechamento

Correção do ordinal implementada após L0 e freeze A/B, com RED→GREEN.
6.648 testes workspace passaram; 3.480 comparações A/B aprovadas; lint sem
erros. Revisão independente PASS no recorte. Evidências, proveniência,
hashes e dívidas restantes em `00_nucleo/diagnosticos/p1315-final-report.md`.
Nenhuma API ou valor CSV alterado. P1315 sem commit adicional.
