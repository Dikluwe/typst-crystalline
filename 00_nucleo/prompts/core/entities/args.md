# Prompt L0 — `entities/args` — arguments and causal occurrences
Hash do Código: 90e38261

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/args.rs`
**References:** ADR-0107/0108/0127/0129/0130.

## Evidence before causal arguments

Baseline b303f1f15b610e09872b567027e0d806387fde8c plus the archived uncommitted
P1306 tree was captured 2026-09-07T17:03:42.154788+00:00 before L0.
args.rs:17-32 held items/named/aggregate span; call_dispatch.rs:389-430,1016-1023
lost origins and earlier named occurrences in spread/With while AST still had
them. Independent historical a51e02804 probes distinguished origins of equal
Args through factory/sink and cast the first pretty:"bad" despite later Bool
override. Upstream foundations/args.rs:218-235 casts every named occurrence;
414-445,485-491 filter/map selectively retain origin;468-482 Add differs from
With. These are language observables, not required Rust memory layout.

The older claim of deferred spread/&[Value] natives is superseded by the
measured existing expansion/&Args at call_dispatch.rs:410-419,461-468.
Legacy local equality includes items/named/span, unlike upstream 461-465
which ignores spans; do not infer general equality parity.

## Exclusive proposed public contract

Args/ArgOccurrence belong only here; Func's owner does not redefine them.
Persistent carriers follow values, without global maps,value comparisons or
later AST recovery. Pure L1,no new I/O/global mutation/crate.

```rust
#[derive(Debug, Clone)]
pub struct ArgOccurrence {
    pub name: Option<EcoString>,
    pub value: Value,
    pub span: Span,
    pub value_span: Span,
}

#[derive(Clone)]
pub struct Args {
    pub items: Vec<Value>,
    pub named: IndexMap<EcoString, Value, FxBuildHasher>,
    pub span: Span,
    pub occurrences: Option<Vec<ArgOccurrence>>,
}

impl Args {
    pub fn positional(items: Vec<Value>) -> Self;
    pub fn from_parts(items: Vec<Value>, named: IndexMap<EcoString, Value, FxBuildHasher>, span: Span) -> Self;
    pub fn from_occurrences(span: Span, occurrences: Vec<ArgOccurrence>) -> Self;
    pub fn occurrence_sequence(&self) -> Vec<ArgOccurrence>;
    pub fn invalidate_occurrences(&mut self);
    pub fn remove_positional(&mut self, index: usize) -> Option<Value>;
    pub fn remove_named(&mut self, name: &str) -> Option<Value>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

```

Occurrence name None is positional,Some retains exact name. span anchors the
whole argument,value_span its expression,each retaining FileId. Detached is
real missing origin,never permission to invent final-call Source. Aggregate
span is independent and never reconstructs individual anchors.

## Authoritative sequence and projected views

Some(sequence) retains joint order and every named occurrence including values
needed by future casts. items projects positional order; named projects last
value per name but keeps first-occurrence key order. Values are clones of the
same evaluation,never reevaluated. from_occurrences builds sequence/views
together. from_parts preserves supplied views/span with None; positional uses
empty named,detached span,None. Legacy len counts only items; is_empty reads
both views.

occurrence_sequence clones Some. For None it synthesizes positional then named
view order with both individual spans detached; it cannot recover lexical
order/origin. Combining synthetic and known fragments detaches only the
synthetic part,not known anchors.

## Coherence under mutation

remove_positional counts positional occurrences only,removes the exact index,
returns value; out-of-range is None without mutation. remove_named removes all
same-name occurrences and returns the last; absent is None without mutation.
With Some,both regenerate views; with None,modify views and retain None.
Aggregate span stays unchanged; no casts or diagnostic precedence here.

Before direct items/named mutation with Some,caller must invalidate_occurrences,
which only clears carrier,not views/span. None needs no redundant invalidation.
This audited synthetic/legacy adaptation is forbidden when user argument origin
would be lost; use coherent methods or transform sequence/from_occurrences.
Direct sequence edits cannot leave stale views; rebuild entire Args. Readers
must not silently choose one inconsistent copy; occurrence_sequence is not a
repair. Changing aggregate span alone never invalidates/rewrites individual
origins. No PartialEq/Value hash/binding name/text search proves origin; equal
values and NaN are not causal identity.

Public fields cannot enforce these rules in Rust. Audit every writer as
synthesis,transport or consumption before implementation; missing authorized
owner blocks migration rather than licensing fallback.

## Clone, equality and ownership boundaries

Clone preserves values/spans without World access or Func execution.
PartialEq deliberately retains legacy items/named/span and ignores occurrences;
never derive it including the new carrier or fix upstream equality debt in this
slice. Debug keeps its prior shape without carrier. Language repr belongs to
`00_nucleo/prompts/core/compiler/eval/repr.md`; collection projections/callbacks
and loading casts/encoders retain their own owners.
Args does not validate signatures,choose winning errors or add typed take<T>/
finish. Natives may use local occurrence cursors to select arg/value span for
failure; dispatcher must not duplicate that decision.

## Approval provenance and acceptance

The original public fields/types/methods proposal requires ADR-0127 approval
because external struct literals break. Its active text lacks a receipt;
retain that provenance gap without treating existing implementation as approval.
The approved migration must replace synthetic literals with from_parts/
positional and transports with occurrences,without forcing every consumer into
new validation. Each owner stays 1:1; drafting alone did not authorize Rust/tests.

After approval require independent checks: repeated named projections,ordinal/
named removal,cross-Source clone,origin-free synthesis,mixed None/Some retaining
anchors,first invalid named casts,factory/sink Args,map/filter/join per owner,
legacy equality including aggregate span/NaN without using equality as origin
proof. Required Unknown fails the gate. Invariant tests must refute writers
leaving Some after view mutation and consumers discarding provenance to obtain
apparently valid results.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/args.md`. Historical observations are not a new parity certification.
