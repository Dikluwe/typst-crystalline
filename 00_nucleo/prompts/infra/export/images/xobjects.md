# L0 — images/xobjects
Hash do Código: c1e271e1

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/images/xobjects.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/images/xobjects.rs:9` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Image XObject streams

Own JPEG, alpha-mask and raster-main stream writers. JPEG preserves original bytes
with DCTDecode, 8 bits/component, supplied dimensions and color space/ICC reference.
Alpha masks use DeviceGray/FlateDecode; raster main streams use compressed RGB and
SMask only when its reference is supplied. Length always counts stored binary bytes.
Do not coerce payloads through UTF-8, re-decode images or choose ICC profiles here.

## Verification

Check binary payload preservation, dimensions, Filter/ColorSpace/SMask and exact
stored-byte Length independently of object allocation.
