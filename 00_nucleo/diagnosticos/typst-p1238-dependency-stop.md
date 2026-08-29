# P1238 — stop de dependência dos espaços polares

**Proveniência:** HEAD `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`,
working tree não commitada, preflight em `2026-08-27T16:36:42-03:00`.
Antes dos artefatos P1238 havia 50 ficheiros tracked alterados, 1870 inserções,
140 remoções e 152 paths untracked. A lista exata está pinada em
`p1238-working-tree-snapshot.txt`, SHA-256
`e444e239f73ef89f95a42397dda2751d7aa9385670c391572fb1238c94919c51`.

**Veredito:** `DEPENDENCY_BLOCKED`.

P1236 não selou alpha: receipt
`4a1df1944d077e84859a9efcc56314d9d96d5990b1d50c19802512dac6becb5d`
registra `PRESEAL_REJECTED`, score `6/9`, nenhum probe executado e alpha
`Unknown`. P1237 encerrou sem promoção: Oklab passou numericamente `0/6` em
Linear e Radial; LinearRgb passou `3/6` em cada geometria, mantendo os quatro
pares `Unknown`.

P1238 exige alpha e hue conjuntamente nos seis pares
Linear/Radial × Oklch/Hsl/Hsv. Sem o selo de P1236, criar ou executar a matriz
polar permitiria que uma falha de alpha fosse atribuída a hue, ou que alpha
`Unknown` fosse promovido implicitamente. Isso viola o gate expresso do passo e
a política de `Unknown` do protocolo Tekt.

Por isso não foram criados contrato polar, fixtures ou probes; nenhum Prompt L0
ou ficheiro produtivo foi alterado. Os seis pares permanecem `Unknown`, sem
analogia entre espaços. O próximo passo causal é reparar e selar P1236 e então
readjudicar as dependências antes de reabrir P1238.

Como a materialização semântica não começou, atestação de isolamento é
`não aplicável` a este stop determinístico.
