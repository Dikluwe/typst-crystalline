# Passo 1318 — posição textual nos erros de CSV em Bytes

Estado: escrito e implementado; validação e revisão independente concluídas.

Acrescentar `at linha:coluna` à causa de parsing em `csv(Bytes)`, preservando
o ordinal do registro, a causa Utf8 e a origem do argumento. A posição usa
o offset do parser, não a localização do byte inválido nem uma linha CSV
reconstruída. Texto válido e binário inválido exigem regras distintas,
incluindo o caso CRLF medido. Path/Str e decode_csv público ficam intactos.

Medição: `00_nucleo/diagnosticos/p1318-measurement.json`. O passo coordena;
o owner normativo é `00_nucleo/prompts/compiler/stdlib/loading.md`, consumer
único `01_core/src/compiler/stdlib/loading.rs`.

1. L0 primeiro, revisão de escopo ADR-0127 e ownership/núcleos.
2. Testes novos e atualização explícita das expectativas nativas antigas;
   nenhuma asserção removida para facilitar GREEN. Registrar delta antes do RED.
3. Freeze A/B independente e revisão pré-patch; implementação privada local.
4. GREEN, build/workspace, lint/linhagem, A/B normal/repeat/reverse e parecer.
5. Relatório em diagnósticos com efeito real, peculiaridades medidas e limites.

Regime A/B sem atestação técnica de isolamento; sem selo geral/mutation score.
Root escreve L0/testes locais/código; testador não lê candidato; revisor não
edita material julgado. Uma medição/freeze e execução nas três ordens;
ajustes focais antes de repetir corpus; duas revisões sem ganho na mesma
causa exigem rever desenho. Unknown bloqueia.

Preservar os artefatos P1315/P1316/P1317 não commitados. Sem stage/commit/push
ou limpeza. Target exclusivo `/tmp/p1318-target.eAgQwp`, cópia independente
do cache P1317; RAM livre insuficiente para cópia completa.

## Resultado

O decoder compartilhado mantém um único parser e habilita a composição da
posição somente para Bytes. L0 ressellado, sem mudança da API pública.
RED: oito falhas de sufixo; GREEN: 67 testes passam. Workspace: 6.659 passam,
zero falhas, três ignorados. Build, fmt, diff-check e linhagem aprovados;
lint sem errors, com 240 warnings e 1.137 infos. A/B: 6.156 comparações nas
três ordens, zero falhas/Unknown; parecer independente PASS.

Números, estado não commitado, hashes, horários, peculiaridades CRLF/Utf8,
incidente do writer e limites constam em
`00_nucleo/diagnosticos/p1318-final-report.md`. O PASS é somente do recorte
CSV Bytes; Path/Str e as demais dívidas continuam abertas. Sem commit.
