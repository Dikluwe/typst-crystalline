# L0 — compiler/eval/bindings/field_access/callee_error
Hash do Código: a39f4be1

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/bindings/field_access/callee_error.rs`. At HEAD `ba5a4c7d305ed1444660d9eb80862f51b417efb6` with a clean tree, `01_core/src/compiler/eval/bindings/field_access.rs:1052-1164` contains the extracted behavior. SHA-256, time and initial state are recorded in `lab/results/construction/diagnostics/atomizacao-field-access-2026-09-30-before.json.gz`.

## Contract

Construct invalid field-call, dictionary-key and missing-method diagnostics. Preserve signatures and semantics from `00_nucleo/prompts/core/compiler/eval/bindings/field_access.md`; dispatch stays static in L1, without I/O or mutable global state.

## Acceptance

Preserve the L1 suite and field-access tests; `crystalline-lint .` must pass with correct ownership and hashes. Do not infer performance gains.


## Nominal Value name projection

### Evidence before decision

`01_core/src/compiler/eval/bindings/field_access/callee_error.rs:18`
(element_or_type_with_name, emitted-v16) is the selected projection. The current
Value enum at entities/value.rs:59–195 has 39 materialized variants; current
payload/catalog data, not the historical future-variant table, govern the audit.
`lab/results/construction/diagnostics/tekt-v16-value-projections-2026-10-16/before.json.gz`
retains HEAD, uncommitted working tree, UTC, status/diff HEAD --stat, hashes and
original owners/consumers. contexts.json comes from the current AST checked
against pinned SARIF; classification.json distinguishes nine emitted V16 from
eight latent projections. Rust form/exhaustiveness is mechanics; names,
conversions, borrows and language observables retain the existing contract.
This is not new parity or performance evidence.

### Exclusive responsibility

Keep Content and LocatedContent as element with their original elem_name.
Explicitly name the other 37 Value alternatives as a shared group returning
type through the same vanilla_type_name(value). Preserve field_callee_error's
diagnostics/hints/spans and the error_formatting owner's table. Name projection
must not lose entity identity/snapshot. C5 controls cover Content/LocatedContent
with a heading and legitimate Location, boolean/integer/string and generic
none/auto/float/array names; preserve this route's long names.

Alternatives sharing a result share one body. A growing Value enum requires a
new nominal decision; do not duplicate bodies to remove V19. V19 is informational:
it counts written alternatives, not executed tests, reachable combinations or
an N× coverage deficit. Choose positive/negative/boundary controls by behavior;
executing one arm does not prove every alternative. No allow/expect, severity
relaxation or exception may hide V16.

### Acceptance and limits

ADR-0127 continuous flow adds no public signature/trait/field, productive variant,
product default, compatibility break or phase change. L0 first, restricted
bilateral resealing and valid V1/V5/V15/V26. Write local controls and verify before/
after behavior. Structural RED is the absence of E0004 at the 17 selected points
in a core copy with a sentinel added only to copied Value. GREEN requires E0004
at all 17 without stubs and exactly the nine selected V16 removed, preserving
other diagnostics. Validate build/workspace suite, strict Clippy, fmt, Rustdoc
and lineage under 12 GiB, zero swap and one Cargo job. The global gate stays open
while external V16 remain. The parser false positive for explicit None is a
separate investigation and must not be hidden here.
