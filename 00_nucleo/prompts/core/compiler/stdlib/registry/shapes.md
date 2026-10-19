# L0 — compiler/stdlib/registry/shapes
Hash do Código: cb1d8682

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/shapes.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Register rect, square, ellipse, circle, line, polygon.regular and curve with move/line/cubic/quad/close children. Preserve qualified names and native bindings.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
