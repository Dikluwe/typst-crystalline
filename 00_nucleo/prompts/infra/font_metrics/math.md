# L0 — infra/font_metrics/math
Hash do Código: c05b0304

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/math.rs`

## Current evidence

- `03_infra/src/font_metrics/math.rs:17` — `extract_variants`.
- `03_infra/src/font_metrics/math.rs:52` — `extract_assembly`.
- `03_infra/src/font_metrics/math.rs:101` — `extract_variants_horizontal`.
- `03_infra/src/font_metrics/math.rs:159` — `extract_assembly_horizontal`.
- `03_infra/src/font_metrics/math.rs:207` — `build_math_glyph_reverse_map`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics/math.rs`.

## Owned contract

- Own MATH constants/variants/assemblies/kern extraction, reverse cmap and canonical ssty eligibility/substitution helpers. Translate parser data into existing neutral L1 types only.
- Keep every existing MATH field in design units and percentages divided by 100. Missing tables fall back; a present zero value is not missing.
- Variant stretch advance and hmtx horizontal advance remain distinct. Attachment uses the table value or (hmtx+italic-correction)/2. Assemblies preserve connectors/extenders, horizontal advances and minimum overlap.
- Kern tables preserve four quadrants with n heights and n+1 values, including the final unbounded None. Reverse cmap retains first base for a shared glyph, including large operators and existing base/underscore fallback.
- ssty eligibility is one character or an ASCII numeric sequence with at least one digit and at most one decimal point. Only math Script/ScriptScript outside TextItem requests level 1/2; missing GSUB leaves the base unchanged. No family-name special case or duplicated shaper predicate.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
