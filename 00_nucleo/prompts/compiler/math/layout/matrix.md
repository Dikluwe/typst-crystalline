# Prompt L0 — `math/layout/matrix` — `MathMatrix`
Hash do Código: dd88f51a

**Camada**: L1 · **Alvo**: `01_core/src/compiler/math/layout/matrix.rs`
**Origem**: fatiado de `rules/math/layout.md` em **P314** (ADR-0104). Núcleo
partilhado: ver `math/layout/_comum.md`.

---

`MathMatrix` — matrizes. Layout de linhas via `layout_grid_rows`/`layout_grid`
do `MathLayouter` (ver `_comum.md`).

## P923 — Células renderizadas em estilo de denominador

**Achado** (`typst-layout/src/math/ir/resolve.rs:1124` — `resolve_cells`):
as células de uma matriz (`MatElem`) são resolvidas pelo vanilla com
`style_for_denominator(styles)`, ou seja, `MathSize` desce um nível
(`script_percent_scale_down`) e `cramped` é forçado a `true`. O cristalino
aplicava o `style` do ambiente directamente às células, fazendo com que
`$mat(1,2;...)$` renderizasse dígitos a `11pt` quando o ambiente era `11pt`,
em vez de `11pt * script_percent_scale_down` (≈`7.7pt`), inflando cada linha
e o delimitador. Paridade medida com `mutool trace` (`NewCMMath-Regular.otf`,
`mat(1,2;3,4;5,6;7,8;9,10;11,12)` a 20pt): vanilla trm=`7.7`, cristalino
trmais alto (`11.0`) — gap médio cristalino ≈`10.98pt` vs vanilla
≈`9.87pt` por linha).

**Correcção**: `layout_matrix` constrói um `cell_style = TextStyle {
    size: style.size * self.constants.script_percent_scale_down,
    cramped: true,
    ..style.clone()
};` e passa-o a todas as operações que medem/layoutam o conteúdo das
células (`layout_node` em cada célula e `layout_grid_rows`/`layout_grid_boxes`).
O `column_gap` (`style.size * 0.5`, paridade `DEFAULT_COL_GAP` do `MatElem`
resolvido contra o estilo exterior) e a altura-alvo dos delimitadores
(`grid_delim_target_du`, P912/P918) continuam a usar o `style` exterior —
o vanilla resolve `gap`/`augment` contra `styles`, não contra
`cell_styles`.

**P923b — row-gap de matriz**: o vanilla usa `DEFAULT_ROW_GAP = 0.2em`
(constante fixa, `matrix.rs:15`) resolvida contra o estilo **exterior**, não
contra `cell_styles` e não `math_leading` lido da tabela MATH (~0.154em).
`layout_grid_rows`/`layout_grid_boxes` passam a receber `row_gap: Pt`
explicitamente; `layout_matrix` passa `style.size * 0.2`. O piso de `(`
sintético em `layout_grid_boxes` (P921) mantém-se inalterado.

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

## P912 — Folga de 10% na Altura dos Delimitadores de Matriz

Em `layout_matrix`, a altura da grelha usada para calcular `min_height_du` dos delimitadores
aplica uma folga de 10%: `grid_height_pt = (grid_box.ascent + grid_box.descent) * 1.1`.

> **Fundamentação, ao fim de três redacções** (2026-08-13, P1026 Fase C): a folga **é** do
> vanilla. `resolve.rs:1173` (`fn resolve_delimiters`, "vector, matrix, or cases") pede
> `Rel::new(Ratio::new(1.1), Abs::zero())` com o mesmo `DELIM_SHORT_FALL`, que
> `stretchy.rs:57-61` também aplica. É **paridade**. Duas redacções anteriores concluíram o
> contrário — a de P1024 ("multiplica em vez de subtrair") e a de P1026 Fase A ("inflação
> extra de 10%", citando `resolve.rs:843`, que é a barra da fracção inclinada). As duas
> afirmações estão **refutadas**, incluindo a de que a divergência "cresce com a altura da
> matriz": não há divergência — 38 configurações medidas dão ≤1 pixel a 300dpi e contagem de
> glifos idêntica. Medição completa em `_comum.md` §P912-folga. **Item fechado.**
>
> Guarda de não-regressão:
> `01_core/src/compiler/math/layout/tests.rs:4766`
> (`p945_grid_delim_target_du_e_altura_vezes_1_1`) e `:3981`
> (`axis_bug_matrix_conteudo_centra_no_axis_height_nao_a_zero`).

## P918 — cálculo de `min_height_du` migrado para `grid_delim_target_du` partilhado

**Achado** (P918 Fase A, `matrix.rs:94-99` vs `cases.rs:28-33`): o bloco de 5 linhas acima
(margem de 10% + conversão condicional pt→du) é idêntico byte-a-byte ao usado em `layout_cases`.
Extraído para `grid_delim_target_du(&self, grid_box, style)` em `mod.rs` (ver `_comum.md` §P918)
— `let min_height_du = self.grid_delim_target_du(&grid_box, style);`. Comportamento inalterado.

## P919 — `apply_axis_offset` move-se para `grid_box` isolado, ANTES de juntar os delimitadores

**Achado** (`typst-passo-919-relatorio.md` Fase A/B, vanilla `table.rs:188`:
`frame.set_baseline(height/2.0 + axis)` — centra o meio da altura total no eixo, mesma fórmula já
usada por `apply_axis_offset`, `shift = axis_pt - (ascent-descent)/2`): o conteúdo da grelha
**deve** ficar centrado no eixo — a fórmula bate exactamente com o vanilla. Até P919, a chamada
(no fim de `layout_matrix`, sobre `result` já concatenado com `left_box`/`right_box`) era um
no-op visual (bug de omissão em `apply_axis_offset`, `_comum.md` §P919, nunca deslocava `items`).

**Achado do Agente A (revisão do design inicial deste L0)**: `result.items` mistura os items dos
delimitadores (`left_box`/`right_box`, já auto-centrados no eixo internamente por
`layout_stretchy_delimiter` — `stretchy.md` §P917) com os items da grelha (`grid_box`, ainda em
baseline própria) **antes** de `apply_axis_offset` correr. Uma vez corrigido o bug de omissão,
chamar `apply_axis_offset(result, ...)` no fim desloca **também** os delimitadores, que já
estavam correctos — desloca-os pela segunda vez. **Correcção de código necessária** (não só o fix
de `apply_axis_offset`): `layout_matrix` passa a chamar `self.apply_axis_offset(grid_box,
style.size)` isoladamente, **antes** de o mesclar com `left_box`/`right_box` — usa o `grid_box`
já centrado (items + `ascent`/`descent` actualizados) para compor `result`. `result` deixa de
chamar `apply_axis_offset` no fim. `min_height_du` (P918, acima) continua a ler o `grid_box`
**antes** do offset — as dimensões dos delimitadores dependem da altura da grelha em baseline
própria, não da versão já centrada (a diferença é só uma translação Y, não muda `ascent+descent`
somados de forma relevante para o dimensionamento, mas a ordem de leitura importa: `grid_
delim_target_du` é chamado antes do `apply_axis_offset` de P919).

**Critério de regressão**: `x + mat(a, b; c, d)` — o centro vertical da matriz fica a `axis_pt`
da baseline partilhada com `x`, não a ~0 como antes de P919; os delimitadores mantêm-se em `y=0`
local (a sua própria auto-centragem, inalterada, não duplicada). Testes do Agente A:
`axis_bug_matrix_conteudo_centra_no_axis_height_nao_a_zero` (guarda explícita de que `(`/`)`
ficam em `y=0`).

---

## P1042 — Delimitadores Customizados e Nulos (`delim`) sem Padding Redundante

Em `layout_matrix`, os delimitadores da matriz são definidos pelo par `delim: (char, char)`:
- Quando `delim.0 != '\0'`, o delimitador esquerdo é renderizado via `layout_stretchy_delimiter` colocado imediatamente adjacente à grelha.
- Quando `delim.1 != '\0'`, o delimitador direito é renderizado via `layout_stretchy_delimiter` colocado imediatamente adjacente à grelha.
- Quando `delim.0 == '\0'` ou `delim.1 == '\0'` (matriz sem delimitador / `delim: none`), o delimitador correspondente não é renderizado.

> **Fundamentação e proveniência (Passo 1042)**: no vanilla (`lab/typst-original/crates/typst-library/src/math/ir/resolve.rs:1164-1200`
> e `lab/typst-original/crates/typst-layout/src/math/table.rs`), os delimitadores esticados (`layout_stretchy_delimiter`) já
> incorporam nas suas próprias métricas de avanço OpenType MATH a folga horizontal necessária. A regra anterior que adicionava
> `padding = 0.1em` manualmente foi **refutada por medição empírica diferencial** (Passo 1042 / `typst-achado-math-cases-gutter-width.md`),
> pois duplicava a margem lateral em +2.20pt no total da matriz. A remoção do padding manual restaura paridade exata de 0.00pt
> com o vanilla em `mat` e `vec`.

## P945 — descida de nível correcta: `Display→Text` é ×1.0, não ×0.7

**Medição que refuta a generalização de P923** (`typst-passo-945` Fase A;
proveniência: commit da correcção de P944 `49ca7a629`, `temp/p945/m33.typ`):
`$ mat(1,2,3;4,5,6;7,8,9) $` em equação de **bloco** (`Display`), vanilla real
vs cristalino P944, `pdftotext -bbox`:

| | vanilla | cristalino (P944) |
|---|---|---|
| altura dos dígitos das células | 11.0pt (trm 11) | 7.7pt (trm 7.7) |
| passo entre linhas (pitch) | 13.16pt | 9.87pt |
| altura da grelha (a+d) | ≈36.3pt | 31.23pt |
| peças do delimitador `(` | 4 glifos (2 extensores) | 3 glifos (1 extensor) |

P923 mediu **inline** (`Text→Script` = ×0.7, correcto nesse contexto) e
generalizou para todo o lado. O vanilla
(`lab/typst-original/crates/typst-library/src/math/style.rs:343-363`):
`style_for_denominator = style_for_numerator + cramped`, e
`style_for_numerator` desce **um nível discreto** — `Display→Text` (**factor
1.0**), `Text→Script` (×`script_percent_scale_down`),
`Script|ScriptScript→ScriptScript`.

**Correcção**: `layout_matrix` constrói o `cell_style` via `denominator_style`
(`_comum.md` §P945) — descida por nível conforme a tabela de
`entities/layout_types.md` §P945, com `cramped: true` — em vez de
`size × script_percent_scale_down` incondicional. `column_gap`/`row_gap` e o
alvo dos delimitadores continuam resolvidos contra o estilo **exterior**
(P923/P923b, inalterado e confirmado contra `table.rs:33`/`matrix.rs:15`).

**Guarda de não-regressão**: matrizes 2×2 (que já estavam visualmente
correctas via variante pré-fabricada) continuam cobertas — o alvo do
delimitador cresce com a grelha maior, mas a selecção de variante/assembly
acompanha (mecanismo P913, inalterado).
