# L0 — entities/color/adjustments
Hash do Código: 126b766f

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/color/adjustments.rs`

## Evidence before specification

At clean HEAD `ea7ae25ab` (2026-09-30), behavior was in
`entities/color.rs:325-390,530-608,619-666`. This bounds ownership, without a
new performance measurement. The later V16 observation at
`01_core/src/entities/color/adjustments.rs:141` read and hash-checked source/L0.
`lab/results/construction/diagnostics/tekt-enums-2026-10-16/before.json.gz`
preserves HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree,
UTC, sources/L0/configuration, status and git diff HEAD --stat. Its SARIF has
11 warnings and one note across seven owners; unreported fallbacks were also
measured and belong to the scope. Rust patterns/enum evolution are mechanics;
observable results, fields, identity and errors must remain unchanged. Infer
neither upstream intent nor an AI benefit.

## Contract

Own lightening, darkening, negation, rotation, saturation and alpha adjustment
in the color's space. Preserve formulas, Spot semantics, Option failures,
public signatures, defaults, phase, compatibility and L1 purity.

For darken, the arm delegating to lighten(-factor) explicitly names
Srgb/Luma/LinearRgb/Oklab/Oklch/Hsl/Hsv; Spot and Cmyk retain priority bodies.
For rotate, unreachable names Srgb/Luma/LinearRgb/Oklab/Cmyk/Spot, preserving
the message and three valid hue arms. For saturate, the Hsv conversion names
Srgb/LinearRgb/Oklab/Oklch/Cmyk; Luma/Spot remain None and Hsl/Hsv remain in
their own spaces. Only patterns change: calls, formulas, alpha, clamp and
error handling remain intact.

## Acceptance and unresolved scope

This is ADR-0127 internal continuous maintenance. No enum definition, field,
trait, public signature, default, dependency, compatibility or phase changes.
Preserve bodies/order/guards; do not move logic between owners or hide a
catchall behind a helper. Nominal patterns must make future variants fail
compilation; neutrality comments are not suppressions.

Require local functional tests and scope-specific V16 RED before code. In a
disposable workspace add one sentinel variant to Color, ColorSpace,
MathStyleKind, FuncRepr and StateUpdate, never to production definitions.
Run cargo check --offline --locked -p typst-core with one job/shared artifact
cache, recording sources/hashes/argv/UTC/diagnostics. For each of 21 selected
matches, baseline must lack E0004 there and GREEN must produce it there,
independent of other matches' existing errors. Sentinel-free compilation and
behavior tests validate real code; the negative probe does not prove parity.
Do not edit production source during commands.

Restrict forward/reverse resealing rather than globally repairing the working
tree. Require L1/full workspace tests, build, Clippy workspace/all-targets
with -D warnings and -D unfulfilled_lint_expectations, formatting, Rustdoc
-D warnings and four HTML indexes, strict/reverse V1/V5/V15/V26 lineage,
SARIF without new warnings and diff --check. MemoryMax=12 GiB,
MemorySwapMax=0, CARGO_BUILD_JOBS=1; heavy jobs sequential. No historical
fixture/tolerance relaxation or inferred performance gain. External V16 debt,
including unreported T027/T158/T160, remains open.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/color/adjustments.md`. Historical observations are not a new parity certification.
