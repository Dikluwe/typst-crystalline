# P1337 — vínculo privado do plano ao módulo sucessor pré-C

O plano original de SHA-256
`b555a0bec8abebc1b0a51cf85b9f21bea374076303067ac1bf84a857e48cca4d`
e runner de SHA-256
`29e3abd140c830ed44886792f3148bcd325abbed84dcb793fd21b7c0e074a62c`
ficam intactos, assim como famílias, oráculos, perfil e orçamento.

Li integralmente o módulo sucessor independente
`p1337-tests-module-r1.rs.txt` SHA-256
`66c306fbfb937b0b1ce816c2297cc40c35243a9a2acabbcf2bd0a5594e966982`
e seu freeze `p1337-tests-freeze-r1.json` SHA-256
`6344fe48865ea4318fdca7fe7a3d73963a3a1c9db8b6e47714d7ca2238564124`.
O novo `p1337_preserve_array_ast_message_and_total_span_debt` está dentro de
`mod p1337_tests` e sob `#[test]`, portanto é alcançado pelo filtro congelado.
Ele chama o helper AST nos quatro perfis e exige mensagem baseline e span
integral `(1, 2).missing`. M5 altera apenas a âncora para field-only; essa
assertion agora discrimina a família sem depender de comparação CLI mutante.

Os demais testes abrangem lookup Bool/None/Auto com spans recebidos, AST das
categorias inclusive Unicode/multilinha e positivos de valores/type/repr.
M1 deve falhar em nome; M2/M3/M4 em span; M5 na sentinela Array. Isto ainda é
previsão a comprovar por controle C e mutantes realmente recompilados.

Antes da preparação, o trecho de testes de C será comparado com esse módulo
congelado, tolerando somente separadores de linha externos ao módulo; durante
todas as mutações, o trecho extraído fica byte-idêntico e seu hash é registrado.
Não alterei ou escrevi testes, oráculos, implementação ou artefatos alheios.
Nenhum candidato foi lido, nenhum Cargo adversarial rodou nesta revisão.
Novo RED do sucessor continua responsabilidade do operador antes de C.

Delta de observabilidade pré-C: uma família previamente sem testemunha local
executada passa a possuir uma testemunha sob o mesmo filtro. Não se atribui
kill antecipado. É a primeira revisão da causa, sem mudança de intenção,
ampliação de escopo ou convergência artificial de dívida Array.
