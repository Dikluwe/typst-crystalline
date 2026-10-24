# Mathematical geometry and variation selectors — R3 — 2026-10-23

All seven reported R3 divergences are corrected. All 19 original render probes
now qualify in complete PDF words, line/page membership and every bbox coordinate,
plus independent SVG glyph/rule coordinates, at the unchanged **0.001 pt** gate.
The added controls cover isolated/mixed math, explicit/paragraph/page breaks,
font sizes/edges, native multi-grapheme symbols, operators, combining marks,
BMP and supplementary variation selectors. Their explicit remaining limits are
listed below; this is not a claim of global mathematical rendering parity.

## Exact received state and L0

HEAD `cf21f47a3b63f0e9fc5308925e1d0b67925e7227`, **uncommitted working tree**. The
[baseline](../../lab/results/math-geometry-r3-2026-10-23/baseline.json) binds UTC,
full dirty stat/status, binary hashes and all source/configuration/L0 inputs to
R2's exact completion. The [source archive](../../lab/results/math-geometry-r3-2026-10-23/baseline-source.json.gz)
includes untracked productive sources. No reset, commit, oracle resync or discarded
existing change occurred. [Diff from the received state](../../lab/results/math-geometry-r3-2026-10-23/changes-from-received.diff)
and [audit](../../lab/results/math-geometry-r3-2026-10-23/audit.json) retain the
21 changed inputs: ten owning prompts, their ten consumers and one shared Nucleus.
Three consumers (fallback catalog, resolution and shaping) change lineage only.
All 1853 font/reference assets remain unchanged.

[Artifact source bindings](../../lab/results/math-geometry-r3-2026-10-23/artifact-source-bindings.json)
map each immutable executable SHA to its actual source archive. A probe records
the host working tree separately; the frozen before executable keeps R2 sources
even when observed after later test drafts.

Oracle commit `bd5ebec285505889e8ff1d4f68804ab147a699f3`, tree
`cac836c07e4dec2a89219370428292393a3216b5`, SHA-256
`9b2373155e9c05bbb0daf5755bf4be88a11e64bd6b4f8b376648fa27303373b9`. The final release build runs
`2026-10-24T02:15:11.792344+00:00`–`2026-10-24T02:23:54.854802+00:00`, SHA-256
`84e99b70d0f538fd5af05a42b6565232618c949be1916559aaa763f4ce0a9005`; [receipt](../../lab/results/math-geometry-r3-2026-10-23/final-build/receipt.json)
and [sources](../../lab/results/math-geometry-r3-2026-10-23/final-build/source.json.gz)
bind its exact dirty state and recipe. Source inputs stay frozen throughout final
validation/build/measurements. Later diagnostic edits have separate hashes in the
completion receipt. [Effective resource properties](../../lab/results/math-geometry-r3-2026-10-23/complete-active-resource-properties.txt)
confirm MemoryMax 12 GiB, MemoryHigh 8 GiB and zero swap; serial single-job commands
and timestamps remain in the raw evidence.

## Measurement before correction

[Fresh original probes](../../lab/results/math-geometry-r3-2026-10-23/before/receipt.json)
reproduce seven failures and twelve successes. [Reductions](../../lab/results/math-geometry-r3-2026-10-23/reductions/receipt.json)
refute a show-rule-specific baseline cause: isolated `$alpha$` also differs,
while ordinary text on the same line gives the correct maximum. The owning
inline equation adapter installed the outer text strut before admitting its
already measured normalized math extent. It now uses the existing known-top
admission with that extent, preserving the 0.7-leading normalization and block
continuation protocol.

Pinned `math/ir/resolve.rs:326–358` resolves native Symbol by grapheme. The old
single-run dispatch omitted per-grapheme italic corrections and class spacing.
The existing sequence owner now composes complete graphemes through its normal
glyph/spacing dispatch, keeping combining marks and selectors attached, public
Symbol identity intact and ordinary MathIdent/Text lexical behavior unchanged.

The body DejaVu Sans declares a MATH table. The shaper used that capability to
activate the mathematical fallback chain even outside mathematical mode, choosing
NewCMMath instead of FreeMono in the mixed Unicode input. Explicit language mode
now selects the extra math chain; declared families retain their priority. The
shared fallback-selection Nucleus and every consumer pin were updated. Run
coverage retains selectors with the base. Both scalar metrics providers exclude
selectors from the sum rather than inventing a missing-glyph 0.6em advance;
base italic correction, complete shaping text and other missing-glyph policy stay.

These are internal parity corrections under ADR-0127 continuous flow, with L0
before code and no public field/signature, product mode/default, compatibility or
pipeline phase change. Observable geometry/text require parity; data structures,
private dispatch and output bytes are mechanics.

## Refutation and RED→GREEN

[Initial core RED](../../lab/results/math-geometry-r3-2026-10-23/red-tests/receipt.json)
contains three behavioral failures; Cargo stops at core, so
[separate infrastructure RED](../../lab/results/math-geometry-r3-2026-10-23/red-infra-tests/receipt.json)
confirms the MATH-table/body-mode failure independently. Initial focused GREEN
passes all four. The [first candidate](../../lab/results/math-geometry-r3-2026-10-23/candidate-debug/receipt.json)
is retained: its isolated baseline fix exposes a space/break strut regression,
and the supplementary selector still has 7.2 pt of spurious advance. It is not
reported as final success. The initial pin-reseal attempt's V26 failure is also
retained; canonical dependency-framed pins were repaired before RED and implementation.

[Space refuters](../../lab/results/math-geometry-r3-2026-10-23/space-reductions/receipt.json)
show that interior Markup/Newline spaces and an explicit break require the outer
text maximum; trailing weak whitespace at EOF does not. The line owner consumes
those actual retained text styles only on a line with inline Formula, using its
existing maximum/commit. Separators remain neutral carriers, no invented ink,
new text or page-specific compensation. Lines without Formula retain their prior
style traversal. A second real [core RED](../../lab/results/math-geometry-r3-2026-10-23/red-supplemental-tests/receipt.json)
and [infrastructure RED](../../lab/results/math-geometry-r3-2026-10-23/red-supplemental-infra-tests/receipt.json)
precede those corrections. Final [GREEN](../../lab/results/math-geometry-r3-2026-10-23/green-supplemental-tests/receipt.json)
passes all six focused tests. A subsequent full-lint preflight rejects the compound
weak-space guard with V17; its receipt remains under preflight-attempt1. Only
that Boolean conjunction was desugared to a nested check, with L0 first and
no suppression or changed behavior. The final source-frozen full suite confirms
all six named geometry tests after that syntax adjustment. No test expectation
or bbox tolerance was relaxed.

## Bilateral final results

[Final release corpus](../../lab/results/math-geometry-r3-2026-10-23/R3/receipt.json)
and [source-bound debug corpus](../../lab/results/math-geometry-r3-2026-10-23/after-debug/receipt.json)
retain raw PDF words/bounds and SVG primitives, argv, diagnostics, font path,
source/binary/worker hashes and UTC. The final observer supports actual multiple
SVG pages; an earlier observer's identical multiple-image refusal in the two
compilers is retained, not counted as successful rendering.

| Reported R3 case | Before PDF delta pt | Final PDF delta pt | Final SVG glyph delta pt |
|---|---:|---:|---:|
| unicode | 2.136005 | 0.000005 | 0.000000 |
| show-math-regex | 0.921190 | 0.000000 | 0.000000 |
| show-math-text | 0.921190 | 0.000000 | 0.000000 |
| math-regex-nonmatch | 0.921190 | 0.000000 | 0.000000 |
| math-regex-per-element | 0.921190 | 0.000000 | 0.000000 |
| show-native-math | 0.921190 | 0.000000 | 0.000000 |
| selected-multichar | 0.492000 | 0.000010 | 0.000000 |

All 19 originals pass, preserving the previously passing twelve. Of
28 additional controls, 17
previously failed controls now qualify; every previously successful control stays
successful. The complete corpus and audit retain all failures, without coordinate
normalization, omitted words or glyph-count-only acceptance.

## Remaining boundaries

| Additional control | Before PDF delta pt | Final PDF delta pt | Final SVG glyph delta pt |
|---|---:|---:|---:|
| control-markup-base | 2.289190 | 2.361190 | 2.361187 |
| control-markup-vs | 2.289190 | 2.361190 | 2.361187 |
| control-math-explicit-break | 0.921190 | 0.921190 | 0.921187 |

The isolated ordinary-text fallback cases draw the correct selected face and
advance but still use the body font's line edge; the mixed Unicode original
qualifies because the actual surrounding text/emoji supplies that maximum.
This additional general fallback-face baseline boundary is not declared closed.
The inline-math-after-explicit-pagebreak control also retains its preexisting
0.92119 pt baseline mismatch; the original block-equation continuation controls
are a separate preserved scope.
No inference of complete UAX29 variation-font support, arbitrary mathematical
callback/rendering, general fallback typography or broader R1/R4–R7 closure follows.

## Complete validation and preservation

[Full validation](../../lab/results/math-geometry-r3-2026-10-23/validation-complete/validation-commands.json):
**12/12 gates**, **9867 Rust tests passed, 7 ignored**,
34 Python CI tests passed. Workspace build, domain/infra/wiring tests, doctests,
all targets, Clippy with denied warnings, fmt, Rustdoc with denied warnings,
strict/reverse lineage, final full crystalline-lint and diff check pass. No
productive source changes during the final commands or observations.

[Replays](../../lab/results/math-geometry-r3-2026-10-23/replays.json) preserve
60 A page/word controls, 28 B init observations, 40 C/D/F complete PDF geometry
controls, 210 E raw outcomes (including existing errors/extensions), 98 H stdout
controls, ten R1 note controls and the complete R2 corpus/boundary classification.
R2's justified optical-margin and refused word/tracking limits remain explicit.
The legacy symbol checker policy debt remains open; preserved raw outcomes do
not make all reflections successful parity.

All seven [fresh RSS workloads](../../lab/results/math-geometry-r3-2026-10-23/memory/receipt.json)
retain the original 3% round-repeatability and current/before median cap of 1.03,
with original PDF geometry at 0.001 pt, in three randomized rounds of three fresh
processes per role/workload and two warmups. Normalized executable is
50027248 bytes versus pinned oracle 52271120, within the
original G cap. These retain G and do not claim a new memory or latency gain.
