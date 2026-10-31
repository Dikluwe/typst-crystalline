//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/builder/fonts.md
//! @prompt-hash 4eba9d7e
//! @layer L3
//! @updated 2026-10-24

use super::navigation::PageIndexConverter;
use super::resources::{colorspace_resource_entry, has_rgb_jpeg, merge_bitmap_xobjects};
use super::serialization::insert_pdf_dict_entry;
use super::{format_dim, PdfBuilder};
use crate::export::build_icc_profile_stream;
use crate::export::build_page_stream;
use crate::export::char_to_utf16_hex;
use crate::export::collect_codepoints;
use crate::export::collect_glyph_ids;
use crate::export::collect_shaped_cluster_texts;
use crate::export::collect_shaped_glyph_mappings;
use crate::export::collect_text_codepoints;
use crate::export::compress_zlib;
use crate::export::map_chars_to_glyphs;
use crate::export::pattern_resources_for_page;
use crate::export::scan_all_gradients;
use crate::export::scan_all_images;
use crate::export::srgb_icc_profile_for_version;
use crate::export::stream::FontScenario;
use crate::export::subset::remap_glyph_id;
use crate::export::subset::subset_font_with_mapping;
use crate::export::subset::FontSubset;
use crate::export::tiling::walk_resources as walk_frame_items;
use crate::export::to_unicode_cmap;
use crate::export::widths_array;
use crate::export::xobject_resources_for_page;
use crate::export::FontKey;
use crate::export::ImageXObject;
use crate::export::PageContext;
use crate::export::PdfExport;
use crate::export::StreamMode;
use crate::font_metrics::build_math_glyph_reverse_map;
use crate::font_variant::axis_variations_for_font_variant;
use crate::font_variant::instantiate_variable_font;
use crate::font_variant::merge_explicit_variations;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use ttf_parser::Face;
use typst_core::entities::document_format_options::PdfAccessibility;
#[cfg(test)]
use typst_core::entities::font_list::FontList;
#[cfg(test)]
use typst_core::entities::font_variations::FontVariations;
use typst_core::entities::frame_visitor::FrameVisitor;
use typst_core::entities::layout_types::FrameItem;
use typst_core::entities::layout_types::PagedDocument;

fn duration_ms(d: std::time::Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

/// **P941** — glyph ids usados no documento, agrupados por fonte resolvida.
///
/// Percorre os itens do documento e, para cada `Text`/`TextShaped`/`Glyph`,
/// atribui os glyph ids à fonte correspondente (mesma selecção de índice que
/// `emit_shaped_pdf` usa: `font_index_for_style`). Necessário para decidir se
/// uma fonte é 100% bitmap (CBDT) e pode deixar de ser embutida.
fn per_font_used_glyphs(
    doc: &PagedDocument,
    fonts: &[FontKey],
    faces: &[Face<'_>],
) -> Vec<BTreeSet<u16>> {
    struct FontGlyphCollector<'a, 'f> {
        fonts: &'a [FontKey],
        faces: &'a [Face<'f>],
        sets: &'a mut [BTreeSet<u16>],
        cache: crate::export::font_binding::LookupCache,
    }
    impl<'a, 'f> FrameVisitor for FontGlyphCollector<'a, 'f> {
        fn visit_text_shaped(&mut self, item: &FrameItem) {
            if let FrameItem::TextShaped { glyphs, style, font, .. } = item {
                let Some(fi) = self.cache.index(self.fonts, style, font.as_ref()) else {
                    return;
                };
                for g in glyphs {
                    self.sets[fi].insert(g.glyph_id);
                }
            }
        }
        fn visit_text(&mut self, item: &FrameItem) {
            if let FrameItem::TextSeparator { style, .. } = item {
                let fi = crate::export::stream::font_index_for_style(self.fonts, style);
                if let Some(gid) = self.faces[fi].glyph_index(' ') {
                    self.sets[fi].insert(gid.0);
                }
            }
            if let FrameItem::Text { text, style, .. } = item {
                let fi = crate::export::stream::font_index_for_style(self.fonts, style);
                let face = &self.faces[fi];
                for c in text.chars() {
                    if let Some(gid) = face.glyph_index(c) {
                        self.sets[fi].insert(gid.0);
                    }
                }
            }
        }
        fn visit_glyph(&mut self, item: &FrameItem) {
            if let FrameItem::Glyph { glyph_id, style, .. } = item {
                let fi = crate::export::stream::font_index_for_style(self.fonts, style);
                self.sets[fi].insert(*glyph_id);
            }
        }
    }
    let mut sets: Vec<BTreeSet<u16>> = vec![BTreeSet::new(); fonts.len()];
    let mut collector = FontGlyphCollector {
        fonts,
        faces,
        sets: &mut sets,
        cache: Default::default(),
    };
    for page in &doc.pages {
        for items in [&page.background, &page.items, &page.foreground] {
            walk_frame_items(&mut collector, items);
        }
    }
    sets
}

/// P517 — gera nome de fonte com prefixo de subset quando a fonte foi
/// efectivamente subsetada. Usa o prefixo fixo `AAAAAA+` conforme
/// convenção PDF para fontes subsetadas (ex: `AAAAAA+FontName`).
fn subset_font_name(base_name: &str, _subset_data: &[u8]) -> String {
    format!("AAAAAA+{}", base_name)
}

/// **P950** — nome real da fonte para `BaseFont`/`FontName`, lido da tabela
/// `name` dos bytes da fonte embutida (PostScript name_id 6 → full name 4 →
/// família 1), sem espaços (convenção BaseFont). Se a leitura falhar (bytes
/// inválidos, subset sem tabela `name`, ausência dos ids), devolve
/// `fallback` (o genérico anterior).
fn real_base_name(font_data: &[u8], fallback: &str) -> String {
    let sanitize = |s: &str| -> String { s.split_whitespace().collect() };
    let Ok(face) = ttf_parser::Face::parse(font_data, 0) else {
        return fallback.to_string();
    };
    for name_id in [6u16, 4, 1] {
        if let Some(entry) = face.names().into_iter().find(|n| n.name_id == name_id) {
            if let Some(s) = entry.to_string() {
                if !s.is_empty() {
                    return sanitize(&s);
                }
            }
        }
    }
    fallback.to_string()
}

/// P560 — extrai a tabela `CFF ` de uma fonte OpenType/SFNT.
///
/// Leitores de PDF esperam o programa CFF puro quando o descritor indica
/// `/Subtype /CIDFontType0C`; o contêiner SFNT completo causa mismatch.
///
/// **P797** — `ttf_parser::Face::parse` falha quando o scalerType do cabeçalho
/// SFNT é `00010000` (TrueType) mas o conteúdo tem tabela `CFF ` (comportamento
/// observado em subsets gerados pelo `oxifont_subset` para Libertinus Serif e
/// outras fontes CFF: o subsetter preserva as tabelas mas mantém o scalerType
/// original da fonte original como `00010000`). Correcção: leitura directa das
/// table tags do cabeçalho SFNT sem passar por `Face::parse`, garantindo que a
/// detecção funciona independentemente do valor do scalerType.
fn cff_table_data(font_data: &[u8]) -> Option<&[u8]> {
    // Leitura directa do cabeçalho SFNT:
    // Offset 4–5: numTables (u16 big-endian)
    // Tabelas: a partir do offset 12, cada entrada é 16 bytes:
    //   [0–3] tag (4 bytes), [4–7] checksum, [8–11] offset, [12–15] length
    if font_data.len() < 12 {
        return None;
    }
    let n_tables = u16::from_be_bytes([font_data[4], font_data[5]]) as usize;
    for i in 0..n_tables {
        let entry_off = 12 + i * 16;
        if entry_off + 16 > font_data.len() {
            break;
        }
        let tag = &font_data[entry_off..entry_off + 4];
        if tag == b"CFF " {
            let offset = u32::from_be_bytes([
                font_data[entry_off + 8],
                font_data[entry_off + 9],
                font_data[entry_off + 10],
                font_data[entry_off + 11],
            ]) as usize;
            let length = u32::from_be_bytes([
                font_data[entry_off + 12],
                font_data[entry_off + 13],
                font_data[entry_off + 14],
                font_data[entry_off + 15],
            ]) as usize;
            if offset + length <= font_data.len() {
                return Some(&font_data[offset..offset + length]);
            }
        }
    }
    None
}

/// **P772u** — detecta fontes CFF2 (`CFF2`, não `CFF `) — usadas por fontes
/// variáveis OpenType (ex.: Cantarell-VF.otf). `ttf_parser::Face::tables()`
/// expõe `cff` (CFF1) e `cff2` como campos **distintos** — `cff_table_data`
/// só verificava `cff`, pelo que uma fonte CFF2 sem tabela `CFF ` caía
/// silenciosamente no ramo TrueType (`/CIDFontType2`/`/FontFile2`), mesmo
/// não tendo tabela `glyf`. Achado por instrumentação (comparação
/// `ttf_parser`/`rustybuzz`, ambos correctos e concordantes — o defeito não
/// estava no shaping, mas no `/Subtype`/`FontFile` PDF declarado para o
/// programa de fonte embutido). Ver `paridade-producao-p772u.md`.
///
/// **P797/P882** — devolve `true` se os bytes da fonte SFNT contiverem a tabela
/// indicada. Usada por `font_embedding_data` para seleccionar o modo de
/// embutimento correcto.
fn has_sfnt_table(font_data: &[u8], wanted: &[u8; 4]) -> bool {
    if font_data.len() < 12 {
        return false;
    }
    let n_tables = u16::from_be_bytes([font_data[4], font_data[5]]) as usize;
    for i in 0..n_tables {
        let entry_off = 12 + i * 16;
        if entry_off + 4 > font_data.len() {
            break;
        }
        let tag = &font_data[entry_off..entry_off + 4];
        if tag == wanted {
            return true;
        }
    }
    false
}

fn has_cff_table(font_data: &[u8]) -> bool {
    has_sfnt_table(font_data, b"CFF ")
}

fn has_cff2_table(font_data: &[u8]) -> bool {
    has_sfnt_table(font_data, b"CFF2")
}

/// P560/P772u/P882 — devolve os componentes PDF correctos e os bytes a embeber.
///
/// TrueType (`glyf`) usa `/CIDFontType2` + `/FontFile2` + stream `/CIDFontType2`
/// (fonte SFNT completa).
///
/// CFF1/OpenType usa `/CIDFontType0` + `/FontFile3` + stream `/CIDFontType0C`,
/// embutindo **apenas** a tabela `CFF ` (programa CFF puro). O wrapper SFNT
/// completo acrescenta ~1.5 KB por ocorrência sem benefício para leitores PDF,
/// que esperam CFF puro neste subtipo (P882).
///
/// CFF2/OpenType (fontes variáveis) usa `/CIDFontType0` + `/FontFile3` + stream
/// `/OpenType` (contêiner SFNT completo), porque o spec PDF não define um
/// subtipo para "programa CFF2 puro".
fn font_embedding_data(
    font_data: &[u8],
) -> (&'static str, &'static str, &'static str, &[u8]) {
    // CFF1: embutir o programa CFF puro (/CIDFontType0C).
    if has_cff_table(font_data) {
        if let Some(cff) = cff_table_data(font_data) {
            return ("/CIDFontType0", "/FontFile3", "CIDFontType0C", cff);
        }
    }
    // CFF2 (OpenType variável): não existe subtipo para CFF2 puro,
    // pelo que se embute o contêiner SFNT completo.
    if has_cff2_table(font_data) {
        return ("/CIDFontType0", "/FontFile3", "OpenType", font_data);
    }
    // TrueType.
    ("/CIDFontType2", "/FontFile2", "CIDFontType2", font_data)
}

/// **P883** — constrói o stream PDF para a fonte embutida, comprimindo com
/// FlateDecode quando possível (paridade com o vanilla 0.15.0).
///
/// Em caso de falha do compressor (input muito pequeno ou erro interno),
/// emite o stream sem compressão — o PDF continua válido.
///
/// **P940** — quando a fonte embutida é o fallback integral (subset falhou,
/// ex.: fontes CBDT/emoji), o stream tem vários megabytes e a compressão
/// Flate domina o `render_ms` (~300 ms medidos para ~10.8 MB). Nesse caso,
/// emitir sem compressão: o custo de CPU cai para uma cópia de memória, ao
/// preço de um PDF maior. O caminho normal (subset bem-sucedido) continua
/// comprimido.
fn build_font_stream(stream_subtype: &str, font_stream_data: &[u8]) -> Vec<u8> {
    /// Limiar acima do qual a compressão do stream de fonte custa mais do que
    /// vale (P940). Subsets normais ficam bem abaixo deste valor.
    const MAX_COMPRESS_FONT_STREAM: usize = 256 * 1024;

    if font_stream_data.len() <= MAX_COMPRESS_FONT_STREAM {
        if let Ok(compressed) = compress_zlib(font_stream_data) {
            let len = compressed.len();
            let mut stream = format!(
                "<< /Length {len} /Filter /FlateDecode /Subtype /{stream_subtype} >>\nstream\n"
            )
            .into_bytes();
            stream.extend_from_slice(&compressed);
            stream.extend_from_slice(b"\nendstream");
            return stream;
        }
    }

    let len = font_stream_data.len();
    let mut stream =
        format!("<< /Length {len} /Subtype /{stream_subtype} >>\nstream\n").into_bytes();
    stream.extend_from_slice(font_stream_data);
    stream.extend_from_slice(b"\nendstream");
    stream
}

/// **P1477** — constrói o stream `/CIDSet` — um bit por CID presente no
/// subset, `bit=1` se presente, resto do último byte a zero. Aceita
/// qualquer conjunto de CIDs (não assume contiguidade), comprimido com
/// FlateDecode (mesmo padrão de `build_icc_profile_stream`/P883).
fn build_cidset_stream(cids: &HashSet<u16>) -> Vec<u8> {
    let max_cid = cids.iter().copied().max().unwrap_or(0);
    let n_bytes = (max_cid as usize / 8) + 1;
    let mut bits = vec![0u8; n_bytes];
    for &cid in cids {
        bits[cid as usize / 8] |= 0x80 >> (cid as usize % 8);
    }
    let compressed = compress_zlib(&bits).unwrap_or(bits);
    let len = compressed.len();
    let mut obj =
        format!("<< /Length {len} /Filter /FlateDecode >>\nstream\n").into_bytes();
    obj.extend_from_slice(&compressed);
    obj.extend_from_slice(b"\nendstream");
    obj
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
            (typo_asc as f64 * scale, typo_desc as f64 * scale)
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

    FontDescriptorMetrics {
        font_bbox,
        italic_angle,
        ascent,
        descent,
        cap_height,
    }
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

impl Default for FontDescriptorMetrics {
    fn default() -> Self {
        Self {
            font_bbox: [-1000.0, -200.0, 2000.0, 900.0],
            italic_angle: 0.0,
            ascent: 800.0,
            descent: -200.0,
            cap_height: 700.0,
        }
    }
}

fn seed_space_unicode(
    mappings: &[(char, u16)],
    clusters: &[(u16, String)],
    glyph_mapping: &HashMap<u16, u16>,
    unicode: &mut Vec<(u16, String)>,
    seen: &mut HashSet<u16>,
) {
    for &(ch, gid) in mappings {
        let aliases_joiner = ch == ' '
            && gid != 0
            && clusters.iter().any(|(old, hex)| {
                remap_glyph_id(*old, glyph_mapping) == gid
                    && !hex.is_empty()
                    && hex.len() % 4 == 0
                    && hex.as_bytes().chunks_exact(4).all(|part| part == b"2060")
            });
        if aliases_joiner && seen.insert(gid) {
            unicode.push((gid, char_to_utf16_hex(ch)));
        }
    }
}

impl PdfBuilder {
    pub(super) fn measure_subset(
        &mut self,
        font_data: &[u8],
        char_to_old_gid: &std::collections::BTreeMap<char, u16>,
        additional_gids: &std::collections::BTreeSet<u16>,
    ) -> Option<FontSubset> {
        let t0 = std::time::Instant::now();
        let result =
            subset_font_with_mapping(font_data, char_to_old_gid, additional_gids);
        self.subset_ms += duration_ms(t0.elapsed());
        result
    }

    // ── Caminho CIDFont (Unicode completo, Identity-H) ─────────────────────

    pub(super) fn build_cidfont(
        mut self,
        doc: &PagedDocument,
        face: &Face<'_>,
        font_data: &[u8],
    ) -> PdfExport<(Vec<u8>, f64)> {
        // **P1468** — ver nota equivalente em `build_helvetica`.
        let pic = PageIndexConverter::new(doc.pages.len(), self.pages.as_ref());
        let n = pic.exported_count();
        let first_page = 3usize;
        let first_stream = first_page + n;
        let font_id = first_stream + n; // Type0 — /F1
        let cidfont_id = font_id + 1;
        let font_descriptor_id = font_id + 2;
        let font_stream_id = font_id + 3;
        let to_unicode_id = font_id + 4;

        // **P777** — reservar ID do perfil ICC sRGB partilhado se houver JPEGs RGB.
        // **P956** — em modo verbose o ICC é embutido SEMPRE, alocado DEPOIS
        // dos objectos de fonte (mesma posição da convenção P777).
        let verbose = self.stream_mode == StreamMode::Verbose;
        let needs_icc = has_rgb_jpeg(doc) || verbose;
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
        for &gid in collect_glyph_ids(doc)
            .iter()
            .chain(mappings.iter().map(|(_, gid)| gid))
        {
            let adv =
                face.glyph_hor_advance(ttf_parser::GlyphId(gid)).unwrap_or(0) as i32;
            glyph_to_nominal.insert(gid, adv);
        }

        // P516 — subsetting TrueType/OpenType.
        // P520: shaped glyphs sobrescrevem codepoints no char_to_old_gid para
        // que ligatures sejam incluídas no subset e tenham ToUnicode parcial.
        // P874 — shaped_mappings contém glifos de TODAS as fontes do documento;
        // só incluir os que pertencem à face actual.
        let n_glyphs = face.number_of_glyphs();
        let mut char_to_old_gid: std::collections::BTreeMap<char, u16> =
            mappings.iter().copied().collect();
        for (&old_gid, &ch) in &shaped_mappings {
            if old_gid < n_glyphs {
                char_to_old_gid.insert(ch, old_gid);
            }
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
        // P874 — `collect_glyph_ids` reúne glyph IDs de todas as fontes do
        // documento. Passar GIDs que não existem na face actual para o
        // subsetter faz com que ele rejeite a fonte como MalformedFont.
        // Filtrar para o range válido desta face antes de subsetar.
        let n_glyphs = face.number_of_glyphs();
        let all_glyph_ids: BTreeSet<u16> =
            all_glyph_ids.into_iter().filter(|&gid| gid < n_glyphs).collect();
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
        // **P805a** — incluir os cluster texts TAMBÉM no embed integral
        // (fallback P797, `glyph_mapping` vazio): antes, o gate
        // `!glyph_mapping.is_empty()` saltava este bloco e as ligaduras
        // ("fi" → gid f_i) ficavam sem entrada ToUnicode — renderizavam, mas
        // a extracção (`pdftotext`) perdia os caracteres. Sem subset, o gid
        // final é o próprio old_gid.
        let mut to_unicode_mappings: Vec<(u16, String)> = Vec::new();
        let mut seen_to_unicode_gids: HashSet<u16> = HashSet::new();
        let single_cluster_texts = collect_shaped_cluster_texts(doc);
        seed_space_unicode(
            &mappings,
            &single_cluster_texts,
            &glyph_mapping,
            &mut to_unicode_mappings,
            &mut seen_to_unicode_gids,
        );
        for (old_gid, hex) in single_cluster_texts {
            let new_gid = if glyph_mapping.is_empty() {
                old_gid
            } else {
                remap_glyph_id(old_gid, &glyph_mapping)
            };
            if new_gid != 0 && seen_to_unicode_gids.insert(new_gid) {
                to_unicode_mappings.push((new_gid, hex));
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

        // **P941** — glifos bitmap (CBDT) como imagens XObject.
        let used_ids = crate::export::bitmap_glyphs::used_glyph_ids_for_face(doc, face);
        let bitmap_glyphs =
            crate::export::bitmap_glyphs::collect_bitmap_glyphs_for_ids(&used_ids, face);
        let bitmap_only = !used_ids.is_empty()
            && used_ids.iter().all(|gid| bitmap_glyphs.contains_key(gid));

        let (img_refs, ptr_to_idx, mut img_xobjects) =
            scan_all_images(doc, first_img_id, icc_profile_id, &mut self.warnings);

        // P263 — gradient pre-pass.
        let first_grad_id = first_img_id + img_xobjects.len() * 2 + 100;
        let (pat_refs, pat_ptr_to_idx, grad_objs) =
            scan_all_gradients(doc, first_grad_id);
        let n_grads = grad_objs.len();
        let mut next_sub_id = first_grad_id + n_grads * 3;
        let tilings = crate::export::pdf_tiling::Patterns::scan(doc, &mut next_sub_id);

        // **P941** — XObjects de glifos bitmap (dedup por glyph id), IDs na zona
        // livre depois dos gradientes. Em seguida, `next_sub_id` avança para
        // não colidir com as sub-Functions dos gradientes.
        let mut next_bitmap_id = next_sub_id;
        let mut bitmap_name_counter = img_refs.len() + 1;
        let mut bitmap_refs: HashMap<u16, crate::export::bitmap_glyphs::BitmapGlyphRef> =
            HashMap::new();
        let mut bitmap_xobjects: Vec<ImageXObject> = Vec::new();
        let mut bitmap_res_entries: Vec<String> = Vec::new();
        for (gid, bmp) in &bitmap_glyphs {
            let smask_id = if bmp.payload.alpha_data_compressed.is_some() {
                let id = next_bitmap_id;
                next_bitmap_id += 1;
                Some(id)
            } else {
                None
            };
            let main_id = next_bitmap_id;
            next_bitmap_id += 1;
            let name = format!("Im{}", bitmap_name_counter);
            bitmap_name_counter += 1;
            bitmap_refs.insert(
                *gid,
                crate::export::bitmap_glyphs::BitmapGlyphRef {
                    type3_code: None,
                    name: name.clone(),
                    width: bmp.payload.width,
                    height: bmp.payload.height,
                    bearing_x: bmp.bearing_x,
                    bearing_y: bmp.bearing_y,
                    pixels_per_em: bmp.pixels_per_em,
                },
            );
            bitmap_res_entries.push(format!("/{name} {main_id} 0 R"));
            bitmap_xobjects.push(ImageXObject::Png {
                payload: bmp.payload.clone(),
                main_obj_id: main_id,
                smask_obj_id: smask_id,
            });
        }
        if bitmap_only && bitmap_refs.len() <= 256 {
            certify_bitmap_codes(&mut bitmap_refs);
        }
        img_xobjects.extend(bitmap_xobjects);
        next_sub_id = next_bitmap_id;

        self.add(1, "<< /Type /Catalog /Pages 2 0 R >>".into());

        let kids = (first_page..first_page + n)
            .map(|i| format!("{i} 0 R"))
            .collect::<Vec<_>>()
            .join(" ");
        self.add(2, format!("<< /Type /Pages /Kids [{kids}] /Count {n} >>"));

        for (i, page) in doc.pages.iter().enumerate() {
            // **P1468** — página excluída pela seleção: não emitir.
            let Some(new_i) = pic.new_index(i) else { continue };
            let page_id = first_page + new_i;
            let stream_id = first_stream + new_i;
            let w = super::pdf_canvas_extent(page.width);
            let h = super::pdf_canvas_extent(page.height);

            let xobj_res = merge_bitmap_xobjects(
                xobject_resources_for_page(page, &ptr_to_idx, &img_refs),
                &bitmap_res_entries,
            );
            let pat_res = tilings.resources(
                &pattern_resources_for_page(page, &pat_ptr_to_idx, &pat_refs),
                i,
            );
            // **P956** — verbose: `/c0` (sRGB ICCBased) nos recursos da página.
            let cs_res = colorspace_resource_entry(verbose, icc_profile_id, page);
            let resources_str =
                format!("/Font << /F1 {font_id} 0 R >> {xobj_res} {pat_res}{cs_res}");

            self.add(
                page_id,
                format!(
                    "<< /Type /Page /Parent 2 0 R \
/MediaBox [0 0 {} {}] \
                   /Contents {stream_id} 0 R \
                   /Resources << {resources_str} >> >>",
                    format_dim(w),
                    format_dim(h)
                ),
            );

            let mut ctx = PageContext::with_font_scenario(
                &ptr_to_idx,
                &img_refs,
                &pat_ptr_to_idx,
                &pat_refs,
                FontScenario::Cidfont {
                    char_to_gid: &char_to_gid,
                    glyph_mapping: &glyph_mapping,
                    glyph_to_nominal: &glyph_to_nominal,
                    bitmap: if bitmap_only { Some(&bitmap_refs) } else { None },
                },
                self.stream_mode,
            )
            .with_pdf_tags(
                self.pdf_tags,
                page,
                matches!(self.standard.accessibility, Some(PdfAccessibility::Ua1)),
            )
            .with_oracle(self.oracle);
            ctx.tiling_names = Some(&tilings.names[i]);
            for (id, bytes) in tilings.emit(i, &mut ctx, &resources_str) {
                self.add_bytes(id, bytes);
            }
            let stream_bytes = self.maybe_oracle(build_page_stream(page, &ctx));
            // P884 — content stream comprimido com FlateDecode quando rentável.
            let stream_object = self.serialize_page_stream(&stream_bytes);
            self.add_bytes(stream_id, stream_object);
        }

        // P517 — nome com prefixo de subset quando aplicável.
        // **P950** — base é o nome REAL da fonte (tabela `name`), não o
        // genérico `CrystallineFont`; o prefixo determinístico mantém-se.
        let real_base = real_base_name(&embed_font_data, "CrystallineFont");
        let base_font_name = if glyph_mapping.is_empty() {
            real_base
        } else {
            subset_font_name(&real_base, &embed_font_data)
        };

        // P560 — descritor PDF e bytes conforme o tipo de fonte (TrueType vs CFF/OpenType).
        let (cid_subtype, font_file_key, stream_subtype, font_stream_data) =
            font_embedding_data(&embed_font_data);

        if bitmap_only && bitmap_refs.len() <= 256 {
            self.add_bitmap_type3(
                font_id,
                face,
                &bitmap_refs,
                doc,
                &bitmap_res_entries,
                &mut next_sub_id,
            );
        } else {
            // Type0 font (F1)
            self.add(
                font_id,
                format!(
                    "<< /Type /Font /Subtype /Type0 /BaseFont /{base_font_name} \
               /Encoding /Identity-H \
               /DescendantFonts [{cidfont_id} 0 R] \
               /ToUnicode {to_unicode_id} 0 R >>"
                ),
            );

            // CIDFont
            self.add(
                cidfont_id,
                format!(
                    "<< /Type /Font /Subtype {cid_subtype} /BaseFont /{base_font_name} \
               /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> \
               /FontDescriptor {font_descriptor_id} 0 R \
               /DW 500 \
               /W [{widths}] >>"
                ),
            );

            // FontDescriptor — P760/P1051: métricas reais da fonte embutida
            let full_face = ttf_parser::Face::parse(font_data, 0).ok();
            let fd = full_face
                .as_ref()
                .or(subset_face.as_ref())
                .map(font_descriptor_metrics)
                .unwrap_or_default();
            // **P941** — fontes 100% bitmap (CBDT) não são embutidas: o descritor
            // não referencia `/FontFile` e o stream da fonte é um objecto vazio.
            // Os glifos são desenhados como imagens XObject (ver `bitmap_glyphs.rs`).
            let font_file_entry = if bitmap_only {
                String::new()
            } else {
                format!("{font_file_key} {font_stream_id} 0 R")
            };
            self.add(
                font_descriptor_id,
                format!(
                    "<< /Type /FontDescriptor /FontName /{base_font_name} \
               /Flags 32 \
               /FontBBox [{:.5} {:.5} {:.5} {:.5}] \
               /ItalicAngle {:.5} /Ascent {:.5} /Descent {:.5} \
               /CapHeight {:.5} /StemV 80 \
               {font_file_entry} >>",
                    fd.font_bbox[0],
                    fd.font_bbox[1],
                    fd.font_bbox[2],
                    fd.font_bbox[3],
                    fd.italic_angle,
                    fd.ascent,
                    fd.descent,
                    fd.cap_height
                ),
            );

            // Font data stream — P516: usa subset se possível, senão fonte completa.
            // P560: stream subtype TrueType (CIDFontType2) ou CFF (CIDFontType0C).
            // P883: comprime com FlateDecode para aproximar o tamanho do vanilla.
            // P941: fontes 100% bitmap não embutem a fonte — objecto vazio para
            // manter a numeração do xref (não é referenciado pelo descritor).
            if bitmap_only {
                self.add_bytes(
                    font_stream_id,
                    b"<< /Length 0 >>\nstream\n\nendstream".to_vec(),
                );
            } else {
                self.add_bytes(
                    font_stream_id,
                    build_font_stream(stream_subtype, font_stream_data),
                );
            }

            // ToUnicode CMap stream
            let cmap = to_unicode_cmap(&to_unicode_mappings);
            let cmap_len = cmap.len();
            let mut cmap_obj = format!("<< /Length {cmap_len} >>\nstream\n").into_bytes();
            cmap_obj.extend_from_slice(&cmap);
            cmap_obj.extend_from_slice(b"\nendstream");
            self.add_bytes(to_unicode_id, cmap_obj);
        }

        // **P777** — emitir perfil ICC sRGB partilhado antes dos JPEGs RGB.
        // **P1477** — perfil selecionado pela versão efetiva do PDF.
        if let Some(id) = icc_profile_id {
            self.add_bytes(
                id,
                build_icc_profile_stream(srgb_icc_profile_for_version(
                    self.standard.version,
                )),
            );
        }

        self.emit_image_xobjects(img_xobjects);

        // P263 — Emit gradient objects.
        let page_dimensions: Vec<(f64, f64)> =
            doc.pages.iter().map(|p| (p.width, p.height)).collect();
        self.emit_gradient_objects(
            grad_objs,
            &page_dimensions,
            &mut next_sub_id,
            icc_profile_id,
        );

        // **P1477** — `/CIDSet` no `/FontDescriptor`, universal para toda
        // CIDFont com `/FontFile*` embutido (não gated a `standard`).
        // Alocado dinamicamente (mesmo padrão de P1475/P1476) DEPOIS de
        // todos os objectos de ID fixo desta fonte (stream/ToUnicode/ICC/
        // imagens/gradientes) já terem sido adicionados: `font_stream_id`
        // é o ID fixo seguinte a `font_descriptor_id`, logo alocar
        // dinamicamente logo após `self.add(font_descriptor_id, ...)`
        // colidiria com ele.
        if !bitmap_only {
            let cids: HashSet<u16> = if !glyph_mapping.is_empty() {
                glyph_mapping.values().copied().collect()
            } else {
                all_glyph_ids.iter().copied().collect()
            };
            let cidset_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;
            self.add_bytes(cidset_id, build_cidset_stream(&cids));
            if let Some((_, content)) =
                self.objects.iter_mut().find(|(id, _)| *id == font_descriptor_id)
            {
                insert_pdf_dict_entry(content, &format!(" /CIDSet {cidset_id} 0 R"));
            }
        }

        self.emit_link_annotations(doc);
        self.emit_named_destinations(doc);
        self.emit_attachments(doc);
        self.emit_outlines(doc);
        self.emit_info(doc);
        self.emit_pdfa_output_intent();
        self.emit_xmp_metadata(doc);
        self.emit_tag_structure(doc);
        self.emit_pdfua_viewer_preferences();
        self.emit_page_labels(doc);
        let subset_ms = self.subset_ms;
        let warnings = std::mem::take(&mut self.warnings);
        PdfExport { output: (self.serialize(), subset_ms), warnings }
    }

    // ── Caminho Multi-font (Passo 146, ADR-0055 decisão 5) ───────────────────

    pub(in crate::export) fn build_multifont(
        mut self,
        doc: &PagedDocument,
        fonts: &[FontKey],
        faces: &[Face<'_>],
    ) -> PdfExport<(Vec<u8>, f64)> {
        // **P1468** — ver nota equivalente em `build_helvetica`.
        let pic = PageIndexConverter::new(doc.pages.len(), self.pages.as_ref());
        let n_pages = pic.exported_count();
        let n_fonts = fonts.len();
        let first_page = 3usize;
        let first_stream = first_page + n_pages;
        // Cada font ocupa 5 IDs consecutivos: type0, cidfont, descriptor,
        // font_stream, to_unicode. Type0 é o "/Fn" referenciado no resource.
        let fonts_start = first_stream + n_pages;

        // **P777** — reservar ID do perfil ICC sRGB partilhado se houver JPEGs RGB.
        // **P956** — em modo verbose o ICC é embutido SEMPRE, alocado DEPOIS
        // dos objectos de fonte (mesma posição da convenção P777).
        let verbose = self.stream_mode == StreamMode::Verbose;
        let needs_icc = has_rgb_jpeg(doc) || verbose;
        let icc_profile_id =
            if needs_icc { Some(fonts_start + 5 * n_fonts) } else { None };
        let first_img_id = fonts_start + 5 * n_fonts + if needs_icc { 1 } else { 0 };

        // Codepoints + glyph mappings por font. Cada font tem o seu
        // mapping (chars partilhados; gids específicos da face).
        let mut per_font_mappings: Vec<Vec<(u16, String)>> = Vec::with_capacity(n_fonts);
        let mut per_font_char_to_gid: Vec<HashMap<char, u16>> =
            Vec::with_capacity(n_fonts);
        let mut per_font_widths: Vec<String> = Vec::with_capacity(n_fonts);
        let mut per_font_embed_data: Vec<Vec<u8>> = Vec::with_capacity(n_fonts);
        let mut per_font_glyph_mapping: Vec<HashMap<u16, u16>> =
            Vec::with_capacity(n_fonts);
        let mut per_font_glyph_to_nominal: Vec<HashMap<u16, i32>> =
            Vec::with_capacity(n_fonts);
        // **P906** — mapa reverso glyph_id→char por fonte, partilhado com
        // `emit_glyph_pdf` (`FontScenario::Multifont::per_font_glyph_reverse`)
        // para seleccionar `/Fn` correcto de `FrameItem::Glyph` — mesmo
        // `build_math_glyph_reverse_map` já computado abaixo para o
        // subsetting DEBT-9/P45, só também guardado aqui. Ver
        // `export/stream.md` §P906.
        let mut per_font_glyph_reverse: Vec<HashMap<u16, char>> =
            Vec::with_capacity(n_fonts);
        let bound_views = crate::export::font_binding::partition(doc, fonts);
        for face in faces {
            let view = &bound_views[per_font_mappings.len()];
            let mut chars = collect_codepoints(view);
            let text_chars = collect_text_codepoints(view);
            chars.extend(text_chars.iter().copied());
            let glyph_ids = collect_glyph_ids(view);
            let shaped_mappings = collect_shaped_glyph_mappings(view);
            let shaped_cluster_texts = collect_shaped_cluster_texts(view);
            let mut mappings = map_chars_to_glyphs(face, &chars);

            // P568 — incluir glyphs de FrameItem::Text para que espaços e outros
            // caracteres do caminho fallback sejam subsetados.
            let mut extended_glyph_ids = glyph_ids.clone();
            for &c in &text_chars {
                if let Some(gid) = face.glyph_index(c) {
                    extended_glyph_ids.insert(gid.0);
                }
            }

            let font_index = per_font_mappings.len();
            let (font_list, font_variant, font_variations) = &fonts[font_index].0;
            let font_bytes = &fonts[font_index].1;
            // P836 — eixos fundidos (derivados de `FontVariant` + explícitos
            // de `#text(variations:)`): a instanciação embute a fonte nas
            // coordenadas pedidas, não só nos avanços do shaper.
            let axis_vars = merge_explicit_variations(
                axis_variations_for_font_variant(font_variant),
                font_variations,
            );

            // Adicionar glifos variantes de tamanho matemático
            // (Passo 45, DEBT-9) — mesmo tratamento que `build_cidfont`.
            let glyph_reverse = build_math_glyph_reverse_map(face);
            let existing_gids: BTreeSet<u16> =
                mappings.iter().map(|(_, gid)| *gid).collect();
            for &gid in &glyph_ids {
                if !existing_gids.contains(&gid) {
                    if let Some(&c) = glyph_reverse.get(&gid) {
                        mappings.push((c, gid));
                    }
                }
            }
            per_font_glyph_reverse.push(glyph_reverse);

            // P516 — subsetting por fonte.
            // P520: shaped glyphs (ligatures) têm prioridade no char_to_old_gid.
            // P874 — shaped_mappings contém glifos de TODAS as fontes; só
            // incluir os que pertencem à face actual.
            let n_glyphs = face.number_of_glyphs();
            let mut char_to_old_gid: std::collections::BTreeMap<char, u16> =
                mappings.iter().copied().collect();
            for (&old_gid, &ch) in &shaped_mappings {
                if old_gid < n_glyphs {
                    char_to_old_gid.insert(ch, old_gid);
                }
            }

            // P874 — `glyph_ids` contém glyph IDs de todas as fontes do
            // documento. Filtrar para o range válido desta face antes de
            // subsetar, senão o subsetter rejeita a fonte como MalformedFont.
            let face_glyph_ids: BTreeSet<u16> =
                collect_glyph_ids(&bound_views[font_index])
                    .into_iter()
                    .filter(|&gid| gid < n_glyphs)
                    .collect();

            // The subsetter freezes variable data. Pass axes while the original
            // face still carries them, rather than instancing its default subset.
            let axis_tuples: Vec<_> =
                axis_vars.iter().map(|v| (v.tag, v.value)).collect();
            let t0 = std::time::Instant::now();
            let subset = crate::export::subset::subset_font_with_variations(
                font_bytes,
                &char_to_old_gid,
                &face_glyph_ids,
                &axis_tuples,
            );
            self.subset_ms += duration_ms(t0.elapsed());
            let (embed_data, glyph_mapping, instancing_applied) = match subset {
                Some(FontSubset { data, mapping }) if Face::parse(&data, 0).is_ok() => {
                    (data, mapping, !axis_vars.is_empty())
                }
                _ => {
                    // Full embedding fallback must also use original variable bytes.
                    let instance = if axis_vars.is_empty() {
                        None
                    } else {
                        instantiate_variable_font(font_bytes, &axis_tuples)
                    };
                    let applied = instance.is_some();
                    if !axis_vars.is_empty() && !applied {
                        self.warn(format!(
                            "Aviso: falha ao instanciar fonte variável para {:?}/{:?}; \
                             a usar instância default.",
                            font_list, font_variant,
                        ));
                    }
                    (
                        instance.unwrap_or_else(|| font_bytes.clone()),
                        HashMap::new(),
                        applied,
                    )
                }
            };

            // Original GIDs and nominal advances must agree with the effective
            // instance used for /W, before applying the subset remapping.
            let mut glyph_to_nominal: HashMap<u16, i32> = HashMap::new();
            {
                let mut nominal_face = face.clone();
                if instancing_applied {
                    for v in &axis_vars {
                        nominal_face.set_variation(v.tag, v.value);
                    }
                }
                for &gid in
                    extended_glyph_ids.iter().chain(mappings.iter().map(|(_, gid)| gid))
                {
                    let adv = nominal_face
                        .glyph_hor_advance(ttf_parser::GlyphId(gid))
                        .unwrap_or(0) as i32;
                    glyph_to_nominal.insert(gid, adv);
                }
            }

            if !glyph_mapping.is_empty() {
                for (_, old_gid) in mappings.iter_mut() {
                    *old_gid = remap_glyph_id(*old_gid, &glyph_mapping);
                }
            }

            // P521 — ToUnicode/widths a partir dos glifos reais do shaper,
            // reconstruindo o texto completo de cada cluster.
            let mut to_unicode_mappings: Vec<(u16, String)> = Vec::new();
            let mut seen_to_unicode_gids: HashSet<u16> = HashSet::new();
            seed_space_unicode(
                &mappings,
                &shaped_cluster_texts,
                &glyph_mapping,
                &mut to_unicode_mappings,
                &mut seen_to_unicode_gids,
            );
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

        // **P941** — glifos bitmap (CBDT) como imagens XObject, por fonte.
        let per_font_used = per_font_used_glyphs(doc, fonts, faces);
        let per_font_bitmap_glyphs: Vec<
            HashMap<u16, crate::export::bitmap_glyphs::BitmapGlyph>,
        > = faces
            .iter()
            .enumerate()
            .map(|(fi, face)| {
                crate::export::bitmap_glyphs::collect_bitmap_glyphs_for_ids(
                    &per_font_used[fi],
                    face,
                )
            })
            .collect();
        let per_font_bitmap_only: Vec<bool> = per_font_used
            .iter()
            .zip(&per_font_bitmap_glyphs)
            .map(|(used, bmp)| {
                !used.is_empty() && used.iter().all(|gid| bmp.contains_key(gid))
            })
            .collect();

        let (img_refs, ptr_to_idx, mut img_xobjects) =
            scan_all_images(doc, first_img_id, icc_profile_id, &mut self.warnings);

        // P263 — gradient pre-pass.
        let first_grad_id = first_img_id + img_xobjects.len() * 2 + 100;
        let (pat_refs, pat_ptr_to_idx, grad_objs) =
            scan_all_gradients(doc, first_grad_id);
        let n_grads = grad_objs.len();
        let mut next_sub_id = first_grad_id + n_grads * 3;
        let tilings = crate::export::pdf_tiling::Patterns::scan(doc, &mut next_sub_id);

        // **P941** — XObjects de glifos bitmap (dedup por glyph id, por fonte),
        // IDs na zona livre depois dos gradientes.
        let mut next_bitmap_id = next_sub_id;
        let mut bitmap_name_counter = img_refs.len() + 1;
        let mut per_font_bitmap: Vec<
            Option<HashMap<u16, crate::export::bitmap_glyphs::BitmapGlyphRef>>,
        > = Vec::with_capacity(n_fonts);
        let mut bitmap_xobjects: Vec<ImageXObject> = Vec::new();
        let mut bitmap_res_entries: Vec<String> = Vec::new();
        for (fi, bmp_map) in per_font_bitmap_glyphs.iter().enumerate() {
            if !per_font_bitmap_only[fi] {
                per_font_bitmap.push(None);
                continue;
            }
            let mut refs: HashMap<u16, crate::export::bitmap_glyphs::BitmapGlyphRef> =
                HashMap::new();
            for (gid, bmp) in bmp_map {
                let smask_id = if bmp.payload.alpha_data_compressed.is_some() {
                    let id = next_bitmap_id;
                    next_bitmap_id += 1;
                    Some(id)
                } else {
                    None
                };
                let main_id = next_bitmap_id;
                next_bitmap_id += 1;
                let name = format!("Im{}", bitmap_name_counter);
                bitmap_name_counter += 1;
                refs.insert(
                    *gid,
                    crate::export::bitmap_glyphs::BitmapGlyphRef {
                        type3_code: None,
                        name: name.clone(),
                        width: bmp.payload.width,
                        height: bmp.payload.height,
                        bearing_x: bmp.bearing_x,
                        bearing_y: bmp.bearing_y,
                        pixels_per_em: bmp.pixels_per_em,
                    },
                );
                bitmap_res_entries.push(format!("/{name} {main_id} 0 R"));
                bitmap_xobjects.push(ImageXObject::Png {
                    payload: bmp.payload.clone(),
                    main_obj_id: main_id,
                    smask_obj_id: smask_id,
                });
            }
            if refs.len() <= 256 {
                certify_bitmap_codes(&mut refs);
            }
            per_font_bitmap.push(Some(refs));
        }
        img_xobjects.extend(bitmap_xobjects);
        next_sub_id = next_bitmap_id;

        self.add(1, "<< /Type /Catalog /Pages 2 0 R >>".into());

        let kids = (first_page..first_page + n_pages)
            .map(|i| format!("{i} 0 R"))
            .collect::<Vec<_>>()
            .join(" ");
        self.add(2, format!("<< /Type /Pages /Kids [{kids}] /Count {n_pages} >>"));

        for (i, page) in doc.pages.iter().enumerate() {
            // **P1468** — página excluída pela seleção: não emitir.
            let Some(new_i) = pic.new_index(i) else { continue };
            let page_id = first_page + new_i;
            let stream_id = first_stream + new_i;
            let w = super::pdf_canvas_extent(page.width);
            let h = super::pdf_canvas_extent(page.height);

            let xobj_res = merge_bitmap_xobjects(
                xobject_resources_for_page(page, &ptr_to_idx, &img_refs),
                &bitmap_res_entries,
            );
            let pat_res = tilings.resources(
                &pattern_resources_for_page(page, &pat_ptr_to_idx, &pat_refs),
                i,
            );
            let font_entries = (0..n_fonts)
                .map(|fi| {
                    let type0_id = fonts_start + 5 * fi;
                    format!("/F{} {} 0 R", fi + 1, type0_id)
                })
                .collect::<Vec<_>>()
                .join(" ");
            // **P956** — verbose: `/c0` (sRGB ICCBased) nos recursos da página.
            let cs_res = colorspace_resource_entry(verbose, icc_profile_id, page);
            let resources_str =
                format!("/Font << {font_entries} >> {xobj_res} {pat_res}{cs_res}");

            self.add(
                page_id,
                format!(
                    "<< /Type /Page /Parent 2 0 R \
/MediaBox [0 0 {} {}] \
                   /Contents {stream_id} 0 R \
                   /Resources << {resources_str} >> >>",
                    format_dim(w),
                    format_dim(h)
                ),
            );

            let mut ctx = PageContext::with_font_scenario(
                &ptr_to_idx,
                &img_refs,
                &pat_ptr_to_idx,
                &pat_refs,
                FontScenario::Multifont {
                    fonts,
                    per_font_char_to_gid: &per_font_char_to_gid,
                    per_font_glyph_mapping: &per_font_glyph_mapping,
                    per_font_glyph_to_nominal: &per_font_glyph_to_nominal,
                    per_font_glyph_reverse: &per_font_glyph_reverse,
                    per_font_bitmap: &per_font_bitmap,
                    bindings: Some(crate::export::font_binding::LookupCache::default()),
                },
                self.stream_mode,
            )
            .with_pdf_tags(
                self.pdf_tags,
                page,
                matches!(self.standard.accessibility, Some(PdfAccessibility::Ua1)),
            )
            .with_oracle(self.oracle);
            ctx.tiling_names = Some(&tilings.names[i]);
            for (id, bytes) in tilings.emit(i, &mut ctx, &resources_str) {
                self.add_bytes(id, bytes);
            }
            let stream_bytes = self.maybe_oracle(build_page_stream(page, &ctx));
            // P884 — content stream comprimido com FlateDecode quando rentável.
            let stream_object = self.serialize_page_stream(&stream_bytes);
            self.add_bytes(stream_id, stream_object);
        }

        // Emit objectos por font (5 cada).
        for (fi, _) in fonts.iter().enumerate() {
            if let Some(refs) = &per_font_bitmap[fi] {
                if refs.values().all(|reference| reference.type3_code.is_some()) {
                    self.add_bitmap_type3(
                        fonts_start + 5 * fi,
                        &faces[fi],
                        refs,
                        &bound_views[fi],
                        &bitmap_res_entries,
                        &mut next_sub_id,
                    );
                    continue;
                }
            }
            let font_data = &per_font_embed_data[fi];
            let type0_id = fonts_start + 5 * fi;
            let cidfont_id = type0_id + 1;
            let descriptor_id = type0_id + 2;
            let stream_id = type0_id + 3;
            let to_unicode_id = type0_id + 4;
            let widths = &per_font_widths[fi];
            let mappings = &per_font_mappings[fi];
            let glyph_mapping = &per_font_glyph_mapping[fi];
            // **P950** — base é o nome REAL da fonte (tabela `name`), não o
            // genérico `CrystallineFont{N}`; prefixo determinístico mantém-se.
            let fallback_name = format!("CrystallineFont{}", fi + 1);
            let real_base = real_base_name(font_data, &fallback_name);
            let name = if glyph_mapping.is_empty() {
                real_base
            } else {
                subset_font_name(&real_base, font_data)
            };

            // P560 — descritor PDF e bytes conforme o tipo de fonte (TrueType vs CFF/OpenType).
            let (cid_subtype, font_file_key, stream_subtype, font_stream_data) =
                font_embedding_data(font_data);

            // Type0
            self.add(
                type0_id,
                format!(
                    "<< /Type /Font /Subtype /Type0 /BaseFont /{name} \
                   /Encoding /Identity-H \
                   /DescendantFonts [{cidfont_id} 0 R] \
                   /ToUnicode {to_unicode_id} 0 R >>"
                ),
            );

            // CIDFont
            self.add(cidfont_id, format!(
                "<< /Type /Font /Subtype {cid_subtype} /BaseFont /{name} \
                   /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> \
                   /FontDescriptor {descriptor_id} 0 R \
                   /DW 500 \
                   /W [{widths}] >>"
            ));

            // P1051 — métricas reais da face para /FontDescriptor no caminho multi-font
            let face_parsed = Face::parse(font_data, 0).ok();
            let face_to_use = faces.get(fi).or(face_parsed.as_ref());
            let fd = face_to_use.map(font_descriptor_metrics).unwrap_or_default();

            // **P941** — fontes 100% bitmap (CBDT) não são embutidas: o
            // descritor não referencia `/FontFile` e o stream é um objecto
            // vazio. Os glifos são desenhados como imagens XObject.
            let font_file_entry = if per_font_bitmap_only[fi] {
                String::new()
            } else {
                format!("{font_file_key} {stream_id} 0 R")
            };
            self.add(
                descriptor_id,
                format!(
                    "<< /Type /FontDescriptor /FontName /{name} \
                   /Flags 32 \
                   /FontBBox [{:.5} {:.5} {:.5} {:.5}] \
                   /ItalicAngle {:.5} /Ascent {:.5} /Descent {:.5} \
                   /CapHeight {:.5} /StemV 80 \
                   {font_file_entry} >>",
                    fd.font_bbox[0],
                    fd.font_bbox[1],
                    fd.font_bbox[2],
                    fd.font_bbox[3],
                    fd.italic_angle,
                    fd.ascent,
                    fd.descent,
                    fd.cap_height
                ),
            );

            // Font data stream — P516: usa subset se possível, senão fonte completa.
            // P560: stream subtype TrueType (CIDFontType2) ou CFF (CIDFontType0C).
            // P883: comprime com FlateDecode para aproximar o tamanho do vanilla.
            // P941: fontes 100% bitmap não embutem a fonte — objecto vazio para
            // manter a numeração do xref (não é referenciado pelo descritor).
            if per_font_bitmap_only[fi] {
                self.add_bytes(
                    stream_id,
                    b"<< /Length 0 >>\nstream\n\nendstream".to_vec(),
                );
            } else {
                self.add_bytes(
                    stream_id,
                    build_font_stream(stream_subtype, font_stream_data),
                );
            }

            // ToUnicode CMap
            let cmap = to_unicode_cmap(mappings);
            let cmap_len = cmap.len();
            let mut cmap_obj = format!("<< /Length {cmap_len} >>\nstream\n").into_bytes();
            cmap_obj.extend_from_slice(&cmap);
            cmap_obj.extend_from_slice(b"\nendstream");
            self.add_bytes(to_unicode_id, cmap_obj);
        }

        // **P777** — emitir perfil ICC sRGB partilhado antes dos JPEGs RGB.
        // **P1477** — perfil selecionado pela versão efetiva do PDF.
        if let Some(id) = icc_profile_id {
            self.add_bytes(
                id,
                build_icc_profile_stream(srgb_icc_profile_for_version(
                    self.standard.version,
                )),
            );
        }

        self.emit_image_xobjects(img_xobjects);

        // P263 — Emit gradient objects.
        let page_dimensions: Vec<(f64, f64)> =
            doc.pages.iter().map(|p| (p.width, p.height)).collect();
        self.emit_gradient_objects(
            grad_objs,
            &page_dimensions,
            &mut next_sub_id,
            icc_profile_id,
        );

        // **P1477** — `/CIDSet` no `/FontDescriptor` de cada fonte embutida
        // (mesmo raciocínio de `build_cidfont`, ver comentário lá). Segunda
        // passagem por `fonts` DEPOIS de todos os objectos de ID fixo (fontes/
        // ICC/imagens/gradientes) já terem sido adicionados, para a alocação
        // dinâmica (`self.objects.max()+1`) não colidir com `stream_id`
        // (o ID fixo seguinte a `descriptor_id` dentro do loop acima).
        for (fi, _) in fonts.iter().enumerate() {
            if per_font_bitmap_only[fi] {
                continue;
            }
            let descriptor_id = fonts_start + 5 * fi + 2;
            let glyph_mapping = &per_font_glyph_mapping[fi];
            let cids: HashSet<u16> = if !glyph_mapping.is_empty() {
                glyph_mapping.values().copied().collect()
            } else {
                let n_glyphs = faces[fi].number_of_glyphs();
                collect_glyph_ids(&bound_views[fi])
                    .into_iter()
                    .filter(|&gid| gid < n_glyphs)
                    .collect()
            };
            let cidset_id = self.objects.iter().map(|(id, _)| *id).max().unwrap_or(0) + 1;
            self.add_bytes(cidset_id, build_cidset_stream(&cids));
            if let Some((_, content)) =
                self.objects.iter_mut().find(|(id, _)| *id == descriptor_id)
            {
                insert_pdf_dict_entry(content, &format!(" /CIDSet {cidset_id} 0 R"));
            }
        }

        self.emit_link_annotations(doc);
        self.emit_named_destinations(doc);
        self.emit_attachments(doc);
        self.emit_outlines(doc);
        self.emit_info(doc);
        self.emit_pdfa_output_intent();
        self.emit_xmp_metadata(doc);
        self.emit_tag_structure(doc);
        self.emit_pdfua_viewer_preferences();
        self.emit_page_labels(doc);
        let subset_ms = self.subset_ms;
        let warnings = std::mem::take(&mut self.warnings);
        PdfExport { output: (self.serialize(), subset_ms), warnings }
    }
}

fn certify_bitmap_codes(
    refs: &mut HashMap<u16, crate::export::bitmap_glyphs::BitmapGlyphRef>,
) {
    let mut ids = refs.keys().copied().collect::<Vec<_>>();
    ids.sort_unstable();
    for (code, gid) in ids.into_iter().enumerate() {
        refs.get_mut(&gid).unwrap().type3_code =
            Some(u8::try_from(code).expect("at most256 certified glyphs"));
    }
}

impl PdfBuilder {
    fn add_bitmap_type3(
        &mut self,
        font_id: usize,
        face: &Face<'_>,
        refs: &HashMap<u16, crate::export::bitmap_glyphs::BitmapGlyphRef>,
        doc: &PagedDocument,
        resources: &[String],
        next: &mut usize,
    ) {
        let mut ids = refs.keys().copied().collect::<Vec<_>>();
        ids.sort_unstable();
        let mut bounds =
            [f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];
        let mut procs = Vec::new();
        let mut widths = Vec::new();
        let mut names = Vec::new();
        let mut unicode = Vec::new();
        let clusters: HashMap<u16, String> =
            collect_shaped_cluster_texts(doc).into_iter().collect();
        let reverse = build_math_glyph_reverse_map(face);
        let upem = f64::from(face.units_per_em().max(1));
        for gid in ids {
            let bmp = &refs[&gid];
            let code = bmp.type3_code.expect("certified source code");
            let scale = upem / f64::from(bmp.pixels_per_em.max(1));
            let x = f64::from(bmp.bearing_x) * scale;
            let y = f64::from(bmp.bearing_y) * scale;
            let w = f64::from(bmp.width) * scale;
            let h = f64::from(bmp.height) * scale;
            bounds[0] = bounds[0].min(x);
            bounds[1] = bounds[1].min(y);
            bounds[2] = bounds[2].max(x + w);
            bounds[3] = bounds[3].max(y + h);
            let width = face.glyph_hor_advance(ttf_parser::GlyphId(gid)).unwrap_or(0);
            let id = *next;
            *next += 1;
            let stream = format!(
                "{width} 0 d0\nq\n{w:.9} 0 0 {h:.9} {x:.9} {y:.9} cm\n/{} Do\nQ\n",
                bmp.name
            );
            self.add_bytes(
                id,
                format!("<< /Length {} >>\nstream\n{stream}endstream", stream.len())
                    .into_bytes(),
            );
            procs.push(format!("/g{code} {id} 0 R"));
            names.push(format!("/g{code}"));
            widths.push(width.to_string());
            if let Some(hex) = clusters
                .get(&gid)
                .cloned()
                .or_else(|| reverse.get(&gid).map(|&c| char_to_utf16_hex(c)))
            {
                unicode.push((code, hex));
            }
        }
        let cmap_id = *next;
        *next += 1;
        let descriptor_id = *next;
        *next += 1;
        let mut cmap=String::from("/CIDInit /ProcSet findresource begin\n12 dict begin\nbegincmap\n/CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def\n/CMapName /CrystallineBitmapUnicode def\n/CMapType 2 def\n1 begincodespacerange\n<00> <FF>\nendcodespacerange\n");
        for chunk in unicode.chunks(100) {
            cmap.push_str(&format!("{} beginbfchar\n", chunk.len()));
            for (code, hex) in chunk {
                cmap.push_str(&format!("<{code:02X}> <{hex}>\n"));
            }
            cmap.push_str("endbfchar\n");
        }
        cmap.push_str(
            "endcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n",
        );
        self.add_bytes(
            cmap_id,
            format!("<< /Length {} >>\nstream\n{cmap}endstream", cmap.len()).into_bytes(),
        );
        let bbox = format!(
            "{:.9} {:.9} {:.9} {:.9}",
            bounds[0], bounds[1], bounds[2], bounds[3]
        );
        let family = face
            .names()
            .into_iter()
            .find(|n| n.name_id == 6)
            .and_then(|n| n.to_string())
            .unwrap_or_else(|| "CrystallineBitmap".to_string())
            .replace(' ', "");
        self.add(descriptor_id,format!("<< /Type /FontDescriptor /FontName /{family} /Flags 131077 /ItalicAngle 0 /Ascent {:.9} /Descent {:.9} /FontBBox [{bbox}] >>",bounds[3],bounds[1]));
        let matrix = 1.0 / upem;
        self.add(font_id,format!("<< /Type /Font /Subtype /Type3 /Name /{family} /FontDescriptor {descriptor_id} 0 R /FontBBox [{bbox}] /FontMatrix [{matrix:.15} 0 0 {matrix:.15} 0 0] /CharProcs << {} >> /Encoding << /Type /Encoding /Differences [0 {}] >> /FirstChar 0 /LastChar {} /Widths [{}] /Resources << /XObject << {} >> >> /ToUnicode {cmap_id} 0 R >>",procs.join(" "),names.join(" "),refs.len()-1,widths.join(" "),resources.join(" ")));
    }
}

#[cfg(test)]
mod variable_instance_tests {
    use super::*;
    use crate::{fonts::FontSlot, world::SystemWorld};
    use typst_core::entities::layout_types::{Page, Point, Pt, TextStyle};

    #[test]
    fn explicit_variable_axes_survive_pdf_subsetting() {
        let path = std::path::PathBuf::from(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/fonts/UbuntuSans-Variable.ttf"
        ));
        let world = SystemWorld::new(env!("CARGO_MANIFEST_DIR"), "Cargo.toml")
            .unwrap()
            .with_fonts(vec![FontSlot::new(path, 0)]);
        let mut items = Vec::new();
        for (i, (weight, axis)) in [(700, 250.0), (400, 900.0)].into_iter().enumerate() {
            let style = TextStyle {
                font: Some(FontList::single("Ubuntu Sans".into())),
                size: Pt(20.0),
                weight: Some(weight),
                variations: Some(FontVariations(vec![(*b"wght", axis)])),
                ..Default::default()
            };
            items.push(FrameItem::Text {
                pos: Point { x: Pt(20.0), y: Pt(30.0 + i as f64 * 30.0) },
                text: "Hello".into(),
                style,
                bidi: None,
            });
        }
        let page = Page {
            width: 200.0,
            height: 200.0,
            logical_number: 1,
            numbering: None,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items,
        };
        let doc = crate::shaper::shape_document(&world, PagedDocument::new(vec![page]));
        for mode in [StreamMode::Compact, StreamMode::Verbose] {
            let pdf = crate::export::export_pdf(&doc, mode);
            let text = String::from_utf8_lossy(&pdf);
            for widths in [
                "/W [1 [689] 2 [547] 3 [247] 4 [580]]",
                "/W [1 [734] 2 [584] 3 [316] 4 [607]]",
            ] {
                assert!(
                    text.contains(widths),
                    "PDF widths must describe the requested instance: {widths}"
                );
            }
        }
    }
}

#[cfg(test)]
mod r10_bitmap_text_tests {
    use super::*;
    use std::io::Write;
    use std::process::{Command, Stdio};
    use typst_core::entities::{
        content::Content,
        font_book::FontVariant,
        font_list::FontList,
        font_variations::FontVariations,
        layout_types::{Page, Point, Pt, TextStyle},
        shaped_glyph::ShapedGlyph,
        world_types::Font,
    };
    #[test]
    fn r10_bitmap_source_character_survives_both_pdf_modes_and_font_paths() {
        let bytes = include_bytes!("../../../fixtures/fonts/raster-glyph.ttf");
        let face = Face::parse(bytes, 0).unwrap();
        let gid = face.glyph_index('☃').unwrap().0;
        let family = FontList::single("Crystalline Raster Fixture".into());
        let style = TextStyle {
            size: Pt(12.0),
            font: Some(family.clone()),
            ..TextStyle::default()
        };
        let doc = PagedDocument::new(vec![Page {
            width: 100.0,
            height: 100.0,
            logical_number: 1,
            numbering: None,
            supplement: Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items: vec![FrameItem::TextShaped {
                pos: Point { x: Pt(10.0), y: Pt(20.0) },
                text: "☃".into(),
                style,
                glyphs: vec![ShapedGlyph {
                    glyph_id: gid,
                    x_advance: 750,
                    x_offset: 0,
                    y_offset: 0,
                    cluster: 0,
                    char_code: '☃',
                }],
                units_per_em: 1000,
                bidi: None,
                font: Some(Font::from_data(bytes.to_vec())),
            }],
        }]);
        let fonts = vec![(
            (family, FontVariant::default(), FontVariations::default()),
            bytes.to_vec(),
        )];
        let mut replaced = doc.clone();
        replaced.pages[0].items = vec![FrameItem::TextReplacement {
            text: "☃".into(),
            items: doc.pages[0].items.clone(),
        }];
        let mut reference = None;
        for document in [&doc, &replaced] {
            for mode in [StreamMode::Compact, StreamMode::Verbose] {
                let outputs = [
                    crate::export::export_pdf_with_font(document, bytes, mode),
                    crate::export::export_pdf_multifont(document, &fonts, mode),
                ];
                for pdf in outputs {
                    let mut child = Command::new("pdftotext")
                        .args(["-bbox", "-", "-"])
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("PDF validation requires pdftotext");
                    child.stdin.take().unwrap().write_all(&pdf).unwrap();
                    let out = child.wait_with_output().unwrap();
                    assert!(
                        out.status.success(),
                        "{}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    let observed = String::from_utf8_lossy(&out.stdout)
                        .lines()
                        .find(|line| line.contains(">☃</word>"))
                        .map(str::to_string);
                    if let Some(expected) = &reference {
                        assert_eq!(&observed,expected,"source-identical replacement preserves actual bitmap glyph coordinates");
                    } else {
                        reference = Some(observed.clone());
                    }
                    assert!(
                        String::from_utf8_lossy(&out.stdout).contains(">☃</word>"),
                        "bitmap paint must retain its source character: {}",
                        String::from_utf8_lossy(&out.stdout)
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod readiness_unicode_tests {
    use std::io::Write;
    use std::process::{Command, Stdio};
    use typst_core::entities::{
        content::Content,
        font_book::FontVariant,
        font_list::FontList,
        font_variations::FontVariations,
        layout_types::{
            FrameItem, Page, PagedDocument, Point, Pt, ShapedGlyph, TextSeparatorOrigin,
            TextStyle,
        },
        world_types::Font,
    };

    fn extracted(pdf: &[u8]) -> String {
        let mut child = Command::new("pdftotext")
            .args(["-raw", "-", "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("pdftotext is a required validation observer");
        child.stdin.take().unwrap().write_all(pdf).unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        String::from_utf8(output.stdout).unwrap()
    }

    fn bbox_words(pdf: &[u8]) -> Vec<(String, [f64; 4])> {
        let mut child = Command::new("pdftotext")
            .args(["-bbox-layout", "-", "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("pdftotext is a required word/bounds observer");
        child.stdin.take().unwrap().write_all(pdf).unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let xml = String::from_utf8(output.stdout).unwrap();
        xml.lines()
            .filter(|line| line.trim_start().starts_with("<word "))
            .map(|line| {
                let (attributes, body) = line.split_once('>').unwrap();
                let text = body.strip_suffix("</word>").unwrap().to_string();
                let bounds = ["xMin", "yMin", "xMax", "yMax"].map(|name| {
                    let prefix = format!("{name}=\"");
                    attributes
                        .split_once(prefix.as_str())
                        .unwrap()
                        .1
                        .split('"')
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap()
                });
                (text, bounds)
            })
            .collect()
    }

    #[test]
    fn readiness_unicode_joiner_prefix_retains_original_word_bounds() {
        let bytes = include_bytes!("../../../fixtures/fonts/DejaVuSans.ttf");
        let face = ttf_parser::Face::parse(bytes, 0).unwrap();
        let family = FontList::single("DejaVu Sans".into());
        let style = TextStyle {
            size: Pt(12.0),
            font: Some(family.clone()),
            ..Default::default()
        };
        let font = Font::from_data(bytes.to_vec());
        let upem = face.units_per_em();
        let advance = |ch| {
            f64::from(face.glyph_hor_advance(face.glyph_index(ch).unwrap()).unwrap())
                * 12.0
                / f64::from(upem)
        };
        let shaped = |text: &str, x| FrameItem::TextShaped {
            pos: Point { x: Pt(x), y: Pt(20.0) },
            text: text.into(),
            style: style.clone(),
            glyphs: text
                .char_indices()
                .map(|(cluster, ch)| {
                    let gid = face
                        .glyph_index(if ch == '\u{2060}' { ' ' } else { ch })
                        .unwrap();
                    ShapedGlyph {
                        glyph_id: gid.0,
                        x_advance: if ch == '\u{2060}' {
                            0
                        } else {
                            i32::from(face.glyph_hor_advance(gid).unwrap())
                        },
                        x_offset: 0,
                        y_offset: 0,
                        cluster: u32::try_from(cluster).unwrap(),
                        char_code: ch,
                    }
                })
                .collect(),
            units_per_em: upem,
            bidi: None,
            font: Some(font.clone()),
        };
        let space = |x| FrameItem::TextSeparator {
            pos: Point { x: Pt(x), y: Pt(20.0) },
            advance: Pt(advance(' ')),
            style: style.clone(),
            origin: TextSeparatorOrigin::Literal,
            bidi: None,
        };
        let dot_end = 10.0 + advance('.');
        let digit_x = dot_end + advance(' ');
        let digit_end = digit_x + advance('1');
        let tail_x = digit_end + advance(' ');
        let fonts = vec![(
            (family, FontVariant::default(), FontVariations::default()),
            bytes.to_vec(),
        )];
        for split in [false, true] {
            let mut original = vec![shaped(".", 10.0), space(dot_end)];
            if split {
                original.extend([shaped("\u{2060}", digit_x), shaped("1", digit_x)]);
            } else {
                original.push(shaped("\u{2060}1", digit_x));
            }
            original.extend([space(digit_end), shaped("TAIL", tail_x)]);
            for local in [false, true] {
                for nested in [false, true] {
                    for replacement in [false, true] {
                        let mut items = original.clone();
                        if nested {
                            use typst_core::entities::layout_types::{
                                LinkTarget, SemanticKind, SemanticPlacement, Size,
                            };
                            items = vec![FrameItem::Semantic {
                                kind: SemanticKind::Label,
                                placement: SemanticPlacement::Inline,
                                alt: None,
                                items: vec![FrameItem::Link {
                                    target: LinkTarget::Url(
                                        "https://example.invalid/".into(),
                                    ),
                                    pos: Point { x: Pt(0.0), y: Pt(0.0) },
                                    size: Size { width: Pt(100.0), height: Pt(40.0) },
                                    items,
                                }],
                            }];
                        }
                        if local {
                            items = vec![FrameItem::Group {
                                paint_root: false,
                                pos: Point { x: Pt(11.0), y: Pt(7.0) },
                                matrix: Default::default(),
                                clip_mask: None,
                                inner_width: 100.0,
                                inner_height: 40.0,
                                items,
                            }];
                        }
                        if replacement {
                            items = vec![FrameItem::TextReplacement {
                                text: "ALT".into(),
                                items,
                            }];
                        }
                        let doc = PagedDocument::new(vec![Page {
                            width: 200.0,
                            height: 100.0,
                            logical_number: 1,
                            numbering: None,
                            supplement: Content::Empty,
                            bleed: Default::default(),
                            fill: Default::default(),
                            background: vec![],
                            foreground: vec![],
                            items,
                        }]);
                        for mode in [
                            crate::export::StreamMode::Compact,
                            crate::export::StreamMode::Verbose,
                        ] {
                            for pdf in [
                                crate::export::export_pdf_with_font(&doc, bytes, mode),
                                crate::export::export_pdf_multifont(&doc, &fonts, mode),
                            ] {
                                let words = bbox_words(&pdf);
                                let names: Vec<_> =
                                    words.iter().map(|(text, _)| text.as_str()).collect();
                                assert_eq!(names, if replacement { vec!["ALT"] } else { vec![". \u{2060}1", "TAIL"] },
                                "original word scopes: split={split}, local={local}, nested={nested}, replacement={replacement}, mode={mode:?}");
                                assert_eq!(
                                    extracted(&pdf).trim(),
                                    if replacement { "ALT" } else { ". \u{2060}1 TAIL" }
                                );
                                if !replacement {
                                    let offset = if local { 11.0 } else { 0.0 };
                                    assert!(
                                        (words[0].1[0] - 10.0 - offset).abs() <= 0.001
                                    );
                                    assert!(
                                        (words[0].1[2] - digit_end - offset).abs()
                                            <= 0.001
                                    );
                                    assert!(
                                        (words[1].1[0] - tail_x - offset).abs() <= 0.001
                                    );
                                    assert!(words[0].1[3] > words[0].1[1]);
                                    assert!(
                                        (words[0].1[1] - words[1].1[1]).abs() <= 0.001
                                    );
                                    assert!(
                                        (words[0].1[3] - words[1].1[3]).abs() <= 0.001
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn readiness_unicode_space_and_joiner_keep_distinct_source_text() {
        let bytes = include_bytes!("../../../fixtures/fonts/DejaVuSans.ttf");
        let face = ttf_parser::Face::parse(bytes, 0).unwrap();
        let family = FontList::single("DejaVu Sans".into());
        let style = TextStyle {
            size: Pt(12.0),
            font: Some(family.clone()),
            ..Default::default()
        };
        let font = Font::from_data(bytes.to_vec());
        let upem = face.units_per_em();
        let advance = |ch| {
            f64::from(face.glyph_hor_advance(face.glyph_index(ch).unwrap()).unwrap())
                * 12.0
                / f64::from(upem)
        };
        let shaped = |ch: char, x, zero| {
            let gid = face.glyph_index(if ch == '\u{2060}' { ' ' } else { ch }).unwrap();
            FrameItem::TextShaped {
                pos: Point { x: Pt(x), y: Pt(20.0) },
                text: ch.to_string().into(),
                style: style.clone(),
                glyphs: vec![ShapedGlyph {
                    glyph_id: gid.0,
                    x_advance: if zero {
                        0
                    } else {
                        i32::from(face.glyph_hor_advance(gid).unwrap())
                    },
                    x_offset: 0,
                    y_offset: 0,
                    cluster: 0,
                    char_code: ch,
                }],
                units_per_em: upem,
                bidi: None,
                font: Some(font.clone()),
            }
        };
        let space = |x| FrameItem::TextSeparator {
            pos: Point { x: Pt(x), y: Pt(20.0) },
            advance: Pt(advance(' ')),
            style: style.clone(),
            origin: TextSeparatorOrigin::Literal,
            bidi: None,
        };
        let a_end = 10.0 + advance('A');
        let bx = a_end + advance(' ');
        let b_end = bx + advance('B');
        let doc = PagedDocument::new(vec![Page {
            width: 120.0,
            height: 100.0,
            logical_number: 1,
            numbering: None,
            supplement: Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items: vec![
                shaped('A', 10.0, false),
                space(a_end),
                shaped('\u{2060}', bx, true),
                shaped('B', bx, false),
                space(b_end),
                shaped('C', b_end + advance(' '), false),
            ],
        }]);
        let fonts = vec![(
            (family, FontVariant::default(), FontVariations::default()),
            bytes.to_vec(),
        )];
        for replacement in [false, true] {
            let mut document = doc.clone();
            if replacement {
                document.pages[0].items = vec![FrameItem::TextReplacement {
                    text: "ALT".into(),
                    items: document.pages[0].items.clone(),
                }];
            }
            for mode in
                [crate::export::StreamMode::Compact, crate::export::StreamMode::Verbose]
            {
                for pdf in [
                    crate::export::export_pdf_with_font(&document, bytes, mode),
                    crate::export::export_pdf_multifont(&document, &fonts, mode),
                ] {
                    let text = extracted(&pdf);
                    assert_eq!(text.trim(), if replacement { "ALT" } else { "A \u{2060}B C" },
                        "ordinary spaces and source joiner must coexist without font-global aliasing");
                }
            }
        }
    }
}
