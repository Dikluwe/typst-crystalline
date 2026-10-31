# L0 — Mathematical node wrapper layout
Hash do Código: 4d2d2f89

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/node_wrappers.rs`

## Evidence before decision

At HEAD ea7ae25ab (clean tree, 2026-09-30),
compiler/math/layout/mod.rs:1070–1104,1119–1193 contains the two nontrivial node
wrapper arms. This evidence establishes the extraction boundary, not performance.

## Contract

The class wrapper preserves glyph advance and italic correction. The style
wrapper resolves internal axes, applies the existing transformation and lays out
the resulting body. The central match stays exhaustive/static. Preserve public
signatures, defaults, pipeline phase and purity. L1 tests/build/crystalline-lint
must pass; infer no performance benefit.

## Nominal MathSize projection and source transparency controls

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

Maintain only the helper/test or MathSize axis named below; do not expand
traversal or rewrite existing scenarios.

Replace only the selected fallback with the current nominal complement of
unconditional arms, preserving bodies/guards/order. Guarded variants remain in
the complement when the guard fails. The attach owner's Option projection is
the sole structural exception: None returns through ?, Some enters a nominal
normalized-Content match without a deep pattern; that remains its own boundary.

L22: node_wrappers.rs:102, layout_styled_node, MathStyleKind. Exclude only
Display/Inline/Script/SScript; there are no guarded alternatives. Glyph setters
do not change MathSize; only those four variants do. The empty body is the
independent axis's no-op.

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


## Explicit Large single-glyph axis and correction

### Measurement before decision

lab/results/construction/diagnostics/readiness-fixes-2026-10-28/
math-class-large-before/receipt.json retains twelve actual sources, UTC,
HEAD/dirty stat, immutable699b6e88 build/source and current oracle64c5e8c0.
class-large-findings-before-decision.json records original docs-class's single
remaining union y delta0.46201pt, also present in inline/direct/scripting controls.
Large x inline differs0.37952pt; display and multiglyph controls coincide; quoted
Large x display differs0.088pt in x. node_wrappers.rs:14 never centers class
Large and still adds nonextended italic correction. Pinned ir/item.rs:243–260
assigns explicit class to a Glyph; typst-layout/math/text.rs:115–120 centers
Large on the font axis; glyph.rs:328 defines centering. Visible single-glyph
class behavior is language/render meaning; internal frame structures are free.

### Exclusive correction and limits

For the existing recognized single-math-leaf body of class Large, use the
existing apply_axis_offset helper at the same layout phase and retain body
height/paint/Source. Preserve all existing italic correction/width and extended
checks for every class, including Large.
Multi-glyph/unrecognized framed bodies keep their prior path. Preserve all
other style-axis/size/Source/width behavior. This bounded clause supersedes only historical no-class-geometry wording;
all-class IC preservation and prior axis rules remain intact.
No new public field/method/signature, stretch helper, font selection, default
mode or phase is introduced. In particular general Large Display stretching
and Equation-wrapper recognition remain open; do not broaden leaf eligibility.

Require physical RED for axis/paint baseline; keep Large and ordinary class IC
advance and multiglyph frames as independent unchanged controls. GREEN, real bilateral inline/display/
script/original-doc and glyph/style controls precede final qualification.
Keep quoted-bold, other framed cases and other existing negatives explicit.
Continuous ADR-0127 L0-first; full twelve gates/history/JPEG original criteria.


### Extra source check refutes the initial IC hypothesis

Before any residual RED or implementation, pinned glyph.rs:210–218 was checked:
italic correction is added whenever the glyph is not extended, independent of
Large. The initial inference to suppress Large IC was therefore wrong, and its
unexecuted L0/test draft is archived in class-large-unrun-draft-refuted.json.gz.
Retain the already observed quoted-Large string x difference as an open negative;
its cause is not established. No productive implementation ever adopted that
hypothesis. The correction is limited to existing-leaf axis centering.


## Prose counterexample narrows Large glyph eligibility

### Measurement before decision

lab/results/construction/diagnostics/readiness-fixes-2026-10-28/
math-private-visual-delta2.json compares934 prior criteria after actual375-case
build2/releasefeffd641/source. One case (#math.class("large", [x])) regresses
three previously passing criteria:0.3795pt,262PDF/244SVG pixels. Its unchanged
fixture was exact in build1. In contrast original docs-class now passes boxes/
PDF after Symbol centering (SVG16pixels remains negative). Pinned ir/item.rs:
243–260 updates the explicit class on MathKind::Glyph; text kind retains its own
class in typst-layout/math/text.rs:104–120. Treating every existing single-leaf
carrier as a mathematical Symbol was too broad. class-large-prose-regression-
before.json.gz retains the exact failed source/L0 and measured delta authority.

### Exclusive repair and acceptance

Narrow the newly added centering to an existing native Content::Symbol with one
recognized scalar, transparently under Source. Preserve all prior IC/extended
logic and every prior Text/MathText/MathIdent/multiglyph/framed baseline. No
string/font-name/glyph-id/fixture test is allowed. The representation expresses
mathematical Symbol versus prose Text, not a Rust equality acceptance criterion.
This replaces only the overly broad new centering predicate. Do not broaden
legacy leaf eligibility or general Display stretching in this repair.

Add a meaningful prose Text baseline control and require it to fail against
build2 implementation before narrowing. Retain the already green Symbol axis,
IC and multiglyph tests. Then full core, source-bound bilateral unchanged
#math.class("large",[x]) plus original docs and all prior375 criteria with no
lost positive. Keep extraction grouping/raster residues explicitly negative.
Internal parity repair remains continuous ADR-0127 without new public/default/
phase/compatibility API. L0 first and all final original gates remain binding.
