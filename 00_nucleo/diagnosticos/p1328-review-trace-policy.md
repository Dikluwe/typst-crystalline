# P1328 — política de trace externo antes do freeze e de C

Revisor `/root/p1328_review`, `2026-09-09T11:41:44Z`; A/B sem atestação
técnica de isolamento. Nenhum produto, L0 ou oráculo alterado pelo revisor.

Entradas:

- Manifesto `p1328-manifest.json`, SHA-256
  `f7a0c4d6e7d6358fc5d34ff75465aca5da8d11188a2731e9275b9efbfe803098`;
  L0 normativo `5d8d8333b2ed042cb5af404957d7c8895fcf6e499798dd863aa54a3b321caa4e`.
- Medição A/B válida `p1328-ab-cli-baseline-r1.json`, SHA-256
  `8f9ddaead847be551b7eb779fbf6719ece3043e84714d872b03c278c9e78db5c`;
  UTC `2026-09-09T11:39:28.773058+00:00`, HEAD
  `d31047d7b8af7837c84adae4ded3d2ff50c62093`, binário BASE SHA-256
  `75e8b97b3788c0feaf457cb4c06b1c2c6735cff5ef3b9804a75ec57f8b148e31`,
  VANILLA `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
  Proveniência da árvore BASE e diff integral: baseline canônico
  `p1328-baseline.json`, SHA-256
  `e049418db46ea039235b336254bfdbffd492253ac66c38782ac6bc56564f2ffb`.

## Medição anterior à decisão

R1 mede os casos `content-with-bound`, `content-with-nested` e
`content-spread-args`. O vanilla ancora o erro no conteúdo pré-vinculado,
fora da chamada posterior; conserva trace `while calling abs`. O baseline
tem âncora detached e trace externo `while calling calc.abs` no mesmo
trecho de chamada posterior. `content-with-empty` é controle distinto:
a origem do conteúdo está dentro da chamada, e o vanilla não exibe trace.

O dispatcher vigente, `eval/call_dispatch.rs:1031–1044`, suprime trace
quando o span do erro está contido na chamada e preserva os demais.
`:1648` passa `func.name()`; a inscrição de abs no baseline é `calc.abs`
(`stdlib/calc.rs:58`). Não é tarefa desta coorte mudar esse nome.

A tentativa `p1328-ab-cli-baseline.json` pôs `--color` depois de `eval`:
o vanilla respondeu `unexpected argument '--color' found`, exit 2. Isso
é falha de instrumentação, não avaliação Typst e não RED. R1 remove esse
argumento, captura saídas literais e avalia as expressões. Nenhuma saída
da tentativa inválida pode ser promovida a evidência funcional.

## Veredito de obrigação

O L0 atual já determina: “Traces de chamadas externas continuam
responsabilidade do dispatcher vigente e são observados integralmente.”
Também determina erro nativo sem trace próprio, âncora correta e ausência
de paridade geral de abs. Portanto a expectativa sucessora desses casos
é diagnóstico/range corrigidos com trace externo `calc.abs` preservado.
Não é necessário amendment normativo para explicitar uma consequência
direta dessas cláusulas. Deve-se registrar a dívida antes do freeze.

O oráculo deve congelar a observação sucessora COMPLETA de cada caso e
perfil, incluindo stdout, stderr e exit, com o texto `calc.abs` previsto.
Ela é derivada do L0 + evidência pré-C, não uma observação executada do
candidato nem uma alegação de igualdade integral ao vanilla. A origem
fora da chamada determina a permanência do trace; uma origem dentro da
chamada deve retirar o trace redundante, conforme o dispatcher vigente.

Não aceitar normalização de outputs, remoção de traces para comparar,
substituição dinâmica `calc.abs` ↔ `abs` no output do candidato, ou
expectativas escolhidas a partir do candidato. A classificação desses
casos deve indicar correção Content COM dívida externa preservada;
`baseline_equals_vanilla` continua falso onde o nome diverge.

O runner inspecionado ainda escolhia VANILLA para toda `correction` e
BASE para `preserved-debt`; essa política provisória não é suficiente
para os casos mistos. Antes do freeze, o autor A/B deve entregar arquivo
imutável de expectativas sucessoras literais e comparador que o leia.
Não foi aprovado freeze nem RED nesta revisão; a auditoria do artefato
final ainda é necessária. A autoria e escrita dos oráculos continuam
exclusivamente com o autor A/B.
