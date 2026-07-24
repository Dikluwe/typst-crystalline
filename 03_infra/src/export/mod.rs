//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/mod.md
//! @prompt-hash bf6cec00
//! @layer L3
//! @updated 2026-07-23

use ttf_parser::Face;
use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::FontList;
use typst_core::entities::font_variations::FontVariations;
use typst_core::entities::layout_types::{Page, PagedDocument};

// Imports usados por `tests.rs` via `use super::*`.
#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use typst_core::entities::layout_types::FrameItem;

// Submódulos extraídos em P307b.1 (ADR-0100 / diagnóstico P307a §5).
mod builder;
mod fonts;
mod gradients;
mod images;
mod render;
mod stream;
mod subset;
mod svg;
use self::builder::PdfBuilder;
use self::fonts::{
    char_to_utf16_hex, collect_codepoints, collect_glyph_ids,
    collect_shaped_cluster_texts, collect_shaped_glyph_mappings, collect_text_codepoints,
    escape_pdf_string, map_chars_to_glyphs, text_to_hex_string, to_unicode_cmap,
    widths_array,
};
use self::gradients::{
    adaptive_n_for_stops, apply_parent_transform, bezier_control_points_for_arc,
    compute_axial_coords, compute_coons_patches_n_stops,
    compute_coons_patches_n_stops_extended, compute_radial_coords, dedup_key_for,
    emit_conic_coons_stream_cmyk, emit_conic_coons_stream_rgb, emit_function_dict,
    emit_function_dict_cmyk, group_bbox_from_fields, multispace_sample_stops,
    multispace_sample_stops_conic, multispace_sample_stops_linear_cmyk,
    multispace_sample_stops_radial, multispace_sample_stops_radial_cmyk,
    pattern_resources_for_page, perceptual_distance_in_space, resolve_relative,
    rgb_to_cmyk, scan_all_gradients, DedupKey, GradientObject, GradientObjectKind,
    PatternRef, RectKey,
};
use self::images::{
    build_icc_profile_stream, build_jpeg_xobject, build_png_rgb_xobject,
    build_png_smask_xobject, compress_zlib, detect_image_format, jpeg_color_space,
    jpeg_is_rgb, process_png_for_pdf, scan_all_images, srgb_icc_profile_bytes,
    xobject_resources_for_page, ImageFormat, ImageRef, ImageXObject, PdfImagePayload,
};
use self::stream::{
    build_page_stream, draw_item_local, emit_glyph_pdf, emit_rounded_rect_ops,
    emit_shape_path_local, emit_stroke_paint, emit_text_pdf, line_rg_prefix,
    FontScenario, PageContext,
};
use self::subset::remap_glyph_id;

/// **P833 (#18)** — re-export para o pipeline: validação de imagens com
/// erro de compilação no formato do vanilla.
pub(crate) use self::images::validate_document_images;

/// Helpers de teste partilhados entre `export::tests` e `integration_tests`.
#[cfg(test)]
pub(crate) use self::test_helpers::{
    extract_object_dict_text, extract_page_content_streams_text, extract_stream_bytes,
};

#[cfg(test)]
pub(crate) mod test_helpers {
    /// Localiza o início e o fim do stream de um objecto PDF.
    pub(crate) fn extract_stream_bytes(pdf: &[u8], obj_id: usize) -> Option<(bool, Vec<u8>)> {
        let marker = format!("{obj_id} 0 obj").into_bytes();
        let idx = pdf.windows(marker.len()).position(|w| w == marker)?;
        let end = pdf[idx..].windows(7).position(|w| w == b"\nendobj")? + idx;
        let obj = &pdf[idx..end];
        let has_flate = String::from_utf8_lossy(obj).contains("/Filter /FlateDecode");
        let stream_start = obj.windows(8).position(|w| w == b"\nstream\n")? + 8;
        let data_end = obj[stream_start..]
            .windows(10)
            .position(|w| w == b"\nendstream")?;
        let data = obj[stream_start..stream_start + data_end].to_vec();
        Some((has_flate, data))
    }

    /// Localiza um objecto PDF pelo ID e devolve o texto do seu dicionário
    /// (antes de `stream`, se existir).
    pub(crate) fn extract_object_dict_text(pdf: &[u8], obj_id: usize) -> Option<String> {
        let marker = format!("{obj_id} 0 obj").into_bytes();
        let idx = pdf.windows(marker.len()).position(|w| w == marker)?;
        let end = pdf[idx..].windows(7).position(|w| w == b"\nendobj")? + idx;
        let obj = &pdf[idx..end];
        if let Some(stream_start) = obj.windows(8).position(|w| w == b"\nstream\n") {
            Some(String::from_utf8_lossy(&obj[..stream_start]).to_string())
        } else {
            Some(String::from_utf8_lossy(obj).to_string())
        }
    }

    /// Extrai todos os content streams de páginas, descomprime se necessário,
    /// e devolve o conteúdo concatenado como string lossy. Usado por testes que
    /// precisam de inspeccionar operadores PDF dentro dos content streams depois
    /// de P884 passar a comprimi-los com FlateDecode.
    pub(crate) fn extract_page_content_streams_text(pdf: &[u8]) -> String {
        let pdf_str = String::from_utf8_lossy(pdf);
        let Some(pages_pos) = pdf_str.find("/Type /Pages") else {
            return String::new();
        };
        let Some(obj_marker_start) = pdf_str[..pages_pos].rfind(" 0 obj\n<<") else {
            return String::new();
        };
        let id_start = pdf_str[..obj_marker_start]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0);
        let Ok(pages_id) = pdf_str[id_start..obj_marker_start].parse::<usize>() else {
            return String::new();
        };

        let Some(dict) = extract_object_dict_text(pdf, pages_id) else {
            return String::new();
        };
        let Some(kids_start) = dict.find("/Kids [") else {
            return String::new();
        };
        let Some(kids_end_rel) = dict[kids_start..].find(']') else {
            return String::new();
        };
        let kids = &dict[kids_start + 7..kids_start + kids_end_rel];

        let mut result = String::new();
        for token in kids.split_whitespace() {
            let Ok(page_id) = token.parse::<usize>() else {
                continue;
            };
            let Some(page_dict) = extract_object_dict_text(pdf, page_id) else {
                continue;
            };
            let Some(contents_start) = page_dict.find("/Contents ") else {
                continue;
            };
            let rest = &page_dict[contents_start + 10..];
            let Some(stream_id_str) = rest.split_whitespace().next() else {
                continue;
            };
            let Ok(stream_id) = stream_id_str.parse::<usize>() else {
                continue;
            };
            let Some((has_flate, bytes)) = extract_stream_bytes(pdf, stream_id) else {
                continue;
            };
            let decompressed = if has_flate {
                let mut decoder = flate2::read::ZlibDecoder::new(&bytes[..]);
                let mut out = Vec::new();
                let _ = std::io::Read::read_to_end(&mut decoder, &mut out);
                out
            } else {
                bytes
            };
            result.push_str(&String::from_utf8_lossy(&decompressed));
        }

        result
    }
}

/// Serializa um `PagedDocument` para bytes PDF-1.7.
///
/// Sem fonte TrueType → fallback para Helvetica Type1 (WinAnsiEncoding, Latin-1).
/// Para suporte Unicode completo, usar `export_pdf_with_font` (ADR-0027).
pub fn export_pdf(doc: &PagedDocument) -> Vec<u8> {
    export_pdf_with_document_id(doc, None)
}

/// **P617** — variant com `DocumentID` externo.
pub fn export_pdf_with_document_id(
    doc: &PagedDocument,
    document_id: Option<[u8; 16]>,
) -> Vec<u8> {
    PdfBuilder::new().with_document_id(document_id).build(doc, None).0
}

/// Serializa com fonte TrueType embebida — CIDFont + Identity-H (ADR-0027).
/// Suporte Unicode completo para codepoints arbitrários.
/// `font_data`: bytes brutos de um ficheiro `.ttf`/`.otf`.
pub fn export_pdf_with_font(doc: &PagedDocument, font_data: &[u8]) -> Vec<u8> {
    export_pdf_with_font_and_document_id(doc, font_data, None)
}

/// **P617** — variant com `DocumentID` externo.
pub fn export_pdf_with_font_and_document_id(
    doc: &PagedDocument,
    font_data: &[u8],
    document_id: Option<[u8; 16]>,
) -> Vec<u8> {
    PdfBuilder::new()
        .with_document_id(document_id)
        .build(doc, Some(font_data))
        .0
}

/// Variante instrumentada de `export_pdf_with_font` (P518).
/// Devolve `(pdf_bytes, subset_ms)` onde `subset_ms` é o tempo gasto
/// em `subset_font_with_mapping` para construir os subsets TrueType.
pub fn export_pdf_with_font_and_timings(
    doc: &PagedDocument,
    font_data: &[u8],
) -> (Vec<u8>, f64) {
    export_pdf_with_font_and_timings_and_document_id(doc, font_data, None)
}

/// **P617** — variant instrumentada com `DocumentID` externo.
pub fn export_pdf_with_font_and_timings_and_document_id(
    doc: &PagedDocument,
    font_data: &[u8],
    document_id: Option<[u8; 16]>,
) -> (Vec<u8>, f64) {
    let builder = PdfBuilder::new().with_document_id(document_id);
    let (pdf, subset_ms) = builder.build(doc, Some(font_data));
    (pdf, subset_ms)
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
    doc: &PagedDocument,
    fonts: &[((FontList, FontVariant, FontVariations), Vec<u8>)],
) -> Vec<u8> {
    export_pdf_multifont_with_document_id(doc, fonts, None)
}

/// **P617** — variant com `DocumentID` externo.
pub fn export_pdf_multifont_with_document_id(
    doc: &PagedDocument,
    fonts: &[((FontList, FontVariant, FontVariations), Vec<u8>)],
    document_id: Option<[u8; 16]>,
) -> Vec<u8> {
    if fonts.is_empty() {
        return PdfBuilder::new().with_document_id(document_id).build(doc, None).0;
    }
    let faces: Vec<Face<'_>> = fonts
        .iter()
        .filter_map(|(_, data)| Face::parse(data, 0).ok())
        .collect();
    if faces.len() != fonts.len() {
        // Algum bytes não parseou — fallback Helvetica.
        return PdfBuilder::new().with_document_id(document_id).build(doc, None).0;
    }
    PdfBuilder::new()
        .with_document_id(document_id)
        .build_multifont(doc, fonts, &faces)
        .0
}

/// Variante instrumentada de `export_pdf_multifont` (P518).
/// Devolve `(pdf_bytes, subset_ms)` onde `subset_ms` é o tempo total
/// gasto em `subset_font_with_mapping` para todas as fontes do documento.
pub fn export_pdf_multifont_and_timings(
    doc: &PagedDocument,
    fonts: &[((FontList, FontVariant, FontVariations), Vec<u8>)],
) -> (Vec<u8>, f64) {
    export_pdf_multifont_and_timings_and_document_id(doc, fonts, None)
}

/// **P617** — variant instrumentada com `DocumentID` externo.
pub fn export_pdf_multifont_and_timings_and_document_id(
    doc: &PagedDocument,
    fonts: &[((FontList, FontVariant, FontVariations), Vec<u8>)],
    document_id: Option<[u8; 16]>,
) -> (Vec<u8>, f64) {
    if fonts.is_empty() {
        let (pdf, _) = PdfBuilder::new().with_document_id(document_id).build(doc, None);
        return (pdf, 0.0);
    }
    let faces: Vec<Face<'_>> = fonts
        .iter()
        .filter_map(|(_, data)| Face::parse(data, 0).ok())
        .collect();
    if faces.len() != fonts.len() {
        // Algum bytes não parseou — fallback Helvetica.
        let (pdf, _) = PdfBuilder::new().with_document_id(document_id).build(doc, None);
        return (pdf, 0.0);
    }
    let builder = PdfBuilder::new().with_document_id(document_id);
    let (pdf, subset_ms) = builder.build_multifont(doc, fonts, &faces);
    (pdf, subset_ms)
}

// ── Exportação PNG/SVG (P870) ──────────────────────────────────────────────

pub use self::render::{
    render_document_to_png, render_page_to_png, render_page_to_png_with_fonts, FontKey,
    RenderOptions,
};
pub use self::svg::{export_svg, export_svg_with_fonts, SvgOptions};

/// Exporta uma página para PNG (texto sem fontes resolvidas é omitido).
pub fn export_png(page: &Page, opts: &RenderOptions) -> Vec<u8> {
    render_page_to_png(page, opts)
}

/// Exporta uma página para PNG com fontes resolvidas.
pub fn export_png_with_fonts(
    page: &Page,
    opts: &RenderOptions,
    fonts: &[((FontList, FontVariant, FontVariations), Vec<u8>)],
) -> Vec<u8> {
    render_page_to_png_with_fonts(page, opts, fonts)
}

// ── Testes ─────────────────────────────────────────────────────────────────
//
// Migrados para `tests.rs` em P307b.1 (ADR-0100 + ADR-0037 Regra 5
// Ajuste C). Ficheiro agregador único por categoria Regra 6
// "infraestrutura de testes E2E".

#[cfg(test)]
mod tests;
