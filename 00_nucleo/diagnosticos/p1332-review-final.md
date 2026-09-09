# P1332 — parecer final

**PASS_SCOPED.** O candidato satisfaz o recorte de nome intrínseco de abs
e seus efeitos transitivos explicitamente congelados. Regime A/B executado
sem atestação técnica de isolamento e sem selo de refinamento.

A auditoria read-only `p1332-review-final-audit.cjs` terminou com exit 0
em `2026-09-09T14:54:42.791Z`; o resumo verificável e os hashes dos onze
gates estão em `p1332-review-final-audit.json`. Não reexecutei o produto.
HEAD confirmado `d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree
não commitado; os recibos pinados conservam diff/stat, inventário, argv
e UTC antes/depois. O índice continua vazio.

Manifesto `b32fa0ac39b79d7a1b7f23f62540011b58ac68e20f0c850009f688b68b412084`.
Relatório auditado `991d0a37a368442e9719b99856d05514d32594eec24dc092e08273996bf37e6d`;
recibo B PASS `7991337b964ef8f929c57ac76a90b37281899e87f0de4eab515e169d416b60ff`.
Os inputs congelados e artefatos históricos permanecem íntegros.

A reconstrução integral confirma somente a troca do nome no registro
produtivo; testes migraram exatamente pelas sete linhas já auditadas.
RED e GREEN executaram os mesmos 35 testes; as falhas pré-C eram apenas
de nome e o GREEN passou integralmente. Os onze gates finais têm exit 0
e o mesmo inventário produtivo. Workspace: 6.721 aprovados, zero falhas,
três ignorados. Linhagem/V5/V15/V26 aprovados. SARIF mantém exatamente o
multiconjunto P1331: zero erros, 240 warnings e 1.146 notes.

Recomparei literalmente 616 células em cada execução normal, repetida e
reversa: 1.848 observações candidatas conferem, sem normalização. Ganho
integral de 444 para 520, sem regressões; no corpus histórico, 404 para
452. Das 144 mudanças autorizadas, 84 históricas e 60 novas, somente 76
atingem equivalência integral; 68 permanecem parciais. Restam 96 células
de dívida, incluindo guards, gradient/show e abs importado em math.
Essa última dívida foi classificada antes de C e conservada literalmente.

Não se declara paridade geral de abs ou Typst, nem se autoriza corrigir
outros owners. A ocorrência inicial de listagem incidental, sem leitura
de conteúdo restrito, permanece registrada no parecer de escopo. Nenhum
input julgado foi editado pelo revisor. O fechamento canônico pelo root
ainda deve pinar este parecer e os recibos, sem alterar suas entradas.
