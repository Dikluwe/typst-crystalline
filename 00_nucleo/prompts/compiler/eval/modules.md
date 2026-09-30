# L0 — compiler/eval/modules
Hash do Código: 786787bd

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/eval/core.toml sha256:e7642a709c937928333439b2a78cdb3a6dbd6b56d67fcc67728efd2a26796e58

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/modules.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/modules.rs:44` — `fn eval_imported_file`.
- `01_core/src/compiler/eval/modules.rs:117` — `fn eval_module_import`.
- `01_core/src/compiler/eval/modules.rs:251` — `fn eval_module_include`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `lab/typst-original/crates/typst-eval/src/import.rs:82-103`.
- `lab/typst-original/crates/typst-eval/src/import.rs:82-105`.
- `typst-library/src/lib.rs:374`.

## Contract

Own imported-file evaluation, module import and include. Obtain sources and
package resolution only through World; preserve current FileId, Route cycle
detection, isolated Engine/scopes, exported bindings, sink and diagnostic traces.
Construct the imported global stdlib object with public name global under lexical
binding std. Ordinary imported modules retain their own stored names.

A bare Module expression uses the syntactic import binding name (identifier,
field or literal file-derived name), not Module::name(). Dynamic source without
as/items fails with `dynamic import requires an explicit name` and
``you can name the import with `as` ``. Dynamic as/items/wildcard imports remain
valid. Wildcards copy exported bindings; item aliases preserve missing-item
diagnostics at the original-name span. Include keeps string/path routes and
causal traces without changing caller bindings into imported exports.

After successful source validation, a bare identifier import with neither as
nor items emits exactly one `this import has no effect` at the source identifier,
without hints or trace. Preserve it even if a later expression fails. Field,
file, alias, items, wildcard, dynamic and failed imports do not acquire this
warning by generalization. Keep existing rename-warning limitations explicit
rather than broadening the warning contract. Existing Portuguese diagnostic
literals in the consumer are observables, not prose to translate.
