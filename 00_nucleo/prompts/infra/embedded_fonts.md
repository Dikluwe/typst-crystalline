# L0 — Embedded font sets
Hash do Código: 528ec5d1

**Layer:** L3
**Exclusive consumer:** `03_infra/src/embedded_fonts.rs`

## Current evidence

- `03_infra/src/embedded_fonts.rs:31` — `EmbeddedFontSets`.
- `03_infra/src/embedded_fonts.rs:66` — `embedded_font_group`.
- `03_infra/src/embedded_fonts.rs:85` — `load_embedded_fonts`.
- `Cargo.lock` — typst-assets revision `c0ae970f64413cf8e39cecd56d15ee50de42eaf4`.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own loading and classification of the compiled `typst_assets::fonts()` bytes. `EmbeddedFontSets` contains `text_slots`, `text_book`, `math_code_slots` and `math_code_book`. Extract metadata through the fonts owner, create embedded slots with display path `<embedded>`, and insert valid metadata/slots together. Loading does not read the filesystem.

Classify the normalized metadata family name, not the raw filename or uncorrected nameID 1. Case-insensitive names starting with `Libertinus Serif` or exactly `New Computer Modern` belong to text. Exactly `New Computer Modern Math`, names starting with `DejaVu Sans Mono`, and unrecognized names belong to math/code. The metadata exception table belongs to `00_nucleo/prompts/infra/fonts.md`; preserve its documented family names with spaces.

The embedded asset set makes Libertinus Serif and the bundled math/code faces available independently of host installation. `00_nucleo/prompts/infra/world/fonts.md` exclusively owns their insertion relative to system/project fonts and the existing builders. The language's default text family belongs to L1 style owners; neither those defaults nor CLI composition are redefined here.

## Verification boundary

Verify embedded slots load without disk access, Libertinus Serif and New Computer Modern occur in text, and New Computer Modern Math/DejaVu Sans Mono occur in math/code. Positive membership controls must accompany exclusion checks; an absent family must not make an exclusion test pass vacuously.

## Static asset storage

### Evidence

`embedded-fonts-memoria2-2026-10-08/before.json.gz` freezes embedded_fonts.rs:92–96:
static slice copied to Vec, cloned into the slot and discarded after metadata.
Those are two buffers per load iteration, not proof of persistent whole-catalog
duplication. `custo-fixo-fontes-item2-2026-10-08/probe-results.json` measures
9, 682, 300 bytes across 17 assets, about 9.24 MiB of owned retained allocation plus
resident executable static pages. Copy site :92–94 contrasts with pinned upstream
typst-kit/fonts.rs:136–141 passing static slices to Bytes without Vec. A simple
CLI comparison observed Tekt 41.83 MiB and vanilla 29.21 MiB with equal visible
text; do not attribute the entire gap here without a post-change experiment.
These are storage/observable-output facts, not historical-intent evidence. Full
provenance and old single-transfer proposal are archived in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/infra/embedded_fonts.md`.

### Current contract and acceptance

The static-slice decision supersedes mandatory asset→Vec transfer, including the
earlier one-copy interim rule. Extract the same metadata directly from the original
typst_assets slice, then create a static-origin slot through the internal fonts
owner constructor; no copy for unselected assets. Preserve composition/classification/
order/book, `<embedded>` display path and paired slots/metadata. The fonts owner
alone materializes a selected face lazily in get; do not duplicate that here.
No public asset/font representation, discovery/language default/API/mode/phase
change. Continuous ADR-0127 acceptance preserves loading/classification and fonts
source/get sharing, export controls, bilateral text/style/geometry/raster wherever
baseline agreed, and paired RSS/time measurements with exact provenance. No
unmeasured allocation or CLI-gain claim.

## Executable fixture controls

### Evidence

`fixtures-ativos-ci-2026-10-13/before.json.gz` retains sources/owners/diagnostics,
HEAD/uncommitted tree/UTC/status/diffstat. embedded_fonts.rs:192 has New Computer
Modern Math presence/exclusion assertions without #[test]; :135 only prints
family metadata; :235 observes DejaVu text exclusion without math/code presence.
The actual pre-edit suite confirms the missing mathematical test. This is not a
new upstream comparison.

### Contract and acceptance

Register p754_newcm_math_is_not_in_text_group as an executable test with unchanged
assertions. Remove only p797_probe_list_all_embedded_families, whose printed
inventory is diagnostic. Add positive DejaVu Sans Mono math/code membership to
the existing exclusion control; keep every other test. The list replaces exactly
the probe name with the math-test name; empty lists cannot satisfy positive
membership. Internal ADR-0127 test maintenance adds no public/default/phase
change. L0 first, affected-owner resealing; shell/infra suites, all-target compile,
fmt/lineage/Clippy inventory. Test counts do not certify parity.
