# L0 — math/layout/attach
Hash do Código: d73bb2a8

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:cf64af6ef56faaa1ba19fa32ec6c134dc3f4f16a4a74fde21eca40259cf3b6ba

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/attach.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/attach.rs:21` locates `struct PrimesRecipe`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Slots, limits and scripts

Consume the six closed MathAttachSlot states under the pinned slot-presence
nucleus. Only Present(Content) creates a script box and contributes
SpaceAfterScript, including Present(Empty). Omitted and ExplicitNone are
geometrically omitted but remain distinct in the public carrier. Never infer
presence from width, ink, character, text, span or font.

Natural large non-integral operators and limit functions stack attachments
in display; integrals default to side scripts. MathOp's limits flag applies
in display. Explicit MathLimitsOverride uses
`limits && (inline || block)`: limits with inline true stacks even inline,
inline false stacks only display, and scripts never stacks. The wrapper's
inline_explicit bit is morphological, not geometric.

Layout the base once. Postfixed sup/sub share their base-side x origin,
modified by their respective quadrant kerns; their widths are maximized,
not added. Include all top/bottom/left/right slots and pre-script extent in
the final width without counting left extension twice. Preserve existing
script spacing and italics-correction conventions.

Consume signed MathGlyphKern for all four quadrants with the active style;
negative kern is valid. Compute script shifts from font constants, script
ink limits and base-drop limits for non-text-like fragments. Superscript
uses the cramped shift when appropriate; subscript style is cramped.
When both scripts are present, enforce sub_superscript_gap_min and the
superscript-bottom-with-subscript constraint, redistributing shifts as
required rather than fixing them from a fixture.

Limits use:
`top_shift=base.ascent+max(upper_limit_baseline_rise_min,
upper_limit_gap_min+top.descent)`;
`bottom_shift=base.descent+max(lower_limit_baseline_drop_min,
lower_limit_gap_min+bottom.ascent)`.
Emit top at negative shift, bottom at positive shift and derive full extents.

Text-like is a property of the resulting fragment, not merely a Content tag.
Large extensible operators are not text-like even inline. Composite
attach/fraction frames are not text-like; sequences and delimiters aggregate
material children, ignoring immaterial collapsed weak space. Textual MathOp
does not inherit the last shaped glyph's italics correction. For a Large
assembly carried by TextReplacement, consult its visual child glyphs for the
same existing italics-correction path; the occurrence text is not a substitute
for visual glyph metrics. Direct TextItem
has semantic correction zero; mathematical glyph leaves retain metric
correction. Postfixed subscript kern includes minus the base italics
correction; superscript does not receive that compensating term.

## Grouped primes

This owner also lays out MathPrimes, without lowering its public identity.
Counts 1–4 use U+2032/U+2033/U+2034/U+2057. Other counts, including zero,
compose U+2032 with successive offsets `i*prime_width/2` and logical width
`prime_width*(count+1)/2`. Zero is not omission or a fabricated empty slot.
Metric-derived geometry and inherited Normal class remain separate from
attachment merging in evaluation.

Acceptance covers all slots, omitted/none/empty-present differences, limits
and scripts overrides, cramped shifts, signed kerns, common postscript origin,
width/extents and grouped-prime morphology. No dynamic dispatch or code in
the entity owner is authorized.


## Unused internal value boundary

Before decision, attach.rs:688 is the compiler-flagged unused/visibility item.
Before-edit usage/active-dispatch research is preserved in
`lab/results/construction/diagnostics/caminhos-residuais-ci-2026-10-13/before.json.gz`,
with sources/L0, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted tree,
UTC/status/diffstat. This is not new bilateral parity evidence. Remove only
uncalled sub_top and its contradictory comment; preserve simultaneous gap
sup_b.descent+sub_b.ascent, shift_up/shift_down adjustments and every slot.
ADR-0127 internal continuous flow introduces no public signature/default/phase;
preserve language/test expectations and validate all targets/core/build/Clippy/
lineage without warning suppression as a substitute.

## Nominal visual-item projection

### Evidence before decision

F039 is compiler/math/layout/attach.rs:485 (layout_attach). AST spans/patterns/guards/
bodies/complements were checked before code.
`lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree,
UTC/sources/L0/config/hashes/status/git diff HEAD --stat. classification.json
records 70 cases/complements in 26 owners using the pinned CI parser. Pertinent
L0/source were read together, without new architecture or inferred upstream
intent. Rust exhaustiveness is mechanics; preserve product semantics/morphology/
geometry/effects/diagnostics.

### Exclusive prescription

For F039 layout_attach, replace only `_` with nominal TextSeparator/Semantic/Line/
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

## Source-transparent attachment projections

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

At layout_attach entry borrow base.source_body() locally and lay out the base
once. In unwrapped_base normalize b before each match, preserving wrapper
precedence and limits/inline/block conditions. Normalize both MathOp.text match
scrutinees and content_is_text_like recursion. Rewrite only extract_char's
projection as `let c = c?; match c.source_body()` with its two existing cases and
nominal None complement. Do not admit Text as newly eligible script, infer
presence from width/ink or erase Source from stored carriers. Add no field/limit
algorithm or formula/kern/shift changes.

Replace only the selected fallback with the current nominal complement of
unconditional arms, preserving bodies/guards/order. Guarded variants remain in
the complement when the guard fails. The attach owner's Option projection is
the sole structural exception: None returns through ?, Some enters a nominal
normalized-Content match without a deep pattern; that remains its own boundary.

| Site | Function | Scrutinee | Exclude unconditionally | Guarded variants kept in complement |
| --- | --- | --- | --- | --- |
| L04 attach.rs:181 | layout_attach | Content | MathLimitsOverride, MathClassOverride, MathStyled, Styled | none |
| L05 attach.rs:189 | layout_attach | Content | MathLimitsOverride | none |
| L06 attach.rs:199 | layout_attach | Content | MathIdent, MathText, MathOp | none |
| L07 attach.rs:253 | layout_attach | Content | Text, MathText, MathIdent | none |
| L08 attach.rs:255 | layout_attach | Content | MathIdent, MathText, MathOp | none |
| L09 attach.rs:262 | layout_attach | Content | MathIdent, MathText | none |
| L10 attach.rs:281 | layout_attach | Content | MathOp, MathIdent, MathText | none |
| L11 attach.rs:302 | content_is_text_like | Content | MathIdent, MathText, MathOp, MathDelimited, MathSequence, Sequence, MathClassOverride, MathLimitsOverride, MathStyled, Styled | HSpace |
| L12 attach.rs:312 | layout_attach | FrameItem | Glyph | none |
| L13 attach.rs:321 | layout_attach | Content | none | MathIdent, MathText |
| L14 attach.rs:335 | layout_attach | Content | Text, MathText, MathIdent | none |
| L15 attach.rs:337 | layout_attach | Content | MathIdent, MathText, MathOp | none |

All Content complements retain their previous bodies outside the requested
property, with only prescribed Source transparency. Guarded alternatives remain
for rejected conditions. L09 returns None for Option None via ?, then inspects
normalized Content for Some; use no deep Some(Content-alternative) pattern.
L12 seeks only Glyph, preserving presence/count assertions; None is not layout
success. The complete individual alternatives above are evolution obligations,
not a claim of execution coverage.

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
