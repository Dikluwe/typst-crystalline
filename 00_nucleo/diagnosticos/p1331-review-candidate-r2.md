# P1331 — C2 e GREEN R2

Revisor `/root/p1331_review`, A/B sem atestação de isolamento, sem refinamento.
Nenhum produto, L0 ou oráculo foi editado pelo revisor.

Recibo GREEN R2: `p1331-unit-green-r2.json`, SHA-256
`d6b2b6df18245f1b39d844d9aef52da42db76b1705aa5760b7c324f60ad2ee67`.
Execução entre `2026-09-09T14:05:14.431022+00:00` e
`2026-09-09T14:07:26.139881+00:00`; comando release/locked
`cargo test --release --locked -p typst-core compiler::stdlib::calc::p13 -- --nocapture`.
O recibo identifica a working tree não commitada sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093` e seus inventários/diff/stat.

Conferi que a lista completa dos testes executados coincide com RED R2:
31 passaram, zero falhas e exit 0. O teste de famílias públicas agora
conclui incluindo Path, com suas asserções de mensagem/âncora/trace intactas.
A sentinela separada de construção Path também passa. Controles P1328,
P1329 e P1330 permanecem passados, assim como guard, origens, warnings,
tipos numéricos, math e limites nativos declarados no freeze.

Verificação integral do candidato: substituindo apenas o snippet congelado
R2 pelo R1 e retirando o header recíproco, os bytes de C2 produzem exatamente
o hash de C1 `35e5681ac54d5207d71329d0da0560bfdeda83f0026030474f85e8c100244c7e`.
Assim, a implementação do fallback reaplicada é a mesma já revisada; a
correção da fixture não foi acompanhada de adaptação do produto.
Hash B final: `8d7359a9585e354c370c032b573a7133d004806b3a3c84cd881f8665bb992521`.
Norma A: `ef406128181e4cfa5b802b86ec0a9c58370cc4f7dab09365a088eef74523956a`.

Inventários before/after do GREEN são iguais e identificam os bytes atuais
do owner. Conferi também todos os hashes de inputs congelados R1 e R2:
permanecem intactos. Os únicos efeitos produtivos continuam montagem local
de nomes longos Str/Bool, mensagem e value_span no fallback existente.

Veredito: CANDIDATE_R2_GREEN_ACCEPTED, no escopo normativo. R1 continua
tentativa não concluída com fixture insuficiente; R2 contém a cadeia válida
sentinela pré-C → RED → C2 → GREEN. Ainda são necessários gates gerais e
corpus CLI antes de `PASS_SCOPED` final. Não há alegação de paridade geral
ou de cobertura da produção introspectiva de Location.
