# L0 — compiler/stdlib/registry/math
Hash do Código: 7f4a553a

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/math.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Register the compatible global math bindings, including op with Engine ABI. Do not remove those bindings or introduce math.underover. Global cancel is not replaced by its qualified counterpart.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
