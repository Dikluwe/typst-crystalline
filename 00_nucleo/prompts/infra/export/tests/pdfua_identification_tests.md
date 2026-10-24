# L0 — infra/export/tests/pdfua_identification_tests
Hash do Código: 9aad43ad

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/tests/pdfua_identification_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/tests/pdfua_identification_tests.rs:17` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## PDF/UA identification regression coverage

Own the public-API fixtures/assertions for pdfuaid part 1 and catalog
ViewerPreferences/DisplayDocTitle true. Cover UA-only, non-UA controls and archival
plus UA, including coexistence with pdfaid and the minimal PDFUAIdentification
extension schema only for the combined mode.

Preserve the explicit local assertion that pdfuaid:part precedes xmpTPg:NPages.
This order is a serializer regression contract, not upstream byte equivalence.
The productive fields and ordering are owned by
`00_nucleo/prompts/infra/export/builder/metadata.md`.
Title-required validation belongs to the pipeline outside this test owner.

## Verification and scope

Check exact field values/presence and combined schema properties through
export_pdf_with_document_id_and_tags_and_serialization_and_pages_and_standard.
Do not infer complete PDF/UA conformance, Direction, language, assistive-technology
behavior or title validation from this identification suite.

Under the approved PDF warning carrier contract, the local export helper returns
`.output` for the existing byte assertions. The fixture remains warning-free;
do not filter exporter warnings or broaden the conformance claim.
