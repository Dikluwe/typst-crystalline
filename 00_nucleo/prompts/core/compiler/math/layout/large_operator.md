# L0 — math/layout/large_operator
Hash do Código: 6724f495

**Layer:** L1
**Exclusive consumer:** `01_core/src/compiler/math/layout/large_operator.rs`

## Evidence and classification

At historical vanilla pin `9f2b6e8715237cb086899a42873660fe744622e8`,
crates/typst-library/src/math/ir/resolve.rs:350–352 requests vertical stretch for
Large in Display using StretchInfo::default(); item.rs:1271–1275 sets relative
target 1 and zero shortfall. crates/typst-layout/src/math/fragment/glyph.rs:265–300
retains a sufficient base, chooses the first sufficient variant, uses the largest
insufficient variant without assembly, and assembles available parts when all
variants are insufficient. glyph.rs:452–454 resolves target relative to
display_operator_min_height; text.rs:115–118 centers Large on the axis.

Repository NewCMMath-Book.otf has upem=1000 and normal minimum 1300du; ∑ variants
are 1001/1401du without assembly, ∫ variants 1112/2223du with assembly. A diagnostic
font copy changing only DisplayOperatorMinHeight=3000 exposes both branches:
vanilla PDF uses ∑'s 1401du variant and a three-glyph ∫ assembly. Glyph selection
and extent are observable visual morphology/semantics; GIDs, PDF bytes and frame
structure are not the contract. This ADR-0127 internal parity correction changes
no public contract, phase or product default. It does not establish parity for
all fonts; a font with different vanilla fallback would refute the branch claim.

## Composite operator selection

Own only the Large glyph path under math layout. Keep `MathClass::Large`
identity; do not turn the operator into a delimiter or a textual MathOp.
In Display, use the active font's `display_operator_min_height` without
delimiter short fall. Preserve a sufficient base glyph. Otherwise choose the
first sufficient vertical variant. If none suffices, use the font's vertical
assembly when present; if absent, retain the largest available variant.
Only when neither variants nor assembly exists, use the base glyph. Inline
and script modes keep the base glyph.

For a selected variant, derive width/x-advance from native horizontal advance
and ascent/descent from real ink. Center the resulting real ink on the font's
math axis. For assembly, reuse the existing vertical assembly owner and its
font-supplied parts, overlap, target and axis; do not build a second assembly
algorithm. Avoid centering twice. Keep the same language identity through
glyph or assembly emission. A multi-part Large assembly is one textual
occurrence: wrap its already-positioned visual Glyph parts in the existing
TextReplacement envelope with `text` equal to the source operator character.
Do not infer identity from part GIDs or remap ToUnicode globally. The envelope
adds no advance, width, height, drawing or pipeline phase; variants and other
stretchy constructions retain their existing representation.

Acceptance covers sufficient variant, largest insufficient variant without
assembly, assembly after insufficient variants, no construction and inline.
Use injected metrics for deterministic layout tests and the pinned vanilla
plus modified-font PDF probe as independent evidence. No hardcoded font
height, glyph ID or fallback order belongs in productive logic.
