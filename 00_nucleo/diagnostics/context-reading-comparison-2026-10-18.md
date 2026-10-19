# Context reading comparison — 2026-10-18

## Method and evidence

Two separate agents received the same bounded maintenance question: for a
Context-produced body replay ending in `Result::Err`, identify occurrence reuse
versus distinction, arena/frame ownership, restoration, the panic exclusion,
and acceptance cases that must remain. Neither implemented code or ran tests.
One used only checkpoint `8ec827fd975bf14cd849bebb9ede245e49033337`; the other used
only the current owner documents. Their exact read sets, byte hashes, consulted
sections, and conclusions are retained in the
[before report](../../lab/results/mature-layout-2026-10-18/context-read-before.md)
and [after report](../../lab/results/mature-layout-2026-10-18/context-read-after.md).

This compares the recovery of a bounded contract, not implementation performance
or a blind controlled experiment. Reading choices were left to each agent. The
before reader consulted supporting ADRs and nuclei; the after reader stopped at
the two owners and listed the unread governance closure. That difference cannot
be attributed causally to the reorganization.

## Comparison

| Question | Recovered from checkpoint and current owners |
|---|---|
| Different producers | Distinct identities even for equal bodies/fields |
| Replay of one producer | Persistent frame; reuse reached slots, grow at the shared free tail, retain retracted reservations without phantom contributions |
| Already located aliases | Preserve their Location and do not allocate again |
| Origin | Reserve the real origin and its canonical introspection; no unrelated prior-pass offset |
| Ownership | Dedicated Context occurrence owner owns arena/frame; realization facade delegates new allocation and preserves existing Located content |
| Error return | Recover allocator/frame state and restore the previous context on every normal return, including `Result::Err` |
| Panic | No catching panic or changing unwind policy; no added panic-restoration guarantee |
| Acceptance | Real origin, producer separation, reuse/growth/contraction, error restoration, aliases/nesting/selective replay, and bilateral count/`it.location` controls |

Both readers identified the same task-specific obligations and no unresolved
contract question for this bounded task. Both require additional full owner,
governance, nucleus, implementation, and test inspection before materialization.
The minimum direct answer still resides in the same two owners. Their dependency
closure was not reduced by changing filenames or shortening prose.

## Decision and limits

The pilot supports recoverability of these particular obligations after
consolidation. The separate clause review and approval-receipt reconciliation
support the editorial changes; lineage and tests assess different properties.
There is no demonstrated reduction in cognitive cost, implementation time, or
number of required architectural dependencies. Do not use this result to claim
that all prompts are adequate, all boundaries are correct, or Tekt outperforms
vanilla. Continue owner-specific consolidation with explicit obligation mapping.
