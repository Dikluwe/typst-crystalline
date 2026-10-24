# L0 — infra/export/tests/pdf_page_labels_tests
Hash do Código: cc817ee5

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/tests/pdf_page_labels_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/tests/pdf_page_labels_tests.rs:13` identifies the current exclusive consumer. The clauses below
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


## Local label-observer expressions

### Evidence

`lab/results/construction/diagnostics/clippy-expressions-2026-10-16/before.json.gz`
retains sources/L0/HEAD/uncommitted tree/status/diffstat/UTC;
initial-diagnostics.json retains suggestions/spans for pdf_page_labels_tests.rs:111,
:116 (clippy::regex_creation_in_loops). Uses/types were inspected first. This is
internal Rust representation, no new upstream parity/performance claim.

### Contract and acceptance

Compile constant St/S Regex once per observer invocation before the entries loop.
Preserve exact patterns/captures, 1/None defaults, prefix interpretation and every
per-page control. No globals or regex sharing across invocations. Continuous
ADR-0127 adds no public/default/phase/dependency/suppression change. L0 first,
scoped resealing, unchanged control inputs/results/tolerances. Selected-lint
RED→GREEN, workspace suite/build/all-target Clippy/fmt/Rustdoc/lineage; no
syntax-mirror tests.

Under the approved PDF warning carrier contract, test helpers extract `.output`
from `PdfExport` before existing byte assertions. Fixtures in this owner are
warning-free; do not suppress or alter exporter diagnostics to satisfy them.
