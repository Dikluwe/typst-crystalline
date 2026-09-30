# L0 — compiler/stdlib/registry/table_grid
Hash do Código: 13d49f9c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/table_grid.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Register table children in header, footer, cell, hline, vline order with short public names. Preserve flat table/grid aliases, but do not add flat *_hline or *_vline aliases.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
