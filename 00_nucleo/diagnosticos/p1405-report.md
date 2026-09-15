# P1405 — verificação de `outline.entry.indented()`

Estado medido: working tree não commitado sobre
`a159b5f18ad42eab12393b4dd101c0d6f9c1b542`, em
`2026-09-15T00:24:07-03:00`. Executado sem atestação de isolamento.

## Resultado

`PASS_FRAGMENT`. O cristalino expõe `outline.entry.indented` como identidade
nativa estática e método de entry. O helper preserva Content, mede o prefixo,
trata indentação fixa, booleana, funcional 0-based e automática, aplica `gap`
e usa o lado lógico correto em RTL. O store privado agrega máximos por outline
e nível para o replay contextual. Prefixos presentes chegam ao PDF tagueado
como `/Lbl` com MCID.

O ataque bilateral rejeitou uma hipótese intermediária que guardava o parent
na entry. No candidato final, uma entry consultada fora do parent falha tanto
no cristalino quanto no vanila com `must be called within the context of an
outline`; a identidade estática produz `indented` nos dois. O caminho positivo
por show rule não recebe crédito: a injeção do parent e a seleção de
`outline.entry` pertencem explicitamente a P1406 e continuam `Unknown`.

O harness focal L1 passou 8 testes e o harness focal L3 passou 1, matando 12 de
12 mutantes contratuais. O binário foi construído sob teto virtual de 2 GiB,
um job, incremental desligado e 512 unidades de geração. As sondas tiveram
teto de 512 MiB, uma thread e timeout de 15 s. A suíte monolítica conhecida por
exceder memória não foi executada e não recebe crédito. Não ocorreu estouro de
memória. O resselo terminou sem drift e o linter com exit 0 e zero violations.
