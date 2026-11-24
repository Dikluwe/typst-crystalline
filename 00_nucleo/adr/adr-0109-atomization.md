# ADR-0109 — Atomization and ownership within a layer

**Status:** ACCEPTED
**Decision owner:** Repository owner
**Consolidated:** 2026-11-24
**Amended:** 2026-11-24 — visibility scope clarified. Approved by the owner on 2026-11-24 (“Aprovo”, in chat, after review of the wording).

## Evidence

Moving layout into an entity's data file would require an import from entities back into the compiler, access to private layouter state and layouter generics on the entity. The established alternative delegates from the exhaustive dispatch to a descendant of the layout module. The content representation also has an approved extension leaf with an object-safe bridge. Both forms are present in the current owners. [00_nucleo/prompts/core/compiler/layout/mod.md](../prompts/core/compiler/layout/mod.md) · [00_nucleo/prompts/core/entities/content.md](../prompts/core/entities/content.md) · [00_nucleo/prompts/core/entities/elements/dynamic.md](../prompts/core/entities/elements/dynamic.md)

## Decision

Group behavior by responsibility within its owning layer. In L1 the closed `Content` enum stores sequences as `Arc<[Content]>` for constant-time cloning and retains the approved representations of individual element payloads. Native dispatch uses an exhaustive static match with a short arm per feature. A layout arm delegates to a free function such as `heading::layout(layouter, heading)` in the layout subtree (Form B). The descendant accesses layouter state through module descent. Keep entity data independent of rendering. Moving behavior into the data struct (Form A), widening `Layouter` state to `pub(crate)`, adding a vtable or removing exhaustive dispatch does not implement this decision.

The visibility restriction covers `Layouter` state. Use the narrowest visibility that reaches the actual consumer. A layout operation or its parameter type needed by a sibling module can use `pub(in crate::compiler)`, or `pub(crate)` when the consumer or signature requires it. Crate visibility elsewhere follows the owning responsibility and is not prohibited by this decision. Existing state fields visible in `compiler` through `pub(super)` keep their current scope, and new external access should prefer an operation.

The E1 extension approved under ADR-0106 is confined to `Content::Dynamic(Arc<dyn DynElement>)`. Preserve the native trait bridge, stable kind, typed field access, structural dynamic equality and the documented realization responsibilities. The native elements retain static dispatch. ADR-0105 remains in force for the conditional transition to PropMap. Starting that transition requires a decision on the mechanism that preserves exhaustive handler coverage. [00_nucleo/prompts/core/entities/elements/dynamic.md](../prompts/core/entities/elements/dynamic.md) · [00_nucleo/prompts/core/compiler/layout/dynamic.md](../prompts/core/compiler/layout/dynamic.md)

Apply cohesion in L1–L4. In L3, export, font access and world adaptation belong to their respective domains. Keep a stable facade when moving implementation to descendants, keep visibility as narrow as the responsibility allows and prefer exposing a needed operation to exposing mutable state. Keep domain tests near their owner. Cross-domain end-to-end tests may stay together when a single invocation exercises the interaction. A cohesive foundational type, a data table, generated code or cross-domain test fixture can justify a large file. The historical 800-line guideline triggers inspection. Record the reason for an exception. Move coherent slices progressively and compare the existing public behavior before and after extraction, including any explicitly contracted artifact snapshot. [00_nucleo/prompts/infra/shaper.md](../prompts/infra/shaper.md) · [00_nucleo/prompts/infra/export/tests/mod.md](../prompts/infra/export/tests/mod.md)

For evaluation state, declare the dependencies a function reads or changes. Group related dependencies in a minimal, scoped carrier and borrow immutable state where possible. Prioritize state that differs between recursive frames when reducing broad context access. Evaluation-wide collectors, resource limits and immutable configuration may remain in the context when their role is documented. This discipline applies to future extractions within their owners and preserves existing public contracts.

Preserve the maintenance guarantees of ADR-0104. Keep the context a reader must inspect and edit for one feature local. Use the owning L0 for each productive consumer and pinned Tekt nuclei for genuinely shared obligations under ADR-0129. When a separately approved design replaces compile-time exhaustiveness with a descriptor or convention, establish equivalent automatic coverage through an exhaustive test or lint before relaxing the compiler guarantee. Evaluate the files and responsibilities touched per feature. An import count such as `content→elements` cannot establish independent readability.

The runtime `Value` contract also uses closed dispatch. Its owner specifies first-class types, callable subsets and argument bundles. Public variants follow the approval gate in ADR-0127. [00_nucleo/prompts/core/entities/value.md](../prompts/core/entities/value.md) · [00_nucleo/prompts/core/entities/args.md](../prompts/core/entities/args.md)

## Compatibility and scope

This consolidates ADR-0026 and its R1 representation revision, progressive decomposition in ADR-0036, domain cohesion in ADR-0037 and ADR-0100, and the reader-cost and verification obligations in ADR-0104. This decision and ADR-0129 expressly supersede ADR-0104’s shared `_comum.md` prescription with pinned nuclei and single-consumer ownership. Preserve the owner’s accepted sequence for the original migration: move element layout into files in the current crates first. A project-wide restructuring or new crate boundary requires its own evidence and decision. The arrangement of files grants no permission to change observable behavior, public contracts or phase ownership.
