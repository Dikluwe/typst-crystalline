# Prompt: GlyphVariants — Mathematical glyph size variants
Hash do Código: 2255c73a

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/glyph_variants.rs`

## Evidence and domain

OpenType MATH defines size variants for extensible delimiters and radicals.
L3 reads ttf-parser::tables::math::Variants; MathLayouter selects the smallest
sufficient variant. Historical upstream glyph.rs:293 uses font.x_advance for
horizontal positioning. The earlier defect conflated vertical MATH height
with horizontal width. The assembly audit also found ignored
minConnectorOverlap (NewCMMath:20 design units); these are historical source
observations, not fresh current-pin measurements.

## Contract

GlyphVariant has public glyph_id:u16, advance:f64, hor_advance:f64, and
optional top_accent_attach in design units. advance is the MATH measurement
along the stretch axis (height vertically, width horizontally), used only
for select/select_with_advance thresholds. hor_advance is the concrete glyph's
native hmtx horizontal advance, used for FrameItem::Glyph.x_advance and box
width, independently of stretch direction. Stretchy/assembly consumers must
preserve that distinction.

GlyphVariants has public variants:Vec<GlyphVariant>, sorted by increasing
size. select(min_advance) returns the first sufficient glyph_id;
select_with_advance also returns its advance. Both compare only advance and
return None if no variant qualifies. is_empty detects no variants; Default
is empty, allowing base-glyph fallback. Pure domain, no system I/O.

GlyphPart has public glyph_id:u16, start_connector:u16,
end_connector:u16, full_advance:u16, is_extender:bool and hor_advance:f64.
Connectors measure overlap with previous/next parts along the stacking axis;
full_advance is the part's unoverlapped advance on that axis; an extender may
repeat to fill height/width. None substitutes for native horizontal advance.

GlyphAssembly carries public min_overlap:u16 in design units, default 0,
preserving previous synthetic-test behavior. L3 fills it; L1 assembly repeat
resolution and positioning consume it.

The optional top accent attachment belongs to the selected concrete horizontal
variant, not the base Unicode character. L3 reads MathTopAccentAttachment;
when absent it uses (hor_advance + italics_correction)/2. Spreader layout uses
this metric to propagate exact widths through nested compositions.

## Acceptance

For advances [500,800,1200], select(600) selects 800; exact 500 selects 500.
A maximum 500 with target 1000 returns None, as does any selection on empty.
Default is_empty is true. Changing hor_advance cannot change selection;
callers read it separately after selection. Preserve distinct axis measures,
min_overlap default and variant-local attachment transport.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/glyph_variants.md`. Historical observations are not a new parity certification.
