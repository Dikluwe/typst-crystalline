//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/builder.md
//! @prompt-hash 66f1c615
//! @layer L3
//! @updated 2026-07-08
//!
//! `PdfBuilder` — orquestrador L3 que constrói o ficheiro PDF
//! agregando objects, xref, trailer. Três caminhos: Helvetica
//! (Type1 fallback), CIDFont (Identity-H), Multifont.
//!
//! Extraído de `export.rs` em P307b.1 (ADR-0100 / diagnóstico
//! P307a §5). Conteúdo bit-exact pré e pós migração.
//!
//! Depende de `super::` para os submódulos extraídos
//! (fonts, gradients, images, stream).

use std::collections::{BTreeSet, HashMap, HashSet};

use ttf_parser::Face;
use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::FontList;
use typst_core::entities::layout_types::{FrameItem, LinkTarget, PagedDocument, Point, Size};

use crate::font_variant::{axis_variations_for_font_variant, instantiate_variable_font};
use ecow::EcoString;

use super::{
    adaptive_n_for_stops, apply_parent_transform, build_icc_profile_stream,
    build_jpeg_xobject, build_page_stream, build_png_rgb_xobject,
    build_png_smask_xobject, collect_codepoints, collect_glyph_ids,
    collect_shaped_cluster_texts, collect_shaped_glyph_mappings,
    collect_text_codepoints, compute_axial_coords, compute_radial_coords,
    detect_format, emit_conic_coons_stream_cmyk, emit_conic_coons_stream_rgb,
    emit_function_dict, emit_function_dict_cmyk, jpeg_color_space, jpeg_is_rgb,
    map_chars_to_glyphs, multispace_sample_stops,
    multispace_sample_stops_linear_cmyk, multispace_sample_stops_radial,
    multispace_sample_stops_radial_cmyk, pattern_resources_for_page,
    resolve_relative, scan_all_gradients, scan_all_images,
    srgb_icc_profile_bytes,
    subset::{remap_glyph_id, subset_font_with_mapping, FontSubset},
    char_to_utf16_hex, to_unicode_cmap, widths_array,
    xobject_resources_for_page, GradientObject,
    GradientObjectKind, ImageFormat, ImageXObject, PageContext,
};

use crate::font_metrics::build_math_glyph_reverse_map;

/// **P777** — verdadeiro se o documento contiver pelo menos um JPEG RGB.
fn has_rgb_jpeg(doc: &PagedDocument) -> bool {
    fn walk(items: &[FrameItem]) -> bool {
        for item in items {
            match item {
                FrameItem::Image { data, .. } => {
                    if detect_format(data) == ImageFormat::Jpeg && jpeg_is_rgb(data) {
                        return true;
                    }
                }
                FrameItem::Group { items: child_items, .. }
                | FrameItem::Link { items: child_items, .. } => {
                    if walk(child_items) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        false
    }

    doc.pages.iter().any(|p| walk(&p.items))
}

fn duration_ms(d: std::time::Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

/// **P611** — devolve o timestamp a usar em `/Info` e no XMP.
/// P601 — em testes de snapshot, permitir congelar a data para manter
/// os PDFs de referência determinísticos.
fn current_pdf_timestamp() -> time::OffsetDateTime {
    std::env::var("CRYSTALLINE_PDF_FIXED_EPOCH")
        .ok()
        .and_then(|s| s.parse::<i64>().ok())
        .and_then(|ts| time::OffsetDateTime::from_unix_timestamp(ts).ok())
        .unwrap_or_else(time::OffsetDateTime::now_utc)
}

/// **P612** — codifica 16 bytes em base64 (URL-safe não é necessário; usa-se
/// o alfabeto standard).
fn base64_encode_16(bytes: [u8; 16]) -> String {
    const ALPHABET: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(24);
    for chunk in bytes.chunks(3) {
        let b = match chunk.len() {
            3 => [chunk[0], chunk[1], chunk[2]],
            2 => [chunk[0], chunk[1], 0],
            1 => [chunk[0], 0, 0],
            _ => unreachable!(),
        };
        out.push(ALPHABET[(b[0] >> 2) as usize] as char);
        out.push(ALPHABET[(((b[0] & 0x03) << 4) | (b[1] >> 4)) as usize] as char);
        out.push(ALPHABET[(((b[1] & 0x0F) << 2) | (b[2] >> 6)) as usize] as char);
        out.push(ALPHABET[(b[2] & 0x3F) as usize] as char);
    }
    match bytes.len() % 3 {
        1 => {
            out.pop();
            out.pop();
            out.push_str("==");
        }
        2 => {
            out.pop();
            out.push('=');
        }
        _ => {}
    }
    out
}

/// **P615** — gera 16 bytes pseudoaleatórios para uso em
/// `DocumentID`/`InstanceID`.
///
/// O seed é gerado uma vez por processo via `getrandom`; dentro da mesma
/// execução, os IDs são determinísticos a partir desse seed. Isto satisfaz
/// dois requisitos contraditórios:
/// 1. Execuções separadas do compilador produzem IDs diferentes (paridade
///    com o vanilla 0.15.0).
/// 2. Testes unitários que compilam o mesmo documento duas no mesmo
///    processo esperam bytes PDF idênticos.
fn random_xmp_id_bytes() -> [u8; 16] {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::OnceLock;

    static SEED: OnceLock<u64> = OnceLock::new();
    static COUNTER: AtomicU64 = AtomicU64::new(0);

    let seed = *SEED.get_or_init(|| {
        let mut bytes = [0u8; 8];
        if getrandom::getrandom(&mut bytes).is_ok() {
            u64::from_be_bytes(bytes)
        } else {
            // Fallback: timestamp de compilação. Nunca deixamos de emitir XMP.
            current_pdf_timestamp().unix_timestamp() as u64
        }
    });

    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut state = seed.wrapping_add(n);
    let mut bytes = [0u8; 16];
    for chunk in bytes.chunks_mut(8) {
        // xorshift64* — gerador simples e determinístico.
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        state = state.wrapping_mul(0x2545_f491_4f6c_dd1d);
        chunk.copy_from_slice(&state.to_be_bytes());
    }
    bytes
}

/// **P615/P617** — devolve `(instance_id, document_id)` para o pacote XMP.
/// Em testes (`CRYSTALLINE_PDF_FIXED_EPOCH` definida), o `InstanceID` é
/// fixo. Em produção, o `InstanceID` é sempre 16 bytes aleatórios.
/// O `DocumentID` usa `external_id` quando fornecido; caso contrário,
/// segue a mesma regra do `InstanceID` (fixo em testes, aleatório em
/// produção), conforme P615.
fn xmp_instance_and_document_id(external_id: Option<[u8; 16]>) -> (String, String) {
    // P615 — em testes (cfg!(test) ou variável de ambiente), o InstanceID
    // é fixo para manter os snapshots de bytes PDF deterministas.
    let is_test = cfg!(test) || std::env::var("CRYSTALLINE_PDF_FIXED_EPOCH").is_ok();
    let instance_id = if is_test {
        "dHlwc3QtY3J5c3QtaW5zdA==".to_string()
    } else {
        base64_encode_16(random_xmp_id_bytes())
    };

    let document_id = if let Some(id) = external_id {
        base64_encode_16(id)
    } else if is_test {
        "dHlwc3QtY3J5c3QtZG9jdQ==".to_string()
    } else {
        base64_encode_16(random_xmp_id_bytes())
    };

    (instance_id, document_id)
}

/// P517 — gera nome de fonte com prefixo de subset quando a fonte foi
/// efectivamente subsetada. Usa o prefixo fixo `AAAAAA+` conforme
/// convenção PDF para fontes subsetadas (ex: `AAAAAA+FontName`).
fn subset_font_name(base_name: &str, _subset_data: &[u8]) -> String {
    format!("AAAAAA+{}", base_name)
}

/// P560 — extrai a tabela `CFF` de uma fonte OpenType/SFNT.
///
/// Leitores de PDF esperam o programa CFF puro quando o descritor indica
/// `/Subtype /CIDFontType0C`; o contêiner SFNT completo causa mismatch.
fn cff_table_data(font_data: &[u8]) -> Option<&[u8]> {
    let face = Face::parse(font_data, 0).ok()?;
    if face.tables().cff.is_some() {
        face.table_data(ttf_parser::Tag::from_bytes(b"CFF "))
    } else {
        None
    }
}

/// P560 — devolve os componentes PDF correctos e os bytes a embeber.
///
/// TrueType (`glyf`) usa `/CIDFontType2` + `/FontFile2` + stream `/CIDFontType2`.
/// CFF/OpenType usa `/CIDFontType0` + `/FontFile3` + stream `/CIDFontType0C`.
fn font_embedding_data(font_data: &[u8]) -> (&'static str, &'static str, &'static str, &[u8]) {
    if let Some(cff) = cff_table_data(font_data) {
        return ("/CIDFontType0", "/FontFile3", "CIDFontType0C", cff);
    }
    ("/CIDFontType2", "/FontFile2", "CIDFontType2", font_data)
}

/// **P760** — métricas do /FontDescriptor a partir de uma face parseada.
///
/// Os valores são convertidos para o espaço de 1000 unidades usado pelo PDF
/// para FontDescriptors de CIDFonts, alinhando-se com o que o Typst vanilla
/// emite.
struct FontDescriptorMetrics {
    font_bbox: [f64; 4],
    italic_angle: f64,
    ascent: f64,
    descent: f64,
    cap_height: f64,
}

fn font_descriptor_metrics(face: &ttf_parser::Face<'_>) -> FontDescriptorMetrics {
    let upem = face.units_per_em().max(1) as f64;
    let scale = 1000.0 / upem;

    let bbox = face.global_bounding_box();
    let font_bbox = [
        bbox.x_min as f64 * scale,
        bbox.y_min as f64 * scale,
        bbox.x_max as f64 * scale,
        bbox.y_max as f64 * scale,
    ];

    let italic_angle = face.italic_angle() as f64;

    // Preferir métricas tipográficas do OS/2 quando disponíveis; senão
    // recair para as métricas do hhea, como faz o Typst vanilla.
    let (ascent, descent) = if let Some(os2) = face.tables().os2 {
        let typo_asc = os2.typographic_ascender();
        let typo_desc = os2.typographic_descender();
        if typo_asc != 0 || typo_desc != 0 {
            (
                typo_asc as f64 * scale,
                typo_desc as f64 * scale,
            )
        } else {
            (face.ascender() as f64 * scale, face.descender() as f64 * scale)
        }
    } else {
        (face.ascender() as f64 * scale, face.descender() as f64 * scale)
    };

    let cap_height = face
        .capital_height()
        .filter(|&h| h > 0)
        .map(|h| h as f64 * scale)
        .unwrap_or(ascent);

    FontDescriptorMetrics { font_bbox, italic_angle, ascent, descent, cap_height }
}

// ── Builder ────────────────────────────────────────────────────────────────

pub(super) struct PdfBuilder {
    objects: Vec<(usize, Vec<u8>)>,
    /// P518 — tempo acumulado em `subset_font_with_mapping` (ms).
    subset_ms: f64,
    /// **P536** — object ID do dicionário `/Info`, ou `None` se não houver
    /// metadados para emitir.
    info_id: Option<usize>,
    /// **P611** — object ID do stream `/Metadata` (XMP), ou `None` se ainda
    /// não emitido.
    xmp_id: Option<usize>,
    /// **P617** — `DocumentID` externo de 16 bytes. Quando `Some`, sobrepõe
    /// o valor aleatório/fixo por defeito. `InstanceID` nunca é fixado.
    document_id: Option<[u8; 16]>,
}

impl PdfBuilder {
    pub(super) fn new() -> Self {
        Self {
            objects: Vec::new(),
            subset_ms: 0.0,
            info_id: None,
            xmp_id: None,
            document_id: None,
        }
    }

    /// **P617** — fixa o `DocumentID` usado no pacote XMP.
    pub(super) fn with_document_id(mut self, id: Option<[u8; 16]>) -> Self {
        self.document_id = id;
        self
    }

    fn measure_subset(
        &mut self,
        font_data: &[u8],
        char_to_old_gid: &std::collections::BTreeMap<char, u16>,
        additional_gids: &std::collections::BTreeSet<u16>,
    ) -> Option<FontSubset> {
        let t0 = std::time::Instant::now();
        let result = subset_font_with_mapping(font_data, char_to_old_gid, additional_gids);
        self.subset_ms += duration_ms(t0.elapsed());
        result
    }

    fn add(&mut self, id: usize, content: String) {
        self.objects.push((id, content.into_bytes()));
    }

    fn add_bytes(&mut self, id: usize, content: Vec<u8>) {
        self.objects.push((id, content));
    }

    pub(super) fn build(self, doc: &PagedDocument, font_data: Option<&[u8]>) -> (Vec<u8>, f64) {
        if let Some(data) = font_data {
            if let Ok(face) = Face::parse(data, 0) {
                return self.build_cidfont(doc, &face, data);
            }
        }
        self.build_helvetica(doc)
    }

    // ── Caminho Helvetica (fallback, Type1 sem embedding) ─────────────────

    fn build_helvetica(mut self, doc: &PagedDocument) -> (Vec<u8>, f64) {
        let n = doc.pages.len().max(1);
        let first_page   = 3usize;
        let first_stream = first_page + n;
        let font_f1      = first_stream + n;
        let font_f2      = font_f1 + 1;
        let font_f3      = font_f2 + 1;

        // **P777** — reservar ID do perfil ICC sRGB partilhado se houver JPEGs RGB.
        let needs_icc = has_rgb_jpeg(doc);
        let icc_profile_id = if needs_icc { Some(font_f3 + 1) } else { None };
        let first_img_id = font_f3 + 1 + if needs_icc { 1 } else { 0 };

        let (img_refs, ptr_to_idx, img_xobjects) = scan_all_images(doc, first_img_id, icc_profile_id);

        // P263 — Allocar IDs após imagens. Reserva n_gradients*3 + N
        // sub-functions (estimativa pessimista: N stops 16 → 15 subs por gradient).
        let first_grad_id = first_img_id + img_xobjects.len() * 2 + 100;
        let (pat_refs, pat_ptr_to_idx, grad_objs) = scan_all_gradients(doc, first_grad_id);
        let n_grads = grad_objs.len();
        // Sub-function IDs após os 3*N gradient object IDs.
        let mut next_sub_id = first_grad_id + n_grads * 3;

        self.add(1, "<< /Type /Catalog /Pages 2 0 R >>".into());

        let kids = (first_page..first_page + n)
            .map(|i| format!("{i} 0 R"))
            .collect::<Vec<_>>().join(" ");
        self.add(2, format!("<< /Type /Pages /Kids [{kids}] /Count {n} >>"));

        for (i, page) in doc.pages.iter().enumerate() {
            let page_id   = first_page + i;
            let stream_id = first_stream + i;
            let w = page.width;
            let h = page.height;

            let xobj_res = xobject_resources_for_page(page, &ptr_to_idx, &img_refs);
            let pat_res  = pattern_resources_for_page(page, &pat_ptr_to_idx, &pat_refs);
            let resources_str = format!(
                "/Font << /F1 {font_f1} 0 R /F2 {font_f2} 0 R /F3 {font_f3} 0 R >> {xobj_res} {pat_res}"
            );

            self.add(page_id, format!(
                "<< /Type /Page /Parent 2 0 R \
                   /MediaBox [0 0 {w:.2} {h:.2}] \
                   /Contents {stream_id} 0 R \
                   /Resources << {resources_str} >> >>"
            ));

            let ctx = PageContext::type1(&ptr_to_idx, &img_refs, &pat_ptr_to_idx, &pat_refs);
            let stream_bytes = build_page_stream(page, &ctx);
            let len = stream_bytes.len();
            let mut obj = format!("<< /Length {len} >>\nstream\n").into_bytes();
            obj.extend_from_slice(&stream_bytes);
            obj.extend_from_slice(b"\nendstream");
            self.add_bytes(stream_id, obj);
        }

        self.add(font_f1, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica \
                            /Encoding /WinAnsiEncoding >>".into());
        self.add(font_f2, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold \
                            /Encoding /WinAnsiEncoding >>".into());
        self.add(font_f3, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Oblique \
                            /Encoding /WinAnsiEncoding >>".into());

        // **P777** — emitir perfil ICC sRGB partilhado antes dos JPEGs RGB.
        if let Some(id) = icc_profile_id {
            self.add_bytes(id, build_icc_profile_stream(srgb_icc_profile_bytes()));
        }

        self.emit_image_xobjects(img_xobjects);

        // P263 — Emit Function/Shading/Pattern objects para gradients.
        let page_dimensions: Vec<(f64, f64)> = doc.pages.iter()
            .map(|p| (p.width, p.height)).collect();
        self.emit_gradient_objects(grad_objs, &page_dimensions, &mut next_sub_id);

        self.emit_link_annotations(doc);
        self.emit_named_destinations(doc);
        self.emit_outlines(doc);
        self.emit_info(doc);
        self.emit_xmp_metadata(doc);
        let subset_ms = self.subset_ms;
        (self.serialize(), subset_ms)
    }

    // ── Caminho CIDFont (Unicode completo, Identity-H) ─────────────────────

    fn build_cidfont(mut self, doc: &PagedDocument, face: &Face<'_>, font_data: &[u8]) -> (Vec<u8>, f64) {
        let n = doc.pages.len().max(1);
        let first_page         = 3usize;
        let first_stream       = first_page + n;
        let font_id            = first_stream + n;      // Type0 — /F1
        let cidfont_id         = font_id + 1;
        let font_descriptor_id = font_id + 2;
        let font_stream_id     = font_id + 3;
        let to_unicode_id      = font_id + 4;

        // **P777** — reservar ID do perfil ICC sRGB partilhado se houver JPEGs RGB.
        let needs_icc = has_rgb_jpeg(doc);
        let icc_profile_id = if needs_icc { Some(to_unicode_id + 1) } else { None };
        let first_img_id = to_unicode_id + 1 + if needs_icc { 1 } else { 0 };

        let mut chars = collect_codepoints(doc);
        chars.extend(collect_text_codepoints(doc).iter().copied());
        let mut mappings = map_chars_to_glyphs(face, &chars);

        // P520 — glifos reais produzidos pelo shaper (incluindo ligatures como
        // "fi" → gid_ligature). O char_code é o primeiro caractere do cluster.
        // Estas entradas têm prioridade sobre o mapeamento codepoint→glyph
        // da fonte porque reflectem o glifo efectivamente usado.
        let shaped_mappings = collect_shaped_glyph_mappings(doc);

        // Passo 45 — DEBT-9: adicionar glifos variantes (FrameItem::Glyph) ao ToUnicode.
        // O dicionário reverso mapeia glyph_id → char base para caracteres extensíveis.
        let glyph_reverse = build_math_glyph_reverse_map(face);
        let existing_gids: BTreeSet<u16> = mappings.iter().map(|(_, gid)| *gid).collect();
        for gid in collect_glyph_ids(doc) {
            if !existing_gids.contains(&gid) {
                if let Some(&c) = glyph_reverse.get(&gid) {
                    mappings.push((c, gid));
                }
            }
        }

        // P520 — recolher larguras nominais (hmtx) dos glyph IDs que o shaper
        // pode usar, para calcular corretamente os deltas do operador TJ.
        let mut glyph_to_nominal: HashMap<u16, i32> = HashMap::new();
        for &gid in collect_glyph_ids(doc).iter().chain(mappings.iter().map(|(_, gid)| gid)) {
            let adv = face.glyph_hor_advance(ttf_parser::GlyphId(gid)).unwrap_or(0) as i32;
            glyph_to_nominal.insert(gid, adv);
        }

        // P516 — subsetting TrueType/OpenType.
        // P520: shaped glyphs sobrescrevem codepoints no char_to_old_gid para
        // que ligatures sejam incluídas no subset e tenham ToUnicode parcial.
        let mut char_to_old_gid: std::collections::BTreeMap<char, u16> =
            mappings.iter().copied().collect();
        for (&old_gid, &ch) in &shaped_mappings {
            char_to_old_gid.insert(ch, old_gid);
        }
        // P520 — todos os glyph IDs reais (incluindo ligatures com mesmo
        // char_code representativo) devem ser preservados no subset.
        // P568 — incluir também glyphs de FrameItem::Text (ex.: espaços entre
        // palavras), que não passam pelo shaper mas precisam de estar no subset.
        let mut all_glyph_ids = collect_glyph_ids(doc);
        for &c in collect_text_codepoints(doc).iter() {
            if let Some(gid) = face.glyph_index(c) {
                all_glyph_ids.insert(gid.0);
            }
        }
        let (embed_font_data, glyph_mapping) =
            match self.measure_subset(font_data, &char_to_old_gid, &all_glyph_ids) {
                Some(FontSubset { data, mapping }) => {
                    if Face::parse(&data, 0).is_ok() {
                        (data, mapping)
                    } else {
                        // Fallback para fonte completa se o subset não parsear.
                        (font_data.to_vec(), HashMap::new())
                    }
                }
                None => (font_data.to_vec(), HashMap::new()),
            };

        // Re-mapear mappings para os novos glyph IDs do subset.
        if !glyph_mapping.is_empty() {
            for (_, old_gid) in mappings.iter_mut() {
                *old_gid = remap_glyph_id(*old_gid, &glyph_mapping);
            }
        }

        // P521 — ToUnicode CMap e /W a partir dos glifos reais do shaper,
        // reconstruindo o texto completo de cada cluster (ligatures, RTL).
        // Caracteres normais (não shaped) são adicionados como fallback
        // single-codepoint.
        let mut to_unicode_mappings: Vec<(u16, String)> = Vec::new();
        let mut seen_to_unicode_gids: HashSet<u16> = HashSet::new();
        if !glyph_mapping.is_empty() {
            for (old_gid, hex) in collect_shaped_cluster_texts(doc) {
                let new_gid = remap_glyph_id(old_gid, &glyph_mapping);
                if new_gid != 0 && seen_to_unicode_gids.insert(new_gid) {
                    to_unicode_mappings.push((new_gid, hex));
                }
            }
        }
        for &(ch, new_gid) in &mappings {
            if new_gid != 0 && seen_to_unicode_gids.insert(new_gid) {
                to_unicode_mappings.push((new_gid, char_to_utf16_hex(ch)));
            }
        }

        let subset_face = Face::parse(&embed_font_data, 0).ok();
        let face_for_widths = subset_face.as_ref().unwrap_or(face);
        let char_to_gid: HashMap<char, u16> = mappings.iter().copied().collect();
        let widths = widths_array(face_for_widths, &to_unicode_mappings);

        let (img_refs, ptr_to_idx, img_xobjects) = scan_all_images(doc, first_img_id, icc_profile_id);

        // P263 — gradient pre-pass.
        let first_grad_id = first_img_id + img_xobjects.len() * 2 + 100;
        let (pat_refs, pat_ptr_to_idx, grad_objs) = scan_all_gradients(doc, first_grad_id);
        let n_grads = grad_objs.len();
        let mut next_sub_id = first_grad_id + n_grads * 3;

        self.add(1, "<< /Type /Catalog /Pages 2 0 R >>".into());

        let kids = (first_page..first_page + n)
            .map(|i| format!("{i} 0 R"))
            .collect::<Vec<_>>().join(" ");
        self.add(2, format!("<< /Type /Pages /Kids [{kids}] /Count {n} >>"));

        for (i, page) in doc.pages.iter().enumerate() {
            let page_id   = first_page + i;
            let stream_id = first_stream + i;
            let w = page.width;
            let h = page.height;

            let xobj_res = xobject_resources_for_page(page, &ptr_to_idx, &img_refs);
            let pat_res  = pattern_resources_for_page(page, &pat_ptr_to_idx, &pat_refs);
            let resources_str = format!("/Font << /F1 {font_id} 0 R >> {xobj_res} {pat_res}");

            self.add(page_id, format!(
                "<< /Type /Page /Parent 2 0 R \
                   /MediaBox [0 0 {w:.2} {h:.2}] \
                   /Contents {stream_id} 0 R \
                   /Resources << {resources_str} >> >>"
            ));

            let ctx = PageContext::cidfont(
                &ptr_to_idx, &img_refs, &pat_ptr_to_idx, &pat_refs,
                &char_to_gid, &glyph_mapping, &glyph_to_nominal,
            );
            let stream_bytes = build_page_stream(page, &ctx);
            let len = stream_bytes.len();
            let mut obj = format!("<< /Length {len} >>\nstream\n").into_bytes();
            obj.extend_from_slice(&stream_bytes);
            obj.extend_from_slice(b"\nendstream");
            self.add_bytes(stream_id, obj);
        }

        // P517 — nome com prefixo de subset quando aplicável.
        let base_font_name = if glyph_mapping.is_empty() {
            "CrystallineFont".to_string()
        } else {
            subset_font_name("CrystallineFont", &embed_font_data)
        };

        // P560 — descritor PDF e bytes conforme o tipo de fonte (TrueType vs CFF/OpenType).
        let (cid_subtype, font_file_key, stream_subtype, font_stream_data) =
            font_embedding_data(&embed_font_data);

        // Type0 font (F1)
        self.add(font_id, format!(
            "<< /Type /Font /Subtype /Type0 /BaseFont /{base_font_name} \
               /Encoding /Identity-H \
               /DescendantFonts [{cidfont_id} 0 R] \
               /ToUnicode {to_unicode_id} 0 R >>"
        ));

        // CIDFont
        self.add(cidfont_id, format!(
            "<< /Type /Font /Subtype {cid_subtype} /BaseFont /{base_font_name} \
               /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> \
               /FontDescriptor {font_descriptor_id} 0 R \
               /DW 500 \
               /W [{widths}] >>"
        ));

        // FontDescriptor — P760: métricas reais da fonte embutida em vez de
        // valores fixos genéricos.
        let fd = subset_face.as_ref()
            .map(font_descriptor_metrics)
            .unwrap_or_else(|| FontDescriptorMetrics {
                font_bbox: [-1000.0, -200.0, 2000.0, 900.0],
                italic_angle: 0.0,
                ascent: 800.0,
                descent: -200.0,
                cap_height: 700.0,
            });
        self.add(font_descriptor_id, format!(
            "<< /Type /FontDescriptor /FontName /{base_font_name} \
               /Flags 32 \
               /FontBBox [{:.5} {:.5} {:.5} {:.5}] \
               /ItalicAngle {:.5} /Ascent {:.5} /Descent {:.5} \
               /CapHeight {:.5} /StemV 80 \
               {font_file_key} {font_stream_id} 0 R >>",
            fd.font_bbox[0], fd.font_bbox[1], fd.font_bbox[2], fd.font_bbox[3],
            fd.italic_angle, fd.ascent, fd.descent, fd.cap_height
        ));

        // Font data stream — P516: usa subset se possível, senão fonte completa.
        // P560: stream subtype TrueType (CIDFontType2) ou CFF (CIDFontType0C).
        let font_len = font_stream_data.len();
        let mut font_stream = format!(
            "<< /Length {font_len} /Subtype /{stream_subtype} >>\nstream\n"
        ).into_bytes();
        font_stream.extend_from_slice(font_stream_data);
        font_stream.extend_from_slice(b"\nendstream");
        self.add_bytes(font_stream_id, font_stream);

        // ToUnicode CMap stream
        let cmap = to_unicode_cmap(&to_unicode_mappings);
        let cmap_len = cmap.len();
        let mut cmap_obj = format!("<< /Length {cmap_len} >>\nstream\n").into_bytes();
        cmap_obj.extend_from_slice(&cmap);
        cmap_obj.extend_from_slice(b"\nendstream");
        self.add_bytes(to_unicode_id, cmap_obj);

        // **P777** — emitir perfil ICC sRGB partilhado antes dos JPEGs RGB.
        if let Some(id) = icc_profile_id {
            self.add_bytes(id, build_icc_profile_stream(srgb_icc_profile_bytes()));
        }

        self.emit_image_xobjects(img_xobjects);

        // P263 — Emit gradient objects.
        let page_dimensions: Vec<(f64, f64)> = doc.pages.iter()
            .map(|p| (p.width, p.height)).collect();
        self.emit_gradient_objects(grad_objs, &page_dimensions, &mut next_sub_id);

        self.emit_link_annotations(doc);
        self.emit_named_destinations(doc);
        self.emit_outlines(doc);
        self.emit_info(doc);
        self.emit_xmp_metadata(doc);
        let subset_ms = self.subset_ms;
        (self.serialize(), subset_ms)
    }

    // ── Caminho Multi-font (Passo 146, ADR-0055 decisão 5) ───────────────────

    pub(super) fn build_multifont(
        mut self,
        doc:   &PagedDocument,
        fonts: &[((FontList, FontVariant), Vec<u8>)],
        faces: &[Face<'_>],
    ) -> (Vec<u8>, f64) {
        let n_pages = doc.pages.len().max(1);
        let n_fonts = fonts.len();
        let first_page   = 3usize;
        let first_stream = first_page + n_pages;
        // Cada font ocupa 5 IDs consecutivos: type0, cidfont, descriptor,
        // font_stream, to_unicode. Type0 é o "/Fn" referenciado no resource.
        let fonts_start  = first_stream + n_pages;

        // **P777** — reservar ID do perfil ICC sRGB partilhado se houver JPEGs RGB.
        let needs_icc = has_rgb_jpeg(doc);
        let icc_profile_id = if needs_icc { Some(fonts_start + 5 * n_fonts) } else { None };
        let first_img_id = fonts_start + 5 * n_fonts + if needs_icc { 1 } else { 0 };

        // Codepoints + glyph mappings por font. Cada font tem o seu
        // mapping (chars partilhados; gids específicos da face).
        let mut chars = collect_codepoints(doc);
        let text_chars = collect_text_codepoints(doc);
        chars.extend(text_chars.iter().copied());
        let glyph_ids = collect_glyph_ids(doc);
        // P520 — glifos reais do shaper (ligatures) mapeados para o primeiro
        // caractere do cluster. Prioridade idêntica a build_cidfont.
        let shaped_mappings = collect_shaped_glyph_mappings(doc);
        // P675 — colectar textos shaped uma única vez e partilhar entre fontes,
        // em vez de percorrer o documento N vezes (uma por fonte).
        let shaped_cluster_texts = collect_shaped_cluster_texts(doc);
        let mut per_font_mappings: Vec<Vec<(u16, String)>> = Vec::with_capacity(n_fonts);
        let mut per_font_char_to_gid: Vec<HashMap<char, u16>> = Vec::with_capacity(n_fonts);
        let mut per_font_widths: Vec<String> = Vec::with_capacity(n_fonts);
        let mut per_font_embed_data: Vec<Vec<u8>> = Vec::with_capacity(n_fonts);
        let mut per_font_glyph_mapping: Vec<HashMap<u16, u16>> = Vec::with_capacity(n_fonts);
        let mut per_font_glyph_to_nominal: Vec<HashMap<u16, i32>> = Vec::with_capacity(n_fonts);
        for face in faces {
            let mut mappings = map_chars_to_glyphs(face, &chars);

            // P568 — incluir glyphs de FrameItem::Text para que espaços e outros
            // caracteres do caminho fallback sejam subsetados.
            let mut extended_glyph_ids = glyph_ids.clone();
            for &c in &text_chars {
                if let Some(gid) = face.glyph_index(c) {
                    extended_glyph_ids.insert(gid.0);
                }
            }

            // P520 — larguras nominais (hmtx) desta face para todos os glyph IDs
            // que podem aparecer no stream.
            let mut glyph_to_nominal: HashMap<u16, i32> = HashMap::new();
            for &gid in extended_glyph_ids.iter().chain(mappings.iter().map(|(_, gid)| gid)) {
                let adv = face.glyph_hor_advance(ttf_parser::GlyphId(gid)).unwrap_or(0) as i32;
                glyph_to_nominal.insert(gid, adv);
            }
            // Adicionar glifos variantes de tamanho matemático
            // (Passo 45, DEBT-9) — mesmo tratamento que `build_cidfont`.
            let glyph_reverse = build_math_glyph_reverse_map(face);
            let existing_gids: BTreeSet<u16> = mappings.iter().map(|(_, gid)| *gid).collect();
            for &gid in &glyph_ids {
                if !existing_gids.contains(&gid) {
                    if let Some(&c) = glyph_reverse.get(&gid) {
                        mappings.push((c, gid));
                    }
                }
            }

            // P516 — subsetting por fonte.
            // P520: shaped glyphs (ligatures) têm prioridade no char_to_old_gid.
            let mut char_to_old_gid: std::collections::BTreeMap<char, u16> =
                mappings.iter().copied().collect();
            for (&old_gid, &ch) in &shaped_mappings {
                char_to_old_gid.insert(ch, old_gid);
            }
            let font_index = per_font_mappings.len();
            let (font_list, font_variant) = &fonts[font_index].0;
            let font_bytes = &fonts[font_index].1;

            let (embed_data, glyph_mapping) =
                match self.measure_subset(font_bytes, &char_to_old_gid, &glyph_ids) {
                    Some(FontSubset { data, mapping }) => {
                        if Face::parse(&data, 0).is_ok() {
                            (data, mapping)
                        } else {
                            (font_bytes.clone(), HashMap::new())
                        }
                    }
                    None => (font_bytes.clone(), HashMap::new()),
                };

            // P530 — instanciar estaticamente a VF se a combinação usar um
            // peso/estilo diferente do default. A instanciação é feita depois
            // do subsetting para operar sobre uma fonte pequena (P529).
            let axis_vars = axis_variations_for_font_variant(font_variant);
            let embed_data = if !axis_vars.is_empty() {
                let axis_tuples: Vec<(ttf_parser::Tag, f32)> = axis_vars
                    .iter()
                    .map(|v| (v.tag, v.value))
                    .collect();
                match instantiate_variable_font(&embed_data, &axis_tuples) {
                    Some(instanced) => instanced,
                    None => {
                        // Se o instancer falhar (ex.: Python/fontTools ausente),
                        // mantém o subset default com aviso.
                        eprintln!(
                            "Aviso: falha ao instanciar fonte variável para {:?}/{:?}; \
                             a usar instância default.",
                            font_list, font_variant
                        );
                        embed_data
                    }
                }
            } else {
                embed_data
            };

            if !glyph_mapping.is_empty() {
                for (_, old_gid) in mappings.iter_mut() {
                    *old_gid = remap_glyph_id(*old_gid, &glyph_mapping);
                }
            }

            // P521 — ToUnicode/widths a partir dos glifos reais do shaper,
            // reconstruindo o texto completo de cada cluster.
            let mut to_unicode_mappings: Vec<(u16, String)> = Vec::new();
            let mut seen_to_unicode_gids: HashSet<u16> = HashSet::new();
            if !glyph_mapping.is_empty() {
                for &(old_gid, ref hex) in &shaped_cluster_texts {
                    let new_gid = remap_glyph_id(old_gid, &glyph_mapping);
                    if new_gid != 0 && seen_to_unicode_gids.insert(new_gid) {
                        to_unicode_mappings.push((new_gid, hex.clone()));
                    }
                }
            }
            for &(ch, new_gid) in &mappings {
                if new_gid != 0 && seen_to_unicode_gids.insert(new_gid) {
                    to_unicode_mappings.push((new_gid, char_to_utf16_hex(ch)));
                }
            }

            let subset_face = Face::parse(&embed_data, 0).ok();
            let face_for_widths = subset_face.as_ref().unwrap_or(face);
            let char_to_gid: HashMap<char, u16> = mappings.iter().copied().collect();
            let widths = widths_array(face_for_widths, &to_unicode_mappings);
            per_font_mappings.push(to_unicode_mappings);
            per_font_char_to_gid.push(char_to_gid);
            per_font_widths.push(widths);
            per_font_embed_data.push(embed_data);
            per_font_glyph_mapping.push(glyph_mapping);
            per_font_glyph_to_nominal.push(glyph_to_nominal);
        }

        let (img_refs, ptr_to_idx, img_xobjects) = scan_all_images(doc, first_img_id, icc_profile_id);

        // P263 — gradient pre-pass.
        let first_grad_id = first_img_id + img_xobjects.len() * 2 + 100;
        let (pat_refs, pat_ptr_to_idx, grad_objs) = scan_all_gradients(doc, first_grad_id);
        let n_grads = grad_objs.len();
        let mut next_sub_id = first_grad_id + n_grads * 3;

        self.add(1, "<< /Type /Catalog /Pages 2 0 R >>".into());

        let kids = (first_page..first_page + n_pages)
            .map(|i| format!("{i} 0 R"))
            .collect::<Vec<_>>().join(" ");
        self.add(2, format!("<< /Type /Pages /Kids [{kids}] /Count {n_pages} >>"));

        for (i, page) in doc.pages.iter().enumerate() {
            let page_id   = first_page + i;
            let stream_id = first_stream + i;
            let w = page.width;
            let h = page.height;

            let xobj_res = xobject_resources_for_page(page, &ptr_to_idx, &img_refs);
            let pat_res  = pattern_resources_for_page(page, &pat_ptr_to_idx, &pat_refs);
            let font_entries = (0..n_fonts).map(|fi| {
                let type0_id = fonts_start + 5 * fi;
                format!("/F{} {} 0 R", fi + 1, type0_id)
            }).collect::<Vec<_>>().join(" ");
            let resources_str = format!("/Font << {font_entries} >> {xobj_res} {pat_res}");

            self.add(page_id, format!(
                "<< /Type /Page /Parent 2 0 R \
                   /MediaBox [0 0 {w:.2} {h:.2}] \
                   /Contents {stream_id} 0 R \
                   /Resources << {resources_str} >> >>"
            ));

            let ctx = PageContext::multifont(
                &ptr_to_idx, &img_refs, &pat_ptr_to_idx, &pat_refs,
                fonts, &per_font_char_to_gid, &per_font_glyph_mapping,
                &per_font_glyph_to_nominal,
            );
            let stream_bytes = build_page_stream(page, &ctx);
            let len = stream_bytes.len();
            let mut obj = format!("<< /Length {len} >>\nstream\n").into_bytes();
            obj.extend_from_slice(&stream_bytes);
            obj.extend_from_slice(b"\nendstream");
            self.add_bytes(stream_id, obj);
        }

        // Emit objectos por font (5 cada).
        for (fi, _) in fonts.iter().enumerate() {
            let font_data = &per_font_embed_data[fi];
            let type0_id      = fonts_start + 5 * fi;
            let cidfont_id    = type0_id + 1;
            let descriptor_id = type0_id + 2;
            let stream_id     = type0_id + 3;
            let to_unicode_id = type0_id + 4;
            let widths = &per_font_widths[fi];
            let mappings = &per_font_mappings[fi];
            let glyph_mapping = &per_font_glyph_mapping[fi];
            let base_name = format!("CrystallineFont{}", fi + 1);
            let name = if glyph_mapping.is_empty() {
                base_name.clone()
            } else {
                subset_font_name(&base_name, font_data)
            };

            // P560 — descritor PDF e bytes conforme o tipo de fonte (TrueType vs CFF/OpenType).
            let (cid_subtype, font_file_key, stream_subtype, font_stream_data) =
                font_embedding_data(font_data);

            // Type0
            self.add(type0_id, format!(
                "<< /Type /Font /Subtype /Type0 /BaseFont /{name} \
                   /Encoding /Identity-H \
                   /DescendantFonts [{cidfont_id} 0 R] \
                   /ToUnicode {to_unicode_id} 0 R >>"
            ));

            // CIDFont
            self.add(cidfont_id, format!(
                "<< /Type /Font /Subtype {cid_subtype} /BaseFont /{name} \
                   /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> \
                   /FontDescriptor {descriptor_id} 0 R \
                   /DW 500 \
                   /W [{widths}] >>"
            ));

            // FontDescriptor
            self.add(descriptor_id, format!(
                "<< /Type /FontDescriptor /FontName /{name} \
                   /Flags 32 \
                   /FontBBox [-1000 -200 2000 900] \
                   /ItalicAngle 0 /Ascent 800 /Descent -200 \
                   /CapHeight 700 /StemV 80 \
                   {font_file_key} {stream_id} 0 R >>"
            ));

            // Font data stream — P516: usa subset se possível, senão fonte completa.
            // P560: stream subtype TrueType (CIDFontType2) ou CFF (CIDFontType0C).
            let font_len = font_stream_data.len();
            let mut font_stream = format!(
                "<< /Length {font_len} /Subtype /{stream_subtype} >>\nstream\n"
            ).into_bytes();
            font_stream.extend_from_slice(font_stream_data);
            font_stream.extend_from_slice(b"\nendstream");
            self.add_bytes(stream_id, font_stream);

            // ToUnicode CMap
            let cmap = to_unicode_cmap(mappings);
            let cmap_len = cmap.len();
            let mut cmap_obj = format!("<< /Length {cmap_len} >>\nstream\n").into_bytes();
            cmap_obj.extend_from_slice(&cmap);
            cmap_obj.extend_from_slice(b"\nendstream");
            self.add_bytes(to_unicode_id, cmap_obj);
        }

        // **P777** — emitir perfil ICC sRGB partilhado antes dos JPEGs RGB.
        if let Some(id) = icc_profile_id {
            self.add_bytes(id, build_icc_profile_stream(srgb_icc_profile_bytes()));
        }

        self.emit_image_xobjects(img_xobjects);

        // P263 — Emit gradient objects.
        let page_dimensions: Vec<(f64, f64)> = doc.pages.iter()
            .map(|p| (p.width, p.height)).collect();
        self.emit_gradient_objects(grad_objs, &page_dimensions, &mut next_sub_id);

        self.emit_link_annotations(doc);
        self.emit_named_destinations(doc);
        self.emit_outlines(doc);
        self.emit_info(doc);
        self.emit_xmp_metadata(doc);
        let subset_ms = self.subset_ms;
        (self.serialize(), subset_ms)
    }

    /// Emite todos os XObjects de imagem pré-processados para o builder.
    ///
    /// Para PNG com alpha: emite /SMask (canal alpha) antes do XObject principal
    /// (canal RGB), para que o SMask apareça antes no ficheiro PDF — o dicionário
    /// do XObject principal referencia o ID do SMask por forward reference.
    /// P263 — Emite objects Function + Shading + Pattern para cada
    /// gradient único pré-processado por `scan_all_gradients`.
    ///
    /// `next_sub_id`: contador de IDs allocaveis para sub-Functions
    /// (Type 3 stitching). Os IDs alocados por gradient (3×N) **não
    /// incluem** as sub-Functions; estas são alocadas em `next_sub_id`
    /// (que deve apontar para zone de IDs livre pós-todos os outros).
    fn emit_gradient_objects(
        &mut self,
        grad_objs: Vec<GradientObject>,
        page_dimensions: &[(f64, f64)],
        next_sub_id: &mut usize,
    ) {
        for go in grad_objs {
            let GradientObject { kind, function_id, shading_id, pattern_id, parent_bbox_at_emit } = go;
            let (page_w, page_h) = page_dimensions.first().copied().unwrap_or((595.0, 842.0));
            // P273.6 — bbox real do Layouter substitui page_bbox 3γ.1 quando
            // disponível; fallback page_bbox preserved P273.5.
            let effective_parent_bbox: (f32, f32, f32, f32) =
                if let Some(rect) = parent_bbox_at_emit {
                    (rect.x.0 as f32, rect.y.0 as f32,
                     (rect.x.0 + rect.w.0) as f32, (rect.y.0 + rect.h.0) as f32)
                } else {
                    (0.0, 0.0, page_w as f32, page_h as f32)
                };
            // P265 + P268 — branching Linear / Radial / Conic.
            match &kind {
                GradientObjectKind::Linear(linear) => {
                    use typst_core::entities::layout_types::ColorSpace;
                    let (x0, y0, x1, y1) = compute_axial_coords(
                        linear.angle.to_rad(), 0.0, 0.0, page_w, page_h);
                    // P273.5 + P273.6 — quando relative=Parent, exercita
                    // apply_parent_transform com effective_parent_bbox:
                    // - P273.6: bbox real do Layouter (Block save/restore) quando disponível.
                    // - P273.5 fallback: page_bbox identity (gradient top-level).
                    let relative = resolve_relative(linear.relative);
                    let (x0, y0, x1, y1) =
                        if relative == typst_core::entities::gradient::RelativeTo::Parent {
                            let local = (
                                (x0 / page_w) as f32,
                                (y0 / page_h) as f32,
                                (x1 / page_w) as f32,
                                (y1 / page_h) as f32,
                            );
                            let (tx0, ty0, tx1, ty1) =
                                apply_parent_transform(local, Some(effective_parent_bbox));
                            (tx0 as f64, ty0 as f64, tx1 as f64, ty1 as f64)
                        } else {
                            (x0, y0, x1, y1)
                        };
                    // P270.2 — dispatcher dual CMYK vs RGB-family.
                    if linear.space == ColorSpace::Cmyk {
                        let stops_cmyk = multispace_sample_stops_linear_cmyk(linear, 16);
                        let shading_dict = format!(
                            "<< /ShadingType 2 /ColorSpace /DeviceCMYK \
                               /Coords [{:.3} {:.3} {:.3} {:.3}] \
                               /Function {} 0 R /Extend [false false] >>",
                            x0, y0, x1, y1, function_id,
                        );
                        let (func_dict, sub_objs) = emit_function_dict_cmyk(&stops_cmyk, function_id, next_sub_id);
                        for (sub_id, sub_dict) in sub_objs {
                            self.add(sub_id, sub_dict);
                        }
                        self.add(function_id, func_dict);
                        self.add(shading_id, shading_dict);
                    } else {
                        // P270.1 pipeline + P274 adaptive N (N=16 baseline
                        // preservado para low-contrast; N=32/64 para
                        // moderate/high contrast — banding suppression).
                        let n = adaptive_n_for_stops(&linear.stops, linear.space);
                        let stops = multispace_sample_stops(linear, n);
                        let shading_dict = format!(
                            "<< /ShadingType 2 /ColorSpace /DeviceRGB \
                               /Coords [{:.3} {:.3} {:.3} {:.3}] \
                               /Function {} 0 R /Extend [false false] >>",
                            x0, y0, x1, y1, function_id,
                        );
                        let (func_dict, sub_objs) = emit_function_dict(&stops, function_id, next_sub_id);
                        for (sub_id, sub_dict) in sub_objs {
                            self.add(sub_id, sub_dict);
                        }
                        self.add(function_id, func_dict);
                        self.add(shading_id, shading_dict);
                    }
                }
                GradientObjectKind::Radial(radial) => {
                    use typst_core::entities::layout_types::ColorSpace;
                    // P269 — passa focal_center/focal_radius reais.
                    // Defaults (focal=center, fr=0) preservam comportamento P265.
                    let (x0, y0, r0, x1, y1, r1) = compute_radial_coords(
                        radial.center, radial.radius,
                        radial.focal_center, radial.focal_radius,
                        page_w, page_h);
                    // P273.5 + P273.6 — paridade Linear; usa effective_parent_bbox.
                    let relative = resolve_relative(radial.relative);
                    let (x0, y0, x1, y1) =
                        if relative == typst_core::entities::gradient::RelativeTo::Parent {
                            let local = (
                                (x0 / page_w) as f32,
                                (y0 / page_h) as f32,
                                (x1 / page_w) as f32,
                                (y1 / page_h) as f32,
                            );
                            let (tx0, ty0, tx1, ty1) =
                                apply_parent_transform(local, Some(effective_parent_bbox));
                            (tx0 as f64, ty0 as f64, tx1 as f64, ty1 as f64)
                        } else {
                            (x0, y0, x1, y1)
                        };
                    // P270.2 — dispatcher dual CMYK vs RGB-family.
                    if radial.space == ColorSpace::Cmyk {
                        let stops_cmyk = multispace_sample_stops_radial_cmyk(radial, 16);
                        let shading_dict = format!(
                            "<< /ShadingType 3 /ColorSpace /DeviceCMYK \
                               /Coords [{:.3} {:.3} {:.3} {:.3} {:.3} {:.3}] \
                               /Function {} 0 R /Extend [true true] >>",
                            x0, y0, r0, x1, y1, r1, function_id,
                        );
                        let (func_dict, sub_objs) = emit_function_dict_cmyk(&stops_cmyk, function_id, next_sub_id);
                        for (sub_id, sub_dict) in sub_objs {
                            self.add(sub_id, sub_dict);
                        }
                        self.add(function_id, func_dict);
                        self.add(shading_id, shading_dict);
                    } else {
                        // P270.1 pipeline + P274 adaptive N (paridade Linear).
                        let n = adaptive_n_for_stops(&radial.stops, radial.space);
                        let stops = multispace_sample_stops_radial(radial, n);
                        let shading_dict = format!(
                            "<< /ShadingType 3 /ColorSpace /DeviceRGB \
                               /Coords [{:.3} {:.3} {:.3} {:.3} {:.3} {:.3}] \
                               /Function {} 0 R /Extend [true true] >>",
                            x0, y0, r0, x1, y1, r1, function_id,
                        );
                        let (func_dict, sub_objs) = emit_function_dict(&stops, function_id, next_sub_id);
                        for (sub_id, sub_dict) in sub_objs {
                            self.add(sub_id, sub_dict);
                        }
                        self.add(function_id, func_dict);
                        self.add(shading_id, shading_dict);
                    }
                }
                GradientObjectKind::Conic(conic) => {
                    use typst_core::entities::layout_types::ColorSpace;
                    // P272 — dispatcher unificado /ShadingType 6 Coons para 8/8 spaces.
                    // ADR-0090 REVOGADO (Type 4 Gouraud descontinuado);
                    // ADR-0092 expandida cumulativamente (Cenário A revisado FINAL).
                    let (stream, colorspace, decode_array, c0, c1) =
                        if conic.space == ColorSpace::Cmyk {
                            // P270.4 — Type 6 Coons CMYK (preserved literal).
                            (emit_conic_coons_stream_cmyk(conic),
                             "/DeviceCMYK",
                             "[0 1 0 1 0 1 0 1 0 1 0 1]",
                             "[0 0 0 0]", "[1 1 1 1]")
                        } else {
                            // P272 — Type 6 Coons RGB (N=stops*4 patches).
                            (emit_conic_coons_stream_rgb(conic),
                             "/DeviceRGB",
                             "[0 1 0 1 0 1 0 1 0 1]",
                             "[0 0 0]", "[1 1 1]")
                        };
                    let len = stream.len();
                    let header = format!(
                        "<< /ShadingType 6 /ColorSpace {} \
                           /BitsPerCoordinate 8 /BitsPerComponent 8 \
                           /BitsPerFlag 8 \
                           /Decode {} \
                           /Length {} >>\nstream\n",
                        colorspace, decode_array, len,
                    );
                    let mut shading_bytes = header.into_bytes();
                    shading_bytes.extend_from_slice(&stream);
                    shading_bytes.extend_from_slice(b"\nendstream");
                    // Type 6 Coons não usa Function dict (cores nos corner colors
                    // do stream). Function vazio preserva numbering.
                    self.add(function_id, format!(
                        "<< /FunctionType 2 /Domain [0 1] /C0 {} /C1 {} /N 1 >>",
                        c0, c1,
                    ));
                    self.add_bytes(shading_id, shading_bytes);
                }
            };
            // Emit Pattern dict.
            let pattern_dict = format!(
                "<< /PatternType 2 /Shading {} 0 R /Matrix [1 0 0 1 0 0] >>",
                shading_id,
            );
            self.add(pattern_id, pattern_dict);
        }
    }

    fn emit_image_xobjects(&mut self, xobjects: Vec<ImageXObject>) {
        for xobj in xobjects {
            match xobj {
                ImageXObject::Jpeg { data, main_obj_id, iw, ih, icc_profile_id } => {
                    let cs = jpeg_color_space(&data);
                    self.add_bytes(main_obj_id, build_jpeg_xobject(&data, iw, ih, cs, icc_profile_id));
                }
                ImageXObject::Png { payload, main_obj_id, smask_obj_id } => {
                    // Emitir /SMask antes do XObject principal.
                    if let (Some(smask_id), Some(alpha)) = (smask_obj_id, &payload.alpha_data_compressed) {
                        self.add_bytes(smask_id, build_png_smask_xobject(payload.width, payload.height, alpha));
                    }
                    self.add_bytes(main_obj_id, build_png_rgb_xobject(&payload, smask_obj_id));
                }
            }
        }
    }

    /// **P424/P463** — Emite annotations `/Subtype /Link` para cada
    /// `FrameItem::Link` do documento: `/A /URI` para URLs externos e
    /// `/A /GoTo` para destinos internos. As annotations são adicionadas como
    /// objetos após todos os recursos e referenciadas pelo `/Annots` de cada
    /// página. Coordenadas convertidas de Y-down (layout) para Y-up (PDF).
    fn emit_link_annotations(&mut self, doc: &PagedDocument) {
        const FIRST_PAGE_ID: usize = 3;

        let mut next_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;

        // Coletar links por página (coordenadas globais de página).
        let mut per_page: Vec<Vec<(LinkTarget, Point, Size)>> = Vec::with_capacity(doc.pages.len());
        for page in &doc.pages {
            let mut links = Vec::new();
            collect_links(&page.items, &mut links);
            per_page.push(links);
        }

        let mut page_annotation_ids: Vec<Vec<usize>> = vec![Vec::new(); doc.pages.len()];
        for (page_idx, links) in per_page.iter().enumerate() {
            let page_h = doc.pages[page_idx].height;
            for (target, pos, size) in links {
                let annot_id = next_id;
                next_id += 1;

                let x0 = pos.x.val();
                let y0 = page_h - pos.y.val() - size.height.val();
                let x1 = x0 + size.width.val();
                let y1 = y0 + size.height.val();

                let action = match target {
                    LinkTarget::Url(url) => {
                        let escaped_url = escape_pdf_uri(url.as_str());
                        format!("<< /Type /Action /S /URI /URI ({escaped_url}) >>")
                    }
                    LinkTarget::Destination(label) => {
                        let name = escape_pdf_dest_name(&label.0);
                        format!("<< /Type /Action /S /GoTo /D {name} >>")
                    }
                };

                self.add(annot_id, format!(
                    "<< /Type /Annot /Subtype /Link \
                       /Rect [{x0:.2} {y0:.2} {x1:.2} {y1:.2}] \
                       /Border [0 0 0] \
                       /A {action} >>"
                ));
                page_annotation_ids[page_idx].push(annot_id);
            }
        }

        // Reescrever os dicionários das páginas que possuem annotations.
        for (page_idx, ids) in page_annotation_ids.iter().enumerate() {
            if ids.is_empty() {
                continue;
            }
            let page_id = FIRST_PAGE_ID + page_idx;
            let refs = ids.iter()
                .map(|id| format!("{id} 0 R"))
                .collect::<Vec<_>>()
                .join(" ");
            if let Some((_, content)) = self.objects.iter_mut().find(|(id, _)| *id == page_id) {
                let s = String::from_utf8_lossy(content);
                if let Some(idx) = s.rfind(">>") {
                    let mut new = s[..idx].to_string();
                    new.push_str(&format!(" /Annots [{refs}]"));
                    new.push_str(&s[idx..]);
                    *content = new.into_bytes();
                }
            }
        }
    }

    /// **P460** — Emite o dicionário `/Names /Dests` no catalog (objecto 1)
    /// para cada label registado no `PagedDocument`.
    ///
    /// Cada destino usa `/XYZ x y null` com coordenadas PDF (y-up). O catalog
    /// original `<< /Type /Catalog /Pages 2 0 R >>` é editado in-place.
    fn emit_named_destinations(&mut self, doc: &PagedDocument) {
        const FIRST_PAGE_ID: usize = 3;

        if doc.extracted_label_pages.is_empty() {
            return;
        }

        // Juntar labels comuns a label_pages e label_positions.
        let mut entries: Vec<(EcoString, usize, Point)> = Vec::new();
        for (label, &page) in &doc.extracted_label_pages {
            let pos = doc.extracted_label_positions.get(label).copied().unwrap_or(Point::ZERO);
            entries.push((label.0.clone().into(), page, pos));
        }
        // Ordenar por nome de label para garantir output PDF determinístico
        // (os HashMaps subjacentes não preservam ordem de inserção).
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        if entries.is_empty() {
            return;
        }

        let mut next_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;
        let names_id = next_id;
        next_id += 1;
        let dests_id = next_id;

        // Construir o dicionário /Dests: /name [page_ref /XYZ x y null]
        let mut dests_dict = String::from("<< ");
        for (name, page, pos) in entries {
            let page_idx = page.saturating_sub(1);
            let page_ref = if page_idx < doc.pages.len() {
                format!("{} 0 R", FIRST_PAGE_ID + page_idx)
            } else {
                // Fallback: última página válida.
                format!("{} 0 R", FIRST_PAGE_ID + doc.pages.len().saturating_sub(1))
            };
            let page_h = doc.pages.get(page_idx).map(|p| p.height).unwrap_or(842.0);
            let pdf_y = page_h - pos.y.val();
            let escaped_name = escape_pdf_dest_name(&name);
            dests_dict.push_str(&format!(
                "{} [{} /XYZ {:.2} {:.2} null] ",
                escaped_name, page_ref, pos.x.val(), pdf_y
            ));
        }
        dests_dict.push_str(">>");

        self.add(dests_id, dests_dict);
        self.add(names_id, format!("<< /Dests {dests_id} 0 R >>"));

        // Editar objecto 1 (catalog) para incluir /Names.
        if let Some((_, content)) = self.objects.iter_mut().find(|(id, _)| *id == 1) {
            let s = String::from_utf8_lossy(content);
            if let Some(idx) = s.rfind(">>") {
                let mut new = s[..idx].to_string();
                new.push_str(&format!(" /Names {names_id} 0 R"));
                new.push_str(&s[idx..]);
                *content = new.into_bytes();
            }
        }
    }

    /// **P535** — Emite a árvore `/Outlines` no catálogo (objecto 1) a partir
    /// dos headings registados em `PagedDocument::extracted_headings`.
    ///
    /// Cada bookmark aponta para a página e posição `(x, y-up)` do heading via
    /// `extracted_label_pages` / `extracted_label_positions`. O aninhamento usa
    /// `/Parent`, `/Prev`, `/Next`, `/First` e `/Last`.
    fn emit_outlines(&mut self, doc: &PagedDocument) {
        const FIRST_PAGE_ID: usize = 3;

        if doc.extracted_headings.is_empty() {
            return;
        }

        struct Node {
            id:          usize,
            title:       String,
            page_ref:    String,
            x:           f64,
            y:           f64,
            parent:      Option<usize>,
            prev:        Option<usize>,
            next:        Option<usize>,
            first_child: Option<usize>,
            last_child:  Option<usize>,
            child_count: usize,
        }

        let mut nodes: Vec<Node> = Vec::new();
        let mut stack: Vec<usize> = Vec::new();

        for (label, _number, body, level) in &doc.extracted_headings {
            let page = doc.extracted_label_pages.get(label).copied().unwrap_or(1);
            let pos = doc.extracted_label_positions.get(label).copied().unwrap_or(Point::ZERO);
            let page_idx = page.saturating_sub(1);
            let page_ref = if page_idx < doc.pages.len() {
                format!("{} 0 R", FIRST_PAGE_ID + page_idx)
            } else {
                format!("{} 0 R", FIRST_PAGE_ID + doc.pages.len().saturating_sub(1))
            };
            let page_h = doc.pages.get(page_idx).map(|p| p.height).unwrap_or(842.0);
            let pdf_y = page_h - pos.y.val();
            let title = body.plain_text();

            // Fechar níveis até ao pai correcto.
            while stack.len() >= *level {
                stack.pop();
            }
            let parent = stack.last().copied();
            let node_idx = nodes.len();

            // Irmão anterior: último filho do pai, ou último top-level.
            let prev = if let Some(p) = parent {
                nodes[p].last_child
            } else {
                nodes.iter().rposition(|n| n.parent.is_none())
            };

            nodes.push(Node {
                id:          0, // preenchido depois
                title,
                page_ref,
                x:           pos.x.val(),
                y:           pdf_y,
                parent,
                prev,
                next:        None,
                first_child: None,
                last_child:  None,
                child_count: 0,
            });

            if let Some(p) = parent {
                if nodes[p].first_child.is_none() {
                    nodes[p].first_child = Some(node_idx);
                }
                nodes[p].last_child = Some(node_idx);
                nodes[p].child_count += 1;
            }
            if let Some(prev_idx) = prev {
                nodes[prev_idx].next = Some(node_idx);
            }

            stack.push(node_idx);
        }

        // Alocar object IDs após todos os objectos existentes.
        let mut next_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;
        let root_id = next_id;
        next_id += 1;
        let first_node_id = next_id;
        for (i, node) in nodes.iter_mut().enumerate() {
            node.id = first_node_id + i;
        }

        // Emitir cada item de outline.
        for node in &nodes {
            let title_hex = utf16be_hex_string(&node.title);
            let parent_ref = node.parent
                .map(|p| format!("{} 0 R", nodes[p].id))
                .unwrap_or_else(|| format!("{root_id} 0 R"));
            let mut dict = format!(
                "<< /Title {title_hex} /Parent {parent_ref} /Dest [{} /XYZ {:.2} {:.2} null]",
                node.page_ref, node.x, node.y
            );
            if let Some(prev_idx) = node.prev {
                dict.push_str(&format!(" /Prev {} 0 R", nodes[prev_idx].id));
            }
            if let Some(next_idx) = node.next {
                dict.push_str(&format!(" /Next {} 0 R", nodes[next_idx].id));
            }
            if let Some(first_idx) = node.first_child {
                dict.push_str(&format!(" /First {} 0 R", nodes[first_idx].id));
            }
            if let Some(last_idx) = node.last_child {
                dict.push_str(&format!(" /Last {} 0 R", nodes[last_idx].id));
            }
            // P602 — /Count com sinal negativo para entradas com filhos
            // (fechadas por defeito), seguindo o vanilla 0.15.0.
            if node.child_count > 0 {
                dict.push_str(&format!(" /Count -{}", node.child_count));
            }
            dict.push_str(" >>");
            self.add(node.id, dict);
        }

        let first_top = nodes.iter().find(|n| n.parent.is_none()).map(|n| n.id).unwrap_or(0);
        let last_top = nodes.iter().rfind(|n| n.parent.is_none()).map(|n| n.id).unwrap_or(0);
        // P602 — /Count na raiz é o número de bookmarks de topo (sem pai),
        // positivo (abertos por defeito).
        let top_count = nodes.iter().filter(|n| n.parent.is_none()).count();
        self.add(root_id, format!(
            "<< /Type /Outlines /First {first_top} 0 R /Last {last_top} 0 R /Count {top_count} >>"
        ));

        // Editar objecto 1 (catalog) para incluir /Outlines.
        if let Some((_, content)) = self.objects.iter_mut().find(|(id, _)| *id == 1) {
            let s = String::from_utf8_lossy(content);
            if let Some(idx) = s.rfind(">>") {
                let mut new = s[..idx].to_string();
                new.push_str(&format!(" /Outlines {root_id} 0 R"));
                new.push_str(&s[idx..]);
                *content = new.into_bytes();
            }
        }
    }

    /// **P536/P601** — emite o dicionário `/Info` com Title, Author, Keywords,
    /// Creator, CreationDate e ModDate. Chamado no final de cada caminho de
    /// build antes de `serialize`. O dicionário é sempre emitido, mesmo quando
    /// `document_info` não tem campos de utilizador (P601).
    fn emit_info(&mut self, doc: &PagedDocument) {
        let mut parts = Vec::new();
        if let Some(title) = &doc.document_info.title {
            parts.push(format!("/Title {}", utf16be_hex_string(title.as_str())));
        }
        if let Some(author) = &doc.document_info.author {
            parts.push(format!("/Author {}", utf16be_hex_string(author.as_str())));
        }
        if let Some(keywords) = &doc.document_info.keywords {
            parts.push(format!("/Keywords {}", utf16be_hex_string(keywords.as_str())));
        }

        // Formato PDF: D:YYYYMMDDHHMMSS (UTC).
        let now = current_pdf_timestamp();
        let date = format!(
            "D:{:04}{:02}{:02}{:02}{:02}{:02}",
            now.year(),
            now.month() as u8,
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        );
        parts.push(format!("/CreationDate ({})" , date));
        parts.push(format!("/ModDate ({})" , date));
        parts.push("/Creator (typst-crystalline)".to_string());

        let next_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;
        self.add(next_id, format!("<< {} >>", parts.join(" ")));
        self.info_id = Some(next_id);
    }

    /// **P611** — emite o stream `/Type /Metadata /Subtype /XML` com um pacote
    /// XMP mínimo, referenciado a partir de `/Metadata` no catálogo. Sempre
    /// emitido, mesmo quando `document_info` está vazio.
    fn emit_xmp_metadata(&mut self, doc: &PagedDocument) {
        let now = current_pdf_timestamp();
        let xmp_date = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            now.year(),
            now.month() as u8,
            now.day(),
            now.hour(),
            now.minute(),
            now.second()
        );

        let n_pages = doc.pages.len().max(1);

        // Campos condicionais a metadados do utilizador.
        let title_elem = doc
            .document_info
            .title
            .as_ref()
            .map(|t| format!(
                "<dc:title><rdf:Alt><rdf:li xml:lang=\"x-default\">{}</rdf:li></rdf:Alt></dc:title>",
                escape_xml_text(t.as_str())
            ))
            .unwrap_or_default();
        let keywords_elem = doc
            .document_info
            .keywords
            .as_ref()
            .map(|k| format!(
                "<pdf:Keywords>{}</pdf:Keywords>",
                escape_xml_text(k.as_str())
            ))
            .unwrap_or_default();
        let creator_elem = doc
            .document_info
            .author
            .as_ref()
            .map(|a| format!(
                "<dc:creator><rdf:Seq><rdf:li>{}</rdf:li></rdf:Seq></dc:creator>",
                escape_xml_text(a.as_str())
            ))
            .unwrap_or_default();

        // Identificadores (16 bytes em base64).
        // P615 — aleatórios por defeito; P617 — DocumentID pode ser fixado
        // externamente. InstanceID continua aleatório (ou fixo em testes).
        let (instance_id, document_id) = xmp_instance_and_document_id(self.document_id);

        // Estrutura exacta do vanilla (krilla + xmp-writer), sem quebras de
        // linha entre elementos, para manter a mesma forma do pacote XMP.
        let xml = format!(
            "<?xpacket begin=\"﻿\" id=\"W5M0MpCehiHzreSzNTczkc9d\"?>\
             <x:xmpmeta xmlns:x=\"adobe:ns:meta/\" x:xmptk=\"xmp-writer\">\
             <rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">\
             <rdf:Description rdf:about=\"\" xmlns:dc=\"http://purl.org/dc/elements/1.1/\" \
             xmlns:xmp=\"http://ns.adobe.com/xap/1.0/\" \
             xmlns:xmpMM=\"http://ns.adobe.com/xap/1.0/mm/\" \
             xmlns:xmpTPg=\"http://ns.adobe.com/xap/1.0/t/pg/\" \
             xmlns:pdf=\"http://ns.adobe.com/pdf/1.3/\">\
             {title}\
             {keywords}\
             {creator}\
             <xmp:CreatorTool>typst-crystalline</xmp:CreatorTool>\
             <dc:language><rdf:Bag><rdf:li>en</rdf:li></rdf:Bag></dc:language>\
             <xmp:ModifyDate>{date}</xmp:ModifyDate>\
             <xmp:CreateDate>{date}</xmp:CreateDate>\
             <xmpTPg:NPages>{n_pages}</xmpTPg:NPages>\
             <dc:format>application/pdf</dc:format>\
             <xmpMM:InstanceID>{instance_id}</xmpMM:InstanceID>\
             <xmpMM:DocumentID>{document_id}</xmpMM:DocumentID>\
             <xmpMM:RenditionClass>proof</xmpMM:RenditionClass>\
             <pdf:PDFVersion>1.7</pdf:PDFVersion></rdf:Description></rdf:RDF></x:xmpmeta>\
             <?xpacket end=\"r\"?>",
            title = title_elem,
            keywords = keywords_elem,
            creator = creator_elem,
            date = xmp_date,
            n_pages = n_pages,
            instance_id = instance_id,
            document_id = document_id,
        );

        let next_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;
        let len = xml.len();
        let mut obj = format!(
            "<< /Length {len} /Type /Metadata /Subtype /XML >>\nstream\n"
        )
        .into_bytes();
        obj.extend_from_slice(xml.as_bytes());
        obj.extend_from_slice(b"\nendstream");
        self.add_bytes(next_id, obj);
        self.xmp_id = Some(next_id);

        // Editar objecto 1 (catálogo) para incluir /Metadata.
        if let Some((_, content)) = self.objects.iter_mut().find(|(id, _)| *id == 1) {
            let s = String::from_utf8_lossy(content);
            if let Some(idx) = s.rfind(">>") {
                let mut new = s[..idx].to_string();
                new.push_str(&format!(" /Metadata {next_id} 0 R"));
                new.push_str(&s[idx..]);
                *content = new.into_bytes();
            }
        }
    }

    fn serialize(self) -> Vec<u8> {
        // Header — %PDF-1.7 + comentário binário (4 bytes > 127)
        let mut out: Vec<u8> = b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n".to_vec();
        let mut offsets: HashMap<usize, usize> = Default::default();

        for (id, content) in &self.objects {
            offsets.insert(*id, out.len());
            out.extend_from_slice(format!("{id} 0 obj\n").as_bytes());
            out.extend_from_slice(content);
            out.extend_from_slice(b"\nendobj\n");
        }

        // xref table
        let xref_start = out.len();
        let max_id = offsets.keys().copied().max().unwrap_or(0);
        out.extend_from_slice(b"xref\n");
        out.extend_from_slice(format!("0 {}\n", max_id + 1).as_bytes());
        out.extend_from_slice(b"0000000000 65535 f \n");
        for id in 1..=max_id {
            let off = offsets.get(&id).copied().unwrap_or(0);
            out.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
        }

        // Trailer
        let info_ref = self
            .info_id
            .map(|id| format!(" /Info {id} 0 R"))
            .unwrap_or_default();
        out.extend_from_slice(format!(
            "trailer\n<< /Size {} /Root 1 0 R{} >>\nstartxref\n{}\n%%EOF\n",
            max_id + 1, info_ref, xref_start
        ).as_bytes());

        out
    }
}

/// **P611** — escapa caracteres XML especiais para texto dentro de elementos.
fn escape_xml_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            ch => out.push(ch),
        }
    }
    out
}

/// Escapa caracteres problemáticos para uma string PDF dentro de `(...)`.
/// Mantém a URI o mais intacta possível; escapa apenas `(`, `)`, `\` e
/// caracteres de controlo (0x00–0x1f, 0x7f) por compatibilidade com leitores.
fn escape_pdf_uri(uri: &str) -> String {
    escape_pdf_literal(uri)
}

/// **P536** — escapa uma string literal PDF (operando `(...)`).
fn escape_pdf_literal(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('(');
    for b in s.bytes() {
        match b {
            b'(' => out.push_str("\\("),
            b')' => out.push_str("\\)"),
            b'\\' => out.push_str("\\\\"),
            0x00..=0x1f | 0x7f => out.push_str(&format!("\\{:03o}", b)),
            _ => out.push(b as char),
        }
    }
    out.push(')');
    out
}

/// Escapa o nome de um destino PDF. Se contiver caracteres fora do conjunto
/// de nomes PDF, usa notação hexadecimal `<...>`; caso contrário usa `/name`.
fn escape_pdf_dest_name(name: &str) -> String {
    let needs_hex = name.is_empty()
        || name.bytes().any(|b| {
            !matches!(b,
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' |
                b'_' | b'-' | b'.' | b':' | b'/' | b'@' | b'*' | b'+'
            )
        });
    if needs_hex {
        let hex: String = name.bytes().map(|b| format!("{:02X}", b)).collect();
        format!("<{}>", hex)
    } else {
        format!("/{}", name)
    }
}

/// Converte uma string para representação PDF UTF-16BE com BOM, em hex.
/// Resultado: `<FEFF...>` adequado para valores `/Title` em bookmarks.
fn utf16be_hex_string(s: &str) -> String {
    let mut bytes: Vec<u8> = Vec::with_capacity(2 + s.encode_utf16().count() * 2);
    bytes.extend_from_slice(&[0xFE, 0xFF]); // BOM
    for unit in s.encode_utf16() {
        bytes.extend_from_slice(&unit.to_be_bytes());
    }
    let hex: String = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    format!("<{hex}>")
}

/// Recolhe `FrameItem::Link` de uma lista de items, incluindo links aninhados
/// dentro de `Group`.
fn collect_links(items: &[FrameItem], out: &mut Vec<(LinkTarget, Point, Size)>) {
    for item in items {
        match item {
            FrameItem::Link { target, items, pos, size } => {
                out.push((target.clone(), *pos, *size));
                collect_links(items, out);
            }
            FrameItem::Group { items, .. } => collect_links(items, out),
            _ => {}
        }
    }
}

