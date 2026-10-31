# L0 — infra/font_metrics/basic
Hash do Código: 93ebe699

**Layer:** L3
**Exclusive consumer:** `03_infra/src/font_metrics/basic.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/layout/optical-overhang.toml sha256:8c4e3b767247e54f966c3a86be1b528b030dc4c8d05ea9b358533351720a3cf2

## Current evidence

- `03_infra/src/font_metrics/basic.rs:24` — `typo_metrics`.
- `03_infra/src/font_metrics/basic.rs:53` — `edge_offset_pt`.
- `03_infra/src/font_metrics/basic.rs:110` — `advance`.
- `03_infra/src/font_metrics/basic.rs:164` — `vertical_metrics`.
- `03_infra/src/font_metrics/basic.rs:178` — `cap_height`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/font_metrics/basic.rs`.

## Owned contract

- Own typo_metrics, edge_offset_pt and FontMetrics for FontBookMetrics. Convert design units proportionally by size/upem, preserving missing-glyph/default fallback and zero-unit guards.
- Use OS/2 metrics before hhea and respect cap-height/baseline edge policy. Explicit Length edges resolve at the supplied size; unknown metrics preserve current defaults.
- Text ink unions use max(0,ymax) and max(0,-ymin). Glyph-by-id ink retains signed ymax/-ymin, so a glyph entirely above baseline can have negative bottom extent. Bbox methods use the requested codepoint or glyph id in this face, without remapping.
- Apply math italic correction only to the singular base outside TextItem provenance; variation selectors are not extra bases. Delegate MATH/ssty extraction to the math child and glyph_to_char to the facade's precomputed map.

## Verification and boundary

Run `cargo test -p typst-infra --lib font_metrics:: -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Explicit fixture initialization

### Evidence

`lab/results/construction/diagnostics/field-reassign-2026-10-15/before.json.gz`
retains source/spans/prior L0/HEAD/working tree/UTC/diffstat; initial-diagnostics.json
records defaults followed by test-local field assignments at basic.rs:505,513,520.
This is Rust representation maintenance, no new parity measurement or product
default change.

### Contract and acceptance

Construct the three local TextStyle values with unchanged top_edge/bottom_edge:
pt/em lengths and baseline metric, inheriting remaining defaults. Preserve supplied
size, unit conversion, text_edges calls and existing tolerances.

Use ..Type::default() for remaining fields; remove mut only where no mutations
remain. No new types/builders/APIs/lint suppression. L0 first and scoped resealing,
continuous ADR-0127. Selected gate must fail before and pass after; preserve test
names/inputs/expectations/tolerances. Revalidate all-target Clippy/workspace suite/
build/fmt/Rustdoc/lineage. No tests merely mirroring initializer syntax.


## Approved font-backed underline metrics — 2026-10-22

### Evidence and public-contract gate

The fresh paired sources and exact source/font/binary/UTC/dirty-state provenance
are in `lab/results/remaining-parity-2026-10-22/underline-before-font-scoped/measurement.json.gz`
(SHA-256 `db81849f43eaa92212629aeaddce587ba5fff8465fd878a22890cd05e894a838`).
DejaVu at 11pt requires downward offset 0.21484375pt and thickness
0.4833984375pt; Nimbus at 11pt requires 1.386pt and 0.55pt. Both 17pt
witnesses preserve the actual table scaling. These measured differences
refute a universal fixed-font coefficient. Noto's numeric equality does not
waive the separately measured SVG open-line fill failure. Exact font-table
hashes and reads are retained in the sibling
`agents/inline/underline-font-table-measurement.json`; rendered pairs and
source language geometry govern, rather than a required Rust representation.

The human approved the exact additive contract and bounded integration in
`lab/results/remaining-parity-2026-10-22/agents/inline/underline-contract-proposal.md`,
SHA-256 `e7f3db142865646f13f9041f36f572582802cd302f1aebb7f8b42bb0610f2c6f`.
Approval is recorded in `lab/results/remaining-parity-2026-10-22/underline-approval.json`.
This satisfies ADR-0127 section 2.1 for exactly the method below; it supersedes
only this addition's earlier no-new-public-method restriction. No new entity,
FrameItem field, phase, dependency, mode or unrelated API is authorized.

### Exclusive real-face provider contract

Implement only the approved FontMetrics::underline_metrics method for the
existing FontBookMetrics. Add one shared private face helper used by the
fallback provider: position from underline table, otherwise -0.2em;
thickness from underline table, otherwise strikeout table, otherwise 0.06em.
Convert offset with reversed sign and requested size/upem; preserve zero
and signed position, and ensure nonnegative thickness. Do not clamp a real
position or substitute fixture coefficients. Underline fallback is separate
from unchanged text edges/vertical metrics and other decoration kinds.

Require genuine real-font RED before the override: portable DejaVu and Nimbus,
11pt and 17pt, deriving expected values directly from their pinned actual
tables. The explicit size must govern even when style.size differs. Include
missing underline-table position/thickness fallbacks and absent-table defaults
using a pure helper witness, plus direct/dynamic provider agreement. This
adds no public L3 helper or new parser dependency.

## Explicit optional absence and closed-domain preservation — 2026-10-23

### Evidence before decision

The exact source and owner bytes, HEAD, dirty-tree stat and initial nine V16
findings are preserved in
`lab/results/v16-option-exhaustiveness-2026-10-23/before-state.json`,
`before/` and `before-v16.sarif`. The parser source
`tekt-linter/03_infra/rs_parser.rs:1605–1621` classifies every bare identifier
pattern as a catch-all, including the Option unit variant `None`. The Rust
compiler resolves that name to the unit variant; qualification makes that
existing nominal meaning explicit to both tools. These are mechanical
coverage observations, not a newly discovered runtime failure or new vanilla
parity claim.

### Exclusive correction and acceptance

`basic.rs:61–66` already enumerates Length/Metric TextEdge and absence. Qualify only the absence pattern as `Option::None`; preserve cap-height/baseline defaults, explicit signed lengths, metric fallback, units and size. Future TextEdge variants must still require a decision.

Use ADR-0127 continuous internal flow: no public contract, default, phase or
compatibility changes. Add no lint exception, allow/expect attribute, severity
change, suppression or match-to-if rewrite. Preserve all existing assertions
and pending limitations. The initial V16 gate is structural RED; preservation
tests may already pass before implementation and must not be misreported as
behavioral RED. Require the final unfiltered linter, tests, strict Clippy,
rustdoc, formatting, canonical paths and both lineage directions.

## R3 variation selectors have no independent advance

Evidence: `lab/results/math-geometry-r3-2026-10-23/candidate-debug/receipt.json` preserves source/binary/UTC/dirty state; the supplementary-selector glyph cluster has a spurious 7.2 pt advance between x and y. Pinned `math/ir/resolve.rs:326–358` retains the selector in its base grapheme, and `math/shaping.rs` measures the shaped cluster. Current advance loops visit selector codepoints independently; when cmap lacks a supplementary selector they add the missing-glyph 0.6em fallback. The existing single-base italic-correction guard already excludes both selector ranges. Geometry and extractable source identity require parity; loop structure is mechanics. Refute by altered base advance, dropped selector identity or changed ordinary missing-glyph behavior.

Exclude FE00–FE0F and E0100–E01EF only from the scalar advance summation, preserving complete source/shaping payload, base italic correction, ssty and every other missing-glyph policy. Selector-only input has zero advance. This does not invent a scalar cmap glyph for a variation sequence or certify every unsupported variation glyph. Internal ADR-0127 flow: L0 first, real font-backed advance RED→GREEN, original and supplemental source→PDF/SVG controls, valid pins/seals and full validation.


## R8 — font-backed note marker geometry (public contract approved)

Status: **APPROVED FOR MATERIALIZATION**. The human approved exactly
FontMetrics::text_superscript_metrics on 2026-10-24. The original drafts and
explicit approval are bound by `lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24/approval.json`.
This satisfies ADR-0127 for this addition; all RED→GREEN and lineage gates apply.

### Measurement before proposal

`lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24/baseline.json`
and `received-source.json.gz` bind the exact dirty HEAD and all inputs. Fresh
`before/receipt.json` reproduces ten current-pin note controls; all ten fail the
actual-output 0.001 pt physical page/word-box gate in `red-geometry.json` and
`red-geometry.log` (ten assertion failures, no harness errors). The original
extracted word order remains separately available; physical matching preserves
page membership and every word occurrence without replacing these observations.

`font-tables.json` binds actual table/font hashes. DejaVu Sans at 11pt has
OS/2 superscript ySize1433/upem2048 =>7.69677734375pt and upward offset
983/upem2048 =>5.27978515625pt. Liberation Serif has ySize1331/upem2048
=>7.14892578125pt and upward offset928/upem2048 =>4.984375pt, plus a `sups`
GSUB feature requiring actual whole-run coverage verification. The current
`compiler/layout/text/style.rs:288–300` instead uses 0.65 and a positive
0.3em baseline resolved after scaling; `cursor.rs:72–75` adds that offset.
The pinned oracle reads OS/2 script metrics in
`typst-library/src/text/font/metrics.rs:67–80`, default superscript metrics
in `text/shift.rs:213–218`, and actual GSUB-or-synthesis selection in
`typst-layout/src/inline/shaping.rs:1160–1215`. Its rendering rules apply 85%
entry text size, 0.5em leading, 1em indent and 0.05em number gap in
`model/footnote.rs:334–341` and `typst-layout/src/rules.rs:401–411`.
These are font-dependent render/language observations; Rust representation and
vanilla iteration mechanics are not acceptance criteria (ADR-0107/0108).

The current public FontMetrics API exposes advances, vertical/ink edges,
underline and mathematical metrics, but no text superscript metrics or native
script-feature coverage. Mathematical MATH constants select a different
semantic source and cannot be repurposed as OS/2 text metrics. L1 must not read
font files or select arbitrary measured coefficients to bypass this boundary.

### Approved exclusive provider obligation

Resolve the injected method from the actual single face with active variations; read OS/2 units and full native-GSUB coverage; keep the common provided fallback when unavailable. Preserve all existing measurement/painting methods.

The only approved public addition is the exact method and tuple contract in
`00_nucleo/prompts/core/compiler/layout/metrics.md` §R8. This provider obligation
is authorized by that recorded human approval. Tests first must compare real font tables,
scaled synthetic metrics and complete native-feature coverage at multiple sizes;
advance and emitted geometry must agree. Preserve all unrelated methods, fonts,
source effects and current error behavior. Full workspace and lineage gates apply.

R8 native measurement obligation: an active superscript run certified by complete sups coverage uses native glyph advances at unchanged outer size, including direct advance calls. The single-face provider may use rustybuzz on its varied face for this existing operation; do not apply native shaping to ordinary text or synthesize from MATH.


## R8 approved resolved native-script choice

Measurement before decision: R8 numbering-candidate5/receipt.json records
Nimbus11 numbering1 Z with1.4718pt error; the full marker chose synthesis but its
shortened digit was shaped with sups again. The root is
lab/results/construction/diagnostics/footnote-layout-r8-2026-10-24/.
The exact public TextStyle::superscript_native: Option<bool>, default None, was
approved from owner draft SHA4b14829d1eb38475d1985a9707a6e6ce40166196dfe524afe512d99b9cabf4d0
and is recorded in native-carrier-approval.json. Its data/default contract belongs
only to core/entities/layout_types.md; this owner retains its existing consumer.
No MATH flag, baseline sign, font-name constant, private string or source identity
may substitute for the explicit choice. No extra public field/signature/phase.
Direct native advance honors Some(false) as a hard refusal; None keeps automatic
coverage and Some(true) still verifies actual face coverage. Metrics capability
queries remain pure font/run queries. Preserve requested size/axes and all other
methods. Test forced synthesis at the same size against actual ordinary glyph
advances and true/native/default states.

Require new field/default/forced-false RED→GREEN, all twelve numbering refuters at0.001pt PDF/SVG, original ten notes, six font controls, warm/cold cache and full preservation/workspace/lineage gates.


## R10 — source superscript runs and mandatory breaks

Measurement before decision: `lab/results/construction/diagnostics/superscript-r10-2026-10-24/{baseline.json,before/receipt.json,baseline-audit.json,approved-metrics.md,approval.json}` binds received R9 source/UTC/dirty stat and pinned bd5ebec2 binaries. Six of fifteen controls fail the unchanged complete physical words/pages and0.001pt gate: three literal-newline cases, Nimbus hard break, and Nimbus/Liberation mixed fallback. Pinned inline/collect.rs:191-196 includes mandatory breaks in source script text; inline/shaping.rs:970-1004,1090-1107,1167-1217 retains outer size, actual face/run and full causal coverage. This is language geometry and source preservation, not a byte/algorithm obligation. Font-specific synthetic metrics cannot be guessed from the primary face after fallback. The exact additive FontMetrics::text_superscript_runs contract was approved at SHA65d01600110f9bffea1eaff0ca349f737ab426e384fb891e88d8f6504cf911b9; every other public method/field/default/phase stays unchanged.

Classification: approved public method plus private parity integration under ADR-0127. No font I/O, callbacks, dynamic evaluation or font-name coefficients in L1. Require real module RED→GREEN before runtime code, default/override/borrowed contract tests, source partition and restoration, native/synthetic/mixed-face/warm-cold/explicit-size refuters; all original controls, R8/R9 and A–H/R1–R7 replays, PDF/SVG0.001pt and full workspace/lineage/resources validation. Preserve all historical observations and unrelated received input bytes.

### Exclusive owning obligation

Override the approved runs method for one real face, preserving actual OS/2 metrics and whole causal-run native coverage, forced synthesis and explicit synthetic size. Keep the original capability-query contract unchanged. The provided tableless fallback is not a substitute for actual face tables.

R10 measurement refinement before code: `font-tables.json` in the R10 evidence
root records actual font hashes and Nimbus OS/2 yOffset350/1000, correcting the
400/1000 numeric typo in the preserved approved draft without changing its exact
public signature or scope. Pinned inline/shaping.rs:473-510 measures line edges
at outer styles.resolve(TextElem::size), while build:350-363 paints at each
reduced glyph size/offset. Resolve run top/bottom edges from the actual face at
that outer size and carry them as existing absolute TextEdge lengths; do not
use the reduced paint size to shrink the line's typographic envelope. No new
field or public-method change follows. Require actual-face line-origin and
explicit-edge controls; a regression in R8/R9 refutes an unrestricted change.


## R10 — expanded signed-tracking, fallback-space and literal-tail refuters

Measurement before decision: candidate2-build/receipt.json/source.json.gz and
candidate2-matrix/receipt.json bind exact dirty inputs, UTC, sources and binary
f2a61237c15688e655ff7c62a117cb50a4d3a154e426bd31bed61fa999385e68
against the fixed bd5ebec2 oracle. Original15 script PDF/SVG controls and all
R8/R9 controls pass; the54 additional controls expose22 failures. At Nimbus11,
trailing literal LF loses a3.058pt following markup space; explicit DejaVu
fallback overmeasures that space by0.43858pt. At DejaVu11 tracking0.2pt, the
script's three intervals overadvance by0.18018pt. Nimbus synthetic digits need
0.12pt intervals and the following fallback glyph needs the same last-prefix
interval rather than zero. Pinned inline/shaping.rs:1269-1295 divides tracking
by outer ctx.size before each glyph's actual reduced size is applied; :473-510
retains outer-size line edges. Source sequence/space causes and actual selected
text face explain the remaining cases, without a font-name branch or looser
physical-word/page/0.001pt gate. This is private parity integration of the
approved method, not another public contract/default/phase change.

### Exclusive owning obligation

The actual-face resolved style carries signed absolute tracking = original tracking at supplied outer size times actual paint_size/outer_size. Native scale1 preserves tracking; synthetic and explicit size use their actual scale. Keep original semantic morphology and outer line-edge contract.

Require module RED→GREEN, expanded54-source matrix without changing thresholds or source membership, original15/R8/R9, A–H/R1–R7, full workspace/Clippy/fmt/Rustdoc/forward-reverse lineage, resource and preservation gates. Retain every failed candidate and its actual source state.

## Approved actual-glyph optical margin ports

Measure first: physical5-candidate/receipt.json and comparison provenance under
readiness-fixes-2026-10-28 bind bed9a095… source/build/UTC/dirty state and exact
article. Accepted justified lines are1.935998pt short at terminal period. Pinned
inline/line.rs:493-526 adds optical remainder only after line selection; :676-716
uses actual GID/face and advance. Do not modify candidate choices or costs.
The direct human approval in optical-overhang-metrics-approval.json binds review
SHA-25629d7f6fcd280ecfe5abd27b54c50b8326822504ea397ab978e62771de9c10ecb.
It supersedes no-new-API clauses ONLY for the two exact methods in that review.
All earlier APIs, phases, source/effect and validation boundaries remain.

Implement the two approved ports on FontBookMetrics. Shape text in this actual
face with existing language/axes/features/size rules; return the actual terminal
metric and total glyph count for certified single-line LTR input. A shared private
helper maps supplied policy chars through actual cmap/GID and multiplies the
provided actual advance. glyph_overhang always uses the Font argument, even when
different from this provider's face. Validate ratios/font/advance without panic;
no matching GID yields Some(0), unprovable data None. No family-name rules.

Require real RED→GREEN, strict lineage/lint/full suites/build and fresh paired
PDF/SVG, keeping all historical negative and performance/resource obligations.
