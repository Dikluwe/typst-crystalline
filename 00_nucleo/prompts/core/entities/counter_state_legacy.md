# Prompt L0 — `entities/counter_state_legacy`
Hash do Código: 66f7e722

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/counter_state_legacy.rs`

## Contract and legacy boundary

CounterStateLegacy is the mutable single-pass layout numbering aggregate.
The rename from CounterState freed that name for the modern counter domain
without changing behavior or signatures; existing walks still receive
&mut CounterStateLegacy. It retains hierarchical HashMap<String,Vec<usize>>,
flat HashMap<String,usize>, and numbering-active HashMap<String,bool>.
Heading [1,2] means section 1.2; flat counters serve figures/equations/custom keys.

Preserve accumulated auxiliary state: resolved_labels,headings_for_toc,
label_pages,known_page_numbers,has_outline,is_readonly,figure_numbers,
figure_label_numbers,local_figure_counters,lang,bib_entries,bib_numbers,
auto_label_counter. The original specification delegates exact field types
to its pre-rename history; this is an explicit specification gap, not permission
to infer new contracts from the implementation.

CounterAction remains here, Debug/Clone/PartialEq with Step and Update(usize),
for existing Content::CounterUpdate compatibility. The separate CounterUpdate
owner does not silently replace it.
CounterStateLegacy derives Debug/Clone/Default and preserves new()->Self,
is_numbering_active(&self,&str)->bool,
step_hierarchical(&mut self,&str,usize),
format_hierarchical(&self,&str)->Option<String>, step_flat(&mut self,&str),
update_flat(&mut self,&str,usize), get_flat(&self,&str)->usize and
display_value(&self,&str)->String.

## Semantics and acceptance

Hierarchical steps advance at the specified level with truncation/push;
flat steps add1, updates set exact values. Display renders hierarchy as 1.2
and flat values as 5. is_readonly blocks every step/update, including TOC
rendering with cloned syntax. Mutations go through explicit step/update/setter
methods so the guard can block them globally. Default is a valid empty state.
Pure in-memory L1 HashMap/Vec, no system I/O or shared mutable cross-thread Arc;
usage is single-threaded layout.

Clone costs O(total map entries+TOC+bibliography size). The original acceptance
rationale bounded cloning to a walk/materialize_time boundary (DEBT-18);
do not treat that historical claim as a fresh performance measurement.
Keep colocated tests and behavior unchanged. The historical claim that all
fields contribute to observable output is recorded, not a blanket waiver of
future measurement.

## Pending modernization

DEBT-10 and single-pass versus upstream introspection differences remain
separate. Introspection walk/materialize_time and layout counters/references/
outline/figure use this aggregate. The planned final fixpoint migration may
replace it or retain a flattened modern-state view through an explicit
contract, not by deleting historical fields during editorial cleanup.
The 2026-04-29 audit
`lab/results/construction/diagnostics/auditoria-isolamento-vs-vanilla.md`
records the original concentration of orthogonal responsibilities.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/counter_state_legacy.md`. Historical observations are not a new parity certification.
