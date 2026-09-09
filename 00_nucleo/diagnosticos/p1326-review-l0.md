# P1326 — auditoria L0 pré-candidato

Predecessor: `p1326-review-preflight.md`. Manifesto verificado por SHA-256
`800db40f2300d0fda0275d06365c5f3019ca0848577ee54a9c834975b231146f`;
baseline verificado `3198d97b15c02085af158e88e44c4025e4f76b29654e7309c2ca21fdb2a41f79`.
L0 integral após amendment, SHA-256 raw
`179e15f125bc867ae9677768a741e63a003d44e67941e7af7d850d9fd7b3725d`;
manifesto declara normalizado
`4b3789c3decc0463ba553937ce7595c3a9c0ecc5fae545726ed009ad928d7451`.
Comparação literal via Node entre owner atual e original_owner do baseline
retornou true nesta fase: nenhum candidato existia durante a auditoria.

Amendment P1326 mediu antes de classificar, distingue língua/mecânica,
delimita inferência e refutadores, preserva Unknown e não atribui intenção
histórica ao vanilla. Autoriza somente Closure subjacente por With,
mensagem user-defined e span field-only quando o lookup é alcançado.
API, entidade, phase/default, Plugin/Element, guard text e predispatch
permanecem fora. Ordem de argumento anterior ao erro é residual explícito,
sem alegação de paridade. Classificação contínua ADR-0127 sustentada.

Sucessão expressa P1311/P1324 cobre expectativas Closure embutidas; a regra
específica P1326 prevalece sobre preservações genéricas anteriores somente
nesse fragmento. Testes Element/Plugin não podem ser removidos ou relaxados.
Não é necessário alterar L0/consumer eval/tests ou Func.

Aceito o L0 como entrada de testes independentes e futuro candidato após
freeze/RED e gate de linhagem aplicável. Não é veredito GREEN nem selo.
Regime continua A/B executado sem atestação técnica de isolamento.
