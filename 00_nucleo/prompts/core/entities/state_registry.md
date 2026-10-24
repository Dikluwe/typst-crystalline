# Prompt L0 — `entities/state_registry`
Hash do Código: 32ea31c3

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/physical-page-occurrences.toml sha256:b33f2e3c6238da01aa48783aecee6a8b6153b4f36ddcdc23edfe78434bae5b42
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:958649ea9ebdd471f101874ff8cf94feb995ee037c43eb6e2f1d1ad19a151ef3

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/state_registry.rs`

## Design and language evidence

The original store maps String keys to Vec<(Location,Value)> history, with O(1) key lookup/append and O(n) temporal lookup. It separates pure state data from Introspector behavior, without I/O, mutable globals or compiler imports. BTreeMap/flat key-location-vector alternatives were not selected. Debug/Clone/Default and public signatures remain; the later private Arc representation below supersedes inline storage while retaining borrowed reads.

Historical upstream `e0e8ca4d:introspection/state.rs:192–194,300–315` carries init in each state object and makes updates effective only when their returned content is placed. First-init-wins and ignore-update-without-init are local defensive legacy rules, not measured vanilla semantics: that language does not have an independent init registry. Public acceptance observes values at document positions. Upstream state.rs:184–188,339–349 also documents convergence and recommends function updates; comemo is memoization, not the state resolver. The old single-pass limitation was observable scope, not invisible mechanics; it must not override the later approved reader runtime.

## Preserved legacy interface

```rust
use crate::entities::location::Location;
use crate::entities::value::Value;

#[derive(Debug, Clone, Default)]
pub struct StateRegistry { /* HashMap<String, Vec<(Location, Value)>> */ }

impl StateRegistry {
    pub fn empty() -> Self;
    pub fn value_at(&self, key: &str, location: Location) -> Option<&Value>;
    pub fn final_value(&self, key: &str) -> Option<&Value>;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;

    pub(crate) fn init(&mut self, key: String, init: Value, location: Location);
    pub(crate) fn update(&mut self, key: String, value: Value, location: Location);
}

```

empty returns no values. Explicit init retains the first init for a key; explicit update ignores keys without init. value_at returns the last eligible history entry at/before the causal target, or None; final_value returns the last materialized value (init if no updates). Distinct keys are isolated. The removed apply_update convenience method is not restored: matching/executing Func belongs to compiler runtime with Engine/EvalContext, never this entity.

The original passive-consumer and planned-only statuses are history. TagIntrospector exposes the store through independently owned state access. Tests retain empty queries, init-only eligibility, before/after updates, multiple updates, isolated keys, ignored uninitialized updates and ignored second init for this legacy interface. Modern object-local init below is distinct.

## Causal order and equivalent temporal search

At HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, StateRegistry/CounterRegistry/query/events compared numeric Location, but realization allocates identities before transforms. CLI evidence and `single_evaluation_counter_follows_causal_order` retain the counterexample. Follow pinned realization-order: private per-registry first-observation ordinal, observe_location/order_key and numeric fallback only for unobserved legacy/test locations. Temporal lookup preserves the causal prefix for nonmonotonic origin/product IDs. No global state or inference of chronology from identity.

`lab/results/construction/diagnostics/predicados-fixtures-ci-2026-10-13/before.json.gz` freezes source/L0/nucleus/UTC/status/diffstat at that uncommitted HEAD. state_registry.rs:80 uses pure order_key in iter/filter/last. rfind may select the same last eligible Vec entry and None without reordering history, changing init/update or substituting IDs. Worst-case O(n) remains, with no measured speed claim. Preserve temporal tests; no public/default/phase change follows from this refinement.

## Synthesized-entry order evidence


The original order-before.json.gz/before.json.gz retain the same uncommitted HEAD/UTC/hashes/diffstat; trace-runtime-result.json/trace-runtime.log show direct entry in pure/runtime/L3 NativeElement indices, while bare outline.entry becomes Kind(OutlineEntry) at foundations/selector.rs:113 and the terminal walk omits it. measurement-order-before.json at historical pin `9f2b6e8715237cb086899a42873660fe744622e8` measures BARE [Heading] versus[Heading,Direct], FILTERED [Direct,Heading] versus[Heading,Direct], STATE2 versus 1, COUNTER((2,),) versus((1,),) at synthesized Location. Post-walk allocation happens after updates but belongs causally inside the earlier outline. Wrong order/prefix/duplication/entry data refutes the correction; language acceptance does not require equal Rust/PDF bytes.


Provide only crate-private observe_locations_after(&mut self,anchor:Location,locations:&[Location]): unknown anchor/empty/already-observed list is no-op. Pure local HashSet deduplicates new locations in order; shift later ordinals and insert directly after anchor. Preserve dense old relative order, idempotence, clones and later append. Compiler chooses anchor; never infer it by Location number. Do not change init/update/history/lookup/values/phases/I/O/external interface or import compiler.

For entry ordering, tests precede code: bare/direct entry and aliases, three-entry order across two outlines with direct entry between, where/no phantom heading, state/counter prefix at synthesized locations. Internal-helper contracts cover unknown anchors, duplicates/repetition, nonmonotonic IDs and dense append. Repeat nine CLI probes (six originals plus order/state/counter) without relaxed outputs. Full suite/build/fmt/strict Clippy+Rustdoc/V1/V5/V15/V26/reverse/diff/SARIF must remove six V16, add seven prescribed V19 and update three existing messages without new V20. Use 12 GiB/swap 0/one-job/sequential heavy commands. This bounded internal/parity repair adds no external signature/default/phase; reseal only nine affected pairs. Original package excluded commit/push/release and left other V16 debts open.


## Object-local state reading evidence and approval

`lab/results/construction/diagnostics/lote-a1-callbacks-reflexao-seletores-2026-10-17/{before,final-freeze,state-counterproof}.json.gz` records HEAD `b1b486f3d42431f8ea360bc5712637ee1e3d4f6c`, uncommitted tree/UTC/status/diffstat/sources, current pin `bd5ebec285505889e8ff1d4f68804ab147a699f3`. Bilateral RED requires same-key initial1/10 after+1→2/11, no execution of unread callbacks, and reused updates executing per occurrence. Upstream state.rs:226–245,419–455,477–520 starts from reader init and document order. The old eager postpass/Func-as-Set lost those values/errors; no Rust identity or algorithm copying is required.

The prior A1-S1 proposal was withdrawn. Owner-approved A1-S2 review is `lab/history/construction/records/00_nucleo/materialization/state-reader-runtime-2026-10-17/review.md`, SHA-256 `6d6a176b8e27cc53c38fe6650c03c217448059f4e7287fd8848fe54e80247fa4`. It ratifies its five signatures and read phase, without expanding other public contracts.

## Current action storage contract

Preserve StateRegistry public definitions/signatures. Privately record StateUpdate actions by key/Location independently of global init. Capture Set/Func and deduplicate only the same occurrence at the same Location. Ordered reads use causal order_key with inclusive prefix semantics; callbacks never execute here. Public materialized history stays legacy; the modern reader does not treat its first init as a global initialization.

Privately store materialized values as Arc<Value>. Crate-private RecordedStateUpdate is Set(Arc<Value>)|Func(Func), without changing public StateUpdate. Share a Set Arc between legacy history and action, avoiding an extra deep copy. Action lists are Arc<Vec<...>>; mutation uses Arc::make_mut so snapshot clones share until extension without changing earlier snapshots. Crate helpers register actions, extract causal ordinal/shared action sequences and obtain eligible legacy init. Recording Set preserves legacy auto-init/latest value; Func materializes no value. Explicit legacy init/update keep their rules and invent no action.

The compiler resolver may evaluate the entire key sequence to propagate later errors while selecting the requested prefix by ordinal. Never discard future actions before checking errors: state-future-error in target-controls.json.gz refutes that optimization. Reading another key executes none of these actions.

## Reader acceptance

L0 and functional RED precede implementation. Require GREEN against the stated pin plus separate Tekt-extension controls. Test nonnumeric order, independent snapshots and physical Set-payload sharing as RAM-contract checks, not language parity. Run build/workspace/doctests/strict Clippy+Rustdoc/fmt/V1/V5/V15/V26/reverse with no increased V16, MemoryMax 12 GiB/High 8 GiB/swap 0/jobs 1 and sequential heavy work. Measure consumption without inferring global benefit from representation alone.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/state_registry.md` preserves the original evidence and chronology.


## Approved physical-page source construction

### Evidence before decision

Clean baseline HEAD `63b7b96b99109b61655b373dc0d248beb7838f86`; exact source/owner bytes, hashes, UTC and working-tree state are archived in `lab/results/set-page-header-implementation-2026-10-21/agents/index/before-l0.json.gz`. Current-pin bilateral source/query evidence and independent source-driver anchors are in `lab/results/set-page-header-2026-10-21/baseline-parity/receipt.json` (SHA-256 `38520d31351eb01fe7a191243a6a36318aba0c60473e0a2cc54103b36d8866bf`) and `lab/results/set-page-header-2026-10-21/agents/parity/index-driver.md`. Repeated physical raw marginals contribute distinct located semantic occurrences; counter/query order is background/header/body/footer/foreground. Dormant overwritten templates remain inert. These are language semantics/refusals, not frame scanning, Rust equality or execution-count parity. The human ratified the exact additive API/phase package in the frozen `lab/results/set-page-header-2026-10-21/agents/core/proposal.md` (SHA-256 `f509e593d67ec2c2ca19f9c32f2d561a9149d4db903e5b12f11d76dbf9cbdaa2`).

### Exclusive responsibility

Provide only private construction support for the same complete admitted causal location order used by counters and source queries. Preserve reader-initialized demanded action interpretation, Set/Func identity, legacy snapshots and every public signature. Record actions once per actual occurrence; intentional Located aliases retain first observation. No eager update callbacks, separate state solver, global mutation or numeric/page-derived ordering.

### Acceptance and preserved boundaries

Require owning L0 before productive code and actual functional RED→GREEN, complete source-packet/parent/authority proofs, fresh raw versus Located-alias controls, retained retries and no phantom dormant/measurement effects. Reuse the shared arena and existing strict read/Sink transactions. No additional public surface, default, field, phase or compatibility change beyond the frozen approved package; no global Frame introspection, copied histories, invented Location, I/O or observer weakening. Preserve old tests and immutable F/G receipts; maintain exhaustive closed decisions and validate full suites/build/strict lint/Clippy/Rustdoc/fmt/paths/forward and reverse lineage/Nucleus DAG. Unrelated bounded debts remain explicit.
