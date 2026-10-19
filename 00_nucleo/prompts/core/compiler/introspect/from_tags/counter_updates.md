# L0 — compiler/introspect/from_tags/counter_updates
Hash do Código: f22a6878

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/page-counter-snapshot.toml sha256:49304f0982abfe05ab825b90082a3448976340a8eacfeee2daa7c7a343013bde
- 00_nucleo/prompts/_nuclei/realization-order.toml sha256:958649ea9ebdd471f101874ff8cf94feb995ee037c43eb6e2f1d1ad19a151ef3


## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/introspect/from_tags/counter_updates.rs`. The original logic was in `01_core/src/compiler/introspect/from_tags.rs:34-103,157-287` at HEAD `4e2a321ce`, clean tree, 2026-09-30 23:49:20 UTC. This is internal extraction; inspection is neither new parity evidence nor a performance measurement.

## Contract

Own filtered counter replay, ordinary counter functions and bundle Page replay. Preserve causal order, key equality, source position, callback inputs/results, diagnostic priority and event history. Preserve public contracts, pipeline phase, defaults, compatibility, static dispatch and L1 purity.

## Acceptance

L1 tests, `cargo build` and `crystalline-lint .` must pass. Do not infer performance gains.




## Causal filtered-counter order

At HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, CounterRegistry::value_at,
StateRegistry::value_at, before/after queries and counter events compare
Location.as_u128. Realization prepares identity before transforming, so the number
no longer orders originals/products. CLI probes and
single_evaluation_counter_follows_causal_order retain evidence in the
implementation diagnostic. Apply the pinned realization-order nucleus; keep
historical fallback only without recorded order. Filtered-counter replay uses recorded causal ordinals, preserving callback order and demanded prefixes.

## Ratified realized Page snapshot

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`; exact working tree/binaries are in
`lab/results/construction/diagnostics/contexto-page-total-2026-10-05/measurement.json.gz`.
single prints B1/0 F0 versus B1/1 F1; two-page total/final 0 versus 2; initial
update 7 keeps total 7 versus 8; an intermediate update changes the previous
page (7/7 and 8/7 versus 1/7 and 7/7); final step prints final 1 versus 2.
Bundle prints 1/7 in its first PDF versus 1/1, final 7 in the second versus 8;
mixed HTML returns 1 versus 7. Historical paged.rs:360–365,840–913 uses the last
update as every page's start; context_stabilization.rs:969–1010 does the same by
body. compiler/stdlib/counter.rs:490–513 and counter/display.rs:96–110 read the
global causal final. Vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`,
introspection/counter.rs:246–254,730–773,934–963 combines document scope, updates
and physical advancement. This is language parity, not a requirement to copy
upstream memoization.

### Ratification and current contract

The human confirmed
`lab/results/construction/diagnostics/contexto-page-total-2026-10-05/l0-manifest.json`,
SHA-256 `1af5de11b1bbbe41cc59b907fdfafc21d98b6aab63fc1db0bfb1a028329ef66e`.
Its ADR-0127 gate is satisfied. Thus the page-counter-snapshot nucleus supersedes
the historical final scope-out for this unit. The public contract and page-aware
update realization were approval-gated; the archived preapproval instruction is
not a new pending gate. Subsequent internal clarification follows continuous
flow, L0 before code and RED→GREEN.


### Exclusive public facade

Preserve the approved pure facade through the existing module; path-only
forwarding is mechanical:

```rust
pub fn realize_page_counter_snapshot(
    tags: &[Tag],
    documents: &[(Vec<Location>, Option<std::num::NonZeroUsize>)],
    intr: &mut TagIntrospector,
    engine: &mut Engine<'_>,
    observer: &EvalContext,
) -> SourceResult<()>;
```

Each tuple contains one body's occurrences and realized physical total, None
for HTML. Never infer document identity from numeric Location order. Validate
disjoint partition/identity using current intr positions. Fold Page from language
initial state through Set/Step/Func and physical advances, honoring preceding
occurrences and final state after last updates/pages. Page callbacks receive
page-aware state and sinks follow contribution retention; do not rerun callbacks
of other keys. Publish no incomplete snapshot before adequate position/partition;
publish both views together only in the same valid generation, preserving original
diagnostics/traces. Characterize callbacks, empty/multilevel state, late updates
and HTML with upstream oracles before closing behavior. Missing installed source
is not a positive test. Preserve signatures/cycles/budget and guess no page/total.

### Causal action and partition refinement

The canonical intr.counters.actions log already extracted from tags preserves
original Func actions; supplied tags do not duplicate events. Bundle descriptors
use walk-recorded occurrence membership, never boundary intervals/raw Location.
A single explicit-occurrence descriptor covers standalone. Rebuild logical_numbers
in existing PageStore from the realized fold, retaining metadata slots until the
numbering owner realizes them; final includes updates after the last read.

With multiple documents use bundle walk membership in CounterRegistry, never
replace it by descriptor checkpoint intervals. Without registered scopes, supplied
lists are explicit disjoint scopes. Empty scopes are valid; a Page action outside
them prevents publication.

### Page read normalization

contexto-page-snapshot-2026-10-05/measurement.json.gz measures update(()) followed
by get as (0,) upstream versus () locally; update((7,2)) gives (7,) versus (7,2).
Historical upstream counter.rs:621–629,798–807,869–880 projects the first level
and supplies zero for empty state even at zero physical delta. Actions advance
only for positive delta (counter.rs:939–948), preserving original callback
arguments between same-page actions. Apply projection only to current/final
read views, never mutate causal state by observing it.

### Acceptance and limits

Use the pinned page-counter-snapshot and realization-order claims; scoped
RED→GREEN, build/suites/lint and restricted lineage are required. Other counter
keys, callback order/source diagnostics and all guards remain unchanged. Do not
publish a partial or cross-generation snapshot, invent missing source evidence,
change existing cycles/budget or claim general upstream parity. The receipt
above ratifies only this package; later internal refinements follow ADR-0127
continuous flow.
