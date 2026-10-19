# L0 — Controlled PDF snapshot corpus
Hash do Código: 2be850e7

**Layer:** L3 (tests and fixtures)
**Exclusive consumer:** `03_infra/src/pdf_snapshot_tests.rs`

## Current evidence

- `03_infra/src/pdf_snapshot_tests.rs` — `compile_fixture` and `assert_bytes_eq`.
- `03_infra/fixtures/p307b/sources/`, `reference/` and `MANIFEST.md` — current input/reference corpus.
- `03_infra/fixtures/fonts/NotoSans-Regular.ttf` and `LICENSE-NotoSans.md` — the CIDFont input and license.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own only the binary snapshot harness and its controlled corpus, not PDF production algorithms. This is crystalline-to-crystalline regression evidence, distinct from language parity against upstream. Byte identity is the selected snapshot observable; it does not prove visual correctness, upstream PDF byte equality or performance.

| Input stem | Primary observable |
| --- | --- |
| 01-markup-plain | Basic text and string escaping |
| 02-markup-heading | Heading/text emission |
| 03-text-styling | Styled bold/italic text |
| 04-shapes | Rect/circle and solid paint |
| 05-gradient-linear | Linear gradient fill |
| 06-gradient-conic | Conic gradient fill |
| 07-multi-feature | Mixed structure/gradient and multiple pages |
| 08-image-jpeg | JPEG resource, with `sources/tiny.jpg` |
| 09-cidfont | Noto Sans subset, CIDFont and text mapping |

Resolve inputs/reference PDFs through `CARGO_MANIFEST_DIR`, not a developer's absolute checkout path. The harness creates a SystemWorld for each source and supplies the repository font directory only for the CIDFont case. Preserve Compact streams, Disabled tags and `CRYSTALLINE_PDF_FIXED_EPOCH=0` as explicit snapshot controls; these do not change the production defaults. Keep source/reference pairing and the licensed font identity recorded in MANIFEST.

Add a fixture for a newly exercised export path or an uncovered regression; remove it only when the behavior is removed or equivalent coverage replaces it. Regenerate references only for an intentional observable change with recorded justification. The current harness reads reference files at runtime and has an explicit `UPDATE_P307B_SNAPSHOTS` maintenance path. Use that harness rather than the removed include_bytes template or obsolete CLI invocation.

Regeneration must preserve the fixed epoch/stream/tag controls, select affected cases, repeat generation to verify deterministic bytes in the same environment, check PDF validity and refresh the operational MANIFEST. Do not bless unrelated reference changes. Do not promise identical output across toolchains, uncontrolled fonts or production identifier defaults. Exporter clocks, identifiers and deterministic serialization belong to their excluded owners, not this fixture prompt.

## Verification boundary

The existing p307b tests compare actual/reference bytes. Maintenance runs may use `cargo test -p typst-infra --lib p307b_ -- --test-threads=1` with the explicit update environment only when regeneration is authorized. No fixture/reference regeneration is part of documentary sanitation.

## Controlled reference changes: evidence and exact scopes

These historical maintenance decisions authorize only their stated cases, not
regeneration during this prose consolidation. Preserve every harness control above
and verify current references through existing tests. Original receipts remain in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/infra/pdf_snapshot_tests.md`.

- CIDFont: tracking-pdf-2026-09-28/tests-before-reference-update.log records only
  p307b_09_cidfont failing after intentional explicit Tc=0 in shaped Compact,
  6, 768 versus 6, 759 bytes, same source/font/no tracking. This prevents inherited
  spacing and alone proves no visual/language regression. Regenerate only
  09-cidfont twice with existing controls; valid PDF/unchanged previous pixels/text,
  record bytes/hash/stream differences and MANIFEST. No other cases/binary-check
  disablement/new exporter algorithm.
- JPEG: p307b_08_image_jpeg changes 3, 557→3, 541 bytes after redundant clip removal
  where clip rectangle equals target. Matrix/JPEG bytes remain; bilateral raster
  matches at 36/72/144/288 DPI. Regenerate only 08-image-jpeg in Compact/Disabled/
  fixed epoch, repeat deterministic bytes/PDF validation/MANIFEST hashes; other
  eight snapshots stay byte-identical.
- Named destinations: destinos-pdf-2026-10-14/workspace.log records only
  02-markup-heading/07-multi-feature failures after name-tree/GoTo correction,
  2, 709→2, 754 and 15, 032→15, 112 bytes. Independent MuPDF resolves new destinations,
  not old. Full state/suite and pre-maintenance refs/fonts/owner retained in
  snapshots/before.json.gz. Regenerate only 02/07 with same controls, repeat,
  qpdf, compare decoded objects/streams/text/render to old: only names change,
  never paint/content. Other seven PDFs byte-identical; all nine tests without
  UPDATE, recorded differences/MANIFEST, binary comparison intact.
- Conic physical positioning: conic-posicionamento-2026-10-14/first-validation/
  workspace.log records only p307b_06_gradient_conic, 5, 327→5, 350 bytes. Independent
  red versus conic(red,red) and five RED→GREEN controls show missing scale/physical
  position including page/group; circular fixture's render change is intentional,
  no failure concealment. Suite SIGTERM after infra is not full validation and
  cause is not inferred. snapshot-before.json.gz retains corpus/source/L0/HEAD/
  UTC/diffstat/hashes before regeneration. Only 06 changes, same controls/two
  deterministic generations/qpdf/raster; object inspection proves only pattern
  Matrix/derived offsets change, same mesh/paint stream. Other eight PDFs/sources/
  assertions intact, MANIFEST/full corpus without UPDATE and full suite. This
  slice alone does not certify center/coverage/CMYK sampling, completed only by
  separate geometry contract below.
- Conic geometry/sampling: conic-geometria-amostragem-2026-10-14/conic-tests.log
  records only 06 divergence in conic tests; geometry-final-tests.log confirms
  center/physical radius/orientation/CMYK offsets/discontinuities/center
  representation. final-coverage/boundary-check retain HEAD/tree/binary; RGB
  unrotated coverage is recovered. Rotated MuPDF solid/conic agree, Poppler retains
  edge differences. Dictionary-stroke controls did not exercise gradients because
  paint was ignored, no stroke evidence; solid/gradient CMYK color remains open,
  no masks. Regenerate only 06 with center/physical geometry/color/16-bit coordinates
  and matching Decode. Expected change is Shading/derived lengths-offsets, preserving
  Matrix/paint stream/other objects. Two identical generations/qpdf/raster, eight
  other PDFs/fonts intact, MANIFEST/corpus/full suite without UPDATE. No vanilla
  byte identity or certification of all color spaces/transparency/stroke/extreme
  domains from this one fixture.
