# L0 — infra/export/tests/p1486_pdf_page_labels_tests
Hash do Código: c8423432

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/tests/p1486_pdf_page_labels_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/tests/p1486_pdf_page_labels_tests.rs:13` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Selected-page label regression coverage

Own in-memory public-API tests that decode the PageLabels number tree and style
into effective reader labels, not merely detect a catalog key.
Cover no selection, full/subrange/discontinuous/single/empty selection, Arabic/
Roman/literal prefix/suffix/no-pattern cases and shifted logical_number.
Retaining every physical page does not require a fallback corrective label tree.

Productive label/filter mapping is owned by
`00_nucleo/prompts/infra/export/builder/navigation.md`.
Check tag/archival/UA coexistence and catalog/tree coherence without fixed object
IDs or whole-PDF byte comparison.

## Verification

Preserve the fixture matrix and per-retained-page labels. Independent qpdf/pypdf
and original bilateral fixture evidence remain separate from local regression tests.
