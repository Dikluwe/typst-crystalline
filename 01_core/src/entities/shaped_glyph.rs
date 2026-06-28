//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/shaped_glyph.md
//! @prompt-hash af10be23
//! @layer L1
//! @updated 2026-06-27

/// **P482** — Glifo com shaping real via rustybuzz.
/// Campos em unidades de fonte; converter para pt no export:
/// `advance_pt = x_advance as f64 / units_per_em * font_size_pt`.
#[derive(Debug, Clone, PartialEq)]
pub struct ShapedGlyph {
    /// ID do glifo na fonte (índice na tabela de glifos).
    pub glyph_id:  u16,
    /// Advance horizontal em unidades de fonte.
    pub x_advance: i32,
    /// Offset horizontal (kerning, marks).
    pub x_offset:  i32,
    /// Offset vertical (diacríticos).
    pub y_offset:  i32,
    /// Índice byte no string original (ToUnicode CMap).
    pub cluster:   u32,
    /// Codepoint Unicode derivado de `cluster`.
    pub char_code: char,
}
