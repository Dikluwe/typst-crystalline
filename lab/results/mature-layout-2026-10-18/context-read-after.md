# Context occurrence identity: read-after context audit

## Scope and answer

This is a read-only context sufficiency audit for the maintenance question: “When a Context-produced body revisits realization and returns `Result::Err`, what identity must stay distinct or be reused, who owns arena/frame responsibility, what restoration is required, what is explicitly excluded for panic, and which acceptance cases must not weaken?” No code or tests were run. The report uses current working-tree prompt documents only; it does not rely on checkpoint prose or reports.

The current specification is sufficient to answer the question. Each contextual producer must receive identities distinct from other producers, while reusing its own allocation slots in the order actually reached on that producer’s re-execution. A single arena owns one Locator for contextual producers in a Session; each producer owns one persistent frame. In `evaluate`, restore the caller’s prior context state on every normal return, explicitly including `Result::Err`, after recovering the Locator and slots. Panic catching and changing unwind policy are explicitly excluded; the stated restoration guarantee covers normal returns only.

Preserve already-`Located` aliases as-is, and never synthesize Location from equality, morphology, span, text, hash, Context ID, or address. The arena reserves Locations in its real origin (including those assigned by canonical introspection of that origin); it does not use results from another pass as an offset. Slot retraction leaves old slots reserved without phantom tags/contributions; growth allocates at the shared free tail.

## Exact read set

Only these current working-tree documents were read. SHA-256 is over the file bytes at audit time.

| File | SHA-256 | Portions consulted and reason |
|---|---|---|
| `00_nucleo/prompts/compiler/realize/context_occurrences.md` | `02606cf047e865217ad342c2a0897e7a2c72dfbac1b52d8b09d73a8eef7c67f3` | `# L0` (lines 1–6): layer, consumer, governance, status. `## Context — ratified occurrence identity` / `### Exclusive responsibility` (lines 16–39): arena/frame ownership, identity and slot lifecycle, restore and panic boundary. `### Acceptance` (lines 41–45): owner and pipeline cases plus validation scope. |
| `00_nucleo/prompts/compiler/realize/mod.md` | `da3b2706c65d9eec31886b652ebda65a56b467561370fd3f90c0142018bdf06e` | `## Identity, realization, and coordination` (lines 54–64): existing occurrence and alias invariants. `## Observable acceptance and performance` (lines 66–78): broader language/execution acceptance that must remain intact. `## Occurrence normalization and index construction` (lines 80–90): partially realized inputs, origin reservations, and ContextBlock range. `## Context — occurrence identity` / `### Measurement and gate` and `### Exclusive responsibility` (lines 284–294): this module delegates acquisition of a new Location to the dedicated owner while preserving existing Located values. |

The hashes are byte identities for these files at read time; the prompt’s short `Hash do Código` fields are lineage seals, not the SHA-256 values reported here.

## Owner and intended invariants

The exclusive owner is `00_nucleo/prompts/compiler/realize/context_occurrences.md`, whose declared productive consumer is `01_core/src/compiler/realize/context_occurrences.rs`. The enclosing realization owner, `00_nucleo/prompts/compiler/realize/mod.md`, assigns `locate` the job of retaining existing `Located` values and delegating only new Location acquisition to that module’s private `next` helper. The realization layer owns transformation and identity decisions; L3 coordinates the pipeline and L4 composes services.

The Context owner specifies a pure arena scoped to one Session. `new(origin)` reserves existing origin Locations plus identities assigned by canonical introspection of that same origin. There is one Locator in the arena. A frame contains private Location slots and a cursor, belongs to the actual producer, and persists across that producer’s re-executions. Each evaluation resets its cursor; reached allocations reuse that producer’s slots, appending a fresh slot from the shared Locator only when needed. Frames are not cloned across producers. Already located aliases do not allocate again. Retracted slots remain reserved but produce no phantom tag or contribution.

The API is additive and opaque; arena/frame have no `Clone`. An immutable Arc token may validate that a frame came from its arena, but its address cannot create identity. The owner rules out I/O, global state, unsafe code, and dynamic dispatch. Identity remains independent of morphological equality and presentation details.

On each normal return, `evaluate` recovers Locator/slots and reinstates prior `EvalContext` state, including when the returned value is `Result::Err`. Its operation is invoked once per call. It does not catch panics or change compiler unwind policy. The owner makes no promise to restore state when the operation panics; panic cleanup is outside the specified scope and must not be added by changing the unwind contract under this prompt.

## Acceptance cases that must not weaken

The dedicated owner requires integrated RED for contextual count and `it.location`, then bilateral GREEN against the pinned version. Local tests must use real locatables and introspection and cover:

- reservation of the real origin;
- disjoint identities across producers;
- slot reuse, growth, and retraction on replay;
- restoration of prior context after `Result::Err`.

Pipeline controls must retain aliases, nesting, and selective replay. The enclosing realization owner also requires that equal headings, repeated insertions of one content value, and same-span elements remain distinguishable; returning `it` must not create a phantom occurrence; contextual repetition must not accumulate stale records, lose originals, or duplicate counters. Its broader acceptance includes preserved queryability of replaced headings, original/generated item order, recipe/effect behavior, and one execution of non-contextual code. Those are existing realization acceptance obligations, not optional substitutes for the dedicated owner’s four local cases.

The target is the pinned version stated in the owner, `bd5ebec285505889e8ff1d4f68804ab147a699f3`; do not generalize a passing case into a claim of overall parity. The prompt separately calls for build, full suite, strict Clippy/Rustdoc, formatting, lineage, and SARIF checks under its resource limits when materializing. This audit did not run those checks.

## Minimum task-relevant reading vs implementation closure

For this question alone, the two documents in the read set contain the direct answer and surrounding occurrence/acceptance constraints. No additional Nucleus or ADR was needed to resolve the lifecycle question. This records the chosen read set only; it does not infer that the read-set size caused any cognitive outcome. The comparison should use both reports and their different query choices.

Before actual implementation, read and validate both owner prompts in full and their declared governance closure. The dedicated owner declares ADR-0107, ADR-0108, ADR-0127, ADR-0129, and ADR-0130. The enclosing owner additionally declares ADR-0026, ADR-0029, ADR-0030, and ADR-0109, plus the same five overlapping ADRs. Its six pinned Nuclei are:

- `00_nucleo/prompts/_nuclei/content/map-content-change.toml` — `ec32b8f89dc95f1de7376ca91e6252eb61a5da2ac0326abb7474fe56fd27fd7a`
- `00_nucleo/prompts/_nuclei/content/sequence-realization.toml` — `825182e1dfe3591249a5efbc4c0ddbd937ebffde953c6595e0d1ed8176106435`
- `00_nucleo/prompts/_nuclei/content/source-origin.toml` — `4b70701bc95837b493354283f66121b166ffc1ad670152f9a3d995794c0a6c67`
- `00_nucleo/prompts/_nuclei/eval/textual-recipe-frontier.toml` — `f225f3039d4663e9bfcb408d26cf1cd6e557f089013bc2d95090ce16c52080c6`
- `00_nucleo/prompts/_nuclei/math/bare-insertion.toml` — `5cfb723d50e9518cdcee2df348927d02330b6019df96acef8394c1429cc950ff`
- `00_nucleo/prompts/_nuclei/table/cell-realization.toml` — `9f76ec456ed18fb3f64ff5d03d6ca7bb4049fe700c700c42fa80d072db8bc0e0`

That closure is derived from current owner headers, not from reading those ADRs/Nuclei in this experiment. Confirm the files still match the owners’ full SHA-256 pins and pass ownership/Nucleus-integrity validation before materialization. Also inspect applicable repository implementation instructions and the affected code’s actual call sites/tests before editing. The owner says status is “materialization in progress”; it does not leave the stated lifecycle or acceptance requirements unresolved. This audit makes no claim of cognitive or performance improvement.
