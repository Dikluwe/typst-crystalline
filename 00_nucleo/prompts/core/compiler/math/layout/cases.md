# L0 — math/layout/cases
Hash do Código: 4a2a200a


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/cases.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/cases.rs:23` locates `pub(super) fn layout_cases(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

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

## Helpers e armazenamento sem uso — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/math/layout/cases.rs` nas linhas 23 contém os itens sinalizados no inventário Clippy. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes, owners, referências, UTC, status e diffstat em `lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`. A pesquisa de chamadas e os caminhos produtivos precedem esta classificação; não é nova medição de paridade nem prova de desempenho.

### Decisão e aceitação

Compilar o helper interno `layout_cases` somente sob `#[cfg(test)]`, conservando assinatura e corpo para os testes existentes. O despacho produtivo usa layout_cases_elem; o wrapper de rows/delim/reverse/gap continua disponível somente para testes.

Preservar as rotas produtivas, nomes e corpos dos testes existentes, APIs externas, defaults e fase. Alteração interna em fluxo contínuo ADR-0127. Revalidar build e check de todos os targets do workspace, suíte core, inventário de testes, Clippy e linhagem; não eliminar expectativas para passar o gate.
