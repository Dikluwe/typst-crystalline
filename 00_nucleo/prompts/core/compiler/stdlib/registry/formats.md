# L0 — compiler/stdlib/registry/formats
Hash do Código: 51b16059

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/formats.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Construct PDF and HTML once each and one asset value. Pass those values by reference to the format factory. Register format, asset only with Bundle, and html. Return the constructed PDF for later module insertion. Registering html does not enable its feature. Do not reconstruct functions/catalogs or implement lookup gates.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
