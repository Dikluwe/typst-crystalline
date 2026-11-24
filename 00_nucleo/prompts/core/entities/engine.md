# Prompt L0 — Engine<'a>
Hash do Código: 80d7324c

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/engine.rs`
**Governance:** ADR-0029, ADR-0072, ADR-0109, ADR-0127, and retained ADR-0042 through ADR-0044.

## Purpose and contract

This owner specifies the evaluator aggregate in
`01_core/src/entities/engine.rs`. The similarly named unit carrier in
`01_core/src/entities/world_types.rs` belongs to
[00_nucleo/prompts/core/entities/world_types.md](world_types.md).

The [original Engine decision](../../../adr/adr-0044-engine-eval-state-aggregator.md)
covers aggregation and individually tracked handles. The
[font-metrics decision](https://github.com/Dikluwe/typst-crystalline/blob/634bf6943f6352a8bf63c72561d7ecc8d891932d/00_nucleo/diagnosticos/typst-passo-858-relatorio.md#L11)
records the owner's choice of the required `&dyn FontMetrics` field.
The original aggregation receipt and the later font-metrics decision cover
different revisions of the public shape.

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
stubs. Their current ownership and public provenance must be reconciled separately. Apply ADR-0127 when changing an interface and record a new decision when the change establishes an architectural choice outside the existing ADRs.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/engine.md`. Historical observations are not a new parity certification.
