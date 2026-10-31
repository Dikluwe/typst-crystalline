# L0 — math/layout/frac
Hash do Código: f6e87bdd


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/frac.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/frac.rs:20` locates `pub(super) fn layout_frac_horizontal(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Fraction geometry

Own vertical, skewed and horizontal fraction geometry. Style resolution and
carrier-preserving dispatch belong to
`00_nucleo/prompts/core/compiler/math/layout/mod.md`.
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

## Test-only legacy wrapper boundary

### Evidence before decision

compiler/math/layout/frac.rs:101 contains the historical Clippy inventory
item. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`
retains sources/owners/references/UTC/status/diffstat. Call-site and productive-path
research precedes classification, without new parity/performance evidence.

### Contract and acceptance

Compile internal layout_frac only under `#[cfg(test)]`, preserving signature/body
for existing tests. Productive dispatch uses layout_frac_with_line and style variants; keep the lined-fraction wrapper only for historical tests.

Preserve productive routes, test names/bodies, external APIs, defaults and phase.
This ADR-0127 internal correction requires all workspace-target build/check,
core suite, test inventory, Clippy and lineage; remove no expectation to pass.


## Measured skewed fraction frame geometry correction

### Measurement before decision

lab/results/construction/diagnostics/readiness-fixes-2026-10-28/frac-skewed-findings-before-decision.json pins the
source-bound twelve-case frac-skewed-geometry-before/receipt.json against
physical15 and oracle64c5e8c0, with exact source archive/HEAD/dirty stat/UTC.
Vertical/horizontal inline/display controls match boxes within0.001pt. Skewed
inline x/y top-edge separation is1.056pt locally versus4.5287pt upstream;
denominator x differs5.5pt. Wide/tall operands and7/19pt controls remain in the
receipt. Local frac.rs:47–99 uses max operand height, +/-vgap/2 and adds slash
width to operand separation. Pinned typst-layout/math/fraction.rs:126–188 uses
sum heights+vgap, centers on font axis and compensates slash overflow. Its
ir/resolve.rs:823 keeps numerator/denominator styles and extensible slash.
This is font-derived visible geometry, not a requirement to copy frame mechanics.

### Exclusive correction and acceptance

Keep existing operand styles, independent font gaps and slash selection helper.
Use total operand heights+vgap as slash target. Fraction height is the larger
of this target and selected slash height. Baseline is half height plus font
axis. Center slash vertically; put numerator at half positive slash overflow
and denominator one numerator height+vgap lower. Operand horizontal origins
differ by numerator width+hgap; slash center is numerator width+hgap/2.
Compensate a negative slash left edge uniformly, and derive width from the
rightmost real operand/slash edge. Convert those top origins to existing
baseline coordinates; preserve every item and immutable input. No fixture
coordinates, swapped gaps, bar or new public metrics are permitted.

Add physical REDs for independent ink separation, operand horizontal gap,
axis/height, wide slash left compensation and vertical overflow. Preserve all
vertical/horizontal/barless-binomial controls. Repeat bilateral original docs
and isolated inline/display/wide/tall/multisize cases without raster/word
normalization; slash text extraction remains open unless independently fixed
and remeasured. This is an internal formula correction under continuous
ADR-0127; no public signature/field/trait, new default intent or phase move.
Full twelve gates, history and unchanged JPEG timing/RSS criteria remain required.
