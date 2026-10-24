# L0 — infra/export/tests/pdfa_associated_files_tests
Hash do Código: 626d0fa3

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/tests/pdfa_associated_files_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/tests/pdfa_associated_files_tests.rs:30` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Associated-file regression coverage

Own the existing fixtures/assertions for AFRelationship and catalog AF in
A3a/b/u and A4f/e, their non-associated controls and extended archival metadata.
Construct AttachedFile/PdfAttach carriers through the existing public representation.
Exercise the existing fully explicit PDF entrypoint without a new builder API.

Productive relationships and attachment serialization belong to
`00_nucleo/prompts/infra/export/builder/navigation.md`; identification, Info and
output intent belong to `00_nucleo/prompts/infra/export/builder/metadata.md`.
This suite does not redeclare those algorithms.

## Verification

Check permitted/omitted AF fields, relationship values, referenced payloads and
archival coexistence; preserve present assertions and names. Do not replace them
by a fixed historical test total or universal PDF-byte parity.

Under the approved PDF warning carrier contract, the local export helper returns
`.output` for the existing byte assertions. The fixture remains warning-free;
do not filter exporter warnings or relax any associated-file assertion.

## Explicit optional absence and closed-domain preservation — 2026-10-23

### Evidence before decision

The exact source and owner bytes, HEAD, dirty-tree stat and initial nine V16
findings are preserved in
`lab/results/v16-option-exhaustiveness-2026-10-23/before-state.json`,
`before/` and `before-v16.sarif`. The parser source
`tekt-linter/03_infra/rs_parser.rs:1605–1621` classifies every bare identifier
pattern as a catch-all, including the Option unit variant `None`. The Rust
compiler resolves that name to the unit variant; qualification makes that
existing nominal meaning explicit to both tools. These are mechanical
coverage observations, not a newly discovered runtime failure or new vanilla
parity claim.

### Exclusive correction and acceptance

`pdfa_associated_files_tests.rs:117–123` independently enumerates the four AttachedFileRelationship variants and absence. Qualify only the absence pattern as `Option::None`; preserve the independent expected-name oracle, Unspecified mapping, all attachment assertions and current fixtures.

Use ADR-0127 continuous internal flow: no public contract, default, phase or
compatibility changes. Add no lint exception, allow/expect attribute, severity
change, suppression or match-to-if rewrite. Preserve all existing assertions
and pending limitations. The initial V16 gate is structural RED; preservation
tests may already pass before implementation and must not be misreported as
behavioral RED. Require the final unfiltered linter, tests, strict Clippy,
rustdoc, formatting, canonical paths and both lineage directions.
