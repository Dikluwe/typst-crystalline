# L0 — infra/shaper/font_selection
Hash do Código: 16aac30f

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/font_selection.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/fonts/bidi-visual-identity.toml sha256:b234a166cefeba71803da8bcc3cb5e3daf4d7bed8313f5883bcc7bf12bf95f15
- 00_nucleo/prompts/_nuclei/fonts/fallback-selection.toml sha256:300b35e6b7c70e79742d5edc4cda0f3419b15a41662e194fd1516b33b8cae1bb
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
