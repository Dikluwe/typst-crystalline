# L0 — math/layout/root
Hash do Código: 3ce30d26


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/root.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/root.rs:19` locates `pub(super) fn layout_root(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Radical geometry

Own square and indexed radical layout. Index absence is distinct from a
present empty/zero index; index_explicit is preserved by the core but has no
effect on geometry.

Layout the radicand cramped, retaining its baseline. Layout an index cramped
at absolute ScriptScript size: relative to the incoming size, factor is
sscript in Display/Text, sscript/script in Script and 1 in ScriptScript.
Use radical_display_style_vertical_gap in Display, otherwise radical_vertical_gap.
Use radical_rule_thickness and stretch the surd with zero shortfall.

Select the surd with the original minimum gap, then redistribute excess
height before computing placement:
`gap=max(gap,(sqrt_height-thickness-radicand_height+gap)/2)`.
Place the rule at `-(radicand.ascent+gap+thickness/2)` and align the surd's
top with the top of that rule, not its center. Do not shift the radicand or
axis-center the complete radical.

For an index, consume RadicalKernBeforeDegree, RadicalKernAfterDegree,
RadicalDegreeBottomRaisePercent and RadicalExtraAscender:
`sqrt_offset=kern_before+index.width+kern_after`;
`shift_up=raise_percent*(inner_ascent-descent_surd)+index.descent`;
`index_x=-min(sqrt_offset,0)+kern_before`;
`index_y=-shift_up`; `sqrt_x=max(sqrt_offset,0)`;
`ascent=max(inner_ascent,shift_up+index.ascent)`.
Place rule and radicand consistently with the surd's horizontal offset.
Do not hardcode the constants of a particular font.

Acceptance covers upper-rule separation, display gap, tall surd selection,
overshoot redistribution, indexed-root position and shared neighboring
baselines. Historical descent/extra-ascender residuals require current
attributable evidence; do not turn them into new language requirements.
