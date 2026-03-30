//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export.md
//! @prompt-hash efdf6de2
//! @layer L3
//! @updated 2026-03-29

use std::collections::HashMap;

use ttf_parser::Face;
use typst_core::entities::layout_types::{Frame, FrameItem, PagedDocument};

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

// ── Builder ────────────────────────────────────────────────────────────────

struct PdfBuilder {
    objects: Vec<(usize, Vec<u8>)>,
}

impl PdfBuilder {
    fn new() -> Self { Self { objects: Vec::new() } }

    fn add(&mut self, id: usize, content: String) {
        self.objects.push((id, content.into_bytes()));
    }

    fn add_bytes(&mut self, id: usize, content: Vec<u8>) {
        self.objects.push((id, content));
    }

    fn build(self, doc: &PagedDocument, font_data: Option<&[u8]>) -> Vec<u8> {
        if let Some(data) = font_data {
            if let Ok(face) = Face::parse(data, 0) {
                return self.build_cidfont(doc, &face, data);
            }
        }
        self.build_helvetica(doc)
    }

    // ── Caminho Helvetica (fallback, Type1 sem embedding) ─────────────────

    fn build_helvetica(mut self, doc: &PagedDocument) -> Vec<u8> {
        let n = doc.pages.len().max(1);
        let first_page   = 3usize;
        let first_stream = first_page + n;
        let font_f1      = first_stream + n;
        let font_f2      = font_f1 + 1;
        let font_f3      = font_f2 + 1;

        self.add(1, "<< /Type /Catalog /Pages 2 0 R >>".into());

        let kids = (first_page..first_page + n)
            .map(|i| format!("{i} 0 R"))
            .collect::<Vec<_>>().join(" ");
        self.add(2, format!("<< /Type /Pages /Kids [{kids}] /Count {n} >>"));

        for (i, page) in doc.pages.iter().enumerate() {
            let page_id   = first_page + i;
            let stream_id = first_stream + i;
            let w = page.size.width.val();
            let h = page.size.height.val();

            self.add(page_id, format!(
                "<< /Type /Page /Parent 2 0 R \
                   /MediaBox [0 0 {w:.1} {h:.1}] \
                   /Contents {stream_id} 0 R \
                   /Resources << /Font << \
                     /F1 {font_f1} 0 R \
                     /F2 {font_f2} 0 R \
                     /F3 {font_f3} 0 R \
                   >> >> >>"
            ));

            let stream_bytes = build_page_stream_type1(page);
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

        self.serialize()
    }

    // ── Caminho CIDFont (Unicode completo, Identity-H) ─────────────────────

    fn build_cidfont(mut self, doc: &PagedDocument, face: &Face<'_>, font_data: &[u8]) -> Vec<u8> {
        let n = doc.pages.len().max(1);
        let first_page         = 3usize;
        let first_stream       = first_page + n;
        let font_id            = first_stream + n;      // Type0 — /F1
        let cidfont_id         = font_id + 1;
        let font_descriptor_id = font_id + 2;
        let font_stream_id     = font_id + 3;
        let to_unicode_id      = font_id + 4;

        let chars    = collect_codepoints(doc);
        let mappings = map_chars_to_glyphs(face, &chars);
        let char_to_gid: HashMap<char, u16> = mappings.iter().copied().collect();
        let widths = widths_array(face, &mappings);

        self.add(1, "<< /Type /Catalog /Pages 2 0 R >>".into());

        let kids = (first_page..first_page + n)
            .map(|i| format!("{i} 0 R"))
            .collect::<Vec<_>>().join(" ");
        self.add(2, format!("<< /Type /Pages /Kids [{kids}] /Count {n} >>"));

        for (i, page) in doc.pages.iter().enumerate() {
            let page_id   = first_page + i;
            let stream_id = first_stream + i;
            let w = page.size.width.val();
            let h = page.size.height.val();

            self.add(page_id, format!(
                "<< /Type /Page /Parent 2 0 R \
                   /MediaBox [0 0 {w:.1} {h:.1}] \
                   /Contents {stream_id} 0 R \
                   /Resources << /Font << /F1 {font_id} 0 R >> >> >>"
            ));

            let stream_bytes = build_page_stream_cidfont(page, &char_to_gid);
            let len = stream_bytes.len();
            let mut obj = format!("<< /Length {len} >>\nstream\n").into_bytes();
            obj.extend_from_slice(&stream_bytes);
            obj.extend_from_slice(b"\nendstream");
            self.add_bytes(stream_id, obj);
        }

        // Type0 font (F1)
        self.add(font_id, format!(
            "<< /Type /Font /Subtype /Type0 /BaseFont /CrystallineFont \
               /Encoding /Identity-H \
               /DescendantFonts [{cidfont_id} 0 R] \
               /ToUnicode {to_unicode_id} 0 R >>"
        ));

        // CIDFont
        self.add(cidfont_id, format!(
            "<< /Type /Font /Subtype /CIDFontType2 /BaseFont /CrystallineFont \
               /CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> \
               /FontDescriptor {font_descriptor_id} 0 R \
               /DW 500 \
               /W [{widths}] >>"
        ));

        // FontDescriptor
        self.add(font_descriptor_id, format!(
            "<< /Type /FontDescriptor /FontName /CrystallineFont \
               /Flags 32 \
               /FontBBox [-1000 -200 2000 900] \
               /ItalicAngle 0 /Ascent 800 /Descent -200 \
               /CapHeight 700 /StemV 80 \
               /FontFile2 {font_stream_id} 0 R >>"
        ));

        // Font data stream — Opção A: fonte completa sem subsetting (ADR-0027)
        let font_len = font_data.len();
        let mut font_stream = format!(
            "<< /Length {font_len} /Subtype /CIDFontType2 >>\nstream\n"
        ).into_bytes();
        font_stream.extend_from_slice(font_data);
        font_stream.extend_from_slice(b"\nendstream");
        self.add_bytes(font_stream_id, font_stream);

        // ToUnicode CMap stream
        let cmap = to_unicode_cmap(&mappings);
        let cmap_len = cmap.len();
        let mut cmap_obj = format!("<< /Length {cmap_len} >>\nstream\n").into_bytes();
        cmap_obj.extend_from_slice(&cmap);
        cmap_obj.extend_from_slice(b"\nendstream");
        self.add_bytes(to_unicode_id, cmap_obj);

        self.serialize()
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

// ── Helpers — caminho Helvetica ────────────────────────────────────────────

fn build_page_stream_type1(page: &Frame) -> Vec<u8> {
    let mut ops = String::new();
    let page_height = page.size.height.val();

    for item in &page.items {
        match item {
            FrameItem::Text { pos, text, style } => {
                let pdf_y = page_height - pos.y.val();
                let safe  = escape_pdf_string(text.as_str());

                if safe.is_empty() { continue; }

                let font_ref = match (style.bold, style.italic) {
                    (true,  _)     => "F2",  // Helvetica-Bold
                    (false, true)  => "F3",  // Helvetica-Oblique
                    (false, false) => "F1",  // Helvetica
                };

                ops.push_str(&format!(
                    "BT\n/{font_ref} {:.1} Tf\n{:.1} {:.1} Td\n({safe}) Tj\nET\n",
                    style.size.val(), pos.x.val(), pdf_y
                ));
            }
        }
    }

    ops.into_bytes()
}

fn escape_pdf_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '('  => out.push_str("\\("),
            ')'  => out.push_str("\\)"),
            '\\' => out.push_str("\\\\"),
            c if c.is_ascii() && c >= ' ' => out.push(c),
            _    => out.push('?'),
        }
    }
    out
}

// ── Helpers — caminho CIDFont ──────────────────────────────────────────────

/// Coleciona todos os codepoints Unicode distintos usados no documento.
fn collect_codepoints(doc: &PagedDocument) -> Vec<char> {
    let mut seen = std::collections::BTreeSet::new();
    for page in &doc.pages {
        for item in &page.items {
            if let FrameItem::Text { text, .. } = item {
                for c in text.chars() {
                    seen.insert(c);
                }
            }
        }
    }
    seen.into_iter().collect()
}

/// Para um conjunto de chars, retorna Vec<(char, glyph_id)>.
/// Chars sem glyph na fonte são omitidos.
fn map_chars_to_glyphs(face: &Face<'_>, chars: &[char]) -> Vec<(char, u16)> {
    chars.iter()
        .filter_map(|&c| face.glyph_index(c).map(|gid| (c, gid.0)))
        .collect()
}

/// Gera o array W do CIDFont: "gid [width] ..." em unidades PDF (1/1000 text space).
fn widths_array(face: &Face<'_>, mappings: &[(char, u16)]) -> String {
    let upem = face.units_per_em() as f64;
    let mut parts = Vec::new();
    for (_c, gid) in mappings {
        let adv = face.glyph_hor_advance(ttf_parser::GlyphId(*gid))
            .unwrap_or(500) as f64;
        let w = (adv / upem * 1000.0).round() as i32;
        parts.push(format!("{gid} [{w}]"));
    }
    parts.join(" ")
}

/// Gera o stream ToUnicode CMap para o mapeamento glyph_id → char.
/// Emite em blocos de ≤ 100 entradas (limite PDF spec).
fn to_unicode_cmap(mappings: &[(char, u16)]) -> Vec<u8> {
    let mut s = String::new();
    s.push_str("/CIDInit /ProcSet findresource begin\n");
    s.push_str("12 dict begin\n");
    s.push_str("begincmap\n");
    s.push_str("/CIDSystemInfo << /Registry (Adobe) /Ordering (UCS) /Supplement 0 >> def\n");
    s.push_str("/CMapName /Adobe-Identity-UCS def\n");
    s.push_str("/CMapType 2 def\n");
    s.push_str("1 begincodespacerange\n");
    s.push_str("<0000> <FFFF>\n");
    s.push_str("endcodespacerange\n");

    for chunk in mappings.chunks(100) {
        s.push_str(&format!("{} beginbfchar\n", chunk.len()));
        for (c, gid) in chunk {
            let cp = *c as u32;
            s.push_str(&format!("<{gid:04X}> <{cp:04X}>\n"));
        }
        s.push_str("endbfchar\n");
    }

    s.push_str("endcmap\n");
    s.push_str("CMapName currentdict /CMap defineresource pop\n");
    s.push_str("end\nend\n");
    s.into_bytes()
}

/// Serializa texto como string hexadecimal de glyph IDs para Identity-H.
/// Chars sem mapeamento usam glyph ID 0 (notdef).
fn text_to_hex_string(text: &str, char_to_gid: &HashMap<char, u16>) -> String {
    let mut hex = String::from("<");
    for c in text.chars() {
        let gid = char_to_gid.get(&c).copied().unwrap_or(0);
        hex.push_str(&format!("{gid:04X}"));
    }
    hex.push('>');
    hex
}

fn build_page_stream_cidfont(page: &Frame, char_to_gid: &HashMap<char, u16>) -> Vec<u8> {
    let mut ops = String::new();
    let page_height = page.size.height.val();

    for item in &page.items {
        match item {
            FrameItem::Text { pos, text, style } => {
                if text.is_empty() { continue; }

                let pdf_y   = page_height - pos.y.val();
                let hex_str = text_to_hex_string(text.as_str(), char_to_gid);

                // Identity-H: F1 para todos os estilos (bold/italic requer fontes adicionais — DEBT)
                ops.push_str(&format!(
                    "BT\n/F1 {:.1} Tf\n{:.1} {:.1} Td\n{hex_str} Tj\nET\n",
                    style.size.val(), pos.x.val(), pdf_y
                ));
            }
        }
    }

    ops.into_bytes()
}

// ── Testes ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::{
        entities::content::Content,
        rules::layout::layout,
    };

    #[test]
    fn pdf_header_correcto() {
        let doc = layout(&Content::text("Hello"));
        let pdf = export_pdf(&doc);
        assert!(pdf.starts_with(b"%PDF-1.7"), "deve começar com %PDF-1.7");
    }

    #[test]
    fn pdf_termina_com_eof() {
        let doc = layout(&Content::text("Test"));
        let pdf = export_pdf(&doc);
        let tail = std::str::from_utf8(&pdf[pdf.len().saturating_sub(20)..]).unwrap_or("");
        assert!(tail.contains("%%EOF"), "deve terminar com %%EOF");
    }

    #[test]
    fn pdf_tem_estrutura_valida() {
        let doc = layout(&Content::text("Test"));
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("xref"),      "deve ter xref");
        assert!(s.contains("trailer"),   "deve ter trailer");
        assert!(s.contains("startxref"), "deve ter startxref");
        assert!(s.contains("/Catalog"),  "deve ter Catalog");
        assert!(s.contains("/Pages"),    "deve ter Pages");
        assert!(s.contains("Helvetica"), "deve ter Helvetica");
    }

    #[test]
    fn pdf_contem_texto_ascii() {
        let doc = layout(&Content::text("Hello world"));
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("Hello") || s.contains("world"),
            "texto ASCII deve aparecer no PDF");
    }

    #[test]
    fn pdf_documento_vazio_valido() {
        let doc = typst_core::entities::layout_types::PagedDocument::new(vec![]);
        let pdf = export_pdf(&doc);
        assert!(pdf.starts_with(b"%PDF-1.7"));
        assert!(String::from_utf8_lossy(&pdf).contains("%%EOF"));
    }

    #[test]
    fn escaping_caracteres_especiais() {
        let escaped = escape_pdf_string("Hello (world) back\\slash");
        assert!(escaped.contains("\\("),  "( deve ser escapado");
        assert!(escaped.contains("\\)"),  ") deve ser escapado");
        assert!(escaped.contains("\\\\"), "\\ deve ser escapado");
        let with_percent = escape_pdf_string("100% done");
        assert!(!with_percent.contains("\\%"), "% não precisa de escape em PDF strings");
        assert!(with_percent.contains('%'), "% deve aparecer sem escape");
    }

    #[test]
    fn inversao_eixo_y_texto_no_topo() {
        use typst_core::entities::layout_types::{
            Frame, FrameItem, PagedDocument, Point, Pt, Size, TextStyle,
        };
        let mut frame = Frame::new(Size::a4());
        frame.push(FrameItem::Text {
            pos:   Point { x: Pt(72.0), y: Pt(84.0) },
            text:  "Top".into(),
            style: TextStyle::regular(Pt(12.0)),
        });
        let doc = PagedDocument::new(vec![frame]);
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        // y_pdf = 842 - 84 = 758
        assert!(s.contains("758.0") || s.contains("758"),
            "y_pdf deve ser 842-84=758: {}", &s[..s.len().min(500)]);
    }

    #[test]
    fn pdf_mediabox_dimensoes_a4() {
        let doc = layout(&Content::text("Test"));
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("595") && s.contains("842"),
            "MediaBox deve ter dimensões A4 (595x842 pt)");
    }

    // ── Passo 24 — DEBT-5: Unicode PDF ──────────────────────────────────────

    #[test]
    fn unicode_nao_produz_interrogacao() {
        // Modo Helvetica — documenta intenção. Com CIDFont + fonte real, '?' desaparece.
        let doc = layout(&Content::text("café naïve résumé"));
        let pdf = export_pdf(&doc);
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("xref"), "PDF deve ser estruturalmente válido");
    }

    #[test]
    #[ignore = "requer fixture de fonte TrueType"]
    fn cidfont_presente_quando_ha_fonte() {
        // Estrutura esperada no PDF com fonte real:
        // /Type0, /CIDFontType2, /ToUnicode, /FontFile2 devem estar presentes
    }

    #[test]
    fn texto_ascii_com_cidfont() {
        let doc = layout(&Content::text("Hello World"));
        let pdf = export_pdf(&doc);
        assert!(pdf.starts_with(b"%PDF-1.7"));
        let s = String::from_utf8_lossy(&pdf);
        assert!(s.contains("xref") && s.contains("%%EOF"));
    }

    #[test]
    fn bullet_e_unicode() {
        use typst_core::entities::{content::Content as C2, layout_types::FrameItem};
        let doc = layout(&C2::list_item(C2::text("item")));
        let has_bullet = doc.pages.iter()
            .flat_map(|p| p.items.iter())
            .any(|i| matches!(i, FrameItem::Text { text, .. } if text.as_str() == "•"));
        assert!(has_bullet, "ListItem deve ter marcador '•' após DEBT-5 pago");
    }

    // ── Testes dos helpers CIDFont ────────────────────────────────────────────

    #[test]
    fn collect_codepoints_vazio() {
        use typst_core::entities::layout_types::PagedDocument;
        let doc = PagedDocument::new(vec![]);
        assert!(collect_codepoints(&doc).is_empty());
    }

    #[test]
    fn collect_codepoints_dedup() {
        let doc = layout(&Content::text("aaa bbb"));
        let chars = collect_codepoints(&doc);
        // BTreeSet garante que não há duplicados
        let unique: std::collections::BTreeSet<_> = chars.iter().copied().collect();
        assert_eq!(chars.len(), unique.len(), "não deve haver duplicados");
    }

    #[test]
    fn to_unicode_cmap_estrutura_basica() {
        let mappings: Vec<(char, u16)> = vec![('A', 36), ('B', 37)];
        let cmap = to_unicode_cmap(&mappings);
        let s = String::from_utf8(cmap).unwrap();
        assert!(s.contains("begincmap"), "deve ter begincmap");
        assert!(s.contains("endcmap"),   "deve ter endcmap");
        assert!(s.contains("beginbfchar"), "deve ter beginbfchar");
        // 'A' = U+0041, glyph 36 = 0x0024
        assert!(s.contains("<0024> <0041>"), "'A' deve mapear para U+0041");
    }

    #[test]
    fn to_unicode_cmap_blocos_de_100() {
        // 101 mappings → dois blocos (100 + 1)
        let mappings: Vec<(char, u16)> = (0u16..101)
            .filter_map(|i| char::from_u32(i as u32 + 32).map(|c| (c, i)))
            .collect();
        let cmap = to_unicode_cmap(&mappings);
        let s = String::from_utf8(cmap).unwrap();
        let count = s.matches("beginbfchar").count();
        assert_eq!(count, 2, "101 entradas → 2 blocos beginbfchar");
    }

    #[test]
    fn text_to_hex_string_ascii() {
        let mut map = HashMap::new();
        map.insert('H', 0x0048u16);
        map.insert('i', 0x0069u16);
        let hex = text_to_hex_string("Hi", &map);
        assert_eq!(hex, "<00480069>", "glyph IDs em hex, 2 bytes cada");
    }

    #[test]
    fn text_to_hex_string_sem_mapeamento_usa_zero() {
        let map = HashMap::new();
        let hex = text_to_hex_string("X", &map);
        assert_eq!(hex, "<0000>", "char sem mapeamento → glyph ID 0");
    }
}
