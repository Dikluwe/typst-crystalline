# L0 — compiler/stdlib/registry/mod
Hash do Código: 9d73338d

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/registry/mod.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Compose a fresh Scope per invocation using explicit inputs and features.
Call registry families in the established binding order. Pass the single PDF
returned by formats to modules. Do not use a dynamic registry, wildcard imports,
reflective lookup, global state or native wrappers. Transport feature arguments
without implementing their behavior. Keep microbenchmarks test-only through its own inner
#![cfg(test)]; do not contain clocks, Criterion or production benchmarking APIs.

Preserve the owned public names, Value kinds, native bindings, children and
aliases. Do not implement Args validation, casts, export or lookup behavior;
those operations are outside this registration consumer.

## Acceptance

Verify the owned behavior through local tests and its existing integrations.
Preserve public paths and observable results. Ownership, nucleus integrity and
lineage checks must remain valid; no unrelated consumer is authorized by this prompt.

## Test-module conditional integration

Before decision, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working
tree. Duplicate cfg(test) occurs at registry/tests.rs:7 / parent:48 and
registry/microbenchmarks.rs:7 / parent:51. Sources/L0/UTC/status/diffstat are in
`lab/results/construction/diagnostics/cfg-test-ci-2026-10-13/before.json.gz`
and refinement-before.json.gz. Pinned Clippy reproduces duplicate attributes.
Removing the inner condition preserves Rust tests but the pinned architectural
linter does not propagate the parent's condition and reports V2 on helpers.

Remove only outer cfg(test) attributes from those declarations in this owner.
Keep module paths/names/visibility and path attributes. Each listed file's
inner `#![cfg(test)]` still excludes the whole module, imports and helpers from
production. Test consumers and their L0 stay intact. Add no dependency/logic/
API/default mode/phase. Validate the same test list, core suite, production
build without cfg(test), Clippy and architecture.
