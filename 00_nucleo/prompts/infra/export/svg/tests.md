# L0 — svg/tests
Hash do Código: e284676c

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/svg/tests.rs:29` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Regression suite ownership

Own the existing cfg(test) SVG suite, fixtures, names/assertions, supplied fonts and
small resvg rasterizations. Exercise public output and private helpers via existing
restricted access. Expected productive behavior belongs to the specialized owners;
this suite does not redeclare paint/clip/context algorithms. Check morphology and
graph/reference closure; byte assertions are local regressions, not new vanilla parity.
