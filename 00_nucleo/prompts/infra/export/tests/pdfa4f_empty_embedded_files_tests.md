# L0 — infra/export/tests/pdfa4f_empty_embedded_files_tests
Hash do Código: 711861db

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/tests/pdfa4f_empty_embedded_files_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/tests/pdfa4f_empty_embedded_files_tests.rs:32` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Empty A4f attachment regression coverage

Own the existing tests that exercise empty and nonempty attachment documents through
the public PDF API. Verify empty EmbeddedFiles Names in A4f without Filespec/payload/
AF objects; retain the negative controls for other empty archival modes and positive
nonempty relationships.

Production ownership is
`00_nucleo/prompts/infra/export/builder/navigation.md`; metadata observations use
`00_nucleo/prompts/infra/export/builder/metadata.md`.
Keep the current fixture imports/assertions and test-module ownership.

## Verification

Resolve the name tree and absence/presence of attachment objects independently
of their global IDs and total serialized bytes.

Under the approved PDF warning carrier contract, the local export helper returns
`.output` for the existing byte assertions. The fixture remains warning-free;
do not filter exporter warnings or relax any archival assertion.
