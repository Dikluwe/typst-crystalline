# L0 — builder/serialization
Hash do Código: 2b5b8ffd

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder/serialization.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/builder/serialization.rs:11` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

Before the UTF-16BE integration, the helper at
`03_infra/src/export/builder/serialization.rs:75-85` stages BOM-prefixed bytes in
a Vec, formats each byte separately, collects a hexadecimal String and then
constructs its angle-bracket wrapper. These are allocation and encoding
mechanics, not language-level parity requirements (ADR-0107/0108). The existing
helper's output establishes the encoding obligation below; changing the storage
organization must not change the represented Unicode text.

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

## UTF-16BE hexadecimal strings

For every valid Rust string, emit one owned output String containing exactly
`<FEFF`, four uppercase ASCII hexadecimal digits for each UTF-16 code unit in
big-endian digit order, and `>`. Empty input produces `<FEFF>`. Encode
supplementary scalars as their ordered surrogate pairs. Preserve embedded BOMs,
noncharacters, NUL, controls, PDF punctuation and distinct combining sequences;
do not normalize, replace, remove or reorder input characters or modify the input.

Count UTF-16 units and calculate `6 + 4 * units` with checked arithmetic and the
`isize::MAX` capacity-representation bound. For a representable length, reserve
the complete output initially and emit directly into that String without a
byte Vec, per-byte formatted String, intermediate hexadecimal String, or later
wrapper allocation. Count and emission may make separate passes over the input.
If the capacity calculation is unrepresentable, emit incrementally using normal
String growth instead; do not introduce a quota, public error, panic from unchecked
capacity arithmetic, or a promise of allocation-failure recovery. Encoding must
not require unsafe code. Capacity calculations at extreme unit counts must be
testable without allocating an impossible input.

This internal organization change follows ADR-0127 continuous flow. Keep public
signatures, serialization order, metadata and trailer identity, PDF modes and
defaults, and pipeline phases unchanged. The known semantic divergence of
outline construction remains outside this helper's scope and remains unresolved;
preserving its encoded titles does not certify outline language parity.

## Verification

Check version headers, sparse IDs, xref offsets, Size, Root/Info and metadata/trailer
ID consistency. Valid structure is not a claim of complete PDF-version conformance.

Keep inline regressions for the UTF-16BE contract: fixed BOM/byte-order vectors,
empty and repeated calls, all valid BMP scalars and supplementary boundary
samples, controls, embedded BOMs/noncharacters, punctuation and combining forms.
Independently derive expected code units from scalar values and decode emitted
units back to the original text. Check representable capacity arithmetic and its
overflow/boundary fallbacks without giant allocations. Verify the complete
initial reservation and absence of growth for ordinary inputs with an isolated
allocator or an equivalent focused regression. Encoding or decoded-text changes,
unexpected growth or additional temporary allocations refute the corresponding
obligation; PDF byte checks are encoding/structure regression checks, not a
substitute for semantic, syntactic or morphological language parity.
