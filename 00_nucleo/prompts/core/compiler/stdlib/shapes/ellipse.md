# L0 — stdlib/shapes/ellipse
Hash do Código: 95ae11dc

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/shapes/optional-stroke.toml sha256:016cad80570d8133fcb275e6e0f693e088dc921e4d6d907a51888898ae8b00ce

## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/stdlib/shapes/ellipse.rs`. At HEAD `a5c6b93c258acb7cb378b9b147a8c6cd0e8d17b9` with a clean tree, `01_core/src/compiler/stdlib/shapes.rs:229-350` contains the extracted behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-shapes-2026-09-30-before.json.gz`. This is internal separation by behavior.

## Contract

Construct ellipse and circle; circle converts radius to diameter and uses ShapeKind::Ellipse. Preserve fill/stroke and fallback.

Detailed rules, historical evidence, parity limits and diagnostics remain in `00_nucleo/prompts/core/compiler/stdlib/shapes.md`. Extraction preserves public signatures, defaults, phase and compatibility. No I/O or mutable global state is permitted.

## Acceptance

Preserve the L1 suite and shapes tests, build, and pass `crystalline-lint .` with correct ownership and hashes. Do not infer performance gains.


## Explicit absence correction

Before decision, visual-rect/visual-circle probes in
`lab/results/construction/diagnostics/paridade-dominios-2026-10-06/measurement.json.gz`
compile at historical vanilla pin 9f2b6e871 but Tekt rejects the stroke cast.
Upstream shapes.rs:125–129,335–339 distinguishes fill-dependent Auto from
Custom(None). The consumer always sent None to the required Stroke parser and
its fallback ignored argument presence. Correct language acceptance/form
without inferring historical intent.

Apply the pinned optional-stroke nucleus to argument reading/fallback for both
constructors. Do not change the shared required parser, still-unrepresented
per-side strokes, signatures, dimensions or radius. Test None with/without fill
and omission without fill, RED→GREEN. This ADR-0127 internal parity correction
adds no API/phase/default, unsafe code or dependency. Revalidate PDF/raster and
macro controls against the qualified ratified pin.


## Ratified Circle constructor origin

### Evidence and approval before implementation

At HEAD `954d13185e26a1ed90f6d7c60f6ba27760ca9889`, the source/owner audit in
`lab/results/v16-corrections-2026-10-19/circle-origin/l0-before.json` freezes
UTC, exact diffstat and hashes. The approved proposal
`lab/results/v16-corrections-2026-10-19/batch-f/agents/geometry/circle-deferred/proposal/review.md`
(SHA-256 `5b7136bc18f419004113d8d83c9c54adb816e3b70389039aeeed35e1c6f772e8`)
binds the baseline equality and rejected full-Length candidate receipts.
`stdlib/shapes/ellipse.rs:92-104` erased the em component;
`entities/elements/shape.rs:21-29` used one geometry carrier for both constructors.
Those observations concern language identity and geometry, not mandatory upstream
Rust structure. The explicit user approval is recorded in
`lab/results/v16-corrections-2026-10-19/circle-origin/approval-receipt.json`;
it satisfies ADR-0127 section 2 for exactly this public type/field and migration.

### Exclusive approved responsibility

Construct native Circle directly with origin ShapeOrigin::Circle and kind
ShapeKind::Ellipse using the current public fields, without mutating a shared Arc.
Ellipse retains the generic Geometry constructor route. Keep names/native
signatures, name validation before radius and radius before paint/stroke parsing.
Keep the pinned optional-stroke nucleus and all existing paint/default rules.

For explicit Float/Int radius, retain the point-number extension via Length::pt.
For explicit Length, preserve absolute and em components. Double the full Length
and store canonical Value::Length in equal width/height. Do not resolve fonts in
eval. Same-circle numeric/float/Length equivalence remains; mixed absolute/em
radii stay distinguishable. Preserve negative and nonfinite numeric/Length
behavior without clamping. Explicit Auto becomes Value::Length(Length::ZERO),
retaining zero diameter and equality with radius 0, 0.0 and 0pt. Omission keeps
None dimensions and remains distinct from explicit Auto. This is explicit retained
default debt, not correct Auto sizing or authority to hardcode an oracle diameter.

Reject every other current Value variant with the measured diagnostic
`expected length or auto, found {type}` at the last named radius occurrence value span, falling back to its argument
span and then args.span when each more specific anchor is detached. Keep the decision
nominally exhaustive, with Auto an explicit debt branch; a real future Value
variant must require a new decision. Ratio and Relative are rejected radius
classes. No silent malformed-to-zero conversion or lint suppression.

Circle and generic Ellipse differ even with equal final dimensions or numeric
carriers. The approved accidental equalities to correct are Circle versus generic
numeric Ellipse and collapsed em radii; do not remove the numeric radius extension.
Verify numeric/Length equivalence, pt/em/numeric Circle-versus-Ellipse, mixed radii,
aliases, arrays, dictionaries, membership and Source/semantic wrappers. Preserve
zero/negative/paint/stroke/default controls and exact malformed/name precedence.
Actual logical geometry and paint bounds must measure radius2em as diameter40pt
at text10pt and80pt at text20pt, radius2pt as4pt, mixed scaling, reuse under two
occurrence font sizes and existing Block/Box nesting. Retain all archived six
Circle refuters; any unapproved matching-case regression refutes implementation.

This narrowly supersedes the earlier absolute-only radius/fallback and unchanged
radius clauses. It does not implement Circle func/fields/selectors/show/set/body/
inset/outset/width-height syntax or complete defaults. Their pending obligations
remain visible. Retain the frozen replay corpus; only approved case-specific
expectations change. Require RED before code, focused/full tests, builds, strict
Clippy/Rustdoc/fmt, paths, direct/reverse lineage, V15/V26, source freeze and exact
V16/V19 delta against recorded binaries/HEAD/UTC.


### Complete rejection witnesses and bounded frame observer

The independent review in
`lab/results/v16-corrections-2026-10-19/circle-origin/contract-test-review.json`
found that the first controls execute only nine of the 35 rejected Value
variants. Exercise all 35 with real typed values, exact literal type diagnostics
and the same causal span; no unsafe or placeholder payload can stand in for a
variant. The additional controls complement, rather than replace, the original
semantic RED, frozen bilateral corpus and future-variant E0004 proof.

The local emitted_shapes test observer deliberately collects only Ellipse frame
leaves and recurses through Group and Link. Replace its wildcard by the explicit
remaining FrameItem variants, retaining that traversal and ignoring non-Ellipse
Shape leaves. Semantic and TextReplacement remain opaque in this bounded helper;
claim no descendant coverage for those carriers. Exact local dimensions/paint
presence/stroke observations remain unchanged. The separate frozen paint corpus
owns physical output bounds and cannot be replaced by this helper or its internal
representation. Do not broaden productive geometry or weaken expected results.


## R13 — approved ShapeElem literal migration

The source inventory in lab/results/construction/diagnostics/relative-lines-r13-2026-10-25/migration-owners.json identifies this consumer. Owner approved the public ShapeElem.relative_line addition (proposal SHA-256 d8c9a01413232bac7113d1148af3c1b3bcd671bb37eeca68a94d5749d671db00). Initialize that field to None in existing constructors and fixture literals, preserving all geometry, paint and behavior. No other implementation change is authorized here. Full existing tests, build and lineage must pass.


## Issue 8894 — approved Stroke migration

Measurement: the source inventory and previous owner bytes in
`lab/results/construction/diagnostics/issue-8894-2026-10-31/` bind exact
HEAD/dirty diffstat/UTC and each affected source/owner before migration.
`stdlib/layout/stroke.rs:430–434` erases em; geometry.rs:81 carries only f64.
The explicit user approval in approval.json ratifies the public Length field
and occurrence-layout resolution, as specified by
`00_nucleo/prompts/core/entities/geometry.md`. This consumer retains its
exclusive responsibility and all other ratified clauses and controls.

Migrate this consumer's declarative Stroke literal thickness from scalar
points to Length::pt without changing their value or other fields. FrameStroke
remains scalar. Keep all existing semantic assertions; update only their
representation as necessary. Producers preserve full Length; layout consumers
resolve against effective font size before scalar geometry. Repr preserves
absolute/em components. RED→GREEN, build/tests and scoped lineage are required.


## Issue 8583 — approved consumer obligation

Measurement and explicit public/phase approval are bound by before/combined
source archives, baseline.json and approval.json under
`lab/results/construction/diagnostics/issue-8583-2026-10-31/`.
The radius caster at casting.rs:31–39 erases ratios and rect.rs:44–56 drops
side dictionaries. These are language geometry/paint losses.

Migrate only this consumer's ShapeElem literal/destructure: legacy rect_radius
and rect_stroke are None (or explicitly bound/observed for observation). Preserve
all existing values, expectations and other APIs; public addition is approved.
Do not materialize unrelated square/container features.
