# Prompt: MathConstants — OpenType MATH constants
Hash do Código: a5678e34

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/math_constants.rs`

## Responsibility and base declaration

Provide pure OpenType MATH design-unit constants for MathLayouter fractions/scripts/radicals/limits/accents. L3 reads fonts through ttf-parser and populates data; this L1 owner never performs I/O or parses font tables. Preserve the complete base declaration below plus the separately specified extensions; the old 17-field count is stale and must not delete listed fields.

```rust
pub struct MathConstants {
    pub upem: f64,
    pub fraction_rule_thickness: f64,
    pub fraction_num_gap: f64,
    pub fraction_denom_gap: f64,
    pub fraction_numerator_shift_up: f64,
    pub fraction_denominator_shift_down: f64,
    pub superscript_shift_up: f64,
    pub superscript_shift_up_cramped: f64,
    pub subscript_shift_down: f64,
    pub radical_vertical_gap: f64,
    pub radical_rule_thickness: f64,
    pub axis_height: f64,
    pub script_percent_scale_down: f64,
    pub script_script_percent_scale_down: f64,
    pub upper_limit_gap_min: f64,
    pub lower_limit_gap_min: f64,
    pub math_leading: f64,
    pub accent_base_height: f64,
    pub flattened_accent_base_height: f64,
}
```

Scale-down fields are ratios 0..1. fraction_num_gap/fraction_denom_gap are minimum floors, not direct additive gaps. to_pt(value,size) is size*(value/upem). axis_height supports fraction/delimiter/root baseline alignment; upper/lower gap minima constrain extensible operators, math_leading separates math lines. MathLayouter obtains constants through metrics.math_constants; child owners consume fraction/script/root fields and nested-script scales.

## Fallback evidence and baseline acceptance

The original fallback was described as STIX Two Math with upem 1000/axis 500, but the archived cramped-shift investigation could not match its overall values to any located STIX file. Do not claim a verified single-font provenance. Keep neutral cramped superscript shift equal to normal 362 and both accent thresholds equal to axis 500 for no-real-font tests/FixedMetrics. Specific later extensions below use independently measured NewCM values. The pending flattened-accent selection does not become implemented merely because its threshold is carried.

Require fallback upem/line thickness/axis>0 and scale ratios 0..1, to_pt(500,Pt12)=Pt6 at upem 1000, to_pt(0,size)=Pt0 and font-size proportionality. Retain `frac_com_axis_height_nao_regride`, `delimitado_com_axis_height_nao_regride`, `sqrt_com_axis_height_nao_regride` sentinel expectations. These historical tests establish local behavior, not current oracle parity.

## Fraction and accent evidence and semantics

Upstream math/fraction.rs:30–53 computes num_gap=max(shift_up−axis−thickness/2−num.descent(),fraction_numerator_gap_min) and denom_gap=max(shift_down+axis−thickness/2−denom.ascent(),fraction_denominator_gap_min). Carry fraction_numerator_shift_up/fraction_denominator_shift_down from corresponding ttf-parser constants. The original direct-gap interpretation is superseded. Their fallback values were measured with fontTools from pinned NewCMMath-Regular.otf; exact values/provenance remain in the original `typst-passo-920-relatorio.md` PhaseB rather than being invented here.

Upstream scripts.rs:325–330 changes only superscript_shift_up to its cramped counterpart in the documented cramped formula; preserve that distinction without changing other terms. Both are font-provided MATH values.

Upstream accent.rs:56–65 computes gap=−accent.descent()−min(base.ascent(),accent_base_height). Read accent_base_height and flattened_accent_base_height from ttf-parser0.25.1 math.rs:214,220. Upstream accent.rs:24–28 uses the latter for flattened variants; that consumer was explicitly deferred locally. Signed accent ink measurement belongs to the dedicated FontMetrics::text_ink_bounds_signed contract; retain general text_ink_bounds and MathBox ascent/descent semantics. This data owner does not move those algorithms.

## Measured extension fields and defaults

All named fields below are public f64 values in design units except the explicitly ratio-valued degree percentage. Keep these additions alongside the base fields.

- display_operator_min_height:1300. Historical NewCMMath evidence observes summation.v1 height 1401du/integral.v1 height 2223du, advances upstream1.444em/0.999em versus local1.056em/0.665em. Upstream resolve.rs:350–354 and fragment/glyph.rs:445–451 stretch Large glyphs in Display to this target. L3 reads it; math layout consumes it.
- upper_limit_baseline_rise_min:111 and lower_limit_baseline_drop_min:600. NewCMMath-Book/fontTools/upem 1000 also measures UpperLimitGapMin200/LowerLimitGapMin167. Upstream compute_limit_shifts scripts.rs:290–313 requires all four; adding baseline terms completes the previously recorded missing-term scope.
- radical_kern_before_degree:278, radical_kern_after_degree:−556, radical_extra_ascender:48 and radical_degree_bottom_raise_percent:0.6. NewCMMath-Book/fontTools/upem 1000 measures raw degree percent 60. Upstream radical.rs:86–96,113–114 positions degree from these. L3 reads three MathValueRecords via .value and the raw i16 percentage divided by 100. Public gate confirmed by owner `Continue` on 2026-08-05 following the original root-degree report.
- radical_display_style_vertical_gap:148. NewCMMath-Book measures Display 148 versus ordinary 50; upstream radical.rs:32–36 selects by math size. L3 reads .value. Owner confirmed its public-field gate on 2026-08-05.
- fraction_numerator_display_style_shift_up:677, fraction_denominator_display_style_shift_down:686, fraction_num_display_style_gap_min:120, fraction_denom_display_style_gap_min:120. Owner approved on 2026-08-06. Upstream fraction.rs:33–52 selects Display constants; the archived 600dpi rho/epsilon-zero probe found local text constants394/345/48 producing near-zero/negative bar gaps versus upstream2.56/1.43pt+. Retain `p990_tests::p990a_frac_display_usa_constantes_display` and `p990a_frac_text_mantem_constantes_de_texto`.

## Stack fraction proposal and approval-verification debt

For line:false, upstream fraction.rs:86–101 uses Stack constants rather than Fraction. The original public-field proposal names stack_top_shift_up444, stack_top_display_style_shift_up677, stack_bottom_shift_down345, stack_bottom_display_style_shift_down686, stack_gap_min120 and stack_display_style_gap_min280. Values were measured with fontTools at `03_infra/fixtures/fonts/NewCMMath-Book.otf`, upem 1000, uncommitted tree on 2026-08-22; fallback uses these measured values. The original P1132f clause explicitly requires ADR-0127 confirmation. This consolidation has not located its receipt and retains the full requirement plus approval-verification debt; implementation presence is not approval evidence.

## Skewed fraction proposal and approval-verification debt

Upstream math/fraction.rs:132–162 consumes SkewedFractionVerticalGap, SkewedFractionHorizontalGap and existing AxisHeight. The proposal adds public skewed_fraction_vertical_gap and skewed_fraction_horizontal_gap, with measured fallback96du/350du. `ttx -t MATH` on NewCMMath-Book.otf at 2026-09-16T17:43:17-03:00, HEAD `451f3ca3d208c997a41d93ccc597a9c117fe1de9`, uncommitted tree with the twelve listed L0 changes and untracked proposal, source SHA-256 `60346e6fc27773d96c3cc9d3ac8a550a85e3ca624437179bd7a88b4ca18f0a18`, supplies provenance. These are font metrics, not fixture coordinates. Layout still converts style.size*value/upem; L1 reads no font. The original P1427 public gate remains an approval-verification debt here; this cleanup does not ratify it or remove its acceptance obligations.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/math_constants.md` preserves the original evidence and chronology.
