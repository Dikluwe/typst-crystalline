//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export.md
//! @prompt-hash efdf6de2
//! @layer L3
//! @updated 2026-03-28

use typst_core::entities::layout_types::{FrameItem, PagedDocument};

/// Serializa um `PagedDocument` para bytes PDF-1.7.
///
/// Geração manual — sem crates externas de PDF (o original usa `krilla`).
/// Inversão de eixo Y: `y_pdf = page_height - y_cristalino`.
/// Não-ASCII → `?` (placeholder; UTF-16BE no Passo 21).
pub fn export_pdf(doc: &PagedDocument) -> Vec<u8> {
    PdfBuilder::new().build(doc)
}

// ── Builder ────────────────────────────────────────────────────────────────

struct PdfBuilder {
    objects: Vec<(usize, Vec<u8>)>,  // (id, conteúdo serializado)
}

impl PdfBuilder {
    fn new() -> Self {
        Self { objects: Vec::new() }
    }

    fn add(&mut self, id: usize, content: String) {
        self.objects.push((id, content.into_bytes()));
    }

    fn build(mut self, doc: &PagedDocument) -> Vec<u8> {
        let n = doc.pages.len().max(1);
        // IDs: 1=Catalog, 2=Pages, 3..3+n=PageDicts, 3+n..3+2n=Streams,
        //      3+2n=F1, 3+2n+1=F2, 3+2n+2=F3
        let first_page   = 3usize;
        let first_stream = first_page + n;
        let font_f1      = first_stream + n;
        let font_f2      = font_f1 + 1;
        let font_f3      = font_f2 + 1;

        // Obj 1: Catalog
        self.add(1, "<< /Type /Catalog /Pages 2 0 R >>".into());

        // Obj 2: Pages
        let kids = (first_page..first_page + n)
            .map(|i| format!("{i} 0 R"))
            .collect::<Vec<_>>()
            .join(" ");
        self.add(2, format!("<< /Type /Pages /Kids [{kids}] /Count {n} >>"));

        // Uma entrada por página
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

            // Content stream
            let stream_bytes = build_page_stream(page);
            let len = stream_bytes.len();
            let mut stream_obj = format!("<< /Length {len} >>\nstream\n").into_bytes();
            stream_obj.extend_from_slice(&stream_bytes);
            stream_obj.extend_from_slice(b"\nendstream");
            self.objects.push((stream_id, stream_obj));
        }

        // Fontes Standard Type1 — sem embedding necessário
        self.add(font_f1, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica \
                            /Encoding /WinAnsiEncoding >>".into());
        self.add(font_f2, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold \
                            /Encoding /WinAnsiEncoding >>".into());
        self.add(font_f3, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Oblique \
                            /Encoding /WinAnsiEncoding >>".into());

        self.serialize()
    }

    fn serialize(self) -> Vec<u8> {
        // Header — %PDF-1.7 + comentário binário (4 bytes > 127)
        let mut out: Vec<u8> = b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n".to_vec();

        let mut offsets: std::collections::HashMap<usize, usize> = Default::default();

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
        // Entrada 0: free list sentinel
        out.extend_from_slice(b"0000000000 65535 f \n");
        // Entradas 1..=max_id
        for id in 1..=max_id {
            let off = offsets.get(&id).copied().unwrap_or(0);
            // Cada entrada xref tem exactamente 20 bytes: "%010d %05d %c \n"
            out.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
        }

        // Trailer
        out.extend_from_slice(format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n",
            max_id + 1,
            xref_start
        ).as_bytes());

        out
    }
}

// ── Helpers ────────────────────────────────────────────────────────────────

fn build_page_stream(page: &typst_core::entities::layout_types::Frame) -> Vec<u8> {
    let mut ops = String::new();
    let page_height = page.size.height.val();

    for item in &page.items {
        match item {
            FrameItem::Text { pos, text, style } => {
                // Inversão do eixo Y: y_pdf = page_height - y_cristalino
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
        // Testar ( ) \ separadamente de % para evitar colisão:
        // '\' → '\\' e '%' adjacente criaria '\%' naturalmente.
        let escaped = escape_pdf_string("Hello (world) back\\slash");
        assert!(escaped.contains("\\("),  "( deve ser escapado");
        assert!(escaped.contains("\\)"),  ") deve ser escapado");
        assert!(escaped.contains("\\\\"), "\\ deve ser escapado");
        // % não deve ser escapado — verificar com input sem \ adjacente
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
}
