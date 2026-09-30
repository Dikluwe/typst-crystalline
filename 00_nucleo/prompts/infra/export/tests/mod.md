# L0 — infra/export/tests/mod
Hash do Código: 0ccf548b

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/tests/mod.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/tests/mod.rs:36` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## PDF test aggregation

Own the existing cross-feature PDF cfg(test) suite, shared fixtures/helpers and
declarations of specialized test modules. Preserve test names/assertions and restricted
access to the public/internal exporter API. Keep genuine joint gradient/image/font/
layout coverage rather than duplicating each child suite in this aggregator.

Specialized test owners are the corresponding files under
`00_nucleo/prompts/infra/export/tests/`: archival core, attachment AF, ICC/CIDSet,
empty A4f, UA identification/structure, spot resources and page labels.
They own coverage, not an alternative productive export contract.
Production behavior is owned by the builder and stream descendants.

## Verification and exclusions

Run the existing export::tests suite and specialized modules without using a
historical fixed test total as an acceptance criterion. Check cross-feature
resource/geometry/text closure. Do not claim global language parity, conformance
or performance from test counts or full-file byte equality. Independent validators
and snapshot harnesses remain separate evidence.
