# L0 — math/layout/stretch
Hash do Código: 126aea6c



**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/stretch.rs`

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/source-origin.toml sha256:059eb35863a413593c98fb7195a521e3d019dadf461433e9c42422218be89065
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9


## Consumer evidence

`01_core/src/compiler/math/layout/stretch.rs:15` locates `fn leaf_char(content: &Content) -> Option<char>`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Public stretch

Own the free function for MathStretchElem, in the render layer rather than
the entity. Preserve body identity and size_explicit. Resolve effective size
by explicit > math.stretch.size > 100%, retaining zero and mixed length as
values rather than absence.

Measure natural body, select its native extensible axis and apply size with
zero shortfall. Percentage references the natural maximum dimension including
attachments; absolute length uses causal font size. Delegate variant/assembly
selection to `00_nucleo/prompts/core/compiler/math/layout/stretchy.md`;
do not replicate MATH tables or continuously scale glyph ink.
Preserve body classes/boundaries. A nonextensible body retains natural layout
and the applicable already-ratified diagnostic; do not invent a new error.

Acceptance covers both axes, attachments, default/absolute/mixed/cascaded
sizes, nesting and fallback. Current leaf-only eligibility and silent
fallbacks do not narrow this approved contract; unresolved implementation/
diagnostic coverage is recorded in the diagnostic.


## Bounded ratification and incomplete composite support

`lab/history/construction/records/00_nucleo/materialization/typst-passo-1446.md:§7`
records explicit owner confirmation of ADR-0127 on 2026-09-18 after proposal
commit `503e6416c78563b0d3949dfb81a58412da79dccb`. Authorization is limited to
that proposal's contract→RED→implementation→adversarial validation→verdict chain,
not universal stretch or general Typst parity. The approved public contract
above is not narrowed by leaf-only implementation; math-stretch-composite remains
open for attachments/compositions with its own metrics and measurement.

## Nominal source-transparent stretch projections

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

Normalize Source for leaf_char eligibility while preserving casts and axis/
variant/assembly/size selection. This does not expand old leaf-only support to
composite bodies or redefine the full native-axis contract. Never ratify the
partial fallback as a normative default or claim full parity.

Replace only the selected fallback with the current nominal complement of
unconditional arms, preserving bodies/guards/order. Guarded variants remain in
the complement when the guard fails. The attach owner's Option projection is
the sole structural exception: None returns through ?, Some enters a nominal
normalized-Content match without a deep pattern; that remains its own boundary.

- L30: stretch.rs:20, leaf_char, Content. Exclude only Text/MathText/MathIdent/
  MathClassOverride/MathLimitsOverride, without guarded variants. Preserve the
  fallback body and broaden classification only by specified Source transparency.
- L31: stretch.rs:34, cascaded_size, Value. Exclude only Relative/Ratio/Length,
  without guards: only these carry size here. Others retain None; this does not
  authorize discarding producer validation errors.

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
