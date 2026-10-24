# L0 — infra/shaper/font_selection
Hash do Código: ad84d4b4

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/font_selection.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/bidi-visual-identity.toml sha256:b234a166cefeba71803da8bcc3cb5e3daf4d7bed8313f5883bcc7bf12bf95f15
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:934979e5e3d19d62b8dff899dcc462783b0522bf8bbde4c2bdbd1385e4c7f949
- 00_nucleo/prompts/_nuclei/fonts/shaper-primary-cache.toml sha256:bb9850c42a48ec519f605159d6aea76b305b0ccf6643ef6a4f8137720c76c898
```

## Current evidence

- `03_infra/src/shaper/font_selection.rs:16` — `primary_candidates`.
- `03_infra/src/shaper/font_selection.rs:116` — `CandidateSet`.
- `03_infra/src/shaper/font_selection.rs:326` — `intern_fallback_slot`.
- `03_infra/src/shaper/font_selection.rs:363` — `split_run_by_font`.
- `03_infra/src/shaper/font_selection.rs:439` — `is_p1404_format_control`.

These are local source anchors, not a new upstream parity measurement.
`lab/results/construction/diagnostics/shaper-prompt-audit/results.json` demonstrates sparse scoring against the wrong slots and a same-face Latin/Greek run merged across its script boundary.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/font_selection.rs`.

## Owned contract

- Own FontList resolution, primary/class/MATH fallback, lazy coverage, similarity preference, dense local interning and script/font segmentation. Measurement and emission use the same primary chain.
- Try declared families with the actual TextStyle variant; accept only readable faces. If none resolve, consume the shared class fallback catalog before global fallback. Catalog spelling/order belongs to `00_nucleo/prompts/infra/fallback_fonts.md`.
- In the existing resolved-declared-list branch, append the math chain when style.math or a primary face has MATH; do not invent a second append branch. Family names do not define math content semantics.
- Global candidates come from World::candidates_for_char, are lazily loaded and confirmed by glyph lookup. Preserve longest-covering-run preference, with FontBook similarity using the first resolved primary to break existing preference ties. Never load uncovered catalog entries or intervening slots.
- The local fallback vector is a sparse interner: one dense local position per distinct global slot; result indices use primary.len()+local position. Resolve through that candidate's associated slot, never treat the local index as a global FontBook index or allocate proportional to the greatest slot. Translate candidates to their associated global slots before FontBook scoring, then translate the selected global slot back to its local candidate position without loading intervening faces.
- U+2060 and the explicit bidi set from the pinned bidi-visual-identity nucleus stay in authored content and bypass coverage lookup, joining the current sub-run or first primary when isolated. Shaping/export owners govern placeholder removal and visual Unicode identity; other invisible characters are not generalized.
- Common/Inherited scripts inherit the active script, initially the first non-generic script in the bidi run when one exists. Update the active script when transitioning; a script or font change splits the sub-run even if both scripts use the same face. Preserve original cluster offsets and existing missing-glyph fallback, without a new color-font/render policy.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.

## Common fallback, controls and last-primary reuse

### Evidence

Upstream text/mod.rs:1083–1101 appends common family chain and inline/shaping.rs:904–948
tries explicit families before global fallback. correcao-rtl-restante-2026-09-29/
after-leading retains six Hebrew probes with 7.68pt divergence: pdffonts finds
Libertinus Serif in vanilla, Noto Sans Hebrew globally in Crystalline, despite
Libertinus coverage. Inferred cause is omitted common chain after resolved primary,
refuted by remaining face/position difference with chain included. Observable
typography, not byte equality, is the criterion.

controles-bidi-pdf-2026-10-06/first-raster.json observes notdef over A in
control-ltr-lri; second-verification.json.gz observes changed face/2.7662pt
divergence in control-ltr-pdi. font_selection.rs:452–454 recognizes only three
controls, :396–416 coverage-queries others. The hypothesis is refuted if keeping
controls on primary subrun without coverage lookup fails to fix it. Internal
ADR-0127 expands only pinned explicit bidi set, preserves U+2060/other character
policy, no API/phase/unsafe.

The full primary-reuse measurement/refuters at
`00_nucleo/prompts/infra/shaper/cache.md` precede this consumer's cache decision:
selection/order are language, repeated lookup is mechanics; exact complete-key
and fresh-result controls are mandatory, no inferred hit/performance from hashes.

### Contract and acceptance

After readable declared families append common ordered catalog with slot dedup.
Declared primary stays first; math chain before common; class recovery when none
resolve retains its policy; global coverage/similarity follows all explicit chains.
Measurement/emission share order. Explicit pinned bidi controls remain on current
primary subrun without coverage query; no generalization to other invisible chars.

Under shaper-primary-cache, primary_candidates compares complete equality before
rebuilding. Miss executes full existing algorithm; publish only nonempty result
without any None entry in face map. Conservative eligibility leaves negative-slot
cache unchanged; prior valid last entry may survive unpublishable request. Return
independent candidate Vec, preserving global coverage/script/instancing/run keys.
RED repeated-query and order/regex/variant/math/new-instance/failed-face refuters
must match fresh controls. Internal continuous ADR-0127 keeps API/default/phase.

## Internal expressions

`ci-simplificacoes-2026-10-14/before.json.gz` retains source/owner/spans/HEAD/dirty
state/UTC/diffstat at :66, 131, 302, 350; red.log selected errors/types checked under
1.92.0. Optional MATH/coverage predicates use is_some_and, absent false, same query
only on Some. Remove only u16→u16 units_per_em casts, preserving max(1). Keep order/
identity/fallback/lazy load/script split/cache/coverage heuristic. L0 first/scoped
seals, no public/default/phase/unsafe/dependency/suppression, unchanged functional
fixtures/expectations/tolerances, selected-lint RED→GREEN/full suite/build/all-targets/
Rustdoc/fmt/lineage/full inventory; no simplification-mirror tests.

`avisos-clippy-tekt-2026-10-15/before.json.gz` retains sources/owners/HEAD/tree/UTC/
diffstat; selected-red.json records diagnostics at :129 reborrowed variant and
:660 fixed-case Vec. Pass variant directly to select_pattern; in
changed_families_variants_and_math_match_fresh_in_both_orders use fixed-size array
with same cases/orders/iterations/expectations. No resolution/coverage/cache/
segmentation change; selected GREEN then existing suite/build/all-target Clippy/
fmt/doc/lineage with scoped seals, no new suppressions/syntax-mirror tests.

## Measured R3 geometry correction — 2026-10-23

### Measurement before decision

`lab/results/math-geometry-r3-2026-10-23/{baseline.json,baseline-source.json.gz,before/receipt.json,reductions/receipt.json}` retain the received R2 state, UTC, dirty HEAD/stat, full source map, exact inputs, binary hashes and commands. Oracle is commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree `cac836c07e4dec2a89219370428292393a3216b5`, SHA-256 `9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. Seven of 19 render probes fail 0.001 pt: five at 0.92119 pt vertical, unicode at 2.136005 pt, and selected-multichar at 0.492 pt. Isolated alpha without show has the same vertical error; adjacent ordinary text refutes a show-specific cause. Markup base and variation form both select NewCMMath rather than oracle FreeMono. Symbol arrow has omitted per-letter italic correction; sum happens to match.

Pinned `math/mod.rs:49–102` admits the normalized inline frame; `math/ir/resolve.rs:326–358` resolves each Symbol grapheme to a glyph; `inline/shaping.rs:929–936` uses ordinary fallback without inferring math from a font table. Current `equation.rs:49–52` installs the outer text strut before measurement, `math/layout/mod.rs:1354–1373` treats an entire Symbol as one glyph run, and `shaper/font_selection.rs:66–70` injects math fallback for ordinary DejaVu Sans because it has MATH. Coordinates, selected visual glyphs and source identity are observable output; storage/algorithm are mechanics. These are internal parity corrections under continuous ADR-0127, with no public contract/default/phase/compatibility change. Refutation requires bilateral PDF and SVG coordinates, adjacent-text, ordinary base/variation and grapheme controls; a passing glyph count alone is insufficient.

### Exclusive obligation
Activate the additional mathematical chain only from explicit math mode. A primary MATH table is a font capability, not a language-mode signal; declared math families remain declared first even outside math. This clause supersedes the earlier primary_has_math trigger, not declared-family order or explicit math fallback. Keep authored variation selectors attached to their base when evaluating run coverage, without preferring a different font solely for a selector's cmap entry; preserve the complete text for the shaper's variation selection. Use the same finite Unicode selector ranges as the existing font-metrics predicate (FE00–FE0F and E0100–E01EF), without widening the format-control set. Preserve global similarity, dense slot interning, cache keys, script boundaries and bidi controls. Test a MATH-capable body primary, true/false mode cache transitions and base-plus-selector run identity.

Require actual RED→GREEN, scoped resealing with valid V15/V26, full workspace validation, unchanged 0.001 pt bounds and preservation of received changes.


## R8 read-only selected-face access

Evidence: red-cache-multiline.json/log under the R8 diagnostic root confirms
native-feature cache contamination, while the snowman refuter confirms the
existing longest-covering selection must stay unchanged. Add only a descendant
read-only accessor to the existing FaceCache for an already selected global slot.
Do not select/rank/intern a different candidate, reload a cached face, apply axes
to the cached base or expose a new public API. The shaping owner clones the face,
applies its existing variations and owns feature certification. Full selection,
font-cache, warm/cold script and workspace/lineage tests remain required.
