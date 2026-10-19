# Prompt L0 — `entities/counter_update`
Hash do Código: 1b9c017b

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/counter_update.rs`

## Evidence before specification

The original extraction preserved CounterAction's unit Step and Update(usize), without duplicating a second action type. That restricted shape could not transport hierarchical arrays, explicit levels or callbacks.

Historical ratified upstream `a51e02804:crates/typst-library/src/introspection/counter.rs:459–517,567–576` accepts positive step levels (default 1), array states and functions. Bilateral probes also established step/update as static and instance public fields. This refuted the hypothesis that the missing behavior required only dispatcher glue. `lab/history/construction/records/00_nucleo/materialization/typst-passo-1148.md` records the evidence, public gate and, in section 12, owner confirmation followed by materialization. The stale pending heading is superseded by that recorded confirmation.

The observation-helper audit found the existing Func variant and owner-local impl here; putting its helper inside impl Func was both invalid at the proposed Rust location and the wrong ownership boundary.

## Ratified action contract

```rust
pub enum CounterUpdate {
    Set(Vec<usize>),
    Step(NonZeroUsize),
    Func(Func),
}
```

Remain a pure L1 enum without I/O. Set retains every language-array component; an integer becomes a one-component vector. Step retains its positive level; zero, negative and wrong-type inputs are rejected before construction. Func retains a callback for applying to current counter state through the separately owned Engine/EvalContext runtime. This data owner does not execute it.

Preserve Clone and payload-required equality/hash. Func equality/hash follow the already defined Func identity/equivalence and never call a callback. Automatic unit-step producers use Step(NonZeroUsize::MIN); former Update(v) maps to Set(vec![v]). The ratified shape supersedes the obsolete two-variant prohibition and original proposed Set(usize)/Step(usize) alternatives. Flat versus hierarchical application remains the relevant consumer's responsibility; it is not a requirement to copy upstream SmallVec storage.

## Observation-only continuity helper

Only under `cfg(p1339_observation)`, provide a crate-visible, read-only owner-local helper. Match the actual Func(function), borrow/clone its existing private observation carrier, and let H10/H12 prove continuity of that action. Return absence for Set/Step. Never create an occurrence, invoke a callback or infer identity from a side table, extension trait, macro, nonlocal impl, name, output, equality or address. Do not alter variants, constructors, Eq, Hash, Debug or normal API. Without that cfg the helper and observation-type references do not exist. This is internal test transport, with no public/default/compatibility/phase change.

## Acceptance and integration

Preserve variant, hash and clone tests and all existing language behavior. Verify complete hierarchical Set transport, positive Step levels and callback retention without execution during equality/hash. Keep the action field and updated type ownership in Content, ElementPayload, the legacy counter state and introspection/layout consumers; do not duplicate CounterAction or retain a second divergent enum. The original extraction's temporary alias was migration logistics, not a new public compatibility requirement. Any further public action-shape change needs its own approved contract.

## Historical record

`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/counter_update.md` preserves the original evidence and chronology.
