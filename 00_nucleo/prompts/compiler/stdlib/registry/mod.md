# L0 — compiler/stdlib/registry/mod
Hash do Código: 108a61a6

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/mod.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Compose a fresh Scope per invocation using explicit inputs and features.
Call registry families in the established binding order. Pass the single PDF
returned by formats to modules. Do not use a dynamic registry, wildcard imports,
reflective lookup, global state or native wrappers. Transport feature arguments
without implementing their behavior. Declare microbenchmarks only under
cfg(test); do not contain clocks, Criterion or production benchmarking APIs.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
