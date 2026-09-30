# L0 — Signed time interval
Hash do Código: 530656de

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/duration.rs`

## Evidence before specification

Historical upstream `e0e8ca4d` explicitly permits positive and negative spans
at `crates/typst-library/src/foundations/duration.rs:9-12`; constructor evidence
at `duration.rs:38-42` distinguishes an interval from a datetime and names weeks,
days, hours, minutes and seconds. The signed observable is language semantics;
the i128 representation is mechanics. These are not current-pin measurements.

## Owned public domain

`Duration` has public `nanos: i128`, deriving
Debug/Clone/Copy/PartialEq/Eq/PartialOrd/Ord/Hash. Negative intervals are
representable, comparable and printable. Default is ZERO.
Preserve ZERO, SECOND, MINUTE, HOUR and DAY, in nanoseconds: 0, 1_000_000_000,
60_000_000_000, 3_600_000_000_000 and 86_400_000_000_000 respectively.
`from_nanos(i128)` and `from_seconds/from_minutes/from_hours/from_days(i64)`
construct signed intervals. `as_seconds/as_minutes/as_hours/as_days(&self)`
return i64 whole-unit totals; `is_zero` tests zero. Existing domain signatures
remain intact. Neg maps to a signed Duration; no allocation or I/O is required
for storage. Constructor overflow handling outside the previously declared
real-value scope remains unresolved, not newly guaranteed by this cleanup.
The domain `to_string` preserves compact canonical formatting such as
`3d2h30m15.500s`, its negative form and `0s`; it is not language repr.

## Exclusive ownership

Value transport, casts and binary arithmetic are owned by
`00_nucleo/prompts/entities/value.md`; public type identity and dispatch by
eval owners; named duration repr by `00_nucleo/prompts/compiler/eval/repr.md`.
The previous cast/arithmetic/field clauses are handoff constraints retained in
the diagnostic, not requirements re-owned by this entity. String parsing remains
outside the declared domain scope. Preserve negative-duration semantics and
public signatures/defaults without introducing a new datetime or clock policy.
