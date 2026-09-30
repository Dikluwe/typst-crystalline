# L0 — introspect/fixpoint
Hash do Código: 9d8c3e8c

```text
```

## Ownership and evidence

Layer: L1. Sole consumer: `01_core/src/compiler/introspect/fixpoint.rs`.
`run_fixpoint` at `:75` returns `Result<TagIntrospector, FixpointError>`;
`introspect_to_fixpoint` at `:149` is its direct wrapper.
The current L3 consumers use single-walk runtime entrypoints from
`00_nucleo/prompts/compiler/introspect.md`, not this full reevaluation loop.
Source call-site evidence and its reproduction are recorded in
`00_nucleo/diagnosticos/sanitization-introspection-columns-history.md`.

## API and bounded loop

Preserve MAX_FIXPOINT_ITERATIONS = 5 and these signatures:

```rust
pub fn run_fixpoint<F>(
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
    eval_step: F,
) -> Result<TagIntrospector, FixpointError>
where
    F: FnMut(&mut Engine<'_>, &mut EvalContext) -> SourceResult<Content>;

pub fn introspect_to_fixpoint<F>(
    engine: &mut Engine<'_>,
    ctx: &mut EvalContext,
    eval_step: F,
) -> Result<TagIntrospector, FixpointError>
where
    F: FnMut(&mut Engine<'_>, &mut EvalContext) -> SourceResult<Content>;
```

Begin with an empty previous introspector and no previous tag hash. On each
bounded iteration, clone that introspector into ctx.introspector, evaluate the
closure, then walk its content with a fresh Locator, tag vector, introspector,
auto-label counter and default StyleChain; the language argument is None.
No separate CounterStateLegacy value is created or returned.

Compute the tag hash and coordinate the child-owned passes in current order:
state functions, counter functions, state displays, counter displays,
equation numbering, equation supplements, equation realization.
After those passes, equality with the preceding iteration's tag hash returns
the current introspector. Otherwise retain its hash/introspector and repeat.
The first hash alone cannot establish convergence. Exhaustion returns
FixpointError::NotConverged; evaluation and fallible postprocessing errors
return FixpointError::Eval immediately, not a partial success.

## Boundaries and acceptance

Hashing is owned by `00_nucleo/prompts/compiler/introspect/convergence.md`;
callback semantics by `00_nucleo/prompts/compiler/introspect/from_tags.md`;
the causal walk by `00_nucleo/prompts/compiler/introspect.md`.
This helper does not independently add the parent's bibliography conversion,
outline-entry synthesis or parent-index preparation. Do not infer equivalence
of every prepared store with the production entrypoint.

Keep the linear bounded mechanism without new memoization or I/O.
Test stable successive evaluations, dependence on the preceding introspector,
bounded non-convergence, evaluation/runtime errors and wrapper equivalence.
Hash equality is this helper's mechanical stopping rule, not proof of all
language observables or authorization to substitute it for pipeline
stabilization. No promised future migration or production adoption is implied.
