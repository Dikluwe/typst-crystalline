//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/stream/text_replacement.md
//! @prompt-hash dc68abf5
//! @layer L3
//! @updated 2026-09-28

/// Preserve occurrence identity without changing its visual font mapping.
pub(super) fn opening(ops: &mut String, text: &str) {
    use std::fmt::Write;
    ops.push_str("/Span << /ActualText <FEFF");
    for unit in text.encode_utf16() {
        write!(ops, "{unit:04X}").expect("writing to String is infallible");
    }
    ops.push_str("> >> BDC\n");
}
pub(super) fn closing(ops: &mut String) {
    ops.push_str("EMC\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn discretionary_identity_pdf_utf16_roundtrips_empty_shy_and_non_bmp() {
        for text in ["", "\u{ad}", "a(\\)𝄞"] {
            let mut ops = String::new();
            opening(&mut ops, text);
            closing(&mut ops);
            let hex = ops.split("<FEFF").nth(1).unwrap().split('>').next().unwrap();
            let units: Vec<u16> = hex
                .as_bytes()
                .chunks(4)
                .map(|chunk| {
                    u16::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap()
                })
                .collect();
            assert_eq!(String::from_utf16(&units).unwrap(), text);
            assert_eq!(ops.matches("BDC").count(), 1);
            assert_eq!(ops.matches("EMC").count(), 1);
        }
    }
    #[test]
    #[allow(deprecated)]
    fn discretionary_identity_pdf_type1_top_and_local_keep_literal_hyphen() {
        use crate::export::{export_pdf_with_tags, PdfTags, StreamMode};
        use typst_core::compiler::layout::layout;
        use typst_core::entities::{
            content::Content,
            layout_types::{FrameItem, Point, Pt, TextStyle, TransformMatrix},
        };
        for mode in [StreamMode::Compact, StreamMode::Verbose] {
            for tags in [PdfTags::Enabled, PdfTags::Disabled] {
                for local in [false, true] {
                    let mut doc = layout(&Content::text("placeholder"));
                    let mut style = TextStyle::regular(Pt(12.0));
                    style.weight = Some(700);
                    let children = vec![
                        FrameItem::TextReplacement {
                            text: "prefix\u{ad}".into(),
                            items: vec![FrameItem::Text {
                                pos: Point { x: Pt(10.0), y: Pt(20.0) },
                                text: "prefix-".into(),
                                style: style.clone(),
                                bidi: None,
                            }],
                        },
                        FrameItem::Text {
                            pos: Point { x: Pt(80.0), y: Pt(20.0) },
                            text: "-".into(),
                            style,
                            bidi: None,
                        },
                    ];
                    doc.pages[0].items = if local {
                        vec![FrameItem::Group {
                            paint_root: false,
                            pos: Point::ZERO,
                            matrix: TransformMatrix::identity(),
                            clip_mask: None,
                            inner_width: 100.0,
                            inner_height: 100.0,
                            items: children,
                        }]
                    } else {
                        children
                    };
                    let pdf = export_pdf_with_tags(&doc, mode, tags);
                    let ops =
                        crate::export::test_helpers::extract_page_content_streams_text(
                            &pdf,
                        );
                    assert_eq!(ops.matches("/ActualText").count(), 1, "{ops}");
                    let occurrence = ops.split("/ActualText").nth(1).unwrap();
                    assert!(
                        occurrence.find("EMC").unwrap() < occurrence.find("Q\n").unwrap(),
                        "replacement restored graphics before extraction: {ops}"
                    );
                    assert!(ops.contains("<FEFF00700072006500660069007800AD>"), "{ops}");
                    assert!(ops.contains("(prefix-)"), "{ops}");
                    assert!(ops.contains("(-)"), "{ops}");
                    assert_eq!(
                        ops.matches("BDC").count(),
                        ops.matches("EMC").count(),
                        "{ops}"
                    );
                }
            }
        }
    }
}
