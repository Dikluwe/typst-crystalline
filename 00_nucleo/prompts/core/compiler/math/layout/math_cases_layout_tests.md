# L0 — core/compiler/math/layout/math_cases_layout_tests
Hash do Código: 4678d861


**Layer:** L1 (tests)
**Exclusive consumer:** `01_core/src/compiler/math/layout/math_cases_layout_tests.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/math_cases_layout_tests.rs:23` locates `struct TestWorld`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Independent cases tests

Own only `01_core/src/compiler/math/layout/math_cases_layout_tests.rs`.
Observe attributable FrameItems with deterministic metrics.
Verify one initial row per child, late MathAlignPoint splitting, denominator
style, left/right delimiter chosen by reverse, omitted None sides,
relative gap resolved against outer style and explicit > cascade > default
without carrier changes.

Retain regression controls for natural alignment, one axis-centering operation,
no redundant padding, baseline-relative composition and full delimiter extents
(the established P919/P923/P945/P1042/P1132 behaviors).
Acceptance observes geometric relationships and frame morphology, not bytes.

Unattributable font, carrier, StyleChain, glyph, frame, metric or geometry is
Unknown; fail closed and do not count it as killing a mutant.

## Delimiter occurrence observation

After cases wraps the visual delimiter in TextReplacement, direction/extents
still measure child Text/TextShaped coordinates. The observer crosses the
envelope without using extractable text as geometry. Integrated RED: four
opening/closing brace observations no longer found top-level child items.

## Nominal FrameItem observer projection

### Evidence before decision

F042 is compiler/math/layout/math_cases_layout_tests.rs:128 (visit). AST spans/patterns/guards/
bodies/complements were checked before code.
`lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree,
UTC/sources/L0/config/hashes/status/git diff HEAD --stat. classification.json
records 70 cases/complements in 26 owners using the pinned CI parser. Pertinent
L0/source were read together, without new architecture or inferred upstream
intent. Rust exhaustiveness is mechanics; preserve product semantics/morphology/
geometry/effects/diagnostics.

### Exclusive prescription

For F042 visit, replace only `_` with nominal TextSeparator/Semantic/Line/Glyph/
Image/Shape/Group/Link alternatives using the in-scope FrameItem
prefix. Preserve the complete fallback body, prior arms, guards, transformations,
calls, returns and order.

Preserve callbacks/effects/metadata/clipping/fonts/positions/ink/width under their
current owners. Do not expand traversals, turn wrappers into leaves or substitute
text/ActualText for geometry. Test helpers retain fixtures/expected results/
tolerances/counts/Unknown failures; missing observation is not acceptance.
Nominal groups share the previous body; keep V19 notes without equating arm
coverage with every alternative.

### Acceptance and limits

ADR-0127 internal flow adds no enum/field/public trait method/signature/default/
dependency/compatibility/phase. Do not materialize unrelated pending proposals.
L0 first and restricted bilateral resealing. No suppressions, threshold changes
or match-to-if-let rewrites to hide V16.

Before code run the new FrameVisitor control and workspace suite on unchanged
expectations and confirm 70-family-warning RED by AST inspection. Afterwards
require zero selected-family V16 and no new warning; external V16 stays open.
For productive-core matches reached by cargo check -p typst-core, a disposable
copy with sentinel FrameItem measures absent E0004 before/present after. Claim
coverage only for compiled sites, not inaccessible cfg(test)/L3. Replay checks
all patterns/guards/bodies, unchanged enum/signatures and real builds/tests;
no stubbed bodies.

Validate full workspace/build, workspace/all-targets Clippy -D warnings
-D unfulfilled_lint_expectations, fmt, Rustdoc -D warnings with four HTML indexes,
strict/reverse V1/V5/V15/V26 lineage, SARIF and diff --check. Freeze sources/L0/
configuration during heavy sequential commands under MemoryMax 12 GiB,
MemorySwapMax0/CARGO_BUILD_JOBS1. Structural sanitation proves neither global
parity nor RAM/AI-reading benefit.

## Nominal carrier finder

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

Maintain only the named helper/test or MathSize axis; do not expand traversal or
rewrite existing scenarios.

Replace only the selected fallback with the current nominal complement of
unconditional arms, preserving bodies/guards/order. Guarded variants remain in
the complement when the guard fails. The attach owner's Option projection is
the sole structural exception: None returns through ?, Some enters a nominal
normalized-Content match without a deep pattern; that remains its own boundary.

L23: math_cases_layout_tests.rs:81, find_cases, Content. Exclude only
MathCases/Equation/Styled/MathDelimited/Sequence/MathSequence, without guards.
Preserve the fallback body outside that property; broaden classification only
by prescribed Source transparency.

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
