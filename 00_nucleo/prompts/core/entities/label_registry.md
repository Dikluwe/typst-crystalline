# Prompt L0 — `entities/label_registry`
Hash do Código: 094d7b65

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/label_registry.rs`
**References:** ADR-0033,ADR-0066,ADR-0073.

## Domain and evidence

LabelRegistry maps Label to multiple Locations, constructed by from_tags from
Start tags with info.label present. The original audit required distinguishing
zero/one/many locations for query_labelled/label_count, superseding the earlier
single-location map. Upstream ElementIntrospector uses MultiMap<Label,usize>;
crystalline intentionally isolates HashMap<Label,Vec<Location>>. Structural
file sizes/implementation counts in the original audit are historical, not
new parity evidence. Related audit:
`lab/results/construction/diagnostics/auditoria-isolamento-vs-vanilla.md`.

## Public contract

LabelRegistry derives Debug/Clone/Default, pure L1,no I/O. Clone satisfies
comemo tracking. Public empty()->Self uses Default;
lookup(&self,&Label)->Option<Location> returns first inserted or None;
lookup_all(&self,&Label)->&[Location] returns all in insertion order or &[];
count(&self,&Label)->usize equals that slice length;
len(&self)->usize counts unique labels,is_empty(&self)->bool;
iter(&self)->impl Iterator<Item=(&Label,&Location)> yields every pair.
Only crate-private add(&mut self,Label,Location) mutates during construction.

add pushes, preserving even duplicate identical pairs. A stored label with
zero locations is not representable. External callers have read-only access.
iter sorts labels alphabetically by Label.0 String,then emits consecutive
locations in insertion order; internal HashMap key order is irrelevant.
Sorting unique labels costs O(n log n); total pairs are iter().count(),not len.
The original equality invariant is internal-map equality including vector
order; its listed derives omit Eq/PartialEq, so it does not newly authorize
public equality traits.

The separate figure_label_numbers HashMap<Label,usize> still assumes one label
per figure; this multi-label contract does not change it. TagIntrospector
query_by_label/query_labelled and from_tags are separate consumer owners;
the original 'no consumers yet' is a construction snapshot, not current policy.

## Acceptance

Empty lookup None,len0,is_empty true,iter empty. Insert/get resolves; five
distinct labels retain independent values. Repeated label insertion preserves
order and first-lookup compatibility; lookup_all returns N items,count N,
and unknown label returns[]/0. Insert at least three labels in arbitrary order:
iter sorts alphabetically independently of insertion and keeps each group's
location order/duplicates. Preserve seven public methods,colocated tests and
the declared facade exposure.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/label_registry.md`. Historical observations are not a new parity certification.
