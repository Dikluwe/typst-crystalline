# P1332 — RED revisado

**Gate RED aprovado; pode iniciar C restrito ao registro autorizado no L0.**
Recibo `p1332-unit-red.json`, SHA-256
`ce2f87b33ee14a6798ffbca953b9ffe2b0fb5d4ee58b8f5151e077440ee08750`,
vinculado ao manifesto
`b32fa0ac39b79d7a1b7f23f62540011b58ac68e20f0c850009f688b68b412084`.

O comando registrado compilou e executou a suíte focal: 28 aprovados e
sete falhas, exit 101. As sete testemunhas são exclusivamente nome
`calc.abs` versus `abs`: seis em Tracepoint e uma em Func::name. Incluem
os quatro helpers históricos e três testes P1332. Não houve falha de
compilação, fixture ou controle preservado; identidade/repr/resultado
P1332 passaram. O GREEN deverá percorrer também os casos posteriores aos
primeiros asserts falhos de cada loop.

Proveniência: HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree
não commitado, inventários e diff/stat antes/depois no recibo, execução
de `2026-09-09T14:41:32.210313+00:00` a
`2026-09-09T14:43:31.789371+00:00`. Os inventários antes/depois são iguais.
A revisão repetiu somente a auditoria read-only do freeze/integração em
`2026-09-09T14:43:59.366Z`: inputs e snippets íntegros, runtime pré-C
reconstruído igual ao baseline salvo header. Não repetiu testes globais.

Este parecer autoriza a próxima fase causal, não aprova C ou fechamento.
Limites e observáveis permanecem os de `p1332-review-pre-c.md`; A/B sem
atestação técnica de isolamento. Nenhum artefato julgado foi alterado.
