# P1288 — recibo independente de medição vanilla

## Autoridade, regime e capacidade

- Papel: **autor de baseline vanilla**.
- Regime: protocolo completo da skill `tekt-materializacao-segregada`.
- Entradas permitidas: `00_nucleo/materialization/typst-passo-1288.md`, os
  ADRs/L0s enumerados pelo passo, a fonte em `lab/typst-original` e o binário
  `/usr/local/bin/typst`.
- Escritas: este recibo, o baseline canônico
  `lab/parity/matrix/p1288-vanilla-baseline.json` e fontes de fixture em
  `lab/parity/matrix/fixtures/p1288/`.
- Implementação candidata, contrato, oráculos, ataques e veredito ficaram fora
  da autoridade deste papel. `target/release/typst` não foi lido nem executado.
- O checkout é compartilhado; a separação é por papel, entradas, capacidade,
  ordem e hashes, não isolamento ambiental forte.

## Proveniência

- Início da medição: `2026-08-31T10:18:04-03:00`.
- Fecho das medições: `2026-08-31T10:30:21-03:00`.
- HEAD do repositório: `53d21c5a602f4045a769a0ab0c935baa5ecd3b88`.
- Working tree: **não commitada e compartilhada**. `git status --short` foi
  capturado antes das fixtures e mostrou alterações extensas P1281–P1287 e
  código candidato fora desta capacidade. Nenhum desses conteúdos foi lido.
  Como o estado exterior não é entrada do baseline, a reprodução é pinada pela
  allowlist de ficheiros e hashes abaixo, não pela árvore compartilhada inteira.
- Vanilla ratificado: upstream/main `a51e02804`.
- Binário: `/usr/local/bin/typst`.
- SHA-256 do binário:
  `7b4f40c56d6fa95082ebcfd893e275d418ebcaed1b97b62785f78284c63ff7b8`.
- `--version`: `typst 0.15.1 (e0e8ca4d)`. Essa string não substitui o pin
  ratificado nem o SHA-256 do binário.
- Ferramentas: `pdfinfo 26.05.0`, `pdftoppm 26.05.0`, `pdftotext 24.02.0`,
  `qpdf 11.9.0`, `mutool 1.23.10`.
- Compilações reprodutíveis usaram `SOURCE_DATE_EPOCH=0` e
  `/usr/bin/env -u TYPST_FEATURES`.

### Hashes das entradas normativas

| Entrada | SHA-256 |
|---|---|
| Passo 1288 | `3858860e2d3e9916b051dfbbea0ff66009382bb5daa3fcec8025e9a2492a3e28` |
| ADR-0107 | `e680d22bbf4486cf93f5bfb4ec85f4ae965e6db788c2a18c48f3be000029d49d` |
| ADR-0108 | `31daec5ae9e84cb5bbdcb806e9a2b6cb9160b7e90519df53e6e0bd8809076405` |
| ADR-0109 | `0cbd3049418073e9b8efd0be1a19030c9ecde905a25ebf3d20922e3cef02676e` |
| ADR-0127 | `5e8581b5f9ebb0798d4213e59e39ee8dfcd4f41b34d4ca639b00b1f287699ad9` |
| ADR-0128 | `917c671f03be9637aae8c3c8556ab21558a78a5a61842a49e4c8b59605bc274e` |
| ADR-0129 | `64756b81ce58ca62e1a166b3776303759bc7af507a1c97a4e3ad91a8dc5b906e` |
| `entities/html.md` | `9cf96396a4073a7e3586c45bd1fadfe92a316caa758a3df9b7d9e4cae6eba40e` |
| `shell/cli.md` | `0d51a9352d531cc82f7e89fc0d9c4f2aae4b2f8bddac7914d1ed3386be45bd3f` |
| `wiring.md` | `f8db4e993fd1931d8f77b69a2ec416377f0700569e3942cc78e32daf945e0ec8` |
| `infra/pipeline.md` | `fbff88b56f8171b436aa247ffe73d0f3c5af6d2f433bd076f63e67bde34b87a9` |
| `compiler/eval.md` | `133c737f66317a57fb25005fd6fca13a7013df8109ad4e6f095fddf077d76fcf` |
| `compiler/stdlib/pdf.md` | `5adf672f3f7693619ef687d7e1c805650c7a029034301e4a2c6e72db7ae02a94` |
| `entities/elements/table.md` | `d5cec52408d151583c19d8f58c8b47b92229bfedf961fd66cb826c91469a3663` |
| `entities/elements/table_cell.md` | `28a8890f5640966a82786ba33bb8095df6014f06f451c13b451c40e2437fc273` |
| `compiler/eval/table.md` | `d9c6af149eba059b04a1839fee0fd2fe24c2cabd1fab56e286abdf4906e648d5` |
| `compiler/layout/table.md` | `fc56d3799892335f75e03503b9f46c440534b2ddb61608e1b374eeb81006fe05` |
| `compiler/layout/table_cell.md` | `54a388c1e2c093e0937667e4060d7d5bde3887a796bd1c3f9f3732a573cd2e4a` |
| `entities/layout_types.md` | `7ef5020ba6d7eb32749261351e125b3f075a55fb2efafe830def56da3492ca17` |
| `infra/export/stream.md` | `6db25badcb0c5a23a7149c00f01da0e4334375f13baaa7a6f33fb2b29b557ee2` |
| `infra/export/builder.md` | `89d71721f1b19f6aaf5e0e6e781c58fc7148d0f62c78eac3dd736e2f46e2fb8b` |

Os hashes das fontes vanilla e de cada fixture estão no baseline JSON. Fontes
vanilla diretamente auditadas incluem `typst-library/src/{lib.rs,pdf/mod.rs,
pdf/accessibility.rs,model/table.rs}`, `typst-cli/src/{args.rs,world.rs,
compile.rs}` e `typst-pdf/src/{lib.rs,tags/context/table.rs,
tags/util/mod.rs,tags/tree/mod.rs}`.

## Comandos reproduzíveis

Os probes de expressão seguiram esta forma, sempre contra o vanilla:

```bash
/usr/bin/env -u TYPST_FEATURES /usr/local/bin/typst eval \
  'repr((type(pdf.table-summary), type(pdf.header-cell), type(pdf.data-cell)))' \
  --format json --features a11y-extras

/usr/bin/env -u TYPST_FEATURES /usr/local/bin/typst eval \
  'repr((type(html), type(pdf.data-cell)))' --format json \
  --features html,a11y-extras
```

Foram repetidos sem flags, com cada feature isolada, feature repetida, ordens
`a11y-extras`→`html` e `html`→`a11y-extras`, forma separada e forma com vírgula,
`bundle` isolado e `not-a-feature`. A matriz de assinatura/casts executou cada
caso positivo e diagnóstico registrado no JSON.

Compilação de cada fixture:

```bash
/usr/bin/env -u TYPST_FEATURES SOURCE_DATE_EPOCH=0 \
  /usr/local/bin/typst compile \
  lab/parity/matrix/fixtures/p1288/<fixture>.typ \
  /tmp/p1288-baseline/<artifact> --features a11y-extras
```

Controles adicionais usaram `--no-pdf-tags`, `--features html`,
`--features html,a11y-extras`, extensão `.svg`/`.png` e `--format html`.

Inspeção:

```bash
pdfinfo <pdf>
pdfinfo -struct <pdf>
pdfinfo -struct-text <pdf>
pdfinfo -box <pdf>
mutool show <pdf> trailer.Root
mutool show <pdf> trailer.Root.StructTreeRoot
mutool show <pdf> trailer.Root.StructTreeRoot.ParentTree
mutool show <pdf> pages.<N>
mutool show <pdf> pages.<N>.Contents
pdftotext -layout <pdf> -
pdftotext -bbox-layout <pdf> -
pdftoppm -png -r 144 <pdf> /tmp/p1288-baseline/render
```

Uma segunda compilação byte a byte de `explicit-semantics.typ` produziu o
mesmo SHA-256 `3d6dbad1…`, confirmando determinismo sob as entradas pinadas.
Os quatro controles PDF de repetição/ordem/composição produziram o mesmo
SHA-256 `989da25e…`.

## Resultados medidos

### Features

- Default: `html` e o trio PDF falham com diagnóstico de feature desligada.
- `--features html`: `type(html)` é `module`; não ativa `a11y-extras`.
- `--features a11y-extras`: os três bindings são `function`; não ativa HTML.
- Repetição, ambas as ordens e forma `html,a11y-extras`: aceites.
- Feature desconhecida: erro clap, exit `2`, lista
  `html, bundle, a11y-extras`.
- `bundle` isolado não ativa `pdf.data-cell`; o comportamento de bundle em si
  permanece scope-out/`Unknown`.

### Assinaturas, defaults, casts e diagnósticos

- `pdf.table-summary(summary: string?, table)` recebe `summary` somente named e
  tabela posicional obrigatória. String é preservada em `/Summary`. Omissão
  representa `None` interno e omite `/Summary`. **`summary: none` explícito é
  rejeitado**: `expected string, found none`.
- `pdf.header-cell(level: 1, scope: "column", cell)` usa `NonZeroU32`:
  zero/negativo dão `number must be positive`; float/string falham no cast.
  Scopes aceites: `both`, `column`, `row`; outro valor lista exatamente os três.
  Conteúdo cru é convertido a célula; `table.cell(...)` conserva colspan/
  rowspan. O argumento `cell` é posicional.
- `pdf.data-cell(cell)` aceita conteúdo cru ou `table.cell(...)`; no segundo
  caso `colspan: 2` chegou ao PDF como `/ColSpan 2`. Inteiro dá
  `expected content, found integer`; ausência dá `missing argument: cell`.

### Estrutura PDF

- Tags são habilitadas por default: `Tagged: yes`, catálogo com
  `/StructTreeRoot` e `/MarkInfo << /Marked true /Suspects false >>`.
- `--no-pdf-tags`: `Tagged: no`; catálogo sem `/StructTreeRoot` e `/MarkInfo`;
  `pdfinfo -struct` vazio.
- Tabela simples: `/Table > /TR > /TD`.
- `table.header`: `/THead`, `/TR`, `/TH /Scope /Column`; corpo em `/TBody` e
  cada `/TD` referencia o TH da coluna.
- `pdf.data-cell` dentro de `table.header` permanece `/TD`; não é promovida.
- `scope: both|column|row` aparece como `/Scope /Both|/Column|/Row`.
- Level não aparece como atributo numérico. A fixture `levels.typ` mediu o
  efeito: THs de nível 2 referenciam os THs de nível 1 da mesma coluna; TDs
  referenciam o nível 2 mais próximo.
- Colspan e rowspan aparecem como `/ColSpan 2` e `/RowSpan 2`.
- Summary string aparece no atributo `/O /Table`; omissão remove-o.
- `explicit-semantics.typ`: MCIDs `0..8`, ParentTree `/Nums [0 3 0 R]`, array
  em ordem dos nove StructElem de célula; `/IDTree` liga os IDs `U1x…` aos TH.
- Multipágina: três páginas, `/StructParents` `0,1,2`, ParentTree
  `/Nums [0 3 0 R 1 4 0 R 2 5 0 R]`; MCIDs reiniciam por página
  (`0..9`, `0..7`, `0..7`). A estrutura lógica contém um único THead e 12
  linhas lógicas de dados; repetições visuais do header não duplicam THs.

`pdfinfo -struct` emitiu em todas essas tabelas:
`Syntax Warning: Attribute BorderColor value is of wrong type (array)`.
A warning é parte do recibo e impede qualquer salto para certificação.

### Texto, páginas, boxes, geometria e outros targets

Com tags on/off, os pares summary, explicit e multipágina tiveram:

- texto `pdftotext -layout` igual;
- XML de boxes `pdftotext -bbox-layout` igual;
- Media/Crop/Bleed/Trim/ArtBox iguais;
- raster 144 ppi byte a byte igual em todas as páginas.

No par plain versus wrappers a11y com visual idêntico:

- HTML: bytes iguais, SHA-256 `0aa89e55…`;
- SVG: bytes iguais, SHA-256 `1989aae1…`;
- PNG: bytes iguais, SHA-256 `19554c50…`.

HTML emitiu as warnings de export experimental e de `page` ignorado. Esta é
igualdade medida para as fixtures, não inferência de efeito geral.

## Unknowns e limites

- Tecnologia assistiva real, navegação por screen reader, reflow, PDF/UA e
  certificação de standard: **Unknown**.
- A presença de tags, MCIDs, ParentTree e `Suspects: no` não prova
  acessibilidade efetiva.
- Relações ou objetos que `pdfinfo`/`mutool` não interpretaram: **Unknown**.
- A warning `BorderColor` não foi reinterpretada nem normalizada.
- HTML/SVG/PNG cobrem somente o par congelado; não estabelecem contrato geral.
- Bundle permanece **Unknown/scope-out**.
- Artefatos PDF ficaram em `/tmp/p1288-baseline`; os inputs persistentes,
  comandos e hashes suficientes para reprodução estão no baseline JSON.

Este recibo não contém veredito de refinamento e não afirma equivalência
funcional geral.
