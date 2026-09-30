# L0 — math/layout/cases

Hash do Código: d31c40b9

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/cases.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/cases.rs:23` locates `pub(super) fn layout_cases(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Cases carrier and rows

Consume complete MathCasesElem, forming one row for each child in order before
late splitting at MathAlignPoint. Preserve children and public identity.
Resolve delimiter pair, reverse and gap independently by explicit >
math.cases StyleChain > defaults, without setting presence bits during layout.
Resolve relative gap against the outer style's font size.

Children use shared denominator style (Display→Text without size reduction,
then discrete script descent, always cramped).
Reverse=false uses only the optional left delimiter; true uses only the
optional right delimiter. None omits the glyph, not a NUL stand-in in the
public carrier. The common grid helper owns row/column placement and the
matrix module provides the existing alignment-boundary helpers; neither
takes ownership of cases behavior.

Use natural class spacing at ampersand boundaries, not extra column gaps.
The grid's 1.1 height target and shortfall determine delimiter selection.
Center the grid on the axis before merging the already-centered delimiter.
Do not insert redundant 0.1em horizontal padding or top-align components.
Final extents include both grid and delimiter maxima.

Acceptance covers row order, late alignment splitting, denominator style,
both orientations, optional sides, relative gap and explicit/cascade/default
precedence, with no public carrier mutation.

## Identidade do delimitador de cases

Na sonda `.typ/sec_09.typ` do diagnóstico `typs-teste-2026-09-30`, a
chave montada é extraída como glifos de construção (`⎧`, `⎨`, `⎩` e barra),
embora seu sinal na linguagem seja a chave de abertura. Ao compor a caixa de
`cases`, preservar os itens visuais da peça delimitadora num único
`TextReplacement` com o caractere pedido pelo carrier. Não alterar a
política comum de seleção/estiramento nem a geometria da grade.
