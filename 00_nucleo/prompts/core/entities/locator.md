# Prompt L0 — `entities/locator`
Hash do Código: e468ee65

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/locator.rs`

## Evidence before specification

The original introspection design requires equivalent walks to assign identical Location sequences for fixpoint convergence. Its comparison with upstream `introspection/locator.rs:153` and `lab/results/construction/diagnostics/inventario-tipos-introspection-vanilla.md` (2026-04-30) classified outer links, measurement modes, splitting and memoization caches as implementation mechanisms, not mandatory local structure.

At HEAD `bde47c744e0bffb6229005e4b5f04c2ed0e9ef08` with uncommitted bundle work, local `locator.rs:24–46` exposed only new/next, while `layout/mod.rs:1108` restarted each layouter at zero. The bilateral bundle probe produced `1. Alpha`/`2. Beta` in vanilla and `1. Alpha`/`1. Beta` locally. Full provenance is in `lab/history/construction/records/00_nucleo/materialization/typst-passo-1485.md`. Numbering is language semantics; counter/seed representation is mechanics. The checkpoint hypothesis is refuted by continued replay divergence or nested collisions, and upstream intent is not inferred from PDF bytes.

`lab/results/construction/diagnostics/clippy-traits-formatacao-2026-10-16/before.json.gz` preserves sources/L0/global hashes/HEAD/uncommitted tree/UTC/diffstat for the 13-warning inventory containing Locator.next. Its return contract differs from Iterator; this is interface evidence, not a new parity or readability measurement.

## Public and internal contract

```rust
pub struct Locator { counter: u64 }
impl Locator {
    pub fn new() -> Self;
    pub fn next(&mut self) -> Location;
    pub fn checkpoint(&self) -> LocatorSeed;
    pub fn from_seed(seed: LocatorSeed) -> Locator;
}
impl Default for Locator { /* delegates to new */ }
```

Own opaque `LocatorSeed`. Keep pure per-walk mutable state exposed only through mutable borrowing, no I/O or shared thread state. new starts at zero; next returns Location::from_raw of the current zero-extended counter, then advances with the existing checked-overflow behavior. Allocations are distinct and monotonic within a walk; independent new instances given equal call sequences agree. The counter stays opaque and Locator is deliberately not Clone.

Checkpoint captures the next allocation without consuming one. A seed supports replay/fixpoint of the same body; it is neither document identity nor a replacement for walk order. Do not expose numeric counters or infer offsets from tags, headings or pages. Crate-internal observe(Location) advances at least beyond the observed location so explicitly realized occurrences remain disjoint from future ordinary allocations; it introduces no alternative Location construction path. The public checkpoint gate was confirmed by the owner on 2026-09-24 in the bundle record above.

## Lint and acceptance

next remains `next(&mut self) -> Location`; identity allocation has no normal Option::None termination. Do not implement Iterator or convert overflow into silent completion. Apply only `#[expect(clippy::should_implement_trait, reason="S002: alocador causal de Location com overflow explícito; não é Iterator de itens opcionais")]` on that method. No impl/module/crate suppression, threshold increase or extension to Styles::from_iter is allowed.

Tests require two distinct successive locations, equal five-call sequences from independent new instances, and increasing as_u128 values. A two-body walk resumed from a checkpoint must match one continuous walk; replay starts at the second body's seed without collisions with the first. Preserve existing functional expectations and bodies. For lint-only maintenance require L0 first, restricted resealing, lint RED→GREEN with unfulfilled_lint_expectations as error, workspace/build/Clippy/docs/fmt/lineage under 12 GiB, zero swap, one job. No new API/default/phase is authorized by that metadata change.

## Deferred scope

The original passive-consumer status is historical. Planned introspection integration uses one locator per walk and an Introspector index over emitted locations. Path-hash refinement, measurement mode, splitting and cross-memoization remain separately scoped; the historical alternative `(parent_path, sibling_index)` design is not a mandate to replace the current checked counter or checkpoint contract. The single-pass design does not certify those future facilities.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/locator.md` preserves the original evidence and chronology.
