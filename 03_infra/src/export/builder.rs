//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/builder.md
//! @prompt-hash c06b0d16
//! @layer L3
//! @updated 2026-05-19
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
use std::sync::Arc;

use ttf_parser::Face;
use typst_core::entities::font_book::FontVariant;
use typst_core::entities::font_list::FontList;
use typst_core::entities::layout_types::{FrameItem, LinkTarget, PagedDocument, Point, Size};

use crate::font_variant::{axis_variations_for_font_variant, instantiate_variable_font};
use ecow::EcoString;

use super::{
    adaptive_n_for_stops, apply_parent_transform, build_jpeg_xobject,
    build_page_stream, build_png_rgb_xobject, build_png_smask_xobject,
    collect_codepoints, collect_glyph_ids, collect_shaped_cluster_texts,
    collect_shaped_glyph_mappings,
    compute_axial_coords, compute_coons_patches_n_stops,
    compute_coons_patches_n_stops_extended, compute_radial_coords,
    emit_conic_coons_stream_cmyk, emit_conic_coons_stream_rgb,
    emit_function_dict, emit_function_dict_cmyk, jpeg_color_space,
    map_chars_to_glyphs, multispace_sample_stops, multispace_sample_stops_conic,
    multispace_sample_stops_linear_cmyk, multispace_sample_stops_radial,
    multispace_sample_stops_radial_cmyk, pattern_resources_for_page,
    resolve_relative, scan_all_gradients, scan_all_images,
    subset::{remap_glyph_id, subset_font_with_mapping, FontSubset},
    char_to_utf16_hex, text_to_hex_string, to_unicode_cmap, widths_array,
    xobject_resources_for_page, FontScenario, GradientObject,
    GradientObjectKind, ImageRef, ImageXObject, PageContext, PatternRef,
};

use crate::font_metrics::build_math_glyph_reverse_map;

fn duration_ms(d: std::time::Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

/// P517 — gera nome de fonte com prefixo de subset quando a fonte foi
/// efectivamente subsetada. Usa o prefixo fixo `AAAAAA+` conforme
/// convenção PDF para fontes subsetadas (ex: `AAAAAA+FontName`).
fn subset_font_name(base_name: &str, _subset_data: &[u8]) -> String {
    format!("AAAAAA+{}", base_name)
}

// ── Builder ────────────────────────────────────────────────────────────────

pub(super) struct PdfBuilder {
    objects: Vec<(usize, Vec<u8>)>,
    /// P518 — tempo acumulado em `subset_font_with_mapping` (ms).
    subset_ms: f64,
}

impl PdfBuilder {
    pub(super) fn new() -> Self {
        Self { objects: Vec::new(), subset_ms: 0.0 }
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
        let first_img_id = font_f3 + 1;

        let (img_refs, ptr_to_idx, img_xobjects) = scan_all_images(doc, first_img_id);

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

        self.emit_image_xobjects(img_xobjects);

        // P263 — Emit Function/Shading/Pattern objects para gradients.
        let page_dimensions: Vec<(f64, f64)> = doc.pages.iter()
            .map(|p| (p.width, p.height)).collect();
        self.emit_gradient_objects(grad_objs, &page_dimensions, &mut next_sub_id);

        self.emit_link_annotations(doc);
        self.emit_named_destinations(doc);
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
        let first_img_id       = to_unicode_id + 1;

        let chars = collect_codepoints(doc);
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
        let all_glyph_ids = collect_glyph_ids(doc);
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

        let (img_refs, ptr_to_idx, img_xobjects) = scan_all_images(doc, first_img_id);

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

        // Type0 font (F1)
        self.add(font_id, format!(
            "<< /Type /Font /Subtype /Type0 /BaseFont /{base_font_name} \
               /Encoding /Identity-H \
               /DescendantFonts [{cidfont_id} 0 R] \
               /ToUnicode {to_unicode_id} 0 R >>"
        ));

        // CIDFont
        self.add(cidfont_id, format!(
            "<< /Type /Font /Subtype /CIDFontType2 /BaseFont /{base_font_name} \
               /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> \
               /FontDescriptor {font_descriptor_id} 0 R \
               /DW 500 \
               /W [{widths}] >>"
        ));

        // FontDescriptor
        self.add(font_descriptor_id, format!(
            "<< /Type /FontDescriptor /FontName /{base_font_name} \
               /Flags 32 \
               /FontBBox [-1000 -200 2000 900] \
               /ItalicAngle 0 /Ascent 800 /Descent -200 \
               /CapHeight 700 /StemV 80 \
               /FontFile2 {font_stream_id} 0 R >>"
        ));

        // Font data stream — P516: usa subset se possível, senão fonte completa.
        let font_len = embed_font_data.len();
        let mut font_stream = format!(
            "<< /Length {font_len} /Subtype /CIDFontType2 >>\nstream\n"
        ).into_bytes();
        font_stream.extend_from_slice(&embed_font_data);
        font_stream.extend_from_slice(b"\nendstream");
        self.add_bytes(font_stream_id, font_stream);

        // ToUnicode CMap stream
        let cmap = to_unicode_cmap(&to_unicode_mappings);
        let cmap_len = cmap.len();
        let mut cmap_obj = format!("<< /Length {cmap_len} >>\nstream\n").into_bytes();
        cmap_obj.extend_from_slice(&cmap);
        cmap_obj.extend_from_slice(b"\nendstream");
        self.add_bytes(to_unicode_id, cmap_obj);

        self.emit_image_xobjects(img_xobjects);

        // P263 — Emit gradient objects.
        let page_dimensions: Vec<(f64, f64)> = doc.pages.iter()
            .map(|p| (p.width, p.height)).collect();
        self.emit_gradient_objects(grad_objs, &page_dimensions, &mut next_sub_id);

        self.emit_link_annotations(doc);
        self.emit_named_destinations(doc);
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
        let first_img_id = fonts_start + 5 * n_fonts;

        // Codepoints + glyph mappings por font. Cada font tem o seu
        // mapping (chars partilhados; gids específicos da face).
        let chars = collect_codepoints(doc);
        let glyph_ids = collect_glyph_ids(doc);
        // P520 — glifos reais do shaper (ligatures) mapeados para o primeiro
        // caractere do cluster. Prioridade idêntica a build_cidfont.
        let shaped_mappings = collect_shaped_glyph_mappings(doc);
        let mut per_font_mappings: Vec<Vec<(u16, String)>> = Vec::with_capacity(n_fonts);
        let mut per_font_char_to_gid: Vec<HashMap<char, u16>> = Vec::with_capacity(n_fonts);
        let mut per_font_widths: Vec<String> = Vec::with_capacity(n_fonts);
        let mut per_font_embed_data: Vec<Vec<u8>> = Vec::with_capacity(n_fonts);
        let mut per_font_glyph_mapping: Vec<HashMap<u16, u16>> = Vec::with_capacity(n_fonts);
        let mut per_font_glyph_to_nominal: Vec<HashMap<u16, i32>> = Vec::with_capacity(n_fonts);
        for face in faces {
            let mut mappings = map_chars_to_glyphs(face, &chars);

            // P520 — larguras nominais (hmtx) desta face para todos os glyph IDs
            // que podem aparecer no stream.
            let mut glyph_to_nominal: HashMap<u16, i32> = HashMap::new();
            for &gid in glyph_ids.iter().chain(mappings.iter().map(|(_, gid)| gid)) {
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

        let (img_refs, ptr_to_idx, img_xobjects) = scan_all_images(doc, first_img_id);

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

            // Type0
            self.add(type0_id, format!(
                "<< /Type /Font /Subtype /Type0 /BaseFont /{name} \
                   /Encoding /Identity-H \
                   /DescendantFonts [{cidfont_id} 0 R] \
                   /ToUnicode {to_unicode_id} 0 R >>"
            ));

            // CIDFont
            self.add(cidfont_id, format!(
                "<< /Type /Font /Subtype /CIDFontType2 /BaseFont /{name} \
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
                   /FontFile2 {stream_id} 0 R >>"
            ));

            // FontFile2 stream — P516: usa subset se possível, senão fonte completa.
            let font_len = font_data.len();
            let mut font_stream = format!(
                "<< /Length {font_len} /Subtype /CIDFontType2 >>\nstream\n"
            ).into_bytes();
            font_stream.extend_from_slice(font_data);
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

        self.emit_image_xobjects(img_xobjects);

        // P263 — Emit gradient objects.
        let page_dimensions: Vec<(f64, f64)> = doc.pages.iter()
            .map(|p| (p.width, p.height)).collect();
        self.emit_gradient_objects(grad_objs, &page_dimensions, &mut next_sub_id);

        self.emit_link_annotations(doc);
        self.emit_named_destinations(doc);
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
                ImageXObject::Jpeg { data, main_obj_id, iw, ih } => {
                    let cs = jpeg_color_space(&data);
                    self.add_bytes(main_obj_id, build_jpeg_xobject(&data, iw, ih, cs));
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
        out.extend_from_slice(format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n",
            max_id + 1, xref_start
        ).as_bytes());

        out
    }
}

/// Escapa caracteres problemáticos para uma string PDF dentro de `(...)`.
/// Mantém a URI o mais intacta possível; escapa apenas `(`, `)`, `\` e
/// caracteres de controlo (0x00–0x1f, 0x7f) por compatibilidade com leitores.
fn escape_pdf_uri(uri: &str) -> String {
    let mut out = String::with_capacity(uri.len());
    for b in uri.bytes() {
        match b {
            b'(' => out.push_str("\\("),
            b')' => out.push_str("\\)"),
            b'\\' => out.push_str("\\\\"),
            0x00..=0x1f | 0x7f => out.push_str(&format!("\\{:03o}", b)),
            _ => out.push(b as char),
        }
    }
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

