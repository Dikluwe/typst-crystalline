//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/mod.md
//! @prompt-hash cf0778db
//! @layer L3
//! @updated 2026-09-21

use ttf_parser::Face;
/// **P1468** — reexportado para as assinaturas `_and_pages` das entry
/// points PDF abaixo (mesmo padrão de `StreamMode`/`PdfTags`/
/// `PdfSerializationMode`, definidos localmente neste módulo).
pub use typst_core::entities::document_format_options::PageRanges;
/// **P1469** (escopo reduzido — contrato de linguagem) — reexportado para
/// as assinaturas `_and_standard` das entry points PDF abaixo (mesmo
/// padrão de `pages`/P1468). Diferente de `pages`, não é `Option`: há
/// sempre um valor efetivo, resolvido antes de chegar a este owner
/// (`infra/pipeline.md` P1469).
pub use typst_core::entities::document_format_options::PdfStandards;
#[cfg(test)]
use typst_core::entities::font_list::FontList;
use typst_core::entities::layout_types::{Page, PagedDocument};

// Imports usados por `tests.rs` via `use super::*`.
#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use typst_core::entities::layout_types::FrameItem;

// Submódulos extraídos em P307b.1 (ADR-0100 / diagnóstico P307a §5).
mod bitmap_glyphs;
mod builder;
mod fonts;
mod gradients;
mod html;
mod images;
#[cfg(test)]
mod line_cap_tests;
pub(crate) mod oracle;
pub mod pdf_defaults;
mod pdf_tiling;
mod render;
mod stream;
mod subset;
mod svg;
mod tiling;

use self::builder::PdfBuilder;
use self::fonts::{
    char_to_utf16_hex, collect_codepoints, collect_glyph_ids,
    collect_shaped_cluster_texts, collect_shaped_glyph_mappings, collect_text_codepoints,
    escape_pdf_string, map_chars_to_glyphs, text_to_hex_string, to_unicode_cmap,
    widths_array,
};
use self::gradients::{
    adaptive_n_for_stops, apply_parent_transform, compute_axial_coords,
    compute_radial_coords, emit_conic_mesh, emit_function_dict, emit_function_dict_cmyk,
    emit_function_dict_spot, group_bbox_from_fields, multispace_sample_stops,
    multispace_sample_stops_linear_cmyk, multispace_sample_stops_radial,
    multispace_sample_stops_radial_cmyk, pattern_resources_for_page, resolve_relative,
    scan_all_gradients, spot_bbox_for_shape, DedupKey, GradientObject,
    GradientObjectKind, PatternRef,
};
#[cfg(test)]
use self::gradients::{
    bezier_control_points_for_arc, compute_coons_patches_n_stops,
    compute_coons_patches_n_stops_extended, emit_conic_coons_stream_cmyk,
    emit_conic_coons_stream_rgb, multispace_sample_stops_conic,
    perceptual_distance_in_space, rgb_to_cmyk,
};
#[cfg(test)]
use self::images::process_png_for_pdf;
use self::images::{
    build_icc_profile_stream, build_jpeg_xobject, build_png_rgb_xobject,
    build_png_smask_xobject, compress_zlib, detect_image_format, jpeg_color_space,
    jpeg_is_rgb, scan_all_images, srgb_icc_profile_for_version,
    xobject_resources_for_page, ImageFormat, ImageRef, ImageXObject,
};
use self::stream::{build_page_stream, PageContext};
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
    pub(crate) fn extract_stream_bytes(
        pdf: &[u8],
        obj_id: usize,
    ) -> Option<(bool, Vec<u8>)> {
        let marker = format!("{obj_id} 0 obj").into_bytes();
        let idx = pdf.windows(marker.len()).position(|w| w == marker)?;
        let end = pdf[idx..].windows(7).position(|w| w == b"\nendobj")? + idx;
        let obj = &pdf[idx..end];
        let has_flate = String::from_utf8_lossy(obj).contains("/Filter /FlateDecode");
        let stream_start = obj.windows(8).position(|w| w == b"\nstream\n")? + 8;
        let data_end =
            obj[stream_start..].windows(10).position(|w| w == b"\nendstream")?;
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
        let id_start =
            pdf_str[..obj_marker_start].rfind('\n').map(|i| i + 1).unwrap_or(0);
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

/// **P956** — modo de emissão dos content streams de texto (ADR-0126).
///
/// `Verbose` = modo vanilla-espelhado, novo padrão de produção (envelope
/// `q/cm` + `cs`/`scn` + `Tm` por bloco — ver `stream.md` §P956).
/// `Compact` = formato Passo 20, preservado byte-inalterado atrás da flag
/// `--compact`. Quebra de assinatura deliberada (precedente P113): cada
/// caller declara o modo explicitamente.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StreamMode {
    #[default]
    Verbose,
    Compact,
}

/// Serialização física dos content streams de página, independente da
/// escolha de operadores ([`StreamMode`]) e da estrutura lógica
/// ([`PdfTags`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PdfSerializationMode {
    #[default]
    Compact,
    Pretty,
}

/// Presença da estrutura lógica PDF, independente de `StreamMode` (P1140.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PdfTags {
    #[default]
    Enabled,
    Disabled,
}

/// Serializa um `PagedDocument` para bytes PDF-1.7.
///
/// Sem fonte TrueType → fallback para Helvetica Type1 (WinAnsiEncoding, Latin-1).
/// Para suporte Unicode completo, usar `export_pdf_with_font` (ADR-0027).
pub fn export_pdf(doc: &PagedDocument, stream_mode: StreamMode) -> Vec<u8> {
    export_pdf_with_tags(doc, stream_mode, PdfTags::Enabled)
}

pub fn export_pdf_with_tags(
    doc: &PagedDocument,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
) -> Vec<u8> {
    export_pdf_with_tags_and_serialization(
        doc,
        stream_mode,
        pdf_tags,
        PdfSerializationMode::Compact,
    )
}

pub fn export_pdf_with_tags_and_serialization(
    doc: &PagedDocument,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
) -> Vec<u8> {
    export_pdf_with_document_id_and_tags_and_serialization(
        doc,
        None,
        stream_mode,
        pdf_tags,
        serialization,
    )
}

/// **P617** — variant com `DocumentID` externo.
/// **P956** — `stream_mode` trailing (mesmo padrão das entry points).
pub fn export_pdf_with_document_id(
    doc: &PagedDocument,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
) -> Vec<u8> {
    export_pdf_with_document_id_and_tags(doc, document_id, stream_mode, PdfTags::Enabled)
}

pub fn export_pdf_with_document_id_and_tags(
    doc: &PagedDocument,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
) -> Vec<u8> {
    export_pdf_with_document_id_and_tags_and_serialization(
        doc,
        document_id,
        stream_mode,
        pdf_tags,
        PdfSerializationMode::Compact,
    )
}

pub fn export_pdf_with_document_id_and_tags_and_serialization(
    doc: &PagedDocument,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
) -> Vec<u8> {
    export_pdf_with_document_id_and_tags_and_serialization_and_pages(
        doc,
        document_id,
        stream_mode,
        pdf_tags,
        serialization,
        None,
    )
}

/// **P1468** — variante explícita com seleção de páginas (`pages`,
/// `None` = todas). Mesmo padrão de `StreamMode`(P956)/`PdfTags`(P1140.6)/
/// `PdfSerializationMode`(P1360): novo último parâmetro, variante anterior
/// delega `None` (comportamento byte-idêntico preservado).
/// **P1469** — delega `PdfStandards::default()` para `standard`; usar
/// `..._and_standard` para `--pdf-standard`/`#set format.pdf(standard: ...)`.
pub fn export_pdf_with_document_id_and_tags_and_serialization_and_pages(
    doc: &PagedDocument,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
    pages: Option<PageRanges>,
) -> Vec<u8> {
    export_pdf_with_document_id_and_tags_and_serialization_and_pages_and_standard(
        doc,
        document_id,
        stream_mode,
        pdf_tags,
        serialization,
        pages,
        PdfStandards::default(),
    )
}

/// **P1469** (escopo reduzido — contrato de linguagem) — variante explícita
/// com o conjunto de standards PDF resolvido (`standard`, sempre um valor
/// efetivo — `infra/pipeline.md` P1469 já resolveu a precedência CLI >
/// documento > default antes de chamar este owner).
pub fn export_pdf_with_document_id_and_tags_and_serialization_and_pages_and_standard(
    doc: &PagedDocument,
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
    pages: Option<PageRanges>,
    standard: PdfStandards,
) -> Vec<u8> {
    PdfBuilder::new()
        .with_document_id(document_id)
        .with_stream_mode(stream_mode)
        .with_pdf_tags(pdf_tags)
        .with_serialization(serialization)
        .with_pages(pages)
        .with_standard(standard)
        .build(doc, None)
        .0
}

/// Serializa com fonte TrueType embebida — CIDFont + Identity-H (ADR-0027).
/// Suporte Unicode completo para codepoints arbitrários.
/// `font_data`: bytes brutos de um ficheiro `.ttf`/`.otf`.
pub fn export_pdf_with_font(
    doc: &PagedDocument,
    font_data: &[u8],
    stream_mode: StreamMode,
) -> Vec<u8> {
    export_pdf_with_font_and_document_id(doc, font_data, None, stream_mode)
}

/// **P617** — variant com `DocumentID` externo.
pub fn export_pdf_with_font_and_document_id(
    doc: &PagedDocument,
    font_data: &[u8],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
) -> Vec<u8> {
    export_pdf_with_font_and_document_id_and_tags(
        doc,
        font_data,
        document_id,
        stream_mode,
        PdfTags::Enabled,
    )
}

pub fn export_pdf_with_font_and_document_id_and_tags(
    doc: &PagedDocument,
    font_data: &[u8],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
) -> Vec<u8> {
    export_pdf_with_font_and_document_id_and_tags_and_serialization(
        doc,
        font_data,
        document_id,
        stream_mode,
        pdf_tags,
        PdfSerializationMode::Compact,
    )
}

pub fn export_pdf_with_font_and_document_id_and_tags_and_serialization(
    doc: &PagedDocument,
    font_data: &[u8],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
) -> Vec<u8> {
    export_pdf_with_font_and_document_id_and_tags_and_serialization_and_pages(
        doc,
        font_data,
        document_id,
        stream_mode,
        pdf_tags,
        serialization,
        None,
    )
}

/// **P1468** — variante explícita com seleção de páginas.
/// **P1469** — delega `PdfStandards::default()` para `standard`.
pub fn export_pdf_with_font_and_document_id_and_tags_and_serialization_and_pages(
    doc: &PagedDocument,
    font_data: &[u8],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
    pages: Option<PageRanges>,
) -> Vec<u8> {
    export_pdf_with_font_and_document_id_and_tags_and_serialization_and_pages_and_standard(
        doc,
        font_data,
        document_id,
        stream_mode,
        pdf_tags,
        serialization,
        pages,
        PdfStandards::default(),
    )
}

/// **P1469** (escopo reduzido — contrato de linguagem) — variante explícita
/// com o conjunto de standards PDF resolvido.
#[expect(
    clippy::too_many_arguments,
    reason = "C074: Fachada pública single-font com oito eixos já publicados, incluindo tags, serialização, pages e standard independentes"
)]
pub fn export_pdf_with_font_and_document_id_and_tags_and_serialization_and_pages_and_standard(
    doc: &PagedDocument,
    font_data: &[u8],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
    pages: Option<PageRanges>,
    standard: PdfStandards,
) -> Vec<u8> {
    PdfBuilder::new()
        .with_document_id(document_id)
        .with_stream_mode(stream_mode)
        .with_pdf_tags(pdf_tags)
        .with_serialization(serialization)
        .with_pages(pages)
        .with_standard(standard)
        .build(doc, Some(font_data))
        .0
}

/// Variante instrumentada de `export_pdf_with_font` (P518).
/// Devolve `(pdf_bytes, subset_ms)` onde `subset_ms` é o tempo gasto
/// em `subset_font_with_mapping` para construir os subsets TrueType.
pub fn export_pdf_with_font_and_timings(
    doc: &PagedDocument,
    font_data: &[u8],
    stream_mode: StreamMode,
) -> (Vec<u8>, f64) {
    export_pdf_with_font_and_timings_and_document_id(doc, font_data, None, stream_mode)
}

/// **P617** — variant instrumentada com `DocumentID` externo.
pub fn export_pdf_with_font_and_timings_and_document_id(
    doc: &PagedDocument,
    font_data: &[u8],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
) -> (Vec<u8>, f64) {
    export_pdf_with_font_and_timings_and_document_id_and_tags(
        doc,
        font_data,
        document_id,
        stream_mode,
        PdfTags::Enabled,
    )
}

pub fn export_pdf_with_font_and_timings_and_document_id_and_tags(
    doc: &PagedDocument,
    font_data: &[u8],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
) -> (Vec<u8>, f64) {
    export_pdf_with_font_and_timings_and_document_id_and_tags_and_serialization(
        doc,
        font_data,
        document_id,
        stream_mode,
        pdf_tags,
        PdfSerializationMode::Compact,
    )
}

pub fn export_pdf_with_font_and_timings_and_document_id_and_tags_and_serialization(
    doc: &PagedDocument,
    font_data: &[u8],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
) -> (Vec<u8>, f64) {
    export_pdf_with_font_and_timings_and_document_id_and_tags_and_serialization_and_pages(
        doc,
        font_data,
        document_id,
        stream_mode,
        pdf_tags,
        serialization,
        None,
    )
}

/// **P1468** — variante explícita instrumentada com seleção de páginas.
/// **P1469** — delega `PdfStandards::default()` para `standard`.
pub fn export_pdf_with_font_and_timings_and_document_id_and_tags_and_serialization_and_pages(
    doc: &PagedDocument,
    font_data: &[u8],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
    pages: Option<PageRanges>,
) -> (Vec<u8>, f64) {
    export_pdf_with_font_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard(
        doc,
        font_data,
        document_id,
        stream_mode,
        pdf_tags,
        serialization,
        pages,
        PdfStandards::default(),
    )
}

/// **P1469** (escopo reduzido — contrato de linguagem) — variante explícita
/// instrumentada com o conjunto de standards PDF resolvido.
#[expect(
    clippy::too_many_arguments,
    reason = "C075: Variante pública single-font com tempos conserva os mesmos oito inputs e devolve subset timing"
)]
pub fn export_pdf_with_font_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard(
    doc: &PagedDocument,
    font_data: &[u8],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
    pages: Option<PageRanges>,
    standard: PdfStandards,
) -> (Vec<u8>, f64) {
    let builder = PdfBuilder::new()
        .with_document_id(document_id)
        .with_stream_mode(stream_mode)
        .with_pdf_tags(pdf_tags)
        .with_serialization(serialization)
        .with_pages(pages)
        .with_standard(standard);
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
    fonts: &[FontKey],
    stream_mode: StreamMode,
) -> Vec<u8> {
    export_pdf_multifont_with_document_id(doc, fonts, None, stream_mode)
}

/// **P617** — variant com `DocumentID` externo.
pub fn export_pdf_multifont_with_document_id(
    doc: &PagedDocument,
    fonts: &[FontKey],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
) -> Vec<u8> {
    export_pdf_multifont_with_document_id_and_tags(
        doc,
        fonts,
        document_id,
        stream_mode,
        PdfTags::Enabled,
    )
}

pub fn export_pdf_multifont_with_document_id_and_tags(
    doc: &PagedDocument,
    fonts: &[FontKey],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
) -> Vec<u8> {
    export_pdf_multifont_with_document_id_and_tags_and_serialization(
        doc,
        fonts,
        document_id,
        stream_mode,
        pdf_tags,
        PdfSerializationMode::Compact,
    )
}

pub fn export_pdf_multifont_with_document_id_and_tags_and_serialization(
    doc: &PagedDocument,
    fonts: &[FontKey],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
) -> Vec<u8> {
    export_pdf_multifont_with_document_id_and_tags_and_serialization_and_pages(
        doc,
        fonts,
        document_id,
        stream_mode,
        pdf_tags,
        serialization,
        None,
    )
}

/// **P1468** — variante explícita com seleção de páginas.
/// **P1469** — delega `PdfStandards::default()` para `standard`.
pub fn export_pdf_multifont_with_document_id_and_tags_and_serialization_and_pages(
    doc: &PagedDocument,
    fonts: &[FontKey],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
    pages: Option<PageRanges>,
) -> Vec<u8> {
    export_pdf_multifont_with_document_id_and_tags_and_serialization_and_pages_and_standard(
        doc,
        fonts,
        document_id,
        stream_mode,
        pdf_tags,
        serialization,
        pages,
        PdfStandards::default(),
    )
}

/// **P1469** (escopo reduzido — contrato de linguagem) — variante explícita
/// com o conjunto de standards PDF resolvido.
#[expect(
    clippy::too_many_arguments,
    reason = "C082: Fachada pública multifont preserva a coleção completa com variantes/eixos e opções PDF explícitas"
)]
pub fn export_pdf_multifont_with_document_id_and_tags_and_serialization_and_pages_and_standard(
    doc: &PagedDocument,
    fonts: &[FontKey],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
    pages: Option<PageRanges>,
    standard: PdfStandards,
) -> Vec<u8> {
    let bound_fonts = font_binding::bind_document(doc, fonts);
    let fonts = bound_fonts.as_ref();
    if fonts.is_empty() {
        return PdfBuilder::new()
            .with_document_id(document_id)
            .with_stream_mode(stream_mode)
            .with_pdf_tags(pdf_tags)
            .with_serialization(serialization)
            .with_pages(pages)
            .with_standard(standard)
            .build(doc, None)
            .0;
    }
    let faces: Vec<Face<'_>> = fonts
        .iter()
        .filter_map(|(_, data)| Face::parse(data, 0).ok())
        .collect();
    if faces.len() != fonts.len() {
        // Algum bytes não parseou — fallback Helvetica.
        return PdfBuilder::new()
            .with_document_id(document_id)
            .with_stream_mode(stream_mode)
            .with_pdf_tags(pdf_tags)
            .with_serialization(serialization)
            .with_pages(pages)
            .with_standard(standard)
            .build(doc, None)
            .0;
    }
    PdfBuilder::new()
        .with_document_id(document_id)
        .with_stream_mode(stream_mode)
        .with_pdf_tags(pdf_tags)
        .with_serialization(serialization)
        .with_pages(pages)
        .with_standard(standard)
        .build_multifont(doc, fonts, &faces)
        .0
}

/// Variante instrumentada de `export_pdf_multifont` (P518).
/// Devolve `(pdf_bytes, subset_ms)` onde `subset_ms` é o tempo total
/// gasto em `subset_font_with_mapping` para todas as fontes do documento.
pub fn export_pdf_multifont_and_timings(
    doc: &PagedDocument,
    fonts: &[FontKey],
    stream_mode: StreamMode,
) -> (Vec<u8>, f64) {
    export_pdf_multifont_and_timings_and_document_id(doc, fonts, None, stream_mode)
}

/// **P617** — variant instrumentada com `DocumentID` externo.
pub fn export_pdf_multifont_and_timings_and_document_id(
    doc: &PagedDocument,
    fonts: &[FontKey],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
) -> (Vec<u8>, f64) {
    export_pdf_multifont_and_timings_and_document_id_and_tags(
        doc,
        fonts,
        document_id,
        stream_mode,
        PdfTags::Enabled,
    )
}

pub fn export_pdf_multifont_and_timings_and_document_id_and_tags(
    doc: &PagedDocument,
    fonts: &[FontKey],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
) -> (Vec<u8>, f64) {
    export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization(
        doc,
        fonts,
        document_id,
        stream_mode,
        pdf_tags,
        PdfSerializationMode::Compact,
    )
}

pub fn export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization(
    doc: &PagedDocument,
    fonts: &[FontKey],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
) -> (Vec<u8>, f64) {
    export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization_and_pages(
        doc,
        fonts,
        document_id,
        stream_mode,
        pdf_tags,
        serialization,
        None,
    )
}

/// **P1468** — variante explícita instrumentada com seleção de páginas.
/// **P1469** — delega `PdfStandards::default()` para `standard`.
pub fn export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization_and_pages(
    doc: &PagedDocument,
    fonts: &[FontKey],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
    pages: Option<PageRanges>,
) -> (Vec<u8>, f64) {
    export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard(
        doc,
        fonts,
        document_id,
        stream_mode,
        pdf_tags,
        serialization,
        pages,
        PdfStandards::default(),
    )
}

/// **P1469** (escopo reduzido — contrato de linguagem) — variante explícita
/// instrumentada com o conjunto de standards PDF resolvido.
#[expect(
    clippy::too_many_arguments,
    reason = "C089: Variante pública multifont com tempos preserva os mesmos eixos e o retorno de timing"
)]
pub fn export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard(
    doc: &PagedDocument,
    fonts: &[FontKey],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
    pages: Option<PageRanges>,
    standard: PdfStandards,
) -> (Vec<u8>, f64) {
    let bound_fonts = font_binding::bind_document(doc, fonts);
    let fonts = bound_fonts.as_ref();
    if fonts.is_empty() {
        let (pdf, _) = PdfBuilder::new()
            .with_document_id(document_id)
            .with_stream_mode(stream_mode)
            .with_pdf_tags(pdf_tags)
            .with_serialization(serialization)
            .with_pages(pages)
            .with_standard(standard)
            .build(doc, None);
        return (pdf, 0.0);
    }
    let faces: Vec<Face<'_>> = fonts
        .iter()
        .filter_map(|(_, data)| Face::parse(data, 0).ok())
        .collect();
    if faces.len() != fonts.len() {
        // Algum bytes não parseou — fallback Helvetica.
        let (pdf, _) = PdfBuilder::new()
            .with_document_id(document_id)
            .with_stream_mode(stream_mode)
            .with_pdf_tags(pdf_tags)
            .with_serialization(serialization)
            .with_pages(pages)
            .with_standard(standard)
            .build(doc, None);
        return (pdf, 0.0);
    }
    let builder = PdfBuilder::new()
        .with_document_id(document_id)
        .with_stream_mode(stream_mode)
        .with_serialization(serialization)
        .with_pages(pages)
        .with_standard(standard);
    let builder = builder.with_pdf_tags(pdf_tags);
    let (pdf, subset_ms) = builder.build_multifont(doc, fonts, &faces);
    (pdf, subset_ms)
}

/// **P980** — entrada do oráculo de paridade de operador (ferramenta de
/// diagnóstico, flag `--oracle-pdf`; `prompts/infra/export/oracle.md`).
/// Mesma resolução/dispatch de fontes da emissão normal, mas com as
/// transformações de `oracle.rs` aplicadas aos content streams
/// (`PdfBuilder::with_oracle(true)`). Devolve `(pdf, subset_ms)` como as
/// entradas instrumentadas.
pub fn export_pdf_oracle(
    doc: &PagedDocument,
    fonts: &[FontKey],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
) -> (Vec<u8>, f64) {
    export_pdf_oracle_and_serialization(
        doc,
        fonts,
        document_id,
        stream_mode,
        pdf_tags,
        PdfSerializationMode::Compact,
    )
}

pub fn export_pdf_oracle_and_serialization(
    doc: &PagedDocument,
    fonts: &[FontKey],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
) -> (Vec<u8>, f64) {
    let bound_fonts = font_binding::bind_document(doc, fonts);
    let fonts = bound_fonts.as_ref();
    if fonts.is_empty() {
        return (
            PdfBuilder::new()
                .with_document_id(document_id)
                .with_stream_mode(stream_mode)
                .with_pdf_tags(pdf_tags)
                .with_serialization(serialization)
                .with_oracle(true)
                .build(doc, None)
                .0,
            0.0,
        );
    }
    let faces: Vec<Face<'_>> = fonts
        .iter()
        .filter_map(|(_, data)| Face::parse(data, 0).ok())
        .collect();
    if faces.len() != fonts.len() {
        return (
            PdfBuilder::new()
                .with_document_id(document_id)
                .with_stream_mode(stream_mode)
                .with_pdf_tags(pdf_tags)
                .with_serialization(serialization)
                .with_oracle(true)
                .build(doc, None)
                .0,
            0.0,
        );
    }
    // Mesma regra do dispatch normal (pipeline.rs): uma fonte não-VF vai
    // pelo caminho single-font; VF com eixos ou 2+ fontes vão a multifont.
    if let [((_, font_variant, variations), bytes)] = fonts {
        let is_vf_with_axes = crate::font_variant::is_variable_font(bytes)
            && !crate::font_variant::merge_explicit_variations(
                crate::font_variant::axis_variations_for_font_variant(font_variant),
                variations,
            )
            .is_empty();
        if !is_vf_with_axes {
            return PdfBuilder::new()
                .with_document_id(document_id)
                .with_stream_mode(stream_mode)
                .with_pdf_tags(pdf_tags)
                .with_serialization(serialization)
                .with_oracle(true)
                .build(doc, Some(bytes));
        }
    }
    PdfBuilder::new()
        .with_document_id(document_id)
        .with_stream_mode(stream_mode)
        .with_pdf_tags(pdf_tags)
        .with_serialization(serialization)
        .with_oracle(true)
        .build_multifont(doc, fonts, &faces)
}

// ── Exportação PNG/SVG (P870) ──────────────────────────────────────────────

pub use self::html::{
    export_html, export_html_with_serialization,
    export_html_with_serialization_and_pretty, HtmlSerializationMode,
};

#[cfg(test)]
mod p1293_tests {
    use super::{export_html, export_html_with_serialization, HtmlSerializationMode};

    #[test]
    fn p1293_c_facade_reexporta_api_sem_duplicar_tipo() {
        let _old = export_html;
        let _new = export_html_with_serialization;
        let _mode = HtmlSerializationMode::Crystalline;
        assert!(std::any::type_name::<HtmlSerializationMode>()
            .ends_with("export::html::HtmlSerializationMode"));
    }
}
pub use self::render::{
    render_document_to_png, render_page_to_png, render_page_to_png_with_fonts, FontKey,
    RenderOptions,
};
pub use self::svg::{
    export_svg, export_svg_with_context, export_svg_with_fonts,
    export_svg_with_fonts_and_context, export_svg_with_fonts_and_contexts,
    GlyphFontIdentity, GlyphFontRequest, SvgDestinationContext, SvgGlyphFontContext,
    SvgOptions,
};

/// Exporta uma página para PNG (texto sem fontes resolvidas é omitido).
pub fn export_png(page: &Page, opts: &RenderOptions) -> Vec<u8> {
    render_page_to_png(page, opts)
}

/// Exporta uma página para PNG com fontes resolvidas.
pub fn export_png_with_fonts(
    page: &Page,
    opts: &RenderOptions,
    fonts: &[FontKey],
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

pub(crate) mod font_binding;
