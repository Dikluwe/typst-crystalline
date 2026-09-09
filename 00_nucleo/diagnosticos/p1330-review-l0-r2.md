# P1330 — revisão L0 R2

**Aprovado**: R2 corrige o inventário de expectativas do mesmo overflow,
sem ampliar o domínio ou mudar a obrigação observável. Revisor
`/root/p1330_review`; A/B sem atestação de isolamento, sem selo.

Conferência em `2026-09-09T13:10:40.350Z`, HEAD
`d31047d7b8af7837c84adae4ded3d2ff50c62093`, working tree não commitado.
O owner inteiro, removida somente a linha @prompt-hash, permanece idêntico
a original_owner do baseline P1330; não há C nesta revisão.

Entradas verificadas:

- L0 completo: `f44d076c8a6b26042a0aba5a7412ca901c271bf7ecf7cf0c7085df7fcdae94b8`.
- L0 normativo: `9a5d734e086297d49d80d567bdb5527919049de19b7aea11f43b4467a972d2db`.
- Manifesto R2: `fa90e9d05855842467eb7baac85d81a157af03bb285b9c1d40df0faf1f660acf`.
- Snippet histórico p1329-ab-p1328-successor.rs:
  `1fb3d1c0b6ddfd486aa1cfd4cbdaffbdf2ecd774021b5512d217b769e92c3a7c`.
- Recibo p1330-l0-r2-reseal.json:
  `eef5dacf30d0d3dcd6f2c958dc7d5e28bfccc747fd702d54123369d8bb67970a`.

A leitura direta confirma MIN→MAX nativo na linha 209 e a expressão
avaliada equivalente na linha 313 do snippet histórico. A revisão
preliminar/R1 do revisor omitiu esse segundo controle; esta nota corrige
explicitamente o inventário anterior sem editar a evidência histórica.
As origens sucessoras detached/argumento estão corretamente diferenciadas.

Revertendo em memória apenas o parágrafo novo de migração, o SHA normativo
volta exatamente ao R1 aprovado
`10c00cbe8bfc815d77ab304c5b208205f6c6a316cd00624f2ff201b420299aa6`.
Assim todos os demais bytes normativos já revisados permanecem iguais.
Manifesto R2 preserva o R1 como predecessor e fixa somente essas duas
asserções e as células CLI equivalentes como sucessoras autorizadas.

Recibo recebido `crystalline-lint --fix-hashes .`, de
`2026-09-09T13:10:07.338448+00:00` a
`2026-09-09T13:10:14.043907+00:00`: exit 0, somente calc aplicado,
hash-a 9a5d734e/hash-b 483d55d0, zero drift remanescente.
O recibo guarda snapshot/diff/stat exatos. Perante o stat R1 revisado,
apenas calc.md passa a `249 +++++-`, total
`14 files changed, 3242 insertions(+), 107 deletions(-)`.

Gate L0 R2 satisfeito; freeze/sucessão efetiva e RED continuam gates
separados antes de C. Nenhum oráculo/output/produto/L0 foi alterado pelo
revisor; não foi aberto o passo citado como metadata do manifesto.
