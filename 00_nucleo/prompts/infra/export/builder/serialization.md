# L0 — builder/serialization
Hash do Código: ea83525b

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder/serialization.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/builder/serialization.rs:11` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## PDF object serialization

Own storage of numbered object bytes and final header, indirect objects, classic
xref and trailer, plus PDF literal/UTF-16BE escaping and dictionary-insertion helpers.
The already resolved standard supplies the supported PDF version; do not choose
standards or rewrite document features merely because the header version changes.
Preserve the binary header comment, object emission order and binary stream payloads.

Write real byte offsets for emitted objects and free xref entries for reserved IDs
that were not emitted within the emitted-ID range; do not manufacture dummy objects.
Size is the maximum emitted object ID plus one, not the maximum reserved ID plus one.
Root references the catalog allocated as object 1; optional Info references its
stored object. Reuse the ID pair
stored by metadata, in instance/document order, rather than regenerating it. The
defensive absent-pair path omits ID; productive build paths already populate it.
Reference parsing and insertion must not corrupt adjacent dictionary delimiters.

## Verification

Check version headers, sparse IDs, xref offsets, Size, Root/Info and metadata/trailer
ID consistency. Valid structure is not a claim of complete PDF-version conformance.
