# L0 — core/compiler/math/layout/math_matrix_layout_tests
Hash do Código: 647723cc


**Layer:** L1 (tests)
**Exclusive consumer:** `01_core/src/compiler/math/layout/math_matrix_layout_tests.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/math_matrix_layout_tests.rs:27` locates `struct TestWorld`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Independent matrix tests

Own only `01_core/src/compiler/math/layout/math_matrix_layout_tests.rs`.
Use deterministic metrics and observe horizontal alignment, late MathAlignPoint
splitting, axis-specific relative gaps, every hline/vline offset including
borders/negatives, auto/custom stroke, delimiters, denominator style and extents.
Out-of-dimension offsets are diagnostic, not clamp/ignore.
Preserve the established P825/P912/P919/P923/P945/P1042 controls and
compatibility wrapper. Unattributable frame/font/stroke/geometry is Unknown
and blocks acceptance.

P26 includes unequal alignment-point counts and mixed classes. For asymmetric
`(1 & 2), (33 & 4)`, require
`x(1)+width(1)==x(33)+width(33)` and `x(2)==x(4)`.
Widths come independently from FixedMetrics::text_width at observed item size,
never from grid-produced positions. Centering subcolumns must fail the first
equation.

P27 compares default, 1em and 150% gaps in both axes.
P29 observes thickness, color and cap: auto=0.05em Square; custom/fold
preserves all three. P30 includes optional left/right delimiters in a tall
grid. P31 covers Display, Text/Inline, Script and ScriptScript with denominator
descent. P34 observes automatic Square, custom cap and compatibility Butt
in final FrameItem::Line.

M23 requires duplicate/out-of-order offsets, retaining coordinate quantity and
order; compute interior midpoints independently from cell metrics, while
borders 0/n coincide with complete box bounds.
M25 uses valid/invalid 2x3 and 3x2 matrices: hline validates rows, vline columns.
M26 calls Content::math_matrix directly, requires the complete mat carrier
and compares geometry with the equivalent common route. None of these
assertions promotes a private implementation shape or bytes to a requirement.

## Ratified compact-Value test adaptation

Before decision, value-compacto-memoria5-2026-10-08/before.json.gz and inventory.json
freeze this consumer and every Value::Stroke file:line. sizes.json measures
272→144 bytes in an external Value model, without RSS proof. The public carrier
belongs only to `00_nucleo/prompts/core/entities/value.md`; this unit consumes it
without sharing ownership.

The human confirmed manifest
`6badf07a4f58f08e7c60ad2e34dfb9a14a3a6ba55db90045afe046be8e988237`
with `confirmo` on 2026-10-08. ADR-0127 is satisfied and the former proposed
clauses are in force. Update only stroke-Value construction through From<Stroke>,
preserving all augment geometry expectations/discrimination. Retain existing
oracles, add pertinent RED→GREEN for new isolation behavior, restrict resealing
to this owner and revalidate. No semantic/default/phase expansion.

## Nominal carrier and observation finders

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

Maintain only the specified helper/test or MathSize axis; do not expand traversal
or rewrite scenarios.

Replace only the selected fallback with the current nominal complement of
unconditional arms, preserving bodies/guards/order. Guarded variants remain in
the complement when the guard fails. The attach owner's Option projection is
the sole structural exception: None returns through ?, Some enters a nominal
normalized-Content match without a deep pattern; that remains its own boundary.

- L24: math_matrix_layout_tests.rs:80, find_mat, Content. Exclude only
  MathMatrix/Equation/Styled/MathDelimited/Sequence/MathSequence, no guards.
  Retain the fallback body and only prescribed Source-transparency expansion.
- L25: :128, texts, FrameItem. Exclude only Text/TextShaped, no guards.
- L26: :148, lines, FrameItem. Exclude only Line, no guards.

The latter collectors seek only their named item species; other variants are
not candidates. Preserve presence/count assertions; None is not layout success.

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
