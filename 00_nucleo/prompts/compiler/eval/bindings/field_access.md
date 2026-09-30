# L0 — compiler/eval/bindings/field_access
Hash do Código: 1b3572cf

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/bindings/field_access.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/bindings/field_access.rs:434` — `fn native_public_name`.
- `01_core/src/compiler/eval/bindings/field_access.rs:451` — `fn is_user_defined_closure`.
- `01_core/src/compiler/eval/bindings/field_access.rs:474` — `fn is_format_pdf_namespace`.
- `01_core/src/compiler/eval/bindings/field_access.rs:479` — `fn is_format_pdf_function`.
- `01_core/src/compiler/eval/bindings/field_access.rs:497` — `fn is_a11y_pdf_target`.
- `01_core/src/compiler/eval/bindings/field_access.rs:504` — `fn is_effective_format_module`.
- `01_core/src/compiler/eval/bindings/field_access.rs:511` — `fn is_effective_std_module`.
- `01_core/src/compiler/eval/bindings/field_access.rs:519` — `fn is_effective_symbol_module`.
- `01_core/src/compiler/eval/bindings/field_access.rs:537` — `fn warn_deprecated_symbol_binding`.
- `01_core/src/compiler/eval/bindings/field_access.rs:557` — `fn eval_field_access`.
- `01_core/src/compiler/eval/bindings/field_access.rs:707` — `fn eval_value_field_access`.
- `01_core/src/compiler/eval/bindings/field_access.rs:1057` — `fn element_or_type_with_name`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/fields.rs:14–67`.
- `foundations/func.rs:280-308`.
- `foundations/func.rs:412-450`.
- `lab/typst-original/crates/typst-eval/src/code.rs:347-366`.
- `lab/typst-original/crates/typst-eval/src/code.rs:347–366`.
- `lab/typst-original/crates/typst-library/src/foundations/func.rs:280–308`.
- `lab/typst-original/crates/typst-library/src/foundations/func.rs:291–308`.
- `lab/typst-original/crates/typst-library/src/foundations/module.rs:139-150`.
- `lab/typst-original/crates/typst-library/src/foundations/value.rs:157–169`.
- `lab/typst-original/crates/typst-library/src/lib.rs:221-225,374`.
- `typst-eval/src/code.rs:347-367`.
- `typst-eval/src/code.rs:347–366`.
- `typst-library/src/symbols.rs:12-35`.

## Contract

Own eval_field_access, eval_value_field_access, field_callee_error,
content method projection, queried-content projection and deprecation adapters.
Keep the existing static catalogs and visibility; method implementation remains
delegated to its native owner. Resolve a receiver once and anchor pure lookup
errors at the supplied field span, without reconstructing syntax or origin.

Native function fields traverse the actual namespace (including With unwrapping).
Unknown native fields use the canonical function identity/name, not a lexical
alias. User closures, also through With, retain
`cannot access fields on user-defined functions`. Do not generalize that rule to
plugins or element functions. Module errors use the stored module name:
``module `<name>` does not contain `<field>` `` with no hint, at the field only.
Aliases of std refer to the object named global; an ordinary std.typ module stays
std. Never identify std by a local spelling or by comparing its bindings.

Dictionary, raw-content and float lookup preserve their distinct diagnostics and
field-only anchors. Other scalar misses use canonical long type names. Array
len/first/last keep their existing AST routing; unknown array fields fail as
array field access. Preserve the known distinction between isolated fields and
bound-call interception rather than making all methods first-class values.
In particular isolated Float is-nan/is-infinite/signum/to-bytes, Angle deg/rad,
Version at, Func where and SpotColorant tint do not become bound method values.
Type lookup exposes only its actual static methods; Type::Function is not thereby
a general constructor. Delegate static float/version/spot operations to stdlib.

Keep Duration days/hours/minutes/seconds, Version major/minor/patch,
Length abs/em, Relative length/ratio, Args positional/named, Stroke paint/thickness
and Alignment x/y projections (the corresponding single axis or None).
For contextual text.size, prefer the typed top style
size over a custom field and preserve derived script size; use the existing
default 11pt only when absent. text.lang keeps its existing contextual lookup.
Do not execute a style/font callback while projecting a field.

Content exposes func/has/at/fields/location through the closed ContentField
projection and existing helpers. Distinguish absent from a present None/Auto.
Consume self/default arguments coherently using the method_dispatch helpers.
OutlineEntry.body routing stays restricted to actual entry content; entry level,
element and fill projection remains distinct from body evaluation.

For LocatedContent Some, the pinned snapshot nucleus is authoritative: use its
complete ordered fields, not raw fallback, styles or reconstructed defaults.
Missing at uses the supplied default only when the field is absent. Preserve
Location independently. LocatedContent None keeps legacy projection, including
realized Equation. Raw projection uses causal presence masks. Equation exposes
body/block and explicit styles; alt is Str or explicit None and precedes body,
with top style precedence and omitted alt absent. MathPrimes exposes count;
MathLr exposes explicit size then body; MathStretch always exposes body and
exposes size only when explicit. Annotated math variants always expose body and
expose annotation only when present, including explicit None, in canonical order.

Feature gates use effective typed native/module identity, not homonymous
dictionaries or aliases by spelling. The three gated pdf members require
A11yExtras; absence produces the existing field diagnostic with its two hints.
HTML format fields require Html, bundle format fields and std.asset require
Bundle; features remain independent. Identity of the actual format namespace
is its existing surface, not a fabricated default projection. Active features
use ordinary lookup and the real constructor namespace, never a placeholder.

Symbol deprecation warnings are delegated to the sym owner and emitted on
successful root binding resolution with the existing causal anchor. Missing
roots do not warn; an invalid modifier of a deprecated root preserves the root
warning. Keep legacy join as a measured extension and the un-deprecated tack
bindings silent. Preserve the narrow tri fallback only for canonical gt/lt with
eq/not modifiers after normal modifier lookup fails; use sym lookup, preserve
the legacy routes and repr parents, and do not recognize arbitrary named Symbols.

Keep messages, severities, hint order, traces and spans unchanged. Public field
availability or generic method-value promotion requires a separate contract
decision; this owner does not authorize it.
