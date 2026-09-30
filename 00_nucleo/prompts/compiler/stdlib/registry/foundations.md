# L0 — compiler/stdlib/registry/foundations
Hash do Código: e7702def

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/foundations.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Register primitive types, repr/range and the global color constructors rgb, luma, oklab, oklch and cmyk. Do not install bare hsl, hsv, linear_rgb or linear-rgb. Register alignment and direction bindings in their established positions. Preserve Type callability without implementing constructors here.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
