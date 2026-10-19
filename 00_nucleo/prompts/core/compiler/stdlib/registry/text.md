# L0 — compiler/stdlib/registry/text
Hash do Código: 82952c36

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/text.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Register decoration/lorem/regex bindings and upper/lower/replace bindings in their established positions. Preserve sub/super/strike/overline/highlight and native identities.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
