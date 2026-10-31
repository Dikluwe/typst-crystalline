# L0 — compiler/stdlib/structural/math/cancel
Hash do Código: 3cbcb4e7

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/stdlib/structural/math/cancel.rs`
**Governance:** ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129, ADR-0130.

## Measurement before decision

The uncommitted working tree captured in `lab/results/construction/diagnostics/atomizacao-stdlib-math-cancel-2026-09-30-before.json.gz` places the complete `native_cancel` body in `01_core/src/compiler/stdlib/structural/math.rs:395-529`. Static `math.cancel` registration is in that parent and the public native reexport is in `structural/mod.rs`. The `native_cancel` baseline filter passed 4 tests. This is an internal structure baseline, not a new bilateral vanilla measurement or runtime benchmark.

## Language contract

Own the native `math.cancel(body, length: 100% + 0.3em, inverted: false, cross: false, angle: auto, stroke: auto, background: false)` cast and semantic carrier construction. One positional body is required; accept Content or String converted to text. Preserve the current validation order, spans and exact diagnostics for missing, invalid and excessive positional arguments, invalid named values and unknown named arguments.

`length` accepts Relative or Length, converting Length into an absolute relative length. `inverted`, `cross` and `background` require Bool. `angle` accepts Auto, Angle or Func. `stroke` accepts None or a value handled by the existing `extract_stroke` cast. Preserve the current defaults and order of named validation. Set the corresponding `MathCancelExplicit` bit whenever a named argument is present, including one equal to its default. Construct `Content::math_cancel_full` with the argument span and unchanged semantic fields. Do not measure the body, execute an angle callback, perform layout or introduce a second runtime.

The parent `math.rs` owns only the static `math.cancel` module registration and its other mathematical behaviors. The structural hub directly reexports `native_cancel` from this child, preserving the public path and signature. The payload remains owned by `00_nucleo/prompts/core/entities/elements/math_cancel.md` and geometry by `00_nucleo/prompts/core/compiler/math/layout/cancel.md`. No public signature, default, pipeline phase or compatibility change is authorized.

## Acceptance

Existing `native_cancel` regression tests and a local test for explicit named presence remain green. The full core suite, `cargo build` and `crystalline-lint .` remain green. Header ownership is 1:1. No speedup is claimed without separate measurement.


## R4 — measured public mathematical projection / qualified conversion

### Measurement before decision

`lab/results/math-fields-r4-2026-10-23/baseline.json` and `before/receipt.json`
retain UTC, HEAD, complete uncommitted diff stat, source map, archived sources
and binary SHA-256. The oracle is pinned bd5ebec285505889e8ff1d4f68804ab147a699f3
(tree cac836c07e4dec2a89219370428292393a3216b5). The before probe records
fields/has/at and accepted/rejected constructor values rather than inferring
intent from implementation. Upstream math/matrix.rs:45–67,238–272 and
math/mod.rs:227–234 declare the already existing language fields. Local
content_methods.rs:151–158,378–380 omits these projections; frac.rs:43–53
and root.rs:27–38 reject supported operands. These are language semantics
and morphology, distinct from byte or Rust algorithm parity.

### Exclusive correction and gate

Use casts::math_lr_content_value for body; preserve all named flags and validation.

This R4 clause supersedes only conflicting historical operand/field clauses
above. Internal parity correction follows continuous ADR-0127: L0 first,
reseal, actual functional RED before implementation, GREEN and full validation.
No new public Rust field/signature/trait method, default mode or phase change.
Preserve other semantics and all received changes. Record source-bound builds,
full workspace tests/build/fmt/Clippy/Rustdoc, lineage and full lint zero; replay
A–H and R1–R3 without reclassifying their existing limitations as parity.


## Typed mathematical inputs — measured 2026-10-29

### Measurement before decision

lab/results/construction/diagnostics/readiness-fixes-2026-10-28/
math-input-findings-before-decision.json binds actual build3/source, pinned
oracle64c5e8c0, UTC, HEAD/dirty stat, exact sources and binary hashes. The fifteen
math-input-values-before cases isolate Ratio, Code local/loop/Func and escaped
accent failures with Length/Relative/literal/Math local/Content and Bool controls.
math-escape-callback-values-before uses a CONTENT-returning callback: an escaped
argument has type content versus native symbol; String stays str. Its markup
escape function is text versus symbol, while literal A stays text. The earlier
raw-Type-return callback has an independent return-display failure and does not
authorize changing callback return policy. All affected owners remain identical
to the archived actual build3.

### Exclusive correction and acceptance

Add Ratio to the existing length cast, using its actual relative value and
absolute Length::ZERO. Pinned math/cancel.rs35 declares Rel<Length>; layout/rel.rs
187-190 defines Ratio conversion with zero absolute part. This supersedes only
the historical Relative-or-Length admission above. Keep explicit.length true,
existing default, Length/Relative casts, Bool rejection, validation order,
argument spans, angle/cross/stroke/background and semantic carrier unchanged.
Do not execute callbacks or measure layout in the constructor.

The co-located direct constructor test checks zero/150/200-percent relative
values, zero absolute part and explicit presence; bilateral Length/Relative and
invalid Bool controls remain binding. Add no public helper or contract.

Internal measured parity correction follows ADR-0127 continuous flow: L0 before
actual functional RED, implementation only after RED, GREEN and source-bound
full tests/build/fmt/Clippy/Rustdoc/lint/lineage. Preserve all earlier positive
visual/history and JPEG criteria; no normalization, suppression or limit change.


## Explicit Code calls / rich cancel stroke — 2026-10-29

### Measurement before decision

lab/results/construction/diagnostics/readiness-fixes-2026-10-28/ retains
math-code-stroke-before/receipt.json (fifteen bilateral cases) and
math-code-namespace-before/receipt.json (eight boundary cases), with actual
immutable build5/source, HEAD/dirty stat, UTC, fixture and executable hashes.
The oracle is64c5e8c02fe0394d913ead36e42ef8c1334d2107/tree483bd6f1d8ac8810db47b65336d402647955fd17.
Actual AST evidence equation-scene2/{receipt.json,code-versus-math-call-roots.log}
uses the existing public API and build5 library: embedded Code calls have Ident
roots, including field chains; ordinary math calls have MathIdent roots.
The diagnostic's empty-font World is not render or font evidence.

### Exclusive correction and acceptance

For stroke Dict only, reuse the existing layout::positional_stroke conversion
with the actual call span. Preserve the dictionary's paint, thickness, dash and
other already supported Stroke fields; do not create a second stroke parser or
change the shared converter. Keep all other current cancel stroke paths and
validation order, including the existing None extension. The native oracle
rejects None; changing that extension would require a distinct compatibility
decision and is outside this repair. Preserve existing Length/Color/Stroke
behavior, defaults, flags, geometry and callbacks.

Co-located constructor tests verify Dict paint/thickness/dash and presence,
invalid paint rejection and preservation of legacy None. The public docs cancel
currently fails at Dict after the Ratio repair; this step does not certify full
CSL/math paint, arbitrary strokes, or unrelated named casts.

Internal ADR-0127 continuous correction: L0 before tests and actual RED, then
implementation, GREEN/full tests/lint/build and all original preservation gates.
No public API, signature, mode, default or phase is introduced.


## Issue 8894 — approved Stroke migration

Measurement: the source inventory and previous owner bytes in
`lab/results/construction/diagnostics/issue-8894-2026-10-31/` bind exact
HEAD/dirty diffstat/UTC and each affected source/owner before migration.
`stdlib/layout/stroke.rs:430–434` erases em; geometry.rs:81 carries only f64.
The explicit user approval in approval.json ratifies the public Length field
and occurrence-layout resolution, as specified by
`00_nucleo/prompts/core/entities/geometry.md`. This consumer retains its
exclusive responsibility and all other ratified clauses and controls.

Migrate this consumer's original point-only Stroke test values/assignments
to Length::pt while preserving exact numeric values and all assertions. When
a prepared scalar is compared to its declarative source, explicitly resolve
that source with the fixture font; do not relax expected geometry or tolerance.
This test migration implements only the approved field representation change;
existing non-stroke obligations and prior approvals remain intact.
