# L0 — images/icc
Hash do Código: 4ec97dcd

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/images/icc.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/images/icc.rs:37` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Vendored ICC profiles

Own the vendored sRGB v2/v4 bytes, getters, version selector and ICC stream writer.
Preserve profile identity and license: v2 SHA-256 is
af4efe28f6d311799865f325ba39184a2e978b113ca74124d60af7de22b105f4.
PDF 1.4/1.5/1.6 select v2; 1.7/2.0 select v4 through exhaustive version handling.
Preserve N 3, Range [0 1 0 1 0 1], FlateDecode and stored-byte Length with the existing
compression helper/raw error fallback. Do not introduce grayscale profiles or alter
the fallback's existing envelope as part of documentation cleanup.

## Verification

Check version-to-profile selection, vendored identity and successful profile stream
decompression. Vendor provenance remains archived evidence, not a second owner.
