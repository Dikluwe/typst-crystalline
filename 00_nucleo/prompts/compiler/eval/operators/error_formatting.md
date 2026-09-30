# L0 — compiler/eval/operators/error_formatting
Hash do Código: d5acbd63

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/eval/operators/error_formatting.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0127, ADR-0129, ADR-0130.

## Source evidence

The current exclusive consumer defines the following owned boundaries:

- `01_core/src/compiler/eval/operators/error_formatting.rs:17` — `fn binary_mismatch`.
- `01_core/src/compiler/eval/operators/error_formatting.rs:28` — `fn unary_mismatch`.
- `01_core/src/compiler/eval/operators/error_formatting.rs:53` — `fn vanilla_type_name`.

Prior source readings supporting these clauses (snapshot provenance is
retained in the assigned history diagnostic, not asserted as a fresh measurement):

- `foundations/ops.rs:170,214,284,340,500`.
- `foundations/ops.rs:54-68,85-86,376`.

## Contract

Own binary_mismatch, unary_mismatch and the exhaustive vanilla_type_name map.
Use public long type names, never Rust variant/Debug names. In particular Bool
is boolean, Int integer, Str string, Dict dictionary, Relative relative length,
Args arguments, and raw/located Content both content. Preserve every other
current variant mapping, including Type -> type and SpotColorant -> spot colorant.
Other long-type-name helpers delegate to this canonical map rather than defining
a competing catalog.

Binary formats are `cannot add {lhs} and {rhs}`,
`cannot subtract {rhs} from {lhs}`, `cannot multiply {lhs} with {rhs}`,
`cannot divide {lhs} by {rhs}`; other operators use
`cannot apply '{operator}' to {lhs} and {rhs}` (including 'in' and 'not in').
For unary Pos on Symbol/Str/Bytes/Content/LocatedContent/Array/Dict/Datetime use
`cannot apply unary '+' to {type}`; Neg on Datetime uses
`cannot apply unary '-' to {type}`. Other failures use
`cannot apply '{operator}' to {type}`. Preserve quoting and operand order.

Ordering's distinction between unsupported types and recognized-but-undefined
comparisons belongs to ordering, not this formatter. This owner does not dispatch
operators, choose spans, inspect scopes or alter values.
