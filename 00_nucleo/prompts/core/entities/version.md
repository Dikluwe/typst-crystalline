# L0 — Version integer components
Hash do Código: a871fde3

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/version.rs`
**Governance:** ADR-0017, ADR-0029, ADR-0107, ADR-0108, ADR-0127.

## Evidence before specification

The historical direct probe at upstream `969087ec` established an arbitrary
sequence of nonnegative integer components, not SemVer text with prerelease or
build metadata. It accepted zero, one, three, or five components; rejected
`pre: "alpha"` with `unexpected argument: pre` and a string component with
`expected integer or array, found string`; equated `(1,2,3)` with `(1,2,3,0)`;
and preserved trailing zeros in repr.

The historical upstream `a51e02804` source at
`lab/typst-original/crates/typst-library/src/foundations/version.rs:109-134`
and the clean baseline `2f42d64253547734564513a1159ee6b584c1c4b4` at
`01_core/src/entities/version.rs:103-120` support explicit-list negative
indexing and positive zero extension. The archived static/bound probes retain
full source, time, command and output provenance. Upstream
`foundations/value.rs::Value::display` (approximately line 200 in the recorded
snapshot) uses Display for markup, not repr.

Historical public-constructor probes at `a51e02804` in
`lab/results/construction/diagnostics/superficie-linguagem-p1140*` distinguish
`type(version) == type` from the former function identity. The prior structural
extraction preserved its catalog and 22 probes; that observation is historical,
not certification against the current pin.

The single-source version discrepancy was measured at HEAD
`781b207b4a5de9c2bfbe5819918a193d1d9293e5` with an uncommitted working tree;
commands/binaries are recorded in
`lab/results/construction/diagnostics/paridade-matriz-p1137-baseline.md`.
Later source resync receipts preserve `586e1bd43` and
`9f2b6e8715237cb086899a42873660fe744622e8` observations in the archive.
The current pin `bd5ebec285505889e8ff1d4f68804ab147a699f3` also reports
`version(0, 15, 1)`, as recorded in
`lab/results/construction/diagnostics/vanilla-resync-2026-10-17/relatorio.md`.
A CLI version string supports the value check but does not prove binary identity.

Default derivation was justified by the same empty-components value and no
new generic bounds at HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, with
uncommitted tree/source/UTC/diffstat in
`lab/results/construction/diagnostics/expressoes-defaults-ci-2026-10-13/before.json.gz`.
Parser and formatting interface evidence is preserved in
`lab/results/construction/diagnostics/clippy-traits-formatacao-2026-10-16/before.json.gz`
and the original formatting proposal. These are Rust-interface measurements,
not new language-parity or performance measurements.

## Domain contract

```rust
#[derive(Debug, Clone, Default)]
pub struct Version {
    pub components: Vec<u64>,
}

impl Version {
    pub fn new(major: u64, minor: u64, patch: u64) -> Self;
    pub fn from_components(components: Vec<u64>) -> Self;
    pub fn component(&self, i: usize) -> u64;
    pub fn major(&self) -> u64;
    pub fn minor(&self) -> u64;
    pub fn patch(&self) -> u64;
    pub fn from_str(s: &str) -> Option<Self>;
    pub fn at(&self, index: i64) -> Result<i64, String>;
}
```

Store components exactly as supplied, including trailing zeros. Clone is not
Copy. Derived Default produces an empty vector, preserving the former manual
Default's value. Remain pure, with no I/O. `component(i)` returns zero beyond
the explicit vector; major/minor/patch read indices 0/1/2.

Manual PartialEq/Eq/Ord compare component-wise over the larger length with
zero extension. Trailing zeros never change equality or order. Manual Hash
ignores trailing zeros and remains consistent with equality. The old phrase
"shorter prefix < longer" applies only when a subsequent component is
nonzero; it must not override the explicit zero-padding rule.

`from_str` trims/parses u64 components separated by dots, returns Option, and
rejects empty input, non-integers, negatives, prerelease and build strings:
`"1.2.3-alpha.1"` and `"1.2.3+build.2"` yield None. Do not add FromStr, an Err
type, implicit trailing-zero normalization, or different rejection behavior.

## Indexing

`Version::at` is the sole semantic query shared by static and bound calls.
A nonnegative index reads the zero-extended sequence, including zero for an
empty version. A negative index counts from `components.len()`, including
explicit trailing zeros, never from an infinite zero extension. Resolve with
checked arithmetic; an invalid negative index, including `i64::MIN`, fails
without overflow or panic and retains the original index in the error:

```text
component index out of bounds (index: {index}, len: {len})
```

Here len is the explicit component count. The entity receives an already
validated i64, never Args/AST, context, World, or call spans. Language discovery,
shared static/bound argument parsing, messages, hints, spans, and diagnostic
priority belong to
`00_nucleo/prompts/core/compiler/stdlib/primitives_constructors/version.md`.
This supersedes the old bindings/closures dispatch plan and, specifically for
`at`, the old exemption from exact argument-error parity. Preserve no named
arguments (including no `default:`), integer positional validation and arity;
the old examples `expected integer, found …` and
`version.at() requires exactly one positional argument` are subject to the
current exact-observable requirement, not an exception to it.

Presence, lookup, and diagnostics are language observables; internal call
structure is mechanics. The inference that the existing query suffices is
refuted if a valid language call requires different `Version::at` semantics.

## Formatting and ratified Rust interface

Implement std::fmt::Display. The former inherent public `to_string` is replaced
by a private `canonical_string` helper; Display writes that string with
`f.write_str`, and ToString comes from Rust's blanket implementation. Preserve
explicit components, order, trailing zeros, and empty output. Formatter width
and precision do not redefine the canonical string. No FromStr is added.

The human ratified this public interface on 2026-10-16, manifesto SHA-256
`d3fba43d799461d294ad562c37649ba9c2e4da2a45d7b7099953624ced28e7e8`;
receipt:
`lab/results/construction/diagnostics/clippy-enums-2026-10-16/approval-receipt.json`.
Approval includes implementation and the original T1–T5 checks, with no release.
It supersedes the earlier note that Display remained a separate unapproved
proposal. Generic code may now rely on Display/ToString; rollback can break
that code. Preserve all other signatures, fields, default behavior, pipeline
phase, dependencies, and suppressions.

The independent Option parser retains only this named-method expectation:

```rust
#[expect(clippy::should_implement_trait, reason="S004: parser de componentes Option preservado; FromStr exige Result e um contrato de erro distinto")]
```

The Portuguese reason is an exact source metadata literal. Do not suppress
an entire module/impl/crate, raise a threshold, or extend this exception to
Styles::from_iter. Preserve parser bodies and functional expectations; missing
expectations are errors. The later Display approval does not authorize FromStr.

## Language boundary obligations

Value transport belongs to `00_nucleo/prompts/core/entities/value.md`:
`Value::Version(Arc<Version>)`, cheap Arc clone, type_name `"version"`,
`From<Version> for Value`, and casts extracting Arc from Version or parsing Str
with `Version::from_str`; other types return None.

Global `version` is `Value::Type(Type::Version)`, callable through static
dispatch to the existing constructor. `type(version) == type` and
`repr(type(version)) == "type"`; constructed values remain type `"version"`.
This is an integration requirement, not a second implementation in this entity.

Repr preserves supplied components as `version(0, 11, 0)`,
`version(1, 2, 3, 4, 5)`, or `version()`. Markup instead uses Display's
`0.11.0`, `1.2.3.4.5`, or empty string; empty display produces None rather
than text content. That branch precedes the evaluator's repr catch-all.
Repr belongs to `00_nucleo/prompts/core/compiler/eval/repr.md`; markup display
to `00_nucleo/prompts/core/compiler/eval/mod.md`.

Only major/minor/patch are named component fields, with zero for absent
components. The earlier `.pre`/`.build` diagnostic literal
`campo desconhecido em version` remains a recorded language-boundary
obligation; the known absent-named-field discrepancy from the static/bound
query review is still pending. Completion of indexing did not close that gap.

## Shared language version

Keep the single public constant
`PARITY_VERSION: (u64, u64, u64) = (0, 15, 1)`.
`00_nucleo/prompts/core/compiler/stdlib/sys.md` consumes it for sys.version;
`00_nucleo/prompts/shell/cli.md` consumes it for CLI --version. Do not duplicate
it or substitute the Cargo package version.

The value follows the explicitly ratified upstream language, never a moving
main branch, network latest, or textual tag automatically. Changing the pin
requires an explicit resync and fresh sys.version measurement; a public value
change follows ADR-0127. The historical correction from 0.15.0 to 0.15.1
intentionally changed repr/markup/comparisons and rebuilt CLI output without
changing a public field, type, or signature. A directly verified ratified
binary returning another sys.version would refute the pinned-value decision;
an unrelated binary's nominal version would not.

## Acceptance and limits

- Exercise empty and arbitrary-length construction; component access; preserved
  trailing zeros; zero-padded equality/order; and hash consistency. Verify parser
  acceptance/rejection and unchanged Option API.
- Exercise static/bound at, valid negative indexing, positive extension, empty
  versions, invalid negatives, `i64::MIN`, and exact original-index diagnostics.
  Preserve constructor, named members, repr/display, casts, and the constant as
  controls; the old "nothing pending" note applies only to its indexing/display
  fragment and is not general language-parity closure.
- Require `sys.version == version(0, 15, 1)`, its repr and markup display, plus
  CLI propagation from the single constant. The historical RED/GREEN value
  correction is retained evidence, not a new test execution claimed here.
- Ratified formatting acceptance: generic Display+ToString usage must fail
  before the trait exists and pass after; Version displays empty and `1.2.0`
  without losing components. Preserve the cross-owner T1–T5 controls:
  Decimal `1.2300`, negative/zero and Option rejections; Duration MIN/MAX/MIN+1,
  zero and negative-boundary corrected text via format!/ToString; Styles collect
  empty and Weight(400),Weight(700) yielding weight Some(700), other fields
  unchanged, with the inherent adapter retaining existing calls. These controls
  stay with their owners and do not authorize implementing those units here.
- Preserve all existing test expectations; validate the language/cast/repr/show/
  CLI suite, complete workspace targets and test inventory, core suite, build,
  Clippy (including unfulfilled_lint_expectations), Rustdoc, formatting, and both
  lineage directions. Use one job, 12 GiB maximum, and no swap; no release.

No new parity or performance claim follows from this editorial consolidation.
The absent-named-field discrepancy remains open; historical snapshots retain
only their originally measured scope.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/version.md`. Historical observations are not a new parity certification.
