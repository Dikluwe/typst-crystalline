# L0 — infra/export/tests/p1476_pdfa_attachments_af_tests
Hash do Código: fb976975

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/tests/p1476_pdfa_attachments_af_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/tests/p1476_pdfa_attachments_af_tests.rs:30` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Associated-file regression coverage

Own the existing fixtures/assertions for AFRelationship and catalog AF in
A3a/b/u and A4f/e, their non-associated controls and extended archival metadata.
Construct AttachedFile/PdfAttach carriers through the existing public representation.
Exercise the existing fully explicit PDF entrypoint without a new builder API.

Productive relationships and attachment serialization belong to
`00_nucleo/prompts/infra/export/builder/navigation.md`; identification, Info and
output intent belong to `00_nucleo/prompts/infra/export/builder/metadata.md`.
This suite does not redeclare those algorithms.

## Verification

Check permitted/omitted AF fields, relationship values, referenced payloads and
archival coexistence; preserve present assertions and names. Do not replace them
by a fixed historical test total or universal PDF-byte parity.
