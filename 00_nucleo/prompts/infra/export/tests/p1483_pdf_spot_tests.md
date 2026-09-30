# L0 — infra/export/tests/p1483_pdf_spot_tests
Hash do Código: ca14f726

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/tests/p1483_pdf_spot_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/tests/p1483_pdf_spot_tests.rs:18` identifies the current exclusive consumer. The clauses below
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
