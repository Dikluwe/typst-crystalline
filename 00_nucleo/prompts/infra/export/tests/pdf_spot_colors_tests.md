# L0 — infra/export/tests/pdf_spot_colors_tests
Hash do Código: 7d581839

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/tiling-frame-transport.toml sha256:96c5b3ef9be3d171d3ca4c57f30fe826068c568fc17437f83b2b9b5d7372aeff

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/tests/pdf_spot_colors_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/tests/pdf_spot_colors_tests.rs:18` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Spot PDF regression coverage

Own the existing in-memory public-API tests for fill/stroke/text Separation, named/
None/All colorants, tint endpoints/midpoint, resource reuse and process controls.
Preserve same-name distinct-fallback identities, PDF-name escaping, Compact versus
Verbose resource controls and axial/radial/conic one-component gradients.

Exercise gradient fills in background as well as body, shape-local axial/conic
placement and reuse at distinct shape locations. Check that un-emitted reserved
object IDs are free xref entries, not in-use zero offsets. SVG/raster controls
observe process fallback color rather than Separation or exact PDF bytes.

The productive owners are
`00_nucleo/prompts/infra/export/builder/resources.md`,
`00_nucleo/prompts/infra/export/stream/shape.md`,
`00_nucleo/prompts/infra/export/gradients/mod.md` and their specialized children.
Do not promote test assertions into duplicate emission algorithms.

## Verification

Preserve colorant/tint and graph/reference checks. Independent qpdf/pdftoppm
evidence is external to these Rust tests; a historic pinned run is not proof
against a newer upstream pin.


## Prepared Shape fixture transport

### Evidence and approval

`lab/results/construction/diagnostics/tiling-nativo-2026-10-14/before.json.gz`
retains HEAD/tree/diffstat/UTC/sources/owners before migration. layout_types.rs:510–511
carried Paint/Stroke; shape.rs:151–191 materialized only group fills; tiling.rs:85
limited/cloned its grid. prepare_cell already returns TilingFrame once per
occurrence. `tiling-preparacao-2026-10-14/{before,after}-renders/results.json.gz`
shows dense-open/stroke-open divergences across all three targets. Manifesto
`d0c1f0c189769ccb40eb538e09e251ce1fd56650cbb23b2b94c357a1341ba32d` was approved;
receipt `lab/history/construction/records/00_nucleo/materialization/tiling-transporte-2026-10-14/approval-receipt.json`.
Preparation/object counts are transport mechanics; rendering/outlines/text/geometry/
diagnostics are language observables.

### Contract and acceptance

Migrate FrameItem::Shape fixtures to FramePaint/FrameStroke with unchanged values/
semantic assertions. Tiling explicitly constructs a prepared cell; eval fixtures
remain declarative. No total productive conversion or weakened assertion.
Consume the pinned nucleus. Approval supersedes only old approval-pending or
Shape-declarative-Paint/Stroke clauses within this package; other behavior survives.
L0 first, RED→GREEN, explicit fixture/consumer migration, no total From<Paint>,
carrier PartialEq or solid fallback merely to compile. Do not raise the old limit
or move layout to L3. Validate fill/stroke/resource/clipping/affine Parent/native-
target/Solid/Gradient/page semantics and full suite/build/doc/fmt/lineage. The
prepared transport, native-target integration and parity slices recorded as
T1c/T2/T3 are complete only after integrated verification; broken/partial states
are no production certificate. T4 and publication remain out of scope.
