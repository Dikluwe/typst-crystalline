# L0 — builder/streams
Hash do Código: c5d4b52a

**Layer:** L3
**Exclusive consumer:** `03_infra/src/export/builder/streams.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130

## Consumer evidence

`03_infra/src/export/builder/streams.rs:20` identifies the current exclusive consumer. The clauses below
are checked against that source and its delegated calls. They specify current
export observables and ownership, not full upstream byte or algorithm equality.

## Physical page-stream envelope

Own only the byte envelope around already generated page operators. In Compact
serialization, use FlateDecode when compression succeeds and reduces length;
otherwise retain raw bytes and omit Filter. In Pretty serialization, page content
streams remain uncompressed and readable. This exception does not change compression
of fonts, images, ICC profiles or other streams, nor graphics StreamMode or PdfTags.
Apply the existing oracle transformation only when explicitly enabled; disabled is
identity. Do not create graphics operators or choose product defaults.

## Verification

Check profitable/unprofitable compression, failure fallback, readable Pretty streams
and oracle-off identity without conflating physical serialization and graphic mode.
