# Prompt L0 — `entities/counter_registry`
Hash do Código: b7e02014

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:958649ea9ebdd471f101874ff8cf94feb995ee037c43eb6e2f1d1ad19a151ef3

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/counter_registry.rs`

## Base design and read contract

Own a pure, externally read-only CounterRegistry substore with Debug/Clone/Default. Current-state storage maps String keys to Vec<usize>; histories map keys to ordered Location/state snapshots. Mutation is crate-internal during construction. Upstream's comemo/ElementIntrospector organization and rich CounterKey are not storage requirements; the original flat-only/passive-consumer description is historical and superseded by hierarchy/actions below.

```rust
use crate::entities::counter_update::CounterUpdate;
use crate::entities::location::Location;

#[derive(Debug, Clone, Default)]
pub struct CounterRegistry {
}

impl CounterRegistry {
    pub fn empty() -> Self;
    pub fn value(&self, key: &str) -> Option<&[usize]>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;

    pub fn format(&self, key: &str) -> Option<String>;

    pub fn value_at(&self, key: &str, location: Location) -> Option<&[usize]>;

    pub fn value_at_index(&self, key: &str, idx: usize) -> Option<&[usize]>;

    pub(crate) fn apply(&mut self, key: String, update: CounterUpdate);

    pub(crate) fn apply_hierarchical(&mut self, key: String, level: usize);

    pub(crate) fn apply_at(&mut self, key: String, update: CounterUpdate, location: Location);

    pub(crate) fn apply_hierarchical_at(&mut self, key: String, level: usize, location: Location);
}
```

empty has no values; value returns a borrowed current slice or None for untouched key. len/is_empty reflect the registry. format joins hierarchical components with dots. apply_hierarchical clamps level to at least 1: []+level 1→[1], [1]+level 2→[1,1], [1,1]+level 1→[2]. The ratified action shape below supersedes old unit-Step/Update-only handling.

value_at returns the last eligible snapshot after all updates at or before the requested causal location, or None if none/key missing. _at methods apply then record a snapshot; non-_at methods do not populate history. value_at_index returns the zero-based history snapshot by appearance position or None out-of-range; hierarchical snapshots retain all levels. In the simple automatic flat-step sequence snapshots are [idx+1], not a general invariant after explicit Set/Func. HashMap iteration has no deterministic order. Different keys remain isolated. The old map-content equality statement does not add an undeclared public Eq trait.

## Complete action evidence and ratification

Historical upstream `a51e02804` counter source accepts hierarchical state, positive step levels and callbacks. Owner confirmation is recorded in `lab/history/construction/records/00_nucleo/materialization/typst-passo-1148.md`, section 12, satisfying the former pending action gate in `00_nucleo/prompts/core/entities/counter_update.md`. Set replaces the complete Vec; Step(level) uses hierarchical stepping; Func applies to the current array with result conversion and propagated eval errors rather than silent ignore. Callback evaluation is compiler-owned with Engine/EvalContext, not code imported into this pure registry. It executes during introspector construction through the existing StateUpdate Engine/EvalContext channel, never during field access; this action package introduces no new phase. Additional public signatures beyond that approved action package need another gate.

## Ordered action log

At HEAD `2f42d64253547734564513a1159ee6b584c1c4b4`, counter_registry.rs:25–36 has states/snapshots and 69–84 distinguishes Step/Set/Func. Inferring actions from snapshot differences loses Set-versus-Step. Historical filtered-key Step/Func/Step produces 12 upstream, not just query size; state/UTC/binaries remain in the archived records. The initial no-implemented-candidate note is historical status, not evidence of current completion.

Keep private ordered events containing Location, automatic/manual origin and true CounterUpdate. Manual events preserve complete CounterKey; automatic events refer to canonical occurrences and effective Step(level)/owner gate. Record one automatic action per occurrence, without duplicating auxiliary figure:kind writes or inferring action from before/after numbers. Crate-only log access is data, not another field store, compiler import or Func execution. Clone/default/empty isolate document/iteration. Existing borrowed state/history read contracts remain; compiler returns owned vectors for arbitrary-key calculations rather than references to temporaries. Approved on-demand resolution still requires the original observation/identity package's other gates.

## Causal ordering and document membership

Historical value_at at counter_registry.rs:151–159 chose the last inserted eligible Location while apply_at:182–194 appended. A later Page Func postpass could insert an earlier-location event after the next boundary: this was a source-inferred risk, not measured Func output. The original bundle diagnostic preserves Set probe provenance. Owner confirmation on 2026-09-24 covers the corresponding introspection contract. Private helpers support compiler-required causal Page snapshot materialization without changing public signatures/other keys or making reset a located occurrence. Compiler evaluates Func; this entity stores states/locations/actions only.

At HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, realization prepared identities before transformation, invalidating numeric-ID chronology in registry/query/events. The CLI probe and `single_evaluation_counter_follows_causal_order` sentinel retain implementation evidence. Apply the pinned realization-order nucleus: private per-registry order maps Location to first-observed ordinal; observe_location records first occurrence, order_key uses it and falls back to numeric ID only for unobserved legacy/test locations. Temporal lookup preserves causal prefixes even when origin/product IDs are nonmonotonic. No global state.

Record actual Tag::Start locations per body in document order after walking each body, including empty ranges. Page realization uses explicit membership, never numeric-ID boundaries or repeatable checkpoint values. Access remains crate-internal.

## Temporal search refinement

`lab/results/construction/diagnostics/predicados-fixtures-ci-2026-10-13/before.json.gz` freezes source/L0/nucleus/UTC/status/diffstat at that HEAD with uncommitted tree. counter_registry.rs:176 uses iter/filter/last with a pure order_key predicate. Search may rfind from the end, preserving exactly the last eligible Vec entry and None. Do not reorder history, replace causal order by IDs, or change updates/init. Worst-case O(n) remains; no timing gain is inferred. Keep temporal controls including nonmonotonic origin IDs. This is internal ADR-0127 maintenance without public/default/phase change.

## Synthesized-entry insertion evidence and contract

The original order-before.json.gz/before.json.gz retain the same uncommitted HEAD/UTC/hashes/diffstat; trace-runtime-result.json/trace-runtime.log show direct entry in pure/runtime/L3 NativeElement indices, while bare outline.entry becomes Kind(OutlineEntry) at foundations/selector.rs:113 and the terminal walk omits it. measurement-order-before.json at historical pin `9f2b6e8715237cb086899a42873660fe744622e8` measures BARE [Heading] versus[Heading,Direct], FILTERED [Direct,Heading] versus[Heading,Direct], STATE2 versus 1, COUNTER((2,),) versus((1,),) at synthesized Location. Post-walk allocation happens after updates but belongs causally inside the earlier outline. Wrong order/prefix/duplication/entry data refutes the correction; language acceptance does not require equal Rust/PDF bytes.

Provide only crate-private `observe_locations_after(&mut self,anchor:Location,locations:&[Location])`. Unknown anchor, empty list or entirely observed list is no-op. Deduplicate new locations in order using a pure local HashSet, not quadratic search. Shift existing ordinals strictly after anchor by the inserted block size; place the block immediately after it. Keep dense order, idempotent aliases/repetition, old relative order and collision-free later append. Never mutate/reorder values/actions/histories/document_locations, execute callbacks or import compiler. The outline_entries owner chooses anchor/block; this entity owns order data only.

## Acceptance

Retain empty heading→None, first unit step→[1], three steps→[3], Set([5])→[5], isolated keys, hierarchy/format and temporal/index controls. Walk/from-tags population and TagIntrospector reads remain independently owned.

For entry ordering, tests precede code: bare/direct entry and aliases, three-entry order across two outlines with direct entry between, where/no phantom heading, state/counter prefix at synthesized locations. Internal-helper contracts cover unknown anchors, duplicates/repetition, nonmonotonic IDs and dense append. Repeat nine CLI probes (six originals plus order/state/counter) without relaxed outputs. Full suite/build/fmt/strict Clippy+Rustdoc/V1/V5/V15/V26/reverse/diff/SARIF must remove six V16, add seven prescribed V19 and update three existing messages without new V20. Use 12 GiB/swap 0/one-job/sequential heavy commands. This bounded internal/parity repair adds no external signature/default/phase; reseal only nine affected pairs. Original package excluded commit/push/release and left other V16 debts open.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/counter_registry.md` preserves the original evidence and chronology.
