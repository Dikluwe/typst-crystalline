# L0 — math/layout/accent
Hash do Código: cb1303fc



**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/accent.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9


## Consumer evidence

`01_core/src/compiler/math/layout/accent.rs:29` locates `pub(super) fn layout_accent(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Complete accent carrier

Consume MathAccentElem without mutating public Content. Resolve size and
dotless independently by explicit > math.accent cascade > default:
size=100%, dotless=true. Preserve absence and explicit false/zero.
Unicode combining class Below and ⏟/⎵/⏝/⏡ select a lower accent.
Above accents use a cramped base and contextual dotless i/j when enabled;
below accents use a non-cramped base and no dotless rewriting.

Resolve horizontal size against the measured base width, with absolute
length against causal font size. Stretch uses shortfall 0.5em through
`00_nucleo/prompts/core/compiler/math/layout/stretchy.md`, not the vertical
delimiter policy. Preserve the base's identity and source presence metadata.

Above placement is baseline-relative:
`accent_y = -base.ascent + min(base.ascent, accent_base_height)`;
`ascent = max(base.ascent, -accent_y + accent.ascent)`.
Keep base descent. Below placement puts the accent beneath the base and
includes its lower ink extent. Attachment, not arbitrary fixture centering,
determines x: `base_attach - accent_attach`, with the existing metric
fallbacks for composite bases. The accent must not erase nested accent extent.

Acceptance distinguishes orientation, size 100%/150%, dotless on/off for i/j,
attachment and nested baseline geometry, using attributable font metrics.
The current oversized-accent width heuristic and incomplete attachment
attribution are audit limitations, not new normative exceptions.

## Extractable accent identity

The .typ/sec_10.typ and .typ/sec_12.typ probes in typs-teste-2026-09-30 show
pdftotext -layout separating/reversing combining marks and bases while language
preserves base+mark. For a realized single-character text leaf with a combining
accent, wrap positioned visual items in one TextReplacement containing base
then mark per occurrence. Composite bases retain their items without invented
text serialization. The envelope changes no size/offset/font/drawing.

Post-correction dot(dot(x)) extracted mark+base+mark with an inner nested
envelope. For nested accents on an atomic textual base, compose all marks from
inner to outer and use only one outer envelope. Inner envelopes are transparent
only on that path, retaining all visual children.

## Internal responsibility separation

Within accent.rs, keep size/dotless resolution, mark orientation, geometric
placement and text-identity composition as distinct private operations.
Apply extractable identity after positioning, without metric changes.
This creates no consumer/API/new behavior.

## Narrow unused-helper maintenance

### Evidence before decision

accent.rs:29 is the historical Clippy inventory item at HEAD
`06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree.
`lab/results/construction/diagnostics/helpers-testes-ci-2026-10-13/before.json.gz`
retains sources/owners/references/UTC/status/diffstat. Call-site/productive-path
research precedes classification; no new parity/performance evidence.
accent.rs:153 is the unused/visibility item whose before-edit usages/dispatchers
are in `lab/results/construction/diagnostics/caminhos-residuais-ci-2026-10-13/before.json.gz`,
with the same HEAD/uncommitted state, sources/L0/UTC/status/diffstat; no new
bilateral parity evidence.

### Exclusive prescription and acceptance

Compile internal layout_accent only under `#[cfg(test)]`, preserving signature/
body. Productive dispatch uses layout_accent_elem; the wrapper's historical
natural-size/dotless-false scenario is test-only. Remove only unused duplicate
difference dx; preserve accent_x=base_attach-accent_attach, all attachment queries,
geometry and identity. Preserve productive routes/test names/bodies/external APIs/
defaults/phase/language expectations. ADR-0127 internal continuous flow requires
all workspace-target build/check, core suite, test inventory, Clippy and lineage;
remove no expectations or suppress warnings instead of fixing.

## Nominal source-transparent accent projections

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

Borrow-normalize Source in extract_base_char, one_char and atom_identity
scrutinees. dotless_base transforms the normalized body and reapplies its first
origin via existing source_span/with_source_span. Preserve explicit/size/dotless,
fields/styles and other arm bodies. This makes Source-wrapped i/j dotless while
retaining one base+marks textual occurrence; never measure drawing from text.

Replace only the selected fallback with the current nominal complement of
unconditional arms, preserving bodies/guards/order. Guarded variants remain in
the complement when the guard fails. The attach owner's Option projection is
the sole structural exception: None returns through ?, Some enters a nominal
normalized-Content match without a deep pattern; that remains its own boundary.

| Site | Scrutinee | Exclude unconditionally | Guarded alternatives retained in complement |
| --- | --- | --- | --- |
| L01 accent.rs:131 extract_base_char | Content | MathAccent | MathText |
| L02 accent.rs:136 layout_accent_resolved | Content | none | MathText |
| L03 accent.rs:241 atom_identity | Content | none | MathText, MathIdent, MathAccent |

Keep previous fallback bodies outside the sought property, expanding only the
specified Source transparency. Guarded variants remain in the complement when
the condition is rejected.

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
