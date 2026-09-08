# Passo 1316 — apontar a origem do CSV fornecido em Bytes

Estado: escrito, implementado e validado; revisão independente PASS no recorte. Sem commit.

Na chamada `csv(bytes("a,b\n1"))`, o erro atual não aponta para o argumento
que contém o CSV. With/Args também perdem essa origem no consumer. Corrigir
essa rota, sem atribuir a um argumento erros que deveriam apontar para um
arquivo externo. Medição: `00_nucleo/diagnosticos/p1316-measurement.json`.

O passo coordena; não é L0. A obrigação proprietária está em
`00_nucleo/prompts/compiler/stdlib/loading.md` para o consumer único
`01_core/src/compiler/stdlib/loading.rs`.

1. Medir Bytes versus Path/Str, atualizar L0 e revisar enquadramento ADR-0127.
2. Congelar A/B sem acesso ao código candidato; comprovar RED local.
3. Aplicar correção apenas na composição nativa Bytes; provar GREEN.
4. Build/testes workspace, lint/linhagem, replays e revisão independente.
5. Relatar efeito real, provas e dívidas restantes em diagnósticos.

Preservar decoder puro, textos dos erros, ordinal P1315, tipos aceitos,
opções, ordem de validação, I/O, Path/Str e outros loaders. Args sintético ou
origem detached não recebe origem inventada. Não alterar os testes anteriores.

Regime A/B, sem atestação de isolamento técnico. Root é autor de L0/código;
testador independente congela expectativas; revisor julga sem editar material
julgado. Não há selo geral ou mutation score. Uma medição/freeze e execução
normal/repeat/reverse; falhas focais são tratadas antes de repetir corpus.
Duas revisões sem ganho na mesma causa obrigam rever desenho, não afrouxar gates.

P1315 e seus registros não commitados permanecem preservados. Não fazer
stage/commit/push neste pedido. Usar target temporário próprio sem sobrescrever
baseline; RAM autorizada se houver espaço suficiente, senão /tmp.

## Entrega

A origem do erro de parsing em `csv(Bytes)` agora usa o primeiro argumento
posicional, inclusive quando transportado por With/Args. Mensagens, decoder
puro, Path/Str e precedência legada permanecem preservados.

RED estável seguido de GREEN; build e 6.652 testes passaram (três ignorados),
3.960 comparações A/B passaram e lint terminou sem erros (240 warnings).
Proveniência, recibos, revisão independente e limites estão em
`00_nucleo/diagnosticos/p1316-final-report.md`. Não há paridade geral CSV
nem atestação técnica de isolamento. P1315 e seus registros foram preservados.
