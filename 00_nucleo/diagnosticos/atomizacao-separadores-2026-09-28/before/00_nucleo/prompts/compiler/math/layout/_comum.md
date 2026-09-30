# L0 — math/layout/_comum

Hash do Código: 4f66513e

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:3cf8b79e22fcda6466dc38b7691a73ada4540956fbed14129c9e1ed5a6a56c43
- 00_nucleo/prompts/_nuclei/math/layout-observables.toml sha256:42a441e59c53fdc5bf619005c0e018eb63251533fb6dc6c16c13d000e592e40c

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/mod.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/mod.rs:24` locates `mod accent;`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

## Core ownership

Own only `01_core/src/compiler/math/layout/mod.rs`: MathLayouter, MathBox,
equation entrypoints, static dispatch, recursive transformations, sequence/grid
composition and external-content bridging. Feature geometry belongs to its
child owner, not to entities. Keep closed static dispatch and exhaustive
FrameItem handling; no registry, vtable or reverse entities-to-compiler import.

Child owners are `00_nucleo/prompts/compiler/math/layout/accent.md`,
`00_nucleo/prompts/compiler/math/layout/assembly.md`,
`00_nucleo/prompts/compiler/math/layout/attach.md`,
`00_nucleo/prompts/compiler/math/layout/callbacks.md`,
`00_nucleo/prompts/compiler/math/layout/cancel.md`,
`00_nucleo/prompts/compiler/math/layout/cases.md`,
`00_nucleo/prompts/compiler/math/layout/delimited.md`,
`00_nucleo/prompts/compiler/math/layout/frac.md`,
`00_nucleo/prompts/compiler/math/layout/matrix.md`,
`00_nucleo/prompts/compiler/math/layout/op.md`,
`00_nucleo/prompts/compiler/math/layout/root.md`,
`00_nucleo/prompts/compiler/math/layout/spacing.md`,
`00_nucleo/prompts/compiler/math/layout/stretch.md`,
`00_nucleo/prompts/compiler/math/layout/stretchy.md`,
`00_nucleo/prompts/compiler/math/layout/underline.md`,
`00_nucleo/prompts/compiler/math/layout/underover.md` and
`00_nucleo/prompts/compiler/math/layout/vec.md`.
Their test consumers have separate owners.

## Pure context and equation entrypoints

MathLayouter is generic over injected FontMetrics; obtain MathConstants from
`metrics.math_constants(style)`, using the active face rather than an
unconditional fallback. Preserve `new(metrics, block, style)` and
`new_with_context(metrics, block, style, equation, region_height,
lexical_styles, callback_pass)`. The contextual constructor supplies the
equation Location, effective region height, causal StyleChain and validated
callback-pass state. The context-free constructor remains available for
existing callers; it is not a substitute for productive contextual layout.

No Func/Engine/World execution occurs here. Each equation's local cancellation
occurrence starts at zero; reserve/increment it once at MathCancel entry,
before descending into its body, including Auto and explicit-angle elements.
Pass that occurrence to cancel; cross lines share it.
Transient math.mid target state is local to the nearest lr scope.

`layout_equation` returns baseline-relative FrameItems.
`layout_equation_measured` returns items and EquationExtent from the same
layout result: width, ascent, descent, line_count and first/last baseline.
Do not introduce a second measuring algorithm or serialize a provisional
callback result as successful compilation. Pipeline realization belongs to
its existing infrastructure owner.

## Baseline and composition

MathBox contains width, ascent, descent and items. Its local y=0 is its own
baseline; y increases downwards. Horizontal concatenation changes x only,
takes maximum ascent/descent and does not top-align siblings. Missing gaps
in hconcat_spaced behave as zero. Offset every supported FrameItem consistently,
including lines, shapes, groups and shaped text.

For axis centering, `shift = axis - (ascent-descent)/2`; add shift to ascent,
subtract it from descent and move items by -shift. Grid content is centered
before merging already-centered delimiters. Fractions, roots and fenced groups
use their child-specific baseline contracts, not blanket axis centering.

Grid delimiter target is `(ascent+descent)*1.1*upem/font_size`, or zero for
nonpositive font size. The 1.1 ratio and delimiter shortfall are separate.
Shared grid helpers measure cells, allocate column widths and position rows
with explicit column/row gaps and alignment-boundary flags. At an ampersand
boundary, class spacing is accounted for in the left cell rather than adding
a normal column gap. Alternating subcolumns align right then left; grids
without alignment points are centered unless a feature specifies alignment.
Row ascent/descent are separately maximized with the synthetic-parenthesis
floor. Accumulate row descent without duplicating a row's ascent.
Multiline equations derive leading from paragraph style rather than
unconditionally using MathConstants.math_leading; retain first/last baselines
for the existing equation integration owner.

Numerator descent by level is Display→Text ×1, Text→Script ×script,
Script→ScriptScript ×sscript/script, ScriptScript→ScriptScript ×1.
Numerators inherit cramped; denominators force cramped. These are discrete
levels, not unconditional multiplication by the script factor.

## Style and morphology preservation

Apply mathematical default glyph transformation once at equation entry.
Single-character eligible Latin/lowercase Greek leaves receive mathematical
italic codepoints; uppercase Greek stays upright. Preserve MathStyled wrappers
for their own handler. Resolve nested setters per independent glyph, size,
bold, italic and cramped axes: the innermost specified setter wins on the same
axis; different axes compose. Explicit false is not absence. Size wrappers
survive glyph transformations; competing size setters are not multiplied.
Bold on a default-italic variable yields bold italic; explicit upright disables
italic. MathOp text is not rewritten into a glyph variant.

Recursive transformations preserve public identity and all causal fields:
attachment slot states; root index and index_explicit; fraction line, optional
style and both deparenthesization bits; limits inline/inline_explicit; accent
mark, size, dotless and presence; cases children, optional delimiter sides,
reverse, gap and presence; matrix rows, alignment, augment, gaps, delimiters
and presence; lr/mid/stretch bodies and explicit fields; every public
under/over spreader identity and optional annotation. Transform children
without baking inherited properties into explicit fields.
Omitted, explicit none, zero, auto and present empty Content are not interchangeable.
Preserve markup containers and semantic styles while descending; do not
flatten a mathematical subtree through plain_text as a transformation.

Dispatch fractions by explicit style > validated math.frac.style > Vertical;
line=false always selects the barless vertical binomial stack. Invalid
validated cascade values fail closed rather than silently choosing a default.
Forward complete accent/cases/matrix/cancel carriers to their geometry owners.
Root index_explicit is geometrically inert. Limits wrappers lay out their body
transparently; attach owns the limit placement decision. Public spreader
lowering is a local layout view, never a rewrite of the public carrier.

## Sequences, text and spacing

Flatten MathSequence. Keep Content::Sequence atomic only when all children are
Text/MathText/MathIdent leaves; open technical sequences with HSpace or
composite math nodes. Weak HSpace collapses at run edges but remains between
material items. Absolute space resolves against current font size; fractional
space is not fabricated as an absolute advance.

A punctuation MathText with terminal whitespace and a subsequent material
sibling contributes its boundary once, through a transient layout/spacing
view; retain the original carrier and preserve terminal whitespace when no
material successor exists. Do not reintroduce normalized glue via Text fallback.

Direct Content::Text is a TextItem: preserve math=true and math_text_item=true
through item/style merges; do not apply glyph-fragment italics correction or
script substitution to it. These flags do not reset size, cramped, font,
fallback, language, direction or features. Other math leaves are not TextItems.
Subtract terminal text glue only for direct Text immediately followed by
MathDelimited. Atomic markup and other boundaries retain their full advance.
Spacing rules belong to the spacing owner; pass active text-space metrics and
suppress class gaps when math_script or Script/ScriptScript size is active.
For a direct single-leaf MathClassOverride, retain glyph-fragment italics
correction unless the glyph has an extensible variant/assembly; do not repeat
that correction for nested composite frames.

Large operators in Display use the applicable display-operator minimum height;
their real ink is centered on the axis in both inline and display modes,
without changing their identity.

## External content boundary

Route Equation/Boxed/Align/Pad/Block and recursive content requiring normal
layout through the existing pure sub-layout bridge. Typed text deltas
(size, fill, weight, font, tracking, leading, lang, bold, italic) also require
that bridge; custom-only style transport and plain markup remain on the
baseline-aligned text path. Preserve lexical/effective style as far as the
existing bridge represents it, without claiming unsupported contextual access.

Use the first declared text/glyph baseline where present; only baseline-less
frames use axis-centered extents. Translate embedded items into local
baseline space; do not invent a top-relative Group wrapper or discard shapes.
If external layout emits nothing but plain_text is nonempty, preserve the
existing textual fallback. Region constraints, introspection, clipping and
metric approximations of this bridge remain documented limitations in the
diagnostic, not new language defaults.

## Acceptance

Observe language morphology, style precedence, baseline/extents and
font-derived geometry with attributable metrics. Preserve complete carriers
through both transformations, including empty-present attachments and optional
fields. Do not accept fixture coordinates, serialized-byte equality,
implementation similarity or historical test totals as parity evidence.
