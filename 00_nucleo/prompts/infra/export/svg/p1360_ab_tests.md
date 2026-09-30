# L0 — svg/p1360_ab_tests
Hash do Código: 3d915fe7

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/svg/p1360_ab_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/svg/p1360_ab_tests.rs:9` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Pretty A/B tests

Own the existing compact/pretty A/B fixtures, names and assertions. Verify structural
equivalence after removing structural whitespace and actual indentation in pretty
output. Do not change SvgOptions, pipeline routing or productive contracts.
