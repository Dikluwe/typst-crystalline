# L0 — entities/color/mixing
Hash do Código: e0449398

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/color/mixing.rs`

## Evidence before contract

At clean HEAD ea7ae25ab (2026-09-30), behavior was in entities/color.rs:391-529.
This bounds ownership, not a new performance measurement. Clippy at the
consumer's line 14 was checked against source before the internal rename;
`lab/results/construction/diagnostics/iteracao-parametros-ci-2026-10-13/before.json.gz`
retains source/owners/spans/references/binary/UTC/HEAD
06848cdcb5450f2c555626bf116bc7acd82bd82b, uncommitted tree/status/diffstat.

The later V16 sites are lines 71 and 116. Source/owner hashes were checked;
`lab/results/construction/diagnostics/tekt-enums-2026-10-16/before.json.gz`
preserves that HEAD/tree/UTC/source/L0/configuration/status/diffstat. Its SARIF
records 11 warnings and one note across seven owners, including separately
measured unreported fallbacks. Pattern/enum mechanics may change; observable
results/fields/identity/errors must not. No upstream-intent or AI-gain inference.

## Contract

Convert components to vectors in the requested space and mix two or N colors,
preserving circular hue paths, weights and Spot identity. Preserve public
signatures, defaults, pipeline phase, compatibility and L1 purity.

The private self-consuming helper is into_vec4_in_space; update only its
same-owner calls. Preserve consumption, clones, fallback, conversions and
weights. Keep mix/mix_weighted public signatures and helper privacy. No new
formula or colorimetric-parity claim follows from renaming. Preserve existing
test expectations; do not add loop-mirroring tests or lint suppressions.
Require core tests (including conflicts/spans/fixups/markers/differential
selection), all-targets, build, docs, Clippy and restricted lineage reseal.

Both hue_idx matches explicitly name Srgb/Luma/LinearRgb/Oklab/Cmyk/Spot as
nonangular; Oklch retains index 2 and Hsl/Hsv index 0. mix default first preserves
the compatible Spot pair, then Oklab fallback with the first axis naming all
nine Color variants. mix_weighted first preserves Spot only when all are
compatible, then Oklab fallback naming all nine variants. Keep guards/order/
short-circuiting, aliases/clones, arrays, weights, circular paths and calls
without changing bodies, adding helpers or normalizations.

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
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/color/mixing.md`. Historical observations are not a new parity certification.
