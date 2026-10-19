# L0 — stdlib/layout/place — place and flush
Hash do Código: 7eda64f2

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/place-coordinate-origin.toml sha256:b9012d07e2d22b7c3984963a6d4e90eedbb9b925280f4136f8db8aaa9459bbc8

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/layout/place.rs`

## Evidence before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-layout-2026-09-30-before.json.gz` contains this family's logic in `01_core/src/compiler/stdlib/layout.rs`; the complete L1 suite passed with 6401 tests and 2 ignored before extraction. This is structural measurement, not a benchmark or new bilateral vanilla parity evidence. Historical decisions, measurements and pin qualifications remain in `00_nucleo/prompts/core/compiler/stdlib/layout.md`.

## Language contract

place preserves alignment, dx/dy, scope, float and relative clearance default 1.5em. flush rejects every argument and produces Content::flush without layout.

Extraction preserves signatures, function identity, syntax, morphology, errors and validation precedence. The hub and registry continue to forward calls statically without duplicating algorithms. Preserve public contracts, default behavior and pipeline phase.

## Acceptance

The L1 suite and `crystalline-lint .` must remain regression-free. Parity is at the language level under ADR-0107/0108; mechanics may diverge. Preserve exclusive ownership under ADR-0129/0130.

## Preserve vertical omission

### Evidence before decision

compiler/stdlib/layout/place.rs:42–48 converts omission into Some(Top).
`lab/results/construction/diagnostics/place-origem-local-2026-10-14/{before,extra-before}/results.json.gz`
retains HEAD, dirty tree/diffstat, UTC, binaries and SVGs against historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`: after text A, omitted alignment anchors
at baseline while explicit Top anchors in the region. Historical upstream
flow/distribute.rs:703–715 distinguishes explicit axis from flow offset. This is
language geometry, not Rust equality. The tests
place_origin_pending_line_and_explicit_top_are_distinct reproduce the divergence
before correction.

### Contract and acceptance

Use the existing Option to preserve v=None for omitted vertical axis; retain
explicit alignment. Add no field or signature. ADR-0127 internal parity flow
introduces no mode/flag/phase. Layout chooses the anchor under the pinned
place-coordinate-origin nucleus. Preserve clearance/scope/validation/floats and
validate pending-line, explicit-Top, geometry, float and full-suite controls.
The separate anchor-after-committed-flow work remains open in the local-origin
diagnostic; this does not close it.

## Exhaustive argument decisions and measured rejection

### Evidence before decision

At baseline `91ea6ee91105b97b1aedf172e34780dddde14c94`, the exact source/L0
hashes and UTC are retained in
`lab/results/v16-corrections-2026-10-19/batch-f/agents/geometry/initial-audit-state.json`.
The baseline eval/query receipts in that directory bind the rebuilt product
SHA to the ratified oracle commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`.
They measure acceptance, diagnostic meaning, supplied-value provenance and
geometry, not Rust representation or rendered bytes. The initial audit separates
source evidence from inferred intent and records refuters.

`layout/place.rs:34-51` converts every unsupported supplied dx/dy to zero.
Pinned `layout/place.rs:163,169` defines relative lengths; baseline Boolean, None,
Auto, String, Fraction and Array are errors upstream while succeeding locally.
The public PlaceElem stores f64 offsets, so full em/relative support cannot be
claimed from this local correction.

### Contract and limits

Make the private point extractor fallible and exhaustive over Value. Float/Int
retain the deliberate point extension; Length retains its current absolute
projection. Ratio/Relative retain their measured zero projection as explicit RM1
debt, not final dimensional semantics. All other variants report
`expected relative length, found {vanilla type}` instead of becoming zero.
Omitted dx/dy still mean zero, distinct from invalid supplied data. Validate dx
then dy at their existing sites before scope/float/clearance/body. Use last named
occurrence value_span then occurrence.span then Args.span. Preserve alignment
omission, scope rules, callbacks, clearance and every native/public signature.

Tests cover invalid types, zero/negative/numeric point compatibility, omitted
vertical alignment, causal span and dx-before-dy precedence. Preserve explicitly
measured dimensional limitations; do not add resolution in eval or change a
public carrier. This is continuous internal contract repair under ADR-0127.
