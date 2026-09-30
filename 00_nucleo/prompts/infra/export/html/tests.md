# L0 — html/tests
Hash do Código: b327f103

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/html/tests.rs:13` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Regression suite ownership

Own the existing cfg(test) HTML suite and its fixtures/assertions. Cover serialization
modes, ordered DOM/attributes, void/presence errors, styled raw/title, whitespace,
explicit roots, ruby, tables/spans, protected pretty contexts and frame resolution.
Exercise public paths and private helpers through existing restricted access.
Expected behavior comes from the productive owners, not a second normative encoding
contract here. Byte assertions are encoder regressions, not universal vanilla parity.
Do not add network, layout or I/O merely to reorganize this suite.
