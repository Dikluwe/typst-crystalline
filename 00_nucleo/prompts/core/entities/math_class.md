# Prompt L0 — `math_class` entity
Hash do Código: 724d41dc

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/math_class.rs`
**References:** ADR-0009, ADR-0011.

## Domain and source

MathClass is the pure typographic classification of Unicode math symbols
(TR25), used for math spacing/operator behavior. Historical override source:
lab/typst-original/crates/typst-utils/src/lib.rs::default_math_class;
base classification: Unicode TR25 through unicode_math_class. Existing
cross-compilation tests at math_class.rs:163-266 document the original check,
not a fresh certification.

## Interface and mapping

MathClass derives Debug/Copy/Clone/Eq/PartialEq/Hash and has 15 variants:
Normal,Alphabetic,Binary,Closing,Diacritic,Fence,GlyphPart,Large,Opening,
Punctuation,Relation,Space,Unary,Vary,Special. Hash supports MathClassOverrideElem.
It mirrors the external variant domain but is a distinct L1 type; public
signatures never expose the external enum.

Public default_math_class(char)->Option<MathClass> applies Typst overrides
first, then unicode_math_class::class, returning None for unclassified input.
Private from_unicode_math_class converts exhaustively. Public
math_class_name(MathClass)->&'static str and parse_math_class(&str)->Option<MathClass>
map lowercase names, except GlyphPart uses "glyph-part"; unknown names reject.
The override table retains upstream Apache-2.0 licensing.

Overrides: ':'→Relation; '.'/'/'/⋯/⋱/⋰/⋮/U+22A5→Normal;
⅋/⟇→Binary; ⎰/⟅→Opening; ⎱/⟆→Closing; Arabic comma U+060C→Punctuation.

## Acceptance and synchronization

Check each explicit example ':' Relation, '.' '/' '⋯' Normal, '⅋' Binary,
'⎰' Opening, '⎱' Closing, '،' Punctuation; delegated 'a' Alphabetic and
'😃' None. Normal equals itself and distinct variants differ; preserve Copy.
All 15 classes roundtrip parse_math_class(math_class_name(c))==Some(c).
GlyphPart maps exactly "glyph-part"; "not-a-class" rejects.

On an explicitly authorized upstream refresh, inspect the typst-utils diff,
update changed overrides and review this contract. New external enum variants
must cause nonexhaustive compilation in the private converter, not a fallback.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/math_class.md`. Historical observations are not a new parity certification.
