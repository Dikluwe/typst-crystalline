# P1370 — relatório final da retificação S02

## Veredito

**MATCH_WITH_LIMITED_ATTESTATION.** Com o oráculo causalmente retificado para
“CLI explícita vence o setting do documento”, as observações brutas P1369
produzem quatro `MATCH`: `default`, `html`, `a11y-extras` e `html+a11y`.
Vanila e cristalino preservam `(true,true,true)` em todos os perfis.

O P1368/P1369 histórico não foi reescrito. O reclassificador ignora o campo
derivado `profiles` do P1369, que carregava o expected refutado, e reconstrói
os verdicts exclusivamente das 24 linhas de observação bruta.

## Poder discriminatório

O RED retornou exit 1 porque o módulo candidato ainda não existia. O GREEN
executou 1/1 teste em 0,000 s. O teste rejeitou os sete vetores booleanos
diferentes de `(true,true,true)`, mutation score 7/7 = 1,0, e manteve
observação incompleta e matriz estruturalmente incompleta como `Unknown`.

## Integridade e memória

O input possuía exatamente 31.718 bytes e o SHA-256 congelado
`1e6c38c2…ed7b05`. O resultado tem 2.294 bytes e SHA-256
`3e0b35ab…246ce`. O teto de leitura e escrita foi 65.536 bytes; a execução leu
o input uma vez, sem repetição, concorrência ou acumulação de canais.

Não foram executados Cargo, Typst, `pdfinfo`, watch, rede ou suíte ampla. Não
houve timeout, OOM, SIGKILL nem falha de alocação. Como não houve subprocesso
de produto, não existe medição nova de RSS; a evidência dinâmica continua
sendo a P1369 pinada.

## Regime e limite da alegação

Protocolo completo executado sem atestação de isolamento: contrato, teste,
implementação e veredito compartilham a autoridade da sessão e o workspace.
O resultado fecha somente o vetor focal S02 nos binários/perfis congelados;
não prova paridade geral de PDF, precedência de outros campos ou comportamento
de builds futuros.

Os gates finais `py_compile`, validação JSON pela biblioteca padrão,
`git diff --check` e `crystalline-lint .` retornaram exit 0. O lint apresentou
somente warnings/infos preexistentes; sua saída foi truncada pelo limite de
visualização.
