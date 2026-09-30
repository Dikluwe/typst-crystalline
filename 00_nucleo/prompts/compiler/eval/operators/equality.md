# L0 — compiler/eval/operators/equality
Hash do Código: 8ecabe9e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/introspection/content-snapshot.toml sha256:5a0270231de70be1212dbd17298cce34b7161b527d74b4b589e3f4c69d35ce24

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/operators/equality.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/operators/equality.rs:21` — `fn apply_binary`.
- `01_core/src/compiler/eval/operators/equality.rs:91` — `fn value_eq`.
- `01_core/src/compiler/eval/operators/equality.rs:103` — `fn values_eq`.
- `01_core/src/compiler/eval/operators/equality.rs:138` — `fn selector_contains_element`.
- `01_core/src/compiler/eval/operators/equality.rs:163` — `fn selectors_eq`.
- `01_core/src/compiler/eval/operators/equality.rs:203` — `fn counter_keys_eq`.
- `01_core/src/compiler/eval/operators/equality.rs:210` — `type ContentFields =`.
- `01_core/src/compiler/eval/operators/equality.rs:213` — `fn raw_content_fields`.
- `01_core/src/compiler/eval/operators/equality.rs:226` — `fn content_values_eq`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/content/raw.rs:354–361`.
- `foundations/selector.rs:75-81`.
- `foundations/value.rs:295-299`.
- `introspection/counter.rs:243,256`.

## Contract

Own Eq/Neq/In/NotIn, values_eq and value_eq. This is language equality, not
derived Rust PartialEq or ObservationEq. Coerce Int/Float recursively, including
array/dictionary values. Containers preserve their existing key/cardinality
comparison; raw Content uses morph_canon and ignores baked render style while
retaining semantic morphology. Version compares all components with zero padding.
Length equals Relative only with zero relative component; Ratio equals Relative
only with zero absolute component and |rel-ratio| < 1e-9. Other existing families
retain their derived fallback. Neq is the negation of the same relation.

For any comparison involving LocatedContent Some, use the pinned snapshot
contract: compare canonical element function and linguistic field sets excluding
label; recursively compare values. For raw/None counterparts use only their
explicitly projected fields, without filling defaults. Field insertion order,
Location and carrier discriminant do not determine this equality. Different
field sets differ; raw is neither automatically equal nor automatically unequal
to its realized counterpart. Keep raw/raw and None fallback unchanged.

Selector::Element compares canonical Func using current function equality and
the ordered filter sequence by name position and recursive values_eq. Empty
group differs from bare Kind; aliases retain identity; NaN filters remain
nonreflexive. Propagate through nested containers and selector compositions,
preserving antecedent comparison for pairs without Element. Counter keys with
Element use the same recursive selector relation, including field order. Do not
exclude Int/Float-equal pairs by structural hash bucket or shortcut with Arc
identity. Before compares base/end/inclusive recursively; After compares
base/start/inclusive; distinct variants differ, with no document query.

Membership uses this same relation for array elements; string-in-dictionary
tests key existence and string-in-string tests substring presence. NotIn negates
these supported combinations. Unsupported combinations use the canonical
quoted-operator diagnostic. Do not modify public PartialEq/Hash, morph_canon or
the content IR to achieve language equality.
