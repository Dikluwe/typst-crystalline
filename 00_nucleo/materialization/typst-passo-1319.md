# Passo 1319 — localizar falhas de CSV em arquivos UTF-8 inválidos

Estado: implementado e validado; PASS no recorte delimitado.

O commit `d31047d7b8af7837c84adae4ded3d2ff50c62093` integra os passos
P1315–P1318 anteriormente validados. Este passo trata somente o diagnóstico
de parsing CSV Path/Str quando o buffer inteiro é UTF-8 inválido: acrescentar
caminho virtual/posição e conservar a origem do argumento. Erro vencedor e
ordinal não mudam, mesmo com byte inválido posterior a UnequalLengths.

O caso de arquivo UTF-8 válido exige identidade externa ausente no retorno
de World.read_path; permanece separado. Não usar include_path como atalho.
Medição e explicação substantiva ficam em diagnósticos. O L0 proprietário é
`00_nucleo/prompts/compiler/stdlib/loading.md`, consumer único loading.rs.

1. Commit anterior, medição bilateral real e preflight V15/V26.
2. L0 primeiro, testes locais RED e freeze A/B independente; revisão pré-patch.
3. Implementação privada no owner, um parser e uma leitura, sem API nova.
4. GREEN, build/workspace, lint/linhagem/fmt, A/B em três ordens e revisão final.
5. Relatório com limites Path válido/resolução/validação e proveniência exata.

Regime A/B sem atestação técnica de isolamento, não selo/mutation score.
Root escreve L0/testes locais/implementação; testador não lê candidato;
revisor não altera artefatos julgados. Unknown bloqueia. Uma medição/freeze
com correções focais antes de corpus final; duas revisões sem ganho na mesma
causa exigem rever desenho. Não alterar evidências anteriores.

Target exclusivo `/tmp/p1319-target.VqXtmj`, cópia sem hardlinks do P1318.
RAM insuficiente para copiar o cache completo; baseline preservado.
Commit solicitado foi aplicado ao trabalho anterior; esta implementação
fica para revisão, sem push ou limpeza de temporários.

## Resultado

Diagnóstico binário Path/Str implementado no owner, com causa preservada,
caminho virtual e origem do argumento. RED confirmado antes da implementação;
69 testes locais e 6.661 testes do workspace passaram. A/B independente:
7.740 comparações aprovadas, sem falhas/Unknown. Build, fmt e diff-check passam;
lint sem erros, com 240 warnings e 1.137 infos, não ausência de dívida.

O linter apresentou falso-negativo no hash reverso. A metadata foi corrigida
e a linhagem bidirecional calculada explicitamente, também pelo revisor;
a ferramenta externa não foi alterada. Parecer final PASS com limites.

Proveniência exata, hashes, comandos, horários e fronteiras restantes:
`00_nucleo/diagnosticos/p1319-final-report.md` e
`00_nucleo/diagnosticos/p1319-review-final.md`.
As medições finais pertencem ao working tree não commitado sobre o commit
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, não ao commit isolado.
