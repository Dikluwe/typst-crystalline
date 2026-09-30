# L0 — compiler/stdlib/registry/contextual
Hash do Código: 1300f488

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/contextual.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Register metadata, state, counter, context, state/counter operations, query, selector, here, target and locate. State/counter/selector remain Type values; native ABIs remain unchanged. Do not decide targets or execute contextual reads.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
