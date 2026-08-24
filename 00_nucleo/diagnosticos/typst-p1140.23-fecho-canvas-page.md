# Diagnóstico P1140.23 — fecho do canvas e das camadas de página

**Estado:** fechado  
**Medição final:** 2026-08-24T15:58:33-03:00  
**HEAD:** `45b547073d7686cdd5d3e3030c82de3e22ec395f`  
**Árvore:** working tree não commitado

## Resultado

`bleed`, `fill`, `background` e `foreground` atravessam eval, os deltas de
`SetPage`/`PageRunElem`, o snapshot físico de `Page` e os exporters. A largura
e altura de `Page` continuam a representar o TrimBox. O canvas físico é
derivado do bleed resolvido no fechamento de cada página.

| Argumento | Owner de domínio | Transporte/consumer | Prova principal |
|---|---|---|---|
| `bleed` | `entities/page_canvas.rs` | eval → SetPage/PageRun → layout → Page → exporters | testes de domínio, eval, layout e caixas PDF |
| `fill` | `entities/page_canvas.rs` | eval → configuração/snapshot → PDF/SVG/raster | estados Auto/None/Paint e testes por target |
| `background` | `Content`/`PageRunElem` | layout atomizado → `Page.background` → exporters | snapshot separado e plain text apenas do body |
| `foreground` | `Content`/`PageRunElem` | layout atomizado → `Page.foreground` → exporters | snapshot separado e ordem final dos consumers |

O owner de layout é `compiler/layout/page_canvas.rs`, na forma B da
ADR-0109. As listas decorativas são distintas de `Page.items`; portanto os
consumers semânticos continuam a observar somente o body. As suítes integrais
de query, introspecção, acessibilidade e PDF tagueado permaneceram verdes.

## Caixas e comportamento por target

- PDF com bleed `(left: 10, right: 20, top: 30, bottom: 40)` e TrimBox
  `200 × 100`: `MediaBox = 230 × 170` e
  `TrimBox = [10 40 210 140]`.
- PDF sem bleed omite `TrimBox`.
- PDF trata Auto e None como transparentes e emite Paint antes de background,
  body e foreground.
- SVG e raster mantêm o TrimBox por defeito; `render_bleed: true` seleciona o
  canvas físico. Auto é branco e None é transparente nesses targets.

## Gates reproduzíveis

- `cargo test -p typst-core p1140_23 --quiet`: 4 passados.
- `cargo test -p typst-infra p1140_23 --quiet`: 3 passados.
- `cargo test -p typst-core --quiet`: 5182 passados; zero falhas.
- `cargo test -p typst-infra --quiet -- --test-threads=1`: 835 passados; zero falhas.
- `cargo test --workspace --quiet -- --test-threads=1`: verde; entre os lotes
  reportados, 5182 + 835 + 53 + 2 + 55 + 2 testes passados e 3 ignorados.
- `cargo build --workspace`: verde.
- `crystalline-lint .`: exit 0, zero violations; avisos históricos permanecem.
- `git diff --check`: verde.
- `crystalline-lint --fix-hashes .`: linhagem de `page_canvas` ressellada em
  `23884959`; zero drift restante.

A execução serial do workspace é deliberada: testes históricos de medição da
infraestrutura compartilham estado global e podem envenenar-se em paralelo. A
mesma suíte de infraestrutura passou integralmente em série.

## Proveniência da árvore medida

`git diff HEAD --stat` no instante acima registrou 117 ficheiros rastreados,
2473 inserções e 575 remoções. Esse total inclui trabalho anterior já presente
na árvore desta sequência P1140, além de P1140.23. Os ficheiros não rastreados
foram registrados por `git status --short` na sessão e incluem os passos,
diagnósticos e novos owners ainda não commitados.

## Fronteira seguinte

Este fecho não expõe `page` nem `std.page`, conforme o escopo. A frente pública
correspondente continua separada. Também não usa igualdade binária de PDF como
critério: as provas são estruturais e no nível observável da linguagem.
