# P1403 — verificação de `outline.entry.prefix()` e `page()`

Estado medido: working tree não commitado sobre
`b7804e4daa9d3dd823dba5d1b828e60c845cc515`, em
`2026-09-14T23:12:11-03:00`. Executado sem atestação de isolamento.

## Resultado

`PASS_FRAGMENT`. O cristalino expõe `outline.entry.prefix` e
`outline.entry.page` como identidades nativas estáticas. Entries realizadas
conservam, em dados privados, o prefixo semântico hierárquico e a Location do
elemento fonte. Entries diretas não inventam esses dados.

As seis sondas bilaterais coincidiram com o vanila ratificado `586e1bd43`:
identidades `(prefix, page)`, prefixo `[1.]`, ausência `none`, página romana
`[I]`, segunda página `[2]` e prefixos hierárquicos `([1.], [1.1.])`.
Argumentos excedentes falham; entry direta sem Location falha com diagnóstico
e dica de show rule.

O ataque romano encontrou e corrigiu uma perda anterior no pipeline: o caminho
sem blocos `context` devolvia posições mas descartava o `PageStore` já
realizado. Depois da correção, `location.page-numbering()` retorna `"I"` em
`--in`. A leitura de `entry.page()` também participa do vocabulário causal do
fixpoint; uma sonda paginada contextual renderizou `II` na segunda página.

O build terminou com teto de 2 GiB, um job, incremental desligado e 512
unidades de geração. Cada sonda teve teto de 512 MiB e timeout de 15 s. A suíte
monolítica conhecida por exceder memória não foi executada e não recebe
crédito. O resselo terminou sem drift e o linter com exit 0 e zero violations.
O catálogo negativo matou 9 de 9 mutantes contratuais.

O veredito não cobre `inner`, `indented`, aplicação de entries em
`#show outline.entry` nem paridade geral da linguagem; essas parcelas continuam
declaradas para P1404–P1406.
