# Prompt L0 — `entities/tag`
Hash do Código: 5f1f9f1e

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/tag.rs`

## Design evidence

Historical upstream `e0e8ca4d:introspection/tag.rs:10–22,47–56` stores the key hash in End to balance enum variant sizes, explicitly without semantic reason. The earlier query-optimization explanation was invented and is withdrawn. Local placement is permitted storage mechanics, not a language parity requirement. Upstream Start requires a Location and distinguishes introspectable/tagged flags; the original local model did not encode that distinction. `lab/results/construction/diagnostics/inventario-tipos-introspection-vanilla.md` (2026-04-30) classifies the flag concept as potentially needed and its vtable TagElem wrapper as outside the local design.

## Compact carrier evidence and approval

`lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/before.json.gz` records HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree/UTC/source/L0/hashes/status/diffstat. probe-details.json records rustc 1.92/argv/rlib/binary/probe hashes/output; probe-result.json records 12 GiB/swap 0/one-job/events. These synthetic real-domain-carrier/registration/copy probes are not compile/layout/RSS benchmarks or a measured workload distribution.

Tag is 416 bytes, Box/Arc Start candidates 48. For 4096 Start/End pairs with heap-free Heading payload, inline requests 3407872 bytes/1 allocation, Box2031616/4097 and Arc2097152/4097. Box clone allocates per Start, but inspected productive fixpoint.rs:86–136 and entrypoints.rs:44–66 use temporary Vec<Tag>/borrowed reads without vector clone; walk:325–356 indexes by reference before moving info. Synthetic clone frequency is not asserted productive.

Transformation/ShowRule are 328/376 bytes; Box/Arc Styles candidates 120/168. rules/show.rs:464–470 copies to_vec per registration; apply.rs:108–112,349–352 filters/clones. Across 128 simple show-set registrations without old snapshots, inline requests 12276960 bytes/384 allocations, Box8194560/8640 and Arc5530624/512. Retained-snapshot peak requests are 3203856,4141744,1477808. With custom styles, copying 4096 rules requests 4107 allocations inline,8203 Box and 11 Arc including Vec growth. apply/intercept/outline/table_cells borrow styles; application still clones deltas when needed.

Keep the counterexamples: building one collection of 4096 simple show-set rules requests 1540096 bytes inline,2031616 Box,2097152 Arc; rich 128-registration final live bytes without old snapshots are 78064 inline versus 156432 Arc because Arc can retain spare custom capacity. Do not claim universal gain or hide atomic/indirection cost. A relevant real-workload memory/time regression refutes the favorable Arc copy-cycle inference. This is storage mechanics under ADR-0030/0107, not language parity or measured AI cognition.

The owner approved manifest `282da3f8f79080a4831e1898879775e32a62caa0b2ffa941b8fca9198136b950` on 2026-10-16, receipt `lab/results/construction/diagnostics/clippy-enums-publicos-2026-10-16/approval-receipt.json`. It satisfies the original ADR-0127 public-payload/compatibility gate for this package and T1–T6, without release. Additional signatures/fields/phase changes require another proposal.

## Current tag contract

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tag {
    Start(Location, Box<ElementInfo>),
    End(Location, u128),
}
```

Remain a pure L1 closed enum. Start marks entry into an indexable element, carrying its Locator-generated unique location and payload/optional label. End marks exit with the same location and content hash from the owned hash_content path. A complete walk pairs every Start with exactly one End and nests subwalks correctly. Location equality is exact; value Eq/Hash permit deterministic regression comparisons.

The ratified Box replaces only the original inline ElementInfo payload. Preserve exclusive payload ownership and deep clone; no Arc inside Tag. ElementInfo/ElementPayload fields, End, derives and variant set remain unchanged. Constructors explicitly Box::new info, borrowed reads auto-deref and consuming reads may use *info. Update test construction without changing data/expectations. No public helper/dynamic dispatch/I/O. Keep chronological walk order, pairing, locations, indexed content and convergence decisions.

## Integration and unresolved scope

The original no-consumer/passive-type statement and parallel legacy-counter migration schedule are construction history. The walker emits tags and Introspector consumes their stream; deterministic equivalent-walk snapshots remain acceptance. Do not use the old payload-is-small claim as current cost evidence.

Local Start separates explicit Location and focused ElementInfo, unlike upstream Content+TagFlags. The original tagged-versus-introspectable distinction remains a documented modeling scope; its absence here neither proves global tagged-PDF support nor forbids separately owned later facilities. Copying upstream TagElem/vtable is not required. Retain constructor/equivalent-tag equality tests and all current integrations.

## Compact carrier acceptance

L0 and tests precede implementation; require real RED→GREEN and restrict resealing to changed owners. Do not silence large_enum_variant, raise thresholds or add dependencies/flags/defaults/phases.

T1: Box<ElementInfo>/Arc<Styles> construction and relative carrier budgets (Tag at most three Locations; Transformation at most Content plus alignment) fail before and pass after, without asserting universal ABI. T2: preserve Start/End/Location/payload/label, value equality/structural hash, independent Box clone and mutation isolation; hashes do not become persisted formats. T3: existing introspection/convergence/nesting/heading/label/state/counter/query/metadata tests retain expectations/errors. T4: Transformation::Style clones share Arc; Arc::make_mut on a copy preserves original styles including custom, and Func/Content/Str variants remain. T5: show-set/func, precedence/scope/labels/outline/table.cell/textual/regex preserve semantics/morphology/errors and execute no extra rules. T6: full workspace/build/all-target Clippy and previously closed expectations/lints as errors, Rustdoc -D warnings and four HTML indices, fmt/diff/strict+reverse lineage/no increased Tekt diagnostics under 12 GiB/swap 0/one-job. Record HEAD/tree/hashes/diffstat/UTC; distinguish type sizes/requested bytes from RSS. No release.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/tag.md` preserves the original evidence and chronology.
