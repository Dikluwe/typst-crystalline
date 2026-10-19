# L0 — math/layout/underline
Hash do Código: 08f6bbc3


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/layout-observables.toml sha256:b1f5e8d937f80e207e7cb5f1cd7ee03f3a3eebc89af5bfc0e13ebd95c615ecd0

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/underline.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/underline.rs:18` locates `pub(super) fn layout<M: FontMetrics>(`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`lab/results/construction/diagnostics/sanitization-remaining-compiler-history.md`.

## Mathematical underline and overline

Own free functions for the distinct MathUnderline/MathOverline identities.
The core only delegates; textual decorations have separate owners.
Use font-derived MATH constants, current fill and baseline-relative geometry,
without changing the public body's morphology.

Underline lays out the body without forcing cramped. Line thickness, gap and
extra descender use underbar_* constants. The line center is
`body.descent+gap+thickness/2`.
Preserve logical body width; line width alone is
`max(0,body.width-italics_correction(body))`. Positive correction shortens,
zero preserves and negative correction extends outside the box without
enlarging it. Do not clamp protruding ink to a root frame or use ink width as
the logical width. Descender includes gap, thickness and extra descender.

Overline lays out the body cramped and uses overbar_* constants. Its rule
center is `-(body.ascent+gap+thickness/2)`; width equals body logical width,
without underline's italics subtraction. Include the extra ascender.
Keep body baseline coordinates consistent; no additional fixture-derived
translation or textual decoration is required.

Both retain body's mathematical boundaries/properties; the line does not
manufacture class, text-like state or italics correction.
Acceptance covers above/below placement, width versus rule width, positive/
zero/negative correction, cramped behavior and full vertical extents.


## Nominal FrameItem projection

### Evidence before decision

F050 is compiler/math/layout/underline.rs:57 (layout). AST spans/patterns/guards/
bodies/complements were checked before code.
`lab/results/construction/diagnostics/tekt-frame-items-2026-10-16/before.json.gz`
retains HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working tree,
UTC/sources/L0/config/hashes/status/git diff HEAD --stat. classification.json
records 70 cases/complements in 26 owners using the pinned CI parser. Pertinent
L0/source were read together, without new architecture or inferred upstream
intent. Rust exhaustiveness is mechanics; preserve product semantics/morphology/
geometry/effects/diagnostics.

### Exclusive prescription

For F050 layout, replace only `_` with nominal TextReplacement/TextSeparator/
Semantic/Line/Image/Shape/Group/Link alternatives using the in-scope FrameItem
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
