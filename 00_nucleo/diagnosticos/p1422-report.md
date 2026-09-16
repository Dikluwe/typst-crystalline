# P1422 — relatório da campanha integral das 66 rotas

## Veredito

**PASS diagnóstico**, executado sem atestação de isolamento. O runner classificou
conclusivamente as 66 rotas congeladas: todas ficaram `PARTIAL`, nenhuma ficou
`UNKNOWN`. Isso fecha a campanha P1422, mas não declara paridade das rotas.

Estado medido em `2026-09-16T12:37:39-03:00`: HEAD
`befed6192746106a896f3cf6d47d61d9c40a13ed`, tree
`67f927a62b7edbd3ac617875f617670dfff20519`, produto causal
`fc803d05f0e7eec7f6bc6a8e363e3cfe1450077d` / tree
`11a404824b77d608b1eda29ad44fb281d89aef03`. Binários: cristalino
`26f1534c7d19b8b8d8dcb320684966dc4e4cdcf27c5263c4eb68b73706b3e860` e
vanilla ratificado `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`.

## Resultado material

- 44 rotas `html.*`: forma qualificada `ABSENT`, alternativa `MATCH`, razão
  `QUALIFIED_ABSENT_ALTERNATIVE_MATCH`.
- 22 rotas `math.*`: forma qualificada `ABSENT`, alternativa `DIFFERENCE`, razão
  `QUALIFIED_ABSENT_ALTERNATIVE_DIFFERENCE`.
- Três ordens produziram o mesmo vetor
  `4557b5bf150d317b200680dfe751d651e3a571ca3fbe1e87983c6f9f3cbd5033`.
- A repetição integral independente em `/tmp/p1422-full-repeat.json`, SHA-256
  `82b19558b7391a273ac3c187482f9f542c88b877cf5d7f6b8e46c416f0c2f2bd`,
  repetiu exatamente `66 PARTIAL`, `44 MATCH`, `22 DIFFERENCE`, zero `UNKNOWN`.

O focal `html.area`/`math.abs` passou antes da campanha integral nas três ordens;
seu recibo tem SHA-256
`4b6635cf45ccc9523a75522e06cccb9498cecad253087afe50440b241cb1e284`.

## Integridade e recursos

`py_compile`, `--validate-only`, `--gate`, formatação, lint cristalino e
`git diff --check` passaram. A validação confirmou 66 rotas, 154 probes de rota
por ordem e 12 controles. O gate rejeitou 27/27 mutantes válidos
(`mutation_score = 1.0`), preservou 8 controles positivos e classificou 9
opacos como `Unknown`.

Foram auditados 996 recibos de processo nas três ordens: zero `--jobs`, zero
`eval --target`, zero timeout, zero truncamento e zero swap. O maior stdout teve
99 bytes, o maior stderr 513 bytes, ambos abaixo de 64 KiB. Todos os hashes
selados P1420/P1421, inventário, produtos e entradas P1422 permanecem iguais.

## Próximo passo único

Executar **P1423 — materializar as 44 funções tipadas qualificadas restantes no
módulo `html`**, com owner
`00_nucleo/prompts/compiler/stdlib/html.md` e consumer
`01_core/src/compiler/stdlib/html.rs`. Essa é a maior causa homogênea e sua
alternativa `html.elem` já coincide bilateralmente; o eixo math permanece
separado porque combina ausência nominal com divergência pública da sintaxe.

Detalhes de agrupamento estão em `p1422-backlog.json`. O resultado integral é
`p1422-results.json`, SHA-256
`b6501f6d9778d6fb4dae0c3b5badd7c45d31c6ecc892f6eda75df31cec4d506b`.
