# L0 — math/layout/underover
Hash do Código: 93ae9024



**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/underover.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9


## Consumer evidence

`01_core/src/compiler/math/layout/underover.rs:20` locates `pub(super) fn layout_overbrace(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Spreaders and optional annotations

Own the geometry of MathUnderover and the public over/under brace, bracket,
paren and shell entrypoints. Public identity/presence is preserved by the
core; local lowering does not replace the language carrier.

Glyph pairs (above/below) are brace U+23DE/U+23DF, bracket U+23B4/U+23B5,
paren U+23DC/U+23DD and shell U+23E0/U+23E1. Stretch the support to the body
width at 100% with zero shortfall; an annotation is not the stretch target.
Support glyphs use full-size accent geometry. Optional annotations retain
their absence/empty-present distinction, use superscript/subscript style,
and may widen the final frame without redefining the support's target.
Under annotations are cramped; over annotations inherit the appropriate
superscript style.

For an above support:
`support_y=-base.ascent+min(base.ascent,accent_base_height)`.
For below:
`support_y=base.descent+max(0,support.ink_up)`.
Use signed actual ink extents; do not substitute overbar/underbar constants
or global font ascent. Union support/base ink for compound extents.

Exact-frame-width placement uses attachments:
`pre=support_attach-base_attach`;
`post=(support.width-support_attach)-(base.width-base_attach)`;
`width=max(pre,0)+base.width+max(post,0)`.
If pre<0, base_x=0 and support_x=-pre; otherwise base_x=pre, support_x=0.
Atomic glyphs use applicable TopAccentAttachment; composite bases use
half-width. A support with one glyph uses its attachment; multi-glyph
assembly uses half the assembled width. Preserve the existing symmetric
single-glyph asymmetry compensation
`2*abs(support_attach-support.width/2)` at the support level only, not
again in an annotation wrapper.

Annotations use limit shifts, not tight ink stacking:
under baseline = `base.descent+max(lower_drop_min,
lower_gap_min+annotation.ascent)`;
over baseline = `-(base.ascent+max(upper_rise_min,
upper_gap_min+annotation.descent))`.
The support and annotation paths remain distinguishable; historical generic
one-character dispatch does not redefine an explicit public annotation.

Acceptance covers selected support variants/assemblies, signed ink gaps,
nested supports, exact width/attachments, optional annotations and font-derived
limit placement without hardcoded positions or serialized-byte equality.

## Extractable support identity and internal operations

A one-character support retains that single extractable character in
TextReplacement around visual items even for multipart horizontal assemblies.
.typ/sec_10.typ shows under/overbracket part extraction creating characters absent
from authored content. Replacement changes neither geometry nor annotation.

Support-versus-annotation selection, annotation style, attachment reading and
support text envelope remain distinct private operations within underover.rs.
Final composition preserves the geometric formulas above and applies identity
only to the support's visual items. Add no consumer/API/product mode.

## Nominal source-transparent support projections

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

Normalize Source in piece_char, nested_spreader, base_char_for_attachment and
layout_underover_with_attach's base classification. Preserve support/annotation
selection, public identity, styles, attachment/limit formulas and translation.
A single-character Source support must not become a script-sized annotation.

Replace only the selected fallback with the current nominal complement of
unconditional arms, preserving bodies/guards/order. Guarded variants remain in
the complement when the guard fails. The attach owner's Option projection is
the sole structural exception: None returns through ?, Some enters a nominal
normalized-Content match without a deep pattern; that remains its own boundary.

| Site | Scrutinee | Exclude unconditionally | Guarded alternatives retained in complement |
| --- | --- | --- | --- |
| L32 underover.rs:37 piece_char | Content | none | MathText |
| L33 underover.rs:53 nested_spreader | Content | MathUnderover | none |
| L34 underover.rs:61 base_char_for_attachment | Content | MathAccent | MathText |
| L35 underover.rs:166 spreader_attachment | FrameItem | Glyph | none |
| L36 underover.rs:227 layout_underover_with_attach | Content | MathUnderover | none |

Content fallbacks keep the previous body outside the sought property with only
specified Source transparency. Guarded variants remain for rejected conditions.
The L35 collector seeks only its named species; other variants are not candidates.
Preserve presence/count assertions; None is not successful layout.

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

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.
