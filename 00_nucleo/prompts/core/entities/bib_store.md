# Prompt L0 — `entities/bib_store`
Hash do Código: 864c9768

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/bib_store.rs`

## Design evidence

The original bibliography inventory and construction plan preserved the legacy Vec<BibEntry> plus HashMap<String,u32> shape, appearance-order concatenation and first-assigned-number retention. Historical `introspect.rs:569–572` assigned numbers before extend; `layout/mod.rs:584,590` used linear entry lookup and map-number lookup. These establish local responsibilities, not a requirement to copy upstream IndexMap/ManuallyHash/comemo.

## Public and construction interface

```rust
use std::collections::HashMap;

use crate::entities::bib_entry::BibEntry;

#[derive(Debug, Clone, Default)]
pub struct BibStore {
    entries: Vec<BibEntry>,
    numbers: HashMap<String, u32>,
    citation_order: Vec<String>,
    back_refs: HashMap<String, Vec<usize>>,
}

impl BibStore {
    pub fn empty() -> Self;
    pub fn entries(&self) -> &[BibEntry];
    pub fn entry_for_key(&self, key: &str) -> Option<&BibEntry>;
    pub fn number_for_key(&self, key: &str) -> Option<u32>;
    pub fn len(&self) -> usize;
    pub fn numbers_len(&self) -> usize;
    pub fn is_empty(&self) -> bool;

    pub fn citation_number_for_key(&self, key: &str) -> Option<u32>;
    pub fn citation_order(&self) -> &[String];
    pub fn back_refs_for_key(&self, key: &str) -> Vec<usize>;

    pub(crate) fn add_bibliography(&mut self, entries: Vec<BibEntry>);
    pub(crate) fn assign_number(&mut self, key: String, number: u32);
    pub(crate) fn record_citation(&mut self, key: String);
}
```

Keep a pure TagIntrospector substore without I/O/global state, externally read-only after construction. Only crate-visible construction methods mutate it. Preserve Debug/Clone/Default, ordinary std HashMap and the absence of added PartialEq/Eq; changing representation or traits requires its own contract.

empty equals Default. entries returns insertion order, concatenating multiple bibliographies without deduplication. entry_for_key linearly returns the first exact BibEntry.key match or None; number_for_key returns the stored number or None. len is entry count; numbers_len counts distinct numbered keys, including duplicates only once. is_empty is exactly entries.is_empty() && numbers.is_empty().

add_bibliography extends entries and assigns no numbers. The caller separately computes next_num=numbers.len()+1 and calls assign_number; assign_number uses entry(key).or_insert(number), preserving the first number. The construction protocol keeps numbers.len()<=entries.len(); BibStore itself does not invent numbers.

record_citation adds a key to citation_order only on first appearance, and always appends the next global one-based citation position to back_refs: the sum of all existing back-reference lengths plus 1. citation_number_for_key returns the one-based position in first-citation order; citation_order returns its slice. back_refs_for_key returns all global one-based positions, or an empty Vec for an uncited key. Preserve repeated citations and their order.

## Acceptance

empty has empty entries, len0, is_empty true and no entry/number for any key. Two add calls of two entries each yield lengths 2 then 4. Assigning a→1 then a→2 retains 1. Missing entry/number returns None; a populated intro key returns the corresponding borrowed entry. Multiple bibliography inputs retain insertion order. Verify first-citation numbering and repeated global back references without conflating them with bibliography-entry numbers.

## Integration and unresolved historical limits

Bibliography payload extraction, locatable walking and from-tags population remain separately owned. Introspector bibliography access delegates to this store; layout citation consumers query through Introspector rather than mutating it. The facade declares pub mod bib_store without general pub use; TagIntrospector's public bib_store composition is specified by its own owner.

The old claim of no consumers is construction history. Likewise, the initial no-hayagriva/no-CSL four-form description predates later CSL integration and is not a current product-wide prohibition or certification. This store retains its explicit representation and linear lookup; a future IndexMap/EcoString/hayagriva reformulation needs a dedicated specification while preserving the substore role. Historical E2E and back-reference rendering (` ↑[1][3]`) belong to integration evidence, not rendering logic in this owner.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/bib_store.md` preserves the original evidence and chronology.
