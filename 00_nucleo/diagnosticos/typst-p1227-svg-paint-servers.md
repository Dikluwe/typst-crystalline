# P1227 — paint servers SVG: implementação parcial adjudicada

O baseline não commitado foi congelado como patch binário e arquivo dos paths
não rastreados. Contrato, oráculos, adversário, quatro verificadores sucessivos
e implementador receberam contextos separados. As permissões globais do
ambiente não permitem ACL por papel; além disso, autores/verificadores
declararam algumas enumerações de nomes fora das allowlists. Portanto esta
execução não possui atestação de isolamento.

O contrato funcional foi refinado antes da implementação. O gate final rejeita
26/26 mutantes válidos (`mutation_score = 1.0`), preserva 4/4 positivos, mantém
8/8 opacos como `Unknown` e é determinístico em duas ordens. O vanilla aceitou
9 fixtures públicas repetidas duas vezes. Conic e conversões entre espaços de
cor permanecem `Unknown` sem orçamento/contrato auxiliar.

O implementador não leu oráculos, ataques ou diffs. A medição no código
produtivo encontrou a primeira perda em `FrameItem::Shape.fill: Option<Color>`:
`ShapeElem.fill: Option<Paint>` é reduzido por `to_color()` no layout antes do
exportador. Stroke sofre redução análoga no SVG. Preservar linear/radial/tiling
exige alterar o contrato público L1→L3 para `Option<Paint>`.

O dono confirmou o gate ADR-0127 em 2026-08-26. `FrameItem::Shape.fill` passou
de `Option<Color>` para `Option<Paint>` e o layout deixou de chamar
`to_color()`. O SVG passou a emitir definições locais determinísticas para
linear/radial sRGB, incluindo stops/alpha e uso em fill ou stroke, e pattern de
cor com tamanho resolvido. Paints morfologicamente iguais reutilizam a mesma
definição sem promover o nome do ID a observável.

Conic, espaços de interpolação não modelados e tilings opacos mantêm o fallback
visual anterior, agora marcado por `data-crystalline-*-fallback`; a lente deve
classificá-los como `Unknown`. A execução pública de `color-spaces.typ` mostrou
o primeiro rect sRGB com `url(#p0)` e os sete espaços restantes marcados como
fallback. `linear-fill-alpha-coincident.typ` ainda produz somente o fundo da
página no cristalino: a primeira perda remanescente ocorre antes do exportador
SVG e fica como próximo gap.

Proveniência da medição final: HEAD
`697eaf31e8ce6aaa4eef7d61d7808e377005c3c5`, working tree não commitada descrita
por `git diff HEAD --stat`, instante `2026-08-26T20:02:47-03:00`; binário
`target/debug/typst` SHA-256
`41d342b9a97e4fed508276efdb8532f2856dde0a34df4d0e2b29a0e1d9952d68`.

```text
PAINT SERVERS PARTIALLY PRESERVED — OPEN GAPS RECORDED — NOT ISOLATION-ATTESTED
```

EXECUTADO SEM ATESTAÇÃO DE ISOLAMENTO.
