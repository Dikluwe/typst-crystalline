# Prompt L0 — `math/layout/matrix` — `MathMatrix`
Hash do Código: e5b98af1

**Camada**: L1 · **Alvo**: `01_core/src/engine/math/layout/matrix.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

`MathMatrix` — matrizes. Layout de linhas via `layout_grid_rows`/`layout_grid`
do `MathLayouter` (ver `_comum.md`).

## P825 — `&` dentro de células de `mat` (LeftRightAlternator)

Paridade vanilla (`math/ir/resolve.rs:1068` + `typst-layout/math/run.rs:320-331`,
medido em P825 — ver `00_nucleo/diagnosticos/typst-passo-825-relatorio.md`):

- Um `MathAlignPoint` (`&`) dentro da `MathSequence` de uma célula **parte a
  célula em várias colunas de alinhamento**. Se alguma célula de alguma linha
  tem `&`, a grelha inteira usa `GridAlign::Alternating` (colunas pares à
  direita, ímpares à esquerda — `LeftRightAlternator::Right` do vanilla); sem
  `&`, as colunas ficam centradas (`GridAlign::Center`) — paridade medida em
  P810 §12 (`mat` sem `&` centra nos dois binários).
- No limite `&` aplica-se o **espaçamento de classe** entre o último nó da
  célula esquerda e o primeiro da direita (`spacing::spacing_between` — ex.:
  `a &= b` → THICK antes de `=`), incorporado na **largura da célula par à
  esquerda** (paridade `run.rs:76-94`: o lspace do item da coluna ímpar é
  movido para a célula par), não no `column_gap` — limites `&` não levam o
  gap de coluna; limites de células separadas por vírgula levam.
- Implementação: a passagem 2 de `layout_grid_rows` vive em
  `layout_grid_boxes` (`_comum.md`), que aceita `align_boundaries` por
  linha/coluna; `matrix.rs` mede as células, soma o espaçamento de classe à
  largura da célula par e delega.

## P912 — Margem de 10% na Altura dos Delimitadores de Matriz

Em `layout_matrix`, a altura da grelha usada para calcular `min_height_du` dos delimitadores
aplica a margem de 10% do vanilla: `grid_height_pt = (grid_box.ascent + grid_box.descent) * 1.1`.

---

## Delimitadores Customizados e Nulos (`delim`)

Em `layout_matrix`, os delimitadores da matriz são definidos pelo par `delim: (char, char)`:
- Quando `delim.0 != '\0'`, o delimitador esquerdo é renderizado via `layout_stretchy_delimiter` seguido de `padding = 0.1em`.
- Quando `delim.1 != '\0'`, o delimitador direito é renderizado via `layout_stretchy_delimiter` precedido de `padding = 0.1em`.
- Quando `delim.0 == '\0'` ou `delim.1 == '\0'` (matriz sem delimitador / `delim: none`), o delimitador correspondente não é renderizado e o padding lateral é omitido.
