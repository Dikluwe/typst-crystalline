# L0 — infra/export/tests/pdfa_core_metadata_tests
Hash do Código: 43690e0e

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/tests/pdfa_core_metadata_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/tests/pdfa_core_metadata_tests.rs:34` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Archival core regression coverage

Own the existing public-API test fixtures and assertions for universal trailer ID,
PDF-version metadata, archival pdfaid identification, output intent and Info
suppression. Exercise selected archival modes plus ordinary/UA controls, using
export_pdf_with_document_id_and_tags_and_serialization_and_pages_and_standard.

The normative emitters are
`00_nucleo/prompts/infra/export/builder/metadata.md` and
`00_nucleo/prompts/infra/export/builder/serialization.md`.
Preserve the suite's A1/A4f/A4e expectations independently of legacy test names. Do not construct a
new productive API or claim complete PDF/A validation from dictionary checks.

## Verification

Retain positive and negative field checks and metadata/trailer relationships.
The current assertions, not a historical pass total, define regression coverage.

Under the approved PDF warning carrier contract, the local export helper returns
`.output` for the existing byte assertions. The fixture remains warning-free;
do not filter exporter warnings or relax metadata relationships.
