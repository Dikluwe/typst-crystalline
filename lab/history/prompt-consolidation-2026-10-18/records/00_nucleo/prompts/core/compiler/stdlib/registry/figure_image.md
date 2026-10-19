# L0 — compiler/stdlib/registry/figure_image
Hash do Código: 4d5f6e93

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/figure_image.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Register figure with caption, image and Type::Tiling, preserving public names and native bindings.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
