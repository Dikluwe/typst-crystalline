# Prompt L0 — `entities/metadata_store`
Hash do Código: 8edf6fb4

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/metadata_store.rs`
**Governance:** ADR-0033, ADR-0066.

## Evidence and responsibility

The original comparison used upstream MetadataElem and generic
ElementIntrospector.elems. Crystalline intentionally isolates metadata in its
own sub-store, alongside label/counter stores. This representation difference
does not require copying upstream mechanics.

## Contract

```rust
use crate::entities::value::Value;

#[derive(Debug, Clone, Default)]
pub struct MetadataStore { /* private Vec<Value> */ }

impl MetadataStore {
    pub fn empty() -> Self;
    pub fn query(&self) -> &[Value];
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;

    pub(crate) fn add(&mut self, value: Value);
}
```

Store embedded metadata Values in insertion/appearance order using a private
Vec. empty() equals Default and query() yields an empty slice. add is
crate-private, append-only, for construction through from_tags; afterwards
external callers see read-only queries. No deduplication: equal metadata
values create separate entries. Keep Debug/Clone/Default, len and is_empty,
with no I/O. from_tags handles ElementPayload::Metadata; TagIntrospector's
metadata/query_metadata exposure belongs to its introspector owner.
The facade reexport belongs to `00_nucleo/prompts/core/entities/mod.md`.
Rich metadata.where(predicate) queries remain separate future scope; the old
creation-time "no consumers" status is history, not a present restriction.

## Acceptance

empty().query() is &[]; one add yields &[value]; three adds yield three
entries in order; heterogeneous Int/Str/Bool coexist. Preserve duplicate
values, local tests, and read-only external access.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/metadata_store.md`. Historical observations are not a new parity certification.
