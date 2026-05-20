//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/mod.md
//! @prompt-hash 54a226fa
//! @layer L3
//! @updated 2026-04-20

use std::collections::{BTreeSet, HashMap};
use std::io::Write;
use std::sync::Arc;

use flate2::Compression;
use flate2::write::ZlibEncoder;

use ttf_parser::Face;
use typst_core::entities::font_list::FontList;
use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument};

use crate::font_metrics::build_math_glyph_reverse_map;

// Submódulos extraídos em P307b.1 (ADR-0100 / diagnóstico P307a §5).
mod builder;
mod fonts;
mod gradients;
mod images;
mod stream;
use self::builder::PdfBuilder;
use self::stream::{
    build_page_stream, draw_item_local, emit_glyph_pdf, emit_rounded_rect_ops,
    emit_shape_path_local, emit_stroke_paint, emit_text_pdf, line_rg_prefix,
    FontScenario, PageContext,
};
use self::fonts::{
    collect_codepoints, collect_glyph_ids, escape_pdf_string,
    map_chars_to_glyphs, text_to_hex_string, to_unicode_cmap, widths_array,
};
use self::gradients::{
    adaptive_n_for_stops, apply_parent_transform, bezier_control_points_for_arc,
    compute_axial_coords, compute_coons_patches_n_stops,
    compute_coons_patches_n_stops_extended, compute_radial_coords,
    dedup_key_for, emit_conic_coons_stream_cmyk, emit_conic_coons_stream_rgb,
    emit_function_dict, emit_function_dict_cmyk, group_bbox_from_fields,
    multispace_sample_stops, multispace_sample_stops_conic,
    multispace_sample_stops_linear_cmyk, multispace_sample_stops_radial,
    multispace_sample_stops_radial_cmyk, pattern_resources_for_page,
    perceptual_distance_in_space, resolve_relative, rgb_to_cmyk,
    scan_all_gradients,
    DedupKey, GradientObject, GradientObjectKind, PatternRef, RectKey,
};
use self::images::{
    build_jpeg_xobject, build_png_rgb_xobject, build_png_smask_xobject,
    compress_zlib, detect_format, jpeg_color_space, process_png_for_pdf,
    scan_all_images, xobject_resources_for_page,
    ImageFormat, ImageRef, ImageXObject, PdfImagePayload,
};

/// Serializa um `PagedDocument` para bytes PDF-1.7.
///
/// Sem fonte TrueType → fallback para Helvetica Type1 (WinAnsiEncoding, Latin-1).
/// Para suporte Unicode completo, usar `export_pdf_with_font` (ADR-0027).
pub fn export_pdf(doc: &PagedDocument) -> Vec<u8> {
    PdfBuilder::new().build(doc, None)
}

/// Serializa com fonte TrueType embebida — CIDFont + Identity-H (ADR-0027).
/// Suporte Unicode completo para codepoints arbitrários.
/// `font_data`: bytes brutos de um ficheiro `.ttf`/`.otf`.
pub fn export_pdf_with_font(doc: &PagedDocument, font_data: &[u8]) -> Vec<u8> {
    PdfBuilder::new().build(doc, Some(font_data))
}

/// Serializa com **N** fontes TrueType embebidas — multi-font per
/// document (Passo 146, ADR-0055 decisão 5). Cada `FrameItem::Text`
/// emite com a entrada `/F{i+1}` cuja `FontList` casa com
/// `style.font`; spans com `style.font == None` ou sem match
/// caem em `/F1` (font 0) por consistência com o caminho
/// single-font (todos os spans usam o mesmo embedding em
/// `export_pdf_with_font`).
///
/// Se nenhuma das fontes parsear como TTF/OTF válida, fallback
/// para `export_pdf` (Helvetica). Single-font (`fonts.len() == 1`)
/// é caso particular válido.
pub fn export_pdf_multifont(
    doc:   &PagedDocument,
    fonts: &[(FontList, Vec<u8>)],
) -> Vec<u8> {
    if fonts.is_empty() {
        return PdfBuilder::new().build(doc, None);
    }
    let faces: Vec<Face<'_>> = fonts.iter()
        .filter_map(|(_, data)| Face::parse(data, 0).ok())
        .collect();
    if faces.len() != fonts.len() {
        // Algum bytes não parseou — fallback Helvetica.
        return PdfBuilder::new().build(doc, None);
    }
    PdfBuilder::new().build_multifont(doc, fonts, &faces)
}


// ── Testes ─────────────────────────────────────────────────────────────────
//
// Migrados para `tests.rs` em P307b.1 (ADR-0100 + ADR-0037 Regra 5
// Ajuste C). Ficheiro agregador único por categoria Regra 6
// "infraestrutura de testes E2E".

#[cfg(test)]
mod tests;
