# P1328 — revisão final

**Veredito: PASS_SCOPED.** Executor `/root/p1328_review`; regime A/B
executado sem atestação técnica de isolamento, sem selo de refinamento.
Este revisor leu produto, L0, oráculos e recibos; escreveu exclusivamente
diagnósticos `p1328-review-*`. Não editou o material julgado.

## Identidade e ordem causal

Baseline canônico `p1328-baseline.json`, SHA-256
`e049418db46ea039235b336254bfdbffd492253ac66c38782ac6bc56564f2ffb`;
manifesto `p1328-manifest.json`, SHA-256
`f7a0c4d6e7d6358fc5d34ff75465aca5da8d11188a2731e9275b9efbfe803098`.
HEAD `d31047d7b8af7837c84adae4ded3d2ff50c62093` mais working tree não
commitado. Os recibos root guardam UTC, argv, diff/stat e inventário SHA-256
integral antes/depois; esta revisão confirmou a identidade final em todos
os gates listados abaixo e não usa apenas seus exit codes como prova.

A sequência L0 → freeze A/B → integração/RED R2 → C → GREEN foi auditada
nas revisões anteriores desta coorte. R1 permaneceu preservado; R2 altera
somente a ordem dos mesmos imports. A tentativa CLI inicial inválida foi
excluída antes de C. O RED válido alcançou falhas da mensagem de Content,
com compilação bem-sucedida; não foi falha de instrumentação ou harness.

Norma final recalculada (excluindo somente metadado recíproco):
`5d8d8333b2ed042cb5af404957d7c8895fcf6e499798dd863aa54a3b321caa4e`.
Owner canônico final:
`33f8772fe163ae379a88b72b50ba2e4926104e84e03e3c9af7e11a399129e2e6`.
Arquivo calc integral:
`7c8096519d042d4e743eb0ba0ba5b77dfaa5fc2879ff4e452a96219c168d5052`.
O suffix atual coincide exatamente com o snippet R2 congelado
`b689de73f3bbe572ed4cc9a0fea0fc29a792f91a8125cc8e1a657a19e676315e`.
Runner e expectativas conservaram os hashes R1
`326235f263b308a59cb3d50726270ad6c262afe8a3591f1b2c23498fc3b0875d` e
`9d633ccf379d5a1124723ff1919f388b0603aab8cd4cd940456aa33144369623`.

## Substância do delta

Comparação do prefixo produtivo com `original_owner` do baseline demonstra
que a única mudança funcional é o ramo Content/LocatedContent em
`calc_abs`. O novo erro tem o texto L0, primeira ocorrência positional
value_span, fallback detached e nenhum hint/trace próprio. Origem já
detached não é substituída. Named, aridade, Int/Float/Decimal, demais tipos
e funções calc preservam seus braços. Fora de `calc_abs`, o produto é
idêntico ao baseline, desconsiderando somente a linhagem e testes integrados.

Content e LocatedContent têm a mesma classe pública de linguagem;
Symbol permanece separado. A origem não é obtida da Location do wrapper,
do span agregado, de nome de função ou de identidade de valores. Nenhum
owner de math, dispatcher ou Args mudou. A comparação dos inventários
baseline/final confirma somente o par calc L0/consumer alterado na coorte;
as mudanças anteriores foram preservadas.

## Gates auditados

- `p1328-unit-green.json`: os mesmos 7 testes R2 passaram integralmente.
- `p1328-final-build.json`: build release locked concluído, exit 0.
- `p1328-workspace-tests.json`, SHA-256
  `fc940d1a7b776ddc004f4de1ae8efb2ecfca29b5370b16af098e56c722d3ae3b`:
  soma independente dos 17 resumos Cargo = 6.693 passes, zero falhas,
  3 ignorados. UTC `2026-09-09T11:56:44.291836+00:00` até
  `2026-09-09T11:59:01.388051+00:00`.
- `p1328-final-fmt.json` e `p1328-final-diff-check.json`: exit 0, sem saída.
- `p1328-final-lineage.json`: hashes canônicos e snippet conferidos;
  `p1328-final-lineage-lint.json`: V5/V15/V26 com fail-on warning,
  zero violations.
- `p1328-final-lint.json`: contagem independente de registros = zero
  errors, 240 warnings e 1.141 infos. Comparação com
  `p1327-r2-final-lint.json` confirma warnings iguais após desconsiderar
  somente deslocamentos de linhas; há duas infos novas. Não é lint geral
  sem avisos.

Em cada gate root acima e nos wrappers CLI normal/repeat/reverse,
inventários antes/depois são idênticos ao inventário final de build;
manifesto e fonte calc são os mesmos. O binário final identificado no build
e nos recibos CLI é
`94c3d8cec16dc98784757227f554b2851fad186a9e76605272bdaab0e6f925f9`.

## CLI, dívidas e relatório

Recomparei diretamente exit/stdout/stderr de cada candidato com os
literais congelados, sem confiar no booleano do runner. Os recibos
`p1328-ab-cli-normal.json`, `p1328-ab-cli-repeat.json` e
`p1328-ab-cli-reverse.json` contêm 112 chaves únicas cada, todas esperadas;
zero diferenças em 336 comparações. Candidatos são iguais por chave nas
três ordens. A auditoria própria do autor A/B está em
`p1328-ab-receipt.md`, SHA-256 confirmado
`082d3d161f02bcd14b6de0e70eb1fee4c6f4ef06a098bc46d970fbd71c67e4f3`.

Por execução, as classes são 44 correções integrais, 12 correções com
dívida externa de trace, 12 controles numéricos em paridade e 44 dívidas
preservadas. Confirmei 56 saídas completas iguais ao vanilla. As rotas
With-bound/nested e Args spread continuam mostrando trace `calc.abs`,
contra `abs` do vanilla, como previsto antes de C. Isso não é normalização
do output nem mudança de expectativa para acomodar o candidato.

O relatório `p1328-final-report.md`, SHA-256
`6dde32461bd3208b7fc49b9a221890cc8dd55e644917ff6d49fa821aa090afec`,
foi revisado quanto a substância, números, proveniência e limites. Está
coerente com os recibos. O fechamento root ainda deve agregar estes hashes
e preservar a identidade auditada.

PASS_SCOPED encerra somente a correção diagnóstica e de origem definida
no L0 P1328. Suporte a outros tipos, overflow, guards divergentes, outras
funções calc e nome do trace externo permanecem dívida. Não se afirma
paridade geral de abs/calc, equivalência de compiladores ou isolamento
técnico. Nenhum bloqueio de revisão permanece para o fechamento deste recorte.
