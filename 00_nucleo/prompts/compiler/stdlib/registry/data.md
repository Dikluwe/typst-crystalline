# L0 — compiler/stdlib/registry/data
Hash do Código: e4417b40

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/data.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Register read, csv, json, yaml, toml, cbor, xml and plugin. JSON/YAML/TOML are callable parents with only short-named encode children. Preserve cbor.encode and plugin.transition. Do not add global encoder aliases or serializers.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.
