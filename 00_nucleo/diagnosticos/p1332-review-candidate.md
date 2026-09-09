# P1332 — candidato e GREEN focal

**Candidato conforme ao recorte; GREEN focal aprovado.** Parecer parcial,
sem fechamento, em regime A/B sem atestação técnica de isolamento.

`p1332-review-candidate-audit.cjs` passou em `2026-09-09T14:48:03.116Z`.
Reverteu apenas em memória a integração de testes e o nome do registro:
o restante do owner é integralmente igual ao baseline P1331, salvo header.
O delta produtivo é exclusivamente `Func::native("calc.abs", calc_abs)`
para `Func::native("abs", calc_abs)`. Os inputs e snippets congelados
conferem; nenhuma mudança em cálculo, guards, despacho ou outro registro.

Bindings verificados: manifesto
`b32fa0ac39b79d7a1b7f23f62540011b58ac68e20f0c850009f688b68b412084`;
resselo `4728524d1deb5ecb625734540cbd953bebb57d33804b7c3aaacf9ff88f46b0d9`;
owner canônico `0f671ea97c0408df0bddeead179cf44a8787cef780c3fbe8bd25b4c7f21a9cec`.
O resselo referencia o RED e seu parecer anteriores à implementação.

GREEN `p1332-unit-green.json`, SHA-256
`5b65eb016388c38d812201925abfa9d077f8e09a0a109938e5736bac85e0f2af`:
compilado, 35 aprovados e zero falhas, de
`2026-09-09T14:45:01.914959+00:00` a
`2026-09-09T14:46:53.949671+00:00`. Proveniência HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado;
diff/stat/inventários estão no recibo. Antes/depois coincidem com o
inventário posterior ao resselo.

Os recibos finais de fmt, linhagem, V5/V15/V26, diff-check e lint
terminaram com exit 0 no mesmo inventário. A revisão não reexecutou
gates globais. Build, workspace e comparação CLI ainda precisam ser
auditados antes de um parecer final; este documento não os antecipa.
