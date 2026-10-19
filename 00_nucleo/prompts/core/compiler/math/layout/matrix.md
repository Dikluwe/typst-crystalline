# L0 — math/layout/matrix
Hash do Código: 0431ed0b

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065


**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/matrix.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/matrix.rs:21` locates `pub(super) fn layout_matrix(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Matrix carrier, alignment and augmentation

Consume complete MathMatrixElem and preserve rows/cells, optional delimiter
sides, alignment, augment, relative gaps and presence metadata.
Derive effective fields by explicit > math.mat StyleChain > defaults;
resolve gaps/stroke against outer style, while cells use shared denominator
style. Default row gap is 0.2em, column gap 0.5em and automatic augment
thickness 0.05em; these are language defaults, not measured PDF adjustments.

Without ampersands, honor start/left/center/right/end alignment. With
MathAlignPoint, split cells late into alternating right/left subcolumns.
Transfer natural class spacing at ampersand boundaries into the left cell's
width, not a normal comma-separated column gap. Preserve unequal split
counts, row order and empty cell morphology.
Physical direction limitations are not a new definition of start/end.

Use shared grid floors, row baselines and 1.1 delimiter target. Axis-center
only grid content before merging already-centered delimiters, with no
redundant horizontal padding. Final ascent/descent includes delimiters;
horizontal composition preserves all component baselines.

Augment accepts multiple horizontal/vertical offsets, preserves duplicates
and order, normalizes negative offsets from the end and accepts borders 0/n.
Horizontal offsets validate against row count, vertical against column count;
out-of-dimension offsets are diagnostic, not silently clamped or ignored.
Interior strokes lie at the corresponding gap midpoint; borders use the
full box boundary. Auto stroke uses Square cap; explicit/custom folded stroke
preserves thickness, paint and cap. The compatibility wrapper retains its
established Butt behavior and does not redefine public mat defaults.
Do not bake raster/PDF behavior into this owner.

Acceptance checks axis-specific relative gaps, asymmetric alignment, every
offset including negatives/borders/duplicates, invalid dimensions, automatic/
custom/folded strokes, optional delimiters and the complete Content::math_matrix
route. Existing consumer deviations remain pending diagnostics, not exceptions
to these contracts.

## Meaningful local geometry control and unused-wrapper boundary

### Evidence before decision

At HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree,
matrix.rs:543–549 contains the measured fragment. Clippy inventory/owner reading
distinguishes effectless code, deliberate invalid data and empty checks.
`lab/results/construction/diagnostics/controle-fluxo-ci-2026-10-13/before.json.gz`
and companion evidence retain sources/L0/UTC/status/diffstat/commands. This is
neither new parity evidence nor upstream-intent inference.

matrix.rs:21,377,378 contains the historical helper/storage inventory items.
`lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`
retains the same HEAD/uncommitted state plus sources/owners/references/UTC/status/
diffstat. Call-site/productive-path research precedes classification, without
new parity/performance evidence.

### Exclusive contract and acceptance

Replace the local always-true test with productive 2×2 matrix layout: no
delimiters, differently sized text, left alignment and explicit absolute gaps.
Verify all four texts in row/column order, shared row baselines, increased column
separation with column gap and increased row separation with row-gap. Expectations
are contract geometry relations, not copied candidate values; empty output or
lost/reordered cells must fail.

Compile internal layout_matrix only under `#[cfg(test)]`, preserving wrapper
signature/body/historical defaults/Butt cap. Production uses complete
layout_matrix_elem; do not transfer wrapper defaults into its carrier. Remove
only the never-read final GridGeometry.row_ascent/row_descent storage, retaining
local vectors and every baseline/height formula in measure. Preserve productive
routes/test names/bodies/external APIs/defaults/phase and expectations. ADR-0127
internal continuous flow requires appropriate suites, all-target build/check,
core/test inventory, Clippy and lineage; remove no expectation to pass.

## Named test-wrapper arity exception

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree, captured
2026-10-15T16:39:18.873477+00:00.
`lab/results/construction/diagnostics/too-many-arguments-2026-10-15/before.json.gz`
retains complete source/prior L0/signatures/hashes/UTC/status/git diff HEAD --stat;
classification.json retains individual classifications/clauses read before
decision. Clippy spans were reconfirmed from prior inventory. C120 at
matrix.rs:22, layout_matrix has eight arguments versus threshold seven.
This is Rust structure, not functional failure or new parity evidence; structural
maintenance is a judgment grounded in those data.

The validated clippy-locais-2026-10-16/clippy-inventory.json has 79 warnings.
`lab/results/construction/diagnostics/clippy-excecoes-2026-10-16/before.json.gz`
retains reconfirmed spans/reasons/signatures/hashes plus HEAD/uncommitted tree/
UTC/diffstat. Concrete functions/data were read first. Arity/type heuristics are
mechanics, not new parity or measured cognitive benefit.

### Exclusive exception and materialized scope

C120 is test-only compatibility: this L0 preserves wrapper signature/body/
historical defaults while production uses layout_matrix_elem's complete carrier.
Do not refactor production due to a test-helper warning. Preserve signature/order/
semantics. Apply exactly this attribute only on layout_matrix:

`#[expect(clippy::too_many_arguments, reason="C120: Compatibilidade exclusivamente cfg(test): o L0 manda conservar assinatura/corpo e defaults históricos do wrapper, enquanto produção usa layout_matrix_elem com carrier completo")]`

The Portuguese reason remains exact machine metadata. Confirm workspace/all-targets
Clippy warning and no unfulfilled_lint_expectations; preserve calls/functional
controls. This named permission supersedes generic suppression bans only for
C120/too_many_arguments. Do not apply at module/impl/crate/configuration, raise
global thresholds or cover another lint. A residual exception after dead-argument
removal depends on that correction. If below threshold, remove/review the
expectation; do not silence unfulfilled_lint_expectations. Other correction duties
remain, including conditional C047/C066 fixes without annotation.

The later batch materializes only already justified direct named C120 metadata,
preserving signature/body/calls/cfg/unrelated attributes. Earlier audit wording
left implementation pending; this later metadata scope supersedes it only for
the named expectation, not pending functional fixes. ADR-0127 flow changes no
public contract/executable body/cfg/default/phase/dependency/architecture.
L0 first and restricted resealing; prove that removing new attributes/header
recovers prior rustfmt-normalized source. Measure RED first, then all-targets
inventory: exactly 46 arity and five test warnings satisfied by 49 expectations,
no new warning and unfulfilled_lint_expectations as error; 11 arities targeted
for correction remain visible. Validate build/fmt/strict/reverse lineage/Tekt
without new errors/warnings, existing Counter tests/coverage, under 12 GiB/
zero swap/one job. Prior full suite remains evidence for identical executable
code; no attribute-mirroring test or new performance trial for metadata.

Private signature corrections migrate callers/L0 before code with owner
language/integration/call controls and unchanged fixtures/tolerances. Any new
public contract/external compatibility need requires a concrete ADR-0127 proposal;
this exception does not preauthorize it. Preserve layers/phase/defaults/static
dispatch and no mutable global state.

## Source-transparent matrix partition

### Evidence before decision

HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree;
`lab/results/construction/diagnostics/tekt-v16-math-layout-2026-10-17/before.json.gz`
retains UTC, sources/L0/hashes/configuration, status and exact git diff HEAD --stat;
initial-ten-owners.json.gz preserves the first snapshot. contexts.json,
all-contexts.json and wildcard-inventory.json retain AST/spans from the pinned
CI parser. The previous suite passed on the same input code, reconfirmed by hash.

source-probe-before.json measures inline limits at width 7.2pt versus 12.912pt
with Source. source-family-before.json preserves the first 12 pairs (11
differences). source-family-supplement.json and analysis preserve 23 attributable
pairs (19 differences, 4 equal controls): limits, kerns, Op.text, accents/dotless,
supports, matrix alignment, spacing, sequences, stretch and class.
source-lines-before.json measures the same equation at two lines versus one and
last baseline 16.2 versus 0 under Source despite equal extents. Geometry,
extractable identity and equation metadata are observables; bytes/pointers/Rust
equality are not. The content-source-origin nucleus already requires transparency;
inspection finds helpers classifying wrappers instead of their bodies. Infer no
historical intent or global parity. The supplemental analyzer's 24-versus-23
count error was corrected from valid original stdout without rerunning or
counting the analyzer error as functional RED.

### Exclusive prescription

Normalize cell.source_body() for split_cell_on_align_point classification and
item.source_body() for MathAlignPoint detection. Derived parts retain their own
origin or receive the cell's proven origin through the existing helper, never
a fabricated span. Fallback retains the entire cell.clone(). edge_node borrows
Source.body to find first/last leaf. Preserve signatures/original cells/plan/
alignment/gaps/augment.

Replace only the selected fallback with the current nominal complement of
unconditional arms, preserving bodies/guards/order. Guarded variants remain in
the complement when the guard fails. The attach owner's Option projection is
the sole structural exception: None returns through ?, Some enters a nominal
normalized-Content match without a deep pattern; that remains its own boundary.

L16: matrix.rs:533, split_cell_on_align_point, Content. Exclude only
MathSequence/MathAlignPoint, no guards. Keep the previous fallback outside the
property and broaden classification only by specified Source transparency.

### Acceptance and limits

L0 before code. This ADR-0127 internal parity correction adds no public field/
signature/trait, dependency, product flag/default, compatibility break or phase.
V19 groups share their previous body with per-case justification; notes remain
visible and do not prove execution coverage per alternative. No allow/expect,
threshold increase, @ capture or hidden match in if-let/matches!. Only named
transparency/shallow restructuring changes functional code; do not change
formulas/constants or relax expectations. Require functional Source RED and
structural warning/probe RED before repair; afterwards regression GREEN, zero
emitted-site V16 and future sentinel review of all 36 sites (including new
Content-under-Option projection) using copied Content/Value/MathStyleKind/FrameItem
in a disposable real workspace compiling cfg(test). Current scrutinees/functions/
composition change only as specified; revalidate AST/replay, signatures and other
owners.

Gates: full workspace suite, build, fmt, workspace/all-targets Clippy -D warnings
-D unfulfilled_lint_expectations, Rustdoc -D warnings with four indexes,
strict/reverse V1/V5/V15/V26, full differential SARIF and diff-check. Expected V16
170→152; new V19 only from prescribed groups/new observer; no V20 growth.
Other violations remain pending. Freeze sources during gates; MemoryMax 12 GiB,
zero swap, one Cargo job, heavy commands sequentially. No commit/push/release or
CI pin change. Lint closure does not prove RAM gain, AI-reading benefit or
complete parity.
