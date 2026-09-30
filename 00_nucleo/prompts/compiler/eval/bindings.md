# L0 — compiler/eval/bindings
Hash do Código: 7a346727

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/bindings/mod.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/bindings/mod.rs:1` — `exclusive lineage header`.

## Contract

This module is a static module and reexport boundary, not a semantic dispatcher.
Declare access, binding, field_access, method_dispatch and value_methods and keep
the existing bindings::<function> call paths and visibility. Each child has its
own exclusive prompt under 00_nucleo/prompts/compiler/eval/bindings/.

Delegate let/destructuring/assignment to binding; mutable-place lookup and
unknown-variable diagnostics to access; field/content/snapshot access and
symbol-deprecation adapters to field_access; mutation and mutable accessors to
method_dispatch; AST adapters and the native function.where helper to
value_methods. Reexport the existing crate-internal content, outline-entry,
element-field and locatability helpers without making them public.
Do not duplicate algorithms, register bindings or absorb font_dict, rules or
show_rule_termination behavior.
