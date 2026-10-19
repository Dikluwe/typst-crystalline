# Prompt L0 — Engine<'a>
Hash do Código: 80d7324c

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/engine.rs`
**Governance:** ADR-0036, ADR-0037, ADR-0042, ADR-0043, ADR-0044.

## Purpose and contract

Engine aggregates evaluator state after responsibilities have been extracted.
The archive preserves the original extraction/aggregation sequence; it is
not an instruction to repeat those construction steps.

```rust
pub struct Engine<'a> {
    pub world: &'a dyn World,
    pub font_metrics: &'a dyn FontMetrics,
    pub route: Tracked<'a, Route<'a>>,
    pub styles: &'a mut StyleChain,
    pub show_rules: &'a mut Arc<[ShowRule]>,
    pub active_guards: &'a mut Vec<RuleId>,
    pub current_file: FileId,
    pub sink: &'a mut TrackedMut<'a, Sink>,
}
```

One lifetime 'a parameterizes the aggregate. route and sink keep individual
tracking; Engine itself must not be comemo::track. Public fields are deliberate,
not getters. Group fields by domain: injected world/font_metrics handles;
route/styles/show_rules/active_guards/current_file flow; sink effects. Field
order follows domain cohesion rather than upstream's literal order; retain
upstream-compatible field names. L1 declares capabilities; L3/L4 provide
concrete implementations.

## Construction and acceptance

The public evaluator and local scope rebuilds (CodeBlock, ContentBlock,
strong/emph/heading, closures, module include) individually reborrow fields.
Use `TrackedMut::reborrow_mut(&mut *engine.sink)` to shorten the sink lifetime;
propagate outer font_metrics unchanged. Root/test entry points without an
outer context use &FixedMetrics as the specified heuristic fallback. Verify
these lifetime/tracking and propagation properties without introducing I/O.

## Deferred ownership reconciliation

The original schema lists introspector, routines, and traced only as future
stubs. Its declared expansion rule requires an applicable ADR when a subsystem
is introduced and upstream-aligned field names. Their implementation status
must be reconciled with the relevant current owners; this cleanup does not
turn stubs into fields, remove newer fields, or authorize a new public API.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/engine.md`. Historical observations are not a new parity certification.
