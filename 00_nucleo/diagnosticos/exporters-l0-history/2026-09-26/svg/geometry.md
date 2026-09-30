---
Hash do Código: 7dca7ec8
prompt: infra/export/svg/geometry
layer: L3
created: 2026-09-26
adr: ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130
---

# Prompt L0 — infra/export/svg/geometry

## Medição anterior à decisão

Fonte anterior `03_infra/src/export/svg.rs:1286-1302, 1475-1724, 1943-1973`, HEAD `6ef8b6986e74de7d5cc8f7b7e009868fc0daa5f5`, working tree não commitado, capturada em `2026-09-26T09:29:07-03:00`. Proveniência exata: `00_nucleo/diagnosticos/svg-atomization.md`. O texto histórico conserva medições e gates anteriores; não é nova prova contra o pin vanilla vigente.

## Decisão e contrato

Consumer exclusivo `03_infra/src/export/svg/geometry.rs`. Possuir render_line, render_shape, rounded_rect_path_data, paths genéricos e writers de fill/stroke/cap/join/dash/fill-rule. Preservar coordenadas, centro de stroke, quatro cantos, alpha, roles e ordem das operações. Consumir PaintReference do owner paints; não reconstruir paint ou layout. O helper de path arredondado também atende o clip geométrico.

Atomização estrutural reversível em L3: fluxo contínuo ADR-0127, sem mudança de contrato público, comportamento por defeito, fase ou compatibilidade. Referências ao monólito no texto histórico abaixo indicam a fonte anterior; a implementação específica passa a pertencer somente a este consumer. Não ampliar alegações de paridade nem interpretar mecanicamente novos IDs/bytes como paridade de linguagem.

## Raios resolvidos e stroke

- P1133: raios de `ShapeKind<Pt>::RoundedRect` chegam como `Corners<Pt>` já resolvidos.
  O SVG serializa esses pontos diretamente, sem `resolve_pt(0.0)` e sem
  projeção de parcela absoluta de `Length`.
- P1222: em rounded rect com stroke, o raio público descreve o contorno
  externo. O path central do stroke usa `max(radius - thickness/2, 0)` por
  canto. Fill e stroke são operações SVG separadas sobre essa geometria,
  preservando a morfologia do vanilla e evitando que o stroke aumente o raio
  exterior. O fill fecha o path; o stroke termina no ponto inicial sem `Z`,
  preservando cap/join do emissor de referência. Sem stroke, o fill usa os
  raios resolvidos sem inset.


### P1224 — stroke complexo preservado no SVG

Quando `FrameItem::Shape` carrega `Stroke`, o exportador preserva tardiamente
`cap`, `join`, `miter_limit`, `dash.array` e `dash.phase`. Emite
`stroke-linecap`, `stroke-linejoin`, `stroke-miterlimit`, `stroke-dasharray` e
`stroke-dashoffset`; `DashLength::LineWidth` resolve para a espessura efetiva
sem reordenar o array. Phase negativa e alpha do paint são observáveis.

Os atributos podem ser omitidos somente quando a omissão for semanticamente
igual ao default SVG. Transform não é assado nos comprimentos de stroke por
este módulo. A entidade L1 é a fonte; L3 não reconstrói informação perdida.

Critério focal: cap round, join bevel, miter 2, dash `[3pt, LineWidth]`, phase
`-0.5pt`, thickness `2pt` e paint com alpha devem aparecer separadamente e na
ordem declarada no SVG.


### P1226 — fill rule de paths

Medição pública: o vanilla emite `fill-rule="evenodd"` para
`curve(fill-rule: "even-odd")`; o cristalino anterior ao contrato emitia o
default `nonzero`. O exportador deve consumir `FrameItem::Shape.fill_rule` e
emitir `evenodd` ou `nonzero` sem inferência pelo winding. Atributo omitido só
quando semanticamente igual ao default SVG e ao valor transportado.

- Documento "Hello" produz SVG parseável com texto renderizado como paths de glifo.
- Documento com formas produz elementos SVG correspondentes.
- Documento com imagem produz `<image>` base64.
- Estrutura SVG comparável à do vanilla (não byte-exact).
- SVG renderiza correctamente mesmo sem a fonte instalada no sistema.
- `radius: 1em` preserva o raio absoluto resolvido pelo layout.
- `rect(radius: 4pt, stroke: 2pt + blue)` emite centro de stroke com raio 3pt,
  fill e stroke separados, mantendo raio exterior 4pt.


## P1286 — morfologia SVG de paths abertos de dois pontos

### Medição anterior à decisão

O oráculo congelado mede `line(start:, end:)` como um `<path>` aberto cuja
origem é o `transform="translate(start)"` e cujo `d` é `M 0 0 l delta`. O
carrier L1 deliberadamente continua `ShapeKind::Path([MoveTo(start),
LineTo(end)])`; igualdade textual de SVG não é o objetivo, mas início e delta
são a morfologia observada pela sonda.

### Decisão

Ao emitir exatamente um path aberto `MoveTo + LineTo`, o SVG normaliza a
representação para translate no primeiro ponto e segmento relativo até o
segundo. A geometria, stroke, fill, bbox e ordem de pintura não mudam. Paths
fechados, cúbicos ou com outra cardinalidade continuam no emissor genérico.
É correção de paridade interna sem API pública, default ou fase nova.

