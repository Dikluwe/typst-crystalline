# L0 — compiler/eval/cast
Hash do Código: b892829e

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/cast.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/cast.rs:18` — `pub enum CastError {`.
- `01_core/src/compiler/eval/cast.rs:26` — `pub fn cast_length`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `lab/typst-original/crates/typst-library/src/layout/rel.rs:11-25`.

## Contract

Own CastError and cast_length(Value) -> Result<Length, CastError>.
Value::Length succeeds unchanged. Value::Relative returns NeedsContext rather
than resolving a reference length implicitly. Other variants return TypeMismatch
with the existing public type information. Preserve both distinct failure kinds.

Relative resolution requires an explicit reference/context under its own owner;
this pure cast does not obtain it from layout, World or globals. It does not
change evaluation phase or claim that every relative value is intrinsically
uncastable when the proper context is supplied elsewhere.
