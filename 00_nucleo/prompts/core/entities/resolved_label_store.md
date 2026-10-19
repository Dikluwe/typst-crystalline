# Prompt L0 — `entities/resolved_label_store`
Hash do Código: deda3ba3

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/resolved_label_store.rs`
**References:** ADR-0033, ADR-0066.

## Scope and evidence

ResolvedLabelStore maps Label to final resolved reference text in
TagIntrospector. The original two-axis audit chose a final snapshot read after
the full walk, with no location-aware variant. It continues the existing
legacy HashMap<Label,String> simplification and BibStore substore pattern;
upstream Locator::resolve's richer Resolved/path/key/content is an architectural
comparison, not a claim that these representations are equivalent.

## Contract

ResolvedLabelStore derives Debug/Clone/Default, without PartialEq/Eq, and has
private labels:std::collections::HashMap<Label,String>. Pure L1, no I/O/global
state. Read-only externally after construction; only crate-private insertion
is allowed while constructing from_tags.

Preserve empty()->Self (Default), get(&self,&Label)->Option<&str>,
len(&self)->usize, is_empty(&self)->bool and crate-private
insert(&mut self,Label,String). get borrows the stored string without cloning;
insert overwrites an existing value, with no implicit deduplication. No order
guarantee. Keep std HashMap consistent with BibStore/LabelRegistry/StateRegistry.

TagIntrospector has public resolved_labels:ResolvedLabelStore beside BibStore.
Introspector::resolved_label_for(&self,&Label)->Option<&str> delegates to get.
The entity facade declares pub mod resolved_label_store without pub use.

## Acceptance

empty is empty,len0,get(any)None. Insert/get returns the text; separate labels
remain isolated; reinsertion overwrites. Introspector trait tests verify
correct delegation. Keep the struct/five methods, four local behavioral checks
and the cross-owner trait integration check.

## Historical transition and outstanding boundaries

The initial compatibility window deliberately left this store empty while
Heading/Labelled wrote legacy and layout references read legacy. The staged
handoff was: reference lookup with store.or_else(legacy), then Labelled tags/
from_tags insertion while Heading dual-wrote, then Heading migration and
removal of legacy mutation, finally removal of legacy field/fallback.
The original named follow-ups are preserved in the archive; this entity
contract alone does not certify that all stages finished. Do not enforce the
old empty-production phase as a permanent invariant or silently declare the
remaining migration complete.

No path information/content reference or cross-iteration comemo memoization
is provided by this bounded string store. A future Locator contract may change
representation while retaining the TagIntrospector/from_tags substore role;
cleanup does not authorize that change.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/resolved_label_store.md`. Historical observations are not a new parity certification.
