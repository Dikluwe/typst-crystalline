# L0 — compiler/eval/bindings/access
Hash do Código: fc6f9833

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/bindings/access.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/bindings/access.rs:33` — `fn missing_key`.
- `01_core/src/compiler/eval/bindings/access.rs:50` — `fn unknown_variable`.
- `01_core/src/compiler/eval/bindings/access.rs:71` — `fn access`.
- `01_core/src/compiler/eval/bindings/access.rs:165` — `fn access_dict`.

## Contract

Own lookup of mutable places and access diagnostics. Ident uses Scopes;
Parenthesized recurses; FieldAccess requires a dictionary place; FuncCall permits
only the mutable first/last/at accessors delegated to method_dispatch. Other
expressions are unassignable. access_dict requires a dictionary and retains
missing-key diagnostics and their causal spans.

An unresolved identifier emits ``unknown variable `<name>` ``. Preserve the existing
hyphen/subtraction hints and the distinction between unknown variables and
variables captured by closures that cannot be mutated. Preserve type, temporary
mutation and dictionary-key diagnostic priorities; canonical long type names
delegate to 00_nucleo/prompts/core/compiler/eval/operators/error_formatting.md.
Assignment may insert a new dictionary key through the existing assignment
path; place lookup must not invent values or change lexical capture semantics.
