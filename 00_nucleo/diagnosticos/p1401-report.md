# P1401 — verificação de `eval --in`, `before` e `after`

Estado medido: working tree não commitado sobre
`1333b159fae3925711b5739a8c644f60d113d223`, em
`2026-09-14T21:53:37-03:00`. Executado sem atestação de isolamento.

## Resultado

`PASS_FRAGMENT`. O cristalino aceita input físico e stdin em `eval --in`,
compila o documento paginado e avalia a expressão com seu introspector. Os
métodos `selector.before` e `selector.after` preservam a primeira fronteira,
o default `inclusive: true`, a forma explícita falsa, repr e árvore estrutural.

Sete sondas bilaterais coincidiram com o vanila ratificado `586e1bd43`,
incluindo uma fixture com duas fronteiras que distingue primeira de última.
O teste focal de L2 passou. O build do binário passou com um job, teto virtual
de 2 GiB, debug metadata desligado e linker gold; cada sonda teve teto de
512 MiB e timeout de 15 s. Swap total era zero. O harness monolítico de testes
de L1 excedeu o teto fixo e foi abortado; o limite não foi elevado e esse
harness não recebe crédito de sucesso.

O linter terminou com exit 0 e zero violations após o resselo. O catálogo
negativo matou 8 de 8 mutantes contratuais. O veredito não se estende a
`eval --target`, HTML contextual, construtores de selector ainda ausentes nem
à paridade geral da linguagem.
