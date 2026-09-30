# L0 — math/layout/attach
Hash do Código: 73a3e44c

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:81b492ca5d01377da0b54b6deb21b6cb24b20919009ea3ea21b7350959779715

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/attach.rs`

## Consumer evidence

`01_core/src/compiler/math/layout/attach.rs:21` locates `struct PrimesRecipe`.
This audit locates the existing consumer; it does not establish language parity.
Historical evidence and unresolved discrepancies are archived in
`00_nucleo/diagnosticos/sanitization-remaining-compiler-history.md`.

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
