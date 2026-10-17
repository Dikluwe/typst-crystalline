# L0 — math/layout/frac
Hash do Código: 24f23dfa


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/frac.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/frac.rs:20` locates `pub(super) fn layout_frac_horizontal(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Fraction geometry

Own vertical, skewed and horizontal fraction geometry. Style resolution and
carrier-preserving dispatch belong to
`00_nucleo/prompts/compiler/math/layout/_comum.md`.
Numerator uses the shared numerator style; denominator uses denominator style
with cramped=true. Preserve the discrete size descent, not unconditional 70%.

Vertical line=true draws exactly one horizontal bar at -axis_height.
Use font-derived rule thickness and distinct numerator/denominator shifts
and minimum gaps; Display selects the dedicated display constants:
`num_gap=max(num_shift-axis-thickness/2-num.descent, num_gap_min)`;
`den_gap=max(den_shift+axis-thickness/2-den.ascent, den_gap_min)`.
Before the axis translation, numerator baseline is
`-(num.descent+num_gap+thickness/2)` and denominator baseline is
`den_gap+thickness/2+den.ascent`. Translate items by -axis and update
extents correspondingly, without generic midpoint centering.

Padding is 0.1em on each side against the outer fraction style.
The bar's width is max(num.width,den.width), centered inside the padded box,
not extended to both edges. Include asymmetric ink extents.

Line=false uses stack_top_shift_up, stack_bottom_shift_down and stack_gap_min,
including Display variants; distribute any gap deficit between the operands,
preserve padding and emit neither bar nor slash. This remains the vertical
binomial stack even under a horizontal/skewed style cascade. Binomial
delimiters size against its real height through the delimited owner.

Skewed places numerator above/left and denominator below/right of an extensible
fraction slash. Consume independent skewed vertical and horizontal gaps;
do not swap them or synthesize them from fixture coordinates. Their existing
fallbacks are 96du vertical and 350du horizontal, owned by MathConstants.
Horizontal places numerator, slash and denominator in a line and restores
parentheses independently only for operands whose deparenthesization bits
require them. No horizontal bar is emitted.

Acceptance discriminates all styles, independent gaps, explicit/cascade
precedence, both deparenthesization bits and the barless binomial control.
The current slash-selection and shift implementation is not itself proof
of upstream geometry parity.

## Helpers e armazenamento sem uso — 2026-10-13

### Medição anterior à decisão

`01_core/src/compiler/math/layout/frac.rs` nas linhas 101 contém os itens sinalizados no inventário Clippy. HEAD 06848cdcb5450f2c555626bf116bc7acd82bd82b, working tree não commitado; fontes, owners, referências, UTC, status e diffstat em `00_nucleo/diagnosticos/helpers-testes-ci-2026-10-13/before.json.gz`. A pesquisa de chamadas e os caminhos produtivos precedem esta classificação; não é nova medição de paridade nem prova de desempenho.

### Decisão e aceitação

Compilar o helper interno `layout_frac` somente sob `#[cfg(test)]`, conservando assinatura e corpo para os testes existentes. O despacho produtivo usa layout_frac_with_line e as variantes de estilo; conservar o wrapper de fração com linha somente para os testes históricos.

Preservar as rotas produtivas, nomes e corpos dos testes existentes, APIs externas, defaults e fase. Alteração interna em fluxo contínuo ADR-0127. Revalidar build e check de todos os targets do workspace, suíte core, inventário de testes, Clippy e linhagem; não eliminar expectativas para passar o gate.
