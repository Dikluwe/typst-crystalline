# P1331 — freeze R2 e sentinela pré-C2

Revisor `/root/p1331_review`, A/B sem atestação de isolamento, sem refinamento.
Este parecer sucede a reabertura registrada em
`p1331-review-green-r1-reopen.md`; R1/C1 permanecem evidência histórica.

Manifesto R2 SHA-256:
`6383d89ef0fccf78290182c1180fccaba290c1a11f36f4fe44ba75df53de81c9`.
Freeze R2: `d70f809014a912e337006b5751bc058eb1c184879e37b125070456afefc127a7`.
Snippet R2: `de3d3797484eb721bc3af3e7f9b259db43feca596d00dc9c8e8af2fc0cb91277`.
Integração R2: `998e582c3452488d11c88fa319652e635025957bd81595a0e909fb93e294e1fa`.
Norma L0 continua `ef406128181e4cfa5b802b86ec0a9c58370cc4f7dab09365a088eef74523956a`.

Auditoria programática em `2026-09-09T14:01:39.759Z` reconstruiu o owner
pré-C a partir do baseline e dos snippets vigentes: igualdade integral,
exceto header de linhagem; artefatos R1 e R2 intactos; as 504 expectativas
CLI permanecem iguais às de R1. O diff do snippet contém só resolver virtual
puro no TestWorld e uma sentinela nova; as sete funções de teste anteriores,
o caso Path e todas as asserções comportamentais foram preservados. O runner
R2 muda somente bindings de metadata para manifesto/expectativas R2.

O resolver da fixture atende ao FileId do main e constrói VirtualPath e
RootedPath em memória. A sentinela avalia `path("p1331.typ")` pela API pública
com esse mesmo TestWorld, sem chamar abs, verificando Value::Path, Project,
caminho `/p1331.typ` e ausência de warnings em cada perfil. O método resolve_path
usado pela avaliação de expressão delega ao mesmo World empregado pelo teste
de documento completo: não é prova obtida somente por outro binário CLI.

Recibo `p1331-fixture-path-r2.json`, SHA-256
`650ed1db7aaeba151033fc962a0738d50bb93b4f7b6682616f401385c711b218`:
comando cargo release/locked, filtro exato
`compiler::stdlib::calc::p1331_tests::p1331_fixture_path_constructor_succeeds_before_abs`,
executado entre `2026-09-09T14:00:35.201752+00:00` e
`2026-09-09T14:02:27.841571+00:00`. Compilou e passou um teste, zero falhas,
com loop dos quatro perfis. Inventários before/after iguais à integração R2.
Esses recibos contêm a working tree não commitada e diff/stat sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`.

Ganho discriminatório concreto: a construção Path no World da fixture agora
é observada separadamente e passa antes de C2. O custo focal evitou repetir
o corpus CLI; nenhuma expectativa foi enfraquecida. A insuficiência R1 foi
resolvida na primeira revisão desta causa.

Veredito: FIXTURE_R2_ACCEPTED. Pode executar RED dos testes R2 no runtime
pré-C. C2 ainda depende de novo RED compilado e da integridade congelada.
