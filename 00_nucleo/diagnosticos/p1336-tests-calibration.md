# P1336 — calibração e congelamento independente

Regime A/B executado sem atestação técnica de isolamento. Autor `/root/p1336_tests`:
L0 integral, skill e referências lidas; baseline acessado para integração do harness
e APIs, nenhuma implementação candidata recebida ou lida. Escritas limitadas a
novos p1336-tests-* e módulo local p1336_tests. Nenhum Cargo executado pelo autor.

O manifesto recebido é SHA-256
`54864bfd67d50197560011d5aae055a0303fcc9cadc4230221dfd4f7a6daea88`.
O recibo p1336-tests-baseline.json preserva before/after UTC, HEAD, working tree,
diff/stat, entradas e argv, identidade SHA-256 dos executáveis, exit, stdout/stderr
texto e base64 integrais. Foi medido antes de C: 45 casos, 4 perfis, 3 ordens,
2 executáveis = 1080 execuções Observed. As saídas são estáveis nas três ordens.

Hipótese refutada na calibração: `"abc".len` sem chamada não chega ao lookup
no cristalino. Baseline devolve 3; vanilla emite erro. Como o L0 P1336 exclui
pré-despacho e gates anteriores ao lookup, esse caso muda de hipótese de convergência
para preservação de dívida antes do congelamento. A fixture e as saídas não mudam.
O teste AST equivalente passa a controle de sucesso preservado, não recebe crédito
de paridade. Lookup puro sobre Str com field len continua exigindo a mensagem
string e o span fornecido, pois o contrato local é distinto da interceptação AST.

A primeira execução gravou baseline integral e parou na asserção dessa hipótese;
uma segunda execução completa redundante terminou na guarda de arquivo existente,
sem sobrescrever o baseline. O custo extra é declarado como execução desperdiçada,
não evidência adicional. A finalização usou `baseline --finalize-baseline`, validando
identidades e fontes antes de classificar as mesmas saídas. Não houve terceira
medição completa ou alteração de expectativas para acomodar candidato.

Por perfil, ficaram 13 casos CONVERGENCE_REQUIRED, 22 PRESERVE_PARITY e
10 PRESERVE_EXISTING_DEBT. As dívidas são: Str.len como field ligado, namespaces
int/str ausentes, Bool, Array, None, Auto, Content.text e ordem de avaliação dos
argumentos panic de Int/Str. Preservá-las não implica paridade. Casos callee benignos
têm classificação própria; não são contados como reparo deste lookup.

O auditor possui controles explícitos de bytes ausentes/inconsistentes, crash,
JSON inválido e transporte deliberadamente opaco. Unknown só satisfaz o controle
planejado do auditor; nunca é resultado aceitável de caso produtivo obrigatório.

Freeze p1336-tests-freeze.json SHA-256:
`ba3f1a49fb0f96dc0770336274f230f4c2dde5ccf390c770c03a8b3244098cfa`.
Módulo Rust congelado: bytes iniciando em `#[cfg(test)]\nmod p1336_tests {`
até antes de `#[cfg(test)]\nmod p1326_tests {`, SHA-256:
`b30cc9449bef081d51d44d664aed04c4c26c48bcde15ecf4b31938a1ecfd9934`.
Foi formatado antes do freeze. Filtro: `cargo test -p typst-core p1336_tests`
(confirmar nome de package local no operador). Cinco testes; nenhuma saída RED
foi mostrada ao implementador pelo testador.

Limite: o corpus A/B CLI cobre os controles enumerados, incluindo PDF nos quatro
perfis. LocatedContent possui o teste AST legado p1325_located_tests, cuja execução
se pede como gate complementar. Text contextual e warnings requerem os gates
legados do workspace; não se declara cobertura A/B específica dessas superfícies.
Não há afirmação de equivalência funcional geral.
