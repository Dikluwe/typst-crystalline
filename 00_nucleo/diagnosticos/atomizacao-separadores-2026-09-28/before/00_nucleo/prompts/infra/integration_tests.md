# L0 — Infrastructure end-to-end test harness
Hash do Código: 09f3072b

**Layer:** L3 (cfg(test) only)
**Exclusive consumer:** `03_infra/src/integration_tests.rs`

## Current evidence

- `03_infra/src/integration_tests.rs` — the test-only module and compile/eval/frame helpers.
- `03_infra/src/integration_tests.rs:5203` — inverted-bracket witness from `.typ/sec_22.typ`.
- `03_infra/src/integration_tests.rs` — bundle feature-gate and active-target cases.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own only this end-to-end harness. Exercise SystemWorld, temporary filesystem, evaluation, introspection, layout and export across the real infrastructure boundaries. Helpers remain test-only/private and remove their temporary resources. Productive modules keep their individual owners; test setup cannot move I/O or concrete World access into L1.

Assert language values, morphology, diagnostics/warnings and observable exported structures. Byte/internal-structure identity is acceptance only when explicitly selected as the observable, such as the independently owned snapshot corpus.

Geometric helpers accumulate ancestor affine transforms: Group translation, matrix and inherited transform compose in exporter order; nested groups compose transitively. Project Text, TextShaped, Glyph, Image, Shape and Line endpoints once into global coordinates. Semantic is transparent and Link keeps its existing convention. Counting/classification may inspect local variants, but geometric assertions use global coordinates. Do not compensate an observer bug by changing expected geometry, fixtures or productive layout.

For the pinned no-text placement witnesses, check an empty page's requested PDF MediaBox, preserved cover-image clip and placed-text baseline against the independently observed vanilla geometry. Run the real World and production pipeline for font-demand controls. A no-font declaration with an isolated shape or image must not request the font book; a placed nontext body must meet the same condition without suppressing explicit font-family validation or text shaping. These assertions belong to this test harness; the productive owners remain responsible for their mechanisms. Record RED failures without weakening expectations to match current output.

Use public color values (`red`, `blue`, `black`) in shape/rotation fixtures, not strings that trigger cast errors. `.typ/sec_22.typ` contains normal and inverted bracket pairs; observe base character, glyph identity and advance and require equivalent vertical variants when positional role changes. That witness protects `00_nucleo/prompts/compiler/math/layout/delimited.md` without requiring a particular MathDelimited representation.

Bundle tests distinguish feature absence, out-of-target calls and the active target. Absent bundle feature must fail during eval with its exact gated diagnostic/hints, before layout/export. Active tests cover byte/UTF-8 assets, nested/normalized paths, duplicate asset/document paths, empty bundles, mixed PDF/SVG/PNG/HTML document entries, explicit/unknown formats, HTML without its feature and loose top-level content. Asset names/payloads are exact observables; document acceptance uses language/format structure. Do not accept an empty placeholder asset as data or leave outputs outside isolated temporary directories.

## Verification boundary

Retain existing integration cases and their owner boundaries; this prompt does not assert a current full-suite result or authorize productive changes to accommodate observer mechanics.
