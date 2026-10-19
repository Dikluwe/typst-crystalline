# L0 — Signed time interval
Hash do Código: a11f7236

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
`00_nucleo/prompts/core/entities/value.md`; public type identity and dispatch by
eval owners; named duration repr by `00_nucleo/prompts/core/compiler/eval/repr.md`.
The previous cast/arithmetic/field clauses are handoff constraints retained in
the diagnostic, not requirements re-owned by this entity. String parsing remains
outside the declared domain scope. Preserve negative-duration semantics and
public signatures/defaults without introducing a new datetime or clock policy.

## Full-range canonical formatting

The isolated historical probe used the unchanged consumer with rustc 1.92 in
debug and -O. `lab/results/construction/diagnostics/clippy-traits-formatacao-2026-10-16/duration-before-probe.json`
records i128::MIN panicking from overflow in debug and producing `--55s` in
release; MIN+1/MAX, zero and -1s are controls. The package's `before.json.gz`
retains source/L0 hashes, HEAD, uncommitted tree, UTC and diffstat;
`probe-result.json` records commands and RAM limits. `duration.rs:103` used
`self.nanos.abs()`, whose positive MIN magnitude cannot fit i128. This is a
formatting defect in an already representable value, not a new numeric domain
or saturation policy. Expected MIN derives from exact integer decomposition
of 2^127, never f64 or the defective release output.

Use unsigned_abs to obtain u128 magnitude and positive u128 unit divisors.
Preserve decomposition, part order, zeros, fractional-zero trimming and exactly
one sign. MIN formats as `-1969226660422097589487121d13h41m55.884105728s`;
MAX as `1969226660422097589487121d13h41m55.884105727s`; MIN+1 is negative MAX.
Preserve -1ns, -1s, unit boundaries, zero and all earlier examples. Constructors,
fields, as_*, Neg, arithmetic overflow policy and language repr remain unchanged.
Do not copy another compiler's defect. Require real extreme-value RED before
implementation, GREEN afterward, and debug/release probes with controls.
The internal formula correction follows ADR-0127 continuous flow, without a
new API, default or phase, and does not claim performance/global parity.

## Ratified Rust formatting interface

The historical inventory measured should_implement_trait on Styles.from_iter
and inherent_to_string on three types. Four implementations and L0 owners were
read; `base.json.gz` in the same package records source/hashes/HEAD/tree/UTC/diffstat.
Two Option parsers and two domain next methods were classified separately and
are outside trait conversion. This is a Rust contract change, not new Typst
language behavior or measured parity.

Implement std::fmt::Display for Duration; remove the public inherent to_string.
Its corrected u128 body becomes private canonical_string; Display writes it
through f.write_str. ToString uses the standard blanket. Preserve all MIN/MAX,
zero/sign/fraction/unit behavior and keep repr, Neg, overflow and as_* unchanged.
Formatter precision/width does not redefine this canonical string. No FromStr,
new fields/variants, flags, defaults, phase, dependencies or suppressions.
Existing to_string documentation now refers to the ToString interface.
Generic callers can depend on these traits, so rollback can break compatibility.

The human approved the three Display implementations, one FromIterator and
migration of three inherent methods on 2026-10-16, manifest SHA-256
`d3fba43d799461d294ad562c37649ba9c2e4da2a45d7b7099953624ced28e7e8`;
receipt: `lab/results/construction/diagnostics/clippy-enums-2026-10-16/approval-receipt.json`.
This ratification supersedes the earlier pending Display proposal and includes
implementation and T1–T5, without a release.

## Acceptance

- T1: generic Display+ToString and collect::<Styles> fail for missing traits
  before implementation and pass afterward.
- T2: Decimal displays 1.2300 without dropping zeros, preserves negative/zero
  and Option rejections; Version preserves empty and 1.2.0 components.
- T3: Duration preserves corrected MIN/MAX/MIN+1, zero and negative boundaries
  through format! and ToString.
- T4: empty Styles collection is empty; Weight(400), Weight(700) yields
  weight Some(700), other fields unchanged; the inherent adapter preserves calls.
- T5: existing language/casts/repr/show/CLI tests, full workspace, build,
  Clippy/expectations, Rustdoc, formatting and forward/reverse lineage pass
  without relaxed expectations. Restricted resealing; maximum 12 GiB RAM,
  zero swap and one job. No inherent_to_string suppression and no release.

## Historical record

Original clauses and chronology: `lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/duration.md`.
