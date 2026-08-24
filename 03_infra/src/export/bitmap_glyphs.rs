//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/builder.md
//! @prompt-hash 8554e949
//! @layer L3
//! @updated 2026-07-31
//!
//! **P941** — Glifos bitmap (CBDT/CBLC) como imagens XObject, não fonte embutida.
//! Ver `builder.md` §P941 para o desenho completo.

use std::collections::{BTreeSet, HashMap};

use typst_core::entities::layout_types::PagedDocument;

use super::fonts::{collect_codepoints, collect_glyph_ids, collect_text_codepoints};
use super::images::process_png_for_pdf;

/// Dados de um glifo bitmap extraído da fonte, prontos para XObject.
pub(crate) struct BitmapGlyph {
    /// Payload PDF (RGB comprimido + alpha comprimido opcional).
    pub payload: super::images::PdfImagePayload,
    /// Bearing em pixels (`RasterGlyphImage.x`).
    pub bearing_x: i16,
    /// Bearing em pixels (`RasterGlyphImage.y`).
    pub bearing_y: i16,
    /// Pixels-per-em do strike seleccionado.
    pub pixels_per_em: u16,
}

/// Referência de um glifo bitmap já emitido como XObject, para o stream de página.
#[derive(Clone)]
pub(crate) struct BitmapGlyphRef {
    /// Nome do XObject principal (ex.: `Im7`).
    pub name: String,
    /// Dimensões em pixels do strike.
    pub width: u32,
    pub height: u32,
    /// Bearing em pixels.
    pub bearing_x: i16,
    pub bearing_y: i16,
    /// Pixels-per-em do strike.
    pub pixels_per_em: u16,
}

/// Para cada glyph id do conjunto, devolve o `BitmapGlyph` se a fonte tiver
/// uma imagem raster PNG para ele (`glyph_raster_image` com o maior strike).
/// Glifos sem imagem raster, ou com formato não-PNG, não entram no mapa e
/// seguem o caminho normal de fonte.
pub(crate) fn collect_bitmap_glyphs_for_ids(
    glyph_ids: &BTreeSet<u16>,
    face: &ttf_parser::Face<'_>,
) -> HashMap<u16, BitmapGlyph> {
    let mut map = HashMap::new();
    for &gid in glyph_ids {
        let Some(img) = face.glyph_raster_image(ttf_parser::GlyphId(gid), u16::MAX)
        else {
            continue;
        };
        if img.format != ttf_parser::RasterImageFormat::PNG {
            continue;
        }
        let Ok(payload) = process_png_for_pdf(img.data) else {
            continue;
        };
        map.insert(
            gid,
            BitmapGlyph {
                payload,
                bearing_x: img.x,
                bearing_y: img.y,
                pixels_per_em: img.pixels_per_em,
            },
        );
    }
    map
}

/// Recolhe os glyph ids usados no documento que pertencem à `face` dada.
///
/// Combina `collect_glyph_ids` (glifos shaped) com os codepoints de texto
/// mapeados via `face.glyph_index`, filtrados ao range válido da face — o
/// mesmo critério que o subsetting usa antes de chamar o subsetter.
pub(crate) fn used_glyph_ids_for_face(
    doc: &PagedDocument,
    face: &ttf_parser::Face<'_>,
) -> BTreeSet<u16> {
    let n_glyphs = face.number_of_glyphs();
    let mut ids: BTreeSet<u16> = collect_glyph_ids(doc)
        .into_iter()
        .filter(|&gid| gid < n_glyphs)
        .collect();
    for c in collect_text_codepoints(doc)
        .into_iter()
        .chain(collect_codepoints(doc))
    {
        if let Some(gid) = face.glyph_index(c) {
            if gid.0 < n_glyphs {
                ids.insert(gid.0);
            }
        }
    }
    ids
}
