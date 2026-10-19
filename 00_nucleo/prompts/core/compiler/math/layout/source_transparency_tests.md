# L0 — core/compiler/math/layout/source_transparency_tests
Hash do Código: 37d31c0e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065

**Layer:** L1 test-only
**Exclusive consumer:** `01_core/src/compiler/math/layout/source_transparency_tests.rs`

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

## Exclusive test contract

Own only pure mathematical Source-transparency regressions. Use MathLayouter's
public entry and deterministic FontMetrics with declared advances/edges,
TopAccentAttachment, signed kerns and horizontal variants. Load no font/World/
clock/file/network. Keep fixtures attributable; they do not prove real-font
selection or complete export parity.

The supplemental 23 pairs are separate cases. Compare width/ascent/descent,
baselines, text/glyph positions/sizes at 1e-9, observable text/character and
extractable per-occurrence identity, without bytes/GID/pointers/private frame
shape/Rust Content equality. The observer knows Text/TextShaped/Glyph,
TextReplacement and Semantic; all six other FrameItem kinds are explicit Unknown
failures, never fabricated observations or ignored items. Text/TextShaped share
visual observation and six refused kinds share the same failure, justifying only
those two V19 groups.

Independent positives retain inline limits stacked/scripts lateral; the mock
has extensible variants with distinct advances; matrix ampersand alignment stays;
internal weak spaces remain and edge spaces collapse. Source preserves atomic/
nested-accent extractable identity and dotless i/j. Never measure glyphs from
identity text.

Add display two-line equation control: Source keeps line_count 2 and first/last
baseline. Add flatten control: fragments retain their own origin or receive a
proven container origin, without replacing known origins or changing author trees.
A negative TextSeparator observer control must fail Unknown. Dotless
transformation has its own co-located causal/origin test under the accent owner;
export no helper just for this consumer and share no productive ownership.

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

## Source markers in the grid

source-markers-before.json confirms Source(Linebreak) turns two lines into one
(a/b on one baseline); Source(MathAlignPoint) keeps two lines but displaces 1/2
relative to 333/4's column. Preserve this RED before repair. Parent-owned
needs_grid_layout/partition_grid classify markers through node.source_body()
without changing separators, empty lines/cells or order. partition_grid's content
arm retains the original clone/origin. Use a flat nominal complement if removing
the prior normalized binding, without @ capture or suppression. New origin never
fabricates Location. This regression consumer adds two public display tests for
Source Linebreak/MathAlignPoint requiring equal baselines/positions. Previous
27 controls remain, total 29; expected RED is 5 passing/24 failing. No new
contract/phase/default/public API.

## Borrowed origin control

Strict Clippy reports cloned_ref_to_slice_refs in the flatten control at
source_transparency_tests.rs:378; attempt-clippy/ and clippy-refinement.json retain
exact candidate sources/receipt. The one-item input only borrows container;
use std::slice::from_ref(&container) rather than a temporary cloned slice, with
identical data/spans/assertions/observations/test count. Add no suppression/API.
