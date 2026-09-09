# P1331 — RED R2 antes de C2

Revisor `/root/p1331_review`; A/B sem atestação técnica de isolamento,
sem refinamento. Predecessor: `p1331-review-pre-c-r2.md`, incluindo a
sentinela compilada de construção Path sem abs no runtime pré-C.

Recibo `p1331-unit-red-r2.json`, SHA-256
`eac32393ec4b1c56cc61494b2397bdfbcb21a19a2c68106ea045706d40f65373`,
manifesto `6383d89ef0fccf78290182c1180fccaba290c1a11f36f4fe44ba75df53de81c9`.
Execução entre `2026-09-09T14:03:28.819241+00:00` e
`2026-09-09T14:03:29.413277+00:00`, com argv
`cargo test --release --locked -p typst-core compiler::stdlib::calc::p13 -- --nocapture`.
Working tree não commitada sobre HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`; inventários e diff/stat
completos ficam no before/after do recibo.

O executável compilado executou 31 testes: 25 passaram e seis falharam,
exit 101. As seis falhas permanecem assertions do diagnóstico novo de
fallback (cinco testes P1331 e o sucessor P1328); não há falha de compilação.
A nova sentinela Path passa, inclusive neste conjunto. P1329/P1330 e os
controles de guard e espécies permanecem passados. Não há diagnóstico de
construtor entre os witnesses falhados.

Conferi igualdade dos inventários antes/depois e com a integração R2,
além dos hashes de todos os artefatos congelados R1 e R2. O runtime continua
pré-C conforme a reconstrução integral anterior; nenhuma expectativa foi
editada. Os testes densos ainda podem parar na primeira assertion; a prova
de Path aqui é a sentinela separada, e o GREEN deverá percorrer o caso
completo de rejeição com as mesmas asserções.

Veredito: RED_R2_ACCEPTED. C2 pode reaplicar somente a mudança do fallback
definida na norma L0 vigente. Não autoriza editar fixtures/oráculos nem
fechar P1331 antes de GREEN e demais gates.
