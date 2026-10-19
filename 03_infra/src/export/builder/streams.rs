//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/builder/streams.md
//! @prompt-hash 7f8515a1
//! @layer L3
//! @updated 2026-09-26

use super::PdfBuilder;
use crate::export::compress_zlib;
use crate::export::PdfSerializationMode;

/// **P884** — constrói o content stream de uma página, comprimindo com
/// FlateDecode quando rentável.
///
/// O vanilla 0.15.0 comprime os content streams; a sonda de P883 mostrou que
/// esta é a maior fonte de diferença de tamanho em documentos longos
/// (~970 KB → ~134 KB em `06-long`).
///
/// Para inputs muito pequenos, a compressão pode aumentar o tamanho ou falhar
/// silenciosamente; neste caso emite-se o stream sem `/Filter`.
fn build_content_stream(
    stream_data: &[u8],
    serialization: PdfSerializationMode,
) -> Vec<u8> {
    if serialization == PdfSerializationMode::Pretty {
        return build_uncompressed_content_stream(stream_data);
    }
    match compress_zlib(stream_data) {
        Ok(compressed) if compressed.len() < stream_data.len() => {
            let len = compressed.len();
            let mut stream =
                format!("<< /Length {len} /Filter /FlateDecode >>\nstream\n")
                    .into_bytes();
            stream.extend_from_slice(&compressed);
            stream.extend_from_slice(b"\nendstream");
            stream
        }
        _ => {
            let len = stream_data.len();
            let mut stream = format!("<< /Length {len} >>\nstream\n").into_bytes();
            stream.extend_from_slice(stream_data);
            stream.extend_from_slice(b"\nendstream");
            stream
        }
    }
}

fn build_uncompressed_content_stream(stream_data: &[u8]) -> Vec<u8> {
    let len = stream_data.len();
    let mut stream = format!("<< /Length {len} >>\nstream\n").into_bytes();
    stream.extend_from_slice(stream_data);
    stream.extend_from_slice(b"\nendstream");
    stream
}

impl PdfBuilder {
    /// **P980** — aplica as transformações do oráculo ao content stream
    /// de uma página quando o modo oráculo está activo; identidade no
    /// caminho normal.
    pub(super) fn maybe_oracle(&self, stream_bytes: Vec<u8>) -> Vec<u8> {
        if self.oracle {
            crate::export::oracle::collapse_trivial_tj_bytes(&stream_bytes)
        } else {
            stream_bytes
        }
    }

    pub(super) fn serialize_page_stream(&self, stream_data: &[u8]) -> Vec<u8> {
        match self.serialization {
            PdfSerializationMode::Compact => {
                build_content_stream(stream_data, PdfSerializationMode::Compact)
            }
            PdfSerializationMode::Pretty => {
                build_uncompressed_content_stream(stream_data)
            }
        }
    }
}

#[cfg(test)]
mod p1360_ab_tests {
    use super::*;
    use crate::export::StreamMode;
    use typst_core::entities::color::Color;
    use typst_core::entities::frame_paint::FramePaint as Paint;
    use typst_core::entities::geometry::ShapeKind;
    use typst_core::entities::layout_types::{FrameItem, Page, PagedDocument, Point, Pt};

    fn page_stream_object(pdf: &[u8]) -> &[u8] {
        let text = String::from_utf8_lossy(pdf);
        let page = text.find("/Type /Page ").expect("Page dictionary");
        let contents = text[page..].find("/Contents ").expect("Page /Contents") + page;
        let reference = text[contents + "/Contents ".len()..]
            .split_whitespace()
            .next()
            .expect("Contents object id");
        let object = format!("\n{reference} 0 obj\n");
        let start = text.find(&object).expect("Contents object") + 1;
        let end = text[start..].find("\nendobj").expect("Contents end") + start;
        &pdf[start..end]
    }

    fn profitable_document() -> PagedDocument {
        let items = (0..96)
            .map(|row| FrameItem::Shape {
                pos: Point { x: Pt(2.0), y: Pt(2.0 + row as f64) },
                kind: ShapeKind::Rect,
                width: 90.0,
                height: 0.5,
                fill: Some(Paint::Solid(Color::rgb(0, 0, 0))),
                stroke: None,
                fill_rule: Default::default(),
                parent_bbox_at_emit: None,
            })
            .collect();
        PagedDocument::new(vec![Page {
            width: 100.0,
            height: 100.0,
            numbering: None,
            logical_number: 1,
            supplement: typst_core::entities::content::Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items,
        }])
    }

    #[test]
    fn p1360_ab_page_stream_compact_vs_pretty_are_orthogonal_to_operators() {
        let doc = profitable_document();
        let compact = crate::export::export_pdf_with_tags_and_serialization(
            &doc,
            StreamMode::Verbose,
            crate::export::PdfTags::Disabled,
            PdfSerializationMode::Compact,
        );
        let pretty = crate::export::export_pdf_with_tags_and_serialization(
            &doc,
            StreamMode::Verbose,
            crate::export::PdfTags::Disabled,
            PdfSerializationMode::Pretty,
        );
        let compact = page_stream_object(&compact);
        let pretty = page_stream_object(&pretty);

        assert!(
            compact
                .windows(b"/Filter /FlateDecode".len())
                .any(|window| window == b"/Filter /FlateDecode"),
            "a profitable compact page content stream must use FlateDecode"
        );
        assert!(
            !pretty
                .windows(b"/FlateDecode".len())
                .any(|window| window == b"/FlateDecode"),
            "pretty page content streams must remain uncompressed"
        );
        assert!(pretty.windows(b"stream\n".len()).any(|w| w == b"stream\n"));
        assert!(pretty.windows(b"endstream".len()).any(|w| w == b"endstream"));
    }
}
