# P1226 — paths SVG gerais: lente ampliada e gap público isolado

No estado não commitado de `697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`,
medido em 2026-08-26T18:11:32-03:00 com os ficheiros alterados enumeráveis por
`git diff HEAD --stat`, a lente passou a normalizar arcos SVG endpoint→center.
Ela conserva centro transformado, dois vetores-base da elipse, ângulo inicial e
delta orientado. Raios zero degeneram em linha; endpoints iguais são vazios;
flags inválidas permanecem `Unknown`. Não há aproximação arco→Bézier.

O RED focal teve cinco falhas; após a implementação, 25/25 testes da lente
passaram. Dez ataques focais foram registrados. O conjunto integral de vinte
não foi selado porque a medição pública atingiu primeiro um gate de contrato.

A fixture pública `p1226_curve.typ` provou inicialmente três gaps do produto:
`fill-rule: "even-odd"` virava `nonzero`, stroke `2pt + blue` desaparecia, e
`curve.close()` era tratado como fechamento reto enquanto o default vanilla é
suave. As partes M/L/C/Q anteriores ao fechamento já coincidiam.

Após confirmação humana, `FillRule` foi propagado por geometry, `ShapeElem` e
`FrameItem::Shape`; stroke rico passou a reutilizar o parser nominal; e
`CloseMode` preserva `Smooth`/`Straight`. O fechamento suave espelha os
controles de entrada e saída como o vanilla. As fixtures públicas default e
straight ficaram `Preserved`, incluindo geometria, winding e stroke complexo.
Os L0 proprietários foram ressellados.

```text
ARC LENS GREEN — PUBLIC CURVE PATHS PRESERVED — OPAQUE SVG REMAINS PARTIAL
```

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
