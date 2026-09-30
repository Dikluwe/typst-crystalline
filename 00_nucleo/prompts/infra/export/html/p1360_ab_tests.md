# L0 — html/p1360_ab_tests
Hash do Código: 855426c5

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/html/p1360_ab_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/html/p1360_ab_tests.rs:11` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Serialization A/B tests

Own the existing mode/pretty A/B fixtures, names and assertions. Verify unchanged
ordered attributes and escapes, protected title/phrasing whitespace and quoted >.
Compare compact/pretty DOM without treating formatting bytes as language parity.
The façade owns API/defaults and the encoding/pretty children own implementation
rules; this suite does not redeclare them.
