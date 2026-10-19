# L0 — math/layout/spacing
Hash do Código: 3b2837a6

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/spacing.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/spacing.rs:29` locates `fn base_math_class(content: &Content) -> MathClass`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Mathematical classes and automatic gaps

Own node_math_class, promote_vary, spacing_between and compute_gaps.
Classify named MathIdent from its resolved Unicode symbol, not the first letter
of the name; unresolved identifiers and MathText retain their existing
first-scalar classification. TextItem is Alphabetic; MathOp is Large regardless
of its limits flag. MathStyled and MathLimitsOverride preserve the body's math
class; MathAttach inherits the base's math class. Mathematical underline,
overline and public spreaders preserve their body's boundary classes.
MathClassOverride explicitly supplies its class. MathMid is Relation;
MathPrimes is Normal. Optional matrix/vector delimiters expose Opening/Closing
only on present sides, otherwise Normal. Preserve existing fenced/fence
boundary handling and sequence edge classes.

Space and SpaceFromNewline remain neutral Normal; newline origin does not
trigger textual fallback, CJK normalization or show/regex processing.
First-scalar classification and composite accent/cancel/underover inheritance
limitations are audit items, not a language-wide definition.

Promote Vary to Binary after Normal, Alphabetic, Closing or Fence; otherwise
it stays unary-like without fabricating a Binary gap. Apply ordered spacing
rules (size in em):
1. Before Punctuation: zero; after Punctuation: 1/6.
2. After Opening or before Closing: zero.
3. Relation–Relation: zero; other Relation boundaries: 5/18.
4. Binary boundaries: 2/9.
5. Large boundaries: 1/6 except the established Opening/Fence exclusions.
6. No explicit rule: zero, subject to spaced-item fallback.

Unary has no dedicated override that would defeat a later Binary rule.
Keep an explicit zero rule distinct from absence of a rule. Only absence
allows fallback to the active font's measured text-space advance for spaced
TextItem/atomic textual markup or Fence. Do not hardcode a point width or
trigger fallback from SpaceFromNewline.
Compute n-1 gaps sequentially with Vary promotion; in script size, class
spacing is suppressed, not merely reduced. The caller supplies script state
when math_script or Script/ScriptScript applies. Preserve alignment-boundary
spacing delegation and explicit-zero priority there too.

Acceptance observes class overrides, boundary asymmetry, Large operators,
integrals, symbol names, textual/fence fallback, punctuation and script
suppression using attributable geometry. Generic spaced frames, normal-class
spaced wrappers, per-item mixed sizes and composite inheritance remain
limitations requiring separate evidence; do not encode their defects as defaults.


## Transparent internal-carrier classes

Internal carriers' math class delegates to visual projection, preserving native
spacing rules. Prior evidence:
`lab/results/construction/diagnostics/table-origens-d08b3c2-2026-10-07/baseline.json.gz`,
with HEAD/working tree/UTC/sources/argv/binaries at historical pin
`9f2b6e8715237cb086899a42873660fe744622e8`. Existing ShowRecipe/Located/Realized
branches have the corresponding transparent operation; Source must not leak as
an observable element. Human approval `confirmo`, 2026-10-07, is preserved in the
final `lab/history/construction/records/00_nucleo/materialization/table-origens-d08b3c2-2026-10-07.md`;
the nested contract.md is the pre-approval candidate.

Transparent ShowRecipe.body delegation also accepts Source.body, without
separately executing origin, publishing the carrier or assigning Location.
Preserve classification/preparation/serialization appropriate to this operation,
exclusive ownership and static dispatch. No I/O/new signature/trait/flag or
eval↔layout phase. C2/C/macro tests/collections, builds and lineage remain required.

## Meaningful spacing test and test-wrapper boundary

### Evidence before decision

At HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree,
spacing.rs:807–811,368–390 is the measured control-flow fragment. Clippy inventory/
owner reading distinguishes effectless code, intentional invalid data and empty
checks. `lab/results/construction/diagnostics/controle-fluxo-ci-2026-10-13/before.json.gz`
and companion evidence retain sources/L0/UTC/status/diffstat/commands, without new
parity or upstream-intent inference. spacing.rs:396 is the helper inventory item
in `lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`,
with sources/owners/references/UTC/status/diffstat and the same HEAD/uncommitted
state. Call-site/productive research precedes classification; no new parity or
performance evidence.

### Exclusive responsibility and acceptance

The hierarchy test calls spacing_between and verifies Relation=5/18em,
Binary=2/9em, Large=1/6em at different sizes, rather than merely comparing
constants. Remove only duplicate Equation/Styled fallback patterns and the
Opening/Fence–Fence/Closing rule already covered by the preceding general rule;
preserve priority arms and explicit zero.

Compile internal spacing_between only under `#[cfg(test)]`, retaining signature/
body. Production uses spacing_between_class through compute_gaps. The zero
catchall wrapper stays in class tests; productive Option semantics remain.
Preserve public APIs/productive paths/test names/bodies/defaults/phase/language
expectations. ADR-0127 internal flow requires appropriate suites, all-target
build/check, core/test inventory, Clippy and lineage; remove no expectations.

## Nominal source-transparent spacing projections

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

Normalize content.source_body() in is_textual_spaced_item/is_bare_pipe and each
child.source_body() for textual Sequence eligibility. Preserve classes/gap rules/
explicit-zero priority/Vary promotion/factors/font size. Restore only
classification Source cannot alter.

Replace only the selected fallback with the current nominal complement of
unconditional arms, preserving bodies/guards/order. Guarded variants remain in
the complement when the guard fails. The attach owner's Option projection is
the sole structural exception: None returns through ?, Some enters a nominal
normalized-Content match without a deep pattern; that remains its own boundary.

- L28: spacing.rs:532, is_textual_spaced_item, Content. Exclude only Text/Sequence,
  no guards.
- L29: spacing.rs:541, is_bare_pipe, Content. Exclude only
  MathText/MathIdent/MathStyled/Styled, no guards.

Both retain the previous fallback body outside the property, with classification
expanded only by the specified Source transparency.

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
