# L0 — core/compiler/eval/stdlib_snapshot
Hash do Código: c5340d03

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/stdlib_snapshot.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130, ADR-0131.

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/builtin-module-identity.toml sha256:e79fb4e7f291d69f9108a4b17738b205861d818c6e9eeea873c337dc64bad2f8

## Evidence before decision

Received HEAD `3c6b0ce7b7632cc74b004cea6681e4746e63d28d`, uncommitted
tree. `compiler/eval/mod.rs:640,848` and `eval/modules.rs:60` construct
stdlib scopes independently; `entities/module.rs` and `entities/func.rs`
define the observable Module/Element/With identities. The exact experimental
resolver/instantiator is in
`lab/experiments/stdlib-qualification/snapshot.rs:71-267`; qualification of
ordered inputs, Features and real registries precedes this decision in
`00_nucleo/diagnostics/stdlib-qualification-2026-11-06.md`.
`stdlib-cli-residual-2026-11-06.md` lifts only the specific +0.74% blocker;
the productive ELF still needs its own acceptance measurements.

At received HEAD `0a15ccebf861e3cc0ea16d0397d55dc057664556`, before this reservation change,
the productive source at
`01_core/src/compiler/eval/stdlib_snapshot.rs:80-85` borrows an immutable
original Scope, creates a private empty destination and fills it once in
original declaration order. `01_core/src/entities/scope.rs:96-97` supplies the
original's stable binding length; module and function namespace instantiation
descend through this same scope routine. This source evidence qualifies an
initial reservation using each original's own length, rather than its map
capacity or another scope's length. The choice concerns RAM construction
mechanics (ADR-0029/0030), not language parity (ADR-0107). The preservation
inference is refuted by changes to values, order, diagnostics, authority,
aliases, occurrences, input/feature guards or per-instance identities.

## Private contract

Own a private `StdlibSnapshot` containing complete SysInputs, complete Features
and the owned stdlib Scope. No RootLibrary abstraction, runtime policy/mutation
flags, public portal, I/O, clock, global mutable cache or lab imports.
Its private `new(SysInputs, Features, Scope)` constructor stores exactly those
values, and `scope(&self) -> &Scope` borrows its captured scope. The resolver
uses these boundaries; tests do not require exposing fields publicly.

Provide the eval-internal boundary
`resolve(&RefCell<Option<Arc<StdlibSnapshot>>>, &SysInputs, Features)` returning
`(Scope, Option<Arc<StdlibSnapshot>>)`. It receives already queried inputs;
never obtains World or evaluates user code. Every decision over closed enums
must remain exhaustive, without a wildcard that silently admits future variants.

On an empty slot, call the hub's existing feature-aware stdlib builder once,
capture that fresh Scope with cloned inputs/features, install the Arc into the
caller slot and return the Scope clone and that seed. Root composition remains
outside this helper: a stable import tree has root plus first-import builders,
not a builder cached from a previous evaluation.

On a populated slot, reuse only if lengths and all ordered input key/value
pairs match and complete Features match. Do not use an incomplete key, hash-only
test or order-insensitive map equality. A match returns a new per-file instance
and the same seed. A mismatch builds a fresh seed for that imported subtree and
returns it, leaving the caller slot/seed intact. A descendant may seed itself
without replacing an ancestor's seed. Dropping the context tree ends retention.

## Bounded builtin graph instantiation

This is not a generic cloning API for evaluated Modules, closures, plugins,
contexts or arbitrary dynamic constructors. It is limited to the current pure,
acyclic builtin registry plus string-valued sys.inputs. The registry tests must
inspect all feature profiles and reject expansion outside the qualified domain.

Every `Instance::scope` must initially construct its private destination with
`Scope::with_capacity(original.len())`, using that invocation's immutable
original. This applies uniformly to the root graph, module namespaces and
function namespaces. Keep the existing declaration-order loop and value
instantiation unchanged. Empty, small, large, nested and large-then-small
original scopes must preserve their semantics; the destination may grow past
its initial reservation if later used for further bindings. Do not reserve
seed construction, root registries, first clones, builders or the Module/Func
identity vectors, reuse scratch capacity, eliminate clones or share new
identity-bearing wrappers. Do not introduce a helper bridge or observer port.
The constructor contract belongs to
`00_nucleo/prompts/core/entities/scope.md`; this owner specifies its own
known-length use without a new shared nucleus.

For each instance, use local identity maps. Recursively recreate Modules and
their scopes, retaining Std/Format authority, public names, all existing content,
introspection, bibliography and document-info carriers. Module aliases in one
instance point to that instance's same new module; separate files never share
the template's identity-bearing wrappers. Recursively clone Arrays/Dicts/Args
without changing order, named occurrences or provenance. Immutable builtin
primitive/symbol/type/math-operator leaves retain their existing clone behavior.

Recreate Native/NativeWithEngine/Format functions and recursive namespaces using
the existing constructors and call pointers. Element and With wrappers get new
identity per file while preserving local aliases, argument occurrences and
diagnostic spans. The qualified Element is raw.line with its existing stateless
constructor: do not execute it during instantiation or broaden this into a
dynamic constructor cloning contract. Closure/Plugin functions are outside the
template domain; never silently share them as newly instantiated functions.
No new public methods or changes to Module/Func/Value equality/representation.

## Tests, acceptance and ownership

Declare test child `stdlib_snapshot/tests.rs` with its own owner
`00_nucleo/prompts/core/compiler/eval/stdlib_snapshot/tests.md`. Keep its
`#![cfg(test)]` inside the child file and a plain `mod tests;` declaration here:
the architectural parser needs the file-local gate, and duplicated outer/inner
gates are unnecessary. Co-locate the independent root-construction smoke test
in a `#[cfg(test)] mod context_creation_tests` here: actual EvalContext::new
must leave the private slot empty. This is genuine inline coverage of the
delayed-root boundary; the separate test consumer owns the broader suite.
Preserve the
root/query/diagnostic/alias/guard invariants with real executable tests, including
large inputs and mismatch ancestry. Directed identity/input/feature guard
mutations must be rejected by assertions rather than compilation failures.

Materialization is the qualified delayed candidate with production naming and
closed decisions, not another optimization experiment. Update L0 first,
RED→GREEN, then valid bijective ownership/nuclei before restricted resealing.
Build/test/lint and remeasure CLI/watch including no-import controls, imports,
real compatible packages and large inputs. Keep layout and source/font freshness
unchanged; no claims that byte-equal PDFs define language parity.
