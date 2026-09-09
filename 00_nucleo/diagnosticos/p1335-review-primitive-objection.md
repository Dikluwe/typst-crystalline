# P1335 — contradição L0 no nome de tipo inteiro

Medição anterior à classificação, conferida em 2026-09-09T17:18:58Z:
`p1335-sentinels-normal-r3.json`, SHA-256
`1ceb634fcb0211935e8125d6825d4f7a018ba2c4f2e129ded73bba483fdcddbb`,
identifica baseline/manifesto P1335, binários, fontes, perfis e canais integrais.
Em `p1325.integer-boundary`, expressão `(1).nope`, vanilla publica
`cannot access fields on type integer` sobre `nope`; cristalino publica
`cannot access fields on type int` sobre o acesso inteiro. Em
`p1325.string-boundary`, `"abc".nope`, repete-se a diferença string/str e âncora.
Essas diferenças existem nos quatro perfis. Não são regressões novas alegadas.

O L0 corrente `00_nucleo/prompts/compiler/eval/bindings/field_access.md`,
SHA-256 `af3c783de9f62b74907252f0ce7d078103f5769e969e43f35327d4556643ba3f`,
foi lido integralmente. Seu critério de verificação na linha 320 é explícito:
`#(1).foo → Err (nomeia "integer")`. Não se infere a promessa de uma regra vaga de
paridade; o nome exigido consta literalmente no L0. As delimitações dos reparos
P1324/P1325/P1326 preservam categorias fora daqueles recortes, mas não substituem
esse critério por uma obrigação de publicar `int`. A preservação expressa da
âncora vigente de outros targets não revoga o nome de tipo nesse critério.

O consumer, SHA-256
`4c0045831976a08e414820c831be0b0c957d522f121d3f9a59ee92e05d67d449`,
seleciona span em `field_access.rs:533` e usa `other.type_name()` em `:836`.
Já importa `vanilla_type_name` em `:28` e usa esse helper em `:851`;
`operators/error_formatting.rs:53` já mapeia Int/Str para integer/string.
O owner dispõe do target, do span individual e do formatter de nome necessário.
Não há evidência de necessidade de nova entidade, API, fachada ou fase para o
recorte de leitura direta Int/Str. A rota de métodos/callee permanece distinta.

Classificação da revisão: Int é contradição L0 em rota canônica, prioridade 2,
não apenas diagnóstico isolado prioridade 3. O classificador precisa decidir
com testemunhas se Int/Str constituem uma única coorte causal ou se deve separar
prioridades; não pode manter Int em prioridade 3 ignorando a cláusula corrente.
Se agrupados legitimamente como mesma causa, os dois paths e um owner entram no
desempate real. Se separados, só o path Int recebe a obrigação textual específica.
O risco medido e os demais candidatos permanecem parte da ordenação; este documento
não elege antecipadamente a coorte.

Refutação: uma obrigação posterior vigente que substitua expressamente o nome
integer por int; igualdade bilateral nova com a mesma fonte/binários; ou um
consumer necessário ao observável completo não disponível neste owner. Não
constitui refutação o fato de um passo focal anterior ter preservado a dívida.
Nenhum L0 ou produto foi alterado pela revisão.
