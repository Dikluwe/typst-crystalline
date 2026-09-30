# L0 — compiler/eval/bindings/value_methods
Hash do Código: 73b6b045

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/bindings/value_methods.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/bindings/value_methods.rs:42` — `fn eval_state_method`.
- `01_core/src/compiler/eval/bindings/value_methods.rs:104` — `fn state_at_dispatch`.
- `01_core/src/compiler/eval/bindings/value_methods.rs:186` — `fn eval_color_method`.
- `01_core/src/compiler/eval/bindings/value_methods.rs:260` — `fn parse_counter_display_args`.
- `01_core/src/compiler/eval/bindings/value_methods.rs:345` — `fn eval_counter_method_value`.
- `01_core/src/compiler/eval/bindings/value_methods.rs:493` — `fn eval_counter_static_method_value`.
- `01_core/src/compiler/eval/bindings/value_methods.rs:633` — `fn eval_version_method_value`.
- `01_core/src/compiler/eval/bindings/value_methods.rs:655` — `fn resolve_state_label`.
- `01_core/src/compiler/eval/bindings/value_methods.rs:684` — `fn eval_element_where`.
- `01_core/src/compiler/eval/bindings/value_methods.rs:717` — `fn native_function_where`.
- `01_core/src/compiler/eval/bindings/value_methods.rs:737` — `fn native_element_fields`.
- `01_core/src/compiler/eval/bindings/value_methods.rs:1170` — `fn validate_element_locatability`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/func.rs:412-450`.
- `introspection/counter.rs:338-357`.
- `introspection/query.rs:160-175`.

## Contract

Own AST adapters for state, counter, color, version and selectors, and the
native function.where semantic helper. Evaluate receiver/arguments once and
delegate native validation and execution to the corresponding stdlib owners;
do not duplicate color, version-index, state or counter parsers.

state supports update/get/display/at/final; counter supports
step/update/get/display/at/final. Resolve causal label requests and register
effective observations before resolution can fail. Counter display preserves
the current at domain (Label/Location/Selector/Auto), both boolean and
string/function/auto numbering. Static at/display adapters preserve a syntactic
label instead of losing it to generic evaluation. Preserve receiver/argument
origins, context gates and failure priorities. Color prepends the receiver and
delegates with World; version delegates to dispatch_version_method. Synthetic
receivers with no syntax retain legitimately detached origin.

Selector and/or/within adapters preserve ordered self-first composition and
the public conversion to query selectors. before/after routing delegates to
the semantic helper owned by
00_nucleo/prompts/compiler/eval/call_dispatch.md; it does not query a document.

function.where recognizes real native element identity by typed function pointer
and the static public field catalog, never by spelling or by executing a function.
Aliases retain identity; With, closures and plugins are rejected as required by
the current helper. Do not promote an arbitrary Func::Element into a native
element. Bound receiver resolution precedes arguments; the static form evaluates
arguments before casting self. Only field names are validated, not field values.
Spreads keep first-key position and last value; invalid literal duplicates remain
syntax errors. Even an empty filter constructs Selector::Element, not Kind.

Preserve `missing argument: self`, positional-self hints, function casts,
`where() can only be called on element functions`, invalid-element-field and
unexpected-argument diagnostics at their current occurrence/value/call anchors.
Keep raw.line fields number/count/text/body and par.line's static field catalog
and locatability metadata without inventing query occurrences. Arbitrary dynamic
element names do not inherit raw.line identity. Locatability validation is a
typed shared helper for consumers, not permission to execute query/counter here.

Under cfg(p1339_observation), retain the callback ledger adapter for functional
counter.update actions with the real aggregate argument origin. No ledger event,
extra evaluation or public instrumentation API is added to normal builds.
