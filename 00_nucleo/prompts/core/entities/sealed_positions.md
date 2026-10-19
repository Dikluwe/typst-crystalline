# L0 — core/entities/sealed_positions
Hash do Código: a8af3489

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/sealed_positions.rs`
**References:** ADR-0074 sealing, ADR-0073 tracking; ADR-0066 deferral superseded.

## Evidence and architecture

Historical upstream introspect.rs:38,60-63 builds PagedIntrospector from final
pages and exposes position(loc). Crystalline intentionally seals one substore,
positions, after Layouter::finish on every fixpoint iteration. It populates
positions in a single pass, then freezes them; use a direct #[comemo::track]
impl on the concrete struct, not a dedicated single-implementation trait.
This supplies structure for the separately owned runtime position query.
It is an architectural comparison, not literal parity certification.

## Contract

SealedPositions derives Debug/Clone/Default and has private
positions:std::collections::HashMap<Location,Position>. No PartialEq/Eq is
required; adding it needs an actual consumer/contract. Pure L1, no I/O/global
state. Read-only after construction; preserve std HashMap consistency with
the runtime map rather than introducing FxHashMap.

The regular impl exposes empty()->Self (Default),
from_runtime(HashMap<Location,Position>)->Self, len(&self)->usize and
is_empty(&self)->bool. from_runtime consumes the moved runtime map; no internal
Arc. Clone is O(n), used for retracking; its historical rarity is not a new
benchmark. Self:Clone is required by comemo.
A separate tracked impl exposes
position_of(&self,location:Location)->Option<Position>, returning None when
absent and permitting repeated-query cache reuse.

PagedDocument has public extracted_positions:SealedPositions; existing
PagedDocument::new test callsites get Default. Layouter::finish constructs
the store from runtime.positions after layout, before returning the document,
while preserving extracted_label_pages and the existing population algorithm.

## Acceptance and scope

Required sentinels: p205b_sealed_positions_struct_existe constructs empty and
from_runtime(empty HashMap); p205b_sealed_positions_e_track checks generated
Track through assert_track::<SealedPositions>(). Recommended integration test
p205b_layouter_finish_produz_sealed_positions runs layout/finish and confirms
positions for locatable content. Keep exact test identifiers as history-bearing
identifiers, not document organization.

This owner does not implement TagIntrospector's real position_of, migrate dual
path consumers, change runtime population, create SealedLabelPages or approve
ADR-0074 globally. Those originally assigned follow-ups remain separately owned;
structural sealing alone does not certify their closure. BibStore is the
related immutable substore pattern.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/sealed_positions.md`. Historical observations are not a new parity certification.
