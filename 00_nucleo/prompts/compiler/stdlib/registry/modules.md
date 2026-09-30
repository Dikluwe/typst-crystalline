# L0 — compiler/stdlib/registry/modules
Hash do Código: f599ce8e

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/modules.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Insert sym, calc, emoji, the supplied PDF, color, gradient, math and sys with supplied inputs. Color/gradient are Type values; math uses its own factory. Do not reconstruct PDF or interpret inputs.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
