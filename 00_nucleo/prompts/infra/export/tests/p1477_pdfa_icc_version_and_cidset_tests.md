# L0 — infra/export/tests/p1477_pdfa_icc_version_and_cidset_tests
Hash do Código: 8bbfec25

**Layer:** L3 (test-only)
**Exclusive consumer:** `03_infra/src/export/tests/p1477_pdfa_icc_version_and_cidset_tests.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0126, ADR-0127, ADR-0128, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/tests/p1477_pdfa_icc_version_and_cidset_tests.rs:29` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## ICC and CIDSet regression coverage

Own the existing tests for version-selected vendored sRGB profiles and CIDSet in
single/multifont embedded CIDFont paths, including default/no-profile controls.
Use shared public exporter/test helpers and the existing zlib decompression helper.
Retain CIDSet bit-density/remapping checks and FlateDecode assertions on both paths.

`00_nucleo/prompts/infra/export/images/icc.md` owns profiles;
`00_nucleo/prompts/infra/export/builder/fonts.md` owns CIDSet;
`00_nucleo/prompts/infra/export/builder/resources.md` owns resource binding.
The existing local name raw_bytes is retained without making a parser workaround
a productive Rust-language requirement.

## Verification

Check profile bytes/version and effective CID bits, not full PDF identity.
Historical parser investigation and suite extraction are diagnostic evidence only.
