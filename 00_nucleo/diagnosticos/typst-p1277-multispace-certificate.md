# P1277 — certificado da fronteira SVG multi-space restante

**Veredito:** `NO-PROMOTION — ALL-EIGHT-UNKNOWN-GENERALIZATION`.

Foram executados 240 grupos: 192 fixtures gerais (24 por par) e 48 grupos
focais, estes com 336 observações. O envelope focal preservou 336/336, mas a
conjunção geral preservou apenas 97/192: grafo
192/192, numérico 98/192, raster
161/192 e custo 192/192. Subgates não se
compensam, portanto os oito pares permanecem `Unknown-generalization`.

Foram rejeitados 30/30
mutantes executados, 56/56
entradas inválidas e 384/384
recibos de determinismo. A ordem inversa reproduziu os artefatos focais, e a
segunda execução geral reproduziu byte a byte `results`, `pairs` e
`oracle-budgets` (além dos demais recibos semânticos).

Luma concentra 32 fixtures com divergência
pública, coerente com a fronteira de alpha já conhecida; isso bloqueia, não é
contado como sucesso. CMYK permanece `Unknown-ADR0097`.

Nenhuma mudança produtiva, de L0 ou de comportamento padrão foi aplicada. Os
oito pares conservam o fallback explícito `gradient-color-space`. Este
certificado é restrito à população executada e não alega equivalência SVG
geral.

Proveniência: HEAD `4d7e224c8a843635b5052ee53e80cbe59bd9ade6`, working tree não commitada, medição
`2026-08-29T13:01:54-03:00`. Manifesto de evidências
`p1277-evidence-manifest.tsv` (`sha256:59b26937c0a491b8698517e11a88338381b39748704c89914b993ab213bca2f3`).

**Atestação:** EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
